#[inline]
pub unsafe fn CertSrvBackupClose(hbc: *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("certadm.dll" "system" fn CertSrvBackupClose(hbc : *mut core::ffi::c_void) -> windows_core::HRESULT);
    CertSrvBackupClose(hbc).ok()
}
#[inline]
pub unsafe fn CertSrvBackupEnd(hbc: *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("certadm.dll" "system" fn CertSrvBackupEnd(hbc : *mut core::ffi::c_void) -> windows_core::HRESULT);
    CertSrvBackupEnd(hbc).ok()
}
#[inline]
pub unsafe fn CertSrvBackupFree(pv: *mut core::ffi::c_void) {
    windows_targets::link!("certadm.dll" "system" fn CertSrvBackupFree(pv : *mut core::ffi::c_void));
    CertSrvBackupFree(pv)
}
#[inline]
pub unsafe fn CertSrvBackupGetBackupLogsW(hbc: *const core::ffi::c_void, ppwszzbackuplogfiles: *mut windows_core::PWSTR, pcbsize: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("certadm.dll" "system" fn CertSrvBackupGetBackupLogsW(hbc : *const core::ffi::c_void, ppwszzbackuplogfiles : *mut windows_core::PWSTR, pcbsize : *mut u32) -> windows_core::HRESULT);
    CertSrvBackupGetBackupLogsW(hbc, ppwszzbackuplogfiles, pcbsize).ok()
}
#[inline]
pub unsafe fn CertSrvBackupGetDatabaseNamesW(hbc: *const core::ffi::c_void, ppwszzattachmentinformation: *mut windows_core::PWSTR, pcbsize: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("certadm.dll" "system" fn CertSrvBackupGetDatabaseNamesW(hbc : *const core::ffi::c_void, ppwszzattachmentinformation : *mut windows_core::PWSTR, pcbsize : *mut u32) -> windows_core::HRESULT);
    CertSrvBackupGetDatabaseNamesW(hbc, ppwszzattachmentinformation, pcbsize).ok()
}
#[inline]
pub unsafe fn CertSrvBackupGetDynamicFileListW(hbc: *const core::ffi::c_void, ppwszzfilelist: *mut windows_core::PWSTR, pcbsize: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("certadm.dll" "system" fn CertSrvBackupGetDynamicFileListW(hbc : *const core::ffi::c_void, ppwszzfilelist : *mut windows_core::PWSTR, pcbsize : *mut u32) -> windows_core::HRESULT);
    CertSrvBackupGetDynamicFileListW(hbc, ppwszzfilelist, pcbsize).ok()
}
#[inline]
pub unsafe fn CertSrvBackupOpenFileW<P0>(hbc: *mut core::ffi::c_void, pwszattachmentname: P0, cbreadhintsize: u32, plifilesize: *mut i64) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("certadm.dll" "system" fn CertSrvBackupOpenFileW(hbc : *mut core::ffi::c_void, pwszattachmentname : windows_core::PCWSTR, cbreadhintsize : u32, plifilesize : *mut i64) -> windows_core::HRESULT);
    CertSrvBackupOpenFileW(hbc, pwszattachmentname.param().abi(), cbreadhintsize, plifilesize).ok()
}
#[inline]
pub unsafe fn CertSrvBackupPrepareW<P0>(pwszservername: P0, grbitjet: u32, dwbackupflags: CSBACKUP_TYPE, phbc: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("certadm.dll" "system" fn CertSrvBackupPrepareW(pwszservername : windows_core::PCWSTR, grbitjet : u32, dwbackupflags : CSBACKUP_TYPE, phbc : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    CertSrvBackupPrepareW(pwszservername.param().abi(), grbitjet, dwbackupflags, phbc).ok()
}
#[inline]
pub unsafe fn CertSrvBackupRead(hbc: *mut core::ffi::c_void, pvbuffer: *mut core::ffi::c_void, cbbuffer: u32, pcbread: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("certadm.dll" "system" fn CertSrvBackupRead(hbc : *mut core::ffi::c_void, pvbuffer : *mut core::ffi::c_void, cbbuffer : u32, pcbread : *mut u32) -> windows_core::HRESULT);
    CertSrvBackupRead(hbc, pvbuffer, cbbuffer, pcbread).ok()
}
#[inline]
pub unsafe fn CertSrvBackupTruncateLogs(hbc: *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("certadm.dll" "system" fn CertSrvBackupTruncateLogs(hbc : *mut core::ffi::c_void) -> windows_core::HRESULT);
    CertSrvBackupTruncateLogs(hbc).ok()
}
#[inline]
pub unsafe fn CertSrvIsServerOnlineW<P0>(pwszservername: P0, pfserveronline: *mut super::super::super::Foundation::BOOL) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("certadm.dll" "system" fn CertSrvIsServerOnlineW(pwszservername : windows_core::PCWSTR, pfserveronline : *mut super::super::super::Foundation:: BOOL) -> windows_core::HRESULT);
    CertSrvIsServerOnlineW(pwszservername.param().abi(), pfserveronline).ok()
}
#[inline]
pub unsafe fn CertSrvRestoreEnd(hbc: *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("certadm.dll" "system" fn CertSrvRestoreEnd(hbc : *mut core::ffi::c_void) -> windows_core::HRESULT);
    CertSrvRestoreEnd(hbc).ok()
}
#[inline]
pub unsafe fn CertSrvRestoreGetDatabaseLocationsW(hbc: *const core::ffi::c_void, ppwszzdatabaselocationlist: *mut windows_core::PWSTR, pcbsize: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("certadm.dll" "system" fn CertSrvRestoreGetDatabaseLocationsW(hbc : *const core::ffi::c_void, ppwszzdatabaselocationlist : *mut windows_core::PWSTR, pcbsize : *mut u32) -> windows_core::HRESULT);
    CertSrvRestoreGetDatabaseLocationsW(hbc, ppwszzdatabaselocationlist, pcbsize).ok()
}
#[inline]
pub unsafe fn CertSrvRestorePrepareW<P0>(pwszservername: P0, dwrestoreflags: u32, phbc: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("certadm.dll" "system" fn CertSrvRestorePrepareW(pwszservername : windows_core::PCWSTR, dwrestoreflags : u32, phbc : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    CertSrvRestorePrepareW(pwszservername.param().abi(), dwrestoreflags, phbc).ok()
}
#[inline]
pub unsafe fn CertSrvRestoreRegisterComplete(hbc: *mut core::ffi::c_void, hrrestorestate: windows_core::HRESULT) -> windows_core::Result<()> {
    windows_targets::link!("certadm.dll" "system" fn CertSrvRestoreRegisterComplete(hbc : *mut core::ffi::c_void, hrrestorestate : windows_core::HRESULT) -> windows_core::HRESULT);
    CertSrvRestoreRegisterComplete(hbc, hrrestorestate).ok()
}
#[inline]
pub unsafe fn CertSrvRestoreRegisterThroughFile<P0, P1, P2>(hbc: *mut core::ffi::c_void, pwszcheckpointfilepath: P0, pwszlogpath: P1, rgrstmap: *mut CSEDB_RSTMAPW, crstmap: i32, pwszbackuplogpath: P2, genlow: u32, genhigh: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("certadm.dll" "system" fn CertSrvRestoreRegisterThroughFile(hbc : *mut core::ffi::c_void, pwszcheckpointfilepath : windows_core::PCWSTR, pwszlogpath : windows_core::PCWSTR, rgrstmap : *mut CSEDB_RSTMAPW, crstmap : i32, pwszbackuplogpath : windows_core::PCWSTR, genlow : u32, genhigh : u32) -> windows_core::HRESULT);
    CertSrvRestoreRegisterThroughFile(hbc, pwszcheckpointfilepath.param().abi(), pwszlogpath.param().abi(), rgrstmap, crstmap, pwszbackuplogpath.param().abi(), genlow, genhigh).ok()
}
#[inline]
pub unsafe fn CertSrvRestoreRegisterW<P0, P1, P2>(hbc: *mut core::ffi::c_void, pwszcheckpointfilepath: P0, pwszlogpath: P1, rgrstmap: *mut CSEDB_RSTMAPW, crstmap: i32, pwszbackuplogpath: P2, genlow: u32, genhigh: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("certadm.dll" "system" fn CertSrvRestoreRegisterW(hbc : *mut core::ffi::c_void, pwszcheckpointfilepath : windows_core::PCWSTR, pwszlogpath : windows_core::PCWSTR, rgrstmap : *mut CSEDB_RSTMAPW, crstmap : i32, pwszbackuplogpath : windows_core::PCWSTR, genlow : u32, genhigh : u32) -> windows_core::HRESULT);
    CertSrvRestoreRegisterW(hbc, pwszcheckpointfilepath.param().abi(), pwszlogpath.param().abi(), rgrstmap, crstmap, pwszbackuplogpath.param().abi(), genlow, genhigh).ok()
}
#[inline]
pub unsafe fn CertSrvServerControlW<P0>(pwszservername: P0, dwcontrolflags: u32, pcbout: *mut u32, ppbout: *mut *mut u8) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("certadm.dll" "system" fn CertSrvServerControlW(pwszservername : windows_core::PCWSTR, dwcontrolflags : u32, pcbout : *mut u32, ppbout : *mut *mut u8) -> windows_core::HRESULT);
    CertSrvServerControlW(pwszservername.param().abi(), dwcontrolflags, pcbout, ppbout).ok()
}
#[inline]
pub unsafe fn PstAcquirePrivateKey(pcert: *const super::CERT_CONTEXT) -> super::super::super::Foundation::NTSTATUS {
    windows_targets::link!("certpoleng.dll" "system" fn PstAcquirePrivateKey(pcert : *const super:: CERT_CONTEXT) -> super::super::super::Foundation:: NTSTATUS);
    PstAcquirePrivateKey(pcert)
}
#[cfg(feature = "Win32_Security_Authentication_Identity")]
#[inline]
pub unsafe fn PstGetCertificateChain(pcert: *const super::CERT_CONTEXT, ptrustedissuers: *const super::super::Authentication::Identity::SecPkgContext_IssuerListInfoEx, ppcertchaincontext: *mut *mut super::CERT_CHAIN_CONTEXT) -> super::super::super::Foundation::NTSTATUS {
    windows_targets::link!("certpoleng.dll" "system" fn PstGetCertificateChain(pcert : *const super:: CERT_CONTEXT, ptrustedissuers : *const super::super::Authentication::Identity:: SecPkgContext_IssuerListInfoEx, ppcertchaincontext : *mut *mut super:: CERT_CHAIN_CONTEXT) -> super::super::super::Foundation:: NTSTATUS);
    PstGetCertificateChain(pcert, ptrustedissuers, ppcertchaincontext)
}
#[inline]
pub unsafe fn PstGetCertificates<P0>(ptargetname: *const super::super::super::Foundation::UNICODE_STRING, rgpcriteria: Option<&[super::CERT_SELECT_CRITERIA]>, bisclient: P0, pdwcertchaincontextcount: *mut u32, ppcertchaincontexts: *mut *mut *mut super::CERT_CHAIN_CONTEXT) -> super::super::super::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Foundation::BOOL>,
{
    windows_targets::link!("certpoleng.dll" "system" fn PstGetCertificates(ptargetname : *const super::super::super::Foundation:: UNICODE_STRING, ccriteria : u32, rgpcriteria : *const super:: CERT_SELECT_CRITERIA, bisclient : super::super::super::Foundation:: BOOL, pdwcertchaincontextcount : *mut u32, ppcertchaincontexts : *mut *mut *mut super:: CERT_CHAIN_CONTEXT) -> super::super::super::Foundation:: NTSTATUS);
    PstGetCertificates(ptargetname, rgpcriteria.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(rgpcriteria.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), bisclient.param().abi(), pdwcertchaincontextcount, ppcertchaincontexts)
}
#[cfg(feature = "Win32_Security_Authentication_Identity")]
#[inline]
pub unsafe fn PstGetTrustAnchors(ptargetname: *const super::super::super::Foundation::UNICODE_STRING, rgpcriteria: Option<&[super::CERT_SELECT_CRITERIA]>, pptrustedissuers: *mut *mut super::super::Authentication::Identity::SecPkgContext_IssuerListInfoEx) -> super::super::super::Foundation::NTSTATUS {
    windows_targets::link!("certpoleng.dll" "system" fn PstGetTrustAnchors(ptargetname : *const super::super::super::Foundation:: UNICODE_STRING, ccriteria : u32, rgpcriteria : *const super:: CERT_SELECT_CRITERIA, pptrustedissuers : *mut *mut super::super::Authentication::Identity:: SecPkgContext_IssuerListInfoEx) -> super::super::super::Foundation:: NTSTATUS);
    PstGetTrustAnchors(ptargetname, rgpcriteria.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(rgpcriteria.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pptrustedissuers)
}
#[cfg(feature = "Win32_Security_Authentication_Identity")]
#[inline]
pub unsafe fn PstGetTrustAnchorsEx(ptargetname: *const super::super::super::Foundation::UNICODE_STRING, rgpcriteria: Option<&[super::CERT_SELECT_CRITERIA]>, pcertcontext: Option<*const super::CERT_CONTEXT>, pptrustedissuers: *mut *mut super::super::Authentication::Identity::SecPkgContext_IssuerListInfoEx) -> super::super::super::Foundation::NTSTATUS {
    windows_targets::link!("certpoleng.dll" "system" fn PstGetTrustAnchorsEx(ptargetname : *const super::super::super::Foundation:: UNICODE_STRING, ccriteria : u32, rgpcriteria : *const super:: CERT_SELECT_CRITERIA, pcertcontext : *const super:: CERT_CONTEXT, pptrustedissuers : *mut *mut super::super::Authentication::Identity:: SecPkgContext_IssuerListInfoEx) -> super::super::super::Foundation:: NTSTATUS);
    PstGetTrustAnchorsEx(ptargetname, rgpcriteria.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(rgpcriteria.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), core::mem::transmute(pcertcontext.unwrap_or(std::ptr::null())), pptrustedissuers)
}
#[inline]
pub unsafe fn PstGetUserNameForCertificate(pcertcontext: *const super::CERT_CONTEXT, username: *mut super::super::super::Foundation::UNICODE_STRING) -> super::super::super::Foundation::NTSTATUS {
    windows_targets::link!("certpoleng.dll" "system" fn PstGetUserNameForCertificate(pcertcontext : *const super:: CERT_CONTEXT, username : *mut super::super::super::Foundation:: UNICODE_STRING) -> super::super::super::Foundation:: NTSTATUS);
    PstGetUserNameForCertificate(pcertcontext, username)
}
#[cfg(feature = "Win32_Security_Authentication_Identity")]
#[inline]
pub unsafe fn PstMapCertificate(pcert: *const super::CERT_CONTEXT, ptokeninformationtype: *mut super::super::Authentication::Identity::LSA_TOKEN_INFORMATION_TYPE, pptokeninformation: *mut *mut core::ffi::c_void) -> super::super::super::Foundation::NTSTATUS {
    windows_targets::link!("certpoleng.dll" "system" fn PstMapCertificate(pcert : *const super:: CERT_CONTEXT, ptokeninformationtype : *mut super::super::Authentication::Identity:: LSA_TOKEN_INFORMATION_TYPE, pptokeninformation : *mut *mut core::ffi::c_void) -> super::super::super::Foundation:: NTSTATUS);
    PstMapCertificate(pcert, ptokeninformationtype, pptokeninformation)
}
#[inline]
pub unsafe fn PstValidate<P0>(ptargetname: Option<*const super::super::super::Foundation::UNICODE_STRING>, bisclient: P0, prequestedissuancepolicy: Option<*const super::CERT_USAGE_MATCH>, phadditionalcertstore: Option<*const super::HCERTSTORE>, pcert: *const super::CERT_CONTEXT, pprovguid: Option<*mut windows_core::GUID>) -> super::super::super::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Foundation::BOOL>,
{
    windows_targets::link!("certpoleng.dll" "system" fn PstValidate(ptargetname : *const super::super::super::Foundation:: UNICODE_STRING, bisclient : super::super::super::Foundation:: BOOL, prequestedissuancepolicy : *const super:: CERT_USAGE_MATCH, phadditionalcertstore : *const super:: HCERTSTORE, pcert : *const super:: CERT_CONTEXT, pprovguid : *mut windows_core::GUID) -> super::super::super::Foundation:: NTSTATUS);
    PstValidate(core::mem::transmute(ptargetname.unwrap_or(std::ptr::null())), bisclient.param().abi(), core::mem::transmute(prequestedissuancepolicy.unwrap_or(std::ptr::null())), core::mem::transmute(phadditionalcertstore.unwrap_or(std::ptr::null())), pcert, core::mem::transmute(pprovguid.unwrap_or(std::ptr::null_mut())))
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IAlternativeName, IAlternativeName_Vtbl, 0x728ab313_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IAlternativeName {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IAlternativeName, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IAlternativeName {
    pub unsafe fn InitializeFromString<P0>(&self, r#type: AlternativeNameType, strvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).InitializeFromString)(windows_core::Interface::as_raw(self), r#type, strvalue.param().abi()).ok()
    }
    pub unsafe fn InitializeFromRawData<P0>(&self, r#type: AlternativeNameType, encoding: EncodingType, strrawdata: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).InitializeFromRawData)(windows_core::Interface::as_raw(self), r#type, encoding, strrawdata.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitializeFromOtherName<P0, P1, P2>(&self, pobjectid: P0, encoding: EncodingType, strrawdata: P1, tobewrapped: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IObjectId>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).InitializeFromOtherName)(windows_core::Interface::as_raw(self), pobjectid.param().abi(), encoding, strrawdata.param().abi(), tobewrapped.param().abi()).ok()
    }
    pub unsafe fn Type(&self) -> windows_core::Result<AlternativeNameType> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Type)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn StrValue(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StrValue)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ObjectId(&self) -> windows_core::Result<IObjectId> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ObjectId)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn get_RawData(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_RawData)(windows_core::Interface::as_raw(self), encoding, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAlternativeName_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub InitializeFromString: unsafe extern "system" fn(*mut core::ffi::c_void, AlternativeNameType, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub InitializeFromRawData: unsafe extern "system" fn(*mut core::ffi::c_void, AlternativeNameType, EncodingType, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub InitializeFromOtherName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, EncodingType, core::mem::MaybeUninit<windows_core::BSTR>, super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InitializeFromOtherName: usize,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AlternativeNameType) -> windows_core::HRESULT,
    pub StrValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ObjectId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ObjectId: usize,
    pub get_RawData: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IAlternativeNames, IAlternativeNames_Vtbl, 0x728ab314_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IAlternativeNames {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IAlternativeNames, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IAlternativeNames {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_ItemByIndex(&self, index: i32) -> windows_core::Result<IAlternativeName> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_ItemByIndex)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Add<P0>(&self, pval: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAlternativeName>,
    {
        (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), pval.param().abi()).ok()
    }
    pub unsafe fn Remove(&self, index: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), index).ok()
    }
    pub unsafe fn Clear(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAlternativeNames_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub get_ItemByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_ItemByIndex: usize,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IBinaryConverter, IBinaryConverter_Vtbl, 0x728ab302_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IBinaryConverter {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IBinaryConverter, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IBinaryConverter {
    pub unsafe fn StringToString<P0>(&self, strencodedin: P0, encodingin: EncodingType, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StringToString)(windows_core::Interface::as_raw(self), strencodedin.param().abi(), encodingin, encoding, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn VariantByteArrayToString(&self, pvarbytearray: *const windows_core::VARIANT, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).VariantByteArrayToString)(windows_core::Interface::as_raw(self), core::mem::transmute(pvarbytearray), encoding, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn StringToVariantByteArray<P0>(&self, strencoded: P0, encoding: EncodingType) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StringToVariantByteArray)(windows_core::Interface::as_raw(self), strencoded.param().abi(), encoding, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IBinaryConverter_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub StringToString: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, EncodingType, EncodingType, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub VariantByteArrayToString: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, EncodingType, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub StringToVariantByteArray: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, EncodingType, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IBinaryConverter2, IBinaryConverter2_Vtbl, 0x8d7928b4_4e17_428d_9a17_728df00d1b2b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IBinaryConverter2 {
    type Target = IBinaryConverter;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IBinaryConverter2, windows_core::IUnknown, super::super::super::System::Com::IDispatch, IBinaryConverter);
#[cfg(feature = "Win32_System_Com")]
impl IBinaryConverter2 {
    pub unsafe fn StringArrayToVariantArray(&self, pvarstringarray: *const windows_core::VARIANT) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StringArrayToVariantArray)(windows_core::Interface::as_raw(self), core::mem::transmute(pvarstringarray), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn VariantArrayToStringArray(&self, pvarvariantarray: *const windows_core::VARIANT) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).VariantArrayToStringArray)(windows_core::Interface::as_raw(self), core::mem::transmute(pvarvariantarray), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IBinaryConverter2_Vtbl {
    pub base__: IBinaryConverter_Vtbl,
    pub StringArrayToVariantArray: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub VariantArrayToStringArray: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICEnroll, ICEnroll_Vtbl, 0x43f8f288_7a20_11d0_8f06_00c04fc295e1);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICEnroll {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICEnroll, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ICEnroll {
    pub unsafe fn createFilePKCS10<P0, P1, P2>(&self, dnname: P0, usage: P1, wszpkcs10filename: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).createFilePKCS10)(windows_core::Interface::as_raw(self), dnname.param().abi(), usage.param().abi(), wszpkcs10filename.param().abi()).ok()
    }
    pub unsafe fn acceptFilePKCS7<P0>(&self, wszpkcs7filename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).acceptFilePKCS7)(windows_core::Interface::as_raw(self), wszpkcs7filename.param().abi()).ok()
    }
    pub unsafe fn createPKCS10<P0, P1>(&self, dnname: P0, usage: P1) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).createPKCS10)(windows_core::Interface::as_raw(self), dnname.param().abi(), usage.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn acceptPKCS7<P0>(&self, pkcs7: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).acceptPKCS7)(windows_core::Interface::as_raw(self), pkcs7.param().abi()).ok()
    }
    pub unsafe fn getCertFromPKCS7<P0>(&self, wszpkcs7: P0) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).getCertFromPKCS7)(windows_core::Interface::as_raw(self), wszpkcs7.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn enumProviders(&self, dwindex: i32, dwflags: i32) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).enumProviders)(windows_core::Interface::as_raw(self), dwindex, dwflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn enumContainers(&self, dwindex: i32) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).enumContainers)(windows_core::Interface::as_raw(self), dwindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn freeRequestInfo<P0>(&self, pkcs7orpkcs10: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).freeRequestInfo)(windows_core::Interface::as_raw(self), pkcs7orpkcs10.param().abi()).ok()
    }
    pub unsafe fn MyStoreName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MyStoreName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetMyStoreName<P0>(&self, bstrname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetMyStoreName)(windows_core::Interface::as_raw(self), bstrname.param().abi()).ok()
    }
    pub unsafe fn MyStoreType(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MyStoreType)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetMyStoreType<P0>(&self, bstrtype: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetMyStoreType)(windows_core::Interface::as_raw(self), bstrtype.param().abi()).ok()
    }
    pub unsafe fn MyStoreFlags(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MyStoreFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMyStoreFlags(&self, dwflags: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMyStoreFlags)(windows_core::Interface::as_raw(self), dwflags).ok()
    }
    pub unsafe fn CAStoreName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CAStoreName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetCAStoreName<P0>(&self, bstrname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetCAStoreName)(windows_core::Interface::as_raw(self), bstrname.param().abi()).ok()
    }
    pub unsafe fn CAStoreType(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CAStoreType)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetCAStoreType<P0>(&self, bstrtype: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetCAStoreType)(windows_core::Interface::as_raw(self), bstrtype.param().abi()).ok()
    }
    pub unsafe fn CAStoreFlags(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CAStoreFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetCAStoreFlags(&self, dwflags: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetCAStoreFlags)(windows_core::Interface::as_raw(self), dwflags).ok()
    }
    pub unsafe fn RootStoreName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RootStoreName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetRootStoreName<P0>(&self, bstrname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetRootStoreName)(windows_core::Interface::as_raw(self), bstrname.param().abi()).ok()
    }
    pub unsafe fn RootStoreType(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RootStoreType)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetRootStoreType<P0>(&self, bstrtype: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetRootStoreType)(windows_core::Interface::as_raw(self), bstrtype.param().abi()).ok()
    }
    pub unsafe fn RootStoreFlags(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RootStoreFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetRootStoreFlags(&self, dwflags: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetRootStoreFlags)(windows_core::Interface::as_raw(self), dwflags).ok()
    }
    pub unsafe fn RequestStoreName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RequestStoreName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetRequestStoreName<P0>(&self, bstrname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetRequestStoreName)(windows_core::Interface::as_raw(self), bstrname.param().abi()).ok()
    }
    pub unsafe fn RequestStoreType(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RequestStoreType)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetRequestStoreType<P0>(&self, bstrtype: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetRequestStoreType)(windows_core::Interface::as_raw(self), bstrtype.param().abi()).ok()
    }
    pub unsafe fn RequestStoreFlags(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RequestStoreFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetRequestStoreFlags(&self, dwflags: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetRequestStoreFlags)(windows_core::Interface::as_raw(self), dwflags).ok()
    }
    pub unsafe fn ContainerName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ContainerName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetContainerName<P0>(&self, bstrcontainer: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetContainerName)(windows_core::Interface::as_raw(self), bstrcontainer.param().abi()).ok()
    }
    pub unsafe fn ProviderName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ProviderName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetProviderName<P0>(&self, bstrprovider: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetProviderName)(windows_core::Interface::as_raw(self), bstrprovider.param().abi()).ok()
    }
    pub unsafe fn ProviderType(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ProviderType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetProviderType(&self, dwtype: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetProviderType)(windows_core::Interface::as_raw(self), dwtype).ok()
    }
    pub unsafe fn KeySpec(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).KeySpec)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetKeySpec(&self, dw: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetKeySpec)(windows_core::Interface::as_raw(self), dw).ok()
    }
    pub unsafe fn ProviderFlags(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ProviderFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetProviderFlags(&self, dwflags: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetProviderFlags)(windows_core::Interface::as_raw(self), dwflags).ok()
    }
    pub unsafe fn UseExistingKeySet(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UseExistingKeySet)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetUseExistingKeySet<P0>(&self, fuseexistingkeys: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetUseExistingKeySet)(windows_core::Interface::as_raw(self), fuseexistingkeys.param().abi()).ok()
    }
    pub unsafe fn GenKeyFlags(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GenKeyFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetGenKeyFlags(&self, dwflags: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetGenKeyFlags)(windows_core::Interface::as_raw(self), dwflags).ok()
    }
    pub unsafe fn DeleteRequestCert(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DeleteRequestCert)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetDeleteRequestCert<P0>(&self, fdelete: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetDeleteRequestCert)(windows_core::Interface::as_raw(self), fdelete.param().abi()).ok()
    }
    pub unsafe fn WriteCertToCSP(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).WriteCertToCSP)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetWriteCertToCSP<P0>(&self, fbool: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetWriteCertToCSP)(windows_core::Interface::as_raw(self), fbool.param().abi()).ok()
    }
    pub unsafe fn SPCFileName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SPCFileName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetSPCFileName<P0>(&self, bstr: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetSPCFileName)(windows_core::Interface::as_raw(self), bstr.param().abi()).ok()
    }
    pub unsafe fn PVKFileName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PVKFileName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetPVKFileName<P0>(&self, bstr: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetPVKFileName)(windows_core::Interface::as_raw(self), bstr.param().abi()).ok()
    }
    pub unsafe fn HashAlgorithm(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).HashAlgorithm)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetHashAlgorithm<P0>(&self, bstr: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetHashAlgorithm)(windows_core::Interface::as_raw(self), bstr.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICEnroll_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub createFilePKCS10: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub acceptFilePKCS7: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub createPKCS10: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub acceptPKCS7: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub getCertFromPKCS7: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub enumProviders: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub enumContainers: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub freeRequestInfo: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub MyStoreName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetMyStoreName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub MyStoreType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetMyStoreType: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub MyStoreFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMyStoreFlags: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub CAStoreName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetCAStoreName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CAStoreType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetCAStoreType: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CAStoreFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetCAStoreFlags: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub RootStoreName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetRootStoreName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub RootStoreType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetRootStoreType: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub RootStoreFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetRootStoreFlags: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub RequestStoreName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetRequestStoreName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub RequestStoreType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetRequestStoreType: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub RequestStoreFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetRequestStoreFlags: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ContainerName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetContainerName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ProviderName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetProviderName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ProviderType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetProviderType: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub KeySpec: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetKeySpec: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ProviderFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetProviderFlags: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub UseExistingKeySet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetUseExistingKeySet: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GenKeyFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetGenKeyFlags: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub DeleteRequestCert: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetDeleteRequestCert: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub WriteCertToCSP: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetWriteCertToCSP: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SPCFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetSPCFileName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub PVKFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetPVKFileName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub HashAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetHashAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICEnroll2, ICEnroll2_Vtbl, 0x704ca730_c90b_11d1_9bec_00c04fc295e1);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICEnroll2 {
    type Target = ICEnroll;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICEnroll2, windows_core::IUnknown, super::super::super::System::Com::IDispatch, ICEnroll);
#[cfg(feature = "Win32_System_Com")]
impl ICEnroll2 {
    pub unsafe fn addCertTypeToRequest<P0>(&self, certtype: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).addCertTypeToRequest)(windows_core::Interface::as_raw(self), certtype.param().abi()).ok()
    }
    pub unsafe fn addNameValuePairToSignature<P0, P1>(&self, name: P0, value: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).addNameValuePairToSignature)(windows_core::Interface::as_raw(self), name.param().abi(), value.param().abi()).ok()
    }
    pub unsafe fn WriteCertToUserDS(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).WriteCertToUserDS)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetWriteCertToUserDS<P0>(&self, fbool: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetWriteCertToUserDS)(windows_core::Interface::as_raw(self), fbool.param().abi()).ok()
    }
    pub unsafe fn EnableT61DNEncoding(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnableT61DNEncoding)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetEnableT61DNEncoding<P0>(&self, fbool: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetEnableT61DNEncoding)(windows_core::Interface::as_raw(self), fbool.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICEnroll2_Vtbl {
    pub base__: ICEnroll_Vtbl,
    pub addCertTypeToRequest: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub addNameValuePairToSignature: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub WriteCertToUserDS: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetWriteCertToUserDS: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub EnableT61DNEncoding: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetEnableT61DNEncoding: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICEnroll3, ICEnroll3_Vtbl, 0xc28c2d95_b7de_11d2_a421_00c04f79fe8e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICEnroll3 {
    type Target = ICEnroll2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICEnroll3, windows_core::IUnknown, super::super::super::System::Com::IDispatch, ICEnroll, ICEnroll2);
#[cfg(feature = "Win32_System_Com")]
impl ICEnroll3 {
    pub unsafe fn InstallPKCS7<P0>(&self, pkcs7: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).InstallPKCS7)(windows_core::Interface::as_raw(self), pkcs7.param().abi()).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetSupportedKeySpec(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSupportedKeySpec)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetKeyLen<P0, P1>(&self, fmin: P0, fexchange: P1) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<super::super::super::Foundation::BOOL>,
        P1: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetKeyLen)(windows_core::Interface::as_raw(self), fmin.param().abi(), fexchange.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn EnumAlgs(&self, dwindex: i32, algclass: i32) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumAlgs)(windows_core::Interface::as_raw(self), dwindex, algclass, &mut result__).map(|| result__)
    }
    pub unsafe fn GetAlgName(&self, algid: i32) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAlgName)(windows_core::Interface::as_raw(self), algid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetReuseHardwareKeyIfUnableToGenNew<P0>(&self, freusehardwarekeyifunabletogennew: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetReuseHardwareKeyIfUnableToGenNew)(windows_core::Interface::as_raw(self), freusehardwarekeyifunabletogennew.param().abi()).ok()
    }
    pub unsafe fn ReuseHardwareKeyIfUnableToGenNew(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ReuseHardwareKeyIfUnableToGenNew)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetHashAlgID(&self, hashalgid: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetHashAlgID)(windows_core::Interface::as_raw(self), hashalgid).ok()
    }
    pub unsafe fn HashAlgID(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).HashAlgID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetLimitExchangeKeyToEncipherment<P0>(&self, flimitexchangekeytoencipherment: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetLimitExchangeKeyToEncipherment)(windows_core::Interface::as_raw(self), flimitexchangekeytoencipherment.param().abi()).ok()
    }
    pub unsafe fn LimitExchangeKeyToEncipherment(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LimitExchangeKeyToEncipherment)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetEnableSMIMECapabilities<P0>(&self, fenablesmimecapabilities: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetEnableSMIMECapabilities)(windows_core::Interface::as_raw(self), fenablesmimecapabilities.param().abi()).ok()
    }
    pub unsafe fn EnableSMIMECapabilities(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnableSMIMECapabilities)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICEnroll3_Vtbl {
    pub base__: ICEnroll2_Vtbl,
    pub InstallPKCS7: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSupportedKeySpec: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetKeyLen: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::BOOL, super::super::super::Foundation::BOOL, *mut i32) -> windows_core::HRESULT,
    pub EnumAlgs: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut i32) -> windows_core::HRESULT,
    pub GetAlgName: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetReuseHardwareKeyIfUnableToGenNew: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub ReuseHardwareKeyIfUnableToGenNew: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetHashAlgID: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub HashAlgID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetLimitExchangeKeyToEncipherment: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub LimitExchangeKeyToEncipherment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetEnableSMIMECapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub EnableSMIMECapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICEnroll4, ICEnroll4_Vtbl, 0xc1f1188a_2eb5_4a80_841b_7e729a356d90);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICEnroll4 {
    type Target = ICEnroll3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICEnroll4, windows_core::IUnknown, super::super::super::System::Com::IDispatch, ICEnroll, ICEnroll2, ICEnroll3);
#[cfg(feature = "Win32_System_Com")]
impl ICEnroll4 {
    pub unsafe fn SetPrivateKeyArchiveCertificate<P0>(&self, bstrcert: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetPrivateKeyArchiveCertificate)(windows_core::Interface::as_raw(self), bstrcert.param().abi()).ok()
    }
    pub unsafe fn PrivateKeyArchiveCertificate(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PrivateKeyArchiveCertificate)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetThumbPrint<P0>(&self, bstrthumbprint: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetThumbPrint)(windows_core::Interface::as_raw(self), bstrthumbprint.param().abi()).ok()
    }
    pub unsafe fn ThumbPrint(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ThumbPrint)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn binaryToString<P0>(&self, flags: i32, strbinary: P0) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).binaryToString)(windows_core::Interface::as_raw(self), flags, strbinary.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn stringToBinary<P0>(&self, flags: i32, strencoded: P0) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).stringToBinary)(windows_core::Interface::as_raw(self), flags, strencoded.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn addExtensionToRequest<P0, P1>(&self, flags: i32, strname: P0, strvalue: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).addExtensionToRequest)(windows_core::Interface::as_raw(self), flags, strname.param().abi(), strvalue.param().abi()).ok()
    }
    pub unsafe fn addAttributeToRequest<P0, P1>(&self, flags: i32, strname: P0, strvalue: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).addAttributeToRequest)(windows_core::Interface::as_raw(self), flags, strname.param().abi(), strvalue.param().abi()).ok()
    }
    pub unsafe fn addNameValuePairToRequest<P0, P1>(&self, flags: i32, strname: P0, strvalue: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).addNameValuePairToRequest)(windows_core::Interface::as_raw(self), flags, strname.param().abi(), strvalue.param().abi()).ok()
    }
    pub unsafe fn resetExtensions(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).resetExtensions)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn resetAttributes(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).resetAttributes)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn createRequest<P0, P1>(&self, flags: CERT_CREATE_REQUEST_FLAGS, strdnname: P0, usage: P1) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).createRequest)(windows_core::Interface::as_raw(self), flags, strdnname.param().abi(), usage.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn createFileRequest<P0, P1, P2>(&self, flags: CERT_CREATE_REQUEST_FLAGS, strdnname: P0, strusage: P1, strrequestfilename: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).createFileRequest)(windows_core::Interface::as_raw(self), flags, strdnname.param().abi(), strusage.param().abi(), strrequestfilename.param().abi()).ok()
    }
    pub unsafe fn acceptResponse<P0>(&self, strresponse: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).acceptResponse)(windows_core::Interface::as_raw(self), strresponse.param().abi()).ok()
    }
    pub unsafe fn acceptFileResponse<P0>(&self, strresponsefilename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).acceptFileResponse)(windows_core::Interface::as_raw(self), strresponsefilename.param().abi()).ok()
    }
    pub unsafe fn getCertFromResponse<P0>(&self, strresponse: P0) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).getCertFromResponse)(windows_core::Interface::as_raw(self), strresponse.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn getCertFromFileResponse<P0>(&self, strresponsefilename: P0) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).getCertFromFileResponse)(windows_core::Interface::as_raw(self), strresponsefilename.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn createPFX<P0>(&self, strpassword: P0) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).createPFX)(windows_core::Interface::as_raw(self), strpassword.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn createFilePFX<P0, P1>(&self, strpassword: P0, strpfxfilename: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).createFilePFX)(windows_core::Interface::as_raw(self), strpassword.param().abi(), strpfxfilename.param().abi()).ok()
    }
    pub unsafe fn setPendingRequestInfo<P0, P1, P2>(&self, lrequestid: i32, strcadns: P0, strcaname: P1, strfriendlyname: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).setPendingRequestInfo)(windows_core::Interface::as_raw(self), lrequestid, strcadns.param().abi(), strcaname.param().abi(), strfriendlyname.param().abi()).ok()
    }
    pub unsafe fn enumPendingRequest(&self, lindex: i32, ldesiredproperty: PENDING_REQUEST_DESIRED_PROPERTY) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).enumPendingRequest)(windows_core::Interface::as_raw(self), lindex, ldesiredproperty, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn removePendingRequest<P0>(&self, strthumbprint: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).removePendingRequest)(windows_core::Interface::as_raw(self), strthumbprint.param().abi()).ok()
    }
    pub unsafe fn GetKeyLenEx(&self, lsizespec: XEKL_KEYSIZE, lkeyspec: XEKL_KEYSPEC) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetKeyLenEx)(windows_core::Interface::as_raw(self), lsizespec, lkeyspec, &mut result__).map(|| result__)
    }
    pub unsafe fn InstallPKCS7Ex<P0>(&self, pkcs7: P0) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).InstallPKCS7Ex)(windows_core::Interface::as_raw(self), pkcs7.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn addCertTypeToRequestEx<P0, P1>(&self, ltype: ADDED_CERT_TYPE, bstroidorname: P0, lmajorversion: i32, fminorversion: P1, lminorversion: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).addCertTypeToRequestEx)(windows_core::Interface::as_raw(self), ltype, bstroidorname.param().abi(), lmajorversion, fminorversion.param().abi(), lminorversion).ok()
    }
    pub unsafe fn getProviderType<P0>(&self, strprovname: P0) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).getProviderType)(windows_core::Interface::as_raw(self), strprovname.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn SetSignerCertificate<P0>(&self, bstrcert: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetSignerCertificate)(windows_core::Interface::as_raw(self), bstrcert.param().abi()).ok()
    }
    pub unsafe fn SetClientId(&self, lclientid: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetClientId)(windows_core::Interface::as_raw(self), lclientid).ok()
    }
    pub unsafe fn ClientId(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ClientId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn addBlobPropertyToCertificate<P0>(&self, lpropertyid: i32, lreserved: i32, bstrproperty: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).addBlobPropertyToCertificate)(windows_core::Interface::as_raw(self), lpropertyid, lreserved, bstrproperty.param().abi()).ok()
    }
    pub unsafe fn resetBlobProperties(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).resetBlobProperties)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetIncludeSubjectKeyID<P0>(&self, finclude: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetIncludeSubjectKeyID)(windows_core::Interface::as_raw(self), finclude.param().abi()).ok()
    }
    pub unsafe fn IncludeSubjectKeyID(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IncludeSubjectKeyID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICEnroll4_Vtbl {
    pub base__: ICEnroll3_Vtbl,
    pub SetPrivateKeyArchiveCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub PrivateKeyArchiveCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetThumbPrint: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ThumbPrint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub binaryToString: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub stringToBinary: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub addExtensionToRequest: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub addAttributeToRequest: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub addNameValuePairToRequest: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub resetExtensions: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub resetAttributes: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub createRequest: unsafe extern "system" fn(*mut core::ffi::c_void, CERT_CREATE_REQUEST_FLAGS, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub createFileRequest: unsafe extern "system" fn(*mut core::ffi::c_void, CERT_CREATE_REQUEST_FLAGS, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub acceptResponse: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub acceptFileResponse: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub getCertFromResponse: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub getCertFromFileResponse: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub createPFX: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub createFilePFX: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub setPendingRequestInfo: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub enumPendingRequest: unsafe extern "system" fn(*mut core::ffi::c_void, i32, PENDING_REQUEST_DESIRED_PROPERTY, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub removePendingRequest: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetKeyLenEx: unsafe extern "system" fn(*mut core::ffi::c_void, XEKL_KEYSIZE, XEKL_KEYSPEC, *mut i32) -> windows_core::HRESULT,
    pub InstallPKCS7Ex: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut i32) -> windows_core::HRESULT,
    pub addCertTypeToRequestEx: unsafe extern "system" fn(*mut core::ffi::c_void, ADDED_CERT_TYPE, core::mem::MaybeUninit<windows_core::BSTR>, i32, super::super::super::Foundation::BOOL, i32) -> windows_core::HRESULT,
    pub getProviderType: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut i32) -> windows_core::HRESULT,
    pub SetSignerCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetClientId: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ClientId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub addBlobPropertyToCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub resetBlobProperties: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetIncludeSubjectKeyID: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub IncludeSubjectKeyID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICertAdmin, ICertAdmin_Vtbl, 0x34df6950_7fb6_11d0_8817_00a0c903b83c);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICertAdmin {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICertAdmin, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ICertAdmin {
    pub unsafe fn IsValidCertificate<P0, P1>(&self, strconfig: P0, strserialnumber: P1) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsValidCertificate)(windows_core::Interface::as_raw(self), strconfig.param().abi(), strserialnumber.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn GetRevocationReason(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRevocationReason)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn RevokeCertificate<P0, P1>(&self, strconfig: P0, strserialnumber: P1, reason: i32, date: f64) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).RevokeCertificate)(windows_core::Interface::as_raw(self), strconfig.param().abi(), strserialnumber.param().abi(), reason, date).ok()
    }
    pub unsafe fn SetRequestAttributes<P0, P1>(&self, strconfig: P0, requestid: i32, strattributes: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetRequestAttributes)(windows_core::Interface::as_raw(self), strconfig.param().abi(), requestid, strattributes.param().abi()).ok()
    }
    pub unsafe fn SetCertificateExtension<P0, P1>(&self, strconfig: P0, requestid: i32, strextensionname: P1, r#type: CERT_PROPERTY_TYPE, flags: i32, pvarvalue: *const windows_core::VARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetCertificateExtension)(windows_core::Interface::as_raw(self), strconfig.param().abi(), requestid, strextensionname.param().abi(), r#type, flags, core::mem::transmute(pvarvalue)).ok()
    }
    pub unsafe fn DenyRequest<P0>(&self, strconfig: P0, requestid: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).DenyRequest)(windows_core::Interface::as_raw(self), strconfig.param().abi(), requestid).ok()
    }
    pub unsafe fn ResubmitRequest<P0>(&self, strconfig: P0, requestid: i32) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ResubmitRequest)(windows_core::Interface::as_raw(self), strconfig.param().abi(), requestid, &mut result__).map(|| result__)
    }
    pub unsafe fn PublishCRL<P0>(&self, strconfig: P0, date: f64) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).PublishCRL)(windows_core::Interface::as_raw(self), strconfig.param().abi(), date).ok()
    }
    pub unsafe fn GetCRL<P0>(&self, strconfig: P0, flags: i32) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCRL)(windows_core::Interface::as_raw(self), strconfig.param().abi(), flags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ImportCertificate<P0, P1>(&self, strconfig: P0, strcertificate: P1, flags: CERT_IMPORT_FLAGS) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ImportCertificate)(windows_core::Interface::as_raw(self), strconfig.param().abi(), strcertificate.param().abi(), flags, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertAdmin_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub IsValidCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *mut i32) -> windows_core::HRESULT,
    pub GetRevocationReason: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub RevokeCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, i32, f64) -> windows_core::HRESULT,
    pub SetRequestAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetCertificateExtension: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, core::mem::MaybeUninit<windows_core::BSTR>, CERT_PROPERTY_TYPE, i32, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub DenyRequest: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32) -> windows_core::HRESULT,
    pub ResubmitRequest: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, *mut i32) -> windows_core::HRESULT,
    pub PublishCRL: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, f64) -> windows_core::HRESULT,
    pub GetCRL: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ImportCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, CERT_IMPORT_FLAGS, *mut i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICertAdmin2, ICertAdmin2_Vtbl, 0xf7c3ac41_b8ce_4fb4_aa58_3d1dc0e36b39);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICertAdmin2 {
    type Target = ICertAdmin;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICertAdmin2, windows_core::IUnknown, super::super::super::System::Com::IDispatch, ICertAdmin);
#[cfg(feature = "Win32_System_Com")]
impl ICertAdmin2 {
    pub unsafe fn PublishCRLs<P0>(&self, strconfig: P0, date: f64, crlflags: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).PublishCRLs)(windows_core::Interface::as_raw(self), strconfig.param().abi(), date, crlflags).ok()
    }
    pub unsafe fn GetCAProperty<P0>(&self, strconfig: P0, propid: i32, propindex: i32, proptype: i32, flags: i32) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCAProperty)(windows_core::Interface::as_raw(self), strconfig.param().abi(), propid, propindex, proptype, flags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetCAProperty<P0>(&self, strconfig: P0, propid: i32, propindex: i32, proptype: CERT_PROPERTY_TYPE, pvarpropertyvalue: *const windows_core::VARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetCAProperty)(windows_core::Interface::as_raw(self), strconfig.param().abi(), propid, propindex, proptype, core::mem::transmute(pvarpropertyvalue)).ok()
    }
    pub unsafe fn GetCAPropertyFlags<P0>(&self, strconfig: P0, propid: i32) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCAPropertyFlags)(windows_core::Interface::as_raw(self), strconfig.param().abi(), propid, &mut result__).map(|| result__)
    }
    pub unsafe fn GetCAPropertyDisplayName<P0>(&self, strconfig: P0, propid: i32) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCAPropertyDisplayName)(windows_core::Interface::as_raw(self), strconfig.param().abi(), propid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetArchivedKey<P0>(&self, strconfig: P0, requestid: i32, flags: i32) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetArchivedKey)(windows_core::Interface::as_raw(self), strconfig.param().abi(), requestid, flags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetConfigEntry<P0, P1, P2>(&self, strconfig: P0, strnodepath: P1, strentryname: P2) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetConfigEntry)(windows_core::Interface::as_raw(self), strconfig.param().abi(), strnodepath.param().abi(), strentryname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetConfigEntry<P0, P1, P2>(&self, strconfig: P0, strnodepath: P1, strentryname: P2, pvarentry: *const windows_core::VARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetConfigEntry)(windows_core::Interface::as_raw(self), strconfig.param().abi(), strnodepath.param().abi(), strentryname.param().abi(), core::mem::transmute(pvarentry)).ok()
    }
    pub unsafe fn ImportKey<P0, P1, P2>(&self, strconfig: P0, requestid: i32, strcerthash: P1, flags: CERT_IMPORT_FLAGS, strkey: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).ImportKey)(windows_core::Interface::as_raw(self), strconfig.param().abi(), requestid, strcerthash.param().abi(), flags, strkey.param().abi()).ok()
    }
    pub unsafe fn GetMyRoles<P0>(&self, strconfig: P0) -> windows_core::Result<CERTADMIN_GET_ROLES_FLAGS>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMyRoles)(windows_core::Interface::as_raw(self), strconfig.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn DeleteRow<P0>(&self, strconfig: P0, flags: CERT_DELETE_ROW_FLAGS, date: f64, table: CVRC_TABLE, rowid: i32) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DeleteRow)(windows_core::Interface::as_raw(self), strconfig.param().abi(), flags, date, table, rowid, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertAdmin2_Vtbl {
    pub base__: ICertAdmin_Vtbl,
    pub PublishCRLs: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, f64, i32) -> windows_core::HRESULT,
    pub GetCAProperty: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, i32, i32, i32, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetCAProperty: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, i32, CERT_PROPERTY_TYPE, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub GetCAPropertyFlags: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, *mut i32) -> windows_core::HRESULT,
    pub GetCAPropertyDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetArchivedKey: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, i32, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetConfigEntry: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetConfigEntry: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub ImportKey: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, core::mem::MaybeUninit<windows_core::BSTR>, CERT_IMPORT_FLAGS, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetMyRoles: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut CERTADMIN_GET_ROLES_FLAGS) -> windows_core::HRESULT,
    pub DeleteRow: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, CERT_DELETE_ROW_FLAGS, f64, CVRC_TABLE, i32, *mut i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICertConfig, ICertConfig_Vtbl, 0x372fce34_4324_11d0_8810_00a0c903b83c);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICertConfig {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICertConfig, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ICertConfig {
    pub unsafe fn Reset(&self, index: i32) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| result__)
    }
    pub unsafe fn Next(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetField<P0>(&self, strfieldname: P0) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetField)(windows_core::Interface::as_raw(self), strfieldname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetConfig(&self, flags: i32) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetConfig)(windows_core::Interface::as_raw(self), flags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertConfig_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetField: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetConfig: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICertConfig2, ICertConfig2_Vtbl, 0x7a18edde_7e78_4163_8ded_78e2c9cee924);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICertConfig2 {
    type Target = ICertConfig;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICertConfig2, windows_core::IUnknown, super::super::super::System::Com::IDispatch, ICertConfig);
#[cfg(feature = "Win32_System_Com")]
impl ICertConfig2 {
    pub unsafe fn SetSharedFolder<P0>(&self, strsharedfolder: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetSharedFolder)(windows_core::Interface::as_raw(self), strsharedfolder.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertConfig2_Vtbl {
    pub base__: ICertConfig_Vtbl,
    pub SetSharedFolder: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICertEncodeAltName, ICertEncodeAltName_Vtbl, 0x1c9a8c70_1271_11d1_9bd4_00c04fb683fa);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICertEncodeAltName {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICertEncodeAltName, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ICertEncodeAltName {
    pub unsafe fn Decode<P0>(&self, strbinary: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Decode)(windows_core::Interface::as_raw(self), strbinary.param().abi()).ok()
    }
    pub unsafe fn GetNameCount(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNameCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetNameChoice(&self, nameindex: i32) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNameChoice)(windows_core::Interface::as_raw(self), nameindex, &mut result__).map(|| result__)
    }
    pub unsafe fn GetName(&self, nameindex: i32) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), nameindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Reset(&self, namecount: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self), namecount).ok()
    }
    pub unsafe fn SetNameEntry<P0>(&self, nameindex: i32, namechoice: CERT_ALT_NAME, strname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetNameEntry)(windows_core::Interface::as_raw(self), nameindex, namechoice, strname.param().abi()).ok()
    }
    pub unsafe fn Encode(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Encode)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertEncodeAltName_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub Decode: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetNameCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetNameChoice: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SetNameEntry: unsafe extern "system" fn(*mut core::ffi::c_void, i32, CERT_ALT_NAME, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Encode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICertEncodeAltName2, ICertEncodeAltName2_Vtbl, 0xf67fe177_5ef1_4535_b4ce_29df15e2e0c3);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICertEncodeAltName2 {
    type Target = ICertEncodeAltName;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICertEncodeAltName2, windows_core::IUnknown, super::super::super::System::Com::IDispatch, ICertEncodeAltName);
#[cfg(feature = "Win32_System_Com")]
impl ICertEncodeAltName2 {
    pub unsafe fn DecodeBlob<P0>(&self, strencodeddata: P0, encoding: EncodingType) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).DecodeBlob)(windows_core::Interface::as_raw(self), strencodeddata.param().abi(), encoding).ok()
    }
    pub unsafe fn EncodeBlob(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EncodeBlob)(windows_core::Interface::as_raw(self), encoding, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetNameBlob(&self, nameindex: i32, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNameBlob)(windows_core::Interface::as_raw(self), nameindex, encoding, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetNameEntryBlob<P0>(&self, nameindex: i32, namechoice: i32, strname: P0, encoding: EncodingType) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetNameEntryBlob)(windows_core::Interface::as_raw(self), nameindex, namechoice, strname.param().abi(), encoding).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertEncodeAltName2_Vtbl {
    pub base__: ICertEncodeAltName_Vtbl,
    pub DecodeBlob: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, EncodingType) -> windows_core::HRESULT,
    pub EncodeBlob: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetNameBlob: unsafe extern "system" fn(*mut core::ffi::c_void, i32, EncodingType, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetNameEntryBlob: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, core::mem::MaybeUninit<windows_core::BSTR>, EncodingType) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICertEncodeBitString, ICertEncodeBitString_Vtbl, 0x6db525be_1278_11d1_9bd4_00c04fb683fa);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICertEncodeBitString {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICertEncodeBitString, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ICertEncodeBitString {
    pub unsafe fn Decode<P0>(&self, strbinary: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Decode)(windows_core::Interface::as_raw(self), strbinary.param().abi()).ok()
    }
    pub unsafe fn GetBitCount(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetBitCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetBitString(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetBitString)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Encode<P0>(&self, bitcount: i32, strbitstring: P0) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Encode)(windows_core::Interface::as_raw(self), bitcount, strbitstring.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertEncodeBitString_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub Decode: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetBitCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetBitString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Encode: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICertEncodeBitString2, ICertEncodeBitString2_Vtbl, 0xe070d6e7_23ef_4dd2_8242_ebd9c928cb30);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICertEncodeBitString2 {
    type Target = ICertEncodeBitString;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICertEncodeBitString2, windows_core::IUnknown, super::super::super::System::Com::IDispatch, ICertEncodeBitString);
#[cfg(feature = "Win32_System_Com")]
impl ICertEncodeBitString2 {
    pub unsafe fn DecodeBlob<P0>(&self, strencodeddata: P0, encoding: EncodingType) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).DecodeBlob)(windows_core::Interface::as_raw(self), strencodeddata.param().abi(), encoding).ok()
    }
    pub unsafe fn EncodeBlob<P0>(&self, bitcount: i32, strbitstring: P0, encodingin: EncodingType, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EncodeBlob)(windows_core::Interface::as_raw(self), bitcount, strbitstring.param().abi(), encodingin, encoding, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetBitStringBlob(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetBitStringBlob)(windows_core::Interface::as_raw(self), encoding, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertEncodeBitString2_Vtbl {
    pub base__: ICertEncodeBitString_Vtbl,
    pub DecodeBlob: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, EncodingType) -> windows_core::HRESULT,
    pub EncodeBlob: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::BSTR>, EncodingType, EncodingType, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetBitStringBlob: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICertEncodeCRLDistInfo, ICertEncodeCRLDistInfo_Vtbl, 0x01958640_bbff_11d0_8825_00a0c903b83c);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICertEncodeCRLDistInfo {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICertEncodeCRLDistInfo, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ICertEncodeCRLDistInfo {
    pub unsafe fn Decode<P0>(&self, strbinary: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Decode)(windows_core::Interface::as_raw(self), strbinary.param().abi()).ok()
    }
    pub unsafe fn GetDistPointCount(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDistPointCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetNameCount(&self, distpointindex: i32) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNameCount)(windows_core::Interface::as_raw(self), distpointindex, &mut result__).map(|| result__)
    }
    pub unsafe fn GetNameChoice(&self, distpointindex: i32, nameindex: i32) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNameChoice)(windows_core::Interface::as_raw(self), distpointindex, nameindex, &mut result__).map(|| result__)
    }
    pub unsafe fn GetName(&self, distpointindex: i32, nameindex: i32) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), distpointindex, nameindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Reset(&self, distpointcount: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self), distpointcount).ok()
    }
    pub unsafe fn SetNameCount(&self, distpointindex: i32, namecount: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetNameCount)(windows_core::Interface::as_raw(self), distpointindex, namecount).ok()
    }
    pub unsafe fn SetNameEntry<P0>(&self, distpointindex: i32, nameindex: i32, namechoice: CERT_ALT_NAME, strname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetNameEntry)(windows_core::Interface::as_raw(self), distpointindex, nameindex, namechoice, strname.param().abi()).ok()
    }
    pub unsafe fn Encode(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Encode)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertEncodeCRLDistInfo_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub Decode: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetDistPointCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetNameCount: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
    pub GetNameChoice: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut i32) -> windows_core::HRESULT,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SetNameCount: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub SetNameEntry: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, CERT_ALT_NAME, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Encode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICertEncodeCRLDistInfo2, ICertEncodeCRLDistInfo2_Vtbl, 0xb4275d4b_3e30_446f_ad36_09d03120b078);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICertEncodeCRLDistInfo2 {
    type Target = ICertEncodeCRLDistInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICertEncodeCRLDistInfo2, windows_core::IUnknown, super::super::super::System::Com::IDispatch, ICertEncodeCRLDistInfo);
#[cfg(feature = "Win32_System_Com")]
impl ICertEncodeCRLDistInfo2 {
    pub unsafe fn DecodeBlob<P0>(&self, strencodeddata: P0, encoding: EncodingType) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).DecodeBlob)(windows_core::Interface::as_raw(self), strencodeddata.param().abi(), encoding).ok()
    }
    pub unsafe fn EncodeBlob(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EncodeBlob)(windows_core::Interface::as_raw(self), encoding, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertEncodeCRLDistInfo2_Vtbl {
    pub base__: ICertEncodeCRLDistInfo_Vtbl,
    pub DecodeBlob: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, EncodingType) -> windows_core::HRESULT,
    pub EncodeBlob: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICertEncodeDateArray, ICertEncodeDateArray_Vtbl, 0x2f9469a0_a470_11d0_8821_00a0c903b83c);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICertEncodeDateArray {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICertEncodeDateArray, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ICertEncodeDateArray {
    pub unsafe fn Decode<P0>(&self, strbinary: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Decode)(windows_core::Interface::as_raw(self), strbinary.param().abi()).ok()
    }
    pub unsafe fn GetCount(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetValue(&self, index: i32) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetValue)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| result__)
    }
    pub unsafe fn Reset(&self, count: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self), count).ok()
    }
    pub unsafe fn SetValue(&self, index: i32, value: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), index, value).ok()
    }
    pub unsafe fn Encode(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Encode)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertEncodeDateArray_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub Decode: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetValue: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut f64) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, i32, f64) -> windows_core::HRESULT,
    pub Encode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICertEncodeDateArray2, ICertEncodeDateArray2_Vtbl, 0x99a4edb5_2b8e_448d_bf95_bba8d7789dc8);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICertEncodeDateArray2 {
    type Target = ICertEncodeDateArray;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICertEncodeDateArray2, windows_core::IUnknown, super::super::super::System::Com::IDispatch, ICertEncodeDateArray);
#[cfg(feature = "Win32_System_Com")]
impl ICertEncodeDateArray2 {
    pub unsafe fn DecodeBlob<P0>(&self, strencodeddata: P0, encoding: EncodingType) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).DecodeBlob)(windows_core::Interface::as_raw(self), strencodeddata.param().abi(), encoding).ok()
    }
    pub unsafe fn EncodeBlob(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EncodeBlob)(windows_core::Interface::as_raw(self), encoding, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertEncodeDateArray2_Vtbl {
    pub base__: ICertEncodeDateArray_Vtbl,
    pub DecodeBlob: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, EncodingType) -> windows_core::HRESULT,
    pub EncodeBlob: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICertEncodeLongArray, ICertEncodeLongArray_Vtbl, 0x15e2f230_a0a2_11d0_8821_00a0c903b83c);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICertEncodeLongArray {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICertEncodeLongArray, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ICertEncodeLongArray {
    pub unsafe fn Decode<P0>(&self, strbinary: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Decode)(windows_core::Interface::as_raw(self), strbinary.param().abi()).ok()
    }
    pub unsafe fn GetCount(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetValue(&self, index: i32) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetValue)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| result__)
    }
    pub unsafe fn Reset(&self, count: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self), count).ok()
    }
    pub unsafe fn SetValue(&self, index: i32, value: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), index, value).ok()
    }
    pub unsafe fn Encode(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Encode)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertEncodeLongArray_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub Decode: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetValue: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub Encode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICertEncodeLongArray2, ICertEncodeLongArray2_Vtbl, 0x4efde84a_bd9b_4fc2_a108_c347d478840f);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICertEncodeLongArray2 {
    type Target = ICertEncodeLongArray;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICertEncodeLongArray2, windows_core::IUnknown, super::super::super::System::Com::IDispatch, ICertEncodeLongArray);
#[cfg(feature = "Win32_System_Com")]
impl ICertEncodeLongArray2 {
    pub unsafe fn DecodeBlob<P0>(&self, strencodeddata: P0, encoding: EncodingType) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).DecodeBlob)(windows_core::Interface::as_raw(self), strencodeddata.param().abi(), encoding).ok()
    }
    pub unsafe fn EncodeBlob(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EncodeBlob)(windows_core::Interface::as_raw(self), encoding, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertEncodeLongArray2_Vtbl {
    pub base__: ICertEncodeLongArray_Vtbl,
    pub DecodeBlob: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, EncodingType) -> windows_core::HRESULT,
    pub EncodeBlob: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICertEncodeStringArray, ICertEncodeStringArray_Vtbl, 0x12a88820_7494_11d0_8816_00a0c903b83c);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICertEncodeStringArray {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICertEncodeStringArray, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ICertEncodeStringArray {
    pub unsafe fn Decode<P0>(&self, strbinary: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Decode)(windows_core::Interface::as_raw(self), strbinary.param().abi()).ok()
    }
    pub unsafe fn GetStringType(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStringType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetCount(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetValue(&self, index: i32) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetValue)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Reset(&self, count: i32, stringtype: super::CERT_RDN_ATTR_VALUE_TYPE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self), count, stringtype).ok()
    }
    pub unsafe fn SetValue<P0>(&self, index: i32, str: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), index, str.param().abi()).ok()
    }
    pub unsafe fn Encode(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Encode)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertEncodeStringArray_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub Decode: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetStringType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetValue: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::CERT_RDN_ATTR_VALUE_TYPE) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Encode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICertEncodeStringArray2, ICertEncodeStringArray2_Vtbl, 0x9c680d93_9b7d_4e95_9018_4ffe10ba5ada);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICertEncodeStringArray2 {
    type Target = ICertEncodeStringArray;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICertEncodeStringArray2, windows_core::IUnknown, super::super::super::System::Com::IDispatch, ICertEncodeStringArray);
#[cfg(feature = "Win32_System_Com")]
impl ICertEncodeStringArray2 {
    pub unsafe fn DecodeBlob<P0>(&self, strencodeddata: P0, encoding: EncodingType) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).DecodeBlob)(windows_core::Interface::as_raw(self), strencodeddata.param().abi(), encoding).ok()
    }
    pub unsafe fn EncodeBlob(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EncodeBlob)(windows_core::Interface::as_raw(self), encoding, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertEncodeStringArray2_Vtbl {
    pub base__: ICertEncodeStringArray_Vtbl,
    pub DecodeBlob: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, EncodingType) -> windows_core::HRESULT,
    pub EncodeBlob: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICertExit, ICertExit_Vtbl, 0xe19ae1a0_7364_11d0_8816_00a0c903b83c);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICertExit {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICertExit, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ICertExit {
    pub unsafe fn Initialize<P0>(&self, strconfig: P0) -> windows_core::Result<CERT_EXIT_EVENT_MASK>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), strconfig.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn Notify(&self, exitevent: i32, context: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Notify)(windows_core::Interface::as_raw(self), exitevent, context).ok()
    }
    pub unsafe fn GetDescription(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDescription)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertExit_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut CERT_EXIT_EVENT_MASK) -> windows_core::HRESULT,
    pub Notify: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub GetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICertExit2, ICertExit2_Vtbl, 0x0abf484b_d049_464d_a7ed_552e7529b0ff);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICertExit2 {
    type Target = ICertExit;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICertExit2, windows_core::IUnknown, super::super::super::System::Com::IDispatch, ICertExit);
#[cfg(feature = "Win32_System_Com")]
impl ICertExit2 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetManageModule(&self) -> windows_core::Result<ICertManageModule> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetManageModule)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertExit2_Vtbl {
    pub base__: ICertExit_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetManageModule: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetManageModule: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICertGetConfig, ICertGetConfig_Vtbl, 0xc7ea09c0_ce17_11d0_8833_00a0c903b83c);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICertGetConfig {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICertGetConfig, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ICertGetConfig {
    pub unsafe fn GetConfig(&self, flags: CERT_GET_CONFIG_FLAGS) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetConfig)(windows_core::Interface::as_raw(self), flags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertGetConfig_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub GetConfig: unsafe extern "system" fn(*mut core::ffi::c_void, CERT_GET_CONFIG_FLAGS, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICertManageModule, ICertManageModule_Vtbl, 0xe7d7ad42_bd3d_11d1_9a4d_00c04fc297eb);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICertManageModule {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICertManageModule, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ICertManageModule {
    pub unsafe fn GetProperty<P0, P1, P2>(&self, strconfig: P0, strstoragelocation: P1, strpropertyname: P2, flags: i32) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), strconfig.param().abi(), strstoragelocation.param().abi(), strpropertyname.param().abi(), flags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetProperty<P0, P1, P2>(&self, strconfig: P0, strstoragelocation: P1, strpropertyname: P2, flags: i32, pvarproperty: *const windows_core::VARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), strconfig.param().abi(), strstoragelocation.param().abi(), strpropertyname.param().abi(), flags, core::mem::transmute(pvarproperty)).ok()
    }
    pub unsafe fn Configure<P0, P1>(&self, strconfig: P0, strstoragelocation: P1, flags: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Configure)(windows_core::Interface::as_raw(self), strconfig.param().abi(), strstoragelocation.param().abi(), flags).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertManageModule_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, i32, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, i32, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Configure: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICertPolicy, ICertPolicy_Vtbl, 0x38bb5a00_7636_11d0_b413_00a0c91bbf8c);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICertPolicy {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICertPolicy, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ICertPolicy {
    pub unsafe fn Initialize<P0>(&self, strconfig: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), strconfig.param().abi()).ok()
    }
    pub unsafe fn VerifyRequest<P0>(&self, strconfig: P0, context: i32, bnewrequest: i32, flags: i32) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).VerifyRequest)(windows_core::Interface::as_raw(self), strconfig.param().abi(), context, bnewrequest, flags, &mut result__).map(|| result__)
    }
    pub unsafe fn GetDescription(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDescription)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ShutDown(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ShutDown)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertPolicy_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub VerifyRequest: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, i32, i32, *mut i32) -> windows_core::HRESULT,
    pub GetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ShutDown: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICertPolicy2, ICertPolicy2_Vtbl, 0x3db4910e_8001_4bf1_aa1b_f43a808317a0);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICertPolicy2 {
    type Target = ICertPolicy;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICertPolicy2, windows_core::IUnknown, super::super::super::System::Com::IDispatch, ICertPolicy);
#[cfg(feature = "Win32_System_Com")]
impl ICertPolicy2 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetManageModule(&self) -> windows_core::Result<ICertManageModule> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetManageModule)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertPolicy2_Vtbl {
    pub base__: ICertPolicy_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetManageModule: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetManageModule: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICertProperties, ICertProperties_Vtbl, 0x728ab32f_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICertProperties {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICertProperties, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ICertProperties {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_ItemByIndex(&self, index: i32) -> windows_core::Result<ICertProperty> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_ItemByIndex)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Add<P0>(&self, pval: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ICertProperty>,
    {
        (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), pval.param().abi()).ok()
    }
    pub unsafe fn Remove(&self, index: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), index).ok()
    }
    pub unsafe fn Clear(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn InitializeFromCertificate<P0, P1>(&self, machinecontext: P0, encoding: EncodingType, strcertificate: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::VARIANT_BOOL>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).InitializeFromCertificate)(windows_core::Interface::as_raw(self), machinecontext.param().abi(), encoding, strcertificate.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertProperties_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub get_ItemByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_ItemByIndex: usize,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InitializeFromCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::VARIANT_BOOL, EncodingType, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICertProperty, ICertProperty_Vtbl, 0x728ab32e_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICertProperty {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICertProperty, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ICertProperty {
    pub unsafe fn InitializeFromCertificate<P0, P1>(&self, machinecontext: P0, encoding: EncodingType, strcertificate: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::VARIANT_BOOL>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).InitializeFromCertificate)(windows_core::Interface::as_raw(self), machinecontext.param().abi(), encoding, strcertificate.param().abi()).ok()
    }
    pub unsafe fn InitializeDecode<P0>(&self, encoding: EncodingType, strencodeddata: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).InitializeDecode)(windows_core::Interface::as_raw(self), encoding, strencodeddata.param().abi()).ok()
    }
    pub unsafe fn PropertyId(&self) -> windows_core::Result<CERTENROLL_PROPERTYID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PropertyId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetPropertyId(&self, value: CERTENROLL_PROPERTYID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPropertyId)(windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn get_RawData(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_RawData)(windows_core::Interface::as_raw(self), encoding, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RemoveFromCertificate<P0, P1>(&self, machinecontext: P0, encoding: EncodingType, strcertificate: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::VARIANT_BOOL>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).RemoveFromCertificate)(windows_core::Interface::as_raw(self), machinecontext.param().abi(), encoding, strcertificate.param().abi()).ok()
    }
    pub unsafe fn SetValueOnCertificate<P0, P1>(&self, machinecontext: P0, encoding: EncodingType, strcertificate: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::VARIANT_BOOL>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetValueOnCertificate)(windows_core::Interface::as_raw(self), machinecontext.param().abi(), encoding, strcertificate.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertProperty_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub InitializeFromCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::VARIANT_BOOL, EncodingType, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub InitializeDecode: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub PropertyId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CERTENROLL_PROPERTYID) -> windows_core::HRESULT,
    pub SetPropertyId: unsafe extern "system" fn(*mut core::ffi::c_void, CERTENROLL_PROPERTYID) -> windows_core::HRESULT,
    pub get_RawData: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub RemoveFromCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::VARIANT_BOOL, EncodingType, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetValueOnCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::VARIANT_BOOL, EncodingType, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICertPropertyArchived, ICertPropertyArchived_Vtbl, 0x728ab337_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICertPropertyArchived {
    type Target = ICertProperty;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICertPropertyArchived, windows_core::IUnknown, super::super::super::System::Com::IDispatch, ICertProperty);
#[cfg(feature = "Win32_System_Com")]
impl ICertPropertyArchived {
    pub unsafe fn Initialize<P0>(&self, archivedvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), archivedvalue.param().abi()).ok()
    }
    pub unsafe fn Archived(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Archived)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertPropertyArchived_Vtbl {
    pub base__: ICertProperty_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Archived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICertPropertyArchivedKeyHash, ICertPropertyArchivedKeyHash_Vtbl, 0x728ab33b_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICertPropertyArchivedKeyHash {
    type Target = ICertProperty;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICertPropertyArchivedKeyHash, windows_core::IUnknown, super::super::super::System::Com::IDispatch, ICertProperty);
#[cfg(feature = "Win32_System_Com")]
impl ICertPropertyArchivedKeyHash {
    pub unsafe fn Initialize<P0>(&self, encoding: EncodingType, strarchivedkeyhashvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), encoding, strarchivedkeyhashvalue.param().abi()).ok()
    }
    pub unsafe fn get_ArchivedKeyHash(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_ArchivedKeyHash)(windows_core::Interface::as_raw(self), encoding, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertPropertyArchivedKeyHash_Vtbl {
    pub base__: ICertProperty_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub get_ArchivedKeyHash: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICertPropertyAutoEnroll, ICertPropertyAutoEnroll_Vtbl, 0x728ab332_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICertPropertyAutoEnroll {
    type Target = ICertProperty;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICertPropertyAutoEnroll, windows_core::IUnknown, super::super::super::System::Com::IDispatch, ICertProperty);
#[cfg(feature = "Win32_System_Com")]
impl ICertPropertyAutoEnroll {
    pub unsafe fn Initialize<P0>(&self, strtemplatename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), strtemplatename.param().abi()).ok()
    }
    pub unsafe fn TemplateName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TemplateName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertPropertyAutoEnroll_Vtbl {
    pub base__: ICertProperty_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub TemplateName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICertPropertyBackedUp, ICertPropertyBackedUp_Vtbl, 0x728ab338_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICertPropertyBackedUp {
    type Target = ICertProperty;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICertPropertyBackedUp, windows_core::IUnknown, super::super::super::System::Com::IDispatch, ICertProperty);
#[cfg(feature = "Win32_System_Com")]
impl ICertPropertyBackedUp {
    pub unsafe fn InitializeFromCurrentTime<P0>(&self, backedupvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).InitializeFromCurrentTime)(windows_core::Interface::as_raw(self), backedupvalue.param().abi()).ok()
    }
    pub unsafe fn Initialize<P0>(&self, backedupvalue: P0, date: f64) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), backedupvalue.param().abi(), date).ok()
    }
    pub unsafe fn BackedUpValue(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BackedUpValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn BackedUpTime(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BackedUpTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertPropertyBackedUp_Vtbl {
    pub base__: ICertProperty_Vtbl,
    pub InitializeFromCurrentTime: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::VARIANT_BOOL, f64) -> windows_core::HRESULT,
    pub BackedUpValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub BackedUpTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICertPropertyDescription, ICertPropertyDescription_Vtbl, 0x728ab331_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICertPropertyDescription {
    type Target = ICertProperty;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICertPropertyDescription, windows_core::IUnknown, super::super::super::System::Com::IDispatch, ICertProperty);
#[cfg(feature = "Win32_System_Com")]
impl ICertPropertyDescription {
    pub unsafe fn Initialize<P0>(&self, strdescription: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), strdescription.param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertPropertyDescription_Vtbl {
    pub base__: ICertProperty_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICertPropertyEnrollment, ICertPropertyEnrollment_Vtbl, 0x728ab339_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICertPropertyEnrollment {
    type Target = ICertProperty;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICertPropertyEnrollment, windows_core::IUnknown, super::super::super::System::Com::IDispatch, ICertProperty);
#[cfg(feature = "Win32_System_Com")]
impl ICertPropertyEnrollment {
    pub unsafe fn Initialize<P0, P1, P2>(&self, requestid: i32, strcadnsname: P0, strcaname: P1, strfriendlyname: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), requestid, strcadnsname.param().abi(), strcaname.param().abi(), strfriendlyname.param().abi()).ok()
    }
    pub unsafe fn RequestId(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RequestId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CADnsName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CADnsName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CAName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CAName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn FriendlyName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FriendlyName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertPropertyEnrollment_Vtbl {
    pub base__: ICertProperty_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub RequestId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CADnsName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CAName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub FriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICertPropertyEnrollmentPolicyServer, ICertPropertyEnrollmentPolicyServer_Vtbl, 0x728ab34a_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICertPropertyEnrollmentPolicyServer {
    type Target = ICertProperty;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICertPropertyEnrollmentPolicyServer, windows_core::IUnknown, super::super::super::System::Com::IDispatch, ICertProperty);
#[cfg(feature = "Win32_System_Com")]
impl ICertPropertyEnrollmentPolicyServer {
    pub unsafe fn Initialize<P0, P1, P2, P3>(&self, propertyflags: EnrollmentPolicyServerPropertyFlags, authflags: X509EnrollmentAuthFlags, enrollmentserverauthflags: X509EnrollmentAuthFlags, urlflags: PolicyServerUrlFlags, strrequestid: P0, strurl: P1, strid: P2, strenrollmentserverurl: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
        P3: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), propertyflags, authflags, enrollmentserverauthflags, urlflags, strrequestid.param().abi(), strurl.param().abi(), strid.param().abi(), strenrollmentserverurl.param().abi()).ok()
    }
    pub unsafe fn GetPolicyServerUrl(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPolicyServerUrl)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetPolicyServerId(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPolicyServerId)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetEnrollmentServerUrl(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetEnrollmentServerUrl)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetRequestIdString(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRequestIdString)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetPropertyFlags(&self) -> windows_core::Result<EnrollmentPolicyServerPropertyFlags> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPropertyFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetUrlFlags(&self) -> windows_core::Result<PolicyServerUrlFlags> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetUrlFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetAuthentication(&self) -> windows_core::Result<X509EnrollmentAuthFlags> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAuthentication)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetEnrollmentServerAuthentication(&self) -> windows_core::Result<X509EnrollmentAuthFlags> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetEnrollmentServerAuthentication)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertPropertyEnrollmentPolicyServer_Vtbl {
    pub base__: ICertProperty_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, EnrollmentPolicyServerPropertyFlags, X509EnrollmentAuthFlags, X509EnrollmentAuthFlags, PolicyServerUrlFlags, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetPolicyServerUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetPolicyServerId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetEnrollmentServerUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetRequestIdString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetPropertyFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut EnrollmentPolicyServerPropertyFlags) -> windows_core::HRESULT,
    pub GetUrlFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PolicyServerUrlFlags) -> windows_core::HRESULT,
    pub GetAuthentication: unsafe extern "system" fn(*mut core::ffi::c_void, *mut X509EnrollmentAuthFlags) -> windows_core::HRESULT,
    pub GetEnrollmentServerAuthentication: unsafe extern "system" fn(*mut core::ffi::c_void, *mut X509EnrollmentAuthFlags) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICertPropertyFriendlyName, ICertPropertyFriendlyName_Vtbl, 0x728ab330_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICertPropertyFriendlyName {
    type Target = ICertProperty;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICertPropertyFriendlyName, windows_core::IUnknown, super::super::super::System::Com::IDispatch, ICertProperty);
#[cfg(feature = "Win32_System_Com")]
impl ICertPropertyFriendlyName {
    pub unsafe fn Initialize<P0>(&self, strfriendlyname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), strfriendlyname.param().abi()).ok()
    }
    pub unsafe fn FriendlyName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FriendlyName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertPropertyFriendlyName_Vtbl {
    pub base__: ICertProperty_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub FriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICertPropertyKeyProvInfo, ICertPropertyKeyProvInfo_Vtbl, 0x728ab336_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICertPropertyKeyProvInfo {
    type Target = ICertProperty;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICertPropertyKeyProvInfo, windows_core::IUnknown, super::super::super::System::Com::IDispatch, ICertProperty);
#[cfg(feature = "Win32_System_Com")]
impl ICertPropertyKeyProvInfo {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Initialize<P0>(&self, pvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IX509PrivateKey>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pvalue.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PrivateKey(&self) -> windows_core::Result<IX509PrivateKey> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PrivateKey)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertPropertyKeyProvInfo_Vtbl {
    pub base__: ICertProperty_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Initialize: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PrivateKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PrivateKey: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICertPropertyRenewal, ICertPropertyRenewal_Vtbl, 0x728ab33a_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICertPropertyRenewal {
    type Target = ICertProperty;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICertPropertyRenewal, windows_core::IUnknown, super::super::super::System::Com::IDispatch, ICertProperty);
#[cfg(feature = "Win32_System_Com")]
impl ICertPropertyRenewal {
    pub unsafe fn Initialize<P0>(&self, encoding: EncodingType, strrenewalvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), encoding, strrenewalvalue.param().abi()).ok()
    }
    pub unsafe fn InitializeFromCertificateHash<P0, P1>(&self, machinecontext: P0, encoding: EncodingType, strcertificate: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::VARIANT_BOOL>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).InitializeFromCertificateHash)(windows_core::Interface::as_raw(self), machinecontext.param().abi(), encoding, strcertificate.param().abi()).ok()
    }
    pub unsafe fn get_Renewal(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Renewal)(windows_core::Interface::as_raw(self), encoding, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertPropertyRenewal_Vtbl {
    pub base__: ICertProperty_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub InitializeFromCertificateHash: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::VARIANT_BOOL, EncodingType, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub get_Renewal: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICertPropertyRequestOriginator, ICertPropertyRequestOriginator_Vtbl, 0x728ab333_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICertPropertyRequestOriginator {
    type Target = ICertProperty;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICertPropertyRequestOriginator, windows_core::IUnknown, super::super::super::System::Com::IDispatch, ICertProperty);
#[cfg(feature = "Win32_System_Com")]
impl ICertPropertyRequestOriginator {
    pub unsafe fn Initialize<P0>(&self, strrequestoriginator: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), strrequestoriginator.param().abi()).ok()
    }
    pub unsafe fn InitializeFromLocalRequestOriginator(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).InitializeFromLocalRequestOriginator)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RequestOriginator(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RequestOriginator)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertPropertyRequestOriginator_Vtbl {
    pub base__: ICertProperty_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub InitializeFromLocalRequestOriginator: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestOriginator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICertPropertySHA1Hash, ICertPropertySHA1Hash_Vtbl, 0x728ab334_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICertPropertySHA1Hash {
    type Target = ICertProperty;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICertPropertySHA1Hash, windows_core::IUnknown, super::super::super::System::Com::IDispatch, ICertProperty);
#[cfg(feature = "Win32_System_Com")]
impl ICertPropertySHA1Hash {
    pub unsafe fn Initialize<P0>(&self, encoding: EncodingType, strrenewalvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), encoding, strrenewalvalue.param().abi()).ok()
    }
    pub unsafe fn get_SHA1Hash(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_SHA1Hash)(windows_core::Interface::as_raw(self), encoding, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertPropertySHA1Hash_Vtbl {
    pub base__: ICertProperty_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub get_SHA1Hash: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICertRequest, ICertRequest_Vtbl, 0x014e4840_5523_11d0_8812_00a0c903b83c);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICertRequest {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICertRequest, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ICertRequest {
    pub unsafe fn Submit<P0, P1, P2>(&self, flags: i32, strrequest: P0, strattributes: P1, strconfig: P2) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Submit)(windows_core::Interface::as_raw(self), flags, strrequest.param().abi(), strattributes.param().abi(), strconfig.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn RetrievePending<P0>(&self, requestid: i32, strconfig: P0) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RetrievePending)(windows_core::Interface::as_raw(self), requestid, strconfig.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn GetLastStatus(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLastStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetRequestId(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRequestId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetDispositionMessage(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDispositionMessage)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCACertificate<P0>(&self, fexchangecertificate: i32, strconfig: P0, flags: i32) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCACertificate)(windows_core::Interface::as_raw(self), fexchangecertificate, strconfig.param().abi(), flags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCertificate(&self, flags: i32) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCertificate)(windows_core::Interface::as_raw(self), flags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertRequest_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub Submit: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *mut i32) -> windows_core::HRESULT,
    pub RetrievePending: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::BSTR>, *mut i32) -> windows_core::HRESULT,
    pub GetLastStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetRequestId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetDispositionMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetCACertificate: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::BSTR>, i32, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICertRequest2, ICertRequest2_Vtbl, 0xa4772988_4a85_4fa9_824e_b5cf5c16405a);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICertRequest2 {
    type Target = ICertRequest;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICertRequest2, windows_core::IUnknown, super::super::super::System::Com::IDispatch, ICertRequest);
#[cfg(feature = "Win32_System_Com")]
impl ICertRequest2 {
    pub unsafe fn GetIssuedCertificate<P0, P1>(&self, strconfig: P0, requestid: i32, strserialnumber: P1) -> windows_core::Result<CR_DISP>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetIssuedCertificate)(windows_core::Interface::as_raw(self), strconfig.param().abi(), requestid, strserialnumber.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn GetErrorMessageText(&self, hrmessage: i32, flags: i32) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetErrorMessageText)(windows_core::Interface::as_raw(self), hrmessage, flags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCAProperty<P0>(&self, strconfig: P0, propid: i32, propindex: i32, proptype: i32, flags: i32) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCAProperty)(windows_core::Interface::as_raw(self), strconfig.param().abi(), propid, propindex, proptype, flags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCAPropertyFlags<P0>(&self, strconfig: P0, propid: i32) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCAPropertyFlags)(windows_core::Interface::as_raw(self), strconfig.param().abi(), propid, &mut result__).map(|| result__)
    }
    pub unsafe fn GetCAPropertyDisplayName<P0>(&self, strconfig: P0, propid: i32) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCAPropertyDisplayName)(windows_core::Interface::as_raw(self), strconfig.param().abi(), propid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetFullResponseProperty(&self, propid: FULL_RESPONSE_PROPERTY_ID, propindex: i32, proptype: CERT_PROPERTY_TYPE, flags: CERT_REQUEST_OUT_TYPE) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFullResponseProperty)(windows_core::Interface::as_raw(self), propid, propindex, proptype, flags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertRequest2_Vtbl {
    pub base__: ICertRequest_Vtbl,
    pub GetIssuedCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, core::mem::MaybeUninit<windows_core::BSTR>, *mut CR_DISP) -> windows_core::HRESULT,
    pub GetErrorMessageText: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetCAProperty: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, i32, i32, i32, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub GetCAPropertyFlags: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, *mut i32) -> windows_core::HRESULT,
    pub GetCAPropertyDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetFullResponseProperty: unsafe extern "system" fn(*mut core::ffi::c_void, FULL_RESPONSE_PROPERTY_ID, i32, CERT_PROPERTY_TYPE, CERT_REQUEST_OUT_TYPE, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICertRequest3, ICertRequest3_Vtbl, 0xafc8f92b_33a2_4861_bf36_2933b7cd67b3);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICertRequest3 {
    type Target = ICertRequest2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICertRequest3, windows_core::IUnknown, super::super::super::System::Com::IDispatch, ICertRequest, ICertRequest2);
#[cfg(feature = "Win32_System_Com")]
impl ICertRequest3 {
    pub unsafe fn SetCredential<P0, P1>(&self, hwnd: i32, authtype: X509EnrollmentAuthFlags, strcredential: P0, strpassword: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetCredential)(windows_core::Interface::as_raw(self), hwnd, authtype, strcredential.param().abi(), strpassword.param().abi()).ok()
    }
    pub unsafe fn GetRequestIdString(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRequestIdString)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetIssuedCertificate2<P0, P1, P2>(&self, strconfig: P0, strrequestid: P1, strserialnumber: P2) -> windows_core::Result<CR_DISP>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetIssuedCertificate2)(windows_core::Interface::as_raw(self), strconfig.param().abi(), strrequestid.param().abi(), strserialnumber.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn GetRefreshPolicy(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRefreshPolicy)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertRequest3_Vtbl {
    pub base__: ICertRequest2_Vtbl,
    pub SetCredential: unsafe extern "system" fn(*mut core::ffi::c_void, i32, X509EnrollmentAuthFlags, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetRequestIdString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetIssuedCertificate2: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *mut CR_DISP) -> windows_core::HRESULT,
    pub GetRefreshPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICertRequestD, ICertRequestD_Vtbl, 0xd99e6e70_fc88_11d0_b498_00a0c90312f3);
impl core::ops::Deref for ICertRequestD {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICertRequestD, windows_core::IUnknown);
impl ICertRequestD {
    pub unsafe fn Request<P0, P1>(&self, dwflags: u32, pwszauthority: P0, pdwrequestid: *mut u32, pdwdisposition: *mut u32, pwszattributes: P1, pctbrequest: *const CERTTRANSBLOB, pctbcertchain: *mut CERTTRANSBLOB, pctbencodedcert: *mut CERTTRANSBLOB, pctbdispositionmessage: *mut CERTTRANSBLOB) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).Request)(windows_core::Interface::as_raw(self), dwflags, pwszauthority.param().abi(), pdwrequestid, pdwdisposition, pwszattributes.param().abi(), pctbrequest, pctbcertchain, pctbencodedcert, pctbdispositionmessage).ok()
    }
    pub unsafe fn GetCACert<P0>(&self, fchain: u32, pwszauthority: P0) -> windows_core::Result<CERTTRANSBLOB>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCACert)(windows_core::Interface::as_raw(self), fchain, pwszauthority.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn Ping<P0>(&self, pwszauthority: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).Ping)(windows_core::Interface::as_raw(self), pwszauthority.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ICertRequestD_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Request: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, *mut u32, *mut u32, windows_core::PCWSTR, *const CERTTRANSBLOB, *mut CERTTRANSBLOB, *mut CERTTRANSBLOB, *mut CERTTRANSBLOB) -> windows_core::HRESULT,
    pub GetCACert: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, *mut CERTTRANSBLOB) -> windows_core::HRESULT,
    pub Ping: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICertRequestD2, ICertRequestD2_Vtbl, 0x5422fd3a_d4b8_4cef_a12e_e87d4ca22e90);
impl core::ops::Deref for ICertRequestD2 {
    type Target = ICertRequestD;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICertRequestD2, windows_core::IUnknown, ICertRequestD);
impl ICertRequestD2 {
    pub unsafe fn Request2<P0, P1, P2>(&self, pwszauthority: P0, dwflags: u32, pwszserialnumber: P1, pdwrequestid: *mut u32, pdwdisposition: *mut u32, pwszattributes: P2, pctbrequest: *const CERTTRANSBLOB, pctbfullresponse: *mut CERTTRANSBLOB, pctbencodedcert: *mut CERTTRANSBLOB, pctbdispositionmessage: *mut CERTTRANSBLOB) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).Request2)(windows_core::Interface::as_raw(self), pwszauthority.param().abi(), dwflags, pwszserialnumber.param().abi(), pdwrequestid, pdwdisposition, pwszattributes.param().abi(), pctbrequest, pctbfullresponse, pctbencodedcert, pctbdispositionmessage).ok()
    }
    pub unsafe fn GetCAProperty<P0>(&self, pwszauthority: P0, propid: i32, propindex: i32, proptype: i32) -> windows_core::Result<CERTTRANSBLOB>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCAProperty)(windows_core::Interface::as_raw(self), pwszauthority.param().abi(), propid, propindex, proptype, &mut result__).map(|| result__)
    }
    pub unsafe fn GetCAPropertyInfo<P0>(&self, pwszauthority: P0, pcproperty: *mut i32, pctbpropinfo: *mut CERTTRANSBLOB) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetCAPropertyInfo)(windows_core::Interface::as_raw(self), pwszauthority.param().abi(), pcproperty, pctbpropinfo).ok()
    }
    pub unsafe fn Ping2<P0>(&self, pwszauthority: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).Ping2)(windows_core::Interface::as_raw(self), pwszauthority.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ICertRequestD2_Vtbl {
    pub base__: ICertRequestD_Vtbl,
    pub Request2: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, windows_core::PCWSTR, *mut u32, *mut u32, windows_core::PCWSTR, *const CERTTRANSBLOB, *mut CERTTRANSBLOB, *mut CERTTRANSBLOB, *mut CERTTRANSBLOB) -> windows_core::HRESULT,
    pub GetCAProperty: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, i32, i32, i32, *mut CERTTRANSBLOB) -> windows_core::HRESULT,
    pub GetCAPropertyInfo: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut i32, *mut CERTTRANSBLOB) -> windows_core::HRESULT,
    pub Ping2: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICertServerExit, ICertServerExit_Vtbl, 0x4ba9eb90_732c_11d0_8816_00a0c903b83c);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICertServerExit {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICertServerExit, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ICertServerExit {
    pub unsafe fn SetContext(&self, context: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetContext)(windows_core::Interface::as_raw(self), context).ok()
    }
    pub unsafe fn GetRequestProperty<P0>(&self, strpropertyname: P0, propertytype: i32) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRequestProperty)(windows_core::Interface::as_raw(self), strpropertyname.param().abi(), propertytype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetRequestAttribute<P0>(&self, strattributename: P0) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRequestAttribute)(windows_core::Interface::as_raw(self), strattributename.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCertificateProperty<P0>(&self, strpropertyname: P0, propertytype: i32) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCertificateProperty)(windows_core::Interface::as_raw(self), strpropertyname.param().abi(), propertytype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCertificateExtension<P0>(&self, strextensionname: P0, r#type: i32) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCertificateExtension)(windows_core::Interface::as_raw(self), strextensionname.param().abi(), r#type, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCertificateExtensionFlags(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCertificateExtensionFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn EnumerateExtensionsSetup(&self, flags: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnumerateExtensionsSetup)(windows_core::Interface::as_raw(self), flags).ok()
    }
    pub unsafe fn EnumerateExtensions(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumerateExtensions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumerateExtensionsClose(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnumerateExtensionsClose)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EnumerateAttributesSetup(&self, flags: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnumerateAttributesSetup)(windows_core::Interface::as_raw(self), flags).ok()
    }
    pub unsafe fn EnumerateAttributes(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumerateAttributes)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumerateAttributesClose(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnumerateAttributesClose)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertServerExit_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub SetContext: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetRequestProperty: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub GetRequestAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetCertificateProperty: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub GetCertificateExtension: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub GetCertificateExtensionFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub EnumerateExtensionsSetup: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub EnumerateExtensions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub EnumerateExtensionsClose: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumerateAttributesSetup: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub EnumerateAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub EnumerateAttributesClose: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICertServerPolicy, ICertServerPolicy_Vtbl, 0xaa000922_ffbe_11cf_8800_00a0c903b83c);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICertServerPolicy {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICertServerPolicy, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ICertServerPolicy {
    pub unsafe fn SetContext(&self, context: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetContext)(windows_core::Interface::as_raw(self), context).ok()
    }
    pub unsafe fn GetRequestProperty<P0>(&self, strpropertyname: P0, propertytype: i32) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRequestProperty)(windows_core::Interface::as_raw(self), strpropertyname.param().abi(), propertytype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetRequestAttribute<P0>(&self, strattributename: P0) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRequestAttribute)(windows_core::Interface::as_raw(self), strattributename.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCertificateProperty<P0>(&self, strpropertyname: P0, propertytype: CERT_PROPERTY_TYPE) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCertificateProperty)(windows_core::Interface::as_raw(self), strpropertyname.param().abi(), propertytype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetCertificateProperty<P0>(&self, strpropertyname: P0, propertytype: i32, pvarpropertyvalue: *const windows_core::VARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetCertificateProperty)(windows_core::Interface::as_raw(self), strpropertyname.param().abi(), propertytype, core::mem::transmute(pvarpropertyvalue)).ok()
    }
    pub unsafe fn GetCertificateExtension<P0>(&self, strextensionname: P0, r#type: CERT_PROPERTY_TYPE) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCertificateExtension)(windows_core::Interface::as_raw(self), strextensionname.param().abi(), r#type, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCertificateExtensionFlags(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCertificateExtensionFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetCertificateExtension<P0>(&self, strextensionname: P0, r#type: i32, extflags: i32, pvarvalue: *const windows_core::VARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetCertificateExtension)(windows_core::Interface::as_raw(self), strextensionname.param().abi(), r#type, extflags, core::mem::transmute(pvarvalue)).ok()
    }
    pub unsafe fn EnumerateExtensionsSetup(&self, flags: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnumerateExtensionsSetup)(windows_core::Interface::as_raw(self), flags).ok()
    }
    pub unsafe fn EnumerateExtensions(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumerateExtensions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumerateExtensionsClose(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnumerateExtensionsClose)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EnumerateAttributesSetup(&self, flags: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnumerateAttributesSetup)(windows_core::Interface::as_raw(self), flags).ok()
    }
    pub unsafe fn EnumerateAttributes(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumerateAttributes)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumerateAttributesClose(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnumerateAttributesClose)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertServerPolicy_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub SetContext: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetRequestProperty: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub GetRequestAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetCertificateProperty: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, CERT_PROPERTY_TYPE, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetCertificateProperty: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub GetCertificateExtension: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, CERT_PROPERTY_TYPE, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub GetCertificateExtensionFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetCertificateExtension: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, i32, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub EnumerateExtensionsSetup: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub EnumerateExtensions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub EnumerateExtensionsClose: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumerateAttributesSetup: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub EnumerateAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub EnumerateAttributesClose: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICertView, ICertView_Vtbl, 0xc3fac344_1e84_11d1_9bd6_00c04fb683fa);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICertView {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICertView, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ICertView {
    pub unsafe fn OpenConnection<P0>(&self, strconfig: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).OpenConnection)(windows_core::Interface::as_raw(self), strconfig.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumCertViewColumn(&self, fresultcolumn: CVRC_COLUMN) -> windows_core::Result<IEnumCERTVIEWCOLUMN> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumCertViewColumn)(windows_core::Interface::as_raw(self), fresultcolumn, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetColumnCount(&self, fresultcolumn: CVRC_COLUMN, pccolumn: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetColumnCount)(windows_core::Interface::as_raw(self), fresultcolumn, pccolumn).ok()
    }
    pub unsafe fn GetColumnIndex<P0>(&self, fresultcolumn: CVRC_COLUMN, strcolumnname: P0, pcolumnindex: *mut i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).GetColumnIndex)(windows_core::Interface::as_raw(self), fresultcolumn, strcolumnname.param().abi(), pcolumnindex).ok()
    }
    pub unsafe fn SetResultColumnCount(&self, cresultcolumn: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetResultColumnCount)(windows_core::Interface::as_raw(self), cresultcolumn).ok()
    }
    pub unsafe fn SetResultColumn(&self, columnindex: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetResultColumn)(windows_core::Interface::as_raw(self), columnindex).ok()
    }
    pub unsafe fn SetRestriction(&self, columnindex: CERT_VIEW_COLUMN_INDEX, seekoperator: CERT_VIEW_SEEK_OPERATOR_FLAGS, sortorder: i32, pvarvalue: *const windows_core::VARIANT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetRestriction)(windows_core::Interface::as_raw(self), columnindex, seekoperator, sortorder, core::mem::transmute(pvarvalue)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenView(&self) -> windows_core::Result<IEnumCERTVIEWROW> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OpenView)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertView_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub OpenConnection: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumCertViewColumn: unsafe extern "system" fn(*mut core::ffi::c_void, CVRC_COLUMN, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumCertViewColumn: usize,
    pub GetColumnCount: unsafe extern "system" fn(*mut core::ffi::c_void, CVRC_COLUMN, *mut i32) -> windows_core::HRESULT,
    pub GetColumnIndex: unsafe extern "system" fn(*mut core::ffi::c_void, CVRC_COLUMN, core::mem::MaybeUninit<windows_core::BSTR>, *mut i32) -> windows_core::HRESULT,
    pub SetResultColumnCount: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SetResultColumn: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SetRestriction: unsafe extern "system" fn(*mut core::ffi::c_void, CERT_VIEW_COLUMN_INDEX, CERT_VIEW_SEEK_OPERATOR_FLAGS, i32, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub OpenView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OpenView: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICertView2, ICertView2_Vtbl, 0xd594b282_8851_4b61_9c66_3edadf848863);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICertView2 {
    type Target = ICertView;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICertView2, windows_core::IUnknown, super::super::super::System::Com::IDispatch, ICertView);
#[cfg(feature = "Win32_System_Com")]
impl ICertView2 {
    pub unsafe fn SetTable(&self, table: CVRC_TABLE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetTable)(windows_core::Interface::as_raw(self), table).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertView2_Vtbl {
    pub base__: ICertView_Vtbl,
    pub SetTable: unsafe extern "system" fn(*mut core::ffi::c_void, CVRC_TABLE) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICertificateAttestationChallenge, ICertificateAttestationChallenge_Vtbl, 0x6f175a7c_4a3a_40ae_9dba_592fd6bbf9b8);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICertificateAttestationChallenge {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICertificateAttestationChallenge, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ICertificateAttestationChallenge {
    pub unsafe fn Initialize<P0>(&self, encoding: EncodingType, strpendingfullcmcresponsewithchallenge: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), encoding, strpendingfullcmcresponsewithchallenge.param().abi()).ok()
    }
    pub unsafe fn DecryptChallenge(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DecryptChallenge)(windows_core::Interface::as_raw(self), encoding, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RequestID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RequestID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertificateAttestationChallenge_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub DecryptChallenge: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub RequestID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICertificateAttestationChallenge2, ICertificateAttestationChallenge2_Vtbl, 0x4631334d_e266_47d6_bd79_be53cb2e2753);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICertificateAttestationChallenge2 {
    type Target = ICertificateAttestationChallenge;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICertificateAttestationChallenge2, windows_core::IUnknown, super::super::super::System::Com::IDispatch, ICertificateAttestationChallenge);
#[cfg(feature = "Win32_System_Com")]
impl ICertificateAttestationChallenge2 {
    pub unsafe fn SetKeyContainerName<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetKeyContainerName)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn put_KeyBlob<P0>(&self, encoding: EncodingType, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).put_KeyBlob)(windows_core::Interface::as_raw(self), encoding, value.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertificateAttestationChallenge2_Vtbl {
    pub base__: ICertificateAttestationChallenge_Vtbl,
    pub SetKeyContainerName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub put_KeyBlob: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICertificatePolicies, ICertificatePolicies_Vtbl, 0x728ab31f_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICertificatePolicies {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICertificatePolicies, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ICertificatePolicies {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_ItemByIndex(&self, index: i32) -> windows_core::Result<ICertificatePolicy> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_ItemByIndex)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Add<P0>(&self, pval: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ICertificatePolicy>,
    {
        (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), pval.param().abi()).ok()
    }
    pub unsafe fn Remove(&self, index: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), index).ok()
    }
    pub unsafe fn Clear(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertificatePolicies_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub get_ItemByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_ItemByIndex: usize,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICertificatePolicy, ICertificatePolicy_Vtbl, 0x728ab31e_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICertificatePolicy {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICertificatePolicy, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ICertificatePolicy {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Initialize<P0>(&self, pvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IObjectId>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pvalue.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ObjectId(&self) -> windows_core::Result<IObjectId> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ObjectId)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PolicyQualifiers(&self) -> windows_core::Result<IPolicyQualifiers> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PolicyQualifiers)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertificatePolicy_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Initialize: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ObjectId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ObjectId: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PolicyQualifiers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PolicyQualifiers: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICertificationAuthorities, ICertificationAuthorities_Vtbl, 0x13b79005_2181_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICertificationAuthorities {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICertificationAuthorities, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ICertificationAuthorities {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_ItemByIndex(&self, index: i32) -> windows_core::Result<ICertificationAuthority> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_ItemByIndex)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Add<P0>(&self, pval: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ICertificationAuthority>,
    {
        (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), pval.param().abi()).ok()
    }
    pub unsafe fn Remove(&self, index: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), index).ok()
    }
    pub unsafe fn Clear(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ComputeSiteCosts(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ComputeSiteCosts)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_ItemByName<P0>(&self, strname: P0) -> windows_core::Result<ICertificationAuthority>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_ItemByName)(windows_core::Interface::as_raw(self), strname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertificationAuthorities_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub get_ItemByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_ItemByIndex: usize,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ComputeSiteCosts: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_ItemByName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_ItemByName: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICertificationAuthority, ICertificationAuthority_Vtbl, 0x835d1f61_1e95_4bc8_b4d3_976c42b968f7);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICertificationAuthority {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICertificationAuthority, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ICertificationAuthority {
    pub unsafe fn get_Property(&self, property: EnrollmentCAProperty) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Property)(windows_core::Interface::as_raw(self), property, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertificationAuthority_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub get_Property: unsafe extern "system" fn(*mut core::ffi::c_void, EnrollmentCAProperty, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICryptAttribute, ICryptAttribute_Vtbl, 0x728ab32c_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICryptAttribute {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICryptAttribute, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ICryptAttribute {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitializeFromObjectId<P0>(&self, pobjectid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IObjectId>,
    {
        (windows_core::Interface::vtable(self).InitializeFromObjectId)(windows_core::Interface::as_raw(self), pobjectid.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitializeFromValues<P0>(&self, pattributes: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IX509Attributes>,
    {
        (windows_core::Interface::vtable(self).InitializeFromValues)(windows_core::Interface::as_raw(self), pattributes.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ObjectId(&self) -> windows_core::Result<IObjectId> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ObjectId)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Values(&self) -> windows_core::Result<IX509Attributes> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Values)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICryptAttribute_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub InitializeFromObjectId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InitializeFromObjectId: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub InitializeFromValues: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InitializeFromValues: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ObjectId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ObjectId: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Values: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Values: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICryptAttributes, ICryptAttributes_Vtbl, 0x728ab32d_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICryptAttributes {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICryptAttributes, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ICryptAttributes {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_ItemByIndex(&self, index: i32) -> windows_core::Result<ICryptAttribute> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_ItemByIndex)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Add<P0>(&self, pval: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ICryptAttribute>,
    {
        (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), pval.param().abi()).ok()
    }
    pub unsafe fn Remove(&self, index: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), index).ok()
    }
    pub unsafe fn Clear(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_IndexByObjectId<P0>(&self, pobjectid: P0) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<IObjectId>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_IndexByObjectId)(windows_core::Interface::as_raw(self), pobjectid.param().abi(), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddRange<P0>(&self, pvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ICryptAttributes>,
    {
        (windows_core::Interface::vtable(self).AddRange)(windows_core::Interface::as_raw(self), pvalue.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICryptAttributes_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub get_ItemByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_ItemByIndex: usize,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_IndexByObjectId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_IndexByObjectId: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AddRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddRange: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICspAlgorithm, ICspAlgorithm_Vtbl, 0x728ab305_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICspAlgorithm {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICspAlgorithm, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ICspAlgorithm {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetAlgorithmOid(&self, length: i32, algflags: AlgorithmFlags) -> windows_core::Result<IObjectId> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAlgorithmOid)(windows_core::Interface::as_raw(self), length, algflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DefaultLength(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DefaultLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IncrementLength(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IncrementLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn LongName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LongName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Valid(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Valid)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn MaxLength(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MaxLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn MinLength(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MinLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Type(&self) -> windows_core::Result<AlgorithmType> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Type)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Operations(&self) -> windows_core::Result<AlgorithmOperationFlags> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Operations)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICspAlgorithm_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetAlgorithmOid: unsafe extern "system" fn(*mut core::ffi::c_void, i32, AlgorithmFlags, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetAlgorithmOid: usize,
    pub DefaultLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub IncrementLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub LongName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Valid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub MaxLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub MinLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AlgorithmType) -> windows_core::HRESULT,
    pub Operations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AlgorithmOperationFlags) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICspAlgorithms, ICspAlgorithms_Vtbl, 0x728ab306_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICspAlgorithms {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICspAlgorithms, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ICspAlgorithms {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_ItemByIndex(&self, index: i32) -> windows_core::Result<ICspAlgorithm> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_ItemByIndex)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Add<P0>(&self, pval: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ICspAlgorithm>,
    {
        (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), pval.param().abi()).ok()
    }
    pub unsafe fn Remove(&self, index: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), index).ok()
    }
    pub unsafe fn Clear(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_ItemByName<P0>(&self, strname: P0) -> windows_core::Result<ICspAlgorithm>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_ItemByName)(windows_core::Interface::as_raw(self), strname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_IndexByObjectId<P0>(&self, pobjectid: P0) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<IObjectId>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_IndexByObjectId)(windows_core::Interface::as_raw(self), pobjectid.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICspAlgorithms_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub get_ItemByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_ItemByIndex: usize,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_ItemByName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_ItemByName: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub get_IndexByObjectId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_IndexByObjectId: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICspInformation, ICspInformation_Vtbl, 0x728ab307_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICspInformation {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICspInformation, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ICspInformation {
    pub unsafe fn InitializeFromName<P0>(&self, strname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).InitializeFromName)(windows_core::Interface::as_raw(self), strname.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitializeFromType<P0, P1>(&self, r#type: X509ProviderType, palgorithm: P0, machinecontext: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IObjectId>,
        P1: windows_core::Param<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).InitializeFromType)(windows_core::Interface::as_raw(self), r#type, palgorithm.param().abi(), machinecontext.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CspAlgorithms(&self) -> windows_core::Result<ICspAlgorithms> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CspAlgorithms)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn HasHardwareRandomNumberGenerator(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).HasHardwareRandomNumberGenerator)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IsHardwareDevice(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsHardwareDevice)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IsRemovable(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsRemovable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IsSoftwareDevice(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsSoftwareDevice)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Valid(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Valid)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn MaxKeyContainerNameLength(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MaxKeyContainerNameLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Type(&self) -> windows_core::Result<X509ProviderType> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Type)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Version(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Version)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn KeySpec(&self) -> windows_core::Result<X509KeySpec> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).KeySpec)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IsSmartCard(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsSmartCard)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetDefaultSecurityDescriptor<P0>(&self, machinecontext: P0) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<super::super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDefaultSecurityDescriptor)(windows_core::Interface::as_raw(self), machinecontext.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn LegacyCsp(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LegacyCsp)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetCspStatusFromOperations<P0>(&self, palgorithm: P0, operations: AlgorithmOperationFlags) -> windows_core::Result<ICspStatus>
    where
        P0: windows_core::Param<IObjectId>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCspStatusFromOperations)(windows_core::Interface::as_raw(self), palgorithm.param().abi(), operations, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICspInformation_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub InitializeFromName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub InitializeFromType: unsafe extern "system" fn(*mut core::ffi::c_void, X509ProviderType, *mut core::ffi::c_void, super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InitializeFromType: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CspAlgorithms: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CspAlgorithms: usize,
    pub HasHardwareRandomNumberGenerator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub IsHardwareDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub IsRemovable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub IsSoftwareDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Valid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub MaxKeyContainerNameLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut X509ProviderType) -> windows_core::HRESULT,
    pub Version: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub KeySpec: unsafe extern "system" fn(*mut core::ffi::c_void, *mut X509KeySpec) -> windows_core::HRESULT,
    pub IsSmartCard: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub GetDefaultSecurityDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::VARIANT_BOOL, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub LegacyCsp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetCspStatusFromOperations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, AlgorithmOperationFlags, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetCspStatusFromOperations: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICspInformations, ICspInformations_Vtbl, 0x728ab308_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICspInformations {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICspInformations, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ICspInformations {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_ItemByIndex(&self, index: i32) -> windows_core::Result<ICspInformation> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_ItemByIndex)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Add<P0>(&self, pval: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ICspInformation>,
    {
        (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), pval.param().abi()).ok()
    }
    pub unsafe fn Remove(&self, index: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), index).ok()
    }
    pub unsafe fn Clear(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn AddAvailableCsps(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddAvailableCsps)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_ItemByName<P0>(&self, strname: P0) -> windows_core::Result<ICspInformation>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_ItemByName)(windows_core::Interface::as_raw(self), strname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetCspStatusFromProviderName<P0>(&self, strprovidername: P0, legacykeyspec: X509KeySpec) -> windows_core::Result<ICspStatus>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCspStatusFromProviderName)(windows_core::Interface::as_raw(self), strprovidername.param().abi(), legacykeyspec, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetCspStatusesFromOperations<P0>(&self, operations: AlgorithmOperationFlags, pcspinformation: P0) -> windows_core::Result<ICspStatuses>
    where
        P0: windows_core::Param<ICspInformation>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCspStatusesFromOperations)(windows_core::Interface::as_raw(self), operations, pcspinformation.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetEncryptionCspAlgorithms<P0>(&self, pcspinformation: P0) -> windows_core::Result<ICspAlgorithms>
    where
        P0: windows_core::Param<ICspInformation>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetEncryptionCspAlgorithms)(windows_core::Interface::as_raw(self), pcspinformation.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetHashAlgorithms<P0>(&self, pcspinformation: P0) -> windows_core::Result<IObjectIds>
    where
        P0: windows_core::Param<ICspInformation>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetHashAlgorithms)(windows_core::Interface::as_raw(self), pcspinformation.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICspInformations_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub get_ItemByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_ItemByIndex: usize,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddAvailableCsps: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_ItemByName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_ItemByName: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetCspStatusFromProviderName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, X509KeySpec, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetCspStatusFromProviderName: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetCspStatusesFromOperations: unsafe extern "system" fn(*mut core::ffi::c_void, AlgorithmOperationFlags, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetCspStatusesFromOperations: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetEncryptionCspAlgorithms: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetEncryptionCspAlgorithms: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetHashAlgorithms: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetHashAlgorithms: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICspStatus, ICspStatus_Vtbl, 0x728ab309_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICspStatus {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICspStatus, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ICspStatus {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Initialize<P0, P1>(&self, pcsp: P0, palgorithm: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ICspInformation>,
        P1: windows_core::Param<ICspAlgorithm>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pcsp.param().abi(), palgorithm.param().abi()).ok()
    }
    pub unsafe fn Ordinal(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Ordinal)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetOrdinal(&self, value: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetOrdinal)(windows_core::Interface::as_raw(self), value).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CspAlgorithm(&self) -> windows_core::Result<ICspAlgorithm> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CspAlgorithm)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CspInformation(&self) -> windows_core::Result<ICspInformation> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CspInformation)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnrollmentStatus(&self) -> windows_core::Result<IX509EnrollmentStatus> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnrollmentStatus)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DisplayName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DisplayName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICspStatus_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Initialize: usize,
    pub Ordinal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetOrdinal: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CspAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CspAlgorithm: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CspInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CspInformation: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnrollmentStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnrollmentStatus: usize,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICspStatuses, ICspStatuses_Vtbl, 0x728ab30a_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICspStatuses {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICspStatuses, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ICspStatuses {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_ItemByIndex(&self, index: i32) -> windows_core::Result<ICspStatus> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_ItemByIndex)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Add<P0>(&self, pval: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ICspStatus>,
    {
        (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), pval.param().abi()).ok()
    }
    pub unsafe fn Remove(&self, index: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), index).ok()
    }
    pub unsafe fn Clear(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_ItemByName<P0, P1>(&self, strcspname: P0, stralgorithmname: P1) -> windows_core::Result<ICspStatus>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_ItemByName)(windows_core::Interface::as_raw(self), strcspname.param().abi(), stralgorithmname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_ItemByOrdinal(&self, ordinal: i32) -> windows_core::Result<ICspStatus> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_ItemByOrdinal)(windows_core::Interface::as_raw(self), ordinal, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_ItemByOperations<P0, P1>(&self, strcspname: P0, stralgorithmname: P1, operations: AlgorithmOperationFlags) -> windows_core::Result<ICspStatus>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_ItemByOperations)(windows_core::Interface::as_raw(self), strcspname.param().abi(), stralgorithmname.param().abi(), operations, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_ItemByProvider<P0>(&self, pcspstatus: P0) -> windows_core::Result<ICspStatus>
    where
        P0: windows_core::Param<ICspStatus>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_ItemByProvider)(windows_core::Interface::as_raw(self), pcspstatus.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICspStatuses_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub get_ItemByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_ItemByIndex: usize,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_ItemByName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_ItemByName: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub get_ItemByOrdinal: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_ItemByOrdinal: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub get_ItemByOperations: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, AlgorithmOperationFlags, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_ItemByOperations: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub get_ItemByProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_ItemByProvider: usize,
}
windows_core::imp::define_interface!(IEnroll, IEnroll_Vtbl, 0xacaa7838_4585_11d1_ab57_00c04fc295e1);
impl core::ops::Deref for IEnroll {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnroll, windows_core::IUnknown);
impl IEnroll {
    pub unsafe fn createFilePKCS10WStr<P0, P1, P2>(&self, dnname: P0, usage: P1, wszpkcs10filename: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).createFilePKCS10WStr)(windows_core::Interface::as_raw(self), dnname.param().abi(), usage.param().abi(), wszpkcs10filename.param().abi()).ok()
    }
    pub unsafe fn acceptFilePKCS7WStr<P0>(&self, wszpkcs7filename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).acceptFilePKCS7WStr)(windows_core::Interface::as_raw(self), wszpkcs7filename.param().abi()).ok()
    }
    pub unsafe fn createPKCS10WStr<P0, P1>(&self, dnname: P0, usage: P1, ppkcs10blob: *mut super::CRYPT_INTEGER_BLOB) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).createPKCS10WStr)(windows_core::Interface::as_raw(self), dnname.param().abi(), usage.param().abi(), ppkcs10blob).ok()
    }
    pub unsafe fn acceptPKCS7Blob(&self, pblobpkcs7: *mut super::CRYPT_INTEGER_BLOB) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).acceptPKCS7Blob)(windows_core::Interface::as_raw(self), pblobpkcs7).ok()
    }
    pub unsafe fn getCertContextFromPKCS7(&self, pblobpkcs7: *mut super::CRYPT_INTEGER_BLOB) -> *mut super::CERT_CONTEXT {
        (windows_core::Interface::vtable(self).getCertContextFromPKCS7)(windows_core::Interface::as_raw(self), pblobpkcs7)
    }
    pub unsafe fn getMyStore(&self) -> super::HCERTSTORE {
        (windows_core::Interface::vtable(self).getMyStore)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn getCAStore(&self) -> super::HCERTSTORE {
        (windows_core::Interface::vtable(self).getCAStore)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn getROOTHStore(&self) -> super::HCERTSTORE {
        (windows_core::Interface::vtable(self).getROOTHStore)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn enumProvidersWStr(&self, dwindex: i32, dwflags: i32, pbstrprovname: *mut windows_core::PWSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).enumProvidersWStr)(windows_core::Interface::as_raw(self), dwindex, dwflags, pbstrprovname).ok()
    }
    pub unsafe fn enumContainersWStr(&self, dwindex: i32, pbstr: *mut windows_core::PWSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).enumContainersWStr)(windows_core::Interface::as_raw(self), dwindex, pbstr).ok()
    }
    pub unsafe fn freeRequestInfoBlob(&self, pkcs7orpkcs10: super::CRYPT_INTEGER_BLOB) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).freeRequestInfoBlob)(windows_core::Interface::as_raw(self), core::mem::transmute(pkcs7orpkcs10)).ok()
    }
    pub unsafe fn MyStoreNameWStr(&self, szwname: *mut windows_core::PWSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).MyStoreNameWStr)(windows_core::Interface::as_raw(self), szwname).ok()
    }
    pub unsafe fn SetMyStoreNameWStr<P0>(&self, szwname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetMyStoreNameWStr)(windows_core::Interface::as_raw(self), szwname.param().abi()).ok()
    }
    pub unsafe fn MyStoreTypeWStr(&self, szwtype: *mut windows_core::PWSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).MyStoreTypeWStr)(windows_core::Interface::as_raw(self), szwtype).ok()
    }
    pub unsafe fn SetMyStoreTypeWStr<P0>(&self, szwtype: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetMyStoreTypeWStr)(windows_core::Interface::as_raw(self), szwtype.param().abi()).ok()
    }
    pub unsafe fn MyStoreFlags(&self, pdwflags: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).MyStoreFlags)(windows_core::Interface::as_raw(self), pdwflags).ok()
    }
    pub unsafe fn SetMyStoreFlags(&self, dwflags: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMyStoreFlags)(windows_core::Interface::as_raw(self), dwflags).ok()
    }
    pub unsafe fn CAStoreNameWStr(&self, szwname: *mut windows_core::PWSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CAStoreNameWStr)(windows_core::Interface::as_raw(self), szwname).ok()
    }
    pub unsafe fn SetCAStoreNameWStr<P0>(&self, szwname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetCAStoreNameWStr)(windows_core::Interface::as_raw(self), szwname.param().abi()).ok()
    }
    pub unsafe fn CAStoreTypeWStr(&self, szwtype: *mut windows_core::PWSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CAStoreTypeWStr)(windows_core::Interface::as_raw(self), szwtype).ok()
    }
    pub unsafe fn SetCAStoreTypeWStr<P0>(&self, szwtype: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetCAStoreTypeWStr)(windows_core::Interface::as_raw(self), szwtype.param().abi()).ok()
    }
    pub unsafe fn CAStoreFlags(&self, pdwflags: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CAStoreFlags)(windows_core::Interface::as_raw(self), pdwflags).ok()
    }
    pub unsafe fn SetCAStoreFlags(&self, dwflags: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetCAStoreFlags)(windows_core::Interface::as_raw(self), dwflags).ok()
    }
    pub unsafe fn RootStoreNameWStr(&self, szwname: *mut windows_core::PWSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RootStoreNameWStr)(windows_core::Interface::as_raw(self), szwname).ok()
    }
    pub unsafe fn SetRootStoreNameWStr<P0>(&self, szwname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetRootStoreNameWStr)(windows_core::Interface::as_raw(self), szwname.param().abi()).ok()
    }
    pub unsafe fn RootStoreTypeWStr(&self, szwtype: *mut windows_core::PWSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RootStoreTypeWStr)(windows_core::Interface::as_raw(self), szwtype).ok()
    }
    pub unsafe fn SetRootStoreTypeWStr<P0>(&self, szwtype: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetRootStoreTypeWStr)(windows_core::Interface::as_raw(self), szwtype.param().abi()).ok()
    }
    pub unsafe fn RootStoreFlags(&self, pdwflags: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RootStoreFlags)(windows_core::Interface::as_raw(self), pdwflags).ok()
    }
    pub unsafe fn SetRootStoreFlags(&self, dwflags: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetRootStoreFlags)(windows_core::Interface::as_raw(self), dwflags).ok()
    }
    pub unsafe fn RequestStoreNameWStr(&self, szwname: *mut windows_core::PWSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RequestStoreNameWStr)(windows_core::Interface::as_raw(self), szwname).ok()
    }
    pub unsafe fn SetRequestStoreNameWStr<P0>(&self, szwname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetRequestStoreNameWStr)(windows_core::Interface::as_raw(self), szwname.param().abi()).ok()
    }
    pub unsafe fn RequestStoreTypeWStr(&self, szwtype: *mut windows_core::PWSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RequestStoreTypeWStr)(windows_core::Interface::as_raw(self), szwtype).ok()
    }
    pub unsafe fn SetRequestStoreTypeWStr<P0>(&self, szwtype: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetRequestStoreTypeWStr)(windows_core::Interface::as_raw(self), szwtype.param().abi()).ok()
    }
    pub unsafe fn RequestStoreFlags(&self, pdwflags: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RequestStoreFlags)(windows_core::Interface::as_raw(self), pdwflags).ok()
    }
    pub unsafe fn SetRequestStoreFlags(&self, dwflags: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetRequestStoreFlags)(windows_core::Interface::as_raw(self), dwflags).ok()
    }
    pub unsafe fn ContainerNameWStr(&self, szwcontainer: *mut windows_core::PWSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ContainerNameWStr)(windows_core::Interface::as_raw(self), szwcontainer).ok()
    }
    pub unsafe fn SetContainerNameWStr<P0>(&self, szwcontainer: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetContainerNameWStr)(windows_core::Interface::as_raw(self), szwcontainer.param().abi()).ok()
    }
    pub unsafe fn ProviderNameWStr(&self, szwprovider: *mut windows_core::PWSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ProviderNameWStr)(windows_core::Interface::as_raw(self), szwprovider).ok()
    }
    pub unsafe fn SetProviderNameWStr<P0>(&self, szwprovider: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetProviderNameWStr)(windows_core::Interface::as_raw(self), szwprovider.param().abi()).ok()
    }
    pub unsafe fn ProviderType(&self, pdwtype: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ProviderType)(windows_core::Interface::as_raw(self), pdwtype).ok()
    }
    pub unsafe fn SetProviderType(&self, dwtype: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetProviderType)(windows_core::Interface::as_raw(self), dwtype).ok()
    }
    pub unsafe fn KeySpec(&self, pdw: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).KeySpec)(windows_core::Interface::as_raw(self), pdw).ok()
    }
    pub unsafe fn SetKeySpec(&self, dw: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetKeySpec)(windows_core::Interface::as_raw(self), dw).ok()
    }
    pub unsafe fn ProviderFlags(&self, pdwflags: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ProviderFlags)(windows_core::Interface::as_raw(self), pdwflags).ok()
    }
    pub unsafe fn SetProviderFlags(&self, dwflags: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetProviderFlags)(windows_core::Interface::as_raw(self), dwflags).ok()
    }
    pub unsafe fn UseExistingKeySet(&self, fuseexistingkeys: *mut super::super::super::Foundation::BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UseExistingKeySet)(windows_core::Interface::as_raw(self), fuseexistingkeys).ok()
    }
    pub unsafe fn SetUseExistingKeySet<P0>(&self, fuseexistingkeys: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetUseExistingKeySet)(windows_core::Interface::as_raw(self), fuseexistingkeys.param().abi()).ok()
    }
    pub unsafe fn GenKeyFlags(&self, pdwflags: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GenKeyFlags)(windows_core::Interface::as_raw(self), pdwflags).ok()
    }
    pub unsafe fn SetGenKeyFlags(&self, dwflags: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetGenKeyFlags)(windows_core::Interface::as_raw(self), dwflags).ok()
    }
    pub unsafe fn DeleteRequestCert(&self, fdelete: *mut super::super::super::Foundation::BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DeleteRequestCert)(windows_core::Interface::as_raw(self), fdelete).ok()
    }
    pub unsafe fn SetDeleteRequestCert<P0>(&self, fdelete: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetDeleteRequestCert)(windows_core::Interface::as_raw(self), fdelete.param().abi()).ok()
    }
    pub unsafe fn WriteCertToUserDS(&self, fbool: *mut super::super::super::Foundation::BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).WriteCertToUserDS)(windows_core::Interface::as_raw(self), fbool).ok()
    }
    pub unsafe fn SetWriteCertToUserDS<P0>(&self, fbool: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetWriteCertToUserDS)(windows_core::Interface::as_raw(self), fbool.param().abi()).ok()
    }
    pub unsafe fn EnableT61DNEncoding(&self, fbool: *mut super::super::super::Foundation::BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnableT61DNEncoding)(windows_core::Interface::as_raw(self), fbool).ok()
    }
    pub unsafe fn SetEnableT61DNEncoding<P0>(&self, fbool: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetEnableT61DNEncoding)(windows_core::Interface::as_raw(self), fbool.param().abi()).ok()
    }
    pub unsafe fn WriteCertToCSP(&self, fbool: *mut super::super::super::Foundation::BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).WriteCertToCSP)(windows_core::Interface::as_raw(self), fbool).ok()
    }
    pub unsafe fn SetWriteCertToCSP<P0>(&self, fbool: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetWriteCertToCSP)(windows_core::Interface::as_raw(self), fbool.param().abi()).ok()
    }
    pub unsafe fn SPCFileNameWStr(&self, szw: *mut windows_core::PWSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SPCFileNameWStr)(windows_core::Interface::as_raw(self), szw).ok()
    }
    pub unsafe fn SetSPCFileNameWStr<P0>(&self, szw: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetSPCFileNameWStr)(windows_core::Interface::as_raw(self), szw.param().abi()).ok()
    }
    pub unsafe fn PVKFileNameWStr(&self, szw: *mut windows_core::PWSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).PVKFileNameWStr)(windows_core::Interface::as_raw(self), szw).ok()
    }
    pub unsafe fn SetPVKFileNameWStr<P0>(&self, szw: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetPVKFileNameWStr)(windows_core::Interface::as_raw(self), szw.param().abi()).ok()
    }
    pub unsafe fn HashAlgorithmWStr(&self, szw: *mut windows_core::PWSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).HashAlgorithmWStr)(windows_core::Interface::as_raw(self), szw).ok()
    }
    pub unsafe fn SetHashAlgorithmWStr<P0>(&self, szw: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetHashAlgorithmWStr)(windows_core::Interface::as_raw(self), szw.param().abi()).ok()
    }
    pub unsafe fn RenewalCertificate(&self, ppcertcontext: *mut *mut super::CERT_CONTEXT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RenewalCertificate)(windows_core::Interface::as_raw(self), ppcertcontext).ok()
    }
    pub unsafe fn SetRenewalCertificate(&self, pcertcontext: *const super::CERT_CONTEXT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetRenewalCertificate)(windows_core::Interface::as_raw(self), pcertcontext).ok()
    }
    pub unsafe fn AddCertTypeToRequestWStr<P0>(&self, szw: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).AddCertTypeToRequestWStr)(windows_core::Interface::as_raw(self), szw.param().abi()).ok()
    }
    pub unsafe fn AddNameValuePairToSignatureWStr<P0, P1>(&self, name: P0, value: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).AddNameValuePairToSignatureWStr)(windows_core::Interface::as_raw(self), name.param().abi(), value.param().abi()).ok()
    }
    pub unsafe fn AddExtensionsToRequest(&self, pcertextensions: *mut super::CERT_EXTENSIONS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddExtensionsToRequest)(windows_core::Interface::as_raw(self), pcertextensions).ok()
    }
    pub unsafe fn AddAuthenticatedAttributesToPKCS7Request(&self, pattributes: *mut super::CRYPT_ATTRIBUTES) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddAuthenticatedAttributesToPKCS7Request)(windows_core::Interface::as_raw(self), pattributes).ok()
    }
    pub unsafe fn CreatePKCS7RequestFromRequest(&self, prequest: *mut super::CRYPT_INTEGER_BLOB, psigningcertcontext: *const super::CERT_CONTEXT, ppkcs7blob: *mut super::CRYPT_INTEGER_BLOB) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CreatePKCS7RequestFromRequest)(windows_core::Interface::as_raw(self), prequest, psigningcertcontext, ppkcs7blob).ok()
    }
}
#[repr(C)]
pub struct IEnroll_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub createFilePKCS10WStr: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub acceptFilePKCS7WStr: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub createPKCS10WStr: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *mut super::CRYPT_INTEGER_BLOB) -> windows_core::HRESULT,
    pub acceptPKCS7Blob: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::CRYPT_INTEGER_BLOB) -> windows_core::HRESULT,
    pub getCertContextFromPKCS7: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::CRYPT_INTEGER_BLOB) -> *mut super::CERT_CONTEXT,
    pub getMyStore: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::HCERTSTORE,
    pub getCAStore: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::HCERTSTORE,
    pub getROOTHStore: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::HCERTSTORE,
    pub enumProvidersWStr: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub enumContainersWStr: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub freeRequestInfoBlob: unsafe extern "system" fn(*mut core::ffi::c_void, super::CRYPT_INTEGER_BLOB) -> windows_core::HRESULT,
    pub MyStoreNameWStr: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetMyStoreNameWStr: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub MyStoreTypeWStr: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetMyStoreTypeWStr: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub MyStoreFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMyStoreFlags: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub CAStoreNameWStr: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetCAStoreNameWStr: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub CAStoreTypeWStr: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetCAStoreTypeWStr: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub CAStoreFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetCAStoreFlags: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub RootStoreNameWStr: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetRootStoreNameWStr: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub RootStoreTypeWStr: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetRootStoreTypeWStr: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub RootStoreFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetRootStoreFlags: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub RequestStoreNameWStr: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetRequestStoreNameWStr: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub RequestStoreTypeWStr: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetRequestStoreTypeWStr: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub RequestStoreFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetRequestStoreFlags: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ContainerNameWStr: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetContainerNameWStr: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub ProviderNameWStr: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetProviderNameWStr: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub ProviderType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetProviderType: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub KeySpec: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetKeySpec: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ProviderFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetProviderFlags: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub UseExistingKeySet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetUseExistingKeySet: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GenKeyFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetGenKeyFlags: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub DeleteRequestCert: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetDeleteRequestCert: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub WriteCertToUserDS: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetWriteCertToUserDS: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub EnableT61DNEncoding: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetEnableT61DNEncoding: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub WriteCertToCSP: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetWriteCertToCSP: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SPCFileNameWStr: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetSPCFileNameWStr: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub PVKFileNameWStr: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetPVKFileNameWStr: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub HashAlgorithmWStr: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetHashAlgorithmWStr: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub RenewalCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::CERT_CONTEXT) -> windows_core::HRESULT,
    pub SetRenewalCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::CERT_CONTEXT) -> windows_core::HRESULT,
    pub AddCertTypeToRequestWStr: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub AddNameValuePairToSignatureWStr: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub AddExtensionsToRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::CERT_EXTENSIONS) -> windows_core::HRESULT,
    pub AddAuthenticatedAttributesToPKCS7Request: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::CRYPT_ATTRIBUTES) -> windows_core::HRESULT,
    pub CreatePKCS7RequestFromRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::CRYPT_INTEGER_BLOB, *const super::CERT_CONTEXT, *mut super::CRYPT_INTEGER_BLOB) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnroll2, IEnroll2_Vtbl, 0xc080e199_b7df_11d2_a421_00c04f79fe8e);
impl core::ops::Deref for IEnroll2 {
    type Target = IEnroll;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnroll2, windows_core::IUnknown, IEnroll);
impl IEnroll2 {
    pub unsafe fn InstallPKCS7Blob(&self, pblobpkcs7: *mut super::CRYPT_INTEGER_BLOB) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).InstallPKCS7Blob)(windows_core::Interface::as_raw(self), pblobpkcs7).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetSupportedKeySpec(&self, pdwkeyspec: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSupportedKeySpec)(windows_core::Interface::as_raw(self), pdwkeyspec).ok()
    }
    pub unsafe fn GetKeyLen<P0, P1>(&self, fmin: P0, fexchange: P1, pdwkeysize: *mut i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::BOOL>,
        P1: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).GetKeyLen)(windows_core::Interface::as_raw(self), fmin.param().abi(), fexchange.param().abi(), pdwkeysize).ok()
    }
    pub unsafe fn EnumAlgs(&self, dwindex: i32, algclass: i32, pdwalgid: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnumAlgs)(windows_core::Interface::as_raw(self), dwindex, algclass, pdwalgid).ok()
    }
    pub unsafe fn GetAlgNameWStr(&self, algid: i32, ppwsz: *mut windows_core::PWSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetAlgNameWStr)(windows_core::Interface::as_raw(self), algid, ppwsz).ok()
    }
    pub unsafe fn SetReuseHardwareKeyIfUnableToGenNew<P0>(&self, freusehardwarekeyifunabletogennew: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetReuseHardwareKeyIfUnableToGenNew)(windows_core::Interface::as_raw(self), freusehardwarekeyifunabletogennew.param().abi()).ok()
    }
    pub unsafe fn ReuseHardwareKeyIfUnableToGenNew(&self, freusehardwarekeyifunabletogennew: *mut super::super::super::Foundation::BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ReuseHardwareKeyIfUnableToGenNew)(windows_core::Interface::as_raw(self), freusehardwarekeyifunabletogennew).ok()
    }
    pub unsafe fn SetHashAlgID(&self, hashalgid: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetHashAlgID)(windows_core::Interface::as_raw(self), hashalgid).ok()
    }
    pub unsafe fn HashAlgID(&self, hashalgid: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).HashAlgID)(windows_core::Interface::as_raw(self), hashalgid).ok()
    }
    pub unsafe fn SetHStoreMy<P0>(&self, hstore: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::HCERTSTORE>,
    {
        (windows_core::Interface::vtable(self).SetHStoreMy)(windows_core::Interface::as_raw(self), hstore.param().abi()).ok()
    }
    pub unsafe fn SetHStoreCA<P0>(&self, hstore: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::HCERTSTORE>,
    {
        (windows_core::Interface::vtable(self).SetHStoreCA)(windows_core::Interface::as_raw(self), hstore.param().abi()).ok()
    }
    pub unsafe fn SetHStoreROOT<P0>(&self, hstore: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::HCERTSTORE>,
    {
        (windows_core::Interface::vtable(self).SetHStoreROOT)(windows_core::Interface::as_raw(self), hstore.param().abi()).ok()
    }
    pub unsafe fn SetHStoreRequest<P0>(&self, hstore: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::HCERTSTORE>,
    {
        (windows_core::Interface::vtable(self).SetHStoreRequest)(windows_core::Interface::as_raw(self), hstore.param().abi()).ok()
    }
    pub unsafe fn SetLimitExchangeKeyToEncipherment<P0>(&self, flimitexchangekeytoencipherment: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetLimitExchangeKeyToEncipherment)(windows_core::Interface::as_raw(self), flimitexchangekeytoencipherment.param().abi()).ok()
    }
    pub unsafe fn LimitExchangeKeyToEncipherment(&self, flimitexchangekeytoencipherment: *mut super::super::super::Foundation::BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).LimitExchangeKeyToEncipherment)(windows_core::Interface::as_raw(self), flimitexchangekeytoencipherment).ok()
    }
    pub unsafe fn SetEnableSMIMECapabilities<P0>(&self, fenablesmimecapabilities: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetEnableSMIMECapabilities)(windows_core::Interface::as_raw(self), fenablesmimecapabilities.param().abi()).ok()
    }
    pub unsafe fn EnableSMIMECapabilities(&self, fenablesmimecapabilities: *mut super::super::super::Foundation::BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnableSMIMECapabilities)(windows_core::Interface::as_raw(self), fenablesmimecapabilities).ok()
    }
}
#[repr(C)]
pub struct IEnroll2_Vtbl {
    pub base__: IEnroll_Vtbl,
    pub InstallPKCS7Blob: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::CRYPT_INTEGER_BLOB) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSupportedKeySpec: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetKeyLen: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::BOOL, super::super::super::Foundation::BOOL, *mut i32) -> windows_core::HRESULT,
    pub EnumAlgs: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut i32) -> windows_core::HRESULT,
    pub GetAlgNameWStr: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetReuseHardwareKeyIfUnableToGenNew: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub ReuseHardwareKeyIfUnableToGenNew: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetHashAlgID: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub HashAlgID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetHStoreMy: unsafe extern "system" fn(*mut core::ffi::c_void, super::HCERTSTORE) -> windows_core::HRESULT,
    pub SetHStoreCA: unsafe extern "system" fn(*mut core::ffi::c_void, super::HCERTSTORE) -> windows_core::HRESULT,
    pub SetHStoreROOT: unsafe extern "system" fn(*mut core::ffi::c_void, super::HCERTSTORE) -> windows_core::HRESULT,
    pub SetHStoreRequest: unsafe extern "system" fn(*mut core::ffi::c_void, super::HCERTSTORE) -> windows_core::HRESULT,
    pub SetLimitExchangeKeyToEncipherment: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub LimitExchangeKeyToEncipherment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetEnableSMIMECapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub EnableSMIMECapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnroll4, IEnroll4_Vtbl, 0xf8053fe5_78f4_448f_a0db_41d61b73446b);
impl core::ops::Deref for IEnroll4 {
    type Target = IEnroll2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnroll4, windows_core::IUnknown, IEnroll, IEnroll2);
impl IEnroll4 {
    pub unsafe fn SetThumbPrintWStr(&self, thumbprintblob: super::CRYPT_INTEGER_BLOB) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetThumbPrintWStr)(windows_core::Interface::as_raw(self), core::mem::transmute(thumbprintblob)).ok()
    }
    pub unsafe fn ThumbPrintWStr(&self, thumbprintblob: *mut super::CRYPT_INTEGER_BLOB) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ThumbPrintWStr)(windows_core::Interface::as_raw(self), thumbprintblob).ok()
    }
    pub unsafe fn SetPrivateKeyArchiveCertificate(&self, pprivatekeyarchivecert: *const super::CERT_CONTEXT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPrivateKeyArchiveCertificate)(windows_core::Interface::as_raw(self), pprivatekeyarchivecert).ok()
    }
    pub unsafe fn GetPrivateKeyArchiveCertificate(&self) -> *mut super::CERT_CONTEXT {
        (windows_core::Interface::vtable(self).GetPrivateKeyArchiveCertificate)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn binaryBlobToString(&self, flags: i32, pblobbinary: *mut super::CRYPT_INTEGER_BLOB, ppwszstring: *mut windows_core::PWSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).binaryBlobToString)(windows_core::Interface::as_raw(self), flags, pblobbinary, ppwszstring).ok()
    }
    pub unsafe fn stringToBinaryBlob<P0>(&self, flags: i32, pwszstring: P0, pblobbinary: *mut super::CRYPT_INTEGER_BLOB, pdwskip: *mut i32, pdwflags: *mut i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).stringToBinaryBlob)(windows_core::Interface::as_raw(self), flags, pwszstring.param().abi(), pblobbinary, pdwskip, pdwflags).ok()
    }
    pub unsafe fn addExtensionToRequestWStr<P0>(&self, flags: i32, pwszname: P0, pblobvalue: *mut super::CRYPT_INTEGER_BLOB) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).addExtensionToRequestWStr)(windows_core::Interface::as_raw(self), flags, pwszname.param().abi(), pblobvalue).ok()
    }
    pub unsafe fn addAttributeToRequestWStr<P0>(&self, flags: i32, pwszname: P0, pblobvalue: *mut super::CRYPT_INTEGER_BLOB) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).addAttributeToRequestWStr)(windows_core::Interface::as_raw(self), flags, pwszname.param().abi(), pblobvalue).ok()
    }
    pub unsafe fn addNameValuePairToRequestWStr<P0, P1>(&self, flags: i32, pwszname: P0, pwszvalue: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).addNameValuePairToRequestWStr)(windows_core::Interface::as_raw(self), flags, pwszname.param().abi(), pwszvalue.param().abi()).ok()
    }
    pub unsafe fn resetExtensions(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).resetExtensions)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn resetAttributes(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).resetAttributes)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn createRequestWStr<P0, P1>(&self, flags: CERT_CREATE_REQUEST_FLAGS, pwszdnname: P0, pwszusage: P1, pblobrequest: *mut super::CRYPT_INTEGER_BLOB) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).createRequestWStr)(windows_core::Interface::as_raw(self), flags, pwszdnname.param().abi(), pwszusage.param().abi(), pblobrequest).ok()
    }
    pub unsafe fn createFileRequestWStr<P0, P1, P2>(&self, flags: CERT_CREATE_REQUEST_FLAGS, pwszdnname: P0, pwszusage: P1, pwszrequestfilename: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).createFileRequestWStr)(windows_core::Interface::as_raw(self), flags, pwszdnname.param().abi(), pwszusage.param().abi(), pwszrequestfilename.param().abi()).ok()
    }
    pub unsafe fn acceptResponseBlob(&self, pblobresponse: *mut super::CRYPT_INTEGER_BLOB) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).acceptResponseBlob)(windows_core::Interface::as_raw(self), pblobresponse).ok()
    }
    pub unsafe fn acceptFileResponseWStr<P0>(&self, pwszresponsefilename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).acceptFileResponseWStr)(windows_core::Interface::as_raw(self), pwszresponsefilename.param().abi()).ok()
    }
    pub unsafe fn getCertContextFromResponseBlob(&self, pblobresponse: *mut super::CRYPT_INTEGER_BLOB, ppcertcontext: *mut *mut super::CERT_CONTEXT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).getCertContextFromResponseBlob)(windows_core::Interface::as_raw(self), pblobresponse, ppcertcontext).ok()
    }
    pub unsafe fn getCertContextFromFileResponseWStr<P0>(&self, pwszresponsefilename: P0, ppcertcontext: *mut *mut super::CERT_CONTEXT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).getCertContextFromFileResponseWStr)(windows_core::Interface::as_raw(self), pwszresponsefilename.param().abi(), ppcertcontext).ok()
    }
    pub unsafe fn createPFXWStr<P0>(&self, pwszpassword: P0, pblobpfx: *mut super::CRYPT_INTEGER_BLOB) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).createPFXWStr)(windows_core::Interface::as_raw(self), pwszpassword.param().abi(), pblobpfx).ok()
    }
    pub unsafe fn createFilePFXWStr<P0, P1>(&self, pwszpassword: P0, pwszpfxfilename: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).createFilePFXWStr)(windows_core::Interface::as_raw(self), pwszpassword.param().abi(), pwszpfxfilename.param().abi()).ok()
    }
    pub unsafe fn setPendingRequestInfoWStr<P0, P1, P2>(&self, lrequestid: i32, pwszcadns: P0, pwszcaname: P1, pwszfriendlyname: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).setPendingRequestInfoWStr)(windows_core::Interface::as_raw(self), lrequestid, pwszcadns.param().abi(), pwszcaname.param().abi(), pwszfriendlyname.param().abi()).ok()
    }
    pub unsafe fn enumPendingRequestWStr(&self, lindex: i32, ldesiredproperty: PENDING_REQUEST_DESIRED_PROPERTY, ppproperty: *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).enumPendingRequestWStr)(windows_core::Interface::as_raw(self), lindex, ldesiredproperty, ppproperty).ok()
    }
    pub unsafe fn removePendingRequestWStr(&self, thumbprintblob: super::CRYPT_INTEGER_BLOB) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).removePendingRequestWStr)(windows_core::Interface::as_raw(self), core::mem::transmute(thumbprintblob)).ok()
    }
    pub unsafe fn GetKeyLenEx(&self, lsizespec: XEKL_KEYSIZE, lkeyspec: XEKL_KEYSPEC, pdwkeysize: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetKeyLenEx)(windows_core::Interface::as_raw(self), lsizespec, lkeyspec, pdwkeysize).ok()
    }
    pub unsafe fn InstallPKCS7BlobEx(&self, pblobpkcs7: *mut super::CRYPT_INTEGER_BLOB, plcertinstalled: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).InstallPKCS7BlobEx)(windows_core::Interface::as_raw(self), pblobpkcs7, plcertinstalled).ok()
    }
    pub unsafe fn AddCertTypeToRequestWStrEx<P0, P1>(&self, ltype: ADDED_CERT_TYPE, pwszoidorname: P0, lmajorversion: i32, fminorversion: P1, lminorversion: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).AddCertTypeToRequestWStrEx)(windows_core::Interface::as_raw(self), ltype, pwszoidorname.param().abi(), lmajorversion, fminorversion.param().abi(), lminorversion).ok()
    }
    pub unsafe fn getProviderTypeWStr<P0>(&self, pwszprovname: P0, plprovtype: *mut i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).getProviderTypeWStr)(windows_core::Interface::as_raw(self), pwszprovname.param().abi(), plprovtype).ok()
    }
    pub unsafe fn addBlobPropertyToCertificateWStr(&self, lpropertyid: i32, lreserved: i32, pblobproperty: *mut super::CRYPT_INTEGER_BLOB) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).addBlobPropertyToCertificateWStr)(windows_core::Interface::as_raw(self), lpropertyid, lreserved, pblobproperty).ok()
    }
    pub unsafe fn SetSignerCertificate(&self, psignercert: *const super::CERT_CONTEXT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSignerCertificate)(windows_core::Interface::as_raw(self), psignercert).ok()
    }
    pub unsafe fn SetClientId(&self, lclientid: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetClientId)(windows_core::Interface::as_raw(self), lclientid).ok()
    }
    pub unsafe fn ClientId(&self, plclientid: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ClientId)(windows_core::Interface::as_raw(self), plclientid).ok()
    }
    pub unsafe fn SetIncludeSubjectKeyID<P0>(&self, finclude: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetIncludeSubjectKeyID)(windows_core::Interface::as_raw(self), finclude.param().abi()).ok()
    }
    pub unsafe fn IncludeSubjectKeyID(&self, pfinclude: *mut super::super::super::Foundation::BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).IncludeSubjectKeyID)(windows_core::Interface::as_raw(self), pfinclude).ok()
    }
}
#[repr(C)]
pub struct IEnroll4_Vtbl {
    pub base__: IEnroll2_Vtbl,
    pub SetThumbPrintWStr: unsafe extern "system" fn(*mut core::ffi::c_void, super::CRYPT_INTEGER_BLOB) -> windows_core::HRESULT,
    pub ThumbPrintWStr: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::CRYPT_INTEGER_BLOB) -> windows_core::HRESULT,
    pub SetPrivateKeyArchiveCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::CERT_CONTEXT) -> windows_core::HRESULT,
    pub GetPrivateKeyArchiveCertificate: unsafe extern "system" fn(*mut core::ffi::c_void) -> *mut super::CERT_CONTEXT,
    pub binaryBlobToString: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut super::CRYPT_INTEGER_BLOB, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub stringToBinaryBlob: unsafe extern "system" fn(*mut core::ffi::c_void, i32, windows_core::PCWSTR, *mut super::CRYPT_INTEGER_BLOB, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub addExtensionToRequestWStr: unsafe extern "system" fn(*mut core::ffi::c_void, i32, windows_core::PCWSTR, *mut super::CRYPT_INTEGER_BLOB) -> windows_core::HRESULT,
    pub addAttributeToRequestWStr: unsafe extern "system" fn(*mut core::ffi::c_void, i32, windows_core::PCWSTR, *mut super::CRYPT_INTEGER_BLOB) -> windows_core::HRESULT,
    pub addNameValuePairToRequestWStr: unsafe extern "system" fn(*mut core::ffi::c_void, i32, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub resetExtensions: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub resetAttributes: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub createRequestWStr: unsafe extern "system" fn(*mut core::ffi::c_void, CERT_CREATE_REQUEST_FLAGS, windows_core::PCWSTR, windows_core::PCWSTR, *mut super::CRYPT_INTEGER_BLOB) -> windows_core::HRESULT,
    pub createFileRequestWStr: unsafe extern "system" fn(*mut core::ffi::c_void, CERT_CREATE_REQUEST_FLAGS, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub acceptResponseBlob: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::CRYPT_INTEGER_BLOB) -> windows_core::HRESULT,
    pub acceptFileResponseWStr: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub getCertContextFromResponseBlob: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::CRYPT_INTEGER_BLOB, *mut *mut super::CERT_CONTEXT) -> windows_core::HRESULT,
    pub getCertContextFromFileResponseWStr: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut super::CERT_CONTEXT) -> windows_core::HRESULT,
    pub createPFXWStr: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut super::CRYPT_INTEGER_BLOB) -> windows_core::HRESULT,
    pub createFilePFXWStr: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub setPendingRequestInfoWStr: unsafe extern "system" fn(*mut core::ffi::c_void, i32, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub enumPendingRequestWStr: unsafe extern "system" fn(*mut core::ffi::c_void, i32, PENDING_REQUEST_DESIRED_PROPERTY, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub removePendingRequestWStr: unsafe extern "system" fn(*mut core::ffi::c_void, super::CRYPT_INTEGER_BLOB) -> windows_core::HRESULT,
    pub GetKeyLenEx: unsafe extern "system" fn(*mut core::ffi::c_void, XEKL_KEYSIZE, XEKL_KEYSPEC, *mut i32) -> windows_core::HRESULT,
    pub InstallPKCS7BlobEx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::CRYPT_INTEGER_BLOB, *mut i32) -> windows_core::HRESULT,
    pub AddCertTypeToRequestWStrEx: unsafe extern "system" fn(*mut core::ffi::c_void, ADDED_CERT_TYPE, windows_core::PCWSTR, i32, super::super::super::Foundation::BOOL, i32) -> windows_core::HRESULT,
    pub getProviderTypeWStr: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut i32) -> windows_core::HRESULT,
    pub addBlobPropertyToCertificateWStr: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut super::CRYPT_INTEGER_BLOB) -> windows_core::HRESULT,
    pub SetSignerCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::CERT_CONTEXT) -> windows_core::HRESULT,
    pub SetClientId: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ClientId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetIncludeSubjectKeyID: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub IncludeSubjectKeyID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IEnumCERTVIEWATTRIBUTE, IEnumCERTVIEWATTRIBUTE_Vtbl, 0xe77db656_7653_11d1_9bde_00c04fb683fa);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IEnumCERTVIEWATTRIBUTE {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IEnumCERTVIEWATTRIBUTE, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IEnumCERTVIEWATTRIBUTE {
    pub unsafe fn Next(&self, pindex: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), pindex).ok()
    }
    pub unsafe fn GetName(&self, pstrout: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), core::mem::transmute(pstrout)).ok()
    }
    pub unsafe fn GetValue(&self, pstrout: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetValue)(windows_core::Interface::as_raw(self), core::mem::transmute(pstrout)).ok()
    }
    pub unsafe fn Skip(&self, celt: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumCERTVIEWATTRIBUTE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IEnumCERTVIEWATTRIBUTE_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Clone: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IEnumCERTVIEWCOLUMN, IEnumCERTVIEWCOLUMN_Vtbl, 0x9c735be2_57a5_11d1_9bdb_00c04fb683fa);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IEnumCERTVIEWCOLUMN {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IEnumCERTVIEWCOLUMN, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IEnumCERTVIEWCOLUMN {
    pub unsafe fn Next(&self, pindex: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), pindex).ok()
    }
    pub unsafe fn GetName(&self, pstrout: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), core::mem::transmute(pstrout)).ok()
    }
    pub unsafe fn GetDisplayName(&self, pstrout: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDisplayName)(windows_core::Interface::as_raw(self), core::mem::transmute(pstrout)).ok()
    }
    pub unsafe fn GetType(&self, ptype: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self), ptype).ok()
    }
    pub unsafe fn IsIndexed(&self, pindexed: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).IsIndexed)(windows_core::Interface::as_raw(self), pindexed).ok()
    }
    pub unsafe fn GetMaxLength(&self, pmaxlength: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetMaxLength)(windows_core::Interface::as_raw(self), pmaxlength).ok()
    }
    pub unsafe fn GetValue(&self, flags: ENUM_CERT_COLUMN_VALUE_FLAGS, pvarvalue: *mut windows_core::VARIANT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetValue)(windows_core::Interface::as_raw(self), flags, core::mem::transmute(pvarvalue)).ok()
    }
    pub unsafe fn Skip(&self, celt: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumCERTVIEWCOLUMN> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IEnumCERTVIEWCOLUMN_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub IsIndexed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetMaxLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetValue: unsafe extern "system" fn(*mut core::ffi::c_void, ENUM_CERT_COLUMN_VALUE_FLAGS, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Clone: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IEnumCERTVIEWEXTENSION, IEnumCERTVIEWEXTENSION_Vtbl, 0xe7dd1466_7653_11d1_9bde_00c04fb683fa);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IEnumCERTVIEWEXTENSION {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IEnumCERTVIEWEXTENSION, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IEnumCERTVIEWEXTENSION {
    pub unsafe fn Next(&self, pindex: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), pindex).ok()
    }
    pub unsafe fn GetName(&self, pstrout: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), core::mem::transmute(pstrout)).ok()
    }
    pub unsafe fn GetFlags(&self, pflags: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFlags)(windows_core::Interface::as_raw(self), pflags).ok()
    }
    pub unsafe fn GetValue(&self, r#type: CERT_PROPERTY_TYPE, flags: ENUM_CERT_COLUMN_VALUE_FLAGS, pvarvalue: *mut windows_core::VARIANT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetValue)(windows_core::Interface::as_raw(self), r#type, flags, core::mem::transmute(pvarvalue)).ok()
    }
    pub unsafe fn Skip(&self, celt: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumCERTVIEWEXTENSION> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IEnumCERTVIEWEXTENSION_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetValue: unsafe extern "system" fn(*mut core::ffi::c_void, CERT_PROPERTY_TYPE, ENUM_CERT_COLUMN_VALUE_FLAGS, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Clone: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IEnumCERTVIEWROW, IEnumCERTVIEWROW_Vtbl, 0xd1157f4c_5af2_11d1_9bdc_00c04fb683fa);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IEnumCERTVIEWROW {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IEnumCERTVIEWROW, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IEnumCERTVIEWROW {
    pub unsafe fn Next(&self, pindex: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), pindex).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumCertViewColumn(&self) -> windows_core::Result<IEnumCERTVIEWCOLUMN> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumCertViewColumn)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumCertViewAttribute(&self, flags: i32) -> windows_core::Result<IEnumCERTVIEWATTRIBUTE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumCertViewAttribute)(windows_core::Interface::as_raw(self), flags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumCertViewExtension(&self, flags: i32) -> windows_core::Result<IEnumCERTVIEWEXTENSION> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumCertViewExtension)(windows_core::Interface::as_raw(self), flags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Skip(&self, celt: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumCERTVIEWROW> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetMaxIndex(&self, pindex: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetMaxIndex)(windows_core::Interface::as_raw(self), pindex).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IEnumCERTVIEWROW_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumCertViewColumn: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumCertViewColumn: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumCertViewAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumCertViewAttribute: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumCertViewExtension: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumCertViewExtension: usize,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Clone: usize,
    pub GetMaxIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INDESPolicy, INDESPolicy_Vtbl, 0x13ca515d_431d_46cc_8c2e_1da269bbd625);
impl core::ops::Deref for INDESPolicy {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(INDESPolicy, windows_core::IUnknown);
impl INDESPolicy {
    pub unsafe fn Initialize(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Uninitialize(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Uninitialize)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GenerateChallenge<P0, P1>(&self, pwsztemplate: P0, pwszparams: P1) -> windows_core::Result<windows_core::PWSTR>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GenerateChallenge)(windows_core::Interface::as_raw(self), pwsztemplate.param().abi(), pwszparams.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn VerifyRequest<P0, P1>(&self, pctbrequest: *mut CERTTRANSBLOB, pctbsigningcertencoded: *mut CERTTRANSBLOB, pwsztemplate: P0, pwsztransactionid: P1) -> windows_core::Result<super::super::super::Foundation::BOOL>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).VerifyRequest)(windows_core::Interface::as_raw(self), pctbrequest, pctbsigningcertencoded, pwsztemplate.param().abi(), pwsztransactionid.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn Notify<P0, P1>(&self, pwszchallenge: P0, pwsztransactionid: P1, disposition: X509SCEPDisposition, lasthresult: i32, pctbissuedcertencoded: *mut CERTTRANSBLOB) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).Notify)(windows_core::Interface::as_raw(self), pwszchallenge.param().abi(), pwsztransactionid.param().abi(), disposition, lasthresult, pctbissuedcertencoded).ok()
    }
}
#[repr(C)]
pub struct INDESPolicy_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Uninitialize: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GenerateChallenge: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub VerifyRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CERTTRANSBLOB, *mut CERTTRANSBLOB, windows_core::PCWSTR, windows_core::PCWSTR, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub Notify: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, X509SCEPDisposition, i32, *mut CERTTRANSBLOB) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IOCSPAdmin, IOCSPAdmin_Vtbl, 0x322e830d_67db_4fe9_9577_4596d9f09294);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IOCSPAdmin {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IOCSPAdmin, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IOCSPAdmin {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OCSPServiceProperties(&self) -> windows_core::Result<IOCSPPropertyCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OCSPServiceProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OCSPCAConfigurationCollection(&self) -> windows_core::Result<IOCSPCAConfigurationCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OCSPCAConfigurationCollection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetConfiguration<P0, P1>(&self, bstrservername: P0, bforce: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).GetConfiguration)(windows_core::Interface::as_raw(self), bstrservername.param().abi(), bforce.param().abi()).ok()
    }
    pub unsafe fn SetConfiguration<P0, P1>(&self, bstrservername: P0, bforce: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetConfiguration)(windows_core::Interface::as_raw(self), bstrservername.param().abi(), bforce.param().abi()).ok()
    }
    pub unsafe fn GetMyRoles<P0>(&self, bstrservername: P0) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMyRoles)(windows_core::Interface::as_raw(self), bstrservername.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn Ping<P0>(&self, bstrservername: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Ping)(windows_core::Interface::as_raw(self), bstrservername.param().abi()).ok()
    }
    pub unsafe fn SetSecurity<P0, P1>(&self, bstrservername: P0, bstrval: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetSecurity)(windows_core::Interface::as_raw(self), bstrservername.param().abi(), bstrval.param().abi()).ok()
    }
    pub unsafe fn GetSecurity<P0>(&self, bstrservername: P0) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSecurity)(windows_core::Interface::as_raw(self), bstrservername.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetSigningCertificates<P0>(&self, bstrservername: P0, pcacertvar: *const windows_core::VARIANT) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSigningCertificates)(windows_core::Interface::as_raw(self), bstrservername.param().abi(), core::mem::transmute(pcacertvar), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetHashAlgorithms<P0, P1>(&self, bstrservername: P0, bstrcaid: P1) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetHashAlgorithms)(windows_core::Interface::as_raw(self), bstrservername.param().abi(), bstrcaid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IOCSPAdmin_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub OCSPServiceProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OCSPServiceProperties: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OCSPCAConfigurationCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OCSPCAConfigurationCollection: usize,
    pub GetConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub GetMyRoles: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut i32) -> windows_core::HRESULT,
    pub Ping: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetSecurity: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetSecurity: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetSigningCertificates: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub GetHashAlgorithms: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IOCSPCAConfiguration, IOCSPCAConfiguration_Vtbl, 0xaec92b40_3d46_433f_87d1_b84d5c1e790d);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IOCSPCAConfiguration {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IOCSPCAConfiguration, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IOCSPCAConfiguration {
    pub unsafe fn Identifier(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Identifier)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CACertificate(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CACertificate)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn HashAlgorithm(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).HashAlgorithm)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetHashAlgorithm<P0>(&self, newval: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetHashAlgorithm)(windows_core::Interface::as_raw(self), newval.param().abi()).ok()
    }
    pub unsafe fn SigningFlags(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SigningFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetSigningFlags(&self, newval: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSigningFlags)(windows_core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn SigningCertificate(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SigningCertificate)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetSigningCertificate<P0>(&self, newval: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetSigningCertificate)(windows_core::Interface::as_raw(self), newval.param().abi()).ok()
    }
    pub unsafe fn ReminderDuration(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ReminderDuration)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetReminderDuration(&self, newval: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetReminderDuration)(windows_core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn ErrorCode(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ErrorCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CSPName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CSPName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn KeySpec(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).KeySpec)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ProviderCLSID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ProviderCLSID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetProviderCLSID<P0>(&self, newval: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetProviderCLSID)(windows_core::Interface::as_raw(self), newval.param().abi()).ok()
    }
    pub unsafe fn ProviderProperties(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ProviderProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetProviderProperties<P0>(&self, newval: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetProviderProperties)(windows_core::Interface::as_raw(self), newval.param().abi()).ok()
    }
    pub unsafe fn Modified(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Modified)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn LocalRevocationInformation(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LocalRevocationInformation)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetLocalRevocationInformation<P0>(&self, newval: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetLocalRevocationInformation)(windows_core::Interface::as_raw(self), newval.param().abi()).ok()
    }
    pub unsafe fn SigningCertificateTemplate(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SigningCertificateTemplate)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetSigningCertificateTemplate<P0>(&self, newval: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetSigningCertificateTemplate)(windows_core::Interface::as_raw(self), newval.param().abi()).ok()
    }
    pub unsafe fn CAConfig(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CAConfig)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetCAConfig<P0>(&self, newval: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetCAConfig)(windows_core::Interface::as_raw(self), newval.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IOCSPCAConfiguration_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub Identifier: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CACertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub HashAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetHashAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SigningFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetSigningFlags: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SigningCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetSigningCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub ReminderDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetReminderDuration: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub CSPName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub KeySpec: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ProviderCLSID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetProviderCLSID: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ProviderProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetProviderProperties: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Modified: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub LocalRevocationInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetLocalRevocationInformation: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SigningCertificateTemplate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetSigningCertificateTemplate: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CAConfig: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetCAConfig: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IOCSPCAConfigurationCollection, IOCSPCAConfigurationCollection_Vtbl, 0x2bebea0b_5ece_4f28_a91c_86b4bb20f0d3);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IOCSPCAConfigurationCollection {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IOCSPCAConfigurationCollection, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IOCSPCAConfigurationCollection {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn get_Item(&self, index: i32) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn get_ItemByName<P0>(&self, bstridentifier: P0) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_ItemByName)(windows_core::Interface::as_raw(self), bstridentifier.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateCAConfiguration<P0, P1>(&self, bstridentifier: P0, varcacert: P1) -> windows_core::Result<IOCSPCAConfiguration>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateCAConfiguration)(windows_core::Interface::as_raw(self), bstridentifier.param().abi(), varcacert.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DeleteCAConfiguration<P0>(&self, bstridentifier: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).DeleteCAConfiguration)(windows_core::Interface::as_raw(self), bstridentifier.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IOCSPCAConfigurationCollection_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_ItemByName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateCAConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateCAConfiguration: usize,
    pub DeleteCAConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IOCSPProperty, IOCSPProperty_Vtbl, 0x66fb7839_5f04_4c25_ad18_9ff1a8376ee0);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IOCSPProperty {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IOCSPProperty, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IOCSPProperty {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Value(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Value)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetValue<P0>(&self, newval: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), newval.param().abi()).ok()
    }
    pub unsafe fn Modified(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Modified)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IOCSPProperty_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Modified: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IOCSPPropertyCollection, IOCSPPropertyCollection_Vtbl, 0x2597c18d_54e6_4b74_9fa9_a6bfda99cbbe);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IOCSPPropertyCollection {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IOCSPPropertyCollection, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IOCSPPropertyCollection {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn get_Item(&self, index: i32) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn get_ItemByName<P0>(&self, bstrpropname: P0) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_ItemByName)(windows_core::Interface::as_raw(self), bstrpropname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateProperty<P0>(&self, bstrpropname: P0, pvarpropvalue: *const windows_core::VARIANT) -> windows_core::Result<IOCSPProperty>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateProperty)(windows_core::Interface::as_raw(self), bstrpropname.param().abi(), core::mem::transmute(pvarpropvalue), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DeleteProperty<P0>(&self, bstrpropname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).DeleteProperty)(windows_core::Interface::as_raw(self), bstrpropname.param().abi()).ok()
    }
    pub unsafe fn InitializeFromProperties(&self, pvarproperties: *const windows_core::VARIANT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).InitializeFromProperties)(windows_core::Interface::as_raw(self), core::mem::transmute(pvarproperties)).ok()
    }
    pub unsafe fn GetAllProperties(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAllProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IOCSPPropertyCollection_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_ItemByName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateProperty: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateProperty: usize,
    pub DeleteProperty: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub InitializeFromProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub GetAllProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IObjectId, IObjectId_Vtbl, 0x728ab300_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IObjectId {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IObjectId, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IObjectId {
    pub unsafe fn InitializeFromName(&self, name: CERTENROLL_OBJECTID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).InitializeFromName)(windows_core::Interface::as_raw(self), name).ok()
    }
    pub unsafe fn InitializeFromValue<P0>(&self, strvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).InitializeFromValue)(windows_core::Interface::as_raw(self), strvalue.param().abi()).ok()
    }
    pub unsafe fn InitializeFromAlgorithmName<P0>(&self, groupid: ObjectIdGroupId, keyflags: ObjectIdPublicKeyFlags, algflags: AlgorithmFlags, stralgorithmname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).InitializeFromAlgorithmName)(windows_core::Interface::as_raw(self), groupid, keyflags, algflags, stralgorithmname.param().abi()).ok()
    }
    pub unsafe fn Name(&self) -> windows_core::Result<CERTENROLL_OBJECTID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn FriendlyName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FriendlyName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetFriendlyName<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetFriendlyName)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn Value(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Value)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetAlgorithmName(&self, groupid: ObjectIdGroupId, keyflags: ObjectIdPublicKeyFlags) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAlgorithmName)(windows_core::Interface::as_raw(self), groupid, keyflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IObjectId_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub InitializeFromName: unsafe extern "system" fn(*mut core::ffi::c_void, CERTENROLL_OBJECTID) -> windows_core::HRESULT,
    pub InitializeFromValue: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub InitializeFromAlgorithmName: unsafe extern "system" fn(*mut core::ffi::c_void, ObjectIdGroupId, ObjectIdPublicKeyFlags, AlgorithmFlags, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CERTENROLL_OBJECTID) -> windows_core::HRESULT,
    pub FriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetFriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetAlgorithmName: unsafe extern "system" fn(*mut core::ffi::c_void, ObjectIdGroupId, ObjectIdPublicKeyFlags, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IObjectIds, IObjectIds_Vtbl, 0x728ab301_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IObjectIds {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IObjectIds, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IObjectIds {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_ItemByIndex(&self, index: i32) -> windows_core::Result<IObjectId> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_ItemByIndex)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Add<P0>(&self, pval: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IObjectId>,
    {
        (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), pval.param().abi()).ok()
    }
    pub unsafe fn Remove(&self, index: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), index).ok()
    }
    pub unsafe fn Clear(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddRange<P0>(&self, pvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IObjectIds>,
    {
        (windows_core::Interface::vtable(self).AddRange)(windows_core::Interface::as_raw(self), pvalue.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IObjectIds_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub get_ItemByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_ItemByIndex: usize,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AddRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddRange: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IPolicyQualifier, IPolicyQualifier_Vtbl, 0x728ab31c_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IPolicyQualifier {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IPolicyQualifier, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IPolicyQualifier {
    pub unsafe fn InitializeEncode<P0>(&self, strqualifier: P0, r#type: PolicyQualifierType) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).InitializeEncode)(windows_core::Interface::as_raw(self), strqualifier.param().abi(), r#type).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ObjectId(&self) -> windows_core::Result<IObjectId> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ObjectId)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Qualifier(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Qualifier)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Type(&self) -> windows_core::Result<PolicyQualifierType> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Type)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn get_RawData(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_RawData)(windows_core::Interface::as_raw(self), encoding, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IPolicyQualifier_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub InitializeEncode: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, PolicyQualifierType) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ObjectId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ObjectId: usize,
    pub Qualifier: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PolicyQualifierType) -> windows_core::HRESULT,
    pub get_RawData: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IPolicyQualifiers, IPolicyQualifiers_Vtbl, 0x728ab31d_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IPolicyQualifiers {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IPolicyQualifiers, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IPolicyQualifiers {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_ItemByIndex(&self, index: i32) -> windows_core::Result<IPolicyQualifier> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_ItemByIndex)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Add<P0>(&self, pval: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IPolicyQualifier>,
    {
        (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), pval.param().abi()).ok()
    }
    pub unsafe fn Remove(&self, index: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), index).ok()
    }
    pub unsafe fn Clear(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IPolicyQualifiers_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub get_ItemByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_ItemByIndex: usize,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISignerCertificate, ISignerCertificate_Vtbl, 0x728ab33d_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISignerCertificate {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISignerCertificate, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISignerCertificate {
    pub unsafe fn Initialize<P0, P1>(&self, machinecontext: P0, verifytype: X509PrivateKeyVerify, encoding: EncodingType, strcertificate: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::VARIANT_BOOL>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), machinecontext.param().abi(), verifytype, encoding, strcertificate.param().abi()).ok()
    }
    pub unsafe fn get_Certificate(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Certificate)(windows_core::Interface::as_raw(self), encoding, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PrivateKey(&self) -> windows_core::Result<IX509PrivateKey> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PrivateKey)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Silent(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Silent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetSilent<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetSilent)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn ParentWindow(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ParentWindow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetParentWindow(&self, value: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetParentWindow)(windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn UIContextMessage(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UIContextMessage)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetUIContextMessage<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetUIContextMessage)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn SetPin<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetPin)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SignatureInformation(&self) -> windows_core::Result<IX509SignatureInformation> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SignatureInformation)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISignerCertificate_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::VARIANT_BOOL, X509PrivateKeyVerify, EncodingType, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub get_Certificate: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub PrivateKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PrivateKey: usize,
    pub Silent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetSilent: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ParentWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetParentWindow: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub UIContextMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetUIContextMessage: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetPin: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SignatureInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SignatureInformation: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISignerCertificates, ISignerCertificates_Vtbl, 0x728ab33e_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISignerCertificates {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISignerCertificates, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISignerCertificates {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_ItemByIndex(&self, index: i32) -> windows_core::Result<ISignerCertificate> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_ItemByIndex)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Add<P0>(&self, pval: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISignerCertificate>,
    {
        (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), pval.param().abi()).ok()
    }
    pub unsafe fn Remove(&self, index: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), index).ok()
    }
    pub unsafe fn Clear(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Find<P0>(&self, psignercert: P0) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<ISignerCertificate>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Find)(windows_core::Interface::as_raw(self), psignercert.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISignerCertificates_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub get_ItemByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_ItemByIndex: usize,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Find: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Find: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISmimeCapabilities, ISmimeCapabilities_Vtbl, 0x728ab31a_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISmimeCapabilities {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISmimeCapabilities, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISmimeCapabilities {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_ItemByIndex(&self, index: i32) -> windows_core::Result<ISmimeCapability> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_ItemByIndex)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Add<P0>(&self, pval: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISmimeCapability>,
    {
        (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), pval.param().abi()).ok()
    }
    pub unsafe fn Remove(&self, index: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), index).ok()
    }
    pub unsafe fn Clear(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddFromCsp<P0>(&self, pvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ICspInformation>,
    {
        (windows_core::Interface::vtable(self).AddFromCsp)(windows_core::Interface::as_raw(self), pvalue.param().abi()).ok()
    }
    pub unsafe fn AddAvailableSmimeCapabilities<P0>(&self, machinecontext: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).AddAvailableSmimeCapabilities)(windows_core::Interface::as_raw(self), machinecontext.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISmimeCapabilities_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub get_ItemByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_ItemByIndex: usize,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AddFromCsp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddFromCsp: usize,
    pub AddAvailableSmimeCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISmimeCapability, ISmimeCapability_Vtbl, 0x728ab319_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISmimeCapability {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISmimeCapability, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISmimeCapability {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Initialize<P0>(&self, pobjectid: P0, bitcount: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IObjectId>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pobjectid.param().abi(), bitcount).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ObjectId(&self) -> windows_core::Result<IObjectId> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ObjectId)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn BitCount(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BitCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISmimeCapability_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Initialize: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ObjectId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ObjectId: usize,
    pub BitCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IX500DistinguishedName, IX500DistinguishedName_Vtbl, 0x728ab303_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IX500DistinguishedName {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IX500DistinguishedName, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IX500DistinguishedName {
    pub unsafe fn Decode<P0>(&self, strencodedname: P0, encoding: EncodingType, nameflags: X500NameFlags) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Decode)(windows_core::Interface::as_raw(self), strencodedname.param().abi(), encoding, nameflags).ok()
    }
    pub unsafe fn Encode<P0>(&self, strname: P0, nameflags: X500NameFlags) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Encode)(windows_core::Interface::as_raw(self), strname.param().abi(), nameflags).ok()
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn get_EncodedName(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_EncodedName)(windows_core::Interface::as_raw(self), encoding, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX500DistinguishedName_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub Decode: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, EncodingType, X500NameFlags) -> windows_core::HRESULT,
    pub Encode: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, X500NameFlags) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub get_EncodedName: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IX509Attribute, IX509Attribute_Vtbl, 0x728ab322_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IX509Attribute {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IX509Attribute, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IX509Attribute {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Initialize<P0, P1>(&self, pobjectid: P0, encoding: EncodingType, strencodeddata: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IObjectId>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pobjectid.param().abi(), encoding, strencodeddata.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ObjectId(&self) -> windows_core::Result<IObjectId> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ObjectId)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn get_RawData(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_RawData)(windows_core::Interface::as_raw(self), encoding, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509Attribute_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, EncodingType, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Initialize: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ObjectId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ObjectId: usize,
    pub get_RawData: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IX509AttributeArchiveKey, IX509AttributeArchiveKey_Vtbl, 0x728ab327_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IX509AttributeArchiveKey {
    type Target = IX509Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IX509AttributeArchiveKey, windows_core::IUnknown, super::super::super::System::Com::IDispatch, IX509Attribute);
#[cfg(feature = "Win32_System_Com")]
impl IX509AttributeArchiveKey {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitializeEncode<P0, P1, P2>(&self, pkey: P0, encoding: EncodingType, strcaxcert: P1, palgorithm: P2, encryptionstrength: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IX509PrivateKey>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<IObjectId>,
    {
        (windows_core::Interface::vtable(self).InitializeEncode)(windows_core::Interface::as_raw(self), pkey.param().abi(), encoding, strcaxcert.param().abi(), palgorithm.param().abi(), encryptionstrength).ok()
    }
    pub unsafe fn InitializeDecode<P0>(&self, encoding: EncodingType, strencodeddata: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).InitializeDecode)(windows_core::Interface::as_raw(self), encoding, strencodeddata.param().abi()).ok()
    }
    pub unsafe fn get_EncryptedKeyBlob(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_EncryptedKeyBlob)(windows_core::Interface::as_raw(self), encoding, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EncryptionAlgorithm(&self) -> windows_core::Result<IObjectId> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EncryptionAlgorithm)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EncryptionStrength(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EncryptionStrength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509AttributeArchiveKey_Vtbl {
    pub base__: IX509Attribute_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub InitializeEncode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, EncodingType, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InitializeEncode: usize,
    pub InitializeDecode: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub get_EncryptedKeyBlob: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub EncryptionAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EncryptionAlgorithm: usize,
    pub EncryptionStrength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IX509AttributeArchiveKeyHash, IX509AttributeArchiveKeyHash_Vtbl, 0x728ab328_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IX509AttributeArchiveKeyHash {
    type Target = IX509Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IX509AttributeArchiveKeyHash, windows_core::IUnknown, super::super::super::System::Com::IDispatch, IX509Attribute);
#[cfg(feature = "Win32_System_Com")]
impl IX509AttributeArchiveKeyHash {
    pub unsafe fn InitializeEncodeFromEncryptedKeyBlob<P0>(&self, encoding: EncodingType, strencryptedkeyblob: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).InitializeEncodeFromEncryptedKeyBlob)(windows_core::Interface::as_raw(self), encoding, strencryptedkeyblob.param().abi()).ok()
    }
    pub unsafe fn InitializeDecode<P0>(&self, encoding: EncodingType, strencodeddata: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).InitializeDecode)(windows_core::Interface::as_raw(self), encoding, strencodeddata.param().abi()).ok()
    }
    pub unsafe fn get_EncryptedKeyHashBlob(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_EncryptedKeyHashBlob)(windows_core::Interface::as_raw(self), encoding, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509AttributeArchiveKeyHash_Vtbl {
    pub base__: IX509Attribute_Vtbl,
    pub InitializeEncodeFromEncryptedKeyBlob: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub InitializeDecode: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub get_EncryptedKeyHashBlob: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IX509AttributeClientId, IX509AttributeClientId_Vtbl, 0x728ab325_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IX509AttributeClientId {
    type Target = IX509Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IX509AttributeClientId, windows_core::IUnknown, super::super::super::System::Com::IDispatch, IX509Attribute);
#[cfg(feature = "Win32_System_Com")]
impl IX509AttributeClientId {
    pub unsafe fn InitializeEncode<P0, P1, P2>(&self, clientid: RequestClientInfoClientId, strmachinednsname: P0, strusersamname: P1, strprocessname: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).InitializeEncode)(windows_core::Interface::as_raw(self), clientid, strmachinednsname.param().abi(), strusersamname.param().abi(), strprocessname.param().abi()).ok()
    }
    pub unsafe fn InitializeDecode<P0>(&self, encoding: EncodingType, strencodeddata: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).InitializeDecode)(windows_core::Interface::as_raw(self), encoding, strencodeddata.param().abi()).ok()
    }
    pub unsafe fn ClientId(&self) -> windows_core::Result<RequestClientInfoClientId> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ClientId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn MachineDnsName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MachineDnsName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn UserSamName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UserSamName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ProcessName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ProcessName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509AttributeClientId_Vtbl {
    pub base__: IX509Attribute_Vtbl,
    pub InitializeEncode: unsafe extern "system" fn(*mut core::ffi::c_void, RequestClientInfoClientId, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub InitializeDecode: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ClientId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RequestClientInfoClientId) -> windows_core::HRESULT,
    pub MachineDnsName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub UserSamName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ProcessName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IX509AttributeCspProvider, IX509AttributeCspProvider_Vtbl, 0x728ab32b_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IX509AttributeCspProvider {
    type Target = IX509Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IX509AttributeCspProvider, windows_core::IUnknown, super::super::super::System::Com::IDispatch, IX509Attribute);
#[cfg(feature = "Win32_System_Com")]
impl IX509AttributeCspProvider {
    pub unsafe fn InitializeEncode<P0, P1>(&self, keyspec: X509KeySpec, strprovidername: P0, encoding: EncodingType, strsignature: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).InitializeEncode)(windows_core::Interface::as_raw(self), keyspec, strprovidername.param().abi(), encoding, strsignature.param().abi()).ok()
    }
    pub unsafe fn InitializeDecode<P0>(&self, encoding: EncodingType, strencodeddata: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).InitializeDecode)(windows_core::Interface::as_raw(self), encoding, strencodeddata.param().abi()).ok()
    }
    pub unsafe fn KeySpec(&self) -> windows_core::Result<X509KeySpec> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).KeySpec)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ProviderName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ProviderName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn get_Signature(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Signature)(windows_core::Interface::as_raw(self), encoding, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509AttributeCspProvider_Vtbl {
    pub base__: IX509Attribute_Vtbl,
    pub InitializeEncode: unsafe extern "system" fn(*mut core::ffi::c_void, X509KeySpec, core::mem::MaybeUninit<windows_core::BSTR>, EncodingType, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub InitializeDecode: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub KeySpec: unsafe extern "system" fn(*mut core::ffi::c_void, *mut X509KeySpec) -> windows_core::HRESULT,
    pub ProviderName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub get_Signature: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IX509AttributeExtensions, IX509AttributeExtensions_Vtbl, 0x728ab324_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IX509AttributeExtensions {
    type Target = IX509Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IX509AttributeExtensions, windows_core::IUnknown, super::super::super::System::Com::IDispatch, IX509Attribute);
#[cfg(feature = "Win32_System_Com")]
impl IX509AttributeExtensions {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitializeEncode<P0>(&self, pextensions: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IX509Extensions>,
    {
        (windows_core::Interface::vtable(self).InitializeEncode)(windows_core::Interface::as_raw(self), pextensions.param().abi()).ok()
    }
    pub unsafe fn InitializeDecode<P0>(&self, encoding: EncodingType, strencodeddata: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).InitializeDecode)(windows_core::Interface::as_raw(self), encoding, strencodeddata.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn X509Extensions(&self) -> windows_core::Result<IX509Extensions> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).X509Extensions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509AttributeExtensions_Vtbl {
    pub base__: IX509Attribute_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub InitializeEncode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InitializeEncode: usize,
    pub InitializeDecode: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub X509Extensions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    X509Extensions: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IX509AttributeOSVersion, IX509AttributeOSVersion_Vtbl, 0x728ab32a_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IX509AttributeOSVersion {
    type Target = IX509Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IX509AttributeOSVersion, windows_core::IUnknown, super::super::super::System::Com::IDispatch, IX509Attribute);
#[cfg(feature = "Win32_System_Com")]
impl IX509AttributeOSVersion {
    pub unsafe fn InitializeEncode<P0>(&self, strosversion: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).InitializeEncode)(windows_core::Interface::as_raw(self), strosversion.param().abi()).ok()
    }
    pub unsafe fn InitializeDecode<P0>(&self, encoding: EncodingType, strencodeddata: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).InitializeDecode)(windows_core::Interface::as_raw(self), encoding, strencodeddata.param().abi()).ok()
    }
    pub unsafe fn OSVersion(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OSVersion)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509AttributeOSVersion_Vtbl {
    pub base__: IX509Attribute_Vtbl,
    pub InitializeEncode: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub InitializeDecode: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub OSVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IX509AttributeRenewalCertificate, IX509AttributeRenewalCertificate_Vtbl, 0x728ab326_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IX509AttributeRenewalCertificate {
    type Target = IX509Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IX509AttributeRenewalCertificate, windows_core::IUnknown, super::super::super::System::Com::IDispatch, IX509Attribute);
#[cfg(feature = "Win32_System_Com")]
impl IX509AttributeRenewalCertificate {
    pub unsafe fn InitializeEncode<P0>(&self, encoding: EncodingType, strcert: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).InitializeEncode)(windows_core::Interface::as_raw(self), encoding, strcert.param().abi()).ok()
    }
    pub unsafe fn InitializeDecode<P0>(&self, encoding: EncodingType, strencodeddata: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).InitializeDecode)(windows_core::Interface::as_raw(self), encoding, strencodeddata.param().abi()).ok()
    }
    pub unsafe fn get_RenewalCertificate(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_RenewalCertificate)(windows_core::Interface::as_raw(self), encoding, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509AttributeRenewalCertificate_Vtbl {
    pub base__: IX509Attribute_Vtbl,
    pub InitializeEncode: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub InitializeDecode: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub get_RenewalCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IX509Attributes, IX509Attributes_Vtbl, 0x728ab323_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IX509Attributes {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IX509Attributes, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IX509Attributes {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_ItemByIndex(&self, index: i32) -> windows_core::Result<IX509Attribute> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_ItemByIndex)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Add<P0>(&self, pval: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IX509Attribute>,
    {
        (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), pval.param().abi()).ok()
    }
    pub unsafe fn Remove(&self, index: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), index).ok()
    }
    pub unsafe fn Clear(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509Attributes_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub get_ItemByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_ItemByIndex: usize,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IX509CertificateRequest, IX509CertificateRequest_Vtbl, 0x728ab341_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IX509CertificateRequest {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IX509CertificateRequest, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IX509CertificateRequest {
    pub unsafe fn Initialize(&self, context: X509CertificateEnrollmentContext) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), context).ok()
    }
    pub unsafe fn Encode(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Encode)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ResetForEncode(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ResetForEncode)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetInnerRequest(&self, level: InnerRequestLevel) -> windows_core::Result<IX509CertificateRequest> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetInnerRequest)(windows_core::Interface::as_raw(self), level, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Type(&self) -> windows_core::Result<X509RequestType> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Type)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn EnrollmentContext(&self) -> windows_core::Result<X509CertificateEnrollmentContext> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnrollmentContext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Silent(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Silent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetSilent<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetSilent)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn ParentWindow(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ParentWindow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetParentWindow(&self, value: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetParentWindow)(windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn UIContextMessage(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UIContextMessage)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetUIContextMessage<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetUIContextMessage)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn SuppressDefaults(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SuppressDefaults)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetSuppressDefaults<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetSuppressDefaults)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn get_RenewalCertificate(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_RenewalCertificate)(windows_core::Interface::as_raw(self), encoding, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn put_RenewalCertificate<P0>(&self, encoding: EncodingType, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).put_RenewalCertificate)(windows_core::Interface::as_raw(self), encoding, value.param().abi()).ok()
    }
    pub unsafe fn ClientId(&self) -> windows_core::Result<RequestClientInfoClientId> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ClientId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetClientId(&self, value: RequestClientInfoClientId) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetClientId)(windows_core::Interface::as_raw(self), value).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CspInformations(&self) -> windows_core::Result<ICspInformations> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CspInformations)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetCspInformations<P0>(&self, pvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ICspInformations>,
    {
        (windows_core::Interface::vtable(self).SetCspInformations)(windows_core::Interface::as_raw(self), pvalue.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn HashAlgorithm(&self) -> windows_core::Result<IObjectId> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).HashAlgorithm)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetHashAlgorithm<P0>(&self, pvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IObjectId>,
    {
        (windows_core::Interface::vtable(self).SetHashAlgorithm)(windows_core::Interface::as_raw(self), pvalue.param().abi()).ok()
    }
    pub unsafe fn AlternateSignatureAlgorithm(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AlternateSignatureAlgorithm)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAlternateSignatureAlgorithm<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetAlternateSignatureAlgorithm)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn get_RawData(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_RawData)(windows_core::Interface::as_raw(self), encoding, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509CertificateRequest_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, X509CertificateEnrollmentContext) -> windows_core::HRESULT,
    pub Encode: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ResetForEncode: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetInnerRequest: unsafe extern "system" fn(*mut core::ffi::c_void, InnerRequestLevel, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetInnerRequest: usize,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut X509RequestType) -> windows_core::HRESULT,
    pub EnrollmentContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut X509CertificateEnrollmentContext) -> windows_core::HRESULT,
    pub Silent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetSilent: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ParentWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetParentWindow: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub UIContextMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetUIContextMessage: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SuppressDefaults: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetSuppressDefaults: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub get_RenewalCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub put_RenewalCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ClientId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RequestClientInfoClientId) -> windows_core::HRESULT,
    pub SetClientId: unsafe extern "system" fn(*mut core::ffi::c_void, RequestClientInfoClientId) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CspInformations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CspInformations: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetCspInformations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetCspInformations: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub HashAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    HashAlgorithm: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetHashAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetHashAlgorithm: usize,
    pub AlternateSignatureAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetAlternateSignatureAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub get_RawData: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IX509CertificateRequestCertificate, IX509CertificateRequestCertificate_Vtbl, 0x728ab343_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IX509CertificateRequestCertificate {
    type Target = IX509CertificateRequestPkcs10;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IX509CertificateRequestCertificate, windows_core::IUnknown, super::super::super::System::Com::IDispatch, IX509CertificateRequest, IX509CertificateRequestPkcs10);
#[cfg(feature = "Win32_System_Com")]
impl IX509CertificateRequestCertificate {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CheckPublicKeySignature<P0>(&self, ppublickey: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IX509PublicKey>,
    {
        (windows_core::Interface::vtable(self).CheckPublicKeySignature)(windows_core::Interface::as_raw(self), ppublickey.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Issuer(&self) -> windows_core::Result<IX500DistinguishedName> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Issuer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetIssuer<P0>(&self, pvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IX500DistinguishedName>,
    {
        (windows_core::Interface::vtable(self).SetIssuer)(windows_core::Interface::as_raw(self), pvalue.param().abi()).ok()
    }
    pub unsafe fn NotBefore(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NotBefore)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetNotBefore(&self, value: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetNotBefore)(windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn NotAfter(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NotAfter)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetNotAfter(&self, value: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetNotAfter)(windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn get_SerialNumber(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_SerialNumber)(windows_core::Interface::as_raw(self), encoding, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn put_SerialNumber<P0>(&self, encoding: EncodingType, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).put_SerialNumber)(windows_core::Interface::as_raw(self), encoding, value.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SignerCertificate(&self) -> windows_core::Result<ISignerCertificate> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SignerCertificate)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSignerCertificate<P0>(&self, pvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISignerCertificate>,
    {
        (windows_core::Interface::vtable(self).SetSignerCertificate)(windows_core::Interface::as_raw(self), pvalue.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509CertificateRequestCertificate_Vtbl {
    pub base__: IX509CertificateRequestPkcs10_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CheckPublicKeySignature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CheckPublicKeySignature: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Issuer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Issuer: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetIssuer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetIssuer: usize,
    pub NotBefore: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetNotBefore: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub NotAfter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetNotAfter: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub get_SerialNumber: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub put_SerialNumber: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SignerCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SignerCertificate: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSignerCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSignerCertificate: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IX509CertificateRequestCertificate2, IX509CertificateRequestCertificate2_Vtbl, 0x728ab35a_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IX509CertificateRequestCertificate2 {
    type Target = IX509CertificateRequestCertificate;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IX509CertificateRequestCertificate2, windows_core::IUnknown, super::super::super::System::Com::IDispatch, IX509CertificateRequest, IX509CertificateRequestPkcs10, IX509CertificateRequestCertificate);
#[cfg(feature = "Win32_System_Com")]
impl IX509CertificateRequestCertificate2 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitializeFromTemplate<P0, P1>(&self, context: X509CertificateEnrollmentContext, ppolicyserver: P0, ptemplate: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IX509EnrollmentPolicyServer>,
        P1: windows_core::Param<IX509CertificateTemplate>,
    {
        (windows_core::Interface::vtable(self).InitializeFromTemplate)(windows_core::Interface::as_raw(self), context, ppolicyserver.param().abi(), ptemplate.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitializeFromPrivateKeyTemplate<P0, P1, P2>(&self, context: X509CertificateEnrollmentContext, pprivatekey: P0, ppolicyserver: P1, ptemplate: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IX509PrivateKey>,
        P1: windows_core::Param<IX509EnrollmentPolicyServer>,
        P2: windows_core::Param<IX509CertificateTemplate>,
    {
        (windows_core::Interface::vtable(self).InitializeFromPrivateKeyTemplate)(windows_core::Interface::as_raw(self), context, pprivatekey.param().abi(), ppolicyserver.param().abi(), ptemplate.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PolicyServer(&self) -> windows_core::Result<IX509EnrollmentPolicyServer> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PolicyServer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Template(&self) -> windows_core::Result<IX509CertificateTemplate> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Template)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509CertificateRequestCertificate2_Vtbl {
    pub base__: IX509CertificateRequestCertificate_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub InitializeFromTemplate: unsafe extern "system" fn(*mut core::ffi::c_void, X509CertificateEnrollmentContext, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InitializeFromTemplate: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub InitializeFromPrivateKeyTemplate: unsafe extern "system" fn(*mut core::ffi::c_void, X509CertificateEnrollmentContext, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InitializeFromPrivateKeyTemplate: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PolicyServer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PolicyServer: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Template: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Template: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IX509CertificateRequestCmc, IX509CertificateRequestCmc_Vtbl, 0x728ab345_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IX509CertificateRequestCmc {
    type Target = IX509CertificateRequestPkcs7;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IX509CertificateRequestCmc, windows_core::IUnknown, super::super::super::System::Com::IDispatch, IX509CertificateRequest, IX509CertificateRequestPkcs7);
#[cfg(feature = "Win32_System_Com")]
impl IX509CertificateRequestCmc {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitializeFromInnerRequestTemplateName<P0, P1>(&self, pinnerrequest: P0, strtemplatename: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IX509CertificateRequest>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).InitializeFromInnerRequestTemplateName)(windows_core::Interface::as_raw(self), pinnerrequest.param().abi(), strtemplatename.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn TemplateObjectId(&self) -> windows_core::Result<IObjectId> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TemplateObjectId)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn NullSigned(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NullSigned)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CryptAttributes(&self) -> windows_core::Result<ICryptAttributes> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CryptAttributes)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn NameValuePairs(&self) -> windows_core::Result<IX509NameValuePairs> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NameValuePairs)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn X509Extensions(&self) -> windows_core::Result<IX509Extensions> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).X509Extensions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CriticalExtensions(&self) -> windows_core::Result<IObjectIds> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CriticalExtensions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SuppressOids(&self) -> windows_core::Result<IObjectIds> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SuppressOids)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn TransactionId(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TransactionId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetTransactionId(&self, value: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetTransactionId)(windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn get_SenderNonce(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_SenderNonce)(windows_core::Interface::as_raw(self), encoding, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn put_SenderNonce<P0>(&self, encoding: EncodingType, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).put_SenderNonce)(windows_core::Interface::as_raw(self), encoding, value.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SignatureInformation(&self) -> windows_core::Result<IX509SignatureInformation> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SignatureInformation)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ArchivePrivateKey(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ArchivePrivateKey)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetArchivePrivateKey<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetArchivePrivateKey)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn get_KeyArchivalCertificate(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_KeyArchivalCertificate)(windows_core::Interface::as_raw(self), encoding, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn put_KeyArchivalCertificate<P0>(&self, encoding: EncodingType, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).put_KeyArchivalCertificate)(windows_core::Interface::as_raw(self), encoding, value.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EncryptionAlgorithm(&self) -> windows_core::Result<IObjectId> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EncryptionAlgorithm)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetEncryptionAlgorithm<P0>(&self, pvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IObjectId>,
    {
        (windows_core::Interface::vtable(self).SetEncryptionAlgorithm)(windows_core::Interface::as_raw(self), pvalue.param().abi()).ok()
    }
    pub unsafe fn EncryptionStrength(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EncryptionStrength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetEncryptionStrength(&self, value: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetEncryptionStrength)(windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn get_EncryptedKeyHash(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_EncryptedKeyHash)(windows_core::Interface::as_raw(self), encoding, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SignerCertificates(&self) -> windows_core::Result<ISignerCertificates> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SignerCertificates)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509CertificateRequestCmc_Vtbl {
    pub base__: IX509CertificateRequestPkcs7_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub InitializeFromInnerRequestTemplateName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InitializeFromInnerRequestTemplateName: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub TemplateObjectId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    TemplateObjectId: usize,
    pub NullSigned: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CryptAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CryptAttributes: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub NameValuePairs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    NameValuePairs: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub X509Extensions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    X509Extensions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CriticalExtensions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CriticalExtensions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SuppressOids: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SuppressOids: usize,
    pub TransactionId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetTransactionId: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_SenderNonce: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub put_SenderNonce: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SignatureInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SignatureInformation: usize,
    pub ArchivePrivateKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetArchivePrivateKey: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub get_KeyArchivalCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub put_KeyArchivalCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub EncryptionAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EncryptionAlgorithm: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetEncryptionAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetEncryptionAlgorithm: usize,
    pub EncryptionStrength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetEncryptionStrength: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_EncryptedKeyHash: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SignerCertificates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SignerCertificates: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IX509CertificateRequestCmc2, IX509CertificateRequestCmc2_Vtbl, 0x728ab35d_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IX509CertificateRequestCmc2 {
    type Target = IX509CertificateRequestCmc;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IX509CertificateRequestCmc2, windows_core::IUnknown, super::super::super::System::Com::IDispatch, IX509CertificateRequest, IX509CertificateRequestPkcs7, IX509CertificateRequestCmc);
#[cfg(feature = "Win32_System_Com")]
impl IX509CertificateRequestCmc2 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitializeFromTemplate<P0, P1>(&self, context: X509CertificateEnrollmentContext, ppolicyserver: P0, ptemplate: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IX509EnrollmentPolicyServer>,
        P1: windows_core::Param<IX509CertificateTemplate>,
    {
        (windows_core::Interface::vtable(self).InitializeFromTemplate)(windows_core::Interface::as_raw(self), context, ppolicyserver.param().abi(), ptemplate.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitializeFromInnerRequestTemplate<P0, P1, P2>(&self, pinnerrequest: P0, ppolicyserver: P1, ptemplate: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IX509CertificateRequest>,
        P1: windows_core::Param<IX509EnrollmentPolicyServer>,
        P2: windows_core::Param<IX509CertificateTemplate>,
    {
        (windows_core::Interface::vtable(self).InitializeFromInnerRequestTemplate)(windows_core::Interface::as_raw(self), pinnerrequest.param().abi(), ppolicyserver.param().abi(), ptemplate.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PolicyServer(&self) -> windows_core::Result<IX509EnrollmentPolicyServer> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PolicyServer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Template(&self) -> windows_core::Result<IX509CertificateTemplate> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Template)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CheckSignature(&self, allowedsignaturetypes: Pkcs10AllowedSignatureTypes) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CheckSignature)(windows_core::Interface::as_raw(self), allowedsignaturetypes).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CheckCertificateSignature<P0, P1>(&self, psignercertificate: P0, validatecertificatechain: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISignerCertificate>,
        P1: windows_core::Param<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).CheckCertificateSignature)(windows_core::Interface::as_raw(self), psignercertificate.param().abi(), validatecertificatechain.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509CertificateRequestCmc2_Vtbl {
    pub base__: IX509CertificateRequestCmc_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub InitializeFromTemplate: unsafe extern "system" fn(*mut core::ffi::c_void, X509CertificateEnrollmentContext, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InitializeFromTemplate: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub InitializeFromInnerRequestTemplate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InitializeFromInnerRequestTemplate: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PolicyServer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PolicyServer: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Template: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Template: usize,
    pub CheckSignature: unsafe extern "system" fn(*mut core::ffi::c_void, Pkcs10AllowedSignatureTypes) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CheckCertificateSignature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CheckCertificateSignature: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IX509CertificateRequestPkcs10, IX509CertificateRequestPkcs10_Vtbl, 0x728ab342_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IX509CertificateRequestPkcs10 {
    type Target = IX509CertificateRequest;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IX509CertificateRequestPkcs10, windows_core::IUnknown, super::super::super::System::Com::IDispatch, IX509CertificateRequest);
#[cfg(feature = "Win32_System_Com")]
impl IX509CertificateRequestPkcs10 {
    pub unsafe fn InitializeFromTemplateName<P0>(&self, context: X509CertificateEnrollmentContext, strtemplatename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).InitializeFromTemplateName)(windows_core::Interface::as_raw(self), context, strtemplatename.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitializeFromPrivateKey<P0, P1>(&self, context: X509CertificateEnrollmentContext, pprivatekey: P0, strtemplatename: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IX509PrivateKey>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).InitializeFromPrivateKey)(windows_core::Interface::as_raw(self), context, pprivatekey.param().abi(), strtemplatename.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitializeFromPublicKey<P0, P1>(&self, context: X509CertificateEnrollmentContext, ppublickey: P0, strtemplatename: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IX509PublicKey>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).InitializeFromPublicKey)(windows_core::Interface::as_raw(self), context, ppublickey.param().abi(), strtemplatename.param().abi()).ok()
    }
    pub unsafe fn InitializeFromCertificate<P0>(&self, context: X509CertificateEnrollmentContext, strcertificate: P0, encoding: EncodingType, inheritoptions: X509RequestInheritOptions) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).InitializeFromCertificate)(windows_core::Interface::as_raw(self), context, strcertificate.param().abi(), encoding, inheritoptions).ok()
    }
    pub unsafe fn InitializeDecode<P0>(&self, strencodeddata: P0, encoding: EncodingType) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).InitializeDecode)(windows_core::Interface::as_raw(self), strencodeddata.param().abi(), encoding).ok()
    }
    pub unsafe fn CheckSignature(&self, allowedsignaturetypes: Pkcs10AllowedSignatureTypes) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CheckSignature)(windows_core::Interface::as_raw(self), allowedsignaturetypes).ok()
    }
    pub unsafe fn IsSmartCard(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsSmartCard)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn TemplateObjectId(&self) -> windows_core::Result<IObjectId> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TemplateObjectId)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PublicKey(&self) -> windows_core::Result<IX509PublicKey> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PublicKey)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PrivateKey(&self) -> windows_core::Result<IX509PrivateKey> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PrivateKey)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn NullSigned(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NullSigned)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ReuseKey(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ReuseKey)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn get_OldCertificate(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_OldCertificate)(windows_core::Interface::as_raw(self), encoding, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Subject(&self) -> windows_core::Result<IX500DistinguishedName> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Subject)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSubject<P0>(&self, pvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IX500DistinguishedName>,
    {
        (windows_core::Interface::vtable(self).SetSubject)(windows_core::Interface::as_raw(self), pvalue.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CspStatuses(&self) -> windows_core::Result<ICspStatuses> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CspStatuses)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SmimeCapabilities(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SmimeCapabilities)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetSmimeCapabilities<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetSmimeCapabilities)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SignatureInformation(&self) -> windows_core::Result<IX509SignatureInformation> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SignatureInformation)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn KeyContainerNamePrefix(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).KeyContainerNamePrefix)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetKeyContainerNamePrefix<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetKeyContainerNamePrefix)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CryptAttributes(&self) -> windows_core::Result<ICryptAttributes> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CryptAttributes)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn X509Extensions(&self) -> windows_core::Result<IX509Extensions> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).X509Extensions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CriticalExtensions(&self) -> windows_core::Result<IObjectIds> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CriticalExtensions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SuppressOids(&self) -> windows_core::Result<IObjectIds> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SuppressOids)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn get_RawDataToBeSigned(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_RawDataToBeSigned)(windows_core::Interface::as_raw(self), encoding, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn get_Signature(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Signature)(windows_core::Interface::as_raw(self), encoding, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetCspStatuses(&self, keyspec: X509KeySpec) -> windows_core::Result<ICspStatuses> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCspStatuses)(windows_core::Interface::as_raw(self), keyspec, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509CertificateRequestPkcs10_Vtbl {
    pub base__: IX509CertificateRequest_Vtbl,
    pub InitializeFromTemplateName: unsafe extern "system" fn(*mut core::ffi::c_void, X509CertificateEnrollmentContext, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub InitializeFromPrivateKey: unsafe extern "system" fn(*mut core::ffi::c_void, X509CertificateEnrollmentContext, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InitializeFromPrivateKey: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub InitializeFromPublicKey: unsafe extern "system" fn(*mut core::ffi::c_void, X509CertificateEnrollmentContext, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InitializeFromPublicKey: usize,
    pub InitializeFromCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, X509CertificateEnrollmentContext, core::mem::MaybeUninit<windows_core::BSTR>, EncodingType, X509RequestInheritOptions) -> windows_core::HRESULT,
    pub InitializeDecode: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, EncodingType) -> windows_core::HRESULT,
    pub CheckSignature: unsafe extern "system" fn(*mut core::ffi::c_void, Pkcs10AllowedSignatureTypes) -> windows_core::HRESULT,
    pub IsSmartCard: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub TemplateObjectId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    TemplateObjectId: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PublicKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PublicKey: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PrivateKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PrivateKey: usize,
    pub NullSigned: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ReuseKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub get_OldCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Subject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Subject: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSubject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSubject: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CspStatuses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CspStatuses: usize,
    pub SmimeCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetSmimeCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SignatureInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SignatureInformation: usize,
    pub KeyContainerNamePrefix: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetKeyContainerNamePrefix: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CryptAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CryptAttributes: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub X509Extensions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    X509Extensions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CriticalExtensions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CriticalExtensions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SuppressOids: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SuppressOids: usize,
    pub get_RawDataToBeSigned: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub get_Signature: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetCspStatuses: unsafe extern "system" fn(*mut core::ffi::c_void, X509KeySpec, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetCspStatuses: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IX509CertificateRequestPkcs10V2, IX509CertificateRequestPkcs10V2_Vtbl, 0x728ab35b_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IX509CertificateRequestPkcs10V2 {
    type Target = IX509CertificateRequestPkcs10;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IX509CertificateRequestPkcs10V2, windows_core::IUnknown, super::super::super::System::Com::IDispatch, IX509CertificateRequest, IX509CertificateRequestPkcs10);
#[cfg(feature = "Win32_System_Com")]
impl IX509CertificateRequestPkcs10V2 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitializeFromTemplate<P0, P1>(&self, context: X509CertificateEnrollmentContext, ppolicyserver: P0, ptemplate: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IX509EnrollmentPolicyServer>,
        P1: windows_core::Param<IX509CertificateTemplate>,
    {
        (windows_core::Interface::vtable(self).InitializeFromTemplate)(windows_core::Interface::as_raw(self), context, ppolicyserver.param().abi(), ptemplate.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitializeFromPrivateKeyTemplate<P0, P1, P2>(&self, context: X509CertificateEnrollmentContext, pprivatekey: P0, ppolicyserver: P1, ptemplate: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IX509PrivateKey>,
        P1: windows_core::Param<IX509EnrollmentPolicyServer>,
        P2: windows_core::Param<IX509CertificateTemplate>,
    {
        (windows_core::Interface::vtable(self).InitializeFromPrivateKeyTemplate)(windows_core::Interface::as_raw(self), context, pprivatekey.param().abi(), ppolicyserver.param().abi(), ptemplate.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitializeFromPublicKeyTemplate<P0, P1, P2>(&self, context: X509CertificateEnrollmentContext, ppublickey: P0, ppolicyserver: P1, ptemplate: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IX509PublicKey>,
        P1: windows_core::Param<IX509EnrollmentPolicyServer>,
        P2: windows_core::Param<IX509CertificateTemplate>,
    {
        (windows_core::Interface::vtable(self).InitializeFromPublicKeyTemplate)(windows_core::Interface::as_raw(self), context, ppublickey.param().abi(), ppolicyserver.param().abi(), ptemplate.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PolicyServer(&self) -> windows_core::Result<IX509EnrollmentPolicyServer> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PolicyServer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Template(&self) -> windows_core::Result<IX509CertificateTemplate> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Template)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509CertificateRequestPkcs10V2_Vtbl {
    pub base__: IX509CertificateRequestPkcs10_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub InitializeFromTemplate: unsafe extern "system" fn(*mut core::ffi::c_void, X509CertificateEnrollmentContext, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InitializeFromTemplate: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub InitializeFromPrivateKeyTemplate: unsafe extern "system" fn(*mut core::ffi::c_void, X509CertificateEnrollmentContext, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InitializeFromPrivateKeyTemplate: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub InitializeFromPublicKeyTemplate: unsafe extern "system" fn(*mut core::ffi::c_void, X509CertificateEnrollmentContext, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InitializeFromPublicKeyTemplate: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PolicyServer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PolicyServer: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Template: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Template: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IX509CertificateRequestPkcs10V3, IX509CertificateRequestPkcs10V3_Vtbl, 0x54ea9942_3d66_4530_b76e_7c9170d3ec52);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IX509CertificateRequestPkcs10V3 {
    type Target = IX509CertificateRequestPkcs10V2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IX509CertificateRequestPkcs10V3, windows_core::IUnknown, super::super::super::System::Com::IDispatch, IX509CertificateRequest, IX509CertificateRequestPkcs10, IX509CertificateRequestPkcs10V2);
#[cfg(feature = "Win32_System_Com")]
impl IX509CertificateRequestPkcs10V3 {
    pub unsafe fn AttestPrivateKey(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AttestPrivateKey)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAttestPrivateKey<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetAttestPrivateKey)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn get_AttestationEncryptionCertificate(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_AttestationEncryptionCertificate)(windows_core::Interface::as_raw(self), encoding, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn put_AttestationEncryptionCertificate<P0>(&self, encoding: EncodingType, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).put_AttestationEncryptionCertificate)(windows_core::Interface::as_raw(self), encoding, value.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EncryptionAlgorithm(&self) -> windows_core::Result<IObjectId> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EncryptionAlgorithm)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetEncryptionAlgorithm<P0>(&self, pvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IObjectId>,
    {
        (windows_core::Interface::vtable(self).SetEncryptionAlgorithm)(windows_core::Interface::as_raw(self), pvalue.param().abi()).ok()
    }
    pub unsafe fn EncryptionStrength(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EncryptionStrength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetEncryptionStrength(&self, value: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetEncryptionStrength)(windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn ChallengePassword(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ChallengePassword)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetChallengePassword<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetChallengePassword)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn NameValuePairs(&self) -> windows_core::Result<IX509NameValuePairs> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NameValuePairs)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509CertificateRequestPkcs10V3_Vtbl {
    pub base__: IX509CertificateRequestPkcs10V2_Vtbl,
    pub AttestPrivateKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetAttestPrivateKey: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub get_AttestationEncryptionCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub put_AttestationEncryptionCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub EncryptionAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EncryptionAlgorithm: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetEncryptionAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetEncryptionAlgorithm: usize,
    pub EncryptionStrength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetEncryptionStrength: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ChallengePassword: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetChallengePassword: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub NameValuePairs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    NameValuePairs: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IX509CertificateRequestPkcs10V4, IX509CertificateRequestPkcs10V4_Vtbl, 0x728ab363_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IX509CertificateRequestPkcs10V4 {
    type Target = IX509CertificateRequestPkcs10V3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IX509CertificateRequestPkcs10V4, windows_core::IUnknown, super::super::super::System::Com::IDispatch, IX509CertificateRequest, IX509CertificateRequestPkcs10, IX509CertificateRequestPkcs10V2, IX509CertificateRequestPkcs10V3);
#[cfg(feature = "Win32_System_Com")]
impl IX509CertificateRequestPkcs10V4 {
    pub unsafe fn ClaimType(&self) -> windows_core::Result<KeyAttestationClaimType> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ClaimType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetClaimType(&self, value: KeyAttestationClaimType) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetClaimType)(windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn AttestPrivateKeyPreferred(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AttestPrivateKeyPreferred)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAttestPrivateKeyPreferred<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetAttestPrivateKeyPreferred)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509CertificateRequestPkcs10V4_Vtbl {
    pub base__: IX509CertificateRequestPkcs10V3_Vtbl,
    pub ClaimType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut KeyAttestationClaimType) -> windows_core::HRESULT,
    pub SetClaimType: unsafe extern "system" fn(*mut core::ffi::c_void, KeyAttestationClaimType) -> windows_core::HRESULT,
    pub AttestPrivateKeyPreferred: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetAttestPrivateKeyPreferred: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IX509CertificateRequestPkcs7, IX509CertificateRequestPkcs7_Vtbl, 0x728ab344_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IX509CertificateRequestPkcs7 {
    type Target = IX509CertificateRequest;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IX509CertificateRequestPkcs7, windows_core::IUnknown, super::super::super::System::Com::IDispatch, IX509CertificateRequest);
#[cfg(feature = "Win32_System_Com")]
impl IX509CertificateRequestPkcs7 {
    pub unsafe fn InitializeFromTemplateName<P0>(&self, context: X509CertificateEnrollmentContext, strtemplatename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).InitializeFromTemplateName)(windows_core::Interface::as_raw(self), context, strtemplatename.param().abi()).ok()
    }
    pub unsafe fn InitializeFromCertificate<P0, P1>(&self, context: X509CertificateEnrollmentContext, renewalrequest: P0, strcertificate: P1, encoding: EncodingType, inheritoptions: X509RequestInheritOptions) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::VARIANT_BOOL>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).InitializeFromCertificate)(windows_core::Interface::as_raw(self), context, renewalrequest.param().abi(), strcertificate.param().abi(), encoding, inheritoptions).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitializeFromInnerRequest<P0>(&self, pinnerrequest: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IX509CertificateRequest>,
    {
        (windows_core::Interface::vtable(self).InitializeFromInnerRequest)(windows_core::Interface::as_raw(self), pinnerrequest.param().abi()).ok()
    }
    pub unsafe fn InitializeDecode<P0>(&self, strencodeddata: P0, encoding: EncodingType) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).InitializeDecode)(windows_core::Interface::as_raw(self), strencodeddata.param().abi(), encoding).ok()
    }
    pub unsafe fn RequesterName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RequesterName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetRequesterName<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetRequesterName)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SignerCertificate(&self) -> windows_core::Result<ISignerCertificate> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SignerCertificate)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSignerCertificate<P0>(&self, pvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISignerCertificate>,
    {
        (windows_core::Interface::vtable(self).SetSignerCertificate)(windows_core::Interface::as_raw(self), pvalue.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509CertificateRequestPkcs7_Vtbl {
    pub base__: IX509CertificateRequest_Vtbl,
    pub InitializeFromTemplateName: unsafe extern "system" fn(*mut core::ffi::c_void, X509CertificateEnrollmentContext, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub InitializeFromCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, X509CertificateEnrollmentContext, super::super::super::Foundation::VARIANT_BOOL, core::mem::MaybeUninit<windows_core::BSTR>, EncodingType, X509RequestInheritOptions) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub InitializeFromInnerRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InitializeFromInnerRequest: usize,
    pub InitializeDecode: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, EncodingType) -> windows_core::HRESULT,
    pub RequesterName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetRequesterName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SignerCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SignerCertificate: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSignerCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSignerCertificate: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IX509CertificateRequestPkcs7V2, IX509CertificateRequestPkcs7V2_Vtbl, 0x728ab35c_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IX509CertificateRequestPkcs7V2 {
    type Target = IX509CertificateRequestPkcs7;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IX509CertificateRequestPkcs7V2, windows_core::IUnknown, super::super::super::System::Com::IDispatch, IX509CertificateRequest, IX509CertificateRequestPkcs7);
#[cfg(feature = "Win32_System_Com")]
impl IX509CertificateRequestPkcs7V2 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitializeFromTemplate<P0, P1>(&self, context: X509CertificateEnrollmentContext, ppolicyserver: P0, ptemplate: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IX509EnrollmentPolicyServer>,
        P1: windows_core::Param<IX509CertificateTemplate>,
    {
        (windows_core::Interface::vtable(self).InitializeFromTemplate)(windows_core::Interface::as_raw(self), context, ppolicyserver.param().abi(), ptemplate.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PolicyServer(&self) -> windows_core::Result<IX509EnrollmentPolicyServer> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PolicyServer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Template(&self) -> windows_core::Result<IX509CertificateTemplate> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Template)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CheckCertificateSignature<P0>(&self, validatecertificatechain: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).CheckCertificateSignature)(windows_core::Interface::as_raw(self), validatecertificatechain.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509CertificateRequestPkcs7V2_Vtbl {
    pub base__: IX509CertificateRequestPkcs7_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub InitializeFromTemplate: unsafe extern "system" fn(*mut core::ffi::c_void, X509CertificateEnrollmentContext, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InitializeFromTemplate: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PolicyServer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PolicyServer: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Template: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Template: usize,
    pub CheckCertificateSignature: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IX509CertificateRevocationList, IX509CertificateRevocationList_Vtbl, 0x728ab360_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IX509CertificateRevocationList {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IX509CertificateRevocationList, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IX509CertificateRevocationList {
    pub unsafe fn Initialize(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn InitializeDecode<P0>(&self, strencodeddata: P0, encoding: EncodingType) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).InitializeDecode)(windows_core::Interface::as_raw(self), strencodeddata.param().abi(), encoding).ok()
    }
    pub unsafe fn Encode(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Encode)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ResetForEncode(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ResetForEncode)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CheckPublicKeySignature<P0>(&self, ppublickey: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IX509PublicKey>,
    {
        (windows_core::Interface::vtable(self).CheckPublicKeySignature)(windows_core::Interface::as_raw(self), ppublickey.param().abi()).ok()
    }
    pub unsafe fn CheckSignature(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CheckSignature)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Issuer(&self) -> windows_core::Result<IX500DistinguishedName> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Issuer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetIssuer<P0>(&self, pvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IX500DistinguishedName>,
    {
        (windows_core::Interface::vtable(self).SetIssuer)(windows_core::Interface::as_raw(self), pvalue.param().abi()).ok()
    }
    pub unsafe fn ThisUpdate(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ThisUpdate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetThisUpdate(&self, value: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetThisUpdate)(windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn NextUpdate(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NextUpdate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetNextUpdate(&self, value: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetNextUpdate)(windows_core::Interface::as_raw(self), value).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn X509CRLEntries(&self) -> windows_core::Result<IX509CertificateRevocationListEntries> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).X509CRLEntries)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn X509Extensions(&self) -> windows_core::Result<IX509Extensions> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).X509Extensions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CriticalExtensions(&self) -> windows_core::Result<IObjectIds> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CriticalExtensions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SignerCertificate(&self) -> windows_core::Result<ISignerCertificate> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SignerCertificate)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSignerCertificate<P0>(&self, pvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISignerCertificate>,
    {
        (windows_core::Interface::vtable(self).SetSignerCertificate)(windows_core::Interface::as_raw(self), pvalue.param().abi()).ok()
    }
    pub unsafe fn get_CRLNumber(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_CRLNumber)(windows_core::Interface::as_raw(self), encoding, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn put_CRLNumber<P0>(&self, encoding: EncodingType, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).put_CRLNumber)(windows_core::Interface::as_raw(self), encoding, value.param().abi()).ok()
    }
    pub unsafe fn CAVersion(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CAVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetCAVersion(&self, pvalue: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetCAVersion)(windows_core::Interface::as_raw(self), pvalue).ok()
    }
    pub unsafe fn BaseCRL(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BaseCRL)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn NullSigned(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NullSigned)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn HashAlgorithm(&self) -> windows_core::Result<IObjectId> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).HashAlgorithm)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetHashAlgorithm<P0>(&self, pvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IObjectId>,
    {
        (windows_core::Interface::vtable(self).SetHashAlgorithm)(windows_core::Interface::as_raw(self), pvalue.param().abi()).ok()
    }
    pub unsafe fn AlternateSignatureAlgorithm(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AlternateSignatureAlgorithm)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAlternateSignatureAlgorithm<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetAlternateSignatureAlgorithm)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SignatureInformation(&self) -> windows_core::Result<IX509SignatureInformation> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SignatureInformation)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn get_RawData(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_RawData)(windows_core::Interface::as_raw(self), encoding, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn get_RawDataToBeSigned(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_RawDataToBeSigned)(windows_core::Interface::as_raw(self), encoding, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn get_Signature(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Signature)(windows_core::Interface::as_raw(self), encoding, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509CertificateRevocationList_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InitializeDecode: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, EncodingType) -> windows_core::HRESULT,
    pub Encode: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ResetForEncode: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CheckPublicKeySignature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CheckPublicKeySignature: usize,
    pub CheckSignature: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Issuer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Issuer: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetIssuer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetIssuer: usize,
    pub ThisUpdate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetThisUpdate: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub NextUpdate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetNextUpdate: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub X509CRLEntries: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    X509CRLEntries: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub X509Extensions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    X509Extensions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CriticalExtensions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CriticalExtensions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SignerCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SignerCertificate: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSignerCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSignerCertificate: usize,
    pub get_CRLNumber: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub put_CRLNumber: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CAVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetCAVersion: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub BaseCRL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub NullSigned: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub HashAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    HashAlgorithm: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetHashAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetHashAlgorithm: usize,
    pub AlternateSignatureAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetAlternateSignatureAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SignatureInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SignatureInformation: usize,
    pub get_RawData: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub get_RawDataToBeSigned: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub get_Signature: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IX509CertificateRevocationListEntries, IX509CertificateRevocationListEntries_Vtbl, 0x728ab35f_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IX509CertificateRevocationListEntries {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IX509CertificateRevocationListEntries, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IX509CertificateRevocationListEntries {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_ItemByIndex(&self, index: i32) -> windows_core::Result<IX509CertificateRevocationListEntry> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_ItemByIndex)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Add<P0>(&self, pval: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IX509CertificateRevocationListEntry>,
    {
        (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), pval.param().abi()).ok()
    }
    pub unsafe fn Remove(&self, index: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), index).ok()
    }
    pub unsafe fn Clear(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn get_IndexBySerialNumber<P0>(&self, encoding: EncodingType, serialnumber: P0) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_IndexBySerialNumber)(windows_core::Interface::as_raw(self), encoding, serialnumber.param().abi(), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddRange<P0>(&self, pvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IX509CertificateRevocationListEntries>,
    {
        (windows_core::Interface::vtable(self).AddRange)(windows_core::Interface::as_raw(self), pvalue.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509CertificateRevocationListEntries_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub get_ItemByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_ItemByIndex: usize,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_IndexBySerialNumber: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, core::mem::MaybeUninit<windows_core::BSTR>, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AddRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddRange: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IX509CertificateRevocationListEntry, IX509CertificateRevocationListEntry_Vtbl, 0x728ab35e_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IX509CertificateRevocationListEntry {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IX509CertificateRevocationListEntry, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IX509CertificateRevocationListEntry {
    pub unsafe fn Initialize<P0>(&self, encoding: EncodingType, serialnumber: P0, revocationdate: f64) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), encoding, serialnumber.param().abi(), revocationdate).ok()
    }
    pub unsafe fn get_SerialNumber(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_SerialNumber)(windows_core::Interface::as_raw(self), encoding, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RevocationDate(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RevocationDate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn RevocationReason(&self) -> windows_core::Result<CRLRevocationReason> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RevocationReason)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetRevocationReason(&self, value: CRLRevocationReason) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetRevocationReason)(windows_core::Interface::as_raw(self), value).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn X509Extensions(&self) -> windows_core::Result<IX509Extensions> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).X509Extensions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CriticalExtensions(&self) -> windows_core::Result<IObjectIds> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CriticalExtensions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509CertificateRevocationListEntry_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, core::mem::MaybeUninit<windows_core::BSTR>, f64) -> windows_core::HRESULT,
    pub get_SerialNumber: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub RevocationDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub RevocationReason: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CRLRevocationReason) -> windows_core::HRESULT,
    pub SetRevocationReason: unsafe extern "system" fn(*mut core::ffi::c_void, CRLRevocationReason) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub X509Extensions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    X509Extensions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CriticalExtensions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CriticalExtensions: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IX509CertificateTemplate, IX509CertificateTemplate_Vtbl, 0x54244a13_555a_4e22_896d_1b0e52f76406);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IX509CertificateTemplate {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IX509CertificateTemplate, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IX509CertificateTemplate {
    pub unsafe fn get_Property(&self, property: EnrollmentTemplateProperty) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Property)(windows_core::Interface::as_raw(self), property, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509CertificateTemplate_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub get_Property: unsafe extern "system" fn(*mut core::ffi::c_void, EnrollmentTemplateProperty, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IX509CertificateTemplateWritable, IX509CertificateTemplateWritable_Vtbl, 0xf49466a7_395a_4e9e_b6e7_32b331600dc0);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IX509CertificateTemplateWritable {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IX509CertificateTemplateWritable, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IX509CertificateTemplateWritable {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Initialize<P0>(&self, pvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IX509CertificateTemplate>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pvalue.param().abi()).ok()
    }
    pub unsafe fn Commit<P0>(&self, commitflags: CommitTemplateFlags, strservercontext: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self), commitflags, strservercontext.param().abi()).ok()
    }
    pub unsafe fn get_Property(&self, property: EnrollmentTemplateProperty) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Property)(windows_core::Interface::as_raw(self), property, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn put_Property<P0>(&self, property: EnrollmentTemplateProperty, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).put_Property)(windows_core::Interface::as_raw(self), property, value.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Template(&self) -> windows_core::Result<IX509CertificateTemplate> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Template)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509CertificateTemplateWritable_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Initialize: usize,
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void, CommitTemplateFlags, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub get_Property: unsafe extern "system" fn(*mut core::ffi::c_void, EnrollmentTemplateProperty, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub put_Property: unsafe extern "system" fn(*mut core::ffi::c_void, EnrollmentTemplateProperty, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Template: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Template: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IX509CertificateTemplates, IX509CertificateTemplates_Vtbl, 0x13b79003_2181_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IX509CertificateTemplates {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IX509CertificateTemplates, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IX509CertificateTemplates {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_ItemByIndex(&self, index: i32) -> windows_core::Result<IX509CertificateTemplate> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_ItemByIndex)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Add<P0>(&self, pval: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IX509CertificateTemplate>,
    {
        (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), pval.param().abi()).ok()
    }
    pub unsafe fn Remove(&self, index: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), index).ok()
    }
    pub unsafe fn Clear(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_ItemByName<P0>(&self, bstrname: P0) -> windows_core::Result<IX509CertificateTemplate>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_ItemByName)(windows_core::Interface::as_raw(self), bstrname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_ItemByOid<P0>(&self, poid: P0) -> windows_core::Result<IX509CertificateTemplate>
    where
        P0: windows_core::Param<IObjectId>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_ItemByOid)(windows_core::Interface::as_raw(self), poid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509CertificateTemplates_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub get_ItemByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_ItemByIndex: usize,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_ItemByName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_ItemByName: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub get_ItemByOid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_ItemByOid: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IX509EndorsementKey, IX509EndorsementKey_Vtbl, 0xb11cd855_f4c4_4fc6_b710_4422237f09e9);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IX509EndorsementKey {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IX509EndorsementKey, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IX509EndorsementKey {
    pub unsafe fn ProviderName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ProviderName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetProviderName<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetProviderName)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn Length(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Length)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Opened(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Opened)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn AddCertificate<P0>(&self, encoding: EncodingType, strcertificate: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).AddCertificate)(windows_core::Interface::as_raw(self), encoding, strcertificate.param().abi()).ok()
    }
    pub unsafe fn RemoveCertificate<P0>(&self, encoding: EncodingType, strcertificate: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).RemoveCertificate)(windows_core::Interface::as_raw(self), encoding, strcertificate.param().abi()).ok()
    }
    pub unsafe fn GetCertificateByIndex<P0>(&self, manufactureronly: P0, dwindex: i32, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<super::super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCertificateByIndex)(windows_core::Interface::as_raw(self), manufactureronly.param().abi(), dwindex, encoding, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCertificateCount<P0>(&self, manufactureronly: P0) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<super::super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCertificateCount)(windows_core::Interface::as_raw(self), manufactureronly.param().abi(), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ExportPublicKey(&self) -> windows_core::Result<IX509PublicKey> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ExportPublicKey)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Open(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509EndorsementKey_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub ProviderName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetProviderName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Length: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Opened: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub AddCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub RemoveCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetCertificateByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::VARIANT_BOOL, i32, EncodingType, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetCertificateCount: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::VARIANT_BOOL, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ExportPublicKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ExportPublicKey: usize,
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IX509Enrollment, IX509Enrollment_Vtbl, 0x728ab346_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IX509Enrollment {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IX509Enrollment, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IX509Enrollment {
    pub unsafe fn Initialize(&self, context: X509CertificateEnrollmentContext) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), context).ok()
    }
    pub unsafe fn InitializeFromTemplateName<P0>(&self, context: X509CertificateEnrollmentContext, strtemplatename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).InitializeFromTemplateName)(windows_core::Interface::as_raw(self), context, strtemplatename.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitializeFromRequest<P0>(&self, prequest: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IX509CertificateRequest>,
    {
        (windows_core::Interface::vtable(self).InitializeFromRequest)(windows_core::Interface::as_raw(self), prequest.param().abi()).ok()
    }
    pub unsafe fn CreateRequest(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateRequest)(windows_core::Interface::as_raw(self), encoding, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Enroll(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Enroll)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn InstallResponse<P0, P1>(&self, restrictions: InstallResponseRestrictionFlags, strresponse: P0, encoding: EncodingType, strpassword: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).InstallResponse)(windows_core::Interface::as_raw(self), restrictions, strresponse.param().abi(), encoding, strpassword.param().abi()).ok()
    }
    pub unsafe fn CreatePFX<P0>(&self, strpassword: P0, exportoptions: PFXExportOptions, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreatePFX)(windows_core::Interface::as_raw(self), strpassword.param().abi(), exportoptions, encoding, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Request(&self) -> windows_core::Result<IX509CertificateRequest> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Request)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Silent(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Silent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetSilent<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetSilent)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn ParentWindow(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ParentWindow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetParentWindow(&self, value: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetParentWindow)(windows_core::Interface::as_raw(self), value).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn NameValuePairs(&self) -> windows_core::Result<IX509NameValuePairs> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NameValuePairs)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnrollmentContext(&self) -> windows_core::Result<X509CertificateEnrollmentContext> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnrollmentContext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Status(&self) -> windows_core::Result<IX509EnrollmentStatus> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Status)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn get_Certificate(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Certificate)(windows_core::Interface::as_raw(self), encoding, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn get_Response(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Response)(windows_core::Interface::as_raw(self), encoding, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CertificateFriendlyName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CertificateFriendlyName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetCertificateFriendlyName<P0>(&self, strvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetCertificateFriendlyName)(windows_core::Interface::as_raw(self), strvalue.param().abi()).ok()
    }
    pub unsafe fn CertificateDescription(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CertificateDescription)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetCertificateDescription<P0>(&self, strvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetCertificateDescription)(windows_core::Interface::as_raw(self), strvalue.param().abi()).ok()
    }
    pub unsafe fn RequestId(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RequestId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CAConfigString(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CAConfigString)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509Enrollment_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, X509CertificateEnrollmentContext) -> windows_core::HRESULT,
    pub InitializeFromTemplateName: unsafe extern "system" fn(*mut core::ffi::c_void, X509CertificateEnrollmentContext, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub InitializeFromRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InitializeFromRequest: usize,
    pub CreateRequest: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Enroll: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InstallResponse: unsafe extern "system" fn(*mut core::ffi::c_void, InstallResponseRestrictionFlags, core::mem::MaybeUninit<windows_core::BSTR>, EncodingType, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CreatePFX: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, PFXExportOptions, EncodingType, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Request: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Request: usize,
    pub Silent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetSilent: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ParentWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetParentWindow: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub NameValuePairs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    NameValuePairs: usize,
    pub EnrollmentContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut X509CertificateEnrollmentContext) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Status: usize,
    pub get_Certificate: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub get_Response: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CertificateFriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetCertificateFriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CertificateDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetCertificateDescription: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub RequestId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CAConfigString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IX509Enrollment2, IX509Enrollment2_Vtbl, 0x728ab350_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IX509Enrollment2 {
    type Target = IX509Enrollment;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IX509Enrollment2, windows_core::IUnknown, super::super::super::System::Com::IDispatch, IX509Enrollment);
#[cfg(feature = "Win32_System_Com")]
impl IX509Enrollment2 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitializeFromTemplate<P0, P1>(&self, context: X509CertificateEnrollmentContext, ppolicyserver: P0, ptemplate: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IX509EnrollmentPolicyServer>,
        P1: windows_core::Param<IX509CertificateTemplate>,
    {
        (windows_core::Interface::vtable(self).InitializeFromTemplate)(windows_core::Interface::as_raw(self), context, ppolicyserver.param().abi(), ptemplate.param().abi()).ok()
    }
    pub unsafe fn InstallResponse2<P0, P1, P2, P3>(&self, restrictions: InstallResponseRestrictionFlags, strresponse: P0, encoding: EncodingType, strpassword: P1, strenrollmentpolicyserverurl: P2, strenrollmentpolicyserverid: P3, enrollmentpolicyserverflags: PolicyServerUrlFlags, authflags: X509EnrollmentAuthFlags) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
        P3: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).InstallResponse2)(windows_core::Interface::as_raw(self), restrictions, strresponse.param().abi(), encoding, strpassword.param().abi(), strenrollmentpolicyserverurl.param().abi(), strenrollmentpolicyserverid.param().abi(), enrollmentpolicyserverflags, authflags).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PolicyServer(&self) -> windows_core::Result<IX509EnrollmentPolicyServer> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PolicyServer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Template(&self) -> windows_core::Result<IX509CertificateTemplate> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Template)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RequestIdString(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RequestIdString)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509Enrollment2_Vtbl {
    pub base__: IX509Enrollment_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub InitializeFromTemplate: unsafe extern "system" fn(*mut core::ffi::c_void, X509CertificateEnrollmentContext, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InitializeFromTemplate: usize,
    pub InstallResponse2: unsafe extern "system" fn(*mut core::ffi::c_void, InstallResponseRestrictionFlags, core::mem::MaybeUninit<windows_core::BSTR>, EncodingType, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, PolicyServerUrlFlags, X509EnrollmentAuthFlags) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub PolicyServer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PolicyServer: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Template: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Template: usize,
    pub RequestIdString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IX509EnrollmentHelper, IX509EnrollmentHelper_Vtbl, 0x728ab351_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IX509EnrollmentHelper {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IX509EnrollmentHelper, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IX509EnrollmentHelper {
    pub unsafe fn AddPolicyServer<P0, P1, P2, P3>(&self, strenrollmentpolicyserveruri: P0, strenrollmentpolicyid: P1, enrollmentpolicyserverflags: PolicyServerUrlFlags, authflags: X509EnrollmentAuthFlags, strcredential: P2, strpassword: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
        P3: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).AddPolicyServer)(windows_core::Interface::as_raw(self), strenrollmentpolicyserveruri.param().abi(), strenrollmentpolicyid.param().abi(), enrollmentpolicyserverflags, authflags, strcredential.param().abi(), strpassword.param().abi()).ok()
    }
    pub unsafe fn AddEnrollmentServer<P0, P1, P2>(&self, strenrollmentserveruri: P0, authflags: X509EnrollmentAuthFlags, strcredential: P1, strpassword: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).AddEnrollmentServer)(windows_core::Interface::as_raw(self), strenrollmentserveruri.param().abi(), authflags, strcredential.param().abi(), strpassword.param().abi()).ok()
    }
    pub unsafe fn Enroll<P0, P1>(&self, strenrollmentpolicyserveruri: P0, strtemplatename: P1, encoding: EncodingType, enrollflags: WebEnrollmentFlags) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Enroll)(windows_core::Interface::as_raw(self), strenrollmentpolicyserveruri.param().abi(), strtemplatename.param().abi(), encoding, enrollflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Initialize(&self, context: X509CertificateEnrollmentContext) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), context).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509EnrollmentHelper_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub AddPolicyServer: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, PolicyServerUrlFlags, X509EnrollmentAuthFlags, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub AddEnrollmentServer: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, X509EnrollmentAuthFlags, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Enroll: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, EncodingType, WebEnrollmentFlags, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, X509CertificateEnrollmentContext) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IX509EnrollmentPolicyServer, IX509EnrollmentPolicyServer_Vtbl, 0x13b79026_2181_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IX509EnrollmentPolicyServer {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IX509EnrollmentPolicyServer, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IX509EnrollmentPolicyServer {
    pub unsafe fn Initialize<P0, P1, P2>(&self, bstrpolicyserverurl: P0, bstrpolicyserverid: P1, authflags: X509EnrollmentAuthFlags, fisuntrusted: P2, context: X509CertificateEnrollmentContext) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), bstrpolicyserverurl.param().abi(), bstrpolicyserverid.param().abi(), authflags, fisuntrusted.param().abi(), context).ok()
    }
    pub unsafe fn LoadPolicy(&self, option: X509EnrollmentPolicyLoadOption) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).LoadPolicy)(windows_core::Interface::as_raw(self), option).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTemplates(&self) -> windows_core::Result<IX509CertificateTemplates> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTemplates)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetCAsForTemplate<P0>(&self, ptemplate: P0) -> windows_core::Result<ICertificationAuthorities>
    where
        P0: windows_core::Param<IX509CertificateTemplate>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCAsForTemplate)(windows_core::Interface::as_raw(self), ptemplate.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetCAs(&self) -> windows_core::Result<ICertificationAuthorities> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCAs)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Validate(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Validate)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetCustomOids(&self) -> windows_core::Result<IObjectIds> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCustomOids)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetNextUpdateTime(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNextUpdateTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetLastUpdateTime(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLastUpdateTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetPolicyServerUrl(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPolicyServerUrl)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetPolicyServerId(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPolicyServerId)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetFriendlyName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFriendlyName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetIsDefaultCEP(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetIsDefaultCEP)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetUseClientId(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetUseClientId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetAllowUnTrustedCA(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAllowUnTrustedCA)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetCachePath(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCachePath)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCacheDir(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCacheDir)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetAuthFlags(&self) -> windows_core::Result<X509EnrollmentAuthFlags> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAuthFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetCredential<P0, P1>(&self, hwndparent: i32, flag: X509EnrollmentAuthFlags, strcredential: P0, strpassword: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetCredential)(windows_core::Interface::as_raw(self), hwndparent, flag, strcredential.param().abi(), strpassword.param().abi()).ok()
    }
    pub unsafe fn QueryChanges(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryChanges)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn InitializeImport<P0>(&self, val: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).InitializeImport)(windows_core::Interface::as_raw(self), val.param().abi()).ok()
    }
    pub unsafe fn Export(&self, exportflags: X509EnrollmentPolicyExportFlags) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Export)(windows_core::Interface::as_raw(self), exportflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Cost(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Cost)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetCost(&self, value: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetCost)(windows_core::Interface::as_raw(self), value).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509EnrollmentPolicyServer_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, X509EnrollmentAuthFlags, super::super::super::Foundation::VARIANT_BOOL, X509CertificateEnrollmentContext) -> windows_core::HRESULT,
    pub LoadPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, X509EnrollmentPolicyLoadOption) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetTemplates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetTemplates: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetCAsForTemplate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetCAsForTemplate: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetCAs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetCAs: usize,
    pub Validate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetCustomOids: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetCustomOids: usize,
    pub GetNextUpdateTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub GetLastUpdateTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub GetPolicyServerUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetPolicyServerId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetFriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetIsDefaultCEP: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub GetUseClientId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub GetAllowUnTrustedCA: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub GetCachePath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetCacheDir: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetAuthFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut X509EnrollmentAuthFlags) -> windows_core::HRESULT,
    pub SetCredential: unsafe extern "system" fn(*mut core::ffi::c_void, i32, X509EnrollmentAuthFlags, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub QueryChanges: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub InitializeImport: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Export: unsafe extern "system" fn(*mut core::ffi::c_void, X509EnrollmentPolicyExportFlags, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Cost: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetCost: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IX509EnrollmentStatus, IX509EnrollmentStatus_Vtbl, 0x728ab304_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IX509EnrollmentStatus {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IX509EnrollmentStatus, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IX509EnrollmentStatus {
    pub unsafe fn AppendText<P0>(&self, strtext: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).AppendText)(windows_core::Interface::as_raw(self), strtext.param().abi()).ok()
    }
    pub unsafe fn Text(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Text)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetText<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetText)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn Selected(&self) -> windows_core::Result<EnrollmentSelectionStatus> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Selected)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetSelected(&self, value: EnrollmentSelectionStatus) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSelected)(windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn Display(&self) -> windows_core::Result<EnrollmentDisplayStatus> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Display)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetDisplay(&self, value: EnrollmentDisplayStatus) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDisplay)(windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn Status(&self) -> windows_core::Result<EnrollmentEnrollStatus> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Status)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetStatus(&self, value: EnrollmentEnrollStatus) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetStatus)(windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn Error(&self) -> windows_core::Result<windows_core::HRESULT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Error)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetError(&self, value: windows_core::HRESULT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetError)(windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn ErrorText(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ErrorText)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509EnrollmentStatus_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub AppendText: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Text: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetText: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Selected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut EnrollmentSelectionStatus) -> windows_core::HRESULT,
    pub SetSelected: unsafe extern "system" fn(*mut core::ffi::c_void, EnrollmentSelectionStatus) -> windows_core::HRESULT,
    pub Display: unsafe extern "system" fn(*mut core::ffi::c_void, *mut EnrollmentDisplayStatus) -> windows_core::HRESULT,
    pub SetDisplay: unsafe extern "system" fn(*mut core::ffi::c_void, EnrollmentDisplayStatus) -> windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut EnrollmentEnrollStatus) -> windows_core::HRESULT,
    pub SetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, EnrollmentEnrollStatus) -> windows_core::HRESULT,
    pub Error: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    pub SetError: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
    pub ErrorText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IX509EnrollmentWebClassFactory, IX509EnrollmentWebClassFactory_Vtbl, 0x728ab349_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IX509EnrollmentWebClassFactory {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IX509EnrollmentWebClassFactory, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IX509EnrollmentWebClassFactory {
    pub unsafe fn CreateObject<P0>(&self, strprogid: P0) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateObject)(windows_core::Interface::as_raw(self), strprogid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509EnrollmentWebClassFactory_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub CreateObject: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IX509Extension, IX509Extension_Vtbl, 0x728ab30d_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IX509Extension {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IX509Extension, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IX509Extension {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Initialize<P0, P1>(&self, pobjectid: P0, encoding: EncodingType, strencodeddata: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IObjectId>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pobjectid.param().abi(), encoding, strencodeddata.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ObjectId(&self) -> windows_core::Result<IObjectId> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ObjectId)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn get_RawData(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_RawData)(windows_core::Interface::as_raw(self), encoding, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Critical(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Critical)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetCritical<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetCritical)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509Extension_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, EncodingType, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Initialize: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ObjectId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ObjectId: usize,
    pub get_RawData: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Critical: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetCritical: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IX509ExtensionAlternativeNames, IX509ExtensionAlternativeNames_Vtbl, 0x728ab315_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IX509ExtensionAlternativeNames {
    type Target = IX509Extension;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IX509ExtensionAlternativeNames, windows_core::IUnknown, super::super::super::System::Com::IDispatch, IX509Extension);
#[cfg(feature = "Win32_System_Com")]
impl IX509ExtensionAlternativeNames {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitializeEncode<P0>(&self, pvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAlternativeNames>,
    {
        (windows_core::Interface::vtable(self).InitializeEncode)(windows_core::Interface::as_raw(self), pvalue.param().abi()).ok()
    }
    pub unsafe fn InitializeDecode<P0>(&self, encoding: EncodingType, strencodeddata: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).InitializeDecode)(windows_core::Interface::as_raw(self), encoding, strencodeddata.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AlternativeNames(&self) -> windows_core::Result<IAlternativeNames> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AlternativeNames)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509ExtensionAlternativeNames_Vtbl {
    pub base__: IX509Extension_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub InitializeEncode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InitializeEncode: usize,
    pub InitializeDecode: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AlternativeNames: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AlternativeNames: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IX509ExtensionAuthorityKeyIdentifier, IX509ExtensionAuthorityKeyIdentifier_Vtbl, 0x728ab318_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IX509ExtensionAuthorityKeyIdentifier {
    type Target = IX509Extension;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IX509ExtensionAuthorityKeyIdentifier, windows_core::IUnknown, super::super::super::System::Com::IDispatch, IX509Extension);
#[cfg(feature = "Win32_System_Com")]
impl IX509ExtensionAuthorityKeyIdentifier {
    pub unsafe fn InitializeEncode<P0>(&self, encoding: EncodingType, strkeyidentifier: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).InitializeEncode)(windows_core::Interface::as_raw(self), encoding, strkeyidentifier.param().abi()).ok()
    }
    pub unsafe fn InitializeDecode<P0>(&self, encoding: EncodingType, strencodeddata: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).InitializeDecode)(windows_core::Interface::as_raw(self), encoding, strencodeddata.param().abi()).ok()
    }
    pub unsafe fn get_AuthorityKeyIdentifier(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_AuthorityKeyIdentifier)(windows_core::Interface::as_raw(self), encoding, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509ExtensionAuthorityKeyIdentifier_Vtbl {
    pub base__: IX509Extension_Vtbl,
    pub InitializeEncode: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub InitializeDecode: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub get_AuthorityKeyIdentifier: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IX509ExtensionBasicConstraints, IX509ExtensionBasicConstraints_Vtbl, 0x728ab316_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IX509ExtensionBasicConstraints {
    type Target = IX509Extension;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IX509ExtensionBasicConstraints, windows_core::IUnknown, super::super::super::System::Com::IDispatch, IX509Extension);
#[cfg(feature = "Win32_System_Com")]
impl IX509ExtensionBasicConstraints {
    pub unsafe fn InitializeEncode<P0>(&self, isca: P0, pathlenconstraint: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).InitializeEncode)(windows_core::Interface::as_raw(self), isca.param().abi(), pathlenconstraint).ok()
    }
    pub unsafe fn InitializeDecode<P0>(&self, encoding: EncodingType, strencodeddata: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).InitializeDecode)(windows_core::Interface::as_raw(self), encoding, strencodeddata.param().abi()).ok()
    }
    pub unsafe fn IsCA(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsCA)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn PathLenConstraint(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PathLenConstraint)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509ExtensionBasicConstraints_Vtbl {
    pub base__: IX509Extension_Vtbl,
    pub InitializeEncode: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::VARIANT_BOOL, i32) -> windows_core::HRESULT,
    pub InitializeDecode: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub IsCA: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub PathLenConstraint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IX509ExtensionCertificatePolicies, IX509ExtensionCertificatePolicies_Vtbl, 0x728ab320_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IX509ExtensionCertificatePolicies {
    type Target = IX509Extension;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IX509ExtensionCertificatePolicies, windows_core::IUnknown, super::super::super::System::Com::IDispatch, IX509Extension);
#[cfg(feature = "Win32_System_Com")]
impl IX509ExtensionCertificatePolicies {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitializeEncode<P0>(&self, pvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ICertificatePolicies>,
    {
        (windows_core::Interface::vtable(self).InitializeEncode)(windows_core::Interface::as_raw(self), pvalue.param().abi()).ok()
    }
    pub unsafe fn InitializeDecode<P0>(&self, encoding: EncodingType, strencodeddata: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).InitializeDecode)(windows_core::Interface::as_raw(self), encoding, strencodeddata.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Policies(&self) -> windows_core::Result<ICertificatePolicies> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Policies)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509ExtensionCertificatePolicies_Vtbl {
    pub base__: IX509Extension_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub InitializeEncode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InitializeEncode: usize,
    pub InitializeDecode: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Policies: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Policies: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IX509ExtensionEnhancedKeyUsage, IX509ExtensionEnhancedKeyUsage_Vtbl, 0x728ab310_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IX509ExtensionEnhancedKeyUsage {
    type Target = IX509Extension;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IX509ExtensionEnhancedKeyUsage, windows_core::IUnknown, super::super::super::System::Com::IDispatch, IX509Extension);
#[cfg(feature = "Win32_System_Com")]
impl IX509ExtensionEnhancedKeyUsage {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitializeEncode<P0>(&self, pvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IObjectIds>,
    {
        (windows_core::Interface::vtable(self).InitializeEncode)(windows_core::Interface::as_raw(self), pvalue.param().abi()).ok()
    }
    pub unsafe fn InitializeDecode<P0>(&self, encoding: EncodingType, strencodeddata: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).InitializeDecode)(windows_core::Interface::as_raw(self), encoding, strencodeddata.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnhancedKeyUsage(&self) -> windows_core::Result<IObjectIds> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnhancedKeyUsage)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509ExtensionEnhancedKeyUsage_Vtbl {
    pub base__: IX509Extension_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub InitializeEncode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InitializeEncode: usize,
    pub InitializeDecode: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub EnhancedKeyUsage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnhancedKeyUsage: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IX509ExtensionKeyUsage, IX509ExtensionKeyUsage_Vtbl, 0x728ab30f_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IX509ExtensionKeyUsage {
    type Target = IX509Extension;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IX509ExtensionKeyUsage, windows_core::IUnknown, super::super::super::System::Com::IDispatch, IX509Extension);
#[cfg(feature = "Win32_System_Com")]
impl IX509ExtensionKeyUsage {
    pub unsafe fn InitializeEncode(&self, usageflags: X509KeyUsageFlags) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).InitializeEncode)(windows_core::Interface::as_raw(self), usageflags).ok()
    }
    pub unsafe fn InitializeDecode<P0>(&self, encoding: EncodingType, strencodeddata: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).InitializeDecode)(windows_core::Interface::as_raw(self), encoding, strencodeddata.param().abi()).ok()
    }
    pub unsafe fn KeyUsage(&self) -> windows_core::Result<X509KeyUsageFlags> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).KeyUsage)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509ExtensionKeyUsage_Vtbl {
    pub base__: IX509Extension_Vtbl,
    pub InitializeEncode: unsafe extern "system" fn(*mut core::ffi::c_void, X509KeyUsageFlags) -> windows_core::HRESULT,
    pub InitializeDecode: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub KeyUsage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut X509KeyUsageFlags) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IX509ExtensionMSApplicationPolicies, IX509ExtensionMSApplicationPolicies_Vtbl, 0x728ab321_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IX509ExtensionMSApplicationPolicies {
    type Target = IX509Extension;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IX509ExtensionMSApplicationPolicies, windows_core::IUnknown, super::super::super::System::Com::IDispatch, IX509Extension);
#[cfg(feature = "Win32_System_Com")]
impl IX509ExtensionMSApplicationPolicies {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitializeEncode<P0>(&self, pvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ICertificatePolicies>,
    {
        (windows_core::Interface::vtable(self).InitializeEncode)(windows_core::Interface::as_raw(self), pvalue.param().abi()).ok()
    }
    pub unsafe fn InitializeDecode<P0>(&self, encoding: EncodingType, strencodeddata: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).InitializeDecode)(windows_core::Interface::as_raw(self), encoding, strencodeddata.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Policies(&self) -> windows_core::Result<ICertificatePolicies> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Policies)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509ExtensionMSApplicationPolicies_Vtbl {
    pub base__: IX509Extension_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub InitializeEncode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InitializeEncode: usize,
    pub InitializeDecode: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Policies: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Policies: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IX509ExtensionSmimeCapabilities, IX509ExtensionSmimeCapabilities_Vtbl, 0x728ab31b_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IX509ExtensionSmimeCapabilities {
    type Target = IX509Extension;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IX509ExtensionSmimeCapabilities, windows_core::IUnknown, super::super::super::System::Com::IDispatch, IX509Extension);
#[cfg(feature = "Win32_System_Com")]
impl IX509ExtensionSmimeCapabilities {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitializeEncode<P0>(&self, pvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISmimeCapabilities>,
    {
        (windows_core::Interface::vtable(self).InitializeEncode)(windows_core::Interface::as_raw(self), pvalue.param().abi()).ok()
    }
    pub unsafe fn InitializeDecode<P0>(&self, encoding: EncodingType, strencodeddata: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).InitializeDecode)(windows_core::Interface::as_raw(self), encoding, strencodeddata.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SmimeCapabilities(&self) -> windows_core::Result<ISmimeCapabilities> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SmimeCapabilities)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509ExtensionSmimeCapabilities_Vtbl {
    pub base__: IX509Extension_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub InitializeEncode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InitializeEncode: usize,
    pub InitializeDecode: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SmimeCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SmimeCapabilities: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IX509ExtensionSubjectKeyIdentifier, IX509ExtensionSubjectKeyIdentifier_Vtbl, 0x728ab317_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IX509ExtensionSubjectKeyIdentifier {
    type Target = IX509Extension;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IX509ExtensionSubjectKeyIdentifier, windows_core::IUnknown, super::super::super::System::Com::IDispatch, IX509Extension);
#[cfg(feature = "Win32_System_Com")]
impl IX509ExtensionSubjectKeyIdentifier {
    pub unsafe fn InitializeEncode<P0>(&self, encoding: EncodingType, strkeyidentifier: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).InitializeEncode)(windows_core::Interface::as_raw(self), encoding, strkeyidentifier.param().abi()).ok()
    }
    pub unsafe fn InitializeDecode<P0>(&self, encoding: EncodingType, strencodeddata: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).InitializeDecode)(windows_core::Interface::as_raw(self), encoding, strencodeddata.param().abi()).ok()
    }
    pub unsafe fn get_SubjectKeyIdentifier(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_SubjectKeyIdentifier)(windows_core::Interface::as_raw(self), encoding, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509ExtensionSubjectKeyIdentifier_Vtbl {
    pub base__: IX509Extension_Vtbl,
    pub InitializeEncode: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub InitializeDecode: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub get_SubjectKeyIdentifier: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IX509ExtensionTemplate, IX509ExtensionTemplate_Vtbl, 0x728ab312_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IX509ExtensionTemplate {
    type Target = IX509Extension;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IX509ExtensionTemplate, windows_core::IUnknown, super::super::super::System::Com::IDispatch, IX509Extension);
#[cfg(feature = "Win32_System_Com")]
impl IX509ExtensionTemplate {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitializeEncode<P0>(&self, ptemplateoid: P0, majorversion: i32, minorversion: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IObjectId>,
    {
        (windows_core::Interface::vtable(self).InitializeEncode)(windows_core::Interface::as_raw(self), ptemplateoid.param().abi(), majorversion, minorversion).ok()
    }
    pub unsafe fn InitializeDecode<P0>(&self, encoding: EncodingType, strencodeddata: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).InitializeDecode)(windows_core::Interface::as_raw(self), encoding, strencodeddata.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn TemplateOid(&self) -> windows_core::Result<IObjectId> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TemplateOid)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn MajorVersion(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MajorVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn MinorVersion(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MinorVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509ExtensionTemplate_Vtbl {
    pub base__: IX509Extension_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub InitializeEncode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InitializeEncode: usize,
    pub InitializeDecode: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub TemplateOid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    TemplateOid: usize,
    pub MajorVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub MinorVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IX509ExtensionTemplateName, IX509ExtensionTemplateName_Vtbl, 0x728ab311_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IX509ExtensionTemplateName {
    type Target = IX509Extension;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IX509ExtensionTemplateName, windows_core::IUnknown, super::super::super::System::Com::IDispatch, IX509Extension);
#[cfg(feature = "Win32_System_Com")]
impl IX509ExtensionTemplateName {
    pub unsafe fn InitializeEncode<P0>(&self, strtemplatename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).InitializeEncode)(windows_core::Interface::as_raw(self), strtemplatename.param().abi()).ok()
    }
    pub unsafe fn InitializeDecode<P0>(&self, encoding: EncodingType, strencodeddata: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).InitializeDecode)(windows_core::Interface::as_raw(self), encoding, strencodeddata.param().abi()).ok()
    }
    pub unsafe fn TemplateName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TemplateName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509ExtensionTemplateName_Vtbl {
    pub base__: IX509Extension_Vtbl,
    pub InitializeEncode: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub InitializeDecode: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub TemplateName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IX509Extensions, IX509Extensions_Vtbl, 0x728ab30e_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IX509Extensions {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IX509Extensions, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IX509Extensions {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_ItemByIndex(&self, index: i32) -> windows_core::Result<IX509Extension> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_ItemByIndex)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Add<P0>(&self, pval: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IX509Extension>,
    {
        (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), pval.param().abi()).ok()
    }
    pub unsafe fn Remove(&self, index: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), index).ok()
    }
    pub unsafe fn Clear(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_IndexByObjectId<P0>(&self, pobjectid: P0) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<IObjectId>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_IndexByObjectId)(windows_core::Interface::as_raw(self), pobjectid.param().abi(), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddRange<P0>(&self, pvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IX509Extensions>,
    {
        (windows_core::Interface::vtable(self).AddRange)(windows_core::Interface::as_raw(self), pvalue.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509Extensions_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub get_ItemByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_ItemByIndex: usize,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_IndexByObjectId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_IndexByObjectId: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AddRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddRange: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IX509MachineEnrollmentFactory, IX509MachineEnrollmentFactory_Vtbl, 0x728ab352_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IX509MachineEnrollmentFactory {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IX509MachineEnrollmentFactory, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IX509MachineEnrollmentFactory {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateObject<P0>(&self, strprogid: P0) -> windows_core::Result<IX509EnrollmentHelper>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateObject)(windows_core::Interface::as_raw(self), strprogid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509MachineEnrollmentFactory_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateObject: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateObject: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IX509NameValuePair, IX509NameValuePair_Vtbl, 0x728ab33f_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IX509NameValuePair {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IX509NameValuePair, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IX509NameValuePair {
    pub unsafe fn Initialize<P0, P1>(&self, strname: P0, strvalue: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), strname.param().abi(), strvalue.param().abi()).ok()
    }
    pub unsafe fn Value(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Value)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509NameValuePair_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IX509NameValuePairs, IX509NameValuePairs_Vtbl, 0x728ab340_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IX509NameValuePairs {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IX509NameValuePairs, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IX509NameValuePairs {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_ItemByIndex(&self, index: i32) -> windows_core::Result<IX509NameValuePair> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_ItemByIndex)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Add<P0>(&self, pval: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IX509NameValuePair>,
    {
        (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), pval.param().abi()).ok()
    }
    pub unsafe fn Remove(&self, index: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), index).ok()
    }
    pub unsafe fn Clear(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509NameValuePairs_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub get_ItemByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_ItemByIndex: usize,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IX509PolicyServerListManager, IX509PolicyServerListManager_Vtbl, 0x884e204b_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IX509PolicyServerListManager {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IX509PolicyServerListManager, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IX509PolicyServerListManager {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_ItemByIndex(&self, index: i32) -> windows_core::Result<IX509PolicyServerUrl> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_ItemByIndex)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Add<P0>(&self, pval: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IX509PolicyServerUrl>,
    {
        (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), pval.param().abi()).ok()
    }
    pub unsafe fn Remove(&self, index: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), index).ok()
    }
    pub unsafe fn Clear(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Initialize(&self, context: X509CertificateEnrollmentContext, flags: PolicyServerUrlFlags) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), context, flags).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509PolicyServerListManager_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub get_ItemByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_ItemByIndex: usize,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, X509CertificateEnrollmentContext, PolicyServerUrlFlags) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IX509PolicyServerUrl, IX509PolicyServerUrl_Vtbl, 0x884e204a_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IX509PolicyServerUrl {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IX509PolicyServerUrl, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IX509PolicyServerUrl {
    pub unsafe fn Initialize(&self, context: X509CertificateEnrollmentContext) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), context).ok()
    }
    pub unsafe fn Url(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Url)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetUrl<P0>(&self, pvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetUrl)(windows_core::Interface::as_raw(self), pvalue.param().abi()).ok()
    }
    pub unsafe fn Default(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Default)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetDefault<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetDefault)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn Flags(&self) -> windows_core::Result<PolicyServerUrlFlags> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Flags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetFlags(&self, flags: PolicyServerUrlFlags) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFlags)(windows_core::Interface::as_raw(self), flags).ok()
    }
    pub unsafe fn AuthFlags(&self) -> windows_core::Result<X509EnrollmentAuthFlags> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AuthFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAuthFlags(&self, flags: X509EnrollmentAuthFlags) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAuthFlags)(windows_core::Interface::as_raw(self), flags).ok()
    }
    pub unsafe fn Cost(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Cost)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetCost(&self, value: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetCost)(windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetStringProperty(&self, propertyid: PolicyServerUrlPropertyID) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStringProperty)(windows_core::Interface::as_raw(self), propertyid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetStringProperty<P0>(&self, propertyid: PolicyServerUrlPropertyID, pvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetStringProperty)(windows_core::Interface::as_raw(self), propertyid, pvalue.param().abi()).ok()
    }
    pub unsafe fn UpdateRegistry(&self, context: X509CertificateEnrollmentContext) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UpdateRegistry)(windows_core::Interface::as_raw(self), context).ok()
    }
    pub unsafe fn RemoveFromRegistry(&self, context: X509CertificateEnrollmentContext) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveFromRegistry)(windows_core::Interface::as_raw(self), context).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509PolicyServerUrl_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, X509CertificateEnrollmentContext) -> windows_core::HRESULT,
    pub Url: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetUrl: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Default: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Flags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PolicyServerUrlFlags) -> windows_core::HRESULT,
    pub SetFlags: unsafe extern "system" fn(*mut core::ffi::c_void, PolicyServerUrlFlags) -> windows_core::HRESULT,
    pub AuthFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut X509EnrollmentAuthFlags) -> windows_core::HRESULT,
    pub SetAuthFlags: unsafe extern "system" fn(*mut core::ffi::c_void, X509EnrollmentAuthFlags) -> windows_core::HRESULT,
    pub Cost: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetCost: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetStringProperty: unsafe extern "system" fn(*mut core::ffi::c_void, PolicyServerUrlPropertyID, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetStringProperty: unsafe extern "system" fn(*mut core::ffi::c_void, PolicyServerUrlPropertyID, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub UpdateRegistry: unsafe extern "system" fn(*mut core::ffi::c_void, X509CertificateEnrollmentContext) -> windows_core::HRESULT,
    pub RemoveFromRegistry: unsafe extern "system" fn(*mut core::ffi::c_void, X509CertificateEnrollmentContext) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IX509PrivateKey, IX509PrivateKey_Vtbl, 0x728ab30c_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IX509PrivateKey {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IX509PrivateKey, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IX509PrivateKey {
    pub unsafe fn Open(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Create(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Delete(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Verify(&self, verifytype: X509PrivateKeyVerify) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Verify)(windows_core::Interface::as_raw(self), verifytype).ok()
    }
    pub unsafe fn Import<P0, P1>(&self, strexporttype: P0, strencodedkey: P1, encoding: EncodingType) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Import)(windows_core::Interface::as_raw(self), strexporttype.param().abi(), strencodedkey.param().abi(), encoding).ok()
    }
    pub unsafe fn Export<P0>(&self, strexporttype: P0, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Export)(windows_core::Interface::as_raw(self), strexporttype.param().abi(), encoding, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ExportPublicKey(&self) -> windows_core::Result<IX509PublicKey> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ExportPublicKey)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ContainerName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ContainerName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetContainerName<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetContainerName)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn ContainerNamePrefix(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ContainerNamePrefix)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetContainerNamePrefix<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetContainerNamePrefix)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn ReaderName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ReaderName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetReaderName<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetReaderName)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CspInformations(&self) -> windows_core::Result<ICspInformations> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CspInformations)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetCspInformations<P0>(&self, pvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ICspInformations>,
    {
        (windows_core::Interface::vtable(self).SetCspInformations)(windows_core::Interface::as_raw(self), pvalue.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CspStatus(&self) -> windows_core::Result<ICspStatus> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CspStatus)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetCspStatus<P0>(&self, pvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ICspStatus>,
    {
        (windows_core::Interface::vtable(self).SetCspStatus)(windows_core::Interface::as_raw(self), pvalue.param().abi()).ok()
    }
    pub unsafe fn ProviderName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ProviderName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetProviderName<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetProviderName)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn ProviderType(&self) -> windows_core::Result<X509ProviderType> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ProviderType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetProviderType(&self, value: X509ProviderType) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetProviderType)(windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn LegacyCsp(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LegacyCsp)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetLegacyCsp<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetLegacyCsp)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Algorithm(&self) -> windows_core::Result<IObjectId> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Algorithm)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetAlgorithm<P0>(&self, pvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IObjectId>,
    {
        (windows_core::Interface::vtable(self).SetAlgorithm)(windows_core::Interface::as_raw(self), pvalue.param().abi()).ok()
    }
    pub unsafe fn KeySpec(&self) -> windows_core::Result<X509KeySpec> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).KeySpec)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetKeySpec(&self, value: X509KeySpec) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetKeySpec)(windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn Length(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Length)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetLength(&self, value: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetLength)(windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn ExportPolicy(&self) -> windows_core::Result<X509PrivateKeyExportFlags> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ExportPolicy)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetExportPolicy(&self, value: X509PrivateKeyExportFlags) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetExportPolicy)(windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn KeyUsage(&self) -> windows_core::Result<X509PrivateKeyUsageFlags> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).KeyUsage)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetKeyUsage(&self, value: X509PrivateKeyUsageFlags) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetKeyUsage)(windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn KeyProtection(&self) -> windows_core::Result<X509PrivateKeyProtection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).KeyProtection)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetKeyProtection(&self, value: X509PrivateKeyProtection) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetKeyProtection)(windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn MachineContext(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MachineContext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMachineContext<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetMachineContext)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn SecurityDescriptor(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SecurityDescriptor)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetSecurityDescriptor<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetSecurityDescriptor)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn get_Certificate(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Certificate)(windows_core::Interface::as_raw(self), encoding, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn put_Certificate<P0>(&self, encoding: EncodingType, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).put_Certificate)(windows_core::Interface::as_raw(self), encoding, value.param().abi()).ok()
    }
    pub unsafe fn UniqueContainerName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UniqueContainerName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Opened(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Opened)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn DefaultContainer(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DefaultContainer)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Existing(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Existing)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetExisting<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetExisting)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn Silent(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Silent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetSilent<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetSilent)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn ParentWindow(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ParentWindow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetParentWindow(&self, value: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetParentWindow)(windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn UIContextMessage(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UIContextMessage)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetUIContextMessage<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetUIContextMessage)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn SetPin<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetPin)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn FriendlyName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FriendlyName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetFriendlyName<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetFriendlyName)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDescription<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509PrivateKey_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Verify: unsafe extern "system" fn(*mut core::ffi::c_void, X509PrivateKeyVerify) -> windows_core::HRESULT,
    pub Import: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, EncodingType) -> windows_core::HRESULT,
    pub Export: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, EncodingType, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ExportPublicKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ExportPublicKey: usize,
    pub ContainerName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetContainerName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ContainerNamePrefix: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetContainerNamePrefix: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ReaderName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetReaderName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CspInformations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CspInformations: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetCspInformations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetCspInformations: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CspStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CspStatus: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetCspStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetCspStatus: usize,
    pub ProviderName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetProviderName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ProviderType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut X509ProviderType) -> windows_core::HRESULT,
    pub SetProviderType: unsafe extern "system" fn(*mut core::ffi::c_void, X509ProviderType) -> windows_core::HRESULT,
    pub LegacyCsp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetLegacyCsp: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Algorithm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Algorithm: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetAlgorithm: usize,
    pub KeySpec: unsafe extern "system" fn(*mut core::ffi::c_void, *mut X509KeySpec) -> windows_core::HRESULT,
    pub SetKeySpec: unsafe extern "system" fn(*mut core::ffi::c_void, X509KeySpec) -> windows_core::HRESULT,
    pub Length: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetLength: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ExportPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut X509PrivateKeyExportFlags) -> windows_core::HRESULT,
    pub SetExportPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, X509PrivateKeyExportFlags) -> windows_core::HRESULT,
    pub KeyUsage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut X509PrivateKeyUsageFlags) -> windows_core::HRESULT,
    pub SetKeyUsage: unsafe extern "system" fn(*mut core::ffi::c_void, X509PrivateKeyUsageFlags) -> windows_core::HRESULT,
    pub KeyProtection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut X509PrivateKeyProtection) -> windows_core::HRESULT,
    pub SetKeyProtection: unsafe extern "system" fn(*mut core::ffi::c_void, X509PrivateKeyProtection) -> windows_core::HRESULT,
    pub MachineContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetMachineContext: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SecurityDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetSecurityDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub get_Certificate: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub put_Certificate: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub UniqueContainerName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Opened: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub DefaultContainer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Existing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetExisting: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Silent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetSilent: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ParentWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetParentWindow: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub UIContextMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetUIContextMessage: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetPin: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub FriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetFriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IX509PrivateKey2, IX509PrivateKey2_Vtbl, 0x728ab362_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IX509PrivateKey2 {
    type Target = IX509PrivateKey;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IX509PrivateKey2, windows_core::IUnknown, super::super::super::System::Com::IDispatch, IX509PrivateKey);
#[cfg(feature = "Win32_System_Com")]
impl IX509PrivateKey2 {
    pub unsafe fn HardwareKeyUsage(&self) -> windows_core::Result<X509HardwareKeyUsageFlags> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).HardwareKeyUsage)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetHardwareKeyUsage(&self, value: X509HardwareKeyUsageFlags) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetHardwareKeyUsage)(windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn AlternateStorageLocation(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AlternateStorageLocation)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetAlternateStorageLocation<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetAlternateStorageLocation)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn AlgorithmName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AlgorithmName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetAlgorithmName<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetAlgorithmName)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn get_AlgorithmParameters(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_AlgorithmParameters)(windows_core::Interface::as_raw(self), encoding, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn put_AlgorithmParameters<P0>(&self, encoding: EncodingType, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).put_AlgorithmParameters)(windows_core::Interface::as_raw(self), encoding, value.param().abi()).ok()
    }
    pub unsafe fn ParametersExportType(&self) -> windows_core::Result<X509KeyParametersExportType> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ParametersExportType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetParametersExportType(&self, value: X509KeyParametersExportType) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetParametersExportType)(windows_core::Interface::as_raw(self), value).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509PrivateKey2_Vtbl {
    pub base__: IX509PrivateKey_Vtbl,
    pub HardwareKeyUsage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut X509HardwareKeyUsageFlags) -> windows_core::HRESULT,
    pub SetHardwareKeyUsage: unsafe extern "system" fn(*mut core::ffi::c_void, X509HardwareKeyUsageFlags) -> windows_core::HRESULT,
    pub AlternateStorageLocation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetAlternateStorageLocation: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub AlgorithmName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetAlgorithmName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub get_AlgorithmParameters: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub put_AlgorithmParameters: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ParametersExportType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut X509KeyParametersExportType) -> windows_core::HRESULT,
    pub SetParametersExportType: unsafe extern "system" fn(*mut core::ffi::c_void, X509KeyParametersExportType) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IX509PublicKey, IX509PublicKey_Vtbl, 0x728ab30b_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IX509PublicKey {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IX509PublicKey, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IX509PublicKey {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Initialize<P0, P1, P2>(&self, pobjectid: P0, strencodedkey: P1, strencodedparameters: P2, encoding: EncodingType) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IObjectId>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pobjectid.param().abi(), strencodedkey.param().abi(), strencodedparameters.param().abi(), encoding).ok()
    }
    pub unsafe fn InitializeFromEncodedPublicKeyInfo<P0>(&self, strencodedpublickeyinfo: P0, encoding: EncodingType) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).InitializeFromEncodedPublicKeyInfo)(windows_core::Interface::as_raw(self), strencodedpublickeyinfo.param().abi(), encoding).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Algorithm(&self) -> windows_core::Result<IObjectId> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Algorithm)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Length(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Length)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn get_EncodedKey(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_EncodedKey)(windows_core::Interface::as_raw(self), encoding, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn get_EncodedParameters(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_EncodedParameters)(windows_core::Interface::as_raw(self), encoding, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ComputeKeyIdentifier(&self, algorithm: KeyIdentifierHashAlgorithm, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ComputeKeyIdentifier)(windows_core::Interface::as_raw(self), algorithm, encoding, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509PublicKey_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, EncodingType) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Initialize: usize,
    pub InitializeFromEncodedPublicKeyInfo: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, EncodingType) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Algorithm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Algorithm: usize,
    pub Length: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_EncodedKey: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub get_EncodedParameters: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ComputeKeyIdentifier: unsafe extern "system" fn(*mut core::ffi::c_void, KeyIdentifierHashAlgorithm, EncodingType, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IX509SCEPEnrollment, IX509SCEPEnrollment_Vtbl, 0x728ab361_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IX509SCEPEnrollment {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IX509SCEPEnrollment, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IX509SCEPEnrollment {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Initialize<P0, P1, P2>(&self, prequest: P0, strthumbprint: P1, thumprintencoding: EncodingType, strservercertificates: P2, encoding: EncodingType) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IX509CertificateRequestPkcs10>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), prequest.param().abi(), strthumbprint.param().abi(), thumprintencoding, strservercertificates.param().abi(), encoding).ok()
    }
    pub unsafe fn InitializeForPending(&self, context: X509CertificateEnrollmentContext) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).InitializeForPending)(windows_core::Interface::as_raw(self), context).ok()
    }
    pub unsafe fn CreateRequestMessage(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateRequestMessage)(windows_core::Interface::as_raw(self), encoding, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateRetrievePendingMessage(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateRetrievePendingMessage)(windows_core::Interface::as_raw(self), encoding, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateRetrieveCertificateMessage<P0, P1>(&self, context: X509CertificateEnrollmentContext, strissuer: P0, issuerencoding: EncodingType, strserialnumber: P1, serialnumberencoding: EncodingType, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateRetrieveCertificateMessage)(windows_core::Interface::as_raw(self), context, strissuer.param().abi(), issuerencoding, strserialnumber.param().abi(), serialnumberencoding, encoding, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ProcessResponseMessage<P0>(&self, strresponse: P0, encoding: EncodingType) -> windows_core::Result<X509SCEPDisposition>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ProcessResponseMessage)(windows_core::Interface::as_raw(self), strresponse.param().abi(), encoding, &mut result__).map(|| result__)
    }
    pub unsafe fn SetServerCapabilities<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetServerCapabilities)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn FailInfo(&self) -> windows_core::Result<X509SCEPFailInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FailInfo)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SignerCertificate(&self) -> windows_core::Result<ISignerCertificate> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SignerCertificate)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSignerCertificate<P0>(&self, pvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISignerCertificate>,
    {
        (windows_core::Interface::vtable(self).SetSignerCertificate)(windows_core::Interface::as_raw(self), pvalue.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OldCertificate(&self) -> windows_core::Result<ISignerCertificate> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OldCertificate)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetOldCertificate<P0>(&self, pvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISignerCertificate>,
    {
        (windows_core::Interface::vtable(self).SetOldCertificate)(windows_core::Interface::as_raw(self), pvalue.param().abi()).ok()
    }
    pub unsafe fn get_TransactionId(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_TransactionId)(windows_core::Interface::as_raw(self), encoding, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn put_TransactionId<P0>(&self, encoding: EncodingType, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).put_TransactionId)(windows_core::Interface::as_raw(self), encoding, value.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Request(&self) -> windows_core::Result<IX509CertificateRequestPkcs10> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Request)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CertificateFriendlyName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CertificateFriendlyName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetCertificateFriendlyName<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetCertificateFriendlyName)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Status(&self) -> windows_core::Result<IX509EnrollmentStatus> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Status)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn get_Certificate(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Certificate)(windows_core::Interface::as_raw(self), encoding, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Silent(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Silent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetSilent<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetSilent)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn DeleteRequest(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DeleteRequest)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509SCEPEnrollment_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, EncodingType, core::mem::MaybeUninit<windows_core::BSTR>, EncodingType) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Initialize: usize,
    pub InitializeForPending: unsafe extern "system" fn(*mut core::ffi::c_void, X509CertificateEnrollmentContext) -> windows_core::HRESULT,
    pub CreateRequestMessage: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CreateRetrievePendingMessage: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CreateRetrieveCertificateMessage: unsafe extern "system" fn(*mut core::ffi::c_void, X509CertificateEnrollmentContext, core::mem::MaybeUninit<windows_core::BSTR>, EncodingType, core::mem::MaybeUninit<windows_core::BSTR>, EncodingType, EncodingType, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ProcessResponseMessage: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, EncodingType, *mut X509SCEPDisposition) -> windows_core::HRESULT,
    pub SetServerCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub FailInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut X509SCEPFailInfo) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SignerCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SignerCertificate: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSignerCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSignerCertificate: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OldCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OldCertificate: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetOldCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetOldCertificate: usize,
    pub get_TransactionId: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub put_TransactionId: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Request: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Request: usize,
    pub CertificateFriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetCertificateFriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Status: usize,
    pub get_Certificate: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Silent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetSilent: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub DeleteRequest: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IX509SCEPEnrollment2, IX509SCEPEnrollment2_Vtbl, 0x728ab364_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IX509SCEPEnrollment2 {
    type Target = IX509SCEPEnrollment;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IX509SCEPEnrollment2, windows_core::IUnknown, super::super::super::System::Com::IDispatch, IX509SCEPEnrollment);
#[cfg(feature = "Win32_System_Com")]
impl IX509SCEPEnrollment2 {
    pub unsafe fn CreateChallengeAnswerMessage(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateChallengeAnswerMessage)(windows_core::Interface::as_raw(self), encoding, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ProcessResponseMessage2<P0>(&self, flags: X509SCEPProcessMessageFlags, strresponse: P0, encoding: EncodingType) -> windows_core::Result<X509SCEPDisposition>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ProcessResponseMessage2)(windows_core::Interface::as_raw(self), flags, strresponse.param().abi(), encoding, &mut result__).map(|| result__)
    }
    pub unsafe fn ResultMessageText(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ResultMessageText)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DelayRetry(&self) -> windows_core::Result<DelayRetryAction> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DelayRetry)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ActivityId(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ActivityId)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetActivityId<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetActivityId)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509SCEPEnrollment2_Vtbl {
    pub base__: IX509SCEPEnrollment_Vtbl,
    pub CreateChallengeAnswerMessage: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ProcessResponseMessage2: unsafe extern "system" fn(*mut core::ffi::c_void, X509SCEPProcessMessageFlags, core::mem::MaybeUninit<windows_core::BSTR>, EncodingType, *mut X509SCEPDisposition) -> windows_core::HRESULT,
    pub ResultMessageText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub DelayRetry: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DelayRetryAction) -> windows_core::HRESULT,
    pub ActivityId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetActivityId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IX509SCEPEnrollmentHelper, IX509SCEPEnrollmentHelper_Vtbl, 0x728ab365_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IX509SCEPEnrollmentHelper {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IX509SCEPEnrollmentHelper, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IX509SCEPEnrollmentHelper {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Initialize<P0, P1, P2, P3>(&self, strserverurl: P0, strrequestheaders: P1, prequest: P2, strcacertificatethumbprint: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<IX509CertificateRequestPkcs10>,
        P3: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), strserverurl.param().abi(), strrequestheaders.param().abi(), prequest.param().abi(), strcacertificatethumbprint.param().abi()).ok()
    }
    pub unsafe fn InitializeForPending<P0, P1, P2>(&self, strserverurl: P0, strrequestheaders: P1, context: X509CertificateEnrollmentContext, strtransactionid: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).InitializeForPending)(windows_core::Interface::as_raw(self), strserverurl.param().abi(), strrequestheaders.param().abi(), context, strtransactionid.param().abi()).ok()
    }
    pub unsafe fn Enroll(&self, processflags: X509SCEPProcessMessageFlags) -> windows_core::Result<X509SCEPDisposition> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Enroll)(windows_core::Interface::as_raw(self), processflags, &mut result__).map(|| result__)
    }
    pub unsafe fn FetchPending(&self, processflags: X509SCEPProcessMessageFlags) -> windows_core::Result<X509SCEPDisposition> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FetchPending)(windows_core::Interface::as_raw(self), processflags, &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn X509SCEPEnrollment(&self) -> windows_core::Result<IX509SCEPEnrollment> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).X509SCEPEnrollment)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ResultMessageText(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ResultMessageText)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509SCEPEnrollmentHelper_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Initialize: usize,
    pub InitializeForPending: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, X509CertificateEnrollmentContext, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Enroll: unsafe extern "system" fn(*mut core::ffi::c_void, X509SCEPProcessMessageFlags, *mut X509SCEPDisposition) -> windows_core::HRESULT,
    pub FetchPending: unsafe extern "system" fn(*mut core::ffi::c_void, X509SCEPProcessMessageFlags, *mut X509SCEPDisposition) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub X509SCEPEnrollment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    X509SCEPEnrollment: usize,
    pub ResultMessageText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IX509SignatureInformation, IX509SignatureInformation_Vtbl, 0x728ab33c_217d_11da_b2a4_000e7bbb2b09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IX509SignatureInformation {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IX509SignatureInformation, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IX509SignatureInformation {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn HashAlgorithm(&self) -> windows_core::Result<IObjectId> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).HashAlgorithm)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetHashAlgorithm<P0>(&self, pvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IObjectId>,
    {
        (windows_core::Interface::vtable(self).SetHashAlgorithm)(windows_core::Interface::as_raw(self), pvalue.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PublicKeyAlgorithm(&self) -> windows_core::Result<IObjectId> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PublicKeyAlgorithm)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetPublicKeyAlgorithm<P0>(&self, pvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IObjectId>,
    {
        (windows_core::Interface::vtable(self).SetPublicKeyAlgorithm)(windows_core::Interface::as_raw(self), pvalue.param().abi()).ok()
    }
    pub unsafe fn get_Parameters(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Parameters)(windows_core::Interface::as_raw(self), encoding, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn put_Parameters<P0>(&self, encoding: EncodingType, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).put_Parameters)(windows_core::Interface::as_raw(self), encoding, value.param().abi()).ok()
    }
    pub unsafe fn AlternateSignatureAlgorithm(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AlternateSignatureAlgorithm)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAlternateSignatureAlgorithm<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetAlternateSignatureAlgorithm)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn AlternateSignatureAlgorithmSet(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AlternateSignatureAlgorithmSet)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn NullSigned(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NullSigned)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetNullSigned<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetNullSigned)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSignatureAlgorithm<P0, P1>(&self, pkcs7signature: P0, signaturekey: P1) -> windows_core::Result<IObjectId>
    where
        P0: windows_core::Param<super::super::super::Foundation::VARIANT_BOOL>,
        P1: windows_core::Param<super::super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSignatureAlgorithm)(windows_core::Interface::as_raw(self), pkcs7signature.param().abi(), signaturekey.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDefaultValues(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDefaultValues)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509SignatureInformation_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub HashAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    HashAlgorithm: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetHashAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetHashAlgorithm: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PublicKeyAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PublicKeyAlgorithm: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetPublicKeyAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetPublicKeyAlgorithm: usize,
    pub get_Parameters: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub put_Parameters: unsafe extern "system" fn(*mut core::ffi::c_void, EncodingType, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub AlternateSignatureAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetAlternateSignatureAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub AlternateSignatureAlgorithmSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub NullSigned: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetNullSigned: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSignatureAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::VARIANT_BOOL, super::super::super::Foundation::VARIANT_BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSignatureAlgorithm: usize,
    pub SetDefaultValues: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub const AlgorithmFlagsNone: AlgorithmFlags = AlgorithmFlags(0i32);
pub const AlgorithmFlagsWrap: AlgorithmFlags = AlgorithmFlags(1i32);
pub const AllowNoOutstandingRequest: InstallResponseRestrictionFlags = InstallResponseRestrictionFlags(1i32);
pub const AllowNone: InstallResponseRestrictionFlags = InstallResponseRestrictionFlags(0i32);
pub const AllowUntrustedCertificate: InstallResponseRestrictionFlags = InstallResponseRestrictionFlags(2i32);
pub const AllowUntrustedRoot: InstallResponseRestrictionFlags = InstallResponseRestrictionFlags(4i32);
pub const AllowedKeySignature: Pkcs10AllowedSignatureTypes = Pkcs10AllowedSignatureTypes(1i32);
pub const AllowedNullSignature: Pkcs10AllowedSignatureTypes = Pkcs10AllowedSignatureTypes(2i32);
pub const CAIF_DSENTRY: u32 = 1u32;
pub const CAIF_LOCAL: u32 = 8u32;
pub const CAIF_REGISTRY: u32 = 4u32;
pub const CAIF_REGISTRYPARENT: u32 = 16u32;
pub const CAIF_SHAREDFOLDERENTRY: u32 = 2u32;
pub const CAPATHLENGTH_INFINITE: u32 = 4294967295u32;
pub const CAPropCertificate: EnrollmentCAProperty = EnrollmentCAProperty(7i32);
pub const CAPropCertificateTypes: EnrollmentCAProperty = EnrollmentCAProperty(6i32);
pub const CAPropCommonName: EnrollmentCAProperty = EnrollmentCAProperty(1i32);
pub const CAPropDNSName: EnrollmentCAProperty = EnrollmentCAProperty(5i32);
pub const CAPropDescription: EnrollmentCAProperty = EnrollmentCAProperty(8i32);
pub const CAPropDistinguishedName: EnrollmentCAProperty = EnrollmentCAProperty(2i32);
pub const CAPropRenewalOnly: EnrollmentCAProperty = EnrollmentCAProperty(12i32);
pub const CAPropSanitizedName: EnrollmentCAProperty = EnrollmentCAProperty(3i32);
pub const CAPropSanitizedShortName: EnrollmentCAProperty = EnrollmentCAProperty(4i32);
pub const CAPropSecurity: EnrollmentCAProperty = EnrollmentCAProperty(11i32);
pub const CAPropSiteName: EnrollmentCAProperty = EnrollmentCAProperty(10i32);
pub const CAPropWebServers: EnrollmentCAProperty = EnrollmentCAProperty(9i32);
pub const CA_ACCESS_ADMIN: CERTADMIN_GET_ROLES_FLAGS = CERTADMIN_GET_ROLES_FLAGS(1u32);
pub const CA_ACCESS_AUDITOR: CERTADMIN_GET_ROLES_FLAGS = CERTADMIN_GET_ROLES_FLAGS(4u32);
pub const CA_ACCESS_ENROLL: CERTADMIN_GET_ROLES_FLAGS = CERTADMIN_GET_ROLES_FLAGS(512u32);
pub const CA_ACCESS_MASKROLES: u32 = 255u32;
pub const CA_ACCESS_OFFICER: CERTADMIN_GET_ROLES_FLAGS = CERTADMIN_GET_ROLES_FLAGS(2u32);
pub const CA_ACCESS_OPERATOR: CERTADMIN_GET_ROLES_FLAGS = CERTADMIN_GET_ROLES_FLAGS(8u32);
pub const CA_ACCESS_READ: CERTADMIN_GET_ROLES_FLAGS = CERTADMIN_GET_ROLES_FLAGS(256u32);
pub const CA_CRL_BASE: u32 = 1u32;
pub const CA_CRL_DELTA: u32 = 2u32;
pub const CA_CRL_REPUBLISH: u32 = 16u32;
pub const CA_DISP_ERROR: u32 = 1u32;
pub const CA_DISP_INCOMPLETE: u32 = 0u32;
pub const CA_DISP_INVALID: u32 = 4u32;
pub const CA_DISP_REVOKED: u32 = 2u32;
pub const CA_DISP_UNDER_SUBMISSION: u32 = 5u32;
pub const CA_DISP_VALID: u32 = 3u32;
pub const CCLOCKSKEWMINUTESDEFAULT: u32 = 10u32;
pub const CC_DEFAULTCONFIG: CERT_GET_CONFIG_FLAGS = CERT_GET_CONFIG_FLAGS(0i32);
pub const CC_FIRSTCONFIG: CERT_GET_CONFIG_FLAGS = CERT_GET_CONFIG_FLAGS(2i32);
pub const CC_LOCALACTIVECONFIG: CERT_GET_CONFIG_FLAGS = CERT_GET_CONFIG_FLAGS(4i32);
pub const CC_LOCALCONFIG: CERT_GET_CONFIG_FLAGS = CERT_GET_CONFIG_FLAGS(3i32);
pub const CC_UIPICKCONFIG: CERT_GET_CONFIG_FLAGS = CERT_GET_CONFIG_FLAGS(1i32);
pub const CC_UIPICKCONFIGSKIPLOCALCA: CERT_GET_CONFIG_FLAGS = CERT_GET_CONFIG_FLAGS(5i32);
pub const CDR_EXPIRED: CERT_DELETE_ROW_FLAGS = CERT_DELETE_ROW_FLAGS(1i32);
pub const CDR_REQUEST_LAST_CHANGED: CERT_DELETE_ROW_FLAGS = CERT_DELETE_ROW_FLAGS(2i32);
pub const CERTENROLL_INDEX_BASE: u32 = 0u32;
pub const CERT_ALT_NAME_DIRECTORY_NAME: CERT_ALT_NAME = CERT_ALT_NAME(5i32);
pub const CERT_ALT_NAME_DNS_NAME: CERT_ALT_NAME = CERT_ALT_NAME(3i32);
pub const CERT_ALT_NAME_IP_ADDRESS: CERT_ALT_NAME = CERT_ALT_NAME(8i32);
pub const CERT_ALT_NAME_OTHER_NAME: CERT_ALT_NAME = CERT_ALT_NAME(1i32);
pub const CERT_ALT_NAME_REGISTERED_ID: CERT_ALT_NAME = CERT_ALT_NAME(9i32);
pub const CERT_ALT_NAME_RFC822_NAME: CERT_ALT_NAME = CERT_ALT_NAME(2i32);
pub const CERT_ALT_NAME_URL: CERT_ALT_NAME = CERT_ALT_NAME(7i32);
pub const CMM_READONLY: u32 = 2u32;
pub const CMM_REFRESHONLY: u32 = 1u32;
pub const CPF_BADURL_ERROR: u32 = 32u32;
pub const CPF_BASE: u32 = 1u32;
pub const CPF_CASTORE_ERROR: u32 = 16u32;
pub const CPF_COMPLETE: u32 = 4u32;
pub const CPF_DELTA: u32 = 2u32;
pub const CPF_FILE_ERROR: u32 = 512u32;
pub const CPF_FTP_ERROR: u32 = 1024u32;
pub const CPF_HTTP_ERROR: u32 = 2048u32;
pub const CPF_LDAP_ERROR: u32 = 256u32;
pub const CPF_MANUAL: u32 = 64u32;
pub const CPF_POSTPONED_BASE_FILE_ERROR: u32 = 8192u32;
pub const CPF_POSTPONED_BASE_LDAP_ERROR: u32 = 4096u32;
pub const CPF_SHADOW: u32 = 8u32;
pub const CPF_SIGNATURE_ERROR: u32 = 128u32;
pub const CRLF_ALLOW_REQUEST_ATTRIBUTE_SUBJECT: u32 = 65536u32;
pub const CRLF_BUILD_ROOTCA_CRLENTRIES_BASEDONKEY: u32 = 2097152u32;
pub const CRLF_CRLNUMBER_CRITICAL: u32 = 4u32;
pub const CRLF_DELETE_EXPIRED_CRLS: u32 = 2u32;
pub const CRLF_DELTA_USE_OLDEST_UNEXPIRED_BASE: u32 = 1u32;
pub const CRLF_DISABLE_CHAIN_VERIFICATION: u32 = 1048576u32;
pub const CRLF_DISABLE_RDN_REORDER: u32 = 2048u32;
pub const CRLF_DISABLE_ROOT_CROSS_CERTS: u32 = 4096u32;
pub const CRLF_ENFORCE_ENROLLMENT_AGENT: u32 = 1024u32;
pub const CRLF_IGNORE_CROSS_CERT_TRUST_ERROR: u32 = 256u32;
pub const CRLF_IGNORE_INVALID_POLICIES: u32 = 16u32;
pub const CRLF_IGNORE_UNKNOWN_CMC_ATTRIBUTES: u32 = 128u32;
pub const CRLF_LOG_FULL_RESPONSE: u32 = 8192u32;
pub const CRLF_PRESERVE_EXPIRED_CA_CERTS: u32 = 262144u32;
pub const CRLF_PRESERVE_REVOKED_CA_CERTS: u32 = 524288u32;
pub const CRLF_PUBLISH_EXPIRED_CERT_CRLS: u32 = 512u32;
pub const CRLF_REBUILD_MODIFIED_SUBJECT_ONLY: u32 = 32u32;
pub const CRLF_REVCHECK_IGNORE_NOREVCHECK: u32 = 131072u32;
pub const CRLF_REVCHECK_IGNORE_OFFLINE: u32 = 8u32;
pub const CRLF_SAVE_FAILED_CERTS: u32 = 64u32;
pub const CRLF_USE_CROSS_CERT_TEMPLATE: u32 = 32768u32;
pub const CRLF_USE_XCHG_CERT_TEMPLATE: u32 = 16384u32;
pub const CRYPT_ENUM_ALL_PROVIDERS: u32 = 1u32;
pub const CR_DISP_DENIED: CR_DISP = CR_DISP(2u32);
pub const CR_DISP_ERROR: CR_DISP = CR_DISP(1u32);
pub const CR_DISP_INCOMPLETE: CR_DISP = CR_DISP(0u32);
pub const CR_DISP_ISSUED: CR_DISP = CR_DISP(3u32);
pub const CR_DISP_ISSUED_OUT_OF_BAND: CR_DISP = CR_DISP(4u32);
pub const CR_DISP_REVOKED: u32 = 6u32;
pub const CR_DISP_UNDER_SUBMISSION: CR_DISP = CR_DISP(5u32);
pub const CR_FLG_CACROSSCERT: u32 = 128u32;
pub const CR_FLG_CAXCHGCERT: u32 = 8u32;
pub const CR_FLG_CHALLENGEPENDING: u32 = 1024u32;
pub const CR_FLG_CHALLENGESATISFIED: u32 = 2048u32;
pub const CR_FLG_DEFINEDCACERT: u32 = 512u32;
pub const CR_FLG_ENFORCEUTF8: u32 = 256u32;
pub const CR_FLG_ENROLLONBEHALFOF: u32 = 16u32;
pub const CR_FLG_FORCETELETEX: u32 = 1u32;
pub const CR_FLG_FORCEUTF8: u32 = 4u32;
pub const CR_FLG_PUBLISHERROR: u32 = 2147483648u32;
pub const CR_FLG_RENEWAL: u32 = 2u32;
pub const CR_FLG_SUBJECTUNMODIFIED: u32 = 32u32;
pub const CR_FLG_TRUSTEKCERT: u32 = 8192u32;
pub const CR_FLG_TRUSTEKKEY: u32 = 16384u32;
pub const CR_FLG_TRUSTONUSE: u32 = 4096u32;
pub const CR_FLG_VALIDENCRYPTEDKEYHASH: u32 = 64u32;
pub const CR_GEMT_DEFAULT: u32 = 0u32;
pub const CR_GEMT_HRESULT_STRING: u32 = 1u32;
pub const CR_GEMT_HTTP_ERROR: u32 = 2u32;
pub const CR_IN_BASE64: CERT_IMPORT_FLAGS = CERT_IMPORT_FLAGS(1i32);
pub const CR_IN_BASE64HEADER: CERT_IMPORT_FLAGS = CERT_IMPORT_FLAGS(0i32);
pub const CR_IN_BINARY: CERT_IMPORT_FLAGS = CERT_IMPORT_FLAGS(2i32);
pub const CR_IN_CERTIFICATETRANSPARENCY: u32 = 67108864u32;
pub const CR_IN_CHALLENGERESPONSE: u32 = 1280u32;
pub const CR_IN_CLIENTIDNONE: u32 = 4194304u32;
pub const CR_IN_CMC: u32 = 1024u32;
pub const CR_IN_CONNECTONLY: u32 = 8388608u32;
pub const CR_IN_CRLS: u32 = 524288u32;
pub const CR_IN_ENCODEANY: u32 = 255u32;
pub const CR_IN_ENCODEMASK: u32 = 255u32;
pub const CR_IN_FORMATANY: u32 = 0u32;
pub const CR_IN_FORMATMASK: u32 = 65280u32;
pub const CR_IN_FULLRESPONSE: u32 = 262144u32;
pub const CR_IN_HTTP: u32 = 196608u32;
pub const CR_IN_KEYGEN: u32 = 512u32;
pub const CR_IN_MACHINE: u32 = 1048576u32;
pub const CR_IN_PKCS10: u32 = 256u32;
pub const CR_IN_PKCS7: u32 = 768u32;
pub const CR_IN_RETURNCHALLENGE: u32 = 16777216u32;
pub const CR_IN_ROBO: u32 = 2097152u32;
pub const CR_IN_RPC: u32 = 131072u32;
pub const CR_IN_SCEP: u32 = 65536u32;
pub const CR_IN_SCEPPOST: u32 = 33554432u32;
pub const CR_IN_SIGNEDCERTIFICATETIMESTAMPLIST: u32 = 1536u32;
pub const CR_OUT_BASE64: CERT_REQUEST_OUT_TYPE = CERT_REQUEST_OUT_TYPE(1i32);
pub const CR_OUT_BASE64HEADER: CERT_REQUEST_OUT_TYPE = CERT_REQUEST_OUT_TYPE(0i32);
pub const CR_OUT_BASE64REQUESTHEADER: u32 = 3u32;
pub const CR_OUT_BASE64X509CRLHEADER: u32 = 9u32;
pub const CR_OUT_BINARY: CERT_REQUEST_OUT_TYPE = CERT_REQUEST_OUT_TYPE(2i32);
pub const CR_OUT_CHAIN: u32 = 256u32;
pub const CR_OUT_CRLS: u32 = 512u32;
pub const CR_OUT_ENCODEMASK: u32 = 255u32;
pub const CR_OUT_HEX: u32 = 4u32;
pub const CR_OUT_HEXADDR: u32 = 10u32;
pub const CR_OUT_HEXASCII: u32 = 5u32;
pub const CR_OUT_HEXASCIIADDR: u32 = 11u32;
pub const CR_OUT_HEXRAW: u32 = 12u32;
pub const CR_OUT_NOCR: u32 = 2147483648u32;
pub const CR_OUT_NOCRLF: u32 = 1073741824u32;
pub const CR_PROP_ADVANCEDSERVER: u32 = 28u32;
pub const CR_PROP_BASECRL: u32 = 17u32;
pub const CR_PROP_BASECRLPUBLISHSTATUS: u32 = 30u32;
pub const CR_PROP_CABACKWARDCROSSCERT: u32 = 36u32;
pub const CR_PROP_CABACKWARDCROSSCERTSTATE: u32 = 38u32;
pub const CR_PROP_CACERTSTATE: u32 = 19u32;
pub const CR_PROP_CACERTSTATUSCODE: u32 = 34u32;
pub const CR_PROP_CACERTVERSION: u32 = 39u32;
pub const CR_PROP_CAFORWARDCROSSCERT: u32 = 35u32;
pub const CR_PROP_CAFORWARDCROSSCERTSTATE: u32 = 37u32;
pub const CR_PROP_CANAME: u32 = 6u32;
pub const CR_PROP_CAPROPIDMAX: u32 = 21u32;
pub const CR_PROP_CASIGCERT: u32 = 12u32;
pub const CR_PROP_CASIGCERTCHAIN: u32 = 13u32;
pub const CR_PROP_CASIGCERTCOUNT: u32 = 11u32;
pub const CR_PROP_CASIGCERTCRLCHAIN: u32 = 32u32;
pub const CR_PROP_CATYPE: u32 = 10u32;
pub const CR_PROP_CAXCHGCERT: u32 = 15u32;
pub const CR_PROP_CAXCHGCERTCHAIN: u32 = 16u32;
pub const CR_PROP_CAXCHGCERTCOUNT: u32 = 14u32;
pub const CR_PROP_CAXCHGCERTCRLCHAIN: u32 = 33u32;
pub const CR_PROP_CERTAIAOCSPURLS: u32 = 43u32;
pub const CR_PROP_CERTAIAURLS: u32 = 42u32;
pub const CR_PROP_CERTCDPURLS: u32 = 41u32;
pub const CR_PROP_CRLSTATE: u32 = 20u32;
pub const CR_PROP_DELTACRL: u32 = 18u32;
pub const CR_PROP_DELTACRLPUBLISHSTATUS: u32 = 31u32;
pub const CR_PROP_DNSNAME: u32 = 22u32;
pub const CR_PROP_EXITCOUNT: u32 = 3u32;
pub const CR_PROP_EXITDESCRIPTION: u32 = 4u32;
pub const CR_PROP_FILEVERSION: u32 = 1u32;
pub const CR_PROP_KRACERT: u32 = 26u32;
pub const CR_PROP_KRACERTCOUNT: u32 = 25u32;
pub const CR_PROP_KRACERTSTATE: u32 = 27u32;
pub const CR_PROP_KRACERTUSEDCOUNT: u32 = 24u32;
pub const CR_PROP_LOCALENAME: u32 = 44u32;
pub const CR_PROP_NONE: u32 = 0u32;
pub const CR_PROP_PARENTCA: u32 = 9u32;
pub const CR_PROP_POLICYDESCRIPTION: u32 = 5u32;
pub const CR_PROP_PRODUCTVERSION: u32 = 2u32;
pub const CR_PROP_ROLESEPARATIONENABLED: u32 = 23u32;
pub const CR_PROP_SANITIZEDCANAME: u32 = 7u32;
pub const CR_PROP_SANITIZEDCASHORTNAME: u32 = 40u32;
pub const CR_PROP_SCEPMAX: u32 = 1002u32;
pub const CR_PROP_SCEPMIN: u32 = 1000u32;
pub const CR_PROP_SCEPSERVERCAPABILITIES: u32 = 1001u32;
pub const CR_PROP_SCEPSERVERCERTS: u32 = 1000u32;
pub const CR_PROP_SCEPSERVERCERTSCHAIN: u32 = 1002u32;
pub const CR_PROP_SHAREDFOLDER: u32 = 8u32;
pub const CR_PROP_SUBJECTTEMPLATE_OIDS: u32 = 45u32;
pub const CR_PROP_TEMPLATES: u32 = 29u32;
pub const CSBACKUP_DISABLE_INCREMENTAL: u32 = 4294967295u32;
pub const CSBACKUP_TYPE_FULL: CSBACKUP_TYPE = CSBACKUP_TYPE(1u32);
pub const CSBACKUP_TYPE_LOGS_ONLY: CSBACKUP_TYPE = CSBACKUP_TYPE(2u32);
pub const CSBACKUP_TYPE_MASK: u32 = 3u32;
pub const CSBFT_DATABASE_DIRECTORY: u32 = 64u32;
pub const CSBFT_DIRECTORY: u32 = 128u32;
pub const CSBFT_LOG_DIRECTORY: u32 = 32u32;
pub const CSCONTROL_RESTART: u64 = 3u64;
pub const CSCONTROL_SHUTDOWN: u64 = 1u64;
pub const CSCONTROL_SUSPEND: u64 = 2u64;
pub const CSRESTORE_TYPE_CATCHUP: u32 = 4u32;
pub const CSRESTORE_TYPE_FULL: u32 = 1u32;
pub const CSRESTORE_TYPE_MASK: u32 = 5u32;
pub const CSRESTORE_TYPE_ONLINE: u32 = 2u32;
pub const CSURL_ADDTOCERTCDP: u32 = 2u32;
pub const CSURL_ADDTOCERTOCSP: u32 = 32u32;
pub const CSURL_ADDTOCRLCDP: u32 = 8u32;
pub const CSURL_ADDTOFRESHESTCRL: u32 = 4u32;
pub const CSURL_ADDTOIDP: u32 = 128u32;
pub const CSURL_PUBLISHRETRY: u32 = 16u32;
pub const CSURL_SERVERPUBLISH: u32 = 1u32;
pub const CSURL_SERVERPUBLISHDELTA: u32 = 64u32;
pub const CSVER_MAJOR: u32 = 7u32;
pub const CSVER_MAJOR_LONGHORN: u32 = 3u32;
pub const CSVER_MAJOR_THRESHOLD: u32 = 7u32;
pub const CSVER_MAJOR_WHISTLER: u32 = 2u32;
pub const CSVER_MAJOR_WIN2K: u32 = 1u32;
pub const CSVER_MAJOR_WIN7: u32 = 4u32;
pub const CSVER_MAJOR_WIN8: u32 = 5u32;
pub const CSVER_MAJOR_WINBLUE: u32 = 6u32;
pub const CSVER_MINOR: u32 = 1u32;
pub const CSVER_MINOR_LONGHORN_BETA1: u32 = 1u32;
pub const CSVER_MINOR_THRESHOLD: u32 = 1u32;
pub const CSVER_MINOR_WHISTLER_BETA2: u32 = 1u32;
pub const CSVER_MINOR_WHISTLER_BETA3: u32 = 2u32;
pub const CSVER_MINOR_WIN2K: u32 = 1u32;
pub const CSVER_MINOR_WIN7: u32 = 1u32;
pub const CSVER_MINOR_WIN8: u32 = 1u32;
pub const CSVER_MINOR_WINBLUE: u32 = 1u32;
pub const CVIEWAGEMINUTESDEFAULT: u32 = 16u32;
pub const CVRC_COLUMN_MASK: CVRC_COLUMN = CVRC_COLUMN(4095i32);
pub const CVRC_COLUMN_RESULT: CVRC_COLUMN = CVRC_COLUMN(1i32);
pub const CVRC_COLUMN_SCHEMA: CVRC_COLUMN = CVRC_COLUMN(0i32);
pub const CVRC_COLUMN_VALUE: CVRC_COLUMN = CVRC_COLUMN(2i32);
pub const CVRC_TABLE_ATTRIBUTES: CVRC_TABLE = CVRC_TABLE(16384i32);
pub const CVRC_TABLE_CRL: CVRC_TABLE = CVRC_TABLE(20480i32);
pub const CVRC_TABLE_EXTENSIONS: CVRC_TABLE = CVRC_TABLE(12288i32);
pub const CVRC_TABLE_MASK: u32 = 61440u32;
pub const CVRC_TABLE_REQCERT: CVRC_TABLE = CVRC_TABLE(0i32);
pub const CVRC_TABLE_SHIFT: u32 = 12u32;
pub const CVR_SEEK_EQ: CERT_VIEW_SEEK_OPERATOR_FLAGS = CERT_VIEW_SEEK_OPERATOR_FLAGS(1i32);
pub const CVR_SEEK_GE: CERT_VIEW_SEEK_OPERATOR_FLAGS = CERT_VIEW_SEEK_OPERATOR_FLAGS(8i32);
pub const CVR_SEEK_GT: CERT_VIEW_SEEK_OPERATOR_FLAGS = CERT_VIEW_SEEK_OPERATOR_FLAGS(16i32);
pub const CVR_SEEK_LE: CERT_VIEW_SEEK_OPERATOR_FLAGS = CERT_VIEW_SEEK_OPERATOR_FLAGS(4i32);
pub const CVR_SEEK_LT: CERT_VIEW_SEEK_OPERATOR_FLAGS = CERT_VIEW_SEEK_OPERATOR_FLAGS(2i32);
pub const CVR_SEEK_MASK: u32 = 255u32;
pub const CVR_SEEK_NODELTA: u32 = 4096u32;
pub const CVR_SEEK_NONE: u32 = 0u32;
pub const CVR_SORT_ASCEND: u32 = 1u32;
pub const CVR_SORT_DESCEND: u32 = 2u32;
pub const CVR_SORT_NONE: u32 = 0u32;
pub const CV_COLUMN_ATTRIBUTE_DEFAULT: i32 = -5i32;
pub const CV_COLUMN_CRL_DEFAULT: i32 = -6i32;
pub const CV_COLUMN_EXTENSION_DEFAULT: i32 = -4i32;
pub const CV_COLUMN_LOG_DEFAULT: CERT_VIEW_COLUMN_INDEX = CERT_VIEW_COLUMN_INDEX(-2i32);
pub const CV_COLUMN_LOG_FAILED_DEFAULT: CERT_VIEW_COLUMN_INDEX = CERT_VIEW_COLUMN_INDEX(-3i32);
pub const CV_COLUMN_LOG_REVOKED_DEFAULT: i32 = -7i32;
pub const CV_COLUMN_QUEUE_DEFAULT: CERT_VIEW_COLUMN_INDEX = CERT_VIEW_COLUMN_INDEX(-1i32);
pub const CV_OUT_BASE64: ENUM_CERT_COLUMN_VALUE_FLAGS = ENUM_CERT_COLUMN_VALUE_FLAGS(1i32);
pub const CV_OUT_BASE64HEADER: ENUM_CERT_COLUMN_VALUE_FLAGS = ENUM_CERT_COLUMN_VALUE_FLAGS(0i32);
pub const CV_OUT_BASE64REQUESTHEADER: ENUM_CERT_COLUMN_VALUE_FLAGS = ENUM_CERT_COLUMN_VALUE_FLAGS(3i32);
pub const CV_OUT_BASE64X509CRLHEADER: ENUM_CERT_COLUMN_VALUE_FLAGS = ENUM_CERT_COLUMN_VALUE_FLAGS(9i32);
pub const CV_OUT_BINARY: ENUM_CERT_COLUMN_VALUE_FLAGS = ENUM_CERT_COLUMN_VALUE_FLAGS(2i32);
pub const CV_OUT_ENCODEMASK: u32 = 255u32;
pub const CV_OUT_HEX: ENUM_CERT_COLUMN_VALUE_FLAGS = ENUM_CERT_COLUMN_VALUE_FLAGS(4i32);
pub const CV_OUT_HEXADDR: ENUM_CERT_COLUMN_VALUE_FLAGS = ENUM_CERT_COLUMN_VALUE_FLAGS(10i32);
pub const CV_OUT_HEXASCII: ENUM_CERT_COLUMN_VALUE_FLAGS = ENUM_CERT_COLUMN_VALUE_FLAGS(5i32);
pub const CV_OUT_HEXASCIIADDR: ENUM_CERT_COLUMN_VALUE_FLAGS = ENUM_CERT_COLUMN_VALUE_FLAGS(11i32);
pub const CV_OUT_HEXRAW: u32 = 12u32;
pub const CV_OUT_NOCR: u32 = 2147483648u32;
pub const CV_OUT_NOCRLF: u32 = 1073741824u32;
pub const ClientIdAutoEnroll: RequestClientInfoClientId = RequestClientInfoClientId(6i32);
pub const ClientIdAutoEnroll2003: RequestClientInfoClientId = RequestClientInfoClientId(2i32);
pub const ClientIdCertReq: RequestClientInfoClientId = RequestClientInfoClientId(9i32);
pub const ClientIdCertReq2003: RequestClientInfoClientId = RequestClientInfoClientId(4i32);
pub const ClientIdDefaultRequest: RequestClientInfoClientId = RequestClientInfoClientId(5i32);
pub const ClientIdEOBO: RequestClientInfoClientId = RequestClientInfoClientId(8i32);
pub const ClientIdNone: RequestClientInfoClientId = RequestClientInfoClientId(0i32);
pub const ClientIdRequestWizard: RequestClientInfoClientId = RequestClientInfoClientId(7i32);
pub const ClientIdTest: RequestClientInfoClientId = RequestClientInfoClientId(10i32);
pub const ClientIdUserStart: RequestClientInfoClientId = RequestClientInfoClientId(1000i32);
pub const ClientIdWinRT: RequestClientInfoClientId = RequestClientInfoClientId(11i32);
pub const ClientIdWizard2003: RequestClientInfoClientId = RequestClientInfoClientId(3i32);
pub const ClientIdXEnroll2003: RequestClientInfoClientId = RequestClientInfoClientId(1i32);
pub const CommitFlagDeleteTemplate: CommitTemplateFlags = CommitTemplateFlags(4i32);
pub const CommitFlagSaveTemplateGenerateOID: CommitTemplateFlags = CommitTemplateFlags(1i32);
pub const CommitFlagSaveTemplateOverwrite: CommitTemplateFlags = CommitTemplateFlags(3i32);
pub const CommitFlagSaveTemplateUseCurrentOID: CommitTemplateFlags = CommitTemplateFlags(2i32);
pub const ContextAdministratorForceMachine: X509CertificateEnrollmentContext = X509CertificateEnrollmentContext(3i32);
pub const ContextMachine: X509CertificateEnrollmentContext = X509CertificateEnrollmentContext(2i32);
pub const ContextNone: X509CertificateEnrollmentContext = X509CertificateEnrollmentContext(0i32);
pub const ContextUser: X509CertificateEnrollmentContext = X509CertificateEnrollmentContext(1i32);
pub const DBFLAGS_CHECKPOINTDEPTH60MB: u32 = 32u32;
pub const DBFLAGS_CIRCULARLOGGING: u32 = 4u32;
pub const DBFLAGS_CREATEIFNEEDED: u32 = 2u32;
pub const DBFLAGS_DISABLESNAPSHOTBACKUP: u32 = 1024u32;
pub const DBFLAGS_ENABLEVOLATILEREQUESTS: u32 = 2048u32;
pub const DBFLAGS_LAZYFLUSH: u32 = 8u32;
pub const DBFLAGS_LOGBUFFERSHUGE: u32 = 128u32;
pub const DBFLAGS_LOGBUFFERSLARGE: u32 = 64u32;
pub const DBFLAGS_LOGFILESIZE16MB: u32 = 256u32;
pub const DBFLAGS_MAXCACHESIZEX100: u32 = 16u32;
pub const DBFLAGS_MULTITHREADTRANSACTIONS: u32 = 512u32;
pub const DBFLAGS_READONLY: u32 = 1u32;
pub const DBG_CERTSRV: u32 = 1u32;
pub const DBSESSIONCOUNTDEFAULT: u32 = 100u32;
pub const DB_DISP_ACTIVE: u32 = 8u32;
pub const DB_DISP_CA_CERT: u32 = 15u32;
pub const DB_DISP_CA_CERT_CHAIN: u32 = 16u32;
pub const DB_DISP_DENIED: u32 = 31u32;
pub const DB_DISP_ERROR: u32 = 30u32;
pub const DB_DISP_FOREIGN: u32 = 12u32;
pub const DB_DISP_ISSUED: u32 = 20u32;
pub const DB_DISP_KRA_CERT: u32 = 17u32;
pub const DB_DISP_LOG_FAILED_MIN: u32 = 30u32;
pub const DB_DISP_LOG_MIN: u32 = 20u32;
pub const DB_DISP_PENDING: u32 = 9u32;
pub const DB_DISP_QUEUE_MAX: u32 = 9u32;
pub const DB_DISP_REVOKED: u32 = 21u32;
pub const DefaultNone: EnrollmentPolicyServerPropertyFlags = EnrollmentPolicyServerPropertyFlags(0i32);
pub const DefaultPolicyServer: EnrollmentPolicyServerPropertyFlags = EnrollmentPolicyServerPropertyFlags(1i32);
pub const DelayRetryLong: DelayRetryAction = DelayRetryAction(3i32);
pub const DelayRetryNone: DelayRetryAction = DelayRetryAction(1i32);
pub const DelayRetryPastSuccess: DelayRetryAction = DelayRetryAction(5i32);
pub const DelayRetryShort: DelayRetryAction = DelayRetryAction(2i32);
pub const DelayRetrySuccess: DelayRetryAction = DelayRetryAction(4i32);
pub const DelayRetryUnknown: DelayRetryAction = DelayRetryAction(0i32);
pub const DisableGroupPolicyList: EnrollmentPolicyFlags = EnrollmentPolicyFlags(2i32);
pub const DisableUserServerList: EnrollmentPolicyFlags = EnrollmentPolicyFlags(4i32);
pub const DisplayNo: EnrollmentDisplayStatus = EnrollmentDisplayStatus(0i32);
pub const DisplayYes: EnrollmentDisplayStatus = EnrollmentDisplayStatus(1i32);
pub const EANR_SUPPRESS_IA5CONVERSION: u32 = 2147483648u32;
pub const EAN_NAMEOBJECTID: u32 = 2147483648u32;
pub const EDITF_ADDOLDCERTTYPE: u32 = 16u32;
pub const EDITF_ADDOLDKEYUSAGE: u32 = 8u32;
pub const EDITF_ATTRIBUTECA: u32 = 512u32;
pub const EDITF_ATTRIBUTEEKU: u32 = 32768u32;
pub const EDITF_ATTRIBUTEENDDATE: u32 = 32u32;
pub const EDITF_ATTRIBUTESUBJECTALTNAME2: u32 = 262144u32;
pub const EDITF_AUDITCERTTEMPLATELOAD: u32 = 2097152u32;
pub const EDITF_BASICCONSTRAINTSCA: u32 = 128u32;
pub const EDITF_BASICCONSTRAINTSCRITICAL: u32 = 64u32;
pub const EDITF_DISABLEEXTENSIONLIST: u32 = 4u32;
pub const EDITF_DISABLELDAPPACKAGELIST: u32 = 8388608u32;
pub const EDITF_DISABLEOLDOSCNUPN: u32 = 4194304u32;
pub const EDITF_EMAILOPTIONAL: u32 = 131072u32;
pub const EDITF_ENABLEAKICRITICAL: u32 = 8192u32;
pub const EDITF_ENABLEAKIISSUERNAME: u32 = 2048u32;
pub const EDITF_ENABLEAKIISSUERSERIAL: u32 = 4096u32;
pub const EDITF_ENABLEAKIKEYID: u32 = 256u32;
pub const EDITF_ENABLECHASECLIENTDC: u32 = 1048576u32;
pub const EDITF_ENABLEDEFAULTSMIME: u32 = 65536u32;
pub const EDITF_ENABLEKEYENCIPHERMENTCACERT: u32 = 134217728u32;
pub const EDITF_ENABLELDAPREFERRALS: u32 = 524288u32;
pub const EDITF_ENABLEOCSPREVNOCHECK: u32 = 33554432u32;
pub const EDITF_ENABLERENEWONBEHALFOF: u32 = 67108864u32;
pub const EDITF_ENABLEREQUESTEXTENSIONS: u32 = 1u32;
pub const EDITF_ENABLEUPNMAP: u32 = 16777216u32;
pub const EDITF_IGNOREREQUESTERGROUP: u32 = 1024u32;
pub const EDITF_REQUESTEXTENSIONLIST: u32 = 2u32;
pub const EDITF_SERVERUPGRADED: u32 = 16384u32;
pub const ENUMEXT_OBJECTID: u32 = 1u32;
pub const ENUM_ENTERPRISE_ROOTCA: ENUM_CATYPES = ENUM_CATYPES(0i32);
pub const ENUM_ENTERPRISE_SUBCA: ENUM_CATYPES = ENUM_CATYPES(1i32);
pub const ENUM_STANDALONE_ROOTCA: ENUM_CATYPES = ENUM_CATYPES(3i32);
pub const ENUM_STANDALONE_SUBCA: ENUM_CATYPES = ENUM_CATYPES(4i32);
pub const ENUM_UNKNOWN_CA: ENUM_CATYPES = ENUM_CATYPES(5i32);
pub const EXITEVENT_CERTDENIED: CERT_EXIT_EVENT_MASK = CERT_EXIT_EVENT_MASK(4u32);
pub const EXITEVENT_CERTIMPORTED: u32 = 512u32;
pub const EXITEVENT_CERTISSUED: CERT_EXIT_EVENT_MASK = CERT_EXIT_EVENT_MASK(1u32);
pub const EXITEVENT_CERTPENDING: CERT_EXIT_EVENT_MASK = CERT_EXIT_EVENT_MASK(2u32);
pub const EXITEVENT_CERTRETRIEVEPENDING: CERT_EXIT_EVENT_MASK = CERT_EXIT_EVENT_MASK(16u32);
pub const EXITEVENT_CERTREVOKED: CERT_EXIT_EVENT_MASK = CERT_EXIT_EVENT_MASK(8u32);
pub const EXITEVENT_CRLISSUED: CERT_EXIT_EVENT_MASK = CERT_EXIT_EVENT_MASK(32u32);
pub const EXITEVENT_INVALID: u32 = 0u32;
pub const EXITEVENT_SHUTDOWN: CERT_EXIT_EVENT_MASK = CERT_EXIT_EVENT_MASK(64u32);
pub const EXITEVENT_STARTUP: u32 = 128u32;
pub const EXITPUB_ACTIVEDIRECTORY: u32 = 2u32;
pub const EXITPUB_DEFAULT_ENTERPRISE: u32 = 2u32;
pub const EXITPUB_DEFAULT_STANDALONE: u32 = 1u32;
pub const EXITPUB_FILE: u32 = 1u32;
pub const EXITPUB_REMOVEOLDCERTS: u32 = 16u32;
pub const EXTENSION_CRITICAL_FLAG: u32 = 1u32;
pub const EXTENSION_DELETE_FLAG: u32 = 4u32;
pub const EXTENSION_DISABLE_FLAG: u32 = 2u32;
pub const EXTENSION_ORIGIN_ADMIN: u32 = 196608u32;
pub const EXTENSION_ORIGIN_CACERT: u32 = 589824u32;
pub const EXTENSION_ORIGIN_CMC: u32 = 524288u32;
pub const EXTENSION_ORIGIN_IMPORTEDCERT: u32 = 393216u32;
pub const EXTENSION_ORIGIN_MASK: u32 = 983040u32;
pub const EXTENSION_ORIGIN_PKCS7: u32 = 458752u32;
pub const EXTENSION_ORIGIN_POLICY: u32 = 131072u32;
pub const EXTENSION_ORIGIN_RENEWALCERT: u32 = 327680u32;
pub const EXTENSION_ORIGIN_REQUEST: u32 = 65536u32;
pub const EXTENSION_ORIGIN_SERVER: u32 = 262144u32;
pub const EXTENSION_POLICY_MASK: u32 = 65535u32;
pub const EnrollDenied: EnrollmentEnrollStatus = EnrollmentEnrollStatus(256i32);
pub const EnrollError: EnrollmentEnrollStatus = EnrollmentEnrollStatus(16i32);
pub const EnrollPended: EnrollmentEnrollStatus = EnrollmentEnrollStatus(2i32);
pub const EnrollPrompt: WebEnrollmentFlags = WebEnrollmentFlags(1i32);
pub const EnrollSkipped: EnrollmentEnrollStatus = EnrollmentEnrollStatus(64i32);
pub const EnrollUIDeferredEnrollmentRequired: EnrollmentEnrollStatus = EnrollmentEnrollStatus(4i32);
pub const EnrollUnknown: EnrollmentEnrollStatus = EnrollmentEnrollStatus(32i32);
pub const Enrolled: EnrollmentEnrollStatus = EnrollmentEnrollStatus(1i32);
pub const EnrollmentAddOCSPNoCheck: X509CertificateTemplateEnrollmentFlag = X509CertificateTemplateEnrollmentFlag(4096i32);
pub const EnrollmentAddTemplateName: X509CertificateTemplateEnrollmentFlag = X509CertificateTemplateEnrollmentFlag(512i32);
pub const EnrollmentAllowEnrollOnBehalfOf: X509CertificateTemplateEnrollmentFlag = X509CertificateTemplateEnrollmentFlag(2048i32);
pub const EnrollmentAutoEnrollment: X509CertificateTemplateEnrollmentFlag = X509CertificateTemplateEnrollmentFlag(32i32);
pub const EnrollmentAutoEnrollmentCheckUserDSCertificate: X509CertificateTemplateEnrollmentFlag = X509CertificateTemplateEnrollmentFlag(16i32);
pub const EnrollmentCertificateIssuancePoliciesFromRequest: X509CertificateTemplateEnrollmentFlag = X509CertificateTemplateEnrollmentFlag(131072i32);
pub const EnrollmentDomainAuthenticationNotRequired: X509CertificateTemplateEnrollmentFlag = X509CertificateTemplateEnrollmentFlag(128i32);
pub const EnrollmentIncludeBasicConstraintsForEECerts: X509CertificateTemplateEnrollmentFlag = X509CertificateTemplateEnrollmentFlag(32768i32);
pub const EnrollmentIncludeSymmetricAlgorithms: X509CertificateTemplateEnrollmentFlag = X509CertificateTemplateEnrollmentFlag(1i32);
pub const EnrollmentNoRevocationInfoInCerts: X509CertificateTemplateEnrollmentFlag = X509CertificateTemplateEnrollmentFlag(16384i32);
pub const EnrollmentPendAllRequests: X509CertificateTemplateEnrollmentFlag = X509CertificateTemplateEnrollmentFlag(2i32);
pub const EnrollmentPreviousApprovalKeyBasedValidateReenrollment: X509CertificateTemplateEnrollmentFlag = X509CertificateTemplateEnrollmentFlag(65536i32);
pub const EnrollmentPreviousApprovalValidateReenrollment: X509CertificateTemplateEnrollmentFlag = X509CertificateTemplateEnrollmentFlag(64i32);
pub const EnrollmentPublishToDS: X509CertificateTemplateEnrollmentFlag = X509CertificateTemplateEnrollmentFlag(8i32);
pub const EnrollmentPublishToKRAContainer: X509CertificateTemplateEnrollmentFlag = X509CertificateTemplateEnrollmentFlag(4i32);
pub const EnrollmentRemoveInvalidCertificateFromPersonalStore: X509CertificateTemplateEnrollmentFlag = X509CertificateTemplateEnrollmentFlag(1024i32);
pub const EnrollmentReuseKeyOnFullSmartCard: X509CertificateTemplateEnrollmentFlag = X509CertificateTemplateEnrollmentFlag(8192i32);
pub const EnrollmentSkipAutoRenewal: X509CertificateTemplateEnrollmentFlag = X509CertificateTemplateEnrollmentFlag(262144i32);
pub const EnrollmentUserInteractionRequired: X509CertificateTemplateEnrollmentFlag = X509CertificateTemplateEnrollmentFlag(256i32);
pub const ExportCAs: X509EnrollmentPolicyExportFlags = X509EnrollmentPolicyExportFlags(4i32);
pub const ExportOIDs: X509EnrollmentPolicyExportFlags = X509EnrollmentPolicyExportFlags(2i32);
pub const ExportTemplates: X509EnrollmentPolicyExportFlags = X509EnrollmentPolicyExportFlags(1i32);
pub const FR_PROP_ATTESTATIONCHALLENGE: FULL_RESPONSE_PROPERTY_ID = FULL_RESPONSE_PROPERTY_ID(20i32);
pub const FR_PROP_ATTESTATIONPROVIDERNAME: FULL_RESPONSE_PROPERTY_ID = FULL_RESPONSE_PROPERTY_ID(21i32);
pub const FR_PROP_BODYPARTSTRING: FULL_RESPONSE_PROPERTY_ID = FULL_RESPONSE_PROPERTY_ID(3i32);
pub const FR_PROP_CAEXCHANGECERTIFICATE: FULL_RESPONSE_PROPERTY_ID = FULL_RESPONSE_PROPERTY_ID(17i32);
pub const FR_PROP_CAEXCHANGECERTIFICATECHAIN: FULL_RESPONSE_PROPERTY_ID = FULL_RESPONSE_PROPERTY_ID(18i32);
pub const FR_PROP_CAEXCHANGECERTIFICATECRLCHAIN: FULL_RESPONSE_PROPERTY_ID = FULL_RESPONSE_PROPERTY_ID(19i32);
pub const FR_PROP_CAEXCHANGECERTIFICATEHASH: FULL_RESPONSE_PROPERTY_ID = FULL_RESPONSE_PROPERTY_ID(16i32);
pub const FR_PROP_CLAIMCHALLENGE: u32 = 22u32;
pub const FR_PROP_ENCRYPTEDKEYHASH: FULL_RESPONSE_PROPERTY_ID = FULL_RESPONSE_PROPERTY_ID(14i32);
pub const FR_PROP_FAILINFO: FULL_RESPONSE_PROPERTY_ID = FULL_RESPONSE_PROPERTY_ID(7i32);
pub const FR_PROP_FULLRESPONSE: FULL_RESPONSE_PROPERTY_ID = FULL_RESPONSE_PROPERTY_ID(1i32);
pub const FR_PROP_FULLRESPONSENOPKCS7: FULL_RESPONSE_PROPERTY_ID = FULL_RESPONSE_PROPERTY_ID(15i32);
pub const FR_PROP_ISSUEDCERTIFICATE: FULL_RESPONSE_PROPERTY_ID = FULL_RESPONSE_PROPERTY_ID(11i32);
pub const FR_PROP_ISSUEDCERTIFICATECHAIN: FULL_RESPONSE_PROPERTY_ID = FULL_RESPONSE_PROPERTY_ID(12i32);
pub const FR_PROP_ISSUEDCERTIFICATECRLCHAIN: FULL_RESPONSE_PROPERTY_ID = FULL_RESPONSE_PROPERTY_ID(13i32);
pub const FR_PROP_ISSUEDCERTIFICATEHASH: FULL_RESPONSE_PROPERTY_ID = FULL_RESPONSE_PROPERTY_ID(10i32);
pub const FR_PROP_NONE: FULL_RESPONSE_PROPERTY_ID = FULL_RESPONSE_PROPERTY_ID(0i32);
pub const FR_PROP_OTHERINFOCHOICE: FULL_RESPONSE_PROPERTY_ID = FULL_RESPONSE_PROPERTY_ID(6i32);
pub const FR_PROP_PENDINFOTIME: FULL_RESPONSE_PROPERTY_ID = FULL_RESPONSE_PROPERTY_ID(9i32);
pub const FR_PROP_PENDINFOTOKEN: FULL_RESPONSE_PROPERTY_ID = FULL_RESPONSE_PROPERTY_ID(8i32);
pub const FR_PROP_STATUS: FULL_RESPONSE_PROPERTY_ID = FULL_RESPONSE_PROPERTY_ID(4i32);
pub const FR_PROP_STATUSINFOCOUNT: FULL_RESPONSE_PROPERTY_ID = FULL_RESPONSE_PROPERTY_ID(2i32);
pub const FR_PROP_STATUSSTRING: FULL_RESPONSE_PROPERTY_ID = FULL_RESPONSE_PROPERTY_ID(5i32);
pub const GeneralCA: X509CertificateTemplateGeneralFlag = X509CertificateTemplateGeneralFlag(128i32);
pub const GeneralCrossCA: X509CertificateTemplateGeneralFlag = X509CertificateTemplateGeneralFlag(2048i32);
pub const GeneralDefault: X509CertificateTemplateGeneralFlag = X509CertificateTemplateGeneralFlag(65536i32);
pub const GeneralDonotPersist: X509CertificateTemplateGeneralFlag = X509CertificateTemplateGeneralFlag(4096i32);
pub const GeneralMachineType: X509CertificateTemplateGeneralFlag = X509CertificateTemplateGeneralFlag(64i32);
pub const GeneralModified: X509CertificateTemplateGeneralFlag = X509CertificateTemplateGeneralFlag(131072i32);
pub const ICF_ALLOWFOREIGN: u32 = 65536u32;
pub const ICF_EXISTINGROW: u32 = 131072u32;
pub const IF_ENABLEADMINASAUDITOR: u32 = 4096u32;
pub const IF_ENABLEEXITKEYRETRIEVAL: u32 = 2048u32;
pub const IF_ENFORCEENCRYPTICERTADMIN: u32 = 1024u32;
pub const IF_ENFORCEENCRYPTICERTREQUEST: u32 = 512u32;
pub const IF_LOCKICERTREQUEST: u32 = 1u32;
pub const IF_NOLOCALICERTADMIN: u32 = 32u32;
pub const IF_NOLOCALICERTADMINBACKUP: u32 = 128u32;
pub const IF_NOLOCALICERTREQUEST: u32 = 4u32;
pub const IF_NOREMOTEICERTADMIN: u32 = 16u32;
pub const IF_NOREMOTEICERTADMINBACKUP: u32 = 64u32;
pub const IF_NOREMOTEICERTREQUEST: u32 = 2u32;
pub const IF_NORPCICERTREQUEST: u32 = 8u32;
pub const IF_NOSNAPSHOTBACKUP: u32 = 256u32;
pub const IKF_OVERWRITE: u32 = 65536u32;
pub const ISSCERT_DEFAULT_DS: u32 = 256u32;
pub const ISSCERT_DEFAULT_NODS: u32 = 256u32;
pub const ISSCERT_ENABLE: u32 = 256u32;
pub const ISSCERT_FILEURL_OLD: u32 = 8u32;
pub const ISSCERT_FTPURL_OLD: u32 = 4u32;
pub const ISSCERT_HTTPURL_OLD: u32 = 2u32;
pub const ISSCERT_LDAPURL_OLD: u32 = 1u32;
pub const ISSCERT_URLMASK_OLD: u32 = 255u32;
pub const ImportExportable: ImportPFXFlags = ImportPFXFlags(16i32);
pub const ImportExportableEncrypted: ImportPFXFlags = ImportPFXFlags(32i32);
pub const ImportForceOverwrite: ImportPFXFlags = ImportPFXFlags(2i32);
pub const ImportInstallCertificate: ImportPFXFlags = ImportPFXFlags(512i32);
pub const ImportInstallChain: ImportPFXFlags = ImportPFXFlags(1024i32);
pub const ImportInstallChainAndRoot: ImportPFXFlags = ImportPFXFlags(2048i32);
pub const ImportMachineContext: ImportPFXFlags = ImportPFXFlags(1i32);
pub const ImportNoUserProtected: ImportPFXFlags = ImportPFXFlags(64i32);
pub const ImportNone: ImportPFXFlags = ImportPFXFlags(0i32);
pub const ImportSaveProperties: ImportPFXFlags = ImportPFXFlags(8i32);
pub const ImportSilent: ImportPFXFlags = ImportPFXFlags(4i32);
pub const ImportUserProtected: ImportPFXFlags = ImportPFXFlags(128i32);
pub const ImportUserProtectedHigh: ImportPFXFlags = ImportPFXFlags(256i32);
pub const InheritDefault: X509RequestInheritOptions = X509RequestInheritOptions(0i32);
pub const InheritExtensionsFlag: X509RequestInheritOptions = X509RequestInheritOptions(256i32);
pub const InheritKeyMask: X509RequestInheritOptions = X509RequestInheritOptions(15i32);
pub const InheritNewDefaultKey: X509RequestInheritOptions = X509RequestInheritOptions(1i32);
pub const InheritNewSimilarKey: X509RequestInheritOptions = X509RequestInheritOptions(2i32);
pub const InheritNone: X509RequestInheritOptions = X509RequestInheritOptions(16i32);
pub const InheritPrivateKey: X509RequestInheritOptions = X509RequestInheritOptions(3i32);
pub const InheritPublicKey: X509RequestInheritOptions = X509RequestInheritOptions(4i32);
pub const InheritRenewalCertificateFlag: X509RequestInheritOptions = X509RequestInheritOptions(32i32);
pub const InheritReserved80000000: X509RequestInheritOptions = X509RequestInheritOptions(-2147483648i32);
pub const InheritSubjectAltNameFlag: X509RequestInheritOptions = X509RequestInheritOptions(512i32);
pub const InheritSubjectFlag: X509RequestInheritOptions = X509RequestInheritOptions(128i32);
pub const InheritTemplateFlag: X509RequestInheritOptions = X509RequestInheritOptions(64i32);
pub const InheritValidityPeriodFlag: X509RequestInheritOptions = X509RequestInheritOptions(1024i32);
pub const KRAF_DISABLEUSEDEFAULTPROVIDER: u32 = 8u32;
pub const KRAF_ENABLEARCHIVEALL: u32 = 4u32;
pub const KRAF_ENABLEFOREIGN: u32 = 1u32;
pub const KRAF_SAVEBADREQUESTKEY: u32 = 2u32;
pub const KRA_DISP_EXPIRED: u32 = 0u32;
pub const KRA_DISP_INVALID: u32 = 4u32;
pub const KRA_DISP_NOTFOUND: u32 = 1u32;
pub const KRA_DISP_NOTLOADED: u32 = 6u32;
pub const KRA_DISP_REVOKED: u32 = 2u32;
pub const KRA_DISP_UNTRUSTED: u32 = 5u32;
pub const KRA_DISP_VALID: u32 = 3u32;
pub const KR_ENABLE_MACHINE: u32 = 1u32;
pub const KR_ENABLE_USER: u32 = 2u32;
pub const LDAPF_SIGNDISABLE: u32 = 2u32;
pub const LDAPF_SSLENABLE: u32 = 1u32;
pub const LevelInnermost: InnerRequestLevel = InnerRequestLevel(0i32);
pub const LevelNext: InnerRequestLevel = InnerRequestLevel(1i32);
pub const LevelSafe: WebSecurityLevel = WebSecurityLevel(1i32);
pub const LevelUnsafe: WebSecurityLevel = WebSecurityLevel(0i32);
pub const LoadOptionCacheOnly: X509EnrollmentPolicyLoadOption = X509EnrollmentPolicyLoadOption(1i32);
pub const LoadOptionDefault: X509EnrollmentPolicyLoadOption = X509EnrollmentPolicyLoadOption(0i32);
pub const LoadOptionRegisterForADChanges: X509EnrollmentPolicyLoadOption = X509EnrollmentPolicyLoadOption(4i32);
pub const LoadOptionReload: X509EnrollmentPolicyLoadOption = X509EnrollmentPolicyLoadOption(2i32);
pub const OCSP_RF_REJECT_SIGNED_REQUESTS: OCSPRequestFlag = OCSPRequestFlag(1i32);
pub const OCSP_SF_ALLOW_NONCE_EXTENSION: OCSPSigningFlag = OCSPSigningFlag(256i32);
pub const OCSP_SF_ALLOW_SIGNINGCERT_AUTOENROLLMENT: OCSPSigningFlag = OCSPSigningFlag(512i32);
pub const OCSP_SF_ALLOW_SIGNINGCERT_AUTORENEWAL: OCSPSigningFlag = OCSPSigningFlag(4i32);
pub const OCSP_SF_AUTODISCOVER_SIGNINGCERT: OCSPSigningFlag = OCSPSigningFlag(16i32);
pub const OCSP_SF_FORCE_SIGNINGCERT_ISSUER_ISCA: OCSPSigningFlag = OCSPSigningFlag(8i32);
pub const OCSP_SF_MANUAL_ASSIGN_SIGNINGCERT: OCSPSigningFlag = OCSPSigningFlag(32i32);
pub const OCSP_SF_RESPONDER_ID_KEYHASH: OCSPSigningFlag = OCSPSigningFlag(64i32);
pub const OCSP_SF_RESPONDER_ID_NAME: OCSPSigningFlag = OCSPSigningFlag(128i32);
pub const OCSP_SF_SILENT: OCSPSigningFlag = OCSPSigningFlag(1i32);
pub const OCSP_SF_USE_CACERT: OCSPSigningFlag = OCSPSigningFlag(2i32);
pub const PFXExportChainNoRoot: PFXExportOptions = PFXExportOptions(1i32);
pub const PFXExportChainWithRoot: PFXExportOptions = PFXExportOptions(2i32);
pub const PFXExportEEOnly: PFXExportOptions = PFXExportOptions(0i32);
pub const PROCFLG_ENFORCEGOODKEYS: u32 = 1u32;
pub const PROCFLG_NONE: u32 = 0u32;
pub const PROPCALLER_ADMIN: u32 = 1024u32;
pub const PROPCALLER_EXIT: u32 = 768u32;
pub const PROPCALLER_MASK: u32 = 3840u32;
pub const PROPCALLER_POLICY: u32 = 512u32;
pub const PROPCALLER_REQUEST: u32 = 1280u32;
pub const PROPCALLER_SERVER: u32 = 256u32;
pub const PROPFLAGS_INDEXED: u32 = 65536u32;
pub const PROPTYPE_BINARY: CERT_PROPERTY_TYPE = CERT_PROPERTY_TYPE(3i32);
pub const PROPTYPE_DATE: CERT_PROPERTY_TYPE = CERT_PROPERTY_TYPE(2i32);
pub const PROPTYPE_LONG: CERT_PROPERTY_TYPE = CERT_PROPERTY_TYPE(1i32);
pub const PROPTYPE_MASK: u32 = 255u32;
pub const PROPTYPE_STRING: CERT_PROPERTY_TYPE = CERT_PROPERTY_TYPE(4i32);
pub const PolicyQualifierTypeFlags: PolicyQualifierType = PolicyQualifierType(3i32);
pub const PolicyQualifierTypeUnknown: PolicyQualifierType = PolicyQualifierType(0i32);
pub const PolicyQualifierTypeUrl: PolicyQualifierType = PolicyQualifierType(1i32);
pub const PolicyQualifierTypeUserNotice: PolicyQualifierType = PolicyQualifierType(2i32);
pub const PrivateKeyAttestMask: X509CertificateTemplatePrivateKeyFlag = X509CertificateTemplatePrivateKeyFlag(12288i32);
pub const PrivateKeyAttestNone: X509CertificateTemplatePrivateKeyFlag = X509CertificateTemplatePrivateKeyFlag(0i32);
pub const PrivateKeyAttestPreferred: X509CertificateTemplatePrivateKeyFlag = X509CertificateTemplatePrivateKeyFlag(4096i32);
pub const PrivateKeyAttestRequired: X509CertificateTemplatePrivateKeyFlag = X509CertificateTemplatePrivateKeyFlag(8192i32);
pub const PrivateKeyAttestWithoutPolicy: X509CertificateTemplatePrivateKeyFlag = X509CertificateTemplatePrivateKeyFlag(16384i32);
pub const PrivateKeyClientVersionMask: X509CertificateTemplatePrivateKeyFlag = X509CertificateTemplatePrivateKeyFlag(251658240i32);
pub const PrivateKeyClientVersionShift: X509CertificateTemplatePrivateKeyFlag = X509CertificateTemplatePrivateKeyFlag(24i32);
pub const PrivateKeyEKTrustOnUse: X509CertificateTemplatePrivateKeyFlag = X509CertificateTemplatePrivateKeyFlag(512i32);
pub const PrivateKeyEKValidateCert: X509CertificateTemplatePrivateKeyFlag = X509CertificateTemplatePrivateKeyFlag(1024i32);
pub const PrivateKeyEKValidateKey: X509CertificateTemplatePrivateKeyFlag = X509CertificateTemplatePrivateKeyFlag(2048i32);
pub const PrivateKeyExportable: X509CertificateTemplatePrivateKeyFlag = X509CertificateTemplatePrivateKeyFlag(16i32);
pub const PrivateKeyHelloKspKey: X509CertificateTemplatePrivateKeyFlag = X509CertificateTemplatePrivateKeyFlag(1048576i32);
pub const PrivateKeyHelloLogonKey: X509CertificateTemplatePrivateKeyFlag = X509CertificateTemplatePrivateKeyFlag(2097152i32);
pub const PrivateKeyRequireAlternateSignatureAlgorithm: X509CertificateTemplatePrivateKeyFlag = X509CertificateTemplatePrivateKeyFlag(64i32);
pub const PrivateKeyRequireArchival: X509CertificateTemplatePrivateKeyFlag = X509CertificateTemplatePrivateKeyFlag(1i32);
pub const PrivateKeyRequireSameKeyRenewal: X509CertificateTemplatePrivateKeyFlag = X509CertificateTemplatePrivateKeyFlag(128i32);
pub const PrivateKeyRequireStrongKeyProtection: X509CertificateTemplatePrivateKeyFlag = X509CertificateTemplatePrivateKeyFlag(32i32);
pub const PrivateKeyServerVersionMask: X509CertificateTemplatePrivateKeyFlag = X509CertificateTemplatePrivateKeyFlag(983040i32);
pub const PrivateKeyServerVersionShift: X509CertificateTemplatePrivateKeyFlag = X509CertificateTemplatePrivateKeyFlag(16i32);
pub const PrivateKeyUseLegacyProvider: X509CertificateTemplatePrivateKeyFlag = X509CertificateTemplatePrivateKeyFlag(256i32);
pub const PsFriendlyName: PolicyServerUrlPropertyID = PolicyServerUrlPropertyID(1i32);
pub const PsPolicyID: PolicyServerUrlPropertyID = PolicyServerUrlPropertyID(0i32);
pub const PsfAllowUnTrustedCA: PolicyServerUrlFlags = PolicyServerUrlFlags(32i32);
pub const PsfAutoEnrollmentEnabled: PolicyServerUrlFlags = PolicyServerUrlFlags(16i32);
pub const PsfLocationGroupPolicy: PolicyServerUrlFlags = PolicyServerUrlFlags(1i32);
pub const PsfLocationRegistry: PolicyServerUrlFlags = PolicyServerUrlFlags(2i32);
pub const PsfNone: PolicyServerUrlFlags = PolicyServerUrlFlags(0i32);
pub const PsfUseClientId: PolicyServerUrlFlags = PolicyServerUrlFlags(4i32);
pub const REQDISP_DEFAULT_ENTERPRISE: u32 = 1u32;
pub const REQDISP_DENY: u32 = 2u32;
pub const REQDISP_ISSUE: u32 = 1u32;
pub const REQDISP_MASK: u32 = 255u32;
pub const REQDISP_PENDING: u32 = 0u32;
pub const REQDISP_PENDINGFIRST: u32 = 256u32;
pub const REQDISP_USEREQUESTATTRIBUTE: u32 = 3u32;
pub const REVEXT_ASPENABLE: u32 = 512u32;
pub const REVEXT_CDPENABLE: u32 = 256u32;
pub const REVEXT_CDPFILEURL_OLD: u32 = 8u32;
pub const REVEXT_CDPFTPURL_OLD: u32 = 4u32;
pub const REVEXT_CDPHTTPURL_OLD: u32 = 2u32;
pub const REVEXT_CDPLDAPURL_OLD: u32 = 1u32;
pub const REVEXT_CDPURLMASK_OLD: u32 = 255u32;
pub const REVEXT_DEFAULT_DS: u32 = 256u32;
pub const REVEXT_DEFAULT_NODS: u32 = 256u32;
pub const SCEPDispositionFailure: X509SCEPDisposition = X509SCEPDisposition(2i32);
pub const SCEPDispositionPending: X509SCEPDisposition = X509SCEPDisposition(3i32);
pub const SCEPDispositionPendingChallenge: X509SCEPDisposition = X509SCEPDisposition(11i32);
pub const SCEPDispositionSuccess: X509SCEPDisposition = X509SCEPDisposition(0i32);
pub const SCEPDispositionUnknown: X509SCEPDisposition = X509SCEPDisposition(-1i32);
pub const SCEPFailBadAlgorithm: X509SCEPFailInfo = X509SCEPFailInfo(0i32);
pub const SCEPFailBadCertId: X509SCEPFailInfo = X509SCEPFailInfo(4i32);
pub const SCEPFailBadMessageCheck: X509SCEPFailInfo = X509SCEPFailInfo(1i32);
pub const SCEPFailBadRequest: X509SCEPFailInfo = X509SCEPFailInfo(2i32);
pub const SCEPFailBadTime: X509SCEPFailInfo = X509SCEPFailInfo(3i32);
pub const SCEPFailUnknown: X509SCEPFailInfo = X509SCEPFailInfo(-1i32);
pub const SCEPMessageCertResponse: X509SCEPMessageType = X509SCEPMessageType(3i32);
pub const SCEPMessageClaimChallengeAnswer: X509SCEPMessageType = X509SCEPMessageType(41i32);
pub const SCEPMessageGetCRL: X509SCEPMessageType = X509SCEPMessageType(22i32);
pub const SCEPMessageGetCert: X509SCEPMessageType = X509SCEPMessageType(21i32);
pub const SCEPMessageGetCertInitial: X509SCEPMessageType = X509SCEPMessageType(20i32);
pub const SCEPMessagePKCSRequest: X509SCEPMessageType = X509SCEPMessageType(19i32);
pub const SCEPMessageUnknown: X509SCEPMessageType = X509SCEPMessageType(-1i32);
pub const SCEPProcessDefault: X509SCEPProcessMessageFlags = X509SCEPProcessMessageFlags(0i32);
pub const SCEPProcessSkipCertInstall: X509SCEPProcessMessageFlags = X509SCEPProcessMessageFlags(1i32);
pub const SETUP_ATTEMPT_VROOT_CREATE: u32 = 128u32;
pub const SETUP_CLIENT_FLAG: u32 = 2u32;
pub const SETUP_CREATEDB_FLAG: u32 = 64u32;
pub const SETUP_DCOM_SECURITY_UPDATED_FLAG: u32 = 8192u32;
pub const SETUP_DENIED_FLAG: u32 = 32u32;
pub const SETUP_FORCECRL_FLAG: u32 = 256u32;
pub const SETUP_ONLINE_FLAG: u32 = 16u32;
pub const SETUP_REQUEST_FLAG: u32 = 8u32;
pub const SETUP_SECURITY_CHANGED: u32 = 4096u32;
pub const SETUP_SERVER_FLAG: u32 = 1u32;
pub const SETUP_SERVER_IS_UP_TO_DATE_FLAG: u32 = 16384u32;
pub const SETUP_SERVER_UPGRADED_FLAG: u32 = 1024u32;
pub const SETUP_SUSPEND_FLAG: u32 = 4u32;
pub const SETUP_UPDATE_CAOBJECT_SVRTYPE: u32 = 512u32;
pub const SETUP_W2K_SECURITY_NOT_UPGRADED_FLAG: u32 = 2048u32;
pub const SKIHashCapiSha1: KeyIdentifierHashAlgorithm = KeyIdentifierHashAlgorithm(2i32);
pub const SKIHashDefault: KeyIdentifierHashAlgorithm = KeyIdentifierHashAlgorithm(0i32);
pub const SKIHashHPKP: KeyIdentifierHashAlgorithm = KeyIdentifierHashAlgorithm(5i32);
pub const SKIHashSha1: KeyIdentifierHashAlgorithm = KeyIdentifierHashAlgorithm(1i32);
pub const SKIHashSha256: KeyIdentifierHashAlgorithm = KeyIdentifierHashAlgorithm(3i32);
pub const SelectedNo: EnrollmentSelectionStatus = EnrollmentSelectionStatus(0i32);
pub const SelectedYes: EnrollmentSelectionStatus = EnrollmentSelectionStatus(1i32);
pub const SubjectAlternativeNameEnrolleeSupplies: X509CertificateTemplateSubjectNameFlag = X509CertificateTemplateSubjectNameFlag(65536i32);
pub const SubjectAlternativeNameRequireDNS: X509CertificateTemplateSubjectNameFlag = X509CertificateTemplateSubjectNameFlag(134217728i32);
pub const SubjectAlternativeNameRequireDirectoryGUID: X509CertificateTemplateSubjectNameFlag = X509CertificateTemplateSubjectNameFlag(16777216i32);
pub const SubjectAlternativeNameRequireDomainDNS: X509CertificateTemplateSubjectNameFlag = X509CertificateTemplateSubjectNameFlag(4194304i32);
pub const SubjectAlternativeNameRequireEmail: X509CertificateTemplateSubjectNameFlag = X509CertificateTemplateSubjectNameFlag(67108864i32);
pub const SubjectAlternativeNameRequireSPN: X509CertificateTemplateSubjectNameFlag = X509CertificateTemplateSubjectNameFlag(8388608i32);
pub const SubjectAlternativeNameRequireUPN: X509CertificateTemplateSubjectNameFlag = X509CertificateTemplateSubjectNameFlag(33554432i32);
pub const SubjectNameAndAlternativeNameOldCertSupplies: X509CertificateTemplateSubjectNameFlag = X509CertificateTemplateSubjectNameFlag(8i32);
pub const SubjectNameEnrolleeSupplies: X509CertificateTemplateSubjectNameFlag = X509CertificateTemplateSubjectNameFlag(1i32);
pub const SubjectNameRequireCommonName: X509CertificateTemplateSubjectNameFlag = X509CertificateTemplateSubjectNameFlag(1073741824i32);
pub const SubjectNameRequireDNS: X509CertificateTemplateSubjectNameFlag = X509CertificateTemplateSubjectNameFlag(268435456i32);
pub const SubjectNameRequireDirectoryPath: X509CertificateTemplateSubjectNameFlag = X509CertificateTemplateSubjectNameFlag(-2147483648i32);
pub const SubjectNameRequireEmail: X509CertificateTemplateSubjectNameFlag = X509CertificateTemplateSubjectNameFlag(536870912i32);
pub const TP_MACHINEPOLICY: u32 = 1u32;
pub const TemplatePropAsymmetricAlgorithm: EnrollmentTemplateProperty = EnrollmentTemplateProperty(18i32);
pub const TemplatePropCertificatePolicies: EnrollmentTemplateProperty = EnrollmentTemplateProperty(16i32);
pub const TemplatePropCommonName: EnrollmentTemplateProperty = EnrollmentTemplateProperty(1i32);
pub const TemplatePropCryptoProviders: EnrollmentTemplateProperty = EnrollmentTemplateProperty(4i32);
pub const TemplatePropDescription: EnrollmentTemplateProperty = EnrollmentTemplateProperty(6i32);
pub const TemplatePropEKUs: EnrollmentTemplateProperty = EnrollmentTemplateProperty(3i32);
pub const TemplatePropEnrollmentFlags: EnrollmentTemplateProperty = EnrollmentTemplateProperty(24i32);
pub const TemplatePropExtensions: EnrollmentTemplateProperty = EnrollmentTemplateProperty(29i32);
pub const TemplatePropFriendlyName: EnrollmentTemplateProperty = EnrollmentTemplateProperty(2i32);
pub const TemplatePropGeneralFlags: EnrollmentTemplateProperty = EnrollmentTemplateProperty(27i32);
pub const TemplatePropHashAlgorithm: EnrollmentTemplateProperty = EnrollmentTemplateProperty(22i32);
pub const TemplatePropKeySecurityDescriptor: EnrollmentTemplateProperty = EnrollmentTemplateProperty(19i32);
pub const TemplatePropKeySpec: EnrollmentTemplateProperty = EnrollmentTemplateProperty(7i32);
pub const TemplatePropKeyUsage: EnrollmentTemplateProperty = EnrollmentTemplateProperty(23i32);
pub const TemplatePropMajorRevision: EnrollmentTemplateProperty = EnrollmentTemplateProperty(5i32);
pub const TemplatePropMinimumKeySize: EnrollmentTemplateProperty = EnrollmentTemplateProperty(11i32);
pub const TemplatePropMinorRevision: EnrollmentTemplateProperty = EnrollmentTemplateProperty(9i32);
pub const TemplatePropOID: EnrollmentTemplateProperty = EnrollmentTemplateProperty(12i32);
pub const TemplatePropPrivateKeyFlags: EnrollmentTemplateProperty = EnrollmentTemplateProperty(26i32);
pub const TemplatePropRACertificatePolicies: EnrollmentTemplateProperty = EnrollmentTemplateProperty(14i32);
pub const TemplatePropRAEKUs: EnrollmentTemplateProperty = EnrollmentTemplateProperty(15i32);
pub const TemplatePropRASignatureCount: EnrollmentTemplateProperty = EnrollmentTemplateProperty(10i32);
pub const TemplatePropRenewalPeriod: EnrollmentTemplateProperty = EnrollmentTemplateProperty(31i32);
pub const TemplatePropSchemaVersion: EnrollmentTemplateProperty = EnrollmentTemplateProperty(8i32);
pub const TemplatePropSecurityDescriptor: EnrollmentTemplateProperty = EnrollmentTemplateProperty(28i32);
pub const TemplatePropSubjectNameFlags: EnrollmentTemplateProperty = EnrollmentTemplateProperty(25i32);
pub const TemplatePropSupersede: EnrollmentTemplateProperty = EnrollmentTemplateProperty(13i32);
pub const TemplatePropSymmetricAlgorithm: EnrollmentTemplateProperty = EnrollmentTemplateProperty(20i32);
pub const TemplatePropSymmetricKeyLength: EnrollmentTemplateProperty = EnrollmentTemplateProperty(21i32);
pub const TemplatePropV1ApplicationPolicy: EnrollmentTemplateProperty = EnrollmentTemplateProperty(17i32);
pub const TemplatePropValidityPeriod: EnrollmentTemplateProperty = EnrollmentTemplateProperty(30i32);
pub const TypeAny: X509RequestType = X509RequestType(0i32);
pub const TypeCertificate: X509RequestType = X509RequestType(4i32);
pub const TypeCmc: X509RequestType = X509RequestType(3i32);
pub const TypePkcs10: X509RequestType = X509RequestType(1i32);
pub const TypePkcs7: X509RequestType = X509RequestType(2i32);
pub const VR_INSTANT_BAD: u32 = 2u32;
pub const VR_INSTANT_OK: u32 = 1u32;
pub const VR_PENDING: u32 = 0u32;
pub const VerifyAllowUI: X509PrivateKeyVerify = X509PrivateKeyVerify(4i32);
pub const VerifyNone: X509PrivateKeyVerify = X509PrivateKeyVerify(0i32);
pub const VerifySilent: X509PrivateKeyVerify = X509PrivateKeyVerify(1i32);
pub const VerifySmartCardNone: X509PrivateKeyVerify = X509PrivateKeyVerify(2i32);
pub const VerifySmartCardSilent: X509PrivateKeyVerify = X509PrivateKeyVerify(3i32);
pub const X509AuthAnonymous: X509EnrollmentAuthFlags = X509EnrollmentAuthFlags(1i32);
pub const X509AuthCertificate: X509EnrollmentAuthFlags = X509EnrollmentAuthFlags(8i32);
pub const X509AuthKerberos: X509EnrollmentAuthFlags = X509EnrollmentAuthFlags(2i32);
pub const X509AuthNone: X509EnrollmentAuthFlags = X509EnrollmentAuthFlags(0i32);
pub const X509AuthUsername: X509EnrollmentAuthFlags = X509EnrollmentAuthFlags(4i32);
pub const XCN_AT_KEYEXCHANGE: X509KeySpec = X509KeySpec(1i32);
pub const XCN_AT_NONE: X509KeySpec = X509KeySpec(0i32);
pub const XCN_AT_SIGNATURE: X509KeySpec = X509KeySpec(2i32);
pub const XCN_BCRYPT_ASYMMETRIC_ENCRYPTION_INTERFACE: AlgorithmType = AlgorithmType(3i32);
pub const XCN_BCRYPT_CIPHER_INTERFACE: AlgorithmType = AlgorithmType(1i32);
pub const XCN_BCRYPT_HASH_INTERFACE: AlgorithmType = AlgorithmType(2i32);
pub const XCN_BCRYPT_KEY_DERIVATION_INTERFACE: AlgorithmType = AlgorithmType(7i32);
pub const XCN_BCRYPT_RNG_INTERFACE: AlgorithmType = AlgorithmType(6i32);
pub const XCN_BCRYPT_SECRET_AGREEMENT_INTERFACE: AlgorithmType = AlgorithmType(4i32);
pub const XCN_BCRYPT_SIGNATURE_INTERFACE: AlgorithmType = AlgorithmType(5i32);
pub const XCN_BCRYPT_UNKNOWN_INTERFACE: AlgorithmType = AlgorithmType(0i32);
pub const XCN_CERT_ACCESS_STATE_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(14i32);
pub const XCN_CERT_AIA_URL_RETRIEVED_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(67i32);
pub const XCN_CERT_ALT_NAME_DIRECTORY_NAME: AlternativeNameType = AlternativeNameType(5i32);
pub const XCN_CERT_ALT_NAME_DNS_NAME: AlternativeNameType = AlternativeNameType(3i32);
pub const XCN_CERT_ALT_NAME_EDI_PARTY_NAME: AlternativeNameType = AlternativeNameType(6i32);
pub const XCN_CERT_ALT_NAME_GUID: AlternativeNameType = AlternativeNameType(10i32);
pub const XCN_CERT_ALT_NAME_IP_ADDRESS: AlternativeNameType = AlternativeNameType(8i32);
pub const XCN_CERT_ALT_NAME_OTHER_NAME: AlternativeNameType = AlternativeNameType(1i32);
pub const XCN_CERT_ALT_NAME_REGISTERED_ID: AlternativeNameType = AlternativeNameType(9i32);
pub const XCN_CERT_ALT_NAME_RFC822_NAME: AlternativeNameType = AlternativeNameType(2i32);
pub const XCN_CERT_ALT_NAME_UNKNOWN: AlternativeNameType = AlternativeNameType(0i32);
pub const XCN_CERT_ALT_NAME_URL: AlternativeNameType = AlternativeNameType(7i32);
pub const XCN_CERT_ALT_NAME_USER_PRINCIPLE_NAME: AlternativeNameType = AlternativeNameType(11i32);
pub const XCN_CERT_ALT_NAME_X400_ADDRESS: AlternativeNameType = AlternativeNameType(4i32);
pub const XCN_CERT_ARCHIVED_KEY_HASH_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(65i32);
pub const XCN_CERT_ARCHIVED_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(19i32);
pub const XCN_CERT_AUTHORITY_INFO_ACCESS_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(68i32);
pub const XCN_CERT_AUTH_ROOT_SHA256_HASH_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(98i32);
pub const XCN_CERT_AUTO_ENROLL_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(21i32);
pub const XCN_CERT_AUTO_ENROLL_RETRY_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(66i32);
pub const XCN_CERT_BACKED_UP_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(69i32);
pub const XCN_CERT_CA_DISABLE_CRL_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(82i32);
pub const XCN_CERT_CA_OCSP_AUTHORITY_INFO_ACCESS_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(81i32);
pub const XCN_CERT_CEP_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(87i32);
pub const XCN_CERT_CERT_NOT_BEFORE_ENHKEY_USAGE_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(127i32);
pub const XCN_CERT_CLR_DELETE_KEY_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(125i32);
pub const XCN_CERT_CRL_SIGN_KEY_USAGE: X509KeyUsageFlags = X509KeyUsageFlags(2i32);
pub const XCN_CERT_CROSS_CERT_DIST_POINTS_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(23i32);
pub const XCN_CERT_CTL_USAGE_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(9i32);
pub const XCN_CERT_DATA_ENCIPHERMENT_KEY_USAGE: X509KeyUsageFlags = X509KeyUsageFlags(16i32);
pub const XCN_CERT_DATE_STAMP_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(27i32);
pub const XCN_CERT_DECIPHER_ONLY_KEY_USAGE: X509KeyUsageFlags = X509KeyUsageFlags(32768i32);
pub const XCN_CERT_DESCRIPTION_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(13i32);
pub const XCN_CERT_DIGITAL_SIGNATURE_KEY_USAGE: X509KeyUsageFlags = X509KeyUsageFlags(128i32);
pub const XCN_CERT_DISALLOWED_ENHKEY_USAGE_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(122i32);
pub const XCN_CERT_DISALLOWED_FILETIME_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(104i32);
pub const XCN_CERT_EFS_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(17i32);
pub const XCN_CERT_ENCIPHER_ONLY_KEY_USAGE: X509KeyUsageFlags = X509KeyUsageFlags(1i32);
pub const XCN_CERT_ENHKEY_USAGE_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(9i32);
pub const XCN_CERT_ENROLLMENT_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(26i32);
pub const XCN_CERT_EXTENDED_ERROR_INFO_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(30i32);
pub const XCN_CERT_FIRST_RESERVED_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(129i32);
pub const XCN_CERT_FIRST_USER_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(32768i32);
pub const XCN_CERT_FORTEZZA_DATA_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(18i32);
pub const XCN_CERT_FRIENDLY_NAME_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(11i32);
pub const XCN_CERT_HASH_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(3i32);
pub const XCN_CERT_HCRYPTPROV_OR_NCRYPT_KEY_HANDLE_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(79i32);
pub const XCN_CERT_HCRYPTPROV_TRANSFER_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(100i32);
pub const XCN_CERT_IE30_RESERVED_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(7i32);
pub const XCN_CERT_ISOLATED_KEY_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(118i32);
pub const XCN_CERT_ISSUER_CHAIN_PUB_KEY_CNG_ALG_BIT_LENGTH_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(96i32);
pub const XCN_CERT_ISSUER_CHAIN_SIGN_HASH_CNG_ALG_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(95i32);
pub const XCN_CERT_ISSUER_PUBLIC_KEY_MD5_HASH_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(24i32);
pub const XCN_CERT_ISSUER_PUB_KEY_BIT_LENGTH_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(94i32);
pub const XCN_CERT_ISSUER_SERIAL_NUMBER_MD5_HASH_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(28i32);
pub const XCN_CERT_KEY_AGREEMENT_KEY_USAGE: X509KeyUsageFlags = X509KeyUsageFlags(8i32);
pub const XCN_CERT_KEY_CERT_SIGN_KEY_USAGE: X509KeyUsageFlags = X509KeyUsageFlags(4i32);
pub const XCN_CERT_KEY_CLASSIFICATION_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(120i32);
pub const XCN_CERT_KEY_CONTEXT_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(5i32);
pub const XCN_CERT_KEY_ENCIPHERMENT_KEY_USAGE: X509KeyUsageFlags = X509KeyUsageFlags(32i32);
pub const XCN_CERT_KEY_IDENTIFIER_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(20i32);
pub const XCN_CERT_KEY_PROV_HANDLE_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(1i32);
pub const XCN_CERT_KEY_PROV_INFO_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(2i32);
pub const XCN_CERT_KEY_REPAIR_ATTEMPTED_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(103i32);
pub const XCN_CERT_KEY_SPEC_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(6i32);
pub const XCN_CERT_LAST_RESERVED_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(32767i32);
pub const XCN_CERT_LAST_USER_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(65535i32);
pub const XCN_CERT_MD5_HASH_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(4i32);
pub const XCN_CERT_NAME_STR_AMBIGUOUS_SEPARATOR_FLAGS: X500NameFlags = X500NameFlags(1275068416i32);
pub const XCN_CERT_NAME_STR_COMMA_FLAG: X500NameFlags = X500NameFlags(67108864i32);
pub const XCN_CERT_NAME_STR_CRLF_FLAG: X500NameFlags = X500NameFlags(134217728i32);
pub const XCN_CERT_NAME_STR_DISABLE_IE4_UTF8_FLAG: X500NameFlags = X500NameFlags(65536i32);
pub const XCN_CERT_NAME_STR_DISABLE_UTF8_DIR_STR_FLAG: X500NameFlags = X500NameFlags(1048576i32);
pub const XCN_CERT_NAME_STR_DS_ESCAPED: X500NameFlags = X500NameFlags(8388608i32);
pub const XCN_CERT_NAME_STR_ENABLE_PUNYCODE_FLAG: X500NameFlags = X500NameFlags(2097152i32);
pub const XCN_CERT_NAME_STR_ENABLE_T61_UNICODE_FLAG: X500NameFlags = X500NameFlags(131072i32);
pub const XCN_CERT_NAME_STR_ENABLE_UTF8_UNICODE_FLAG: X500NameFlags = X500NameFlags(262144i32);
pub const XCN_CERT_NAME_STR_FORCE_UTF8_DIR_STR_FLAG: X500NameFlags = X500NameFlags(524288i32);
pub const XCN_CERT_NAME_STR_FORWARD_FLAG: X500NameFlags = X500NameFlags(16777216i32);
pub const XCN_CERT_NAME_STR_NONE: X500NameFlags = X500NameFlags(0i32);
pub const XCN_CERT_NAME_STR_NO_PLUS_FLAG: X500NameFlags = X500NameFlags(536870912i32);
pub const XCN_CERT_NAME_STR_NO_QUOTING_FLAG: X500NameFlags = X500NameFlags(268435456i32);
pub const XCN_CERT_NAME_STR_REVERSE_FLAG: X500NameFlags = X500NameFlags(33554432i32);
pub const XCN_CERT_NAME_STR_SEMICOLON_FLAG: X500NameFlags = X500NameFlags(1073741824i32);
pub const XCN_CERT_NCRYPT_KEY_HANDLE_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(78i32);
pub const XCN_CERT_NCRYPT_KEY_HANDLE_TRANSFER_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(99i32);
pub const XCN_CERT_NEW_KEY_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(74i32);
pub const XCN_CERT_NEXT_UPDATE_LOCATION_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(10i32);
pub const XCN_CERT_NONCOMPLIANT_ROOT_URL_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(123i32);
pub const XCN_CERT_NON_REPUDIATION_KEY_USAGE: X509KeyUsageFlags = X509KeyUsageFlags(64i32);
pub const XCN_CERT_NOT_BEFORE_FILETIME_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(126i32);
pub const XCN_CERT_NO_AUTO_EXPIRE_CHECK_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(77i32);
pub const XCN_CERT_NO_EXPIRE_NOTIFICATION_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(97i32);
pub const XCN_CERT_NO_KEY_USAGE: X509KeyUsageFlags = X509KeyUsageFlags(0i32);
pub const XCN_CERT_OCSP_CACHE_PREFIX_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(75i32);
pub const XCN_CERT_OCSP_RESPONSE_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(70i32);
pub const XCN_CERT_OFFLINE_CRL_SIGN_KEY_USAGE: X509KeyUsageFlags = X509KeyUsageFlags(2i32);
pub const XCN_CERT_OID_NAME_STR: X500NameFlags = X500NameFlags(2i32);
pub const XCN_CERT_PIN_SHA256_HASH_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(124i32);
pub const XCN_CERT_PUBKEY_ALG_PARA_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(22i32);
pub const XCN_CERT_PUBKEY_HASH_RESERVED_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(8i32);
pub const XCN_CERT_PUB_KEY_CNG_ALG_BIT_LENGTH_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(93i32);
pub const XCN_CERT_PVK_FILE_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(12i32);
pub const XCN_CERT_RENEWAL_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(64i32);
pub const XCN_CERT_REQUEST_ORIGINATOR_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(71i32);
pub const XCN_CERT_ROOT_PROGRAM_CERT_POLICIES_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(83i32);
pub const XCN_CERT_ROOT_PROGRAM_CHAIN_POLICIES_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(105i32);
pub const XCN_CERT_ROOT_PROGRAM_NAME_CONSTRAINTS_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(84i32);
pub const XCN_CERT_SCARD_PIN_ID_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(90i32);
pub const XCN_CERT_SCARD_PIN_INFO_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(91i32);
pub const XCN_CERT_SCEP_CA_CERT_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(111i32);
pub const XCN_CERT_SCEP_ENCRYPT_HASH_CNG_ALG_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(114i32);
pub const XCN_CERT_SCEP_FLAGS_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(115i32);
pub const XCN_CERT_SCEP_GUID_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(116i32);
pub const XCN_CERT_SCEP_NONCE_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(113i32);
pub const XCN_CERT_SCEP_RA_ENCRYPTION_CERT_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(110i32);
pub const XCN_CERT_SCEP_RA_SIGNATURE_CERT_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(109i32);
pub const XCN_CERT_SCEP_SERVER_CERTS_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(108i32);
pub const XCN_CERT_SCEP_SIGNER_CERT_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(112i32);
pub const XCN_CERT_SEND_AS_TRUSTED_ISSUER_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(102i32);
pub const XCN_CERT_SERIALIZABLE_KEY_CONTEXT_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(117i32);
pub const XCN_CERT_SERIAL_CHAIN_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(119i32);
pub const XCN_CERT_SHA1_HASH_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(3i32);
pub const XCN_CERT_SHA256_HASH_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(107i32);
pub const XCN_CERT_SIGNATURE_HASH_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(15i32);
pub const XCN_CERT_SIGN_HASH_CNG_ALG_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(89i32);
pub const XCN_CERT_SIMPLE_NAME_STR: X500NameFlags = X500NameFlags(1i32);
pub const XCN_CERT_SMART_CARD_DATA_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(16i32);
pub const XCN_CERT_SMART_CARD_READER_NON_REMOVABLE_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(106i32);
pub const XCN_CERT_SMART_CARD_READER_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(101i32);
pub const XCN_CERT_SMART_CARD_ROOT_INFO_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(76i32);
pub const XCN_CERT_SOURCE_LOCATION_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(72i32);
pub const XCN_CERT_SOURCE_URL_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(73i32);
pub const XCN_CERT_STORE_LOCALIZED_NAME_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(4096i32);
pub const XCN_CERT_SUBJECT_DISABLE_CRL_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(86i32);
pub const XCN_CERT_SUBJECT_INFO_ACCESS_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(80i32);
pub const XCN_CERT_SUBJECT_NAME_MD5_HASH_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(29i32);
pub const XCN_CERT_SUBJECT_OCSP_AUTHORITY_INFO_ACCESS_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(85i32);
pub const XCN_CERT_SUBJECT_PUBLIC_KEY_MD5_HASH_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(25i32);
pub const XCN_CERT_SUBJECT_PUB_KEY_BIT_LENGTH_PROP_ID: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(92i32);
pub const XCN_CERT_X500_NAME_STR: X500NameFlags = X500NameFlags(3i32);
pub const XCN_CERT_XML_NAME_STR: X500NameFlags = X500NameFlags(4i32);
pub const XCN_CRL_REASON_AA_COMPROMISE: CRLRevocationReason = CRLRevocationReason(10i32);
pub const XCN_CRL_REASON_AFFILIATION_CHANGED: CRLRevocationReason = CRLRevocationReason(3i32);
pub const XCN_CRL_REASON_CA_COMPROMISE: CRLRevocationReason = CRLRevocationReason(2i32);
pub const XCN_CRL_REASON_CERTIFICATE_HOLD: CRLRevocationReason = CRLRevocationReason(6i32);
pub const XCN_CRL_REASON_CESSATION_OF_OPERATION: CRLRevocationReason = CRLRevocationReason(5i32);
pub const XCN_CRL_REASON_KEY_COMPROMISE: CRLRevocationReason = CRLRevocationReason(1i32);
pub const XCN_CRL_REASON_PRIVILEGE_WITHDRAWN: CRLRevocationReason = CRLRevocationReason(9i32);
pub const XCN_CRL_REASON_REMOVE_FROM_CRL: CRLRevocationReason = CRLRevocationReason(8i32);
pub const XCN_CRL_REASON_SUPERSEDED: CRLRevocationReason = CRLRevocationReason(4i32);
pub const XCN_CRL_REASON_UNSPECIFIED: CRLRevocationReason = CRLRevocationReason(0i32);
pub const XCN_CRYPT_ANY_GROUP_ID: ObjectIdGroupId = ObjectIdGroupId(0i32);
pub const XCN_CRYPT_ENCRYPT_ALG_OID_GROUP_ID: ObjectIdGroupId = ObjectIdGroupId(2i32);
pub const XCN_CRYPT_ENHKEY_USAGE_OID_GROUP_ID: ObjectIdGroupId = ObjectIdGroupId(7i32);
pub const XCN_CRYPT_EXT_OR_ATTR_OID_GROUP_ID: ObjectIdGroupId = ObjectIdGroupId(6i32);
pub const XCN_CRYPT_FIRST_ALG_OID_GROUP_ID: ObjectIdGroupId = ObjectIdGroupId(1i32);
pub const XCN_CRYPT_GROUP_ID_MASK: ObjectIdGroupId = ObjectIdGroupId(65535i32);
pub const XCN_CRYPT_HASH_ALG_OID_GROUP_ID: ObjectIdGroupId = ObjectIdGroupId(1i32);
pub const XCN_CRYPT_KDF_OID_GROUP_ID: ObjectIdGroupId = ObjectIdGroupId(10i32);
pub const XCN_CRYPT_KEY_LENGTH_MASK: ObjectIdGroupId = ObjectIdGroupId(268369920i32);
pub const XCN_CRYPT_LAST_ALG_OID_GROUP_ID: ObjectIdGroupId = ObjectIdGroupId(4i32);
pub const XCN_CRYPT_LAST_OID_GROUP_ID: ObjectIdGroupId = ObjectIdGroupId(10i32);
pub const XCN_CRYPT_OID_DISABLE_SEARCH_DS_FLAG: ObjectIdGroupId = ObjectIdGroupId(-2147483648i32);
pub const XCN_CRYPT_OID_INFO_OID_GROUP_BIT_LEN_MASK: ObjectIdGroupId = ObjectIdGroupId(268369920i32);
pub const XCN_CRYPT_OID_INFO_OID_GROUP_BIT_LEN_SHIFT: ObjectIdGroupId = ObjectIdGroupId(16i32);
pub const XCN_CRYPT_OID_INFO_PUBKEY_ANY: ObjectIdPublicKeyFlags = ObjectIdPublicKeyFlags(0i32);
pub const XCN_CRYPT_OID_INFO_PUBKEY_ENCRYPT_KEY_FLAG: ObjectIdPublicKeyFlags = ObjectIdPublicKeyFlags(1073741824i32);
pub const XCN_CRYPT_OID_INFO_PUBKEY_SIGN_KEY_FLAG: ObjectIdPublicKeyFlags = ObjectIdPublicKeyFlags(-2147483648i32);
pub const XCN_CRYPT_OID_PREFER_CNG_ALGID_FLAG: ObjectIdGroupId = ObjectIdGroupId(1073741824i32);
pub const XCN_CRYPT_OID_USE_CURVE_NAME_FOR_ENCODE_FLAG: X509KeyParametersExportType = X509KeyParametersExportType(536870912i32);
pub const XCN_CRYPT_OID_USE_CURVE_NONE: X509KeyParametersExportType = X509KeyParametersExportType(0i32);
pub const XCN_CRYPT_OID_USE_CURVE_PARAMETERS_FOR_ENCODE_FLAG: X509KeyParametersExportType = X509KeyParametersExportType(268435456i32);
pub const XCN_CRYPT_POLICY_OID_GROUP_ID: ObjectIdGroupId = ObjectIdGroupId(8i32);
pub const XCN_CRYPT_PUBKEY_ALG_OID_GROUP_ID: ObjectIdGroupId = ObjectIdGroupId(3i32);
pub const XCN_CRYPT_RDN_ATTR_OID_GROUP_ID: ObjectIdGroupId = ObjectIdGroupId(5i32);
pub const XCN_CRYPT_SIGN_ALG_OID_GROUP_ID: ObjectIdGroupId = ObjectIdGroupId(4i32);
pub const XCN_CRYPT_STRING_ANY: EncodingType = EncodingType(7i32);
pub const XCN_CRYPT_STRING_BASE64: EncodingType = EncodingType(1i32);
pub const XCN_CRYPT_STRING_BASE64HEADER: EncodingType = EncodingType(0i32);
pub const XCN_CRYPT_STRING_BASE64REQUESTHEADER: EncodingType = EncodingType(3i32);
pub const XCN_CRYPT_STRING_BASE64URI: EncodingType = EncodingType(13i32);
pub const XCN_CRYPT_STRING_BASE64X509CRLHEADER: EncodingType = EncodingType(9i32);
pub const XCN_CRYPT_STRING_BASE64_ANY: EncodingType = EncodingType(6i32);
pub const XCN_CRYPT_STRING_BINARY: EncodingType = EncodingType(2i32);
pub const XCN_CRYPT_STRING_CHAIN: EncodingType = EncodingType(256i32);
pub const XCN_CRYPT_STRING_ENCODEMASK: EncodingType = EncodingType(255i32);
pub const XCN_CRYPT_STRING_HASHDATA: EncodingType = EncodingType(268435456i32);
pub const XCN_CRYPT_STRING_HEX: EncodingType = EncodingType(4i32);
pub const XCN_CRYPT_STRING_HEXADDR: EncodingType = EncodingType(10i32);
pub const XCN_CRYPT_STRING_HEXASCII: EncodingType = EncodingType(5i32);
pub const XCN_CRYPT_STRING_HEXASCIIADDR: EncodingType = EncodingType(11i32);
pub const XCN_CRYPT_STRING_HEXRAW: EncodingType = EncodingType(12i32);
pub const XCN_CRYPT_STRING_HEX_ANY: EncodingType = EncodingType(8i32);
pub const XCN_CRYPT_STRING_NOCR: EncodingType = EncodingType(-2147483648i32);
pub const XCN_CRYPT_STRING_NOCRLF: EncodingType = EncodingType(1073741824i32);
pub const XCN_CRYPT_STRING_PERCENTESCAPE: EncodingType = EncodingType(134217728i32);
pub const XCN_CRYPT_STRING_STRICT: EncodingType = EncodingType(536870912i32);
pub const XCN_CRYPT_STRING_TEXT: EncodingType = EncodingType(512i32);
pub const XCN_CRYPT_TEMPLATE_OID_GROUP_ID: ObjectIdGroupId = ObjectIdGroupId(9i32);
pub const XCN_NCRYPT_ALLOW_ALL_USAGES: X509PrivateKeyUsageFlags = X509PrivateKeyUsageFlags(16777215i32);
pub const XCN_NCRYPT_ALLOW_ARCHIVING_FLAG: X509PrivateKeyExportFlags = X509PrivateKeyExportFlags(4i32);
pub const XCN_NCRYPT_ALLOW_DECRYPT_FLAG: X509PrivateKeyUsageFlags = X509PrivateKeyUsageFlags(1i32);
pub const XCN_NCRYPT_ALLOW_EXPORT_FLAG: X509PrivateKeyExportFlags = X509PrivateKeyExportFlags(1i32);
pub const XCN_NCRYPT_ALLOW_EXPORT_NONE: X509PrivateKeyExportFlags = X509PrivateKeyExportFlags(0i32);
pub const XCN_NCRYPT_ALLOW_KEY_AGREEMENT_FLAG: X509PrivateKeyUsageFlags = X509PrivateKeyUsageFlags(4i32);
pub const XCN_NCRYPT_ALLOW_KEY_IMPORT_FLAG: X509PrivateKeyUsageFlags = X509PrivateKeyUsageFlags(8i32);
pub const XCN_NCRYPT_ALLOW_PLAINTEXT_ARCHIVING_FLAG: X509PrivateKeyExportFlags = X509PrivateKeyExportFlags(8i32);
pub const XCN_NCRYPT_ALLOW_PLAINTEXT_EXPORT_FLAG: X509PrivateKeyExportFlags = X509PrivateKeyExportFlags(2i32);
pub const XCN_NCRYPT_ALLOW_SIGNING_FLAG: X509PrivateKeyUsageFlags = X509PrivateKeyUsageFlags(2i32);
pub const XCN_NCRYPT_ALLOW_USAGES_NONE: X509PrivateKeyUsageFlags = X509PrivateKeyUsageFlags(0i32);
pub const XCN_NCRYPT_ANY_ASYMMETRIC_OPERATION: AlgorithmOperationFlags = AlgorithmOperationFlags(28i32);
pub const XCN_NCRYPT_ASYMMETRIC_ENCRYPTION_OPERATION: AlgorithmOperationFlags = AlgorithmOperationFlags(4i32);
pub const XCN_NCRYPT_CIPHER_OPERATION: AlgorithmOperationFlags = AlgorithmOperationFlags(1i32);
pub const XCN_NCRYPT_CLAIM_AUTHORITY_AND_SUBJECT: KeyAttestationClaimType = KeyAttestationClaimType(3i32);
pub const XCN_NCRYPT_CLAIM_AUTHORITY_ONLY: KeyAttestationClaimType = KeyAttestationClaimType(1i32);
pub const XCN_NCRYPT_CLAIM_NONE: KeyAttestationClaimType = KeyAttestationClaimType(0i32);
pub const XCN_NCRYPT_CLAIM_SUBJECT_ONLY: KeyAttestationClaimType = KeyAttestationClaimType(2i32);
pub const XCN_NCRYPT_CLAIM_UNKNOWN: KeyAttestationClaimType = KeyAttestationClaimType(4096i32);
pub const XCN_NCRYPT_EXACT_MATCH_OPERATION: AlgorithmOperationFlags = AlgorithmOperationFlags(8388608i32);
pub const XCN_NCRYPT_HASH_OPERATION: AlgorithmOperationFlags = AlgorithmOperationFlags(2i32);
pub const XCN_NCRYPT_KEY_DERIVATION_OPERATION: AlgorithmOperationFlags = AlgorithmOperationFlags(64i32);
pub const XCN_NCRYPT_NO_OPERATION: AlgorithmOperationFlags = AlgorithmOperationFlags(0i32);
pub const XCN_NCRYPT_PCP_ENCRYPTION_KEY: X509HardwareKeyUsageFlags = X509HardwareKeyUsageFlags(2i32);
pub const XCN_NCRYPT_PCP_GENERIC_KEY: X509HardwareKeyUsageFlags = X509HardwareKeyUsageFlags(3i32);
pub const XCN_NCRYPT_PCP_IDENTITY_KEY: X509HardwareKeyUsageFlags = X509HardwareKeyUsageFlags(8i32);
pub const XCN_NCRYPT_PCP_NONE: X509HardwareKeyUsageFlags = X509HardwareKeyUsageFlags(0i32);
pub const XCN_NCRYPT_PCP_SIGNATURE_KEY: X509HardwareKeyUsageFlags = X509HardwareKeyUsageFlags(1i32);
pub const XCN_NCRYPT_PCP_STORAGE_KEY: X509HardwareKeyUsageFlags = X509HardwareKeyUsageFlags(4i32);
pub const XCN_NCRYPT_PREFERENCE_MASK_OPERATION: AlgorithmOperationFlags = AlgorithmOperationFlags(14680064i32);
pub const XCN_NCRYPT_PREFER_NON_SIGNATURE_OPERATION: AlgorithmOperationFlags = AlgorithmOperationFlags(4194304i32);
pub const XCN_NCRYPT_PREFER_SIGNATURE_ONLY_OPERATION: AlgorithmOperationFlags = AlgorithmOperationFlags(2097152i32);
pub const XCN_NCRYPT_RNG_OPERATION: AlgorithmOperationFlags = AlgorithmOperationFlags(32i32);
pub const XCN_NCRYPT_SECRET_AGREEMENT_OPERATION: AlgorithmOperationFlags = AlgorithmOperationFlags(8i32);
pub const XCN_NCRYPT_SIGNATURE_OPERATION: AlgorithmOperationFlags = AlgorithmOperationFlags(16i32);
pub const XCN_NCRYPT_TPM12_PROVIDER: X509HardwareKeyUsageFlags = X509HardwareKeyUsageFlags(65536i32);
pub const XCN_NCRYPT_UI_APPCONTAINER_ACCESS_MEDIUM_FLAG: X509PrivateKeyProtection = X509PrivateKeyProtection(8i32);
pub const XCN_NCRYPT_UI_FINGERPRINT_PROTECTION_FLAG: X509PrivateKeyProtection = X509PrivateKeyProtection(4i32);
pub const XCN_NCRYPT_UI_FORCE_HIGH_PROTECTION_FLAG: X509PrivateKeyProtection = X509PrivateKeyProtection(2i32);
pub const XCN_NCRYPT_UI_NO_PROTECTION_FLAG: X509PrivateKeyProtection = X509PrivateKeyProtection(0i32);
pub const XCN_NCRYPT_UI_PROTECT_KEY_FLAG: X509PrivateKeyProtection = X509PrivateKeyProtection(1i32);
pub const XCN_OIDVerisign_FailInfo: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(431i32);
pub const XCN_OIDVerisign_MessageType: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(429i32);
pub const XCN_OIDVerisign_PkiStatus: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(430i32);
pub const XCN_OIDVerisign_RecipientNonce: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(433i32);
pub const XCN_OIDVerisign_SenderNonce: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(432i32);
pub const XCN_OIDVerisign_TransactionID: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(434i32);
pub const XCN_OID_ANSI_X942: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(53i32);
pub const XCN_OID_ANSI_X942_DH: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(54i32);
pub const XCN_OID_ANY_APPLICATION_POLICY: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(216i32);
pub const XCN_OID_ANY_CERT_POLICY: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(180i32);
pub const XCN_OID_ANY_ENHANCED_KEY_USAGE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(352i32);
pub const XCN_OID_APPLICATION_CERT_POLICIES: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(229i32);
pub const XCN_OID_APPLICATION_POLICY_CONSTRAINTS: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(231i32);
pub const XCN_OID_APPLICATION_POLICY_MAPPINGS: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(230i32);
pub const XCN_OID_ARCHIVED_KEY_ATTR: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(232i32);
pub const XCN_OID_ARCHIVED_KEY_CERT_HASH: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(235i32);
pub const XCN_OID_ATTR_SUPPORTED_ALGORITHMS: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(355i32);
pub const XCN_OID_ATTR_TPM_SECURITY_ASSERTIONS: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(356i32);
pub const XCN_OID_ATTR_TPM_SPECIFICATION: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(357i32);
pub const XCN_OID_AUTHORITY_INFO_ACCESS: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(204i32);
pub const XCN_OID_AUTHORITY_KEY_IDENTIFIER: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(169i32);
pub const XCN_OID_AUTHORITY_KEY_IDENTIFIER2: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(181i32);
pub const XCN_OID_AUTHORITY_REVOCATION_LIST: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(156i32);
pub const XCN_OID_AUTO_ENROLL_CTL_USAGE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(217i32);
pub const XCN_OID_BACKGROUND_OTHER_LOGOTYPE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(327i32);
pub const XCN_OID_BASIC_CONSTRAINTS: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(175i32);
pub const XCN_OID_BASIC_CONSTRAINTS2: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(178i32);
pub const XCN_OID_BIOMETRIC_EXT: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(205i32);
pub const XCN_OID_BUSINESS_CATEGORY: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(133i32);
pub const XCN_OID_CA_CERTIFICATE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(155i32);
pub const XCN_OID_CERTIFICATE_REVOCATION_LIST: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(157i32);
pub const XCN_OID_CERTIFICATE_TEMPLATE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(226i32);
pub const XCN_OID_CERTSRV_CA_VERSION: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(220i32);
pub const XCN_OID_CERTSRV_CROSSCA_VERSION: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(240i32);
pub const XCN_OID_CERTSRV_PREVIOUS_CERT_HASH: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(221i32);
pub const XCN_OID_CERT_DISALLOWED_FILETIME_PROP_ID: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(358i32);
pub const XCN_OID_CERT_EXTENSIONS: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(207i32);
pub const XCN_OID_CERT_ISSUER_SERIAL_NUMBER_MD5_HASH_PROP_ID: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(339i32);
pub const XCN_OID_CERT_KEY_IDENTIFIER_PROP_ID: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(338i32);
pub const XCN_OID_CERT_MANIFOLD: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(219i32);
pub const XCN_OID_CERT_MD5_HASH_PROP_ID: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(341i32);
pub const XCN_OID_CERT_POLICIES: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(179i32);
pub const XCN_OID_CERT_POLICIES_95: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(171i32);
pub const XCN_OID_CERT_POLICIES_95_QUALIFIER1: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(281i32);
pub const XCN_OID_CERT_PROP_ID_PREFIX: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(337i32);
pub const XCN_OID_CERT_SIGNATURE_HASH_PROP_ID: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(359i32);
pub const XCN_OID_CERT_STRONG_KEY_OS_1: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(360i32);
pub const XCN_OID_CERT_STRONG_KEY_OS_CURRENT: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(361i32);
pub const XCN_OID_CERT_STRONG_KEY_OS_PREFIX: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(362i32);
pub const XCN_OID_CERT_STRONG_SIGN_OS_1: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(363i32);
pub const XCN_OID_CERT_STRONG_SIGN_OS_CURRENT: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(364i32);
pub const XCN_OID_CERT_STRONG_SIGN_OS_PREFIX: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(365i32);
pub const XCN_OID_CERT_SUBJECT_NAME_MD5_HASH_PROP_ID: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(340i32);
pub const XCN_OID_CMC: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(304i32);
pub const XCN_OID_CMC_ADD_ATTRIBUTES: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(325i32);
pub const XCN_OID_CMC_ADD_EXTENSIONS: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(312i32);
pub const XCN_OID_CMC_DATA_RETURN: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(308i32);
pub const XCN_OID_CMC_DECRYPTED_POP: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(314i32);
pub const XCN_OID_CMC_ENCRYPTED_POP: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(313i32);
pub const XCN_OID_CMC_GET_CERT: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(316i32);
pub const XCN_OID_CMC_GET_CRL: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(317i32);
pub const XCN_OID_CMC_IDENTIFICATION: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(306i32);
pub const XCN_OID_CMC_IDENTITY_PROOF: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(307i32);
pub const XCN_OID_CMC_ID_CONFIRM_CERT_ACCEPTANCE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(324i32);
pub const XCN_OID_CMC_ID_POP_LINK_RANDOM: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(322i32);
pub const XCN_OID_CMC_ID_POP_LINK_WITNESS: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(323i32);
pub const XCN_OID_CMC_LRA_POP_WITNESS: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(315i32);
pub const XCN_OID_CMC_QUERY_PENDING: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(321i32);
pub const XCN_OID_CMC_RECIPIENT_NONCE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(311i32);
pub const XCN_OID_CMC_REG_INFO: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(319i32);
pub const XCN_OID_CMC_RESPONSE_INFO: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(320i32);
pub const XCN_OID_CMC_REVOKE_REQUEST: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(318i32);
pub const XCN_OID_CMC_SENDER_NONCE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(310i32);
pub const XCN_OID_CMC_STATUS_INFO: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(305i32);
pub const XCN_OID_CMC_TRANSACTION_ID: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(309i32);
pub const XCN_OID_COMMON_NAME: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(121i32);
pub const XCN_OID_COUNTRY_NAME: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(124i32);
pub const XCN_OID_CRL_DIST_POINTS: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(187i32);
pub const XCN_OID_CRL_NEXT_PUBLISH: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(223i32);
pub const XCN_OID_CRL_NUMBER: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(189i32);
pub const XCN_OID_CRL_REASON_CODE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(185i32);
pub const XCN_OID_CRL_SELF_CDP: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(233i32);
pub const XCN_OID_CRL_VIRTUAL_BASE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(222i32);
pub const XCN_OID_CROSS_CERTIFICATE_PAIR: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(158i32);
pub const XCN_OID_CROSS_CERT_DIST_POINTS: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(210i32);
pub const XCN_OID_CTL: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(211i32);
pub const XCN_OID_CT_PKI_DATA: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(301i32);
pub const XCN_OID_CT_PKI_RESPONSE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(302i32);
pub const XCN_OID_DELTA_CRL_INDICATOR: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(190i32);
pub const XCN_OID_DESCRIPTION: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(131i32);
pub const XCN_OID_DESTINATION_INDICATOR: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(145i32);
pub const XCN_OID_DEVICE_SERIAL_NUMBER: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(123i32);
pub const XCN_OID_DH_SINGLE_PASS_STDDH_SHA1_KDF: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(366i32);
pub const XCN_OID_DH_SINGLE_PASS_STDDH_SHA256_KDF: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(367i32);
pub const XCN_OID_DH_SINGLE_PASS_STDDH_SHA384_KDF: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(368i32);
pub const XCN_OID_DISALLOWED_HASH: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(369i32);
pub const XCN_OID_DISALLOWED_LIST: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(370i32);
pub const XCN_OID_DN_QUALIFIER: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(161i32);
pub const XCN_OID_DOMAIN_COMPONENT: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(162i32);
pub const XCN_OID_DRM: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(273i32);
pub const XCN_OID_DRM_INDIVIDUALIZATION: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(274i32);
pub const XCN_OID_DS: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(58i32);
pub const XCN_OID_DSALG: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(59i32);
pub const XCN_OID_DSALG_CRPT: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(60i32);
pub const XCN_OID_DSALG_HASH: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(61i32);
pub const XCN_OID_DSALG_RSA: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(63i32);
pub const XCN_OID_DSALG_SIGN: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(62i32);
pub const XCN_OID_DS_EMAIL_REPLICATION: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(237i32);
pub const XCN_OID_ECC_CURVE_P256: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(371i32);
pub const XCN_OID_ECC_CURVE_P384: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(372i32);
pub const XCN_OID_ECC_CURVE_P521: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(373i32);
pub const XCN_OID_ECC_PUBLIC_KEY: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(349i32);
pub const XCN_OID_ECDSA_SHA1: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(350i32);
pub const XCN_OID_ECDSA_SHA256: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(374i32);
pub const XCN_OID_ECDSA_SHA384: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(375i32);
pub const XCN_OID_ECDSA_SHA512: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(376i32);
pub const XCN_OID_ECDSA_SPECIFIED: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(351i32);
pub const XCN_OID_EFS_RECOVERY: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(260i32);
pub const XCN_OID_EMBEDDED_NT_CRYPTO: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(264i32);
pub const XCN_OID_ENCRYPTED_KEY_HASH: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(239i32);
pub const XCN_OID_ENHANCED_KEY_USAGE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(188i32);
pub const XCN_OID_ENROLLMENT_AGENT: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(201i32);
pub const XCN_OID_ENROLLMENT_CSP_PROVIDER: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(199i32);
pub const XCN_OID_ENROLLMENT_NAME_VALUE_PAIR: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(198i32);
pub const XCN_OID_ENROLL_ATTESTATION_CHALLENGE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(435i32);
pub const XCN_OID_ENROLL_ATTESTATION_STATEMENT: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(436i32);
pub const XCN_OID_ENROLL_CAXCHGCERT_HASH: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(377i32);
pub const XCN_OID_ENROLL_CERTTYPE_EXTENSION: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(218i32);
pub const XCN_OID_ENROLL_EKPUB_CHALLENGE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(379i32);
pub const XCN_OID_ENROLL_EKVERIFYCERT: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(380i32);
pub const XCN_OID_ENROLL_EKVERIFYCREDS: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(381i32);
pub const XCN_OID_ENROLL_EKVERIFYKEY: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(382i32);
pub const XCN_OID_ENROLL_EK_INFO: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(378i32);
pub const XCN_OID_ENROLL_ENCRYPTION_ALGORITHM: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(437i32);
pub const XCN_OID_ENROLL_KSP_NAME: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(438i32);
pub const XCN_OID_ENROLL_SCEP_ERROR: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(428i32);
pub const XCN_OID_ENTERPRISE_OID_ROOT: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(227i32);
pub const XCN_OID_EV_RDN_COUNTRY: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(383i32);
pub const XCN_OID_EV_RDN_LOCALE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(384i32);
pub const XCN_OID_EV_RDN_STATE_OR_PROVINCE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(385i32);
pub const XCN_OID_FACSIMILE_TELEPHONE_NUMBER: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(141i32);
pub const XCN_OID_FRESHEST_CRL: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(192i32);
pub const XCN_OID_GIVEN_NAME: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(159i32);
pub const XCN_OID_INFOSEC: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(99i32);
pub const XCN_OID_INFOSEC_SuiteAConfidentiality: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(113i32);
pub const XCN_OID_INFOSEC_SuiteAIntegrity: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(114i32);
pub const XCN_OID_INFOSEC_SuiteAKMandSig: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(117i32);
pub const XCN_OID_INFOSEC_SuiteAKeyManagement: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(116i32);
pub const XCN_OID_INFOSEC_SuiteASignature: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(112i32);
pub const XCN_OID_INFOSEC_SuiteATokenProtection: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(115i32);
pub const XCN_OID_INFOSEC_mosaicConfidentiality: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(103i32);
pub const XCN_OID_INFOSEC_mosaicIntegrity: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(105i32);
pub const XCN_OID_INFOSEC_mosaicKMandSig: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(111i32);
pub const XCN_OID_INFOSEC_mosaicKMandUpdSig: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(119i32);
pub const XCN_OID_INFOSEC_mosaicKeyManagement: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(109i32);
pub const XCN_OID_INFOSEC_mosaicSignature: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(101i32);
pub const XCN_OID_INFOSEC_mosaicTokenProtection: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(107i32);
pub const XCN_OID_INFOSEC_mosaicUpdatedInteg: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(120i32);
pub const XCN_OID_INFOSEC_mosaicUpdatedSig: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(118i32);
pub const XCN_OID_INFOSEC_sdnsConfidentiality: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(102i32);
pub const XCN_OID_INFOSEC_sdnsIntegrity: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(104i32);
pub const XCN_OID_INFOSEC_sdnsKMandSig: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(110i32);
pub const XCN_OID_INFOSEC_sdnsKeyManagement: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(108i32);
pub const XCN_OID_INFOSEC_sdnsSignature: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(100i32);
pub const XCN_OID_INFOSEC_sdnsTokenProtection: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(106i32);
pub const XCN_OID_INHIBIT_ANY_POLICY: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(386i32);
pub const XCN_OID_INITIALS: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(160i32);
pub const XCN_OID_INTERNATIONALIZED_EMAIL_ADDRESS: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(387i32);
pub const XCN_OID_INTERNATIONAL_ISDN_NUMBER: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(143i32);
pub const XCN_OID_IPSEC_KP_IKE_INTERMEDIATE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(254i32);
pub const XCN_OID_ISSUED_CERT_HASH: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(236i32);
pub const XCN_OID_ISSUER_ALT_NAME: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(174i32);
pub const XCN_OID_ISSUER_ALT_NAME2: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(184i32);
pub const XCN_OID_ISSUING_DIST_POINT: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(191i32);
pub const XCN_OID_KEYID_RDN: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(168i32);
pub const XCN_OID_KEY_ATTRIBUTES: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(170i32);
pub const XCN_OID_KEY_USAGE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(176i32);
pub const XCN_OID_KEY_USAGE_RESTRICTION: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(172i32);
pub const XCN_OID_KP_CA_EXCHANGE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(224i32);
pub const XCN_OID_KP_CSP_SIGNATURE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(272i32);
pub const XCN_OID_KP_CTL_USAGE_SIGNING: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(255i32);
pub const XCN_OID_KP_DOCUMENT_SIGNING: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(268i32);
pub const XCN_OID_KP_EFS: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(259i32);
pub const XCN_OID_KP_KERNEL_MODE_CODE_SIGNING: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(388i32);
pub const XCN_OID_KP_KERNEL_MODE_HAL_EXTENSION_SIGNING: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(389i32);
pub const XCN_OID_KP_KERNEL_MODE_TRUSTED_BOOT_SIGNING: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(390i32);
pub const XCN_OID_KP_KEY_RECOVERY: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(267i32);
pub const XCN_OID_KP_KEY_RECOVERY_AGENT: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(225i32);
pub const XCN_OID_KP_LIFETIME_SIGNING: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(269i32);
pub const XCN_OID_KP_MOBILE_DEVICE_SOFTWARE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(270i32);
pub const XCN_OID_KP_QUALIFIED_SUBORDINATION: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(266i32);
pub const XCN_OID_KP_SMARTCARD_LOGON: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(277i32);
pub const XCN_OID_KP_SMART_DISPLAY: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(271i32);
pub const XCN_OID_KP_TIME_STAMP_SIGNING: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(256i32);
pub const XCN_OID_KP_TPM_AIK_CERTIFICATE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(391i32);
pub const XCN_OID_KP_TPM_EK_CERTIFICATE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(392i32);
pub const XCN_OID_KP_TPM_PLATFORM_CERTIFICATE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(393i32);
pub const XCN_OID_LEGACY_POLICY_MAPPINGS: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(195i32);
pub const XCN_OID_LICENSES: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(275i32);
pub const XCN_OID_LICENSE_SERVER: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(276i32);
pub const XCN_OID_LOCALITY_NAME: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(125i32);
pub const XCN_OID_LOCAL_MACHINE_KEYSET: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(166i32);
pub const XCN_OID_LOGOTYPE_EXT: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(206i32);
pub const XCN_OID_LOYALTY_OTHER_LOGOTYPE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(326i32);
pub const XCN_OID_MEMBER: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(149i32);
pub const XCN_OID_NAME_CONSTRAINTS: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(193i32);
pub const XCN_OID_NETSCAPE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(289i32);
pub const XCN_OID_NETSCAPE_BASE_URL: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(292i32);
pub const XCN_OID_NETSCAPE_CA_POLICY_URL: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(296i32);
pub const XCN_OID_NETSCAPE_CA_REVOCATION_URL: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(294i32);
pub const XCN_OID_NETSCAPE_CERT_EXTENSION: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(290i32);
pub const XCN_OID_NETSCAPE_CERT_RENEWAL_URL: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(295i32);
pub const XCN_OID_NETSCAPE_CERT_SEQUENCE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(300i32);
pub const XCN_OID_NETSCAPE_CERT_TYPE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(291i32);
pub const XCN_OID_NETSCAPE_COMMENT: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(298i32);
pub const XCN_OID_NETSCAPE_DATA_TYPE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(299i32);
pub const XCN_OID_NETSCAPE_REVOCATION_URL: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(293i32);
pub const XCN_OID_NETSCAPE_SSL_SERVER_NAME: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(297i32);
pub const XCN_OID_NEXT_UPDATE_LOCATION: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(208i32);
pub const XCN_OID_NIST_AES128_CBC: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(394i32);
pub const XCN_OID_NIST_AES128_WRAP: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(395i32);
pub const XCN_OID_NIST_AES192_CBC: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(396i32);
pub const XCN_OID_NIST_AES192_WRAP: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(397i32);
pub const XCN_OID_NIST_AES256_CBC: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(398i32);
pub const XCN_OID_NIST_AES256_WRAP: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(399i32);
pub const XCN_OID_NIST_sha256: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(345i32);
pub const XCN_OID_NIST_sha384: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(346i32);
pub const XCN_OID_NIST_sha512: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(347i32);
pub const XCN_OID_NONE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(0i32);
pub const XCN_OID_NT5_CRYPTO: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(262i32);
pub const XCN_OID_NTDS_REPLICATION: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(241i32);
pub const XCN_OID_NT_PRINCIPAL_NAME: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(214i32);
pub const XCN_OID_OEM_WHQL_CRYPTO: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(263i32);
pub const XCN_OID_OIW: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(64i32);
pub const XCN_OID_OIWDIR: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(93i32);
pub const XCN_OID_OIWDIR_CRPT: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(94i32);
pub const XCN_OID_OIWDIR_HASH: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(95i32);
pub const XCN_OID_OIWDIR_SIGN: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(96i32);
pub const XCN_OID_OIWDIR_md2: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(97i32);
pub const XCN_OID_OIWDIR_md2RSA: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(98i32);
pub const XCN_OID_OIWSEC: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(65i32);
pub const XCN_OID_OIWSEC_desCBC: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(70i32);
pub const XCN_OID_OIWSEC_desCFB: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(72i32);
pub const XCN_OID_OIWSEC_desECB: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(69i32);
pub const XCN_OID_OIWSEC_desEDE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(80i32);
pub const XCN_OID_OIWSEC_desMAC: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(73i32);
pub const XCN_OID_OIWSEC_desOFB: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(71i32);
pub const XCN_OID_OIWSEC_dhCommMod: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(79i32);
pub const XCN_OID_OIWSEC_dsa: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(75i32);
pub const XCN_OID_OIWSEC_dsaComm: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(83i32);
pub const XCN_OID_OIWSEC_dsaCommSHA: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(84i32);
pub const XCN_OID_OIWSEC_dsaCommSHA1: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(91i32);
pub const XCN_OID_OIWSEC_dsaSHA1: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(90i32);
pub const XCN_OID_OIWSEC_keyHashSeal: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(86i32);
pub const XCN_OID_OIWSEC_md2RSASign: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(87i32);
pub const XCN_OID_OIWSEC_md4RSA: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(66i32);
pub const XCN_OID_OIWSEC_md4RSA2: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(68i32);
pub const XCN_OID_OIWSEC_md5RSA: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(67i32);
pub const XCN_OID_OIWSEC_md5RSASign: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(88i32);
pub const XCN_OID_OIWSEC_mdc2: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(82i32);
pub const XCN_OID_OIWSEC_mdc2RSA: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(77i32);
pub const XCN_OID_OIWSEC_rsaSign: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(74i32);
pub const XCN_OID_OIWSEC_rsaXchg: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(85i32);
pub const XCN_OID_OIWSEC_sha: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(81i32);
pub const XCN_OID_OIWSEC_sha1: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(89i32);
pub const XCN_OID_OIWSEC_sha1RSASign: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(92i32);
pub const XCN_OID_OIWSEC_shaDSA: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(76i32);
pub const XCN_OID_OIWSEC_shaRSA: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(78i32);
pub const XCN_OID_ORGANIZATIONAL_UNIT_NAME: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(129i32);
pub const XCN_OID_ORGANIZATION_NAME: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(128i32);
pub const XCN_OID_OS_VERSION: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(200i32);
pub const XCN_OID_OWNER: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(150i32);
pub const XCN_OID_PHYSICAL_DELIVERY_OFFICE_NAME: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(137i32);
pub const XCN_OID_PKCS: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(2i32);
pub const XCN_OID_PKCS_1: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(5i32);
pub const XCN_OID_PKCS_10: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(14i32);
pub const XCN_OID_PKCS_12: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(15i32);
pub const XCN_OID_PKCS_12_EXTENDED_ATTRIBUTES: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(167i32);
pub const XCN_OID_PKCS_12_FRIENDLY_NAME_ATTR: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(163i32);
pub const XCN_OID_PKCS_12_KEY_PROVIDER_NAME_ATTR: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(165i32);
pub const XCN_OID_PKCS_12_LOCAL_KEY_ID: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(164i32);
pub const XCN_OID_PKCS_12_PROTECTED_PASSWORD_SECRET_BAG_TYPE_ID: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(407i32);
pub const XCN_OID_PKCS_12_PbeIds: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(400i32);
pub const XCN_OID_PKCS_12_pbeWithSHA1And128BitRC2: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(401i32);
pub const XCN_OID_PKCS_12_pbeWithSHA1And128BitRC4: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(402i32);
pub const XCN_OID_PKCS_12_pbeWithSHA1And2KeyTripleDES: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(403i32);
pub const XCN_OID_PKCS_12_pbeWithSHA1And3KeyTripleDES: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(404i32);
pub const XCN_OID_PKCS_12_pbeWithSHA1And40BitRC2: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(405i32);
pub const XCN_OID_PKCS_12_pbeWithSHA1And40BitRC4: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(406i32);
pub const XCN_OID_PKCS_2: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(6i32);
pub const XCN_OID_PKCS_3: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(7i32);
pub const XCN_OID_PKCS_4: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(8i32);
pub const XCN_OID_PKCS_5: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(9i32);
pub const XCN_OID_PKCS_6: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(10i32);
pub const XCN_OID_PKCS_7: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(11i32);
pub const XCN_OID_PKCS_7_DATA: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(329i32);
pub const XCN_OID_PKCS_7_DIGESTED: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(333i32);
pub const XCN_OID_PKCS_7_ENCRYPTED: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(334i32);
pub const XCN_OID_PKCS_7_ENVELOPED: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(331i32);
pub const XCN_OID_PKCS_7_SIGNED: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(330i32);
pub const XCN_OID_PKCS_7_SIGNEDANDENVELOPED: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(332i32);
pub const XCN_OID_PKCS_8: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(12i32);
pub const XCN_OID_PKCS_9: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(13i32);
pub const XCN_OID_PKCS_9_CONTENT_TYPE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(335i32);
pub const XCN_OID_PKCS_9_MESSAGE_DIGEST: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(336i32);
pub const XCN_OID_PKINIT_KP_KDC: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(408i32);
pub const XCN_OID_PKIX: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(202i32);
pub const XCN_OID_PKIX_ACC_DESCR: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(282i32);
pub const XCN_OID_PKIX_CA_ISSUERS: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(284i32);
pub const XCN_OID_PKIX_CA_REPOSITORY: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(409i32);
pub const XCN_OID_PKIX_KP: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(243i32);
pub const XCN_OID_PKIX_KP_CLIENT_AUTH: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(245i32);
pub const XCN_OID_PKIX_KP_CODE_SIGNING: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(246i32);
pub const XCN_OID_PKIX_KP_EMAIL_PROTECTION: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(247i32);
pub const XCN_OID_PKIX_KP_IPSEC_END_SYSTEM: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(248i32);
pub const XCN_OID_PKIX_KP_IPSEC_TUNNEL: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(249i32);
pub const XCN_OID_PKIX_KP_IPSEC_USER: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(250i32);
pub const XCN_OID_PKIX_KP_OCSP_SIGNING: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(252i32);
pub const XCN_OID_PKIX_KP_SERVER_AUTH: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(244i32);
pub const XCN_OID_PKIX_KP_TIMESTAMP_SIGNING: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(251i32);
pub const XCN_OID_PKIX_NO_SIGNATURE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(303i32);
pub const XCN_OID_PKIX_OCSP: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(283i32);
pub const XCN_OID_PKIX_OCSP_BASIC_SIGNED_RESPONSE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(328i32);
pub const XCN_OID_PKIX_OCSP_NOCHECK: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(253i32);
pub const XCN_OID_PKIX_OCSP_NONCE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(410i32);
pub const XCN_OID_PKIX_PE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(203i32);
pub const XCN_OID_PKIX_POLICY_QUALIFIER_CPS: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(279i32);
pub const XCN_OID_PKIX_POLICY_QUALIFIER_USERNOTICE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(280i32);
pub const XCN_OID_PKIX_TIME_STAMPING: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(411i32);
pub const XCN_OID_POLICY_CONSTRAINTS: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(196i32);
pub const XCN_OID_POLICY_MAPPINGS: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(194i32);
pub const XCN_OID_POSTAL_ADDRESS: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(134i32);
pub const XCN_OID_POSTAL_CODE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(135i32);
pub const XCN_OID_POST_OFFICE_BOX: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(136i32);
pub const XCN_OID_PREFERRED_DELIVERY_METHOD: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(146i32);
pub const XCN_OID_PRESENTATION_ADDRESS: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(147i32);
pub const XCN_OID_PRIVATEKEY_USAGE_PERIOD: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(177i32);
pub const XCN_OID_PRODUCT_UPDATE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(215i32);
pub const XCN_OID_QC_EU_COMPLIANCE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(412i32);
pub const XCN_OID_QC_SSCD: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(413i32);
pub const XCN_OID_QC_STATEMENTS_EXT: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(414i32);
pub const XCN_OID_RDN_DUMMY_SIGNER: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(228i32);
pub const XCN_OID_RDN_TPM_MANUFACTURER: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(415i32);
pub const XCN_OID_RDN_TPM_MODEL: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(416i32);
pub const XCN_OID_RDN_TPM_VERSION: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(417i32);
pub const XCN_OID_REASON_CODE_HOLD: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(186i32);
pub const XCN_OID_REGISTERED_ADDRESS: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(144i32);
pub const XCN_OID_REMOVE_CERTIFICATE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(209i32);
pub const XCN_OID_RENEWAL_CERTIFICATE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(197i32);
pub const XCN_OID_REQUEST_CLIENT_INFO: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(238i32);
pub const XCN_OID_REQUIRE_CERT_CHAIN_POLICY: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(234i32);
pub const XCN_OID_REVOKED_LIST_SIGNER: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(418i32);
pub const XCN_OID_RFC3161_counterSign: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(419i32);
pub const XCN_OID_ROLE_OCCUPANT: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(151i32);
pub const XCN_OID_ROOT_LIST_SIGNER: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(265i32);
pub const XCN_OID_ROOT_PROGRAM_AUTO_UPDATE_CA_REVOCATION: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(420i32);
pub const XCN_OID_ROOT_PROGRAM_AUTO_UPDATE_END_REVOCATION: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(421i32);
pub const XCN_OID_ROOT_PROGRAM_FLAGS: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(422i32);
pub const XCN_OID_ROOT_PROGRAM_NO_OCSP_FAILOVER_TO_CRL: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(423i32);
pub const XCN_OID_RSA: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(1i32);
pub const XCN_OID_RSAES_OAEP: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(425i32);
pub const XCN_OID_RSA_DES_EDE3_CBC: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(51i32);
pub const XCN_OID_RSA_DH: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(22i32);
pub const XCN_OID_RSA_ENCRYPT: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(4i32);
pub const XCN_OID_RSA_HASH: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(3i32);
pub const XCN_OID_RSA_MD2: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(46i32);
pub const XCN_OID_RSA_MD2RSA: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(17i32);
pub const XCN_OID_RSA_MD4: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(47i32);
pub const XCN_OID_RSA_MD4RSA: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(18i32);
pub const XCN_OID_RSA_MD5: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(48i32);
pub const XCN_OID_RSA_MD5RSA: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(19i32);
pub const XCN_OID_RSA_MGF1: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(348i32);
pub const XCN_OID_RSA_PSPECIFIED: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(424i32);
pub const XCN_OID_RSA_RC2CBC: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(49i32);
pub const XCN_OID_RSA_RC4: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(50i32);
pub const XCN_OID_RSA_RC5_CBCPad: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(52i32);
pub const XCN_OID_RSA_RSA: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(16i32);
pub const XCN_OID_RSA_SETOAEP_RSA: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(21i32);
pub const XCN_OID_RSA_SHA1RSA: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(20i32);
pub const XCN_OID_RSA_SHA256RSA: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(342i32);
pub const XCN_OID_RSA_SHA384RSA: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(343i32);
pub const XCN_OID_RSA_SHA512RSA: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(344i32);
pub const XCN_OID_RSA_SMIMECapabilities: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(40i32);
pub const XCN_OID_RSA_SMIMEalg: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(42i32);
pub const XCN_OID_RSA_SMIMEalgCMS3DESwrap: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(44i32);
pub const XCN_OID_RSA_SMIMEalgCMSRC2wrap: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(45i32);
pub const XCN_OID_RSA_SMIMEalgESDH: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(43i32);
pub const XCN_OID_RSA_SSA_PSS: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(353i32);
pub const XCN_OID_RSA_certExtensions: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(39i32);
pub const XCN_OID_RSA_challengePwd: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(36i32);
pub const XCN_OID_RSA_contentType: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(32i32);
pub const XCN_OID_RSA_counterSign: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(35i32);
pub const XCN_OID_RSA_data: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(23i32);
pub const XCN_OID_RSA_digestedData: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(27i32);
pub const XCN_OID_RSA_emailAddr: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(30i32);
pub const XCN_OID_RSA_encryptedData: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(29i32);
pub const XCN_OID_RSA_envelopedData: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(25i32);
pub const XCN_OID_RSA_extCertAttrs: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(38i32);
pub const XCN_OID_RSA_hashedData: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(28i32);
pub const XCN_OID_RSA_messageDigest: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(33i32);
pub const XCN_OID_RSA_preferSignedData: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(41i32);
pub const XCN_OID_RSA_signEnvData: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(26i32);
pub const XCN_OID_RSA_signedData: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(24i32);
pub const XCN_OID_RSA_signingTime: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(34i32);
pub const XCN_OID_RSA_unstructAddr: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(37i32);
pub const XCN_OID_RSA_unstructName: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(31i32);
pub const XCN_OID_SEARCH_GUIDE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(132i32);
pub const XCN_OID_SEE_ALSO: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(152i32);
pub const XCN_OID_SERIALIZED: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(213i32);
pub const XCN_OID_SERVER_GATED_CRYPTO: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(257i32);
pub const XCN_OID_SGC_NETSCAPE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(258i32);
pub const XCN_OID_SORTED_CTL: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(212i32);
pub const XCN_OID_STATE_OR_PROVINCE_NAME: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(126i32);
pub const XCN_OID_STREET_ADDRESS: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(127i32);
pub const XCN_OID_SUBJECT_ALT_NAME: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(173i32);
pub const XCN_OID_SUBJECT_ALT_NAME2: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(183i32);
pub const XCN_OID_SUBJECT_DIR_ATTRS: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(242i32);
pub const XCN_OID_SUBJECT_INFO_ACCESS: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(426i32);
pub const XCN_OID_SUBJECT_KEY_IDENTIFIER: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(182i32);
pub const XCN_OID_SUPPORTED_APPLICATION_CONTEXT: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(148i32);
pub const XCN_OID_SUR_NAME: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(122i32);
pub const XCN_OID_TELEPHONE_NUMBER: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(138i32);
pub const XCN_OID_TELETEXT_TERMINAL_IDENTIFIER: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(140i32);
pub const XCN_OID_TELEX_NUMBER: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(139i32);
pub const XCN_OID_TIMESTAMP_TOKEN: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(427i32);
pub const XCN_OID_TITLE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(130i32);
pub const XCN_OID_USER_CERTIFICATE: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(154i32);
pub const XCN_OID_USER_PASSWORD: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(153i32);
pub const XCN_OID_VERISIGN_BITSTRING_6_13: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(287i32);
pub const XCN_OID_VERISIGN_ISS_STRONG_CRYPTO: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(288i32);
pub const XCN_OID_VERISIGN_ONSITE_JURISDICTION_HASH: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(286i32);
pub const XCN_OID_VERISIGN_PRIVATE_6_9: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(285i32);
pub const XCN_OID_WHQL_CRYPTO: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(261i32);
pub const XCN_OID_X21_ADDRESS: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(142i32);
pub const XCN_OID_X957: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(55i32);
pub const XCN_OID_X957_DSA: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(56i32);
pub const XCN_OID_X957_SHA1DSA: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(57i32);
pub const XCN_OID_YESNO_TRUST_ATTR: CERTENROLL_OBJECTID = CERTENROLL_OBJECTID(278i32);
pub const XCN_PROPERTYID_NONE: CERTENROLL_PROPERTYID = CERTENROLL_PROPERTYID(0i32);
pub const XCN_PROV_DH_SCHANNEL: X509ProviderType = X509ProviderType(18i32);
pub const XCN_PROV_DSS: X509ProviderType = X509ProviderType(3i32);
pub const XCN_PROV_DSS_DH: X509ProviderType = X509ProviderType(13i32);
pub const XCN_PROV_EC_ECDSA_FULL: X509ProviderType = X509ProviderType(16i32);
pub const XCN_PROV_EC_ECDSA_SIG: X509ProviderType = X509ProviderType(14i32);
pub const XCN_PROV_EC_ECNRA_FULL: X509ProviderType = X509ProviderType(17i32);
pub const XCN_PROV_EC_ECNRA_SIG: X509ProviderType = X509ProviderType(15i32);
pub const XCN_PROV_FORTEZZA: X509ProviderType = X509ProviderType(4i32);
pub const XCN_PROV_INTEL_SEC: X509ProviderType = X509ProviderType(22i32);
pub const XCN_PROV_MS_EXCHANGE: X509ProviderType = X509ProviderType(5i32);
pub const XCN_PROV_NONE: X509ProviderType = X509ProviderType(0i32);
pub const XCN_PROV_REPLACE_OWF: X509ProviderType = X509ProviderType(23i32);
pub const XCN_PROV_RNG: X509ProviderType = X509ProviderType(21i32);
pub const XCN_PROV_RSA_AES: X509ProviderType = X509ProviderType(24i32);
pub const XCN_PROV_RSA_FULL: X509ProviderType = X509ProviderType(1i32);
pub const XCN_PROV_RSA_SCHANNEL: X509ProviderType = X509ProviderType(12i32);
pub const XCN_PROV_RSA_SIG: X509ProviderType = X509ProviderType(2i32);
pub const XCN_PROV_SPYRUS_LYNKS: X509ProviderType = X509ProviderType(20i32);
pub const XCN_PROV_SSL: X509ProviderType = X509ProviderType(6i32);
pub const XECI_AUTOENROLL: u32 = 2u32;
pub const XECI_CERTREQ: u32 = 4u32;
pub const XECI_DISABLE: u32 = 0u32;
pub const XECI_REQWIZARD: u32 = 3u32;
pub const XECI_XENROLL: u32 = 1u32;
pub const XECP_STRING_PROPERTY: u32 = 1u32;
pub const XECR_CMC: CERT_CREATE_REQUEST_FLAGS = CERT_CREATE_REQUEST_FLAGS(3i32);
pub const XECR_PKCS10_V1_5: CERT_CREATE_REQUEST_FLAGS = CERT_CREATE_REQUEST_FLAGS(4i32);
pub const XECR_PKCS10_V2_0: CERT_CREATE_REQUEST_FLAGS = CERT_CREATE_REQUEST_FLAGS(1i32);
pub const XECR_PKCS7: CERT_CREATE_REQUEST_FLAGS = CERT_CREATE_REQUEST_FLAGS(2i32);
pub const XECT_EXTENSION_V1: ADDED_CERT_TYPE = ADDED_CERT_TYPE(1i32);
pub const XECT_EXTENSION_V2: ADDED_CERT_TYPE = ADDED_CERT_TYPE(2i32);
pub const XEKL_KEYSIZE_DEFAULT: u32 = 4u32;
pub const XEKL_KEYSIZE_INC: XEKL_KEYSIZE = XEKL_KEYSIZE(3i32);
pub const XEKL_KEYSIZE_MAX: XEKL_KEYSIZE = XEKL_KEYSIZE(2i32);
pub const XEKL_KEYSIZE_MIN: XEKL_KEYSIZE = XEKL_KEYSIZE(1i32);
pub const XEKL_KEYSPEC_KEYX: XEKL_KEYSPEC = XEKL_KEYSPEC(1i32);
pub const XEKL_KEYSPEC_SIG: XEKL_KEYSPEC = XEKL_KEYSPEC(2i32);
pub const XEPR_CADNS: PENDING_REQUEST_DESIRED_PROPERTY = PENDING_REQUEST_DESIRED_PROPERTY(1i32);
pub const XEPR_CAFRIENDLYNAME: PENDING_REQUEST_DESIRED_PROPERTY = PENDING_REQUEST_DESIRED_PROPERTY(3i32);
pub const XEPR_CANAME: PENDING_REQUEST_DESIRED_PROPERTY = PENDING_REQUEST_DESIRED_PROPERTY(2i32);
pub const XEPR_DATE: u32 = 5u32;
pub const XEPR_ENUM_FIRST: i32 = -1i32;
pub const XEPR_HASH: PENDING_REQUEST_DESIRED_PROPERTY = PENDING_REQUEST_DESIRED_PROPERTY(8i32);
pub const XEPR_REQUESTID: PENDING_REQUEST_DESIRED_PROPERTY = PENDING_REQUEST_DESIRED_PROPERTY(4i32);
pub const XEPR_TEMPLATENAME: u32 = 6u32;
pub const XEPR_V1TEMPLATENAME: u32 = 9u32;
pub const XEPR_V2TEMPLATEOID: u32 = 16u32;
pub const XEPR_VERSION: u32 = 7u32;
pub const dwCAXCHGOVERLAPPERIODCOUNTDEFAULT: u32 = 1u32;
pub const dwCAXCHGVALIDITYPERIODCOUNTDEFAULT: u32 = 1u32;
pub const dwCRLDELTAOVERLAPPERIODCOUNTDEFAULT: u32 = 0u32;
pub const dwCRLDELTAPERIODCOUNTDEFAULT: u32 = 1u32;
pub const dwCRLOVERLAPPERIODCOUNTDEFAULT: u32 = 0u32;
pub const dwCRLPERIODCOUNTDEFAULT: u32 = 1u32;
pub const dwVALIDITYPERIODCOUNTDEFAULT_ENTERPRISE: u32 = 2u32;
pub const dwVALIDITYPERIODCOUNTDEFAULT_ROOT: u32 = 5u32;
pub const dwVALIDITYPERIODCOUNTDEFAULT_STANDALONE: u32 = 1u32;
pub const szBACKUPANNOTATION: windows_core::PCSTR = windows_core::s!("Cert Server Backup Interface");
pub const szDBBASENAMEPARM: windows_core::PCSTR = windows_core::s!("edb");
pub const szNAMESEPARATORDEFAULT: windows_core::PCSTR = windows_core::s!("\n");
pub const szPROPASNTAG: windows_core::PCSTR = windows_core::s!("{asn}");
pub const szRESTOREANNOTATION: windows_core::PCSTR = windows_core::s!("Cert Server Restore Interface");
pub const wszAT_EKCERTINF: windows_core::PCWSTR = windows_core::w!("@EKCert");
pub const wszAT_TESTROOT: windows_core::PCWSTR = windows_core::w!("@TestRoot");
pub const wszCAPOLICYFILE: windows_core::PCWSTR = windows_core::w!("CAPolicy.inf");
pub const wszCERTEXITMODULE_POSTFIX: windows_core::PCWSTR = windows_core::w!(".Exit");
pub const wszCERTIFICATETRANSPARENCYFLAGS: windows_core::PCWSTR = windows_core::w!("CertificateTransparencyFlags");
pub const wszCERTMANAGE_SUFFIX: windows_core::PCWSTR = windows_core::w!("Manage");
pub const wszCERTPOLICYMODULE_POSTFIX: windows_core::PCWSTR = windows_core::w!(".Policy");
pub const wszCERT_TYPE: windows_core::PCWSTR = windows_core::w!("RequestType");
pub const wszCERT_TYPE_CLIENT: windows_core::PCWSTR = windows_core::w!("Client");
pub const wszCERT_TYPE_CODESIGN: windows_core::PCWSTR = windows_core::w!("CodeSign");
pub const wszCERT_TYPE_CUSTOMER: windows_core::PCWSTR = windows_core::w!("SetCustomer");
pub const wszCERT_TYPE_MERCHANT: windows_core::PCWSTR = windows_core::w!("SetMerchant");
pub const wszCERT_TYPE_PAYMENT: windows_core::PCWSTR = windows_core::w!("SetPayment");
pub const wszCERT_TYPE_SERVER: windows_core::PCWSTR = windows_core::w!("Server");
pub const wszCERT_VERSION: windows_core::PCWSTR = windows_core::w!("Version");
pub const wszCERT_VERSION_1: windows_core::PCWSTR = windows_core::w!("1");
pub const wszCERT_VERSION_2: windows_core::PCWSTR = windows_core::w!("2");
pub const wszCERT_VERSION_3: windows_core::PCWSTR = windows_core::w!("3");
pub const wszCLASS_CERTADMIN: windows_core::PCWSTR = windows_core::w!("CertificateAuthority.Admin");
pub const wszCLASS_CERTCONFIG: windows_core::PCWSTR = windows_core::w!("CertificateAuthority.Config");
pub const wszCLASS_CERTDBMEM: windows_core::PCWSTR = windows_core::w!("CertificateAuthority.DBMem");
pub const wszCLASS_CERTENCODE: windows_core::PCWSTR = windows_core::w!("CertificateAuthority.Encode");
pub const wszCLASS_CERTGETCONFIG: windows_core::PCWSTR = windows_core::w!("CertificateAuthority.GetConfig");
pub const wszCLASS_CERTREQUEST: windows_core::PCWSTR = windows_core::w!("CertificateAuthority.Request");
pub const wszCLASS_CERTSERVEREXIT: windows_core::PCWSTR = windows_core::w!("CertificateAuthority.ServerExit");
pub const wszCLASS_CERTSERVERPOLICY: windows_core::PCWSTR = windows_core::w!("CertificateAuthority.ServerPolicy");
pub const wszCLASS_CERTVIEW: windows_core::PCWSTR = windows_core::w!("CertificateAuthority.View");
pub const wszCMM_PROP_COPYRIGHT: windows_core::PCWSTR = windows_core::w!("Copyright");
pub const wszCMM_PROP_DESCRIPTION: windows_core::PCWSTR = windows_core::w!("Description");
pub const wszCMM_PROP_DISPLAY_HWND: windows_core::PCWSTR = windows_core::w!("HWND");
pub const wszCMM_PROP_FILEVER: windows_core::PCWSTR = windows_core::w!("File Version");
pub const wszCMM_PROP_ISMULTITHREADED: windows_core::PCWSTR = windows_core::w!("IsMultiThreaded");
pub const wszCMM_PROP_NAME: windows_core::PCWSTR = windows_core::w!("Name");
pub const wszCMM_PROP_PRODUCTVER: windows_core::PCWSTR = windows_core::w!("Product Version");
pub const wszCNGENCRYPTIONALGORITHM: windows_core::PCWSTR = windows_core::w!("CNGEncryptionAlgorithm");
pub const wszCNGHASHALGORITHM: windows_core::PCWSTR = windows_core::w!("CNGHashAlgorithm");
pub const wszCNGPUBLICKEYALGORITHM: windows_core::PCWSTR = windows_core::w!("CNGPublicKeyAlgorithm");
pub const wszCONFIG_AUTHORITY: windows_core::PCWSTR = windows_core::w!("Authority");
pub const wszCONFIG_COMMENT: windows_core::PCWSTR = windows_core::w!("Comment");
pub const wszCONFIG_COMMONNAME: windows_core::PCWSTR = windows_core::w!("CommonName");
pub const wszCONFIG_CONFIG: windows_core::PCWSTR = windows_core::w!("Config");
pub const wszCONFIG_COUNTRY: windows_core::PCWSTR = windows_core::w!("Country");
pub const wszCONFIG_DESCRIPTION: windows_core::PCWSTR = windows_core::w!("Description");
pub const wszCONFIG_EXCHANGECERTIFICATE: windows_core::PCWSTR = windows_core::w!("ExchangeCertificate");
pub const wszCONFIG_FLAGS: windows_core::PCWSTR = windows_core::w!("Flags");
pub const wszCONFIG_LOCALITY: windows_core::PCWSTR = windows_core::w!("Locality");
pub const wszCONFIG_ORGANIZATION: windows_core::PCWSTR = windows_core::w!("Organization");
pub const wszCONFIG_ORGUNIT: windows_core::PCWSTR = windows_core::w!("OrgUnit");
pub const wszCONFIG_SANITIZEDNAME: windows_core::PCWSTR = windows_core::w!("SanitizedName");
pub const wszCONFIG_SANITIZEDSHORTNAME: windows_core::PCWSTR = windows_core::w!("SanitizedShortName");
pub const wszCONFIG_SERVER: windows_core::PCWSTR = windows_core::w!("Server");
pub const wszCONFIG_SHORTNAME: windows_core::PCWSTR = windows_core::w!("ShortName");
pub const wszCONFIG_SIGNATURECERTIFICATE: windows_core::PCWSTR = windows_core::w!("SignatureCertificate");
pub const wszCONFIG_STATE: windows_core::PCWSTR = windows_core::w!("State");
pub const wszCONFIG_WEBENROLLMENTSERVERS: windows_core::PCWSTR = windows_core::w!("WebEnrollmentServers");
pub const wszCRLPUBLISHRETRYCOUNT: windows_core::PCWSTR = windows_core::w!("CRLPublishRetryCount");
pub const wszCRTFILENAMEEXT: windows_core::PCWSTR = windows_core::w!(".crt");
pub const wszDATFILENAMEEXT: windows_core::PCWSTR = windows_core::w!(".dat");
pub const wszDBBACKUPCERTBACKDAT: windows_core::PCWSTR = windows_core::w!("certbkxp.dat");
pub const wszDBBACKUPSUBDIR: windows_core::PCWSTR = windows_core::w!("DataBase");
pub const wszDBFILENAMEEXT: windows_core::PCWSTR = windows_core::w!(".edb");
pub const wszENCRYPTIONALGORITHM: windows_core::PCWSTR = windows_core::w!("EncryptionAlgorithm");
pub const wszENROLLMENTAGENTRIGHTS: windows_core::PCWSTR = windows_core::w!("EnrollmentAgentRights");
pub const wszHASHALGORITHM: windows_core::PCWSTR = windows_core::w!("HashAlgorithm");
pub const wszINFKEY_ALTERNATESIGNATUREALGORITHM: windows_core::PCWSTR = windows_core::w!("AlternateSignatureAlgorithm");
pub const wszINFKEY_ATTESTPRIVATEKEY: windows_core::PCWSTR = windows_core::w!("AttestPrivateKey");
pub const wszINFKEY_CACAPABILITIES: windows_core::PCWSTR = windows_core::w!("CACapabilities");
pub const wszINFKEY_CACERTS: windows_core::PCWSTR = windows_core::w!("CACerts");
pub const wszINFKEY_CATHUMBPRINT: windows_core::PCWSTR = windows_core::w!("CAThumbprint");
pub const wszINFKEY_CCDPSYNCDELTATIME: windows_core::PCWSTR = windows_core::w!("SyncDeltaTime");
pub const wszINFKEY_CHALLENGEPASSWORD: windows_core::PCWSTR = windows_core::w!("ChallengePassword");
pub const wszINFKEY_CONTINUE: windows_core::PCWSTR = windows_core::w!("_continue_");
pub const wszINFKEY_CRITICAL: windows_core::PCWSTR = windows_core::w!("Critical");
pub const wszINFKEY_CRLDELTAPERIODCOUNT: windows_core::PCWSTR = windows_core::w!("CRLDeltaPeriodUnits");
pub const wszINFKEY_CRLDELTAPERIODSTRING: windows_core::PCWSTR = windows_core::w!("CRLDeltaPeriod");
pub const wszINFKEY_CRLPERIODCOUNT: windows_core::PCWSTR = windows_core::w!("CRLPeriodUnits");
pub const wszINFKEY_CRLPERIODSTRING: windows_core::PCWSTR = windows_core::w!("CRLPeriod");
pub const wszINFKEY_DIRECTORYNAME: windows_core::PCWSTR = windows_core::w!("DirectoryName");
pub const wszINFKEY_DNS: windows_core::PCWSTR = windows_core::w!("DNS");
pub const wszINFKEY_ECCKEYPARAMETERS: windows_core::PCWSTR = windows_core::w!("EccKeyParameters");
pub const wszINFKEY_ECCKEYPARAMETERSTYPE: windows_core::PCWSTR = windows_core::w!("EccKeyParametersType");
pub const wszINFKEY_ECCKEYPARAMETERS_A: windows_core::PCWSTR = windows_core::w!("EccKeyParameters_A");
pub const wszINFKEY_ECCKEYPARAMETERS_B: windows_core::PCWSTR = windows_core::w!("EccKeyParameters_B");
pub const wszINFKEY_ECCKEYPARAMETERS_BASE: windows_core::PCWSTR = windows_core::w!("EccKeyParameters_Base");
pub const wszINFKEY_ECCKEYPARAMETERS_COFACTOR: windows_core::PCWSTR = windows_core::w!("EccKeyParameters_Cofactor");
pub const wszINFKEY_ECCKEYPARAMETERS_ORDER: windows_core::PCWSTR = windows_core::w!("EccKeyParameters_Order");
pub const wszINFKEY_ECCKEYPARAMETERS_P: windows_core::PCWSTR = windows_core::w!("EccKeyParameters_P");
pub const wszINFKEY_ECCKEYPARAMETERS_SEED: windows_core::PCWSTR = windows_core::w!("EccKeyParameters_Seed");
pub const wszINFKEY_EMAIL: windows_core::PCWSTR = windows_core::w!("EMail");
pub const wszINFKEY_EMPTY: windows_core::PCWSTR = windows_core::w!("Empty");
pub const wszINFKEY_ENABLEKEYCOUNTING: windows_core::PCWSTR = windows_core::w!("EnableKeyCounting");
pub const wszINFKEY_ENCRYPTIONALGORITHM: windows_core::PCWSTR = windows_core::w!("EncryptionAlgorithm");
pub const wszINFKEY_ENCRYPTIONLENGTH: windows_core::PCWSTR = windows_core::w!("EncryptionLength");
pub const wszINFKEY_EXCLUDE: windows_core::PCWSTR = windows_core::w!("Exclude");
pub const wszINFKEY_EXPORTABLE: windows_core::PCWSTR = windows_core::w!("Exportable");
pub const wszINFKEY_EXPORTABLEENCRYPTED: windows_core::PCWSTR = windows_core::w!("ExportableEncrypted");
pub const wszINFKEY_FLAGS: windows_core::PCWSTR = windows_core::w!("Flags");
pub const wszINFKEY_FORCEUTF8: windows_core::PCWSTR = windows_core::w!("ForceUTF8");
pub const wszINFKEY_FRIENDLYNAME: windows_core::PCWSTR = windows_core::w!("FriendlyName");
pub const wszINFKEY_HASHALGORITHM: windows_core::PCWSTR = windows_core::w!("HashAlgorithm");
pub const wszINFKEY_INCLUDE: windows_core::PCWSTR = windows_core::w!("Include");
pub const wszINFKEY_INHIBITPOLICYMAPPING: windows_core::PCWSTR = windows_core::w!("InhibitPolicyMapping");
pub const wszINFKEY_IPADDRESS: windows_core::PCWSTR = windows_core::w!("IPAddress");
pub const wszINFKEY_KEYALGORITHM: windows_core::PCWSTR = windows_core::w!("KeyAlgorithm");
pub const wszINFKEY_KEYALGORITHMPARMETERS: windows_core::PCWSTR = windows_core::w!("KeyAlgorithmParameters");
pub const wszINFKEY_KEYCONTAINER: windows_core::PCWSTR = windows_core::w!("KeyContainer");
pub const wszINFKEY_KEYLENGTH: windows_core::PCWSTR = windows_core::w!("KeyLength");
pub const wszINFKEY_KEYPROTECTION: windows_core::PCWSTR = windows_core::w!("KeyProtection");
pub const wszINFKEY_KEYUSAGEEXTENSION: windows_core::PCWSTR = windows_core::w!("KeyUsage");
pub const wszINFKEY_KEYUSAGEPROPERTY: windows_core::PCWSTR = windows_core::w!("KeyUsageProperty");
pub const wszINFKEY_LEGACYKEYSPEC: windows_core::PCWSTR = windows_core::w!("KeySpec");
pub const wszINFKEY_LOADDEFAULTTEMPLATES: windows_core::PCWSTR = windows_core::w!("LoadDefaultTemplates");
pub const wszINFKEY_MACHINEKEYSET: windows_core::PCWSTR = windows_core::w!("MachineKeySet");
pub const wszINFKEY_NOTAFTER: windows_core::PCWSTR = windows_core::w!("NotAfter");
pub const wszINFKEY_NOTBEFORE: windows_core::PCWSTR = windows_core::w!("NotBefore");
pub const wszINFKEY_NOTICE: windows_core::PCWSTR = windows_core::w!("Notice");
pub const wszINFKEY_OID: windows_core::PCWSTR = windows_core::w!("OID");
pub const wszINFKEY_OTHERNAME: windows_core::PCWSTR = windows_core::w!("OtherName");
pub const wszINFKEY_PATHLENGTH: windows_core::PCWSTR = windows_core::w!("PathLength");
pub const wszINFKEY_POLICIES: windows_core::PCWSTR = windows_core::w!("Policies");
pub const wszINFKEY_PRIVATEKEYARCHIVE: windows_core::PCWSTR = windows_core::w!("PrivateKeyArchive");
pub const wszINFKEY_PROVIDERNAME: windows_core::PCWSTR = windows_core::w!("ProviderName");
pub const wszINFKEY_PROVIDERTYPE: windows_core::PCWSTR = windows_core::w!("ProviderType");
pub const wszINFKEY_PUBLICKEY: windows_core::PCWSTR = windows_core::w!("PublicKey");
pub const wszINFKEY_PUBLICKEYPARAMETERS: windows_core::PCWSTR = windows_core::w!("PublicKeyParameters");
pub const wszINFKEY_READERNAME: windows_core::PCWSTR = windows_core::w!("ReaderName");
pub const wszINFKEY_REGISTEREDID: windows_core::PCWSTR = windows_core::w!("RegisteredId");
pub const wszINFKEY_RENEWALCERT: windows_core::PCWSTR = windows_core::w!("RenewalCert");
pub const wszINFKEY_RENEWALKEYLENGTH: windows_core::PCWSTR = windows_core::w!("RenewalKeyLength");
pub const wszINFKEY_RENEWALVALIDITYPERIODCOUNT: windows_core::PCWSTR = windows_core::w!("RenewalValidityPeriodUnits");
pub const wszINFKEY_RENEWALVALIDITYPERIODSTRING: windows_core::PCWSTR = windows_core::w!("RenewalValidityPeriod");
pub const wszINFKEY_REQUESTTYPE: windows_core::PCWSTR = windows_core::w!("RequestType");
pub const wszINFKEY_REQUIREEXPLICITPOLICY: windows_core::PCWSTR = windows_core::w!("RequireExplicitPolicy");
pub const wszINFKEY_SECURITYDESCRIPTOR: windows_core::PCWSTR = windows_core::w!("SecurityDescriptor");
pub const wszINFKEY_SERIALNUMBER: windows_core::PCWSTR = windows_core::w!("SerialNumber");
pub const wszINFKEY_SHOWALLCSPS: windows_core::PCWSTR = windows_core::w!("ShowAllCSPs");
pub const wszINFKEY_SILENT: windows_core::PCWSTR = windows_core::w!("Silent");
pub const wszINFKEY_SMIME: windows_core::PCWSTR = windows_core::w!("SMIME");
pub const wszINFKEY_SUBJECT: windows_core::PCWSTR = windows_core::w!("Subject");
pub const wszINFKEY_SUBJECTNAMEFLAGS: windows_core::PCWSTR = windows_core::w!("SubjectNameFlags");
pub const wszINFKEY_SUBTREE: windows_core::PCWSTR = windows_core::w!("SubTree");
pub const wszINFKEY_SUPPRESSDEFAULTS: windows_core::PCWSTR = windows_core::w!("SuppressDefaults");
pub const wszINFKEY_UICONTEXTMESSAGE: windows_core::PCWSTR = windows_core::w!("UIContextMessage");
pub const wszINFKEY_UPN: windows_core::PCWSTR = windows_core::w!("UPN");
pub const wszINFKEY_URL: windows_core::PCWSTR = windows_core::w!("URL");
pub const wszINFKEY_USEEXISTINGKEY: windows_core::PCWSTR = windows_core::w!("UseExistingKeySet");
pub const wszINFKEY_USERPROTECTED: windows_core::PCWSTR = windows_core::w!("UserProtected");
pub const wszINFKEY_UTF8: windows_core::PCWSTR = windows_core::w!("UTF8");
pub const wszINFKEY_X500NAMEFLAGS: windows_core::PCWSTR = windows_core::w!("X500NameFlags");
pub const wszINFSECTION_AIA: windows_core::PCWSTR = windows_core::w!("AuthorityInformationAccess");
pub const wszINFSECTION_APPLICATIONPOLICYCONSTRAINTS: windows_core::PCWSTR = windows_core::w!("ApplicationPolicyConstraintsExtension");
pub const wszINFSECTION_APPLICATIONPOLICYMAPPINGS: windows_core::PCWSTR = windows_core::w!("ApplicationPolicyMappingsExtension");
pub const wszINFSECTION_APPLICATIONPOLICYSTATEMENT: windows_core::PCWSTR = windows_core::w!("ApplicationPolicyStatementExtension");
pub const wszINFSECTION_BASICCONSTRAINTS: windows_core::PCWSTR = windows_core::w!("BasicConstraintsExtension");
pub const wszINFSECTION_CAPOLICY: windows_core::PCWSTR = windows_core::w!("CAPolicy");
pub const wszINFSECTION_CCDP: windows_core::PCWSTR = windows_core::w!("CrossCertificateDistributionPointsExtension");
pub const wszINFSECTION_CDP: windows_core::PCWSTR = windows_core::w!("CRLDistributionPoint");
pub const wszINFSECTION_CERTSERVER: windows_core::PCWSTR = windows_core::w!("certsrv_server");
pub const wszINFSECTION_EKU: windows_core::PCWSTR = windows_core::w!("EnhancedKeyUsageExtension");
pub const wszINFSECTION_EXTENSIONS: windows_core::PCWSTR = windows_core::w!("Extensions");
pub const wszINFSECTION_NAMECONSTRAINTS: windows_core::PCWSTR = windows_core::w!("NameConstraintsExtension");
pub const wszINFSECTION_NEWREQUEST: windows_core::PCWSTR = windows_core::w!("NewRequest");
pub const wszINFSECTION_POLICYCONSTRAINTS: windows_core::PCWSTR = windows_core::w!("PolicyConstraintsExtension");
pub const wszINFSECTION_POLICYMAPPINGS: windows_core::PCWSTR = windows_core::w!("PolicyMappingsExtension");
pub const wszINFSECTION_POLICYSTATEMENT: windows_core::PCWSTR = windows_core::w!("PolicyStatementExtension");
pub const wszINFSECTION_PROPERTIES: windows_core::PCWSTR = windows_core::w!("Properties");
pub const wszINFSECTION_REQUESTATTRIBUTES: windows_core::PCWSTR = windows_core::w!("RequestAttributes");
pub const wszINFVALUE_ENDORSEMENTKEY: windows_core::PCWSTR = windows_core::w!("EndorsementKey");
pub const wszINFVALUE_REQUESTTYPE_CERT: windows_core::PCWSTR = windows_core::w!("Cert");
pub const wszINFVALUE_REQUESTTYPE_CMC: windows_core::PCWSTR = windows_core::w!("CMC");
pub const wszINFVALUE_REQUESTTYPE_PKCS10: windows_core::PCWSTR = windows_core::w!("PKCS10");
pub const wszINFVALUE_REQUESTTYPE_PKCS7: windows_core::PCWSTR = windows_core::w!("PKCS7");
pub const wszINFVALUE_REQUESTTYPE_SCEP: windows_core::PCWSTR = windows_core::w!("SCEP");
pub const wszLDAPSESSIONOPTIONVALUE: windows_core::PCWSTR = windows_core::w!("LDAPSessionOptionValue");
pub const wszLOCALIZEDTIMEPERIODUNITS: windows_core::PCWSTR = windows_core::w!("LocalizedTimePeriodUnits");
pub const wszLOGFILENAMEEXT: windows_core::PCWSTR = windows_core::w!(".log");
pub const wszLOGPATH: windows_core::PCWSTR = windows_core::w!("CertLog");
pub const wszMACHINEKEYSET: windows_core::PCWSTR = windows_core::w!("MachineKeyset");
pub const wszMICROSOFTCERTMODULE_PREFIX: windows_core::PCWSTR = windows_core::w!("CertificateAuthority_MicrosoftDefault");
pub const wszNETSCAPEREVOCATIONTYPE: windows_core::PCWSTR = windows_core::w!("Netscape");
pub const wszOCSPCAPROP_CACERTIFICATE: windows_core::PCWSTR = windows_core::w!("CACertificate");
pub const wszOCSPCAPROP_CACONFIG: windows_core::PCWSTR = windows_core::w!("CAConfig");
pub const wszOCSPCAPROP_CSPNAME: windows_core::PCWSTR = windows_core::w!("CSPName");
pub const wszOCSPCAPROP_ERRORCODE: windows_core::PCWSTR = windows_core::w!("ErrorCode");
pub const wszOCSPCAPROP_HASHALGORITHMID: windows_core::PCWSTR = windows_core::w!("HashAlgorithmId");
pub const wszOCSPCAPROP_KEYSPEC: windows_core::PCWSTR = windows_core::w!("KeySpec");
pub const wszOCSPCAPROP_LOCALREVOCATIONINFORMATION: windows_core::PCWSTR = windows_core::w!("LocalRevocationInformation");
pub const wszOCSPCAPROP_PROVIDERCLSID: windows_core::PCWSTR = windows_core::w!("ProviderCLSID");
pub const wszOCSPCAPROP_PROVIDERPROPERTIES: windows_core::PCWSTR = windows_core::w!("Provider");
pub const wszOCSPCAPROP_REMINDERDURATION: windows_core::PCWSTR = windows_core::w!("ReminderDuration");
pub const wszOCSPCAPROP_SIGNINGCERTIFICATE: windows_core::PCWSTR = windows_core::w!("SigningCertificate");
pub const wszOCSPCAPROP_SIGNINGCERTIFICATETEMPLATE: windows_core::PCWSTR = windows_core::w!("SigningCertificateTemplate");
pub const wszOCSPCAPROP_SIGNINGFLAGS: windows_core::PCWSTR = windows_core::w!("SigningFlags");
pub const wszOCSPCOMMONPROP_MAXINCOMINGMESSAGESIZE: windows_core::PCWSTR = windows_core::w!("MaxIncomingMessageSize");
pub const wszOCSPCOMMONPROP_MAXNUMOFREQUESTENTRIES: windows_core::PCWSTR = windows_core::w!("MaxNumOfRequestEntries");
pub const wszOCSPCOMMONPROP_REQFLAGS: windows_core::PCWSTR = windows_core::w!("RequestFlags");
pub const wszOCSPISAPIPROP_DEBUG: windows_core::PCWSTR = windows_core::w!("ISAPIDebug");
pub const wszOCSPISAPIPROP_MAXAGE: windows_core::PCWSTR = windows_core::w!("MaxAge");
pub const wszOCSPISAPIPROP_MAXNUMOFCACHEENTRIES: windows_core::PCWSTR = windows_core::w!("MaxNumOfCacheEntries");
pub const wszOCSPISAPIPROP_NUMOFBACKENDCONNECTIONS: windows_core::PCWSTR = windows_core::w!("NumOfBackendConnections");
pub const wszOCSPISAPIPROP_NUMOFTHREADS: windows_core::PCWSTR = windows_core::w!("NumOfThreads");
pub const wszOCSPISAPIPROP_REFRESHRATE: windows_core::PCWSTR = windows_core::w!("RefreshRate");
pub const wszOCSPISAPIPROP_VIRTUALROOTNAME: windows_core::PCWSTR = windows_core::w!("VirtualRootName");
pub const wszOCSPPROP_ARRAYCONTROLLER: windows_core::PCWSTR = windows_core::w!("ArrayController");
pub const wszOCSPPROP_ARRAYMEMBERS: windows_core::PCWSTR = windows_core::w!("ArrayMembers");
pub const wszOCSPPROP_AUDITFILTER: windows_core::PCWSTR = windows_core::w!("AuditFilter");
pub const wszOCSPPROP_DEBUG: windows_core::PCWSTR = windows_core::w!("Debug");
pub const wszOCSPPROP_ENROLLPOLLINTERVAL: windows_core::PCWSTR = windows_core::w!("EnrollPollInterval");
pub const wszOCSPPROP_LOGLEVEL: windows_core::PCWSTR = windows_core::w!("LogLevel");
pub const wszOCSPREVPROP_BASECRL: windows_core::PCWSTR = windows_core::w!("BaseCrl");
pub const wszOCSPREVPROP_BASECRLURLS: windows_core::PCWSTR = windows_core::w!("BaseCrlUrls");
pub const wszOCSPREVPROP_CRLURLTIMEOUT: windows_core::PCWSTR = windows_core::w!("CrlUrlTimeOut");
pub const wszOCSPREVPROP_DELTACRL: windows_core::PCWSTR = windows_core::w!("DeltaCrl");
pub const wszOCSPREVPROP_DELTACRLURLS: windows_core::PCWSTR = windows_core::w!("DeltaCrlUrls");
pub const wszOCSPREVPROP_ERRORCODE: windows_core::PCWSTR = windows_core::w!("RevocationErrorCode");
pub const wszOCSPREVPROP_REFRESHTIMEOUT: windows_core::PCWSTR = windows_core::w!("RefreshTimeOut");
pub const wszOCSPREVPROP_SERIALNUMBERSDIRS: windows_core::PCWSTR = windows_core::w!("IssuedSerialNumbersDirectories");
pub const wszPERIODDAYS: windows_core::PCWSTR = windows_core::w!("Days");
pub const wszPERIODHOURS: windows_core::PCWSTR = windows_core::w!("Hours");
pub const wszPERIODMINUTES: windows_core::PCWSTR = windows_core::w!("Minutes");
pub const wszPERIODMONTHS: windows_core::PCWSTR = windows_core::w!("Months");
pub const wszPERIODSECONDS: windows_core::PCWSTR = windows_core::w!("Seconds");
pub const wszPERIODWEEKS: windows_core::PCWSTR = windows_core::w!("Weeks");
pub const wszPERIODYEARS: windows_core::PCWSTR = windows_core::w!("Years");
pub const wszPFXFILENAMEEXT: windows_core::PCWSTR = windows_core::w!(".p12");
pub const wszPROPATTESTATIONCHALLENGE: windows_core::PCWSTR = windows_core::w!("AttestationChallenge");
pub const wszPROPATTRIBNAME: windows_core::PCWSTR = windows_core::w!("AttributeName");
pub const wszPROPATTRIBREQUESTID: windows_core::PCWSTR = windows_core::w!("AttributeRequestId");
pub const wszPROPATTRIBVALUE: windows_core::PCWSTR = windows_core::w!("AttributeValue");
pub const wszPROPCALLERNAME: windows_core::PCWSTR = windows_core::w!("CallerName");
pub const wszPROPCATYPE: windows_core::PCWSTR = windows_core::w!("CAType");
pub const wszPROPCERTCLIENTMACHINE: windows_core::PCWSTR = windows_core::w!("ccm");
pub const wszPROPCERTCOUNT: windows_core::PCWSTR = windows_core::w!("CertCount");
pub const wszPROPCERTIFICATEENROLLMENTFLAGS: windows_core::PCWSTR = windows_core::w!("EnrollmentFlags");
pub const wszPROPCERTIFICATEGENERALFLAGS: windows_core::PCWSTR = windows_core::w!("GeneralFlags");
pub const wszPROPCERTIFICATEHASH: windows_core::PCWSTR = windows_core::w!("CertificateHash");
pub const wszPROPCERTIFICATENOTAFTERDATE: windows_core::PCWSTR = windows_core::w!("NotAfter");
pub const wszPROPCERTIFICATENOTBEFOREDATE: windows_core::PCWSTR = windows_core::w!("NotBefore");
pub const wszPROPCERTIFICATEPRIVATEKEYFLAGS: windows_core::PCWSTR = windows_core::w!("PrivatekeyFlags");
pub const wszPROPCERTIFICATEPUBLICKEYALGORITHM: windows_core::PCWSTR = windows_core::w!("PublicKeyAlgorithm");
pub const wszPROPCERTIFICATEPUBLICKEYLENGTH: windows_core::PCWSTR = windows_core::w!("PublicKeyLength");
pub const wszPROPCERTIFICATERAWPUBLICKEY: windows_core::PCWSTR = windows_core::w!("RawPublicKey");
pub const wszPROPCERTIFICATERAWPUBLICKEYALGORITHMPARAMETERS: windows_core::PCWSTR = windows_core::w!("RawPublicKeyAlgorithmParameters");
pub const wszPROPCERTIFICATERAWSMIMECAPABILITIES: windows_core::PCWSTR = windows_core::w!("RawSMIMECapabilities");
pub const wszPROPCERTIFICATEREQUESTID: windows_core::PCWSTR = windows_core::w!("RequestID");
pub const wszPROPCERTIFICATESERIALNUMBER: windows_core::PCWSTR = windows_core::w!("SerialNumber");
pub const wszPROPCERTIFICATESUBJECTKEYIDENTIFIER: windows_core::PCWSTR = windows_core::w!("SubjectKeyIdentifier");
pub const wszPROPCERTIFICATETEMPLATE: windows_core::PCWSTR = windows_core::w!("CertificateTemplate");
pub const wszPROPCERTIFICATETYPE: windows_core::PCWSTR = windows_core::w!("CertificateType");
pub const wszPROPCERTIFICATEUPN: windows_core::PCWSTR = windows_core::w!("UPN");
pub const wszPROPCERTSTATE: windows_core::PCWSTR = windows_core::w!("CertState");
pub const wszPROPCERTSUFFIX: windows_core::PCWSTR = windows_core::w!("CertSuffix");
pub const wszPROPCERTTEMPLATE: windows_core::PCWSTR = windows_core::w!("CertificateTemplate");
pub const wszPROPCERTTYPE: windows_core::PCWSTR = windows_core::w!("CertType");
pub const wszPROPCERTUSAGE: windows_core::PCWSTR = windows_core::w!("CertificateUsage");
pub const wszPROPCHALLENGE: windows_core::PCWSTR = windows_core::w!("Challenge");
pub const wszPROPCLIENTBROWSERMACHINE: windows_core::PCWSTR = windows_core::w!("cbm");
pub const wszPROPCLIENTDCDNS: windows_core::PCWSTR = windows_core::w!("cdc");
pub const wszPROPCOMMONNAME: windows_core::PCWSTR = windows_core::w!("CommonName");
pub const wszPROPCONFIGDN: windows_core::PCWSTR = windows_core::w!("ConfigDN");
pub const wszPROPCOUNTRY: windows_core::PCWSTR = windows_core::w!("Country");
pub const wszPROPCRITICALTAG: windows_core::PCWSTR = windows_core::w!("{critical}");
pub const wszPROPCRLCOUNT: windows_core::PCWSTR = windows_core::w!("CRLCount");
pub const wszPROPCRLEFFECTIVE: windows_core::PCWSTR = windows_core::w!("CRLEffective");
pub const wszPROPCRLINDEX: windows_core::PCWSTR = windows_core::w!("CRLIndex");
pub const wszPROPCRLLASTPUBLISHED: windows_core::PCWSTR = windows_core::w!("CRLLastPublished");
pub const wszPROPCRLMINBASE: windows_core::PCWSTR = windows_core::w!("CRLMinBase");
pub const wszPROPCRLNAMEID: windows_core::PCWSTR = windows_core::w!("CRLNameId");
pub const wszPROPCRLNEXTPUBLISH: windows_core::PCWSTR = windows_core::w!("CRLNextPublish");
pub const wszPROPCRLNEXTUPDATE: windows_core::PCWSTR = windows_core::w!("CRLNextUpdate");
pub const wszPROPCRLNUMBER: windows_core::PCWSTR = windows_core::w!("CRLNumber");
pub const wszPROPCRLPROPAGATIONCOMPLETE: windows_core::PCWSTR = windows_core::w!("CRLPropagationComplete");
pub const wszPROPCRLPUBLISHATTEMPTS: windows_core::PCWSTR = windows_core::w!("CRLPublishAttempts");
pub const wszPROPCRLPUBLISHERROR: windows_core::PCWSTR = windows_core::w!("CRLPublishError");
pub const wszPROPCRLPUBLISHFLAGS: windows_core::PCWSTR = windows_core::w!("CRLPublishFlags");
pub const wszPROPCRLPUBLISHSTATUSCODE: windows_core::PCWSTR = windows_core::w!("CRLPublishStatusCode");
pub const wszPROPCRLRAWCRL: windows_core::PCWSTR = windows_core::w!("CRLRawCRL");
pub const wszPROPCRLROWID: windows_core::PCWSTR = windows_core::w!("CRLRowId");
pub const wszPROPCRLSTATE: windows_core::PCWSTR = windows_core::w!("CRLState");
pub const wszPROPCRLSUFFIX: windows_core::PCWSTR = windows_core::w!("CRLSuffix");
pub const wszPROPCRLTHISPUBLISH: windows_core::PCWSTR = windows_core::w!("CRLThisPublish");
pub const wszPROPCRLTHISUPDATE: windows_core::PCWSTR = windows_core::w!("CRLThisUpdate");
pub const wszPROPCROSSFOREST: windows_core::PCWSTR = windows_core::w!("CrossForest");
pub const wszPROPDCNAME: windows_core::PCWSTR = windows_core::w!("DCName");
pub const wszPROPDECIMALTAG: windows_core::PCWSTR = windows_core::w!("{decimal}");
pub const wszPROPDELTACRLSDISABLED: windows_core::PCWSTR = windows_core::w!("fDeltaCRLsDisabled");
pub const wszPROPDEVICESERIALNUMBER: windows_core::PCWSTR = windows_core::w!("DeviceSerialNumber");
pub const wszPROPDISPOSITION: windows_core::PCWSTR = windows_core::w!("Disposition");
pub const wszPROPDISPOSITIONDENY: windows_core::PCWSTR = windows_core::w!("Deny");
pub const wszPROPDISPOSITIONPENDING: windows_core::PCWSTR = windows_core::w!("Pending");
pub const wszPROPDISTINGUISHEDNAME: windows_core::PCWSTR = windows_core::w!("DistinguishedName");
pub const wszPROPDN: windows_core::PCWSTR = windows_core::w!("dn");
pub const wszPROPDNS: windows_core::PCWSTR = windows_core::w!("dns");
pub const wszPROPDOMAINCOMPONENT: windows_core::PCWSTR = windows_core::w!("DomainComponent");
pub const wszPROPDOMAINDN: windows_core::PCWSTR = windows_core::w!("DomainDN");
pub const wszPROPEMAIL: windows_core::PCWSTR = windows_core::w!("EMail");
pub const wszPROPENDORSEMENTCERTIFICATEHASH: windows_core::PCWSTR = windows_core::w!("EndorsementCertificateHash");
pub const wszPROPENDORSEMENTKEYHASH: windows_core::PCWSTR = windows_core::w!("EndorsementKeyHash");
pub const wszPROPEVENTLOGERROR: windows_core::PCWSTR = windows_core::w!("EventLogError");
pub const wszPROPEVENTLOGEXHAUSTIVE: windows_core::PCWSTR = windows_core::w!("EventLogExhaustive");
pub const wszPROPEVENTLOGTERSE: windows_core::PCWSTR = windows_core::w!("EventLogTerse");
pub const wszPROPEVENTLOGVERBOSE: windows_core::PCWSTR = windows_core::w!("EventLogVerbose");
pub const wszPROPEVENTLOGWARNING: windows_core::PCWSTR = windows_core::w!("EventLogWarning");
pub const wszPROPEXITCERTFILE: windows_core::PCWSTR = windows_core::w!("CertFile");
pub const wszPROPEXPECTEDCHALLENGE: windows_core::PCWSTR = windows_core::w!("ExpectedChallenge");
pub const wszPROPEXPIRATIONDATE: windows_core::PCWSTR = windows_core::w!("ExpirationDate");
pub const wszPROPEXTFLAGS: windows_core::PCWSTR = windows_core::w!("ExtensionFlags");
pub const wszPROPEXTNAME: windows_core::PCWSTR = windows_core::w!("ExtensionName");
pub const wszPROPEXTRAWVALUE: windows_core::PCWSTR = windows_core::w!("ExtensionRawValue");
pub const wszPROPEXTREQUESTID: windows_core::PCWSTR = windows_core::w!("ExtensionRequestId");
pub const wszPROPFILETAG: windows_core::PCWSTR = windows_core::w!("{file}");
pub const wszPROPGIVENNAME: windows_core::PCWSTR = windows_core::w!("GivenName");
pub const wszPROPGUID: windows_core::PCWSTR = windows_core::w!("guid");
pub const wszPROPHEXTAG: windows_core::PCWSTR = windows_core::w!("{hex}");
pub const wszPROPINITIALS: windows_core::PCWSTR = windows_core::w!("Initials");
pub const wszPROPIPADDRESS: windows_core::PCWSTR = windows_core::w!("ipaddress");
pub const wszPROPKEYARCHIVED: windows_core::PCWSTR = windows_core::w!("KeyArchived");
pub const wszPROPLOCALITY: windows_core::PCWSTR = windows_core::w!("Locality");
pub const wszPROPLOGLEVEL: windows_core::PCWSTR = windows_core::w!("LogLevel");
pub const wszPROPMACHINEDNSNAME: windows_core::PCWSTR = windows_core::w!("MachineDNSName");
pub const wszPROPMODULEREGLOC: windows_core::PCWSTR = windows_core::w!("ModuleRegistryLocation");
pub const wszPROPNAMETYPE: windows_core::PCWSTR = windows_core::w!("NameType");
pub const wszPROPOCTETTAG: windows_core::PCWSTR = windows_core::w!("{octet}");
pub const wszPROPOFFICER: windows_core::PCWSTR = windows_core::w!("Officer");
pub const wszPROPOID: windows_core::PCWSTR = windows_core::w!("oid");
pub const wszPROPORGANIZATION: windows_core::PCWSTR = windows_core::w!("Organization");
pub const wszPROPORGUNIT: windows_core::PCWSTR = windows_core::w!("OrgUnit");
pub const wszPROPPUBLISHEXPIREDCERTINCRL: windows_core::PCWSTR = windows_core::w!("PublishExpiredCertInCRL");
pub const wszPROPRAWCACERTIFICATE: windows_core::PCWSTR = windows_core::w!("RawCACertificate");
pub const wszPROPRAWCERTIFICATE: windows_core::PCWSTR = windows_core::w!("RawCertificate");
pub const wszPROPRAWCRL: windows_core::PCWSTR = windows_core::w!("RawCRL");
pub const wszPROPRAWDELTACRL: windows_core::PCWSTR = windows_core::w!("RawDeltaCRL");
pub const wszPROPRAWNAME: windows_core::PCWSTR = windows_core::w!("RawName");
pub const wszPROPRAWPRECERTIFICATE: windows_core::PCWSTR = windows_core::w!("RawPrecertificate");
pub const wszPROPREQUESTARCHIVEDKEY: windows_core::PCWSTR = windows_core::w!("ArchivedKey");
pub const wszPROPREQUESTATTRIBUTES: windows_core::PCWSTR = windows_core::w!("RequestAttributes");
pub const wszPROPREQUESTCSPPROVIDER: windows_core::PCWSTR = windows_core::w!("RequestCSPProvider");
pub const wszPROPREQUESTDISPOSITION: windows_core::PCWSTR = windows_core::w!("Disposition");
pub const wszPROPREQUESTDISPOSITIONMESSAGE: windows_core::PCWSTR = windows_core::w!("DispositionMessage");
pub const wszPROPREQUESTDOT: windows_core::PCWSTR = windows_core::w!("Request.");
pub const wszPROPREQUESTERCAACCESS: windows_core::PCWSTR = windows_core::w!("RequesterCAAccess");
pub const wszPROPREQUESTERDN: windows_core::PCWSTR = windows_core::w!("RequesterDN");
pub const wszPROPREQUESTERNAME: windows_core::PCWSTR = windows_core::w!("RequesterName");
pub const wszPROPREQUESTERNAMEFROMOLDCERTIFICATE: windows_core::PCWSTR = windows_core::w!("RequesterNameFromOldCertificate");
pub const wszPROPREQUESTERSAMNAME: windows_core::PCWSTR = windows_core::w!("RequesterSAMName");
pub const wszPROPREQUESTERUPN: windows_core::PCWSTR = windows_core::w!("RequesterUPN");
pub const wszPROPREQUESTFLAGS: windows_core::PCWSTR = windows_core::w!("RequestFlags");
pub const wszPROPREQUESTKEYRECOVERYHASHES: windows_core::PCWSTR = windows_core::w!("KeyRecoveryHashes");
pub const wszPROPREQUESTMACHINEDNS: windows_core::PCWSTR = windows_core::w!("rmd");
pub const wszPROPREQUESTOSVERSION: windows_core::PCWSTR = windows_core::w!("RequestOSVersion");
pub const wszPROPREQUESTRAWARCHIVEDKEY: windows_core::PCWSTR = windows_core::w!("RawArchivedKey");
pub const wszPROPREQUESTRAWOLDCERTIFICATE: windows_core::PCWSTR = windows_core::w!("RawOldCertificate");
pub const wszPROPREQUESTRAWREQUEST: windows_core::PCWSTR = windows_core::w!("RawRequest");
pub const wszPROPREQUESTREQUESTID: windows_core::PCWSTR = windows_core::w!("RequestID");
pub const wszPROPREQUESTRESOLVEDWHEN: windows_core::PCWSTR = windows_core::w!("ResolvedWhen");
pub const wszPROPREQUESTREVOKEDEFFECTIVEWHEN: windows_core::PCWSTR = windows_core::w!("RevokedEffectiveWhen");
pub const wszPROPREQUESTREVOKEDREASON: windows_core::PCWSTR = windows_core::w!("RevokedReason");
pub const wszPROPREQUESTREVOKEDWHEN: windows_core::PCWSTR = windows_core::w!("RevokedWhen");
pub const wszPROPREQUESTSTATUSCODE: windows_core::PCWSTR = windows_core::w!("StatusCode");
pub const wszPROPREQUESTSUBMITTEDWHEN: windows_core::PCWSTR = windows_core::w!("SubmittedWhen");
pub const wszPROPREQUESTTYPE: windows_core::PCWSTR = windows_core::w!("RequestType");
pub const wszPROPSANITIZEDCANAME: windows_core::PCWSTR = windows_core::w!("SanitizedCAName");
pub const wszPROPSANITIZEDSHORTNAME: windows_core::PCWSTR = windows_core::w!("SanitizedShortName");
pub const wszPROPSEAUDITFILTER: windows_core::PCWSTR = windows_core::w!("SEAuditFilter");
pub const wszPROPSEAUDITID: windows_core::PCWSTR = windows_core::w!("SEAuditId");
pub const wszPROPSERVERUPGRADED: windows_core::PCWSTR = windows_core::w!("fServerUpgraded");
pub const wszPROPSESSIONCOUNT: windows_core::PCWSTR = windows_core::w!("SessionCount");
pub const wszPROPSIGNERAPPLICATIONPOLICIES: windows_core::PCWSTR = windows_core::w!("SignerApplicationPolicies");
pub const wszPROPSIGNERPOLICIES: windows_core::PCWSTR = windows_core::w!("SignerPolicies");
pub const wszPROPSTATE: windows_core::PCWSTR = windows_core::w!("State");
pub const wszPROPSTREETADDRESS: windows_core::PCWSTR = windows_core::w!("StreetAddress");
pub const wszPROPSUBJECTALTNAME2: windows_core::PCWSTR = windows_core::w!("san");
pub const wszPROPSUBJECTDOT: windows_core::PCWSTR = windows_core::w!("Subject.");
pub const wszPROPSURNAME: windows_core::PCWSTR = windows_core::w!("SurName");
pub const wszPROPTEMPLATECHANGESEQUENCENUMBER: windows_core::PCWSTR = windows_core::w!("TemplateChangeSequenceNumber");
pub const wszPROPTEXTTAG: windows_core::PCWSTR = windows_core::w!("{text}");
pub const wszPROPTITLE: windows_core::PCWSTR = windows_core::w!("Title");
pub const wszPROPUNSTRUCTUREDADDRESS: windows_core::PCWSTR = windows_core::w!("UnstructuredAddress");
pub const wszPROPUNSTRUCTUREDNAME: windows_core::PCWSTR = windows_core::w!("UnstructuredName");
pub const wszPROPUPN: windows_core::PCWSTR = windows_core::w!("upn");
pub const wszPROPURL: windows_core::PCWSTR = windows_core::w!("url");
pub const wszPROPUSEDS: windows_core::PCWSTR = windows_core::w!("fUseDS");
pub const wszPROPUSERDN: windows_core::PCWSTR = windows_core::w!("UserDN");
pub const wszPROPUTF8TAG: windows_core::PCWSTR = windows_core::w!("{utf8}");
pub const wszPROPVALIDITYPERIODCOUNT: windows_core::PCWSTR = windows_core::w!("ValidityPeriodUnits");
pub const wszPROPVALIDITYPERIODSTRING: windows_core::PCWSTR = windows_core::w!("ValidityPeriod");
pub const wszPROPVOLATILEMODE: windows_core::PCWSTR = windows_core::w!("VolatileMode");
pub const wszREGACTIVE: windows_core::PCWSTR = windows_core::w!("Active");
pub const wszREGAELOGLEVEL_OLD: windows_core::PCWSTR = windows_core::w!("AEEventLogLevel");
pub const wszREGAIKCLOUDCAURL: windows_core::PCWSTR = windows_core::w!("AIKCloudCAURL");
pub const wszREGAIKKEYALGORITHM: windows_core::PCWSTR = windows_core::w!("AIKKeyAlgorithm");
pub const wszREGAIKKEYLENGTH: windows_core::PCWSTR = windows_core::w!("AIKKeyLength");
pub const wszREGALLPROVIDERS: windows_core::PCWSTR = windows_core::w!("All");
pub const wszREGALTERNATEPUBLISHDOMAINS: windows_core::PCWSTR = windows_core::w!("AlternatePublishDomains");
pub const wszREGALTERNATESIGNATUREALGORITHM: windows_core::PCWSTR = windows_core::w!("AlternateSignatureAlgorithm");
pub const wszREGAUDITFILTER: windows_core::PCWSTR = windows_core::w!("AuditFilter");
pub const wszREGB2ICERTMANAGEMODULE: windows_core::PCWSTR = windows_core::w!("ICertManageModule");
pub const wszREGBACKUPLOGDIRECTORY: windows_core::PCWSTR = windows_core::w!("BackupLogDirectory");
pub const wszREGCACERTFILENAME: windows_core::PCWSTR = windows_core::w!("CACertFileName");
pub const wszREGCACERTHASH: windows_core::PCWSTR = windows_core::w!("CACertHash");
pub const wszREGCACERTPUBLICATIONURLS: windows_core::PCWSTR = windows_core::w!("CACertPublicationURLs");
pub const wszREGCADESCRIPTION: windows_core::PCWSTR = windows_core::w!("CADescription");
pub const wszREGCAPATHLENGTH: windows_core::PCWSTR = windows_core::w!("CAPathLength");
pub const wszREGCASECURITY: windows_core::PCWSTR = windows_core::w!("Security");
pub const wszREGCASERIALNUMBER: windows_core::PCWSTR = windows_core::w!("CACertSerialNumber");
pub const wszREGCASERVERNAME: windows_core::PCWSTR = windows_core::w!("CAServerName");
pub const wszREGCATYPE: windows_core::PCWSTR = windows_core::w!("CAType");
pub const wszREGCAUSEDS: windows_core::PCWSTR = windows_core::w!("UseDS");
pub const wszREGCAXCHGCERTHASH: windows_core::PCWSTR = windows_core::w!("CAXchgCertHash");
pub const wszREGCAXCHGOVERLAPPERIODCOUNT: windows_core::PCWSTR = windows_core::w!("CAXchgOverlapPeriodUnits");
pub const wszREGCAXCHGOVERLAPPERIODSTRING: windows_core::PCWSTR = windows_core::w!("CAXchgOverlapPeriod");
pub const wszREGCAXCHGVALIDITYPERIODCOUNT: windows_core::PCWSTR = windows_core::w!("CAXchgValidityPeriodUnits");
pub const wszREGCAXCHGVALIDITYPERIODSTRING: windows_core::PCWSTR = windows_core::w!("CAXchgValidityPeriod");
pub const wszREGCERTENROLLCOMPATIBLE: windows_core::PCWSTR = windows_core::w!("CertEnrollCompatible");
pub const wszREGCERTIFICATETRANSPARENCYINFOOID: windows_core::PCWSTR = windows_core::w!("CTInformationExtensionOid");
pub const wszREGCERTPUBLISHFLAGS: windows_core::PCWSTR = windows_core::w!("PublishCertFlags");
pub const wszREGCERTSRVDEBUG: windows_core::PCWSTR = windows_core::w!("Debug");
pub const wszREGCHECKPOINTFILE: windows_core::PCWSTR = windows_core::w!("CheckPointFile");
pub const wszREGCLOCKSKEWMINUTES: windows_core::PCWSTR = windows_core::w!("ClockSkewMinutes");
pub const wszREGCOMMONNAME: windows_core::PCWSTR = windows_core::w!("CommonName");
pub const wszREGCRLATTEMPTREPUBLISH: windows_core::PCWSTR = windows_core::w!("CRLAttemptRepublish");
pub const wszREGCRLDELTANEXTPUBLISH: windows_core::PCWSTR = windows_core::w!("CRLDeltaNextPublish");
pub const wszREGCRLDELTAOVERLAPPERIODCOUNT: windows_core::PCWSTR = windows_core::w!("CRLDeltaOverlapUnits");
pub const wszREGCRLDELTAOVERLAPPERIODSTRING: windows_core::PCWSTR = windows_core::w!("CRLDeltaOverlapPeriod");
pub const wszREGCRLDELTAPERIODCOUNT: windows_core::PCWSTR = windows_core::w!("CRLDeltaPeriodUnits");
pub const wszREGCRLDELTAPERIODSTRING: windows_core::PCWSTR = windows_core::w!("CRLDeltaPeriod");
pub const wszREGCRLEDITFLAGS: windows_core::PCWSTR = windows_core::w!("CRLEditFlags");
pub const wszREGCRLFLAGS: windows_core::PCWSTR = windows_core::w!("CRLFlags");
pub const wszREGCRLNEXTPUBLISH: windows_core::PCWSTR = windows_core::w!("CRLNextPublish");
pub const wszREGCRLOVERLAPPERIODCOUNT: windows_core::PCWSTR = windows_core::w!("CRLOverlapUnits");
pub const wszREGCRLOVERLAPPERIODSTRING: windows_core::PCWSTR = windows_core::w!("CRLOverlapPeriod");
pub const wszREGCRLPATH_OLD: windows_core::PCWSTR = windows_core::w!("CRLPath");
pub const wszREGCRLPERIODCOUNT: windows_core::PCWSTR = windows_core::w!("CRLPeriodUnits");
pub const wszREGCRLPERIODSTRING: windows_core::PCWSTR = windows_core::w!("CRLPeriod");
pub const wszREGCRLPUBLICATIONURLS: windows_core::PCWSTR = windows_core::w!("CRLPublicationURLs");
pub const wszREGDATABASERECOVERED: windows_core::PCWSTR = windows_core::w!("DatabaseRecovered");
pub const wszREGDBDIRECTORY: windows_core::PCWSTR = windows_core::w!("DBDirectory");
pub const wszREGDBFLAGS: windows_core::PCWSTR = windows_core::w!("DBFlags");
pub const wszREGDBLASTFULLBACKUP: windows_core::PCWSTR = windows_core::w!("DBLastFullBackup");
pub const wszREGDBLASTINCREMENTALBACKUP: windows_core::PCWSTR = windows_core::w!("DBLastIncrementalBackup");
pub const wszREGDBLASTRECOVERY: windows_core::PCWSTR = windows_core::w!("DBLastRecovery");
pub const wszREGDBLOGDIRECTORY: windows_core::PCWSTR = windows_core::w!("DBLogDirectory");
pub const wszREGDBMAXREADSESSIONCOUNT: windows_core::PCWSTR = windows_core::w!("DBMaxReadSessionCount");
pub const wszREGDBSESSIONCOUNT: windows_core::PCWSTR = windows_core::w!("DBSessionCount");
pub const wszREGDBSYSDIRECTORY: windows_core::PCWSTR = windows_core::w!("DBSystemDirectory");
pub const wszREGDBTEMPDIRECTORY: windows_core::PCWSTR = windows_core::w!("DBTempDirectory");
pub const wszREGDEFAULTSMIME: windows_core::PCWSTR = windows_core::w!("DefaultSMIME");
pub const wszREGDIRECTORY: windows_core::PCWSTR = windows_core::w!("ConfigurationDirectory");
pub const wszREGDISABLEEXTENSIONLIST: windows_core::PCWSTR = windows_core::w!("DisableExtensionList");
pub const wszREGDSCONFIGDN: windows_core::PCWSTR = windows_core::w!("DSConfigDN");
pub const wszREGDSDOMAINDN: windows_core::PCWSTR = windows_core::w!("DSDomainDN");
pub const wszREGEDITFLAGS: windows_core::PCWSTR = windows_core::w!("EditFlags");
pub const wszREGEKPUBLISTDIRECTORIES: windows_core::PCWSTR = windows_core::w!("EndorsementKeyListDirectories");
pub const wszREGEKUOIDSFORPUBLISHEXPIREDCERTINCRL: windows_core::PCWSTR = windows_core::w!("EKUOIDsForPublishExpiredCertInCRL");
pub const wszREGEKUOIDSFORVOLATILEREQUESTS: windows_core::PCWSTR = windows_core::w!("EKUOIDsforVolatileRequests");
pub const wszREGENABLED: windows_core::PCWSTR = windows_core::w!("Enabled");
pub const wszREGENABLEDEKUFORDEFINEDCACERT: windows_core::PCWSTR = windows_core::w!("EnabledEKUForDefinedCACert");
pub const wszREGENABLEENROLLEEREQUESTEXTENSIONLIST: windows_core::PCWSTR = windows_core::w!("EnableEnrolleeRequestExtensionList");
pub const wszREGENABLEREQUESTEXTENSIONLIST: windows_core::PCWSTR = windows_core::w!("EnableRequestExtensionList");
pub const wszREGENFORCEX500NAMELENGTHS: windows_core::PCWSTR = windows_core::w!("EnforceX500NameLengths");
pub const wszREGENROLLFLAGS: windows_core::PCWSTR = windows_core::w!("EnrollFlags");
pub const wszREGEXITBODYARG: windows_core::PCWSTR = windows_core::w!("BodyArg");
pub const wszREGEXITBODYFORMAT: windows_core::PCWSTR = windows_core::w!("BodyFormat");
pub const wszREGEXITCRLISSUEDKEY: windows_core::PCWSTR = windows_core::w!("CRLIssued");
pub const wszREGEXITDENIEDKEY: windows_core::PCWSTR = windows_core::w!("Denied");
pub const wszREGEXITIMPORTEDKEY: windows_core::PCWSTR = windows_core::w!("Imported");
pub const wszREGEXITISSUEDKEY: windows_core::PCWSTR = windows_core::w!("Issued");
pub const wszREGEXITPENDINGKEY: windows_core::PCWSTR = windows_core::w!("Pending");
pub const wszREGEXITPROPNOTFOUND: windows_core::PCWSTR = windows_core::w!("???");
pub const wszREGEXITREVOKEDKEY: windows_core::PCWSTR = windows_core::w!("Revoked");
pub const wszREGEXITSHUTDOWNKEY: windows_core::PCWSTR = windows_core::w!("Shutdown");
pub const wszREGEXITSMTPAUTHENTICATE: windows_core::PCWSTR = windows_core::w!("SMTPAuthenticate");
pub const wszREGEXITSMTPCC: windows_core::PCWSTR = windows_core::w!("Cc");
pub const wszREGEXITSMTPEVENTFILTER: windows_core::PCWSTR = windows_core::w!("EventFilter");
pub const wszREGEXITSMTPFROM: windows_core::PCWSTR = windows_core::w!("From");
pub const wszREGEXITSMTPKEY: windows_core::PCWSTR = windows_core::w!("SMTP");
pub const wszREGEXITSMTPSERVER: windows_core::PCWSTR = windows_core::w!("SMTPServer");
pub const wszREGEXITSMTPTEMPLATES: windows_core::PCWSTR = windows_core::w!("Templates");
pub const wszREGEXITSMTPTO: windows_core::PCWSTR = windows_core::w!("To");
pub const wszREGEXITSTARTUPKEY: windows_core::PCWSTR = windows_core::w!("Startup");
pub const wszREGEXITTITLEARG: windows_core::PCWSTR = windows_core::w!("TitleArg");
pub const wszREGEXITTITLEFORMAT: windows_core::PCWSTR = windows_core::w!("TitleFormat");
pub const wszREGFILEISSUERCERTURL_OLD: windows_core::PCWSTR = windows_core::w!("FileIssuerCertURL");
pub const wszREGFILEREVOCATIONCRLURL_OLD: windows_core::PCWSTR = windows_core::w!("FileRevocationCRLURL");
pub const wszREGFORCETELETEX: windows_core::PCWSTR = windows_core::w!("ForceTeletex");
pub const wszREGFTPISSUERCERTURL_OLD: windows_core::PCWSTR = windows_core::w!("FTPIssuerCertURL");
pub const wszREGFTPREVOCATIONCRLURL_OLD: windows_core::PCWSTR = windows_core::w!("FTPRevocationCRLURL");
pub const wszREGHIGHLOGNUMBER: windows_core::PCWSTR = windows_core::w!("HighLogNumber");
pub const wszREGHIGHSERIAL: windows_core::PCWSTR = windows_core::w!("HighSerial");
pub const wszREGINTERFACEFLAGS: windows_core::PCWSTR = windows_core::w!("InterfaceFlags");
pub const wszREGISSUERCERTURLFLAGS: windows_core::PCWSTR = windows_core::w!("IssuerCertURLFlags");
pub const wszREGISSUERCERTURL_OLD: windows_core::PCWSTR = windows_core::w!("IssuerCertURL");
pub const wszREGKEYBASE: windows_core::PCWSTR = windows_core::w!("SYSTEM\\CurrentControlSet\\Services\\CertSvc");
pub const wszREGKEYCERTSVCPATH: windows_core::PCWSTR = windows_core::w!("SYSTEM\\CurrentControlSet\\Services\\CertSvc");
pub const wszREGKEYCONFIG: windows_core::PCWSTR = windows_core::w!("Configuration");
pub const wszREGKEYCSP: windows_core::PCWSTR = windows_core::w!("CSP");
pub const wszREGKEYDBPARAMETERS: windows_core::PCWSTR = windows_core::w!("DBParameters");
pub const wszREGKEYENCRYPTIONCSP: windows_core::PCWSTR = windows_core::w!("EncryptionCSP");
pub const wszREGKEYENROLLMENT: windows_core::PCWSTR = windows_core::w!("Software\\Microsoft\\Cryptography\\AutoEnrollment");
pub const wszREGKEYEXITMODULES: windows_core::PCWSTR = windows_core::w!("ExitModules");
pub const wszREGKEYGROUPPOLICYENROLLMENT: windows_core::PCWSTR = windows_core::w!("Software\\Policies\\Microsoft\\Cryptography\\AutoEnrollment");
pub const wszREGKEYNOSYSTEMCERTSVCPATH: windows_core::PCWSTR = windows_core::w!("CurrentControlSet\\Services\\CertSvc");
pub const wszREGKEYPOLICYMODULES: windows_core::PCWSTR = windows_core::w!("PolicyModules");
pub const wszREGKEYREPAIR: windows_core::PCWSTR = windows_core::w!("KeyRepair");
pub const wszREGKEYRESTOREINPROGRESS: windows_core::PCWSTR = windows_core::w!("RestoreInProgress");
pub const wszREGKEYSIZE: windows_core::PCWSTR = windows_core::w!("KeySize");
pub const wszREGKRACERTCOUNT: windows_core::PCWSTR = windows_core::w!("KRACertCount");
pub const wszREGKRACERTHASH: windows_core::PCWSTR = windows_core::w!("KRACertHash");
pub const wszREGKRAFLAGS: windows_core::PCWSTR = windows_core::w!("KRAFlags");
pub const wszREGLDAPFLAGS: windows_core::PCWSTR = windows_core::w!("LDAPFlags");
pub const wszREGLDAPISSUERCERTURL_OLD: windows_core::PCWSTR = windows_core::w!("LDAPIssuerCertURL");
pub const wszREGLDAPREVOCATIONCRLURL_OLD: windows_core::PCWSTR = windows_core::w!("LDAPRevocationCRLURL");
pub const wszREGLDAPREVOCATIONDNTEMPLATE_OLD: windows_core::PCWSTR = windows_core::w!("LDAPRevocationDNTemplate");
pub const wszREGLDAPREVOCATIONDN_OLD: windows_core::PCWSTR = windows_core::w!("LDAPRevocationDN");
pub const wszREGLDAPSESSIONOPTIONS: windows_core::PCWSTR = windows_core::w!("LDAPSessionOptions");
pub const wszREGLOGLEVEL: windows_core::PCWSTR = windows_core::w!("LogLevel");
pub const wszREGLOGPATH: windows_core::PCWSTR = windows_core::w!("LogPath");
pub const wszREGLOWLOGNUMBER: windows_core::PCWSTR = windows_core::w!("LowLogNumber");
pub const wszREGMAXINCOMINGALLOCSIZE: windows_core::PCWSTR = windows_core::w!("MaxIncomingAllocSize");
pub const wszREGMAXINCOMINGMESSAGESIZE: windows_core::PCWSTR = windows_core::w!("MaxIncomingMessageSize");
pub const wszREGMAXPENDINGREQUESTDAYS: windows_core::PCWSTR = windows_core::w!("MaxPendingRequestDays");
pub const wszREGMAXSCTLISTSIZE: windows_core::PCWSTR = windows_core::w!("MaxSCTListSize");
pub const wszREGNAMESEPARATOR: windows_core::PCWSTR = windows_core::w!("SubjectNameSeparator");
pub const wszREGNETSCAPECERTTYPE: windows_core::PCWSTR = windows_core::w!("NetscapeCertType");
pub const wszREGOFFICERRIGHTS: windows_core::PCWSTR = windows_core::w!("OfficerRights");
pub const wszREGPARENTCAMACHINE: windows_core::PCWSTR = windows_core::w!("ParentCAMachine");
pub const wszREGPARENTCANAME: windows_core::PCWSTR = windows_core::w!("ParentCAName");
pub const wszREGPOLICYFLAGS: windows_core::PCWSTR = windows_core::w!("PolicyFlags");
pub const wszREGPRESERVESCEPDUMMYCERTS: windows_core::PCWSTR = windows_core::w!("PreserveSCEPDummyCerts");
pub const wszREGPROCESSINGFLAGS: windows_core::PCWSTR = windows_core::w!("ProcessingFlags");
pub const wszREGPROVIDER: windows_core::PCWSTR = windows_core::w!("Provider");
pub const wszREGPROVIDERTYPE: windows_core::PCWSTR = windows_core::w!("ProviderType");
pub const wszREGREQUESTDISPOSITION: windows_core::PCWSTR = windows_core::w!("RequestDisposition");
pub const wszREGREQUESTFILENAME: windows_core::PCWSTR = windows_core::w!("RequestFileName");
pub const wszREGREQUESTID: windows_core::PCWSTR = windows_core::w!("RequestId");
pub const wszREGREQUESTKEYCONTAINER: windows_core::PCWSTR = windows_core::w!("RequestKeyContainer");
pub const wszREGREQUESTKEYINDEX: windows_core::PCWSTR = windows_core::w!("RequestKeyIndex");
pub const wszREGRESTOREMAP: windows_core::PCWSTR = windows_core::w!("RestoreMap");
pub const wszREGRESTOREMAPCOUNT: windows_core::PCWSTR = windows_core::w!("RestoreMapCount");
pub const wszREGRESTORESTATUS: windows_core::PCWSTR = windows_core::w!("RestoreStatus");
pub const wszREGREVOCATIONCRLURL_OLD: windows_core::PCWSTR = windows_core::w!("RevocationCRLURL");
pub const wszREGREVOCATIONTYPE: windows_core::PCWSTR = windows_core::w!("RevocationType");
pub const wszREGREVOCATIONURL: windows_core::PCWSTR = windows_core::w!("RevocationURL");
pub const wszREGROLESEPARATIONENABLED: windows_core::PCWSTR = windows_core::w!("RoleSeparationEnabled");
pub const wszREGSETUPSTATUS: windows_core::PCWSTR = windows_core::w!("SetupStatus");
pub const wszREGSP4DEFAULTCONFIGURATION: windows_core::PCWSTR = windows_core::w!("DefaultConfiguration");
pub const wszREGSP4KEYSETNAME: windows_core::PCWSTR = windows_core::w!("KeySetName");
pub const wszREGSP4NAMES: windows_core::PCWSTR = windows_core::w!("Names");
pub const wszREGSP4QUERIES: windows_core::PCWSTR = windows_core::w!("Queries");
pub const wszREGSP4SUBJECTNAMESEPARATOR: windows_core::PCWSTR = windows_core::w!("SubjectNameSeparator");
pub const wszREGSUBJECTALTNAME: windows_core::PCWSTR = windows_core::w!("SubjectAltName");
pub const wszREGSUBJECTALTNAME2: windows_core::PCWSTR = windows_core::w!("SubjectAltName2");
pub const wszREGSUBJECTTEMPLATE: windows_core::PCWSTR = windows_core::w!("SubjectTemplate");
pub const wszREGSYMMETRICKEYSIZE: windows_core::PCWSTR = windows_core::w!("SymmetricKeySize");
pub const wszREGUNICODE: windows_core::PCWSTR = windows_core::w!("Unicode");
pub const wszREGUPNMAP: windows_core::PCWSTR = windows_core::w!("UPNMap");
pub const wszREGUSEDEFINEDCACERTINREQ: windows_core::PCWSTR = windows_core::w!("UseDefinedCACertInRequest");
pub const wszREGVALIDITYPERIODCOUNT: windows_core::PCWSTR = windows_core::w!("ValidityPeriodUnits");
pub const wszREGVALIDITYPERIODSTRING: windows_core::PCWSTR = windows_core::w!("ValidityPeriod");
pub const wszREGVERIFYFLAGS: windows_core::PCWSTR = windows_core::w!("VerifyFlags");
pub const wszREGVERSION: windows_core::PCWSTR = windows_core::w!("Version");
pub const wszREGVIEWAGEMINUTES: windows_core::PCWSTR = windows_core::w!("ViewAgeMinutes");
pub const wszREGVIEWIDLEMINUTES: windows_core::PCWSTR = windows_core::w!("ViewIdleMinutes");
pub const wszREGWEBCLIENTCAMACHINE: windows_core::PCWSTR = windows_core::w!("WebClientCAMachine");
pub const wszREGWEBCLIENTCANAME: windows_core::PCWSTR = windows_core::w!("WebClientCAName");
pub const wszREGWEBCLIENTCATYPE: windows_core::PCWSTR = windows_core::w!("WebClientCAType");
pub const wszSECUREDATTRIBUTES: windows_core::PCWSTR = windows_core::w!("SignedAttributes");
pub const wszSERVICE_NAME: windows_core::PCWSTR = windows_core::w!("CertSvc");
pub const wszzDEFAULTSIGNEDATTRIBUTES: windows_core::PCWSTR = windows_core::w!("RequesterName\u{0}");
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ADDED_CERT_TYPE(pub i32);
impl windows_core::TypeKind for ADDED_CERT_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ADDED_CERT_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ADDED_CERT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AlgorithmFlags(pub i32);
impl windows_core::TypeKind for AlgorithmFlags {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AlgorithmFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AlgorithmFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AlgorithmOperationFlags(pub i32);
impl windows_core::TypeKind for AlgorithmOperationFlags {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AlgorithmOperationFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AlgorithmOperationFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AlgorithmType(pub i32);
impl windows_core::TypeKind for AlgorithmType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AlgorithmType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AlgorithmType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AlternativeNameType(pub i32);
impl windows_core::TypeKind for AlternativeNameType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AlternativeNameType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AlternativeNameType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CERTADMIN_GET_ROLES_FLAGS(pub u32);
impl windows_core::TypeKind for CERTADMIN_GET_ROLES_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CERTADMIN_GET_ROLES_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CERTADMIN_GET_ROLES_FLAGS").field(&self.0).finish()
    }
}
impl CERTADMIN_GET_ROLES_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CERTADMIN_GET_ROLES_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CERTADMIN_GET_ROLES_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CERTADMIN_GET_ROLES_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CERTADMIN_GET_ROLES_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CERTADMIN_GET_ROLES_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CERTENROLL_OBJECTID(pub i32);
impl windows_core::TypeKind for CERTENROLL_OBJECTID {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CERTENROLL_OBJECTID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CERTENROLL_OBJECTID").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CERTENROLL_PROPERTYID(pub i32);
impl windows_core::TypeKind for CERTENROLL_PROPERTYID {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CERTENROLL_PROPERTYID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CERTENROLL_PROPERTYID").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CERT_ALT_NAME(pub i32);
impl windows_core::TypeKind for CERT_ALT_NAME {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CERT_ALT_NAME {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CERT_ALT_NAME").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CERT_CREATE_REQUEST_FLAGS(pub i32);
impl windows_core::TypeKind for CERT_CREATE_REQUEST_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CERT_CREATE_REQUEST_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CERT_CREATE_REQUEST_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CERT_DELETE_ROW_FLAGS(pub i32);
impl windows_core::TypeKind for CERT_DELETE_ROW_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CERT_DELETE_ROW_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CERT_DELETE_ROW_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CERT_EXIT_EVENT_MASK(pub u32);
impl windows_core::TypeKind for CERT_EXIT_EVENT_MASK {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CERT_EXIT_EVENT_MASK {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CERT_EXIT_EVENT_MASK").field(&self.0).finish()
    }
}
impl CERT_EXIT_EVENT_MASK {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CERT_EXIT_EVENT_MASK {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CERT_EXIT_EVENT_MASK {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CERT_EXIT_EVENT_MASK {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CERT_EXIT_EVENT_MASK {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CERT_EXIT_EVENT_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CERT_GET_CONFIG_FLAGS(pub i32);
impl windows_core::TypeKind for CERT_GET_CONFIG_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CERT_GET_CONFIG_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CERT_GET_CONFIG_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CERT_IMPORT_FLAGS(pub i32);
impl windows_core::TypeKind for CERT_IMPORT_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CERT_IMPORT_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CERT_IMPORT_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CERT_PROPERTY_TYPE(pub i32);
impl windows_core::TypeKind for CERT_PROPERTY_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CERT_PROPERTY_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CERT_PROPERTY_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CERT_REQUEST_OUT_TYPE(pub i32);
impl windows_core::TypeKind for CERT_REQUEST_OUT_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CERT_REQUEST_OUT_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CERT_REQUEST_OUT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CERT_VIEW_COLUMN_INDEX(pub i32);
impl windows_core::TypeKind for CERT_VIEW_COLUMN_INDEX {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CERT_VIEW_COLUMN_INDEX {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CERT_VIEW_COLUMN_INDEX").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CERT_VIEW_SEEK_OPERATOR_FLAGS(pub i32);
impl windows_core::TypeKind for CERT_VIEW_SEEK_OPERATOR_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CERT_VIEW_SEEK_OPERATOR_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CERT_VIEW_SEEK_OPERATOR_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CRLRevocationReason(pub i32);
impl windows_core::TypeKind for CRLRevocationReason {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CRLRevocationReason {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CRLRevocationReason").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CR_DISP(pub u32);
impl windows_core::TypeKind for CR_DISP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CR_DISP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CR_DISP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CSBACKUP_TYPE(pub u32);
impl windows_core::TypeKind for CSBACKUP_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CSBACKUP_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CSBACKUP_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CVRC_COLUMN(pub i32);
impl windows_core::TypeKind for CVRC_COLUMN {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CVRC_COLUMN {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CVRC_COLUMN").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CVRC_TABLE(pub i32);
impl windows_core::TypeKind for CVRC_TABLE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CVRC_TABLE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CVRC_TABLE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CommitTemplateFlags(pub i32);
impl windows_core::TypeKind for CommitTemplateFlags {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CommitTemplateFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CommitTemplateFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DelayRetryAction(pub i32);
impl windows_core::TypeKind for DelayRetryAction {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DelayRetryAction {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DelayRetryAction").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ENUM_CATYPES(pub i32);
impl windows_core::TypeKind for ENUM_CATYPES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ENUM_CATYPES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ENUM_CATYPES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ENUM_CERT_COLUMN_VALUE_FLAGS(pub i32);
impl windows_core::TypeKind for ENUM_CERT_COLUMN_VALUE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ENUM_CERT_COLUMN_VALUE_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ENUM_CERT_COLUMN_VALUE_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct EncodingType(pub i32);
impl windows_core::TypeKind for EncodingType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for EncodingType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("EncodingType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct EnrollmentCAProperty(pub i32);
impl windows_core::TypeKind for EnrollmentCAProperty {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for EnrollmentCAProperty {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("EnrollmentCAProperty").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct EnrollmentDisplayStatus(pub i32);
impl windows_core::TypeKind for EnrollmentDisplayStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for EnrollmentDisplayStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("EnrollmentDisplayStatus").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct EnrollmentEnrollStatus(pub i32);
impl windows_core::TypeKind for EnrollmentEnrollStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for EnrollmentEnrollStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("EnrollmentEnrollStatus").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct EnrollmentPolicyFlags(pub i32);
impl windows_core::TypeKind for EnrollmentPolicyFlags {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for EnrollmentPolicyFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("EnrollmentPolicyFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct EnrollmentPolicyServerPropertyFlags(pub i32);
impl windows_core::TypeKind for EnrollmentPolicyServerPropertyFlags {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for EnrollmentPolicyServerPropertyFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("EnrollmentPolicyServerPropertyFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct EnrollmentSelectionStatus(pub i32);
impl windows_core::TypeKind for EnrollmentSelectionStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for EnrollmentSelectionStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("EnrollmentSelectionStatus").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct EnrollmentTemplateProperty(pub i32);
impl windows_core::TypeKind for EnrollmentTemplateProperty {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for EnrollmentTemplateProperty {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("EnrollmentTemplateProperty").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FULL_RESPONSE_PROPERTY_ID(pub i32);
impl windows_core::TypeKind for FULL_RESPONSE_PROPERTY_ID {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FULL_RESPONSE_PROPERTY_ID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FULL_RESPONSE_PROPERTY_ID").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ImportPFXFlags(pub i32);
impl windows_core::TypeKind for ImportPFXFlags {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ImportPFXFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ImportPFXFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct InnerRequestLevel(pub i32);
impl windows_core::TypeKind for InnerRequestLevel {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for InnerRequestLevel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("InnerRequestLevel").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct InstallResponseRestrictionFlags(pub i32);
impl windows_core::TypeKind for InstallResponseRestrictionFlags {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for InstallResponseRestrictionFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("InstallResponseRestrictionFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct KeyAttestationClaimType(pub i32);
impl windows_core::TypeKind for KeyAttestationClaimType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for KeyAttestationClaimType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("KeyAttestationClaimType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct KeyIdentifierHashAlgorithm(pub i32);
impl windows_core::TypeKind for KeyIdentifierHashAlgorithm {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for KeyIdentifierHashAlgorithm {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("KeyIdentifierHashAlgorithm").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct OCSPRequestFlag(pub i32);
impl windows_core::TypeKind for OCSPRequestFlag {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for OCSPRequestFlag {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OCSPRequestFlag").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct OCSPSigningFlag(pub i32);
impl windows_core::TypeKind for OCSPSigningFlag {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for OCSPSigningFlag {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OCSPSigningFlag").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ObjectIdGroupId(pub i32);
impl windows_core::TypeKind for ObjectIdGroupId {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ObjectIdGroupId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ObjectIdGroupId").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ObjectIdPublicKeyFlags(pub i32);
impl windows_core::TypeKind for ObjectIdPublicKeyFlags {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ObjectIdPublicKeyFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ObjectIdPublicKeyFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PENDING_REQUEST_DESIRED_PROPERTY(pub i32);
impl windows_core::TypeKind for PENDING_REQUEST_DESIRED_PROPERTY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PENDING_REQUEST_DESIRED_PROPERTY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PENDING_REQUEST_DESIRED_PROPERTY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PFXExportOptions(pub i32);
impl windows_core::TypeKind for PFXExportOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PFXExportOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PFXExportOptions").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct Pkcs10AllowedSignatureTypes(pub i32);
impl windows_core::TypeKind for Pkcs10AllowedSignatureTypes {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for Pkcs10AllowedSignatureTypes {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Pkcs10AllowedSignatureTypes").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PolicyQualifierType(pub i32);
impl windows_core::TypeKind for PolicyQualifierType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PolicyQualifierType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PolicyQualifierType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PolicyServerUrlFlags(pub i32);
impl windows_core::TypeKind for PolicyServerUrlFlags {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PolicyServerUrlFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PolicyServerUrlFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PolicyServerUrlPropertyID(pub i32);
impl windows_core::TypeKind for PolicyServerUrlPropertyID {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PolicyServerUrlPropertyID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PolicyServerUrlPropertyID").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RequestClientInfoClientId(pub i32);
impl windows_core::TypeKind for RequestClientInfoClientId {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RequestClientInfoClientId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RequestClientInfoClientId").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WebEnrollmentFlags(pub i32);
impl windows_core::TypeKind for WebEnrollmentFlags {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WebEnrollmentFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WebEnrollmentFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WebSecurityLevel(pub i32);
impl windows_core::TypeKind for WebSecurityLevel {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WebSecurityLevel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WebSecurityLevel").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct X500NameFlags(pub i32);
impl windows_core::TypeKind for X500NameFlags {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for X500NameFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("X500NameFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct X509CertificateEnrollmentContext(pub i32);
impl windows_core::TypeKind for X509CertificateEnrollmentContext {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for X509CertificateEnrollmentContext {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("X509CertificateEnrollmentContext").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct X509CertificateTemplateEnrollmentFlag(pub i32);
impl windows_core::TypeKind for X509CertificateTemplateEnrollmentFlag {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for X509CertificateTemplateEnrollmentFlag {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("X509CertificateTemplateEnrollmentFlag").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct X509CertificateTemplateGeneralFlag(pub i32);
impl windows_core::TypeKind for X509CertificateTemplateGeneralFlag {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for X509CertificateTemplateGeneralFlag {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("X509CertificateTemplateGeneralFlag").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct X509CertificateTemplatePrivateKeyFlag(pub i32);
impl windows_core::TypeKind for X509CertificateTemplatePrivateKeyFlag {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for X509CertificateTemplatePrivateKeyFlag {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("X509CertificateTemplatePrivateKeyFlag").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct X509CertificateTemplateSubjectNameFlag(pub i32);
impl windows_core::TypeKind for X509CertificateTemplateSubjectNameFlag {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for X509CertificateTemplateSubjectNameFlag {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("X509CertificateTemplateSubjectNameFlag").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct X509EnrollmentAuthFlags(pub i32);
impl windows_core::TypeKind for X509EnrollmentAuthFlags {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for X509EnrollmentAuthFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("X509EnrollmentAuthFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct X509EnrollmentPolicyExportFlags(pub i32);
impl windows_core::TypeKind for X509EnrollmentPolicyExportFlags {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for X509EnrollmentPolicyExportFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("X509EnrollmentPolicyExportFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct X509EnrollmentPolicyLoadOption(pub i32);
impl windows_core::TypeKind for X509EnrollmentPolicyLoadOption {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for X509EnrollmentPolicyLoadOption {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("X509EnrollmentPolicyLoadOption").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct X509HardwareKeyUsageFlags(pub i32);
impl windows_core::TypeKind for X509HardwareKeyUsageFlags {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for X509HardwareKeyUsageFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("X509HardwareKeyUsageFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct X509KeyParametersExportType(pub i32);
impl windows_core::TypeKind for X509KeyParametersExportType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for X509KeyParametersExportType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("X509KeyParametersExportType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct X509KeySpec(pub i32);
impl windows_core::TypeKind for X509KeySpec {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for X509KeySpec {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("X509KeySpec").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct X509KeyUsageFlags(pub i32);
impl windows_core::TypeKind for X509KeyUsageFlags {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for X509KeyUsageFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("X509KeyUsageFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct X509PrivateKeyExportFlags(pub i32);
impl windows_core::TypeKind for X509PrivateKeyExportFlags {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for X509PrivateKeyExportFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("X509PrivateKeyExportFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct X509PrivateKeyProtection(pub i32);
impl windows_core::TypeKind for X509PrivateKeyProtection {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for X509PrivateKeyProtection {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("X509PrivateKeyProtection").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct X509PrivateKeyUsageFlags(pub i32);
impl windows_core::TypeKind for X509PrivateKeyUsageFlags {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for X509PrivateKeyUsageFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("X509PrivateKeyUsageFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct X509PrivateKeyVerify(pub i32);
impl windows_core::TypeKind for X509PrivateKeyVerify {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for X509PrivateKeyVerify {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("X509PrivateKeyVerify").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct X509ProviderType(pub i32);
impl windows_core::TypeKind for X509ProviderType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for X509ProviderType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("X509ProviderType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct X509RequestInheritOptions(pub i32);
impl windows_core::TypeKind for X509RequestInheritOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for X509RequestInheritOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("X509RequestInheritOptions").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct X509RequestType(pub i32);
impl windows_core::TypeKind for X509RequestType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for X509RequestType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("X509RequestType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct X509SCEPDisposition(pub i32);
impl windows_core::TypeKind for X509SCEPDisposition {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for X509SCEPDisposition {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("X509SCEPDisposition").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct X509SCEPFailInfo(pub i32);
impl windows_core::TypeKind for X509SCEPFailInfo {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for X509SCEPFailInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("X509SCEPFailInfo").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct X509SCEPMessageType(pub i32);
impl windows_core::TypeKind for X509SCEPMessageType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for X509SCEPMessageType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("X509SCEPMessageType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct X509SCEPProcessMessageFlags(pub i32);
impl windows_core::TypeKind for X509SCEPProcessMessageFlags {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for X509SCEPProcessMessageFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("X509SCEPProcessMessageFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct XEKL_KEYSIZE(pub i32);
impl windows_core::TypeKind for XEKL_KEYSIZE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for XEKL_KEYSIZE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("XEKL_KEYSIZE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct XEKL_KEYSPEC(pub i32);
impl windows_core::TypeKind for XEKL_KEYSPEC {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for XEKL_KEYSPEC {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("XEKL_KEYSPEC").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CAINFO {
    pub cbSize: u32,
    pub CAType: ENUM_CATYPES,
    pub cCASignatureCerts: u32,
    pub cCAExchangeCerts: u32,
    pub cExitModules: u32,
    pub lPropIdMax: i32,
    pub lRoleSeparationEnabled: i32,
    pub cKRACertUsedCount: u32,
    pub cKRACertCount: u32,
    pub fAdvancedServer: u32,
}
impl windows_core::TypeKind for CAINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for CAINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CAlternativeName: windows_core::GUID = windows_core::GUID::from_u128(0x884e2013_217d_11da_b2a4_000e7bbb2b09);
pub const CAlternativeNames: windows_core::GUID = windows_core::GUID::from_u128(0x884e2014_217d_11da_b2a4_000e7bbb2b09);
pub const CBinaryConverter: windows_core::GUID = windows_core::GUID::from_u128(0x884e2002_217d_11da_b2a4_000e7bbb2b09);
pub const CCertAdmin: windows_core::GUID = windows_core::GUID::from_u128(0x37eabaf0_7fb6_11d0_8817_00a0c903b83c);
pub const CCertConfig: windows_core::GUID = windows_core::GUID::from_u128(0x372fce38_4324_11d0_8810_00a0c903b83c);
pub const CCertEncodeAltName: windows_core::GUID = windows_core::GUID::from_u128(0x1cfc4cda_1271_11d1_9bd4_00c04fb683fa);
pub const CCertEncodeBitString: windows_core::GUID = windows_core::GUID::from_u128(0x6d6b3cd8_1278_11d1_9bd4_00c04fb683fa);
pub const CCertEncodeCRLDistInfo: windows_core::GUID = windows_core::GUID::from_u128(0x01fa60a0_bbff_11d0_8825_00a0c903b83c);
pub const CCertEncodeDateArray: windows_core::GUID = windows_core::GUID::from_u128(0x301f77b0_a470_11d0_8821_00a0c903b83c);
pub const CCertEncodeLongArray: windows_core::GUID = windows_core::GUID::from_u128(0x4e0680a0_a0a2_11d0_8821_00a0c903b83c);
pub const CCertEncodeStringArray: windows_core::GUID = windows_core::GUID::from_u128(0x19a76fe0_7494_11d0_8816_00a0c903b83c);
pub const CCertGetConfig: windows_core::GUID = windows_core::GUID::from_u128(0xc6cc49b0_ce17_11d0_8833_00a0c903b83c);
pub const CCertProperties: windows_core::GUID = windows_core::GUID::from_u128(0x884e202f_217d_11da_b2a4_000e7bbb2b09);
pub const CCertProperty: windows_core::GUID = windows_core::GUID::from_u128(0x884e202e_217d_11da_b2a4_000e7bbb2b09);
pub const CCertPropertyArchived: windows_core::GUID = windows_core::GUID::from_u128(0x884e2037_217d_11da_b2a4_000e7bbb2b09);
pub const CCertPropertyArchivedKeyHash: windows_core::GUID = windows_core::GUID::from_u128(0x884e203b_217d_11da_b2a4_000e7bbb2b09);
pub const CCertPropertyAutoEnroll: windows_core::GUID = windows_core::GUID::from_u128(0x884e2032_217d_11da_b2a4_000e7bbb2b09);
pub const CCertPropertyBackedUp: windows_core::GUID = windows_core::GUID::from_u128(0x884e2038_217d_11da_b2a4_000e7bbb2b09);
pub const CCertPropertyDescription: windows_core::GUID = windows_core::GUID::from_u128(0x884e2031_217d_11da_b2a4_000e7bbb2b09);
pub const CCertPropertyEnrollment: windows_core::GUID = windows_core::GUID::from_u128(0x884e2039_217d_11da_b2a4_000e7bbb2b09);
pub const CCertPropertyEnrollmentPolicyServer: windows_core::GUID = windows_core::GUID::from_u128(0x884e204c_217d_11da_b2a4_000e7bbb2b09);
pub const CCertPropertyFriendlyName: windows_core::GUID = windows_core::GUID::from_u128(0x884e2030_217d_11da_b2a4_000e7bbb2b09);
pub const CCertPropertyKeyProvInfo: windows_core::GUID = windows_core::GUID::from_u128(0x884e2036_217d_11da_b2a4_000e7bbb2b09);
pub const CCertPropertyRenewal: windows_core::GUID = windows_core::GUID::from_u128(0x884e203a_217d_11da_b2a4_000e7bbb2b09);
pub const CCertPropertyRequestOriginator: windows_core::GUID = windows_core::GUID::from_u128(0x884e2033_217d_11da_b2a4_000e7bbb2b09);
pub const CCertPropertySHA1Hash: windows_core::GUID = windows_core::GUID::from_u128(0x884e2034_217d_11da_b2a4_000e7bbb2b09);
pub const CCertRequest: windows_core::GUID = windows_core::GUID::from_u128(0x98aff3f0_5524_11d0_8812_00a0c903b83c);
pub const CCertServerExit: windows_core::GUID = windows_core::GUID::from_u128(0x4c4a5e40_732c_11d0_8816_00a0c903b83c);
pub const CCertServerPolicy: windows_core::GUID = windows_core::GUID::from_u128(0xaa000926_ffbe_11cf_8800_00a0c903b83c);
pub const CCertView: windows_core::GUID = windows_core::GUID::from_u128(0xa12d0f7a_1e84_11d1_9bd6_00c04fb683fa);
pub const CCertificateAttestationChallenge: windows_core::GUID = windows_core::GUID::from_u128(0x1362ada1_eb60_456a_b6e1_118050db741b);
pub const CCertificatePolicies: windows_core::GUID = windows_core::GUID::from_u128(0x884e201f_217d_11da_b2a4_000e7bbb2b09);
pub const CCertificatePolicy: windows_core::GUID = windows_core::GUID::from_u128(0x884e201e_217d_11da_b2a4_000e7bbb2b09);
pub const CCryptAttribute: windows_core::GUID = windows_core::GUID::from_u128(0x884e202c_217d_11da_b2a4_000e7bbb2b09);
pub const CCryptAttributes: windows_core::GUID = windows_core::GUID::from_u128(0x884e202d_217d_11da_b2a4_000e7bbb2b09);
pub const CCspInformation: windows_core::GUID = windows_core::GUID::from_u128(0x884e2007_217d_11da_b2a4_000e7bbb2b09);
pub const CCspInformations: windows_core::GUID = windows_core::GUID::from_u128(0x884e2008_217d_11da_b2a4_000e7bbb2b09);
pub const CCspStatus: windows_core::GUID = windows_core::GUID::from_u128(0x884e2009_217d_11da_b2a4_000e7bbb2b09);
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CERTTRANSBLOB {
    pub cb: u32,
    pub pb: *mut u8,
}
impl windows_core::TypeKind for CERTTRANSBLOB {
    type TypeKind = windows_core::CopyType;
}
impl Default for CERTTRANSBLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CERTVIEWRESTRICTION {
    pub ColumnIndex: u32,
    pub SeekOperator: i32,
    pub SortOrder: i32,
    pub pbValue: *mut u8,
    pub cbValue: u32,
}
impl windows_core::TypeKind for CERTVIEWRESTRICTION {
    type TypeKind = windows_core::CopyType;
}
impl Default for CERTVIEWRESTRICTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CEnroll: windows_core::GUID = windows_core::GUID::from_u128(0x43f8f289_7a20_11d0_8f06_00c04fc295e1);
pub const CEnroll2: windows_core::GUID = windows_core::GUID::from_u128(0x127698e4_e730_4e5c_a2b1_21490a70c8a1);
pub const CObjectId: windows_core::GUID = windows_core::GUID::from_u128(0x884e2000_217d_11da_b2a4_000e7bbb2b09);
pub const CObjectIds: windows_core::GUID = windows_core::GUID::from_u128(0x884e2001_217d_11da_b2a4_000e7bbb2b09);
pub const CPolicyQualifier: windows_core::GUID = windows_core::GUID::from_u128(0x884e201c_217d_11da_b2a4_000e7bbb2b09);
pub const CPolicyQualifiers: windows_core::GUID = windows_core::GUID::from_u128(0x884e201d_217d_11da_b2a4_000e7bbb2b09);
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CSEDB_RSTMAPW {
    pub pwszDatabaseName: windows_core::PWSTR,
    pub pwszNewDatabaseName: windows_core::PWSTR,
}
impl windows_core::TypeKind for CSEDB_RSTMAPW {
    type TypeKind = windows_core::CopyType;
}
impl Default for CSEDB_RSTMAPW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CSignerCertificate: windows_core::GUID = windows_core::GUID::from_u128(0x884e203d_217d_11da_b2a4_000e7bbb2b09);
pub const CSmimeCapabilities: windows_core::GUID = windows_core::GUID::from_u128(0x884e201a_217d_11da_b2a4_000e7bbb2b09);
pub const CSmimeCapability: windows_core::GUID = windows_core::GUID::from_u128(0x884e2019_217d_11da_b2a4_000e7bbb2b09);
pub const CX500DistinguishedName: windows_core::GUID = windows_core::GUID::from_u128(0x884e2003_217d_11da_b2a4_000e7bbb2b09);
pub const CX509Attribute: windows_core::GUID = windows_core::GUID::from_u128(0x884e2022_217d_11da_b2a4_000e7bbb2b09);
pub const CX509AttributeArchiveKey: windows_core::GUID = windows_core::GUID::from_u128(0x884e2027_217d_11da_b2a4_000e7bbb2b09);
pub const CX509AttributeArchiveKeyHash: windows_core::GUID = windows_core::GUID::from_u128(0x884e2028_217d_11da_b2a4_000e7bbb2b09);
pub const CX509AttributeClientId: windows_core::GUID = windows_core::GUID::from_u128(0x884e2025_217d_11da_b2a4_000e7bbb2b09);
pub const CX509AttributeCspProvider: windows_core::GUID = windows_core::GUID::from_u128(0x884e202b_217d_11da_b2a4_000e7bbb2b09);
pub const CX509AttributeExtensions: windows_core::GUID = windows_core::GUID::from_u128(0x884e2024_217d_11da_b2a4_000e7bbb2b09);
pub const CX509AttributeOSVersion: windows_core::GUID = windows_core::GUID::from_u128(0x884e202a_217d_11da_b2a4_000e7bbb2b09);
pub const CX509AttributeRenewalCertificate: windows_core::GUID = windows_core::GUID::from_u128(0x884e2026_217d_11da_b2a4_000e7bbb2b09);
pub const CX509Attributes: windows_core::GUID = windows_core::GUID::from_u128(0x884e2023_217d_11da_b2a4_000e7bbb2b09);
pub const CX509CertificateRequestCertificate: windows_core::GUID = windows_core::GUID::from_u128(0x884e2043_217d_11da_b2a4_000e7bbb2b09);
pub const CX509CertificateRequestCmc: windows_core::GUID = windows_core::GUID::from_u128(0x884e2045_217d_11da_b2a4_000e7bbb2b09);
pub const CX509CertificateRequestPkcs10: windows_core::GUID = windows_core::GUID::from_u128(0x884e2042_217d_11da_b2a4_000e7bbb2b09);
pub const CX509CertificateRequestPkcs7: windows_core::GUID = windows_core::GUID::from_u128(0x884e2044_217d_11da_b2a4_000e7bbb2b09);
pub const CX509CertificateRevocationList: windows_core::GUID = windows_core::GUID::from_u128(0x884e2060_217d_11da_b2a4_000e7bbb2b09);
pub const CX509CertificateRevocationListEntries: windows_core::GUID = windows_core::GUID::from_u128(0x884e205f_217d_11da_b2a4_000e7bbb2b09);
pub const CX509CertificateRevocationListEntry: windows_core::GUID = windows_core::GUID::from_u128(0x884e205e_217d_11da_b2a4_000e7bbb2b09);
pub const CX509CertificateTemplateADWritable: windows_core::GUID = windows_core::GUID::from_u128(0x8336e323_2e6a_4a04_937c_548f681839b3);
pub const CX509EndorsementKey: windows_core::GUID = windows_core::GUID::from_u128(0x11a25a1d_b9a3_4edd_af83_3b59adbed361);
pub const CX509Enrollment: windows_core::GUID = windows_core::GUID::from_u128(0x884e2046_217d_11da_b2a4_000e7bbb2b09);
pub const CX509EnrollmentHelper: windows_core::GUID = windows_core::GUID::from_u128(0x884e2050_217d_11da_b2a4_000e7bbb2b09);
pub const CX509EnrollmentPolicyActiveDirectory: windows_core::GUID = windows_core::GUID::from_u128(0x91f39027_217f_11da_b2a4_000e7bbb2b09);
pub const CX509EnrollmentPolicyWebService: windows_core::GUID = windows_core::GUID::from_u128(0x91f39028_217f_11da_b2a4_000e7bbb2b09);
pub const CX509EnrollmentWebClassFactory: windows_core::GUID = windows_core::GUID::from_u128(0x884e2049_217d_11da_b2a4_000e7bbb2b09);
pub const CX509Extension: windows_core::GUID = windows_core::GUID::from_u128(0x884e200d_217d_11da_b2a4_000e7bbb2b09);
pub const CX509ExtensionAlternativeNames: windows_core::GUID = windows_core::GUID::from_u128(0x884e2015_217d_11da_b2a4_000e7bbb2b09);
pub const CX509ExtensionAuthorityKeyIdentifier: windows_core::GUID = windows_core::GUID::from_u128(0x884e2018_217d_11da_b2a4_000e7bbb2b09);
pub const CX509ExtensionBasicConstraints: windows_core::GUID = windows_core::GUID::from_u128(0x884e2016_217d_11da_b2a4_000e7bbb2b09);
pub const CX509ExtensionCertificatePolicies: windows_core::GUID = windows_core::GUID::from_u128(0x884e2020_217d_11da_b2a4_000e7bbb2b09);
pub const CX509ExtensionEnhancedKeyUsage: windows_core::GUID = windows_core::GUID::from_u128(0x884e2010_217d_11da_b2a4_000e7bbb2b09);
pub const CX509ExtensionKeyUsage: windows_core::GUID = windows_core::GUID::from_u128(0x884e200f_217d_11da_b2a4_000e7bbb2b09);
pub const CX509ExtensionMSApplicationPolicies: windows_core::GUID = windows_core::GUID::from_u128(0x884e2021_217d_11da_b2a4_000e7bbb2b09);
pub const CX509ExtensionSmimeCapabilities: windows_core::GUID = windows_core::GUID::from_u128(0x884e201b_217d_11da_b2a4_000e7bbb2b09);
pub const CX509ExtensionSubjectKeyIdentifier: windows_core::GUID = windows_core::GUID::from_u128(0x884e2017_217d_11da_b2a4_000e7bbb2b09);
pub const CX509ExtensionTemplate: windows_core::GUID = windows_core::GUID::from_u128(0x884e2012_217d_11da_b2a4_000e7bbb2b09);
pub const CX509ExtensionTemplateName: windows_core::GUID = windows_core::GUID::from_u128(0x884e2011_217d_11da_b2a4_000e7bbb2b09);
pub const CX509Extensions: windows_core::GUID = windows_core::GUID::from_u128(0x884e200e_217d_11da_b2a4_000e7bbb2b09);
pub const CX509MachineEnrollmentFactory: windows_core::GUID = windows_core::GUID::from_u128(0x884e2051_217d_11da_b2a4_000e7bbb2b09);
pub const CX509NameValuePair: windows_core::GUID = windows_core::GUID::from_u128(0x884e203f_217d_11da_b2a4_000e7bbb2b09);
pub const CX509PolicyServerListManager: windows_core::GUID = windows_core::GUID::from_u128(0x91f39029_217f_11da_b2a4_000e7bbb2b09);
pub const CX509PolicyServerUrl: windows_core::GUID = windows_core::GUID::from_u128(0x91f3902a_217f_11da_b2a4_000e7bbb2b09);
pub const CX509PrivateKey: windows_core::GUID = windows_core::GUID::from_u128(0x884e200c_217d_11da_b2a4_000e7bbb2b09);
pub const CX509PublicKey: windows_core::GUID = windows_core::GUID::from_u128(0x884e200b_217d_11da_b2a4_000e7bbb2b09);
pub const CX509SCEPEnrollment: windows_core::GUID = windows_core::GUID::from_u128(0x884e2061_217d_11da_b2a4_000e7bbb2b09);
pub const CX509SCEPEnrollmentHelper: windows_core::GUID = windows_core::GUID::from_u128(0x884e2062_217d_11da_b2a4_000e7bbb2b09);
pub const OCSPAdmin: windows_core::GUID = windows_core::GUID::from_u128(0xd3f73511_92c9_47cb_8ff2_8d891a7c4de4);
pub const OCSPPropertyCollection: windows_core::GUID = windows_core::GUID::from_u128(0xf935a528_ba8a_4dd9_ba79_f283275cb2de);
pub type FNCERTSRVBACKUPCLOSE = Option<unsafe extern "system" fn(hbc: *mut core::ffi::c_void) -> windows_core::HRESULT>;
pub type FNCERTSRVBACKUPEND = Option<unsafe extern "system" fn(hbc: *mut core::ffi::c_void) -> windows_core::HRESULT>;
pub type FNCERTSRVBACKUPFREE = Option<unsafe extern "system" fn(pv: *mut core::ffi::c_void)>;
pub type FNCERTSRVBACKUPGETBACKUPLOGSW = Option<unsafe extern "system" fn(hbc: *mut core::ffi::c_void, ppwszzbackuplogfiles: *mut *mut u16, pcbsize: *mut u32) -> windows_core::HRESULT>;
pub type FNCERTSRVBACKUPGETDATABASENAMESW = Option<unsafe extern "system" fn(hbc: *mut core::ffi::c_void, ppwszzattachmentinformation: *mut *mut u16, pcbsize: *mut u32) -> windows_core::HRESULT>;
pub type FNCERTSRVBACKUPGETDYNAMICFILELISTW = Option<unsafe extern "system" fn(hbc: *const core::ffi::c_void, ppwszzfilelist: *mut *mut u16, pcbsize: *mut u32) -> windows_core::HRESULT>;
pub type FNCERTSRVBACKUPOPENFILEW = Option<unsafe extern "system" fn(hbc: *mut core::ffi::c_void, pwszattachmentname: windows_core::PCWSTR, cbreadhintsize: u32, plifilesize: *mut i64) -> windows_core::HRESULT>;
pub type FNCERTSRVBACKUPPREPAREW = Option<unsafe extern "system" fn(pwszservername: windows_core::PCWSTR, grbitjet: u32, dwbackupflags: u32, phbc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT>;
pub type FNCERTSRVBACKUPREAD = Option<unsafe extern "system" fn(hbc: *mut core::ffi::c_void, pvbuffer: *mut core::ffi::c_void, cbbuffer: u32, pcbread: *mut u32) -> windows_core::HRESULT>;
pub type FNCERTSRVBACKUPTRUNCATELOGS = Option<unsafe extern "system" fn(hbc: *mut core::ffi::c_void) -> windows_core::HRESULT>;
pub type FNCERTSRVISSERVERONLINEW = Option<unsafe extern "system" fn(pwszservername: windows_core::PCWSTR, pfserveronline: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT>;
pub type FNCERTSRVRESTOREEND = Option<unsafe extern "system" fn(hbc: *mut core::ffi::c_void) -> windows_core::HRESULT>;
pub type FNCERTSRVRESTOREGETDATABASELOCATIONSW = Option<unsafe extern "system" fn(hbc: *mut core::ffi::c_void, ppwszzdatabaselocationlist: *mut *mut u16, pcbsize: *mut u32) -> windows_core::HRESULT>;
pub type FNCERTSRVRESTOREPREPAREW = Option<unsafe extern "system" fn(pwszservername: windows_core::PCWSTR, dwrestoreflags: u32, phbc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT>;
pub type FNCERTSRVRESTOREREGISTERCOMPLETE = Option<unsafe extern "system" fn(hbc: *mut core::ffi::c_void, hrrestorestate: windows_core::HRESULT) -> windows_core::HRESULT>;
pub type FNCERTSRVRESTOREREGISTERW = Option<unsafe extern "system" fn(hbc: *mut core::ffi::c_void, pwszcheckpointfilepath: windows_core::PCWSTR, pwszlogpath: windows_core::PCWSTR, rgrstmap: *mut CSEDB_RSTMAPW, crstmap: i32, pwszbackuplogpath: windows_core::PCWSTR, genlow: u32, genhigh: u32) -> windows_core::HRESULT>;
pub type FNCERTSRVSERVERCONTROLW = Option<unsafe extern "system" fn(pwszservername: windows_core::PCWSTR, dwcontrolflags: u32, pcbout: *mut u32, ppbout: *mut *mut u8) -> windows_core::HRESULT>;
pub type FNIMPORTPFXTOPROVIDER = Option<unsafe extern "system" fn(hwndparent: super::super::super::Foundation::HWND, pbpfx: *const u8, cbpfx: u32, importflags: ImportPFXFlags, pwszpassword: windows_core::PCWSTR, pwszprovidername: windows_core::PCWSTR, pwszreadername: windows_core::PCWSTR, pwszcontainernameprefix: windows_core::PCWSTR, pwszpin: windows_core::PCWSTR, pwszfriendlyname: windows_core::PCWSTR, pccertout: *mut u32, prgpcertout: *mut *mut *mut super::CERT_CONTEXT) -> windows_core::HRESULT>;
pub type FNIMPORTPFXTOPROVIDERFREEDATA = Option<unsafe extern "system" fn(ccert: u32, rgpcert: *const *const super::CERT_CONTEXT)>;
#[cfg(feature = "implement")]
core::include!("impl.rs");
