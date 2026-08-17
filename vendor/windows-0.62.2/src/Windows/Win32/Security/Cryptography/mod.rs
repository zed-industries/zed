#[cfg(feature = "Win32_Security_Cryptography_Catalog")]
pub mod Catalog;
#[cfg(feature = "Win32_Security_Cryptography_Certificates")]
pub mod Certificates;
#[cfg(feature = "Win32_Security_Cryptography_Sip")]
pub mod Sip;
#[cfg(feature = "Win32_Security_Cryptography_UI")]
pub mod UI;
#[inline]
pub unsafe fn BCryptAddContextFunction<P1, P3>(dwtable: BCRYPT_TABLE, pszcontext: P1, dwinterface: BCRYPT_INTERFACE, pszfunction: P3, dwposition: u32) -> super::super::Foundation::NTSTATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("bcrypt.dll" "system" fn BCryptAddContextFunction(dwtable : BCRYPT_TABLE, pszcontext : windows_core::PCWSTR, dwinterface : BCRYPT_INTERFACE, pszfunction : windows_core::PCWSTR, dwposition : u32) -> super::super::Foundation:: NTSTATUS);
    unsafe { BCryptAddContextFunction(dwtable, pszcontext.param().abi(), dwinterface, pszfunction.param().abi(), dwposition) }
}
#[inline]
pub unsafe fn BCryptAddContextFunctionProvider<P1, P3, P4>(dwtable: u32, pszcontext: P1, dwinterface: u32, pszfunction: P3, pszprovider: P4, dwposition: u32) -> super::super::Foundation::NTSTATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("bcrypt.dll" "system" fn BCryptAddContextFunctionProvider(dwtable : u32, pszcontext : windows_core::PCWSTR, dwinterface : u32, pszfunction : windows_core::PCWSTR, pszprovider : windows_core::PCWSTR, dwposition : u32) -> super::super::Foundation:: NTSTATUS);
    unsafe { BCryptAddContextFunctionProvider(dwtable, pszcontext.param().abi(), dwinterface, pszfunction.param().abi(), pszprovider.param().abi(), dwposition) }
}
#[inline]
pub unsafe fn BCryptCloseAlgorithmProvider(halgorithm: BCRYPT_ALG_HANDLE, dwflags: u32) -> super::super::Foundation::NTSTATUS {
    windows_core::link!("bcrypt.dll" "system" fn BCryptCloseAlgorithmProvider(halgorithm : BCRYPT_ALG_HANDLE, dwflags : u32) -> super::super::Foundation:: NTSTATUS);
    unsafe { BCryptCloseAlgorithmProvider(halgorithm as _, dwflags) }
}
#[inline]
pub unsafe fn BCryptConfigureContext<P1>(dwtable: BCRYPT_TABLE, pszcontext: P1, pconfig: *const CRYPT_CONTEXT_CONFIG) -> super::super::Foundation::NTSTATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("bcrypt.dll" "system" fn BCryptConfigureContext(dwtable : BCRYPT_TABLE, pszcontext : windows_core::PCWSTR, pconfig : *const CRYPT_CONTEXT_CONFIG) -> super::super::Foundation:: NTSTATUS);
    unsafe { BCryptConfigureContext(dwtable, pszcontext.param().abi(), pconfig) }
}
#[inline]
pub unsafe fn BCryptConfigureContextFunction<P1, P3>(dwtable: BCRYPT_TABLE, pszcontext: P1, dwinterface: BCRYPT_INTERFACE, pszfunction: P3, pconfig: *const CRYPT_CONTEXT_FUNCTION_CONFIG) -> super::super::Foundation::NTSTATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("bcrypt.dll" "system" fn BCryptConfigureContextFunction(dwtable : BCRYPT_TABLE, pszcontext : windows_core::PCWSTR, dwinterface : BCRYPT_INTERFACE, pszfunction : windows_core::PCWSTR, pconfig : *const CRYPT_CONTEXT_FUNCTION_CONFIG) -> super::super::Foundation:: NTSTATUS);
    unsafe { BCryptConfigureContextFunction(dwtable, pszcontext.param().abi(), dwinterface, pszfunction.param().abi(), pconfig) }
}
#[inline]
pub unsafe fn BCryptCreateContext<P1>(dwtable: BCRYPT_TABLE, pszcontext: P1, pconfig: Option<*const CRYPT_CONTEXT_CONFIG>) -> super::super::Foundation::NTSTATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("bcrypt.dll" "system" fn BCryptCreateContext(dwtable : BCRYPT_TABLE, pszcontext : windows_core::PCWSTR, pconfig : *const CRYPT_CONTEXT_CONFIG) -> super::super::Foundation:: NTSTATUS);
    unsafe { BCryptCreateContext(dwtable, pszcontext.param().abi(), pconfig.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn BCryptCreateHash(halgorithm: BCRYPT_ALG_HANDLE, phhash: *mut BCRYPT_HASH_HANDLE, pbhashobject: Option<&mut [u8]>, pbsecret: Option<&[u8]>, dwflags: u32) -> super::super::Foundation::NTSTATUS {
    windows_core::link!("bcrypt.dll" "system" fn BCryptCreateHash(halgorithm : BCRYPT_ALG_HANDLE, phhash : *mut BCRYPT_HASH_HANDLE, pbhashobject : *mut u8, cbhashobject : u32, pbsecret : *const u8, cbsecret : u32, dwflags : u32) -> super::super::Foundation:: NTSTATUS);
    unsafe { BCryptCreateHash(halgorithm as _, phhash as _, core::mem::transmute(pbhashobject.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pbhashobject.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pbsecret.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pbsecret.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), dwflags) }
}
#[inline]
pub unsafe fn BCryptCreateMultiHash(halgorithm: BCRYPT_ALG_HANDLE, phhash: *mut BCRYPT_HASH_HANDLE, nhashes: u32, pbhashobject: Option<&mut [u8]>, pbsecret: Option<&[u8]>, dwflags: u32) -> super::super::Foundation::NTSTATUS {
    windows_core::link!("bcrypt.dll" "system" fn BCryptCreateMultiHash(halgorithm : BCRYPT_ALG_HANDLE, phhash : *mut BCRYPT_HASH_HANDLE, nhashes : u32, pbhashobject : *mut u8, cbhashobject : u32, pbsecret : *const u8, cbsecret : u32, dwflags : u32) -> super::super::Foundation:: NTSTATUS);
    unsafe { BCryptCreateMultiHash(halgorithm as _, phhash as _, nhashes, core::mem::transmute(pbhashobject.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pbhashobject.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pbsecret.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pbsecret.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), dwflags) }
}
#[inline]
pub unsafe fn BCryptDecrypt(hkey: BCRYPT_KEY_HANDLE, pbinput: Option<&[u8]>, ppaddinginfo: Option<*const core::ffi::c_void>, pbiv: Option<&mut [u8]>, pboutput: Option<&mut [u8]>, pcbresult: *mut u32, dwflags: BCRYPT_FLAGS) -> super::super::Foundation::NTSTATUS {
    windows_core::link!("bcrypt.dll" "system" fn BCryptDecrypt(hkey : BCRYPT_KEY_HANDLE, pbinput : *const u8, cbinput : u32, ppaddinginfo : *const core::ffi::c_void, pbiv : *mut u8, cbiv : u32, pboutput : *mut u8, cboutput : u32, pcbresult : *mut u32, dwflags : BCRYPT_FLAGS) -> super::super::Foundation:: NTSTATUS);
    unsafe {
        BCryptDecrypt(
            hkey as _,
            core::mem::transmute(pbinput.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
            pbinput.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
            ppaddinginfo.unwrap_or(core::mem::zeroed()) as _,
            core::mem::transmute(pbiv.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
            pbiv.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
            core::mem::transmute(pboutput.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
            pboutput.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
            pcbresult as _,
            dwflags,
        )
    }
}
#[inline]
pub unsafe fn BCryptDeleteContext<P1>(dwtable: BCRYPT_TABLE, pszcontext: P1) -> super::super::Foundation::NTSTATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("bcrypt.dll" "system" fn BCryptDeleteContext(dwtable : BCRYPT_TABLE, pszcontext : windows_core::PCWSTR) -> super::super::Foundation:: NTSTATUS);
    unsafe { BCryptDeleteContext(dwtable, pszcontext.param().abi()) }
}
#[inline]
pub unsafe fn BCryptDeriveKey<P1>(hsharedsecret: BCRYPT_SECRET_HANDLE, pwszkdf: P1, pparameterlist: Option<*const BCryptBufferDesc>, pbderivedkey: Option<&mut [u8]>, pcbresult: *mut u32, dwflags: u32) -> super::super::Foundation::NTSTATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("bcrypt.dll" "system" fn BCryptDeriveKey(hsharedsecret : BCRYPT_SECRET_HANDLE, pwszkdf : windows_core::PCWSTR, pparameterlist : *const BCryptBufferDesc, pbderivedkey : *mut u8, cbderivedkey : u32, pcbresult : *mut u32, dwflags : u32) -> super::super::Foundation:: NTSTATUS);
    unsafe { BCryptDeriveKey(hsharedsecret, pwszkdf.param().abi(), pparameterlist.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pbderivedkey.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pbderivedkey.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbresult as _, dwflags) }
}
#[inline]
pub unsafe fn BCryptDeriveKeyCapi(hhash: BCRYPT_HASH_HANDLE, htargetalg: Option<BCRYPT_ALG_HANDLE>, pbderivedkey: &mut [u8], dwflags: u32) -> super::super::Foundation::NTSTATUS {
    windows_core::link!("bcrypt.dll" "system" fn BCryptDeriveKeyCapi(hhash : BCRYPT_HASH_HANDLE, htargetalg : BCRYPT_ALG_HANDLE, pbderivedkey : *mut u8, cbderivedkey : u32, dwflags : u32) -> super::super::Foundation:: NTSTATUS);
    unsafe { BCryptDeriveKeyCapi(hhash, htargetalg.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pbderivedkey.as_ptr()), pbderivedkey.len().try_into().unwrap(), dwflags) }
}
#[inline]
pub unsafe fn BCryptDeriveKeyPBKDF2(hprf: BCRYPT_ALG_HANDLE, pbpassword: Option<&[u8]>, pbsalt: Option<&[u8]>, citerations: u64, pbderivedkey: &mut [u8], dwflags: u32) -> super::super::Foundation::NTSTATUS {
    windows_core::link!("bcrypt.dll" "system" fn BCryptDeriveKeyPBKDF2(hprf : BCRYPT_ALG_HANDLE, pbpassword : *const u8, cbpassword : u32, pbsalt : *const u8, cbsalt : u32, citerations : u64, pbderivedkey : *mut u8, cbderivedkey : u32, dwflags : u32) -> super::super::Foundation:: NTSTATUS);
    unsafe { BCryptDeriveKeyPBKDF2(hprf, core::mem::transmute(pbpassword.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pbpassword.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pbsalt.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pbsalt.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), citerations, core::mem::transmute(pbderivedkey.as_ptr()), pbderivedkey.len().try_into().unwrap(), dwflags) }
}
#[inline]
pub unsafe fn BCryptDestroyHash(hhash: BCRYPT_HASH_HANDLE) -> super::super::Foundation::NTSTATUS {
    windows_core::link!("bcrypt.dll" "system" fn BCryptDestroyHash(hhash : BCRYPT_HASH_HANDLE) -> super::super::Foundation:: NTSTATUS);
    unsafe { BCryptDestroyHash(hhash as _) }
}
#[inline]
pub unsafe fn BCryptDestroyKey(hkey: BCRYPT_KEY_HANDLE) -> super::super::Foundation::NTSTATUS {
    windows_core::link!("bcrypt.dll" "system" fn BCryptDestroyKey(hkey : BCRYPT_KEY_HANDLE) -> super::super::Foundation:: NTSTATUS);
    unsafe { BCryptDestroyKey(hkey as _) }
}
#[inline]
pub unsafe fn BCryptDestroySecret(hsecret: BCRYPT_SECRET_HANDLE) -> super::super::Foundation::NTSTATUS {
    windows_core::link!("bcrypt.dll" "system" fn BCryptDestroySecret(hsecret : BCRYPT_SECRET_HANDLE) -> super::super::Foundation:: NTSTATUS);
    unsafe { BCryptDestroySecret(hsecret as _) }
}
#[inline]
pub unsafe fn BCryptDuplicateHash(hhash: BCRYPT_HASH_HANDLE, phnewhash: *mut BCRYPT_HASH_HANDLE, pbhashobject: Option<&mut [u8]>, dwflags: u32) -> super::super::Foundation::NTSTATUS {
    windows_core::link!("bcrypt.dll" "system" fn BCryptDuplicateHash(hhash : BCRYPT_HASH_HANDLE, phnewhash : *mut BCRYPT_HASH_HANDLE, pbhashobject : *mut u8, cbhashobject : u32, dwflags : u32) -> super::super::Foundation:: NTSTATUS);
    unsafe { BCryptDuplicateHash(hhash, phnewhash as _, core::mem::transmute(pbhashobject.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pbhashobject.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), dwflags) }
}
#[inline]
pub unsafe fn BCryptDuplicateKey(hkey: BCRYPT_KEY_HANDLE, phnewkey: *mut BCRYPT_KEY_HANDLE, pbkeyobject: Option<&mut [u8]>, dwflags: u32) -> super::super::Foundation::NTSTATUS {
    windows_core::link!("bcrypt.dll" "system" fn BCryptDuplicateKey(hkey : BCRYPT_KEY_HANDLE, phnewkey : *mut BCRYPT_KEY_HANDLE, pbkeyobject : *mut u8, cbkeyobject : u32, dwflags : u32) -> super::super::Foundation:: NTSTATUS);
    unsafe { BCryptDuplicateKey(hkey, phnewkey as _, core::mem::transmute(pbkeyobject.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pbkeyobject.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), dwflags) }
}
#[inline]
pub unsafe fn BCryptEncrypt(hkey: BCRYPT_KEY_HANDLE, pbinput: Option<&[u8]>, ppaddinginfo: Option<*const core::ffi::c_void>, pbiv: Option<&mut [u8]>, pboutput: Option<&mut [u8]>, pcbresult: *mut u32, dwflags: BCRYPT_FLAGS) -> super::super::Foundation::NTSTATUS {
    windows_core::link!("bcrypt.dll" "system" fn BCryptEncrypt(hkey : BCRYPT_KEY_HANDLE, pbinput : *const u8, cbinput : u32, ppaddinginfo : *const core::ffi::c_void, pbiv : *mut u8, cbiv : u32, pboutput : *mut u8, cboutput : u32, pcbresult : *mut u32, dwflags : BCRYPT_FLAGS) -> super::super::Foundation:: NTSTATUS);
    unsafe {
        BCryptEncrypt(
            hkey as _,
            core::mem::transmute(pbinput.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
            pbinput.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
            ppaddinginfo.unwrap_or(core::mem::zeroed()) as _,
            core::mem::transmute(pbiv.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
            pbiv.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
            core::mem::transmute(pboutput.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
            pboutput.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
            pcbresult as _,
            dwflags,
        )
    }
}
#[inline]
pub unsafe fn BCryptEnumAlgorithms(dwalgoperations: BCRYPT_OPERATION, palgcount: *mut u32, ppalglist: *mut *mut BCRYPT_ALGORITHM_IDENTIFIER, dwflags: u32) -> super::super::Foundation::NTSTATUS {
    windows_core::link!("bcrypt.dll" "system" fn BCryptEnumAlgorithms(dwalgoperations : BCRYPT_OPERATION, palgcount : *mut u32, ppalglist : *mut *mut BCRYPT_ALGORITHM_IDENTIFIER, dwflags : u32) -> super::super::Foundation:: NTSTATUS);
    unsafe { BCryptEnumAlgorithms(dwalgoperations, palgcount as _, ppalglist as _, dwflags) }
}
#[inline]
pub unsafe fn BCryptEnumContextFunctionProviders<P1, P3>(dwtable: BCRYPT_TABLE, pszcontext: P1, dwinterface: BCRYPT_INTERFACE, pszfunction: P3, pcbbuffer: *mut u32, ppbuffer: Option<*mut *mut CRYPT_CONTEXT_FUNCTION_PROVIDERS>) -> super::super::Foundation::NTSTATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("bcrypt.dll" "system" fn BCryptEnumContextFunctionProviders(dwtable : BCRYPT_TABLE, pszcontext : windows_core::PCWSTR, dwinterface : BCRYPT_INTERFACE, pszfunction : windows_core::PCWSTR, pcbbuffer : *mut u32, ppbuffer : *mut *mut CRYPT_CONTEXT_FUNCTION_PROVIDERS) -> super::super::Foundation:: NTSTATUS);
    unsafe { BCryptEnumContextFunctionProviders(dwtable, pszcontext.param().abi(), dwinterface, pszfunction.param().abi(), pcbbuffer as _, ppbuffer.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn BCryptEnumContextFunctions<P1>(dwtable: BCRYPT_TABLE, pszcontext: P1, dwinterface: BCRYPT_INTERFACE, pcbbuffer: *mut u32, ppbuffer: Option<*mut *mut CRYPT_CONTEXT_FUNCTIONS>) -> super::super::Foundation::NTSTATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("bcrypt.dll" "system" fn BCryptEnumContextFunctions(dwtable : BCRYPT_TABLE, pszcontext : windows_core::PCWSTR, dwinterface : BCRYPT_INTERFACE, pcbbuffer : *mut u32, ppbuffer : *mut *mut CRYPT_CONTEXT_FUNCTIONS) -> super::super::Foundation:: NTSTATUS);
    unsafe { BCryptEnumContextFunctions(dwtable, pszcontext.param().abi(), dwinterface, pcbbuffer as _, ppbuffer.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn BCryptEnumContexts(dwtable: BCRYPT_TABLE, pcbbuffer: *mut u32, ppbuffer: Option<*mut *mut CRYPT_CONTEXTS>) -> super::super::Foundation::NTSTATUS {
    windows_core::link!("bcrypt.dll" "system" fn BCryptEnumContexts(dwtable : BCRYPT_TABLE, pcbbuffer : *mut u32, ppbuffer : *mut *mut CRYPT_CONTEXTS) -> super::super::Foundation:: NTSTATUS);
    unsafe { BCryptEnumContexts(dwtable, pcbbuffer as _, ppbuffer.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn BCryptEnumProviders<P0>(pszalgid: P0, pimplcount: *mut u32, ppimpllist: *mut *mut BCRYPT_PROVIDER_NAME, dwflags: u32) -> super::super::Foundation::NTSTATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("bcrypt.dll" "system" fn BCryptEnumProviders(pszalgid : windows_core::PCWSTR, pimplcount : *mut u32, ppimpllist : *mut *mut BCRYPT_PROVIDER_NAME, dwflags : u32) -> super::super::Foundation:: NTSTATUS);
    unsafe { BCryptEnumProviders(pszalgid.param().abi(), pimplcount as _, ppimpllist as _, dwflags) }
}
#[inline]
pub unsafe fn BCryptEnumRegisteredProviders(pcbbuffer: *mut u32, ppbuffer: Option<*mut *mut CRYPT_PROVIDERS>) -> super::super::Foundation::NTSTATUS {
    windows_core::link!("bcrypt.dll" "system" fn BCryptEnumRegisteredProviders(pcbbuffer : *mut u32, ppbuffer : *mut *mut CRYPT_PROVIDERS) -> super::super::Foundation:: NTSTATUS);
    unsafe { BCryptEnumRegisteredProviders(pcbbuffer as _, ppbuffer.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn BCryptExportKey<P2>(hkey: BCRYPT_KEY_HANDLE, hexportkey: Option<BCRYPT_KEY_HANDLE>, pszblobtype: P2, pboutput: Option<&mut [u8]>, pcbresult: *mut u32, dwflags: u32) -> super::super::Foundation::NTSTATUS
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("bcrypt.dll" "system" fn BCryptExportKey(hkey : BCRYPT_KEY_HANDLE, hexportkey : BCRYPT_KEY_HANDLE, pszblobtype : windows_core::PCWSTR, pboutput : *mut u8, cboutput : u32, pcbresult : *mut u32, dwflags : u32) -> super::super::Foundation:: NTSTATUS);
    unsafe { BCryptExportKey(hkey, hexportkey.unwrap_or(core::mem::zeroed()) as _, pszblobtype.param().abi(), core::mem::transmute(pboutput.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pboutput.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbresult as _, dwflags) }
}
#[inline]
pub unsafe fn BCryptFinalizeKeyPair(hkey: BCRYPT_KEY_HANDLE, dwflags: u32) -> super::super::Foundation::NTSTATUS {
    windows_core::link!("bcrypt.dll" "system" fn BCryptFinalizeKeyPair(hkey : BCRYPT_KEY_HANDLE, dwflags : u32) -> super::super::Foundation:: NTSTATUS);
    unsafe { BCryptFinalizeKeyPair(hkey as _, dwflags) }
}
#[inline]
pub unsafe fn BCryptFinishHash(hhash: BCRYPT_HASH_HANDLE, pboutput: &mut [u8], dwflags: u32) -> super::super::Foundation::NTSTATUS {
    windows_core::link!("bcrypt.dll" "system" fn BCryptFinishHash(hhash : BCRYPT_HASH_HANDLE, pboutput : *mut u8, cboutput : u32, dwflags : u32) -> super::super::Foundation:: NTSTATUS);
    unsafe { BCryptFinishHash(hhash as _, core::mem::transmute(pboutput.as_ptr()), pboutput.len().try_into().unwrap(), dwflags) }
}
#[inline]
pub unsafe fn BCryptFreeBuffer(pvbuffer: *const core::ffi::c_void) {
    windows_core::link!("bcrypt.dll" "system" fn BCryptFreeBuffer(pvbuffer : *const core::ffi::c_void));
    unsafe { BCryptFreeBuffer(pvbuffer) }
}
#[inline]
pub unsafe fn BCryptGenRandom(halgorithm: Option<BCRYPT_ALG_HANDLE>, pbbuffer: &mut [u8], dwflags: BCRYPTGENRANDOM_FLAGS) -> super::super::Foundation::NTSTATUS {
    windows_core::link!("bcrypt.dll" "system" fn BCryptGenRandom(halgorithm : BCRYPT_ALG_HANDLE, pbbuffer : *mut u8, cbbuffer : u32, dwflags : BCRYPTGENRANDOM_FLAGS) -> super::super::Foundation:: NTSTATUS);
    unsafe { BCryptGenRandom(halgorithm.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pbbuffer.as_ptr()), pbbuffer.len().try_into().unwrap(), dwflags) }
}
#[inline]
pub unsafe fn BCryptGenerateKeyPair(halgorithm: BCRYPT_ALG_HANDLE, phkey: *mut BCRYPT_KEY_HANDLE, dwlength: u32, dwflags: u32) -> super::super::Foundation::NTSTATUS {
    windows_core::link!("bcrypt.dll" "system" fn BCryptGenerateKeyPair(halgorithm : BCRYPT_ALG_HANDLE, phkey : *mut BCRYPT_KEY_HANDLE, dwlength : u32, dwflags : u32) -> super::super::Foundation:: NTSTATUS);
    unsafe { BCryptGenerateKeyPair(halgorithm as _, phkey as _, dwlength, dwflags) }
}
#[inline]
pub unsafe fn BCryptGenerateSymmetricKey(halgorithm: BCRYPT_ALG_HANDLE, phkey: *mut BCRYPT_KEY_HANDLE, pbkeyobject: Option<&mut [u8]>, pbsecret: &[u8], dwflags: u32) -> super::super::Foundation::NTSTATUS {
    windows_core::link!("bcrypt.dll" "system" fn BCryptGenerateSymmetricKey(halgorithm : BCRYPT_ALG_HANDLE, phkey : *mut BCRYPT_KEY_HANDLE, pbkeyobject : *mut u8, cbkeyobject : u32, pbsecret : *const u8, cbsecret : u32, dwflags : u32) -> super::super::Foundation:: NTSTATUS);
    unsafe { BCryptGenerateSymmetricKey(halgorithm as _, phkey as _, core::mem::transmute(pbkeyobject.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pbkeyobject.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pbsecret.as_ptr()), pbsecret.len().try_into().unwrap(), dwflags) }
}
#[inline]
pub unsafe fn BCryptGetFipsAlgorithmMode(pfenabled: *mut u8) -> super::super::Foundation::NTSTATUS {
    windows_core::link!("bcrypt.dll" "system" fn BCryptGetFipsAlgorithmMode(pfenabled : *mut u8) -> super::super::Foundation:: NTSTATUS);
    unsafe { BCryptGetFipsAlgorithmMode(pfenabled as _) }
}
#[inline]
pub unsafe fn BCryptGetProperty<P1>(hobject: BCRYPT_HANDLE, pszproperty: P1, pboutput: Option<&mut [u8]>, pcbresult: *mut u32, dwflags: u32) -> super::super::Foundation::NTSTATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("bcrypt.dll" "system" fn BCryptGetProperty(hobject : BCRYPT_HANDLE, pszproperty : windows_core::PCWSTR, pboutput : *mut u8, cboutput : u32, pcbresult : *mut u32, dwflags : u32) -> super::super::Foundation:: NTSTATUS);
    unsafe { BCryptGetProperty(hobject, pszproperty.param().abi(), core::mem::transmute(pboutput.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pboutput.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbresult as _, dwflags) }
}
#[inline]
pub unsafe fn BCryptHash(halgorithm: BCRYPT_ALG_HANDLE, pbsecret: Option<&[u8]>, pbinput: &[u8], pboutput: &mut [u8]) -> super::super::Foundation::NTSTATUS {
    windows_core::link!("bcrypt.dll" "system" fn BCryptHash(halgorithm : BCRYPT_ALG_HANDLE, pbsecret : *const u8, cbsecret : u32, pbinput : *const u8, cbinput : u32, pboutput : *mut u8, cboutput : u32) -> super::super::Foundation:: NTSTATUS);
    unsafe { BCryptHash(halgorithm as _, core::mem::transmute(pbsecret.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pbsecret.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pbinput.as_ptr()), pbinput.len().try_into().unwrap(), core::mem::transmute(pboutput.as_ptr()), pboutput.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn BCryptHashData(hhash: BCRYPT_HASH_HANDLE, pbinput: &[u8], dwflags: u32) -> super::super::Foundation::NTSTATUS {
    windows_core::link!("bcrypt.dll" "system" fn BCryptHashData(hhash : BCRYPT_HASH_HANDLE, pbinput : *const u8, cbinput : u32, dwflags : u32) -> super::super::Foundation:: NTSTATUS);
    unsafe { BCryptHashData(hhash as _, core::mem::transmute(pbinput.as_ptr()), pbinput.len().try_into().unwrap(), dwflags) }
}
#[inline]
pub unsafe fn BCryptImportKey<P2>(halgorithm: BCRYPT_ALG_HANDLE, himportkey: Option<BCRYPT_KEY_HANDLE>, pszblobtype: P2, phkey: *mut BCRYPT_KEY_HANDLE, pbkeyobject: Option<&mut [u8]>, pbinput: &[u8], dwflags: u32) -> super::super::Foundation::NTSTATUS
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("bcrypt.dll" "system" fn BCryptImportKey(halgorithm : BCRYPT_ALG_HANDLE, himportkey : BCRYPT_KEY_HANDLE, pszblobtype : windows_core::PCWSTR, phkey : *mut BCRYPT_KEY_HANDLE, pbkeyobject : *mut u8, cbkeyobject : u32, pbinput : *const u8, cbinput : u32, dwflags : u32) -> super::super::Foundation:: NTSTATUS);
    unsafe { BCryptImportKey(halgorithm, himportkey.unwrap_or(core::mem::zeroed()) as _, pszblobtype.param().abi(), phkey as _, core::mem::transmute(pbkeyobject.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pbkeyobject.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pbinput.as_ptr()), pbinput.len().try_into().unwrap(), dwflags) }
}
#[inline]
pub unsafe fn BCryptImportKeyPair<P2>(halgorithm: BCRYPT_ALG_HANDLE, himportkey: Option<BCRYPT_KEY_HANDLE>, pszblobtype: P2, phkey: *mut BCRYPT_KEY_HANDLE, pbinput: &[u8], dwflags: u32) -> super::super::Foundation::NTSTATUS
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("bcrypt.dll" "system" fn BCryptImportKeyPair(halgorithm : BCRYPT_ALG_HANDLE, himportkey : BCRYPT_KEY_HANDLE, pszblobtype : windows_core::PCWSTR, phkey : *mut BCRYPT_KEY_HANDLE, pbinput : *const u8, cbinput : u32, dwflags : u32) -> super::super::Foundation:: NTSTATUS);
    unsafe { BCryptImportKeyPair(halgorithm, himportkey.unwrap_or(core::mem::zeroed()) as _, pszblobtype.param().abi(), phkey as _, core::mem::transmute(pbinput.as_ptr()), pbinput.len().try_into().unwrap(), dwflags) }
}
#[inline]
pub unsafe fn BCryptKeyDerivation(hkey: BCRYPT_KEY_HANDLE, pparameterlist: Option<*const BCryptBufferDesc>, pbderivedkey: &mut [u8], pcbresult: *mut u32, dwflags: u32) -> super::super::Foundation::NTSTATUS {
    windows_core::link!("bcrypt.dll" "system" fn BCryptKeyDerivation(hkey : BCRYPT_KEY_HANDLE, pparameterlist : *const BCryptBufferDesc, pbderivedkey : *mut u8, cbderivedkey : u32, pcbresult : *mut u32, dwflags : u32) -> super::super::Foundation:: NTSTATUS);
    unsafe { BCryptKeyDerivation(hkey, pparameterlist.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pbderivedkey.as_ptr()), pbderivedkey.len().try_into().unwrap(), pcbresult as _, dwflags) }
}
#[inline]
pub unsafe fn BCryptOpenAlgorithmProvider<P1, P2>(phalgorithm: *mut BCRYPT_ALG_HANDLE, pszalgid: P1, pszimplementation: P2, dwflags: BCRYPT_OPEN_ALGORITHM_PROVIDER_FLAGS) -> super::super::Foundation::NTSTATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("bcrypt.dll" "system" fn BCryptOpenAlgorithmProvider(phalgorithm : *mut BCRYPT_ALG_HANDLE, pszalgid : windows_core::PCWSTR, pszimplementation : windows_core::PCWSTR, dwflags : BCRYPT_OPEN_ALGORITHM_PROVIDER_FLAGS) -> super::super::Foundation:: NTSTATUS);
    unsafe { BCryptOpenAlgorithmProvider(phalgorithm as _, pszalgid.param().abi(), pszimplementation.param().abi(), dwflags) }
}
#[inline]
pub unsafe fn BCryptProcessMultiOperations(hobject: BCRYPT_HANDLE, operationtype: BCRYPT_MULTI_OPERATION_TYPE, poperations: *const core::ffi::c_void, cboperations: u32, dwflags: u32) -> super::super::Foundation::NTSTATUS {
    windows_core::link!("bcrypt.dll" "system" fn BCryptProcessMultiOperations(hobject : BCRYPT_HANDLE, operationtype : BCRYPT_MULTI_OPERATION_TYPE, poperations : *const core::ffi::c_void, cboperations : u32, dwflags : u32) -> super::super::Foundation:: NTSTATUS);
    unsafe { BCryptProcessMultiOperations(hobject as _, operationtype, poperations, cboperations, dwflags) }
}
#[inline]
pub unsafe fn BCryptQueryContextConfiguration<P1>(dwtable: BCRYPT_TABLE, pszcontext: P1, pcbbuffer: *mut u32, ppbuffer: Option<*mut *mut CRYPT_CONTEXT_CONFIG>) -> super::super::Foundation::NTSTATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("bcrypt.dll" "system" fn BCryptQueryContextConfiguration(dwtable : BCRYPT_TABLE, pszcontext : windows_core::PCWSTR, pcbbuffer : *mut u32, ppbuffer : *mut *mut CRYPT_CONTEXT_CONFIG) -> super::super::Foundation:: NTSTATUS);
    unsafe { BCryptQueryContextConfiguration(dwtable, pszcontext.param().abi(), pcbbuffer as _, ppbuffer.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn BCryptQueryContextFunctionConfiguration<P1, P3>(dwtable: BCRYPT_TABLE, pszcontext: P1, dwinterface: BCRYPT_INTERFACE, pszfunction: P3, pcbbuffer: *mut u32, ppbuffer: Option<*mut *mut CRYPT_CONTEXT_FUNCTION_CONFIG>) -> super::super::Foundation::NTSTATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("bcrypt.dll" "system" fn BCryptQueryContextFunctionConfiguration(dwtable : BCRYPT_TABLE, pszcontext : windows_core::PCWSTR, dwinterface : BCRYPT_INTERFACE, pszfunction : windows_core::PCWSTR, pcbbuffer : *mut u32, ppbuffer : *mut *mut CRYPT_CONTEXT_FUNCTION_CONFIG) -> super::super::Foundation:: NTSTATUS);
    unsafe { BCryptQueryContextFunctionConfiguration(dwtable, pszcontext.param().abi(), dwinterface, pszfunction.param().abi(), pcbbuffer as _, ppbuffer.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn BCryptQueryContextFunctionProperty<P1, P3, P4>(dwtable: BCRYPT_TABLE, pszcontext: P1, dwinterface: BCRYPT_INTERFACE, pszfunction: P3, pszproperty: P4, pcbvalue: *mut u32, ppbvalue: Option<*mut *mut u8>) -> super::super::Foundation::NTSTATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("bcrypt.dll" "system" fn BCryptQueryContextFunctionProperty(dwtable : BCRYPT_TABLE, pszcontext : windows_core::PCWSTR, dwinterface : BCRYPT_INTERFACE, pszfunction : windows_core::PCWSTR, pszproperty : windows_core::PCWSTR, pcbvalue : *mut u32, ppbvalue : *mut *mut u8) -> super::super::Foundation:: NTSTATUS);
    unsafe { BCryptQueryContextFunctionProperty(dwtable, pszcontext.param().abi(), dwinterface, pszfunction.param().abi(), pszproperty.param().abi(), pcbvalue as _, ppbvalue.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn BCryptQueryProviderRegistration<P0>(pszprovider: P0, dwmode: BCRYPT_QUERY_PROVIDER_MODE, dwinterface: BCRYPT_INTERFACE, pcbbuffer: *mut u32, ppbuffer: Option<*mut *mut CRYPT_PROVIDER_REG>) -> super::super::Foundation::NTSTATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("bcrypt.dll" "system" fn BCryptQueryProviderRegistration(pszprovider : windows_core::PCWSTR, dwmode : BCRYPT_QUERY_PROVIDER_MODE, dwinterface : BCRYPT_INTERFACE, pcbbuffer : *mut u32, ppbuffer : *mut *mut CRYPT_PROVIDER_REG) -> super::super::Foundation:: NTSTATUS);
    unsafe { BCryptQueryProviderRegistration(pszprovider.param().abi(), dwmode, dwinterface, pcbbuffer as _, ppbuffer.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn BCryptRegisterConfigChangeNotify(phevent: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::NTSTATUS {
    windows_core::link!("bcrypt.dll" "system" fn BCryptRegisterConfigChangeNotify(phevent : *mut super::super::Foundation:: HANDLE) -> super::super::Foundation:: NTSTATUS);
    unsafe { BCryptRegisterConfigChangeNotify(phevent as _) }
}
#[inline]
pub unsafe fn BCryptRegisterProvider<P0>(pszprovider: P0, dwflags: u32, preg: *const CRYPT_PROVIDER_REG) -> super::super::Foundation::NTSTATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("bcrypt.dll" "system" fn BCryptRegisterProvider(pszprovider : windows_core::PCWSTR, dwflags : u32, preg : *const CRYPT_PROVIDER_REG) -> super::super::Foundation:: NTSTATUS);
    unsafe { BCryptRegisterProvider(pszprovider.param().abi(), dwflags, preg) }
}
#[inline]
pub unsafe fn BCryptRemoveContextFunction<P1, P3>(dwtable: BCRYPT_TABLE, pszcontext: P1, dwinterface: BCRYPT_INTERFACE, pszfunction: P3) -> super::super::Foundation::NTSTATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("bcrypt.dll" "system" fn BCryptRemoveContextFunction(dwtable : BCRYPT_TABLE, pszcontext : windows_core::PCWSTR, dwinterface : BCRYPT_INTERFACE, pszfunction : windows_core::PCWSTR) -> super::super::Foundation:: NTSTATUS);
    unsafe { BCryptRemoveContextFunction(dwtable, pszcontext.param().abi(), dwinterface, pszfunction.param().abi()) }
}
#[inline]
pub unsafe fn BCryptRemoveContextFunctionProvider<P1, P3, P4>(dwtable: u32, pszcontext: P1, dwinterface: u32, pszfunction: P3, pszprovider: P4) -> super::super::Foundation::NTSTATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("bcrypt.dll" "system" fn BCryptRemoveContextFunctionProvider(dwtable : u32, pszcontext : windows_core::PCWSTR, dwinterface : u32, pszfunction : windows_core::PCWSTR, pszprovider : windows_core::PCWSTR) -> super::super::Foundation:: NTSTATUS);
    unsafe { BCryptRemoveContextFunctionProvider(dwtable, pszcontext.param().abi(), dwinterface, pszfunction.param().abi(), pszprovider.param().abi()) }
}
#[inline]
pub unsafe fn BCryptResolveProviders<P0, P2, P3>(pszcontext: P0, dwinterface: Option<u32>, pszfunction: P2, pszprovider: P3, dwmode: BCRYPT_QUERY_PROVIDER_MODE, dwflags: BCRYPT_RESOLVE_PROVIDERS_FLAGS, pcbbuffer: *mut u32, ppbuffer: Option<*mut *mut CRYPT_PROVIDER_REFS>) -> super::super::Foundation::NTSTATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("bcrypt.dll" "system" fn BCryptResolveProviders(pszcontext : windows_core::PCWSTR, dwinterface : u32, pszfunction : windows_core::PCWSTR, pszprovider : windows_core::PCWSTR, dwmode : BCRYPT_QUERY_PROVIDER_MODE, dwflags : BCRYPT_RESOLVE_PROVIDERS_FLAGS, pcbbuffer : *mut u32, ppbuffer : *mut *mut CRYPT_PROVIDER_REFS) -> super::super::Foundation:: NTSTATUS);
    unsafe { BCryptResolveProviders(pszcontext.param().abi(), dwinterface.unwrap_or(core::mem::zeroed()) as _, pszfunction.param().abi(), pszprovider.param().abi(), dwmode, dwflags, pcbbuffer as _, ppbuffer.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn BCryptSecretAgreement(hprivkey: BCRYPT_KEY_HANDLE, hpubkey: BCRYPT_KEY_HANDLE, phagreedsecret: *mut BCRYPT_SECRET_HANDLE, dwflags: u32) -> super::super::Foundation::NTSTATUS {
    windows_core::link!("bcrypt.dll" "system" fn BCryptSecretAgreement(hprivkey : BCRYPT_KEY_HANDLE, hpubkey : BCRYPT_KEY_HANDLE, phagreedsecret : *mut BCRYPT_SECRET_HANDLE, dwflags : u32) -> super::super::Foundation:: NTSTATUS);
    unsafe { BCryptSecretAgreement(hprivkey, hpubkey, phagreedsecret as _, dwflags) }
}
#[inline]
pub unsafe fn BCryptSetContextFunctionProperty<P1, P3, P4>(dwtable: BCRYPT_TABLE, pszcontext: P1, dwinterface: BCRYPT_INTERFACE, pszfunction: P3, pszproperty: P4, pbvalue: Option<&[u8]>) -> super::super::Foundation::NTSTATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("bcrypt.dll" "system" fn BCryptSetContextFunctionProperty(dwtable : BCRYPT_TABLE, pszcontext : windows_core::PCWSTR, dwinterface : BCRYPT_INTERFACE, pszfunction : windows_core::PCWSTR, pszproperty : windows_core::PCWSTR, cbvalue : u32, pbvalue : *const u8) -> super::super::Foundation:: NTSTATUS);
    unsafe { BCryptSetContextFunctionProperty(dwtable, pszcontext.param().abi(), dwinterface, pszfunction.param().abi(), pszproperty.param().abi(), pbvalue.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pbvalue.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))) }
}
#[inline]
pub unsafe fn BCryptSetProperty<P1>(hobject: BCRYPT_HANDLE, pszproperty: P1, pbinput: &[u8], dwflags: u32) -> super::super::Foundation::NTSTATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("bcrypt.dll" "system" fn BCryptSetProperty(hobject : BCRYPT_HANDLE, pszproperty : windows_core::PCWSTR, pbinput : *const u8, cbinput : u32, dwflags : u32) -> super::super::Foundation:: NTSTATUS);
    unsafe { BCryptSetProperty(hobject as _, pszproperty.param().abi(), core::mem::transmute(pbinput.as_ptr()), pbinput.len().try_into().unwrap(), dwflags) }
}
#[inline]
pub unsafe fn BCryptSignHash(hkey: BCRYPT_KEY_HANDLE, ppaddinginfo: Option<*const core::ffi::c_void>, pbinput: &[u8], pboutput: Option<&mut [u8]>, pcbresult: *mut u32, dwflags: BCRYPT_FLAGS) -> super::super::Foundation::NTSTATUS {
    windows_core::link!("bcrypt.dll" "system" fn BCryptSignHash(hkey : BCRYPT_KEY_HANDLE, ppaddinginfo : *const core::ffi::c_void, pbinput : *const u8, cbinput : u32, pboutput : *mut u8, cboutput : u32, pcbresult : *mut u32, dwflags : BCRYPT_FLAGS) -> super::super::Foundation:: NTSTATUS);
    unsafe { BCryptSignHash(hkey, ppaddinginfo.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pbinput.as_ptr()), pbinput.len().try_into().unwrap(), core::mem::transmute(pboutput.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pboutput.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbresult as _, dwflags) }
}
#[inline]
pub unsafe fn BCryptUnregisterConfigChangeNotify(hevent: super::super::Foundation::HANDLE) -> super::super::Foundation::NTSTATUS {
    windows_core::link!("bcrypt.dll" "system" fn BCryptUnregisterConfigChangeNotify(hevent : super::super::Foundation:: HANDLE) -> super::super::Foundation:: NTSTATUS);
    unsafe { BCryptUnregisterConfigChangeNotify(hevent) }
}
#[inline]
pub unsafe fn BCryptUnregisterProvider<P0>(pszprovider: P0) -> super::super::Foundation::NTSTATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("bcrypt.dll" "system" fn BCryptUnregisterProvider(pszprovider : windows_core::PCWSTR) -> super::super::Foundation:: NTSTATUS);
    unsafe { BCryptUnregisterProvider(pszprovider.param().abi()) }
}
#[inline]
pub unsafe fn BCryptVerifySignature(hkey: BCRYPT_KEY_HANDLE, ppaddinginfo: Option<*const core::ffi::c_void>, pbhash: &[u8], pbsignature: &[u8], dwflags: BCRYPT_FLAGS) -> super::super::Foundation::NTSTATUS {
    windows_core::link!("bcrypt.dll" "system" fn BCryptVerifySignature(hkey : BCRYPT_KEY_HANDLE, ppaddinginfo : *const core::ffi::c_void, pbhash : *const u8, cbhash : u32, pbsignature : *const u8, cbsignature : u32, dwflags : BCRYPT_FLAGS) -> super::super::Foundation:: NTSTATUS);
    unsafe { BCryptVerifySignature(hkey, ppaddinginfo.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pbhash.as_ptr()), pbhash.len().try_into().unwrap(), core::mem::transmute(pbsignature.as_ptr()), pbsignature.len().try_into().unwrap(), dwflags) }
}
#[inline]
pub unsafe fn CertAddCRLContextToStore(hcertstore: Option<HCERTSTORE>, pcrlcontext: *const CRL_CONTEXT, dwadddisposition: u32, ppstorecontext: Option<*mut *mut CRL_CONTEXT>) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CertAddCRLContextToStore(hcertstore : HCERTSTORE, pcrlcontext : *const CRL_CONTEXT, dwadddisposition : u32, ppstorecontext : *mut *mut CRL_CONTEXT) -> windows_core::BOOL);
    unsafe { CertAddCRLContextToStore(hcertstore.unwrap_or(core::mem::zeroed()) as _, pcrlcontext, dwadddisposition, ppstorecontext.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CertAddCRLLinkToStore(hcertstore: HCERTSTORE, pcrlcontext: *const CRL_CONTEXT, dwadddisposition: u32, ppstorecontext: Option<*mut *mut CRL_CONTEXT>) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CertAddCRLLinkToStore(hcertstore : HCERTSTORE, pcrlcontext : *const CRL_CONTEXT, dwadddisposition : u32, ppstorecontext : *mut *mut CRL_CONTEXT) -> windows_core::BOOL);
    unsafe { CertAddCRLLinkToStore(hcertstore, pcrlcontext, dwadddisposition, ppstorecontext.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CertAddCTLContextToStore(hcertstore: Option<HCERTSTORE>, pctlcontext: *const CTL_CONTEXT, dwadddisposition: u32, ppstorecontext: Option<*mut *mut CTL_CONTEXT>) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CertAddCTLContextToStore(hcertstore : HCERTSTORE, pctlcontext : *const CTL_CONTEXT, dwadddisposition : u32, ppstorecontext : *mut *mut CTL_CONTEXT) -> windows_core::BOOL);
    unsafe { CertAddCTLContextToStore(hcertstore.unwrap_or(core::mem::zeroed()) as _, pctlcontext, dwadddisposition, ppstorecontext.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CertAddCTLLinkToStore(hcertstore: HCERTSTORE, pctlcontext: *const CTL_CONTEXT, dwadddisposition: u32, ppstorecontext: Option<*mut *mut CTL_CONTEXT>) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CertAddCTLLinkToStore(hcertstore : HCERTSTORE, pctlcontext : *const CTL_CONTEXT, dwadddisposition : u32, ppstorecontext : *mut *mut CTL_CONTEXT) -> windows_core::BOOL);
    unsafe { CertAddCTLLinkToStore(hcertstore, pctlcontext, dwadddisposition, ppstorecontext.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CertAddCertificateContextToStore(hcertstore: Option<HCERTSTORE>, pcertcontext: *const CERT_CONTEXT, dwadddisposition: u32, ppstorecontext: Option<*mut *mut CERT_CONTEXT>) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CertAddCertificateContextToStore(hcertstore : HCERTSTORE, pcertcontext : *const CERT_CONTEXT, dwadddisposition : u32, ppstorecontext : *mut *mut CERT_CONTEXT) -> windows_core::BOOL);
    unsafe { CertAddCertificateContextToStore(hcertstore.unwrap_or(core::mem::zeroed()) as _, pcertcontext, dwadddisposition, ppstorecontext.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CertAddCertificateLinkToStore(hcertstore: HCERTSTORE, pcertcontext: *const CERT_CONTEXT, dwadddisposition: u32, ppstorecontext: Option<*mut *mut CERT_CONTEXT>) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CertAddCertificateLinkToStore(hcertstore : HCERTSTORE, pcertcontext : *const CERT_CONTEXT, dwadddisposition : u32, ppstorecontext : *mut *mut CERT_CONTEXT) -> windows_core::BOOL);
    unsafe { CertAddCertificateLinkToStore(hcertstore, pcertcontext, dwadddisposition, ppstorecontext.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CertAddEncodedCRLToStore(hcertstore: Option<HCERTSTORE>, dwcertencodingtype: CERT_QUERY_ENCODING_TYPE, pbcrlencoded: &[u8], dwadddisposition: u32, ppcrlcontext: Option<*mut *mut CRL_CONTEXT>) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CertAddEncodedCRLToStore(hcertstore : HCERTSTORE, dwcertencodingtype : CERT_QUERY_ENCODING_TYPE, pbcrlencoded : *const u8, cbcrlencoded : u32, dwadddisposition : u32, ppcrlcontext : *mut *mut CRL_CONTEXT) -> windows_core::BOOL);
    unsafe { CertAddEncodedCRLToStore(hcertstore.unwrap_or(core::mem::zeroed()) as _, dwcertencodingtype, core::mem::transmute(pbcrlencoded.as_ptr()), pbcrlencoded.len().try_into().unwrap(), dwadddisposition, ppcrlcontext.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CertAddEncodedCTLToStore(hcertstore: Option<HCERTSTORE>, dwmsgandcertencodingtype: CERT_QUERY_ENCODING_TYPE, pbctlencoded: &[u8], dwadddisposition: u32, ppctlcontext: Option<*mut *mut CTL_CONTEXT>) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CertAddEncodedCTLToStore(hcertstore : HCERTSTORE, dwmsgandcertencodingtype : CERT_QUERY_ENCODING_TYPE, pbctlencoded : *const u8, cbctlencoded : u32, dwadddisposition : u32, ppctlcontext : *mut *mut CTL_CONTEXT) -> windows_core::BOOL);
    unsafe { CertAddEncodedCTLToStore(hcertstore.unwrap_or(core::mem::zeroed()) as _, dwmsgandcertencodingtype, core::mem::transmute(pbctlencoded.as_ptr()), pbctlencoded.len().try_into().unwrap(), dwadddisposition, ppctlcontext.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CertAddEncodedCertificateToStore(hcertstore: Option<HCERTSTORE>, dwcertencodingtype: CERT_QUERY_ENCODING_TYPE, pbcertencoded: &[u8], dwadddisposition: u32, ppcertcontext: Option<*mut *mut CERT_CONTEXT>) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CertAddEncodedCertificateToStore(hcertstore : HCERTSTORE, dwcertencodingtype : CERT_QUERY_ENCODING_TYPE, pbcertencoded : *const u8, cbcertencoded : u32, dwadddisposition : u32, ppcertcontext : *mut *mut CERT_CONTEXT) -> windows_core::BOOL);
    unsafe { CertAddEncodedCertificateToStore(hcertstore.unwrap_or(core::mem::zeroed()) as _, dwcertencodingtype, core::mem::transmute(pbcertencoded.as_ptr()), pbcertencoded.len().try_into().unwrap(), dwadddisposition, ppcertcontext.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CertAddEncodedCertificateToSystemStoreA<P0>(szcertstorename: P0, pbcertencoded: &[u8]) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("crypt32.dll" "system" fn CertAddEncodedCertificateToSystemStoreA(szcertstorename : windows_core::PCSTR, pbcertencoded : *const u8, cbcertencoded : u32) -> windows_core::BOOL);
    unsafe { CertAddEncodedCertificateToSystemStoreA(szcertstorename.param().abi(), core::mem::transmute(pbcertencoded.as_ptr()), pbcertencoded.len().try_into().unwrap()).ok() }
}
#[inline]
pub unsafe fn CertAddEncodedCertificateToSystemStoreW<P0>(szcertstorename: P0, pbcertencoded: &[u8]) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("crypt32.dll" "system" fn CertAddEncodedCertificateToSystemStoreW(szcertstorename : windows_core::PCWSTR, pbcertencoded : *const u8, cbcertencoded : u32) -> windows_core::BOOL);
    unsafe { CertAddEncodedCertificateToSystemStoreW(szcertstorename.param().abi(), core::mem::transmute(pbcertencoded.as_ptr()), pbcertencoded.len().try_into().unwrap()).ok() }
}
#[inline]
pub unsafe fn CertAddEnhancedKeyUsageIdentifier<P1>(pcertcontext: *const CERT_CONTEXT, pszusageidentifier: P1) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("crypt32.dll" "system" fn CertAddEnhancedKeyUsageIdentifier(pcertcontext : *const CERT_CONTEXT, pszusageidentifier : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { CertAddEnhancedKeyUsageIdentifier(pcertcontext, pszusageidentifier.param().abi()).ok() }
}
#[inline]
pub unsafe fn CertAddRefServerOcspResponse(hserverocspresponse: Option<*const core::ffi::c_void>) {
    windows_core::link!("crypt32.dll" "system" fn CertAddRefServerOcspResponse(hserverocspresponse : *const core::ffi::c_void));
    unsafe { CertAddRefServerOcspResponse(hserverocspresponse.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CertAddRefServerOcspResponseContext(pserverocspresponsecontext: Option<*const CERT_SERVER_OCSP_RESPONSE_CONTEXT>) {
    windows_core::link!("crypt32.dll" "system" fn CertAddRefServerOcspResponseContext(pserverocspresponsecontext : *const CERT_SERVER_OCSP_RESPONSE_CONTEXT));
    unsafe { CertAddRefServerOcspResponseContext(pserverocspresponsecontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CertAddSerializedElementToStore(hcertstore: Option<HCERTSTORE>, pbelement: &[u8], dwadddisposition: u32, dwflags: u32, dwcontexttypeflags: u32, pdwcontexttype: Option<*mut u32>, ppvcontext: Option<*mut *mut core::ffi::c_void>) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CertAddSerializedElementToStore(hcertstore : HCERTSTORE, pbelement : *const u8, cbelement : u32, dwadddisposition : u32, dwflags : u32, dwcontexttypeflags : u32, pdwcontexttype : *mut u32, ppvcontext : *mut *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { CertAddSerializedElementToStore(hcertstore.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pbelement.as_ptr()), pbelement.len().try_into().unwrap(), dwadddisposition, dwflags, dwcontexttypeflags, pdwcontexttype.unwrap_or(core::mem::zeroed()) as _, ppvcontext.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CertAddStoreToCollection(hcollectionstore: HCERTSTORE, hsiblingstore: Option<HCERTSTORE>, dwupdateflags: u32, dwpriority: u32) -> windows_core::BOOL {
    windows_core::link!("crypt32.dll" "system" fn CertAddStoreToCollection(hcollectionstore : HCERTSTORE, hsiblingstore : HCERTSTORE, dwupdateflags : u32, dwpriority : u32) -> windows_core::BOOL);
    unsafe { CertAddStoreToCollection(hcollectionstore, hsiblingstore.unwrap_or(core::mem::zeroed()) as _, dwupdateflags, dwpriority) }
}
#[inline]
pub unsafe fn CertAlgIdToOID(dwalgid: u32) -> windows_core::PCSTR {
    windows_core::link!("crypt32.dll" "system" fn CertAlgIdToOID(dwalgid : u32) -> windows_core::PCSTR);
    unsafe { CertAlgIdToOID(dwalgid) }
}
#[inline]
pub unsafe fn CertCloseServerOcspResponse(hserverocspresponse: Option<*const core::ffi::c_void>, dwflags: u32) {
    windows_core::link!("crypt32.dll" "system" fn CertCloseServerOcspResponse(hserverocspresponse : *const core::ffi::c_void, dwflags : u32));
    unsafe { CertCloseServerOcspResponse(hserverocspresponse.unwrap_or(core::mem::zeroed()) as _, dwflags) }
}
#[inline]
pub unsafe fn CertCloseStore(hcertstore: Option<HCERTSTORE>, dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CertCloseStore(hcertstore : HCERTSTORE, dwflags : u32) -> windows_core::BOOL);
    unsafe { CertCloseStore(hcertstore.unwrap_or(core::mem::zeroed()) as _, dwflags).ok() }
}
#[inline]
pub unsafe fn CertCompareCertificate(dwcertencodingtype: CERT_QUERY_ENCODING_TYPE, pcertid1: *const CERT_INFO, pcertid2: *const CERT_INFO) -> windows_core::BOOL {
    windows_core::link!("crypt32.dll" "system" fn CertCompareCertificate(dwcertencodingtype : CERT_QUERY_ENCODING_TYPE, pcertid1 : *const CERT_INFO, pcertid2 : *const CERT_INFO) -> windows_core::BOOL);
    unsafe { CertCompareCertificate(dwcertencodingtype, pcertid1, pcertid2) }
}
#[inline]
pub unsafe fn CertCompareCertificateName(dwcertencodingtype: CERT_QUERY_ENCODING_TYPE, pcertname1: *const CRYPT_INTEGER_BLOB, pcertname2: *const CRYPT_INTEGER_BLOB) -> windows_core::BOOL {
    windows_core::link!("crypt32.dll" "system" fn CertCompareCertificateName(dwcertencodingtype : CERT_QUERY_ENCODING_TYPE, pcertname1 : *const CRYPT_INTEGER_BLOB, pcertname2 : *const CRYPT_INTEGER_BLOB) -> windows_core::BOOL);
    unsafe { CertCompareCertificateName(dwcertencodingtype, pcertname1, pcertname2) }
}
#[inline]
pub unsafe fn CertCompareIntegerBlob(pint1: *const CRYPT_INTEGER_BLOB, pint2: *const CRYPT_INTEGER_BLOB) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CertCompareIntegerBlob(pint1 : *const CRYPT_INTEGER_BLOB, pint2 : *const CRYPT_INTEGER_BLOB) -> windows_core::BOOL);
    unsafe { CertCompareIntegerBlob(pint1, pint2).ok() }
}
#[inline]
pub unsafe fn CertComparePublicKeyInfo(dwcertencodingtype: CERT_QUERY_ENCODING_TYPE, ppublickey1: *const CERT_PUBLIC_KEY_INFO, ppublickey2: *const CERT_PUBLIC_KEY_INFO) -> windows_core::BOOL {
    windows_core::link!("crypt32.dll" "system" fn CertComparePublicKeyInfo(dwcertencodingtype : CERT_QUERY_ENCODING_TYPE, ppublickey1 : *const CERT_PUBLIC_KEY_INFO, ppublickey2 : *const CERT_PUBLIC_KEY_INFO) -> windows_core::BOOL);
    unsafe { CertComparePublicKeyInfo(dwcertencodingtype, ppublickey1, ppublickey2) }
}
#[inline]
pub unsafe fn CertControlStore(hcertstore: HCERTSTORE, dwflags: CERT_CONTROL_STORE_FLAGS, dwctrltype: u32, pvctrlpara: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CertControlStore(hcertstore : HCERTSTORE, dwflags : CERT_CONTROL_STORE_FLAGS, dwctrltype : u32, pvctrlpara : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { CertControlStore(hcertstore, dwflags, dwctrltype, pvctrlpara.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CertCreateCRLContext(dwcertencodingtype: CERT_QUERY_ENCODING_TYPE, pbcrlencoded: &[u8]) -> *mut CRL_CONTEXT {
    windows_core::link!("crypt32.dll" "system" fn CertCreateCRLContext(dwcertencodingtype : CERT_QUERY_ENCODING_TYPE, pbcrlencoded : *const u8, cbcrlencoded : u32) -> *mut CRL_CONTEXT);
    unsafe { CertCreateCRLContext(dwcertencodingtype, core::mem::transmute(pbcrlencoded.as_ptr()), pbcrlencoded.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn CertCreateCTLContext(dwmsgandcertencodingtype: u32, pbctlencoded: &[u8]) -> *mut CTL_CONTEXT {
    windows_core::link!("crypt32.dll" "system" fn CertCreateCTLContext(dwmsgandcertencodingtype : u32, pbctlencoded : *const u8, cbctlencoded : u32) -> *mut CTL_CONTEXT);
    unsafe { CertCreateCTLContext(dwmsgandcertencodingtype, core::mem::transmute(pbctlencoded.as_ptr()), pbctlencoded.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn CertCreateCTLEntryFromCertificateContextProperties(pcertcontext: *const CERT_CONTEXT, rgoptattr: Option<&[CRYPT_ATTRIBUTE]>, dwflags: u32, pvreserved: Option<*const core::ffi::c_void>, pctlentry: Option<*mut CTL_ENTRY>, pcbctlentry: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CertCreateCTLEntryFromCertificateContextProperties(pcertcontext : *const CERT_CONTEXT, coptattr : u32, rgoptattr : *const CRYPT_ATTRIBUTE, dwflags : u32, pvreserved : *const core::ffi::c_void, pctlentry : *mut CTL_ENTRY, pcbctlentry : *mut u32) -> windows_core::BOOL);
    unsafe { CertCreateCTLEntryFromCertificateContextProperties(pcertcontext, rgoptattr.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(rgoptattr.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), dwflags, pvreserved.unwrap_or(core::mem::zeroed()) as _, pctlentry.unwrap_or(core::mem::zeroed()) as _, pcbctlentry as _).ok() }
}
#[inline]
pub unsafe fn CertCreateCertificateChainEngine(pconfig: *const CERT_CHAIN_ENGINE_CONFIG, phchainengine: *mut HCERTCHAINENGINE) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CertCreateCertificateChainEngine(pconfig : *const CERT_CHAIN_ENGINE_CONFIG, phchainengine : *mut HCERTCHAINENGINE) -> windows_core::BOOL);
    unsafe { CertCreateCertificateChainEngine(pconfig, phchainengine as _).ok() }
}
#[inline]
pub unsafe fn CertCreateCertificateContext(dwcertencodingtype: CERT_QUERY_ENCODING_TYPE, pbcertencoded: &[u8]) -> *mut CERT_CONTEXT {
    windows_core::link!("crypt32.dll" "system" fn CertCreateCertificateContext(dwcertencodingtype : CERT_QUERY_ENCODING_TYPE, pbcertencoded : *const u8, cbcertencoded : u32) -> *mut CERT_CONTEXT);
    unsafe { CertCreateCertificateContext(dwcertencodingtype, core::mem::transmute(pbcertencoded.as_ptr()), pbcertencoded.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn CertCreateContext(dwcontexttype: u32, dwencodingtype: u32, pbencoded: &[u8], dwflags: u32, pcreatepara: Option<*const CERT_CREATE_CONTEXT_PARA>) -> *mut core::ffi::c_void {
    windows_core::link!("crypt32.dll" "system" fn CertCreateContext(dwcontexttype : u32, dwencodingtype : u32, pbencoded : *const u8, cbencoded : u32, dwflags : u32, pcreatepara : *const CERT_CREATE_CONTEXT_PARA) -> *mut core::ffi::c_void);
    unsafe { CertCreateContext(dwcontexttype, dwencodingtype, core::mem::transmute(pbencoded.as_ptr()), pbencoded.len().try_into().unwrap(), dwflags, pcreatepara.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CertCreateSelfSignCertificate(hcryptprovorncryptkey: Option<HCRYPTPROV_OR_NCRYPT_KEY_HANDLE>, psubjectissuerblob: *const CRYPT_INTEGER_BLOB, dwflags: CERT_CREATE_SELFSIGN_FLAGS, pkeyprovinfo: Option<*const CRYPT_KEY_PROV_INFO>, psignaturealgorithm: Option<*const CRYPT_ALGORITHM_IDENTIFIER>, pstarttime: Option<*const super::super::Foundation::SYSTEMTIME>, pendtime: Option<*const super::super::Foundation::SYSTEMTIME>, pextensions: Option<*const CERT_EXTENSIONS>) -> *mut CERT_CONTEXT {
    windows_core::link!("crypt32.dll" "system" fn CertCreateSelfSignCertificate(hcryptprovorncryptkey : HCRYPTPROV_OR_NCRYPT_KEY_HANDLE, psubjectissuerblob : *const CRYPT_INTEGER_BLOB, dwflags : CERT_CREATE_SELFSIGN_FLAGS, pkeyprovinfo : *const CRYPT_KEY_PROV_INFO, psignaturealgorithm : *const CRYPT_ALGORITHM_IDENTIFIER, pstarttime : *const super::super::Foundation:: SYSTEMTIME, pendtime : *const super::super::Foundation:: SYSTEMTIME, pextensions : *const CERT_EXTENSIONS) -> *mut CERT_CONTEXT);
    unsafe { CertCreateSelfSignCertificate(hcryptprovorncryptkey.unwrap_or(core::mem::zeroed()) as _, psubjectissuerblob, dwflags, pkeyprovinfo.unwrap_or(core::mem::zeroed()) as _, psignaturealgorithm.unwrap_or(core::mem::zeroed()) as _, pstarttime.unwrap_or(core::mem::zeroed()) as _, pendtime.unwrap_or(core::mem::zeroed()) as _, pextensions.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CertDeleteCRLFromStore(pcrlcontext: *const CRL_CONTEXT) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CertDeleteCRLFromStore(pcrlcontext : *const CRL_CONTEXT) -> windows_core::BOOL);
    unsafe { CertDeleteCRLFromStore(pcrlcontext).ok() }
}
#[inline]
pub unsafe fn CertDeleteCTLFromStore(pctlcontext: *const CTL_CONTEXT) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CertDeleteCTLFromStore(pctlcontext : *const CTL_CONTEXT) -> windows_core::BOOL);
    unsafe { CertDeleteCTLFromStore(pctlcontext).ok() }
}
#[inline]
pub unsafe fn CertDeleteCertificateFromStore(pcertcontext: *const CERT_CONTEXT) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CertDeleteCertificateFromStore(pcertcontext : *const CERT_CONTEXT) -> windows_core::BOOL);
    unsafe { CertDeleteCertificateFromStore(pcertcontext).ok() }
}
#[inline]
pub unsafe fn CertDuplicateCRLContext(pcrlcontext: Option<*const CRL_CONTEXT>) -> *mut CRL_CONTEXT {
    windows_core::link!("crypt32.dll" "system" fn CertDuplicateCRLContext(pcrlcontext : *const CRL_CONTEXT) -> *mut CRL_CONTEXT);
    unsafe { CertDuplicateCRLContext(pcrlcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CertDuplicateCTLContext(pctlcontext: Option<*const CTL_CONTEXT>) -> *mut CTL_CONTEXT {
    windows_core::link!("crypt32.dll" "system" fn CertDuplicateCTLContext(pctlcontext : *const CTL_CONTEXT) -> *mut CTL_CONTEXT);
    unsafe { CertDuplicateCTLContext(pctlcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CertDuplicateCertificateChain(pchaincontext: *const CERT_CHAIN_CONTEXT) -> *mut CERT_CHAIN_CONTEXT {
    windows_core::link!("crypt32.dll" "system" fn CertDuplicateCertificateChain(pchaincontext : *const CERT_CHAIN_CONTEXT) -> *mut CERT_CHAIN_CONTEXT);
    unsafe { CertDuplicateCertificateChain(pchaincontext) }
}
#[inline]
pub unsafe fn CertDuplicateCertificateContext(pcertcontext: Option<*const CERT_CONTEXT>) -> *mut CERT_CONTEXT {
    windows_core::link!("crypt32.dll" "system" fn CertDuplicateCertificateContext(pcertcontext : *const CERT_CONTEXT) -> *mut CERT_CONTEXT);
    unsafe { CertDuplicateCertificateContext(pcertcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CertDuplicateStore(hcertstore: HCERTSTORE) -> HCERTSTORE {
    windows_core::link!("crypt32.dll" "system" fn CertDuplicateStore(hcertstore : HCERTSTORE) -> HCERTSTORE);
    unsafe { CertDuplicateStore(hcertstore) }
}
#[inline]
pub unsafe fn CertEnumCRLContextProperties(pcrlcontext: *const CRL_CONTEXT, dwpropid: u32) -> u32 {
    windows_core::link!("crypt32.dll" "system" fn CertEnumCRLContextProperties(pcrlcontext : *const CRL_CONTEXT, dwpropid : u32) -> u32);
    unsafe { CertEnumCRLContextProperties(pcrlcontext, dwpropid) }
}
#[inline]
pub unsafe fn CertEnumCRLsInStore(hcertstore: HCERTSTORE, pprevcrlcontext: Option<*const CRL_CONTEXT>) -> *mut CRL_CONTEXT {
    windows_core::link!("crypt32.dll" "system" fn CertEnumCRLsInStore(hcertstore : HCERTSTORE, pprevcrlcontext : *const CRL_CONTEXT) -> *mut CRL_CONTEXT);
    unsafe { CertEnumCRLsInStore(hcertstore, pprevcrlcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CertEnumCTLContextProperties(pctlcontext: *const CTL_CONTEXT, dwpropid: u32) -> u32 {
    windows_core::link!("crypt32.dll" "system" fn CertEnumCTLContextProperties(pctlcontext : *const CTL_CONTEXT, dwpropid : u32) -> u32);
    unsafe { CertEnumCTLContextProperties(pctlcontext, dwpropid) }
}
#[inline]
pub unsafe fn CertEnumCTLsInStore(hcertstore: HCERTSTORE, pprevctlcontext: Option<*const CTL_CONTEXT>) -> *mut CTL_CONTEXT {
    windows_core::link!("crypt32.dll" "system" fn CertEnumCTLsInStore(hcertstore : HCERTSTORE, pprevctlcontext : *const CTL_CONTEXT) -> *mut CTL_CONTEXT);
    unsafe { CertEnumCTLsInStore(hcertstore, pprevctlcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CertEnumCertificateContextProperties(pcertcontext: *const CERT_CONTEXT, dwpropid: u32) -> u32 {
    windows_core::link!("crypt32.dll" "system" fn CertEnumCertificateContextProperties(pcertcontext : *const CERT_CONTEXT, dwpropid : u32) -> u32);
    unsafe { CertEnumCertificateContextProperties(pcertcontext, dwpropid) }
}
#[inline]
pub unsafe fn CertEnumCertificatesInStore(hcertstore: HCERTSTORE, pprevcertcontext: Option<*const CERT_CONTEXT>) -> *mut CERT_CONTEXT {
    windows_core::link!("crypt32.dll" "system" fn CertEnumCertificatesInStore(hcertstore : HCERTSTORE, pprevcertcontext : *const CERT_CONTEXT) -> *mut CERT_CONTEXT);
    unsafe { CertEnumCertificatesInStore(hcertstore, pprevcertcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CertEnumPhysicalStore(pvsystemstore: *const core::ffi::c_void, dwflags: u32, pvarg: Option<*mut core::ffi::c_void>, pfnenum: PFN_CERT_ENUM_PHYSICAL_STORE) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CertEnumPhysicalStore(pvsystemstore : *const core::ffi::c_void, dwflags : u32, pvarg : *mut core::ffi::c_void, pfnenum : PFN_CERT_ENUM_PHYSICAL_STORE) -> windows_core::BOOL);
    unsafe { CertEnumPhysicalStore(pvsystemstore, dwflags, pvarg.unwrap_or(core::mem::zeroed()) as _, pfnenum).ok() }
}
#[inline]
pub unsafe fn CertEnumSubjectInSortedCTL(pctlcontext: *const CTL_CONTEXT, ppvnextsubject: *mut *mut core::ffi::c_void, psubjectidentifier: Option<*mut CRYPT_INTEGER_BLOB>, pencodedattributes: Option<*mut CRYPT_INTEGER_BLOB>) -> windows_core::BOOL {
    windows_core::link!("crypt32.dll" "system" fn CertEnumSubjectInSortedCTL(pctlcontext : *const CTL_CONTEXT, ppvnextsubject : *mut *mut core::ffi::c_void, psubjectidentifier : *mut CRYPT_INTEGER_BLOB, pencodedattributes : *mut CRYPT_INTEGER_BLOB) -> windows_core::BOOL);
    unsafe { CertEnumSubjectInSortedCTL(pctlcontext, ppvnextsubject as _, psubjectidentifier.unwrap_or(core::mem::zeroed()) as _, pencodedattributes.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CertEnumSystemStore(dwflags: u32, pvsystemstorelocationpara: Option<*const core::ffi::c_void>, pvarg: Option<*mut core::ffi::c_void>, pfnenum: PFN_CERT_ENUM_SYSTEM_STORE) -> windows_core::BOOL {
    windows_core::link!("crypt32.dll" "system" fn CertEnumSystemStore(dwflags : u32, pvsystemstorelocationpara : *const core::ffi::c_void, pvarg : *mut core::ffi::c_void, pfnenum : PFN_CERT_ENUM_SYSTEM_STORE) -> windows_core::BOOL);
    unsafe { CertEnumSystemStore(dwflags, pvsystemstorelocationpara.unwrap_or(core::mem::zeroed()) as _, pvarg.unwrap_or(core::mem::zeroed()) as _, pfnenum) }
}
#[inline]
pub unsafe fn CertEnumSystemStoreLocation(dwflags: u32, pvarg: Option<*mut core::ffi::c_void>, pfnenum: PFN_CERT_ENUM_SYSTEM_STORE_LOCATION) -> windows_core::BOOL {
    windows_core::link!("crypt32.dll" "system" fn CertEnumSystemStoreLocation(dwflags : u32, pvarg : *mut core::ffi::c_void, pfnenum : PFN_CERT_ENUM_SYSTEM_STORE_LOCATION) -> windows_core::BOOL);
    unsafe { CertEnumSystemStoreLocation(dwflags, pvarg.unwrap_or(core::mem::zeroed()) as _, pfnenum) }
}
#[inline]
pub unsafe fn CertFindAttribute<P0>(pszobjid: P0, rgattr: &[CRYPT_ATTRIBUTE]) -> *mut CRYPT_ATTRIBUTE
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("crypt32.dll" "system" fn CertFindAttribute(pszobjid : windows_core::PCSTR, cattr : u32, rgattr : *const CRYPT_ATTRIBUTE) -> *mut CRYPT_ATTRIBUTE);
    unsafe { CertFindAttribute(pszobjid.param().abi(), rgattr.len().try_into().unwrap(), core::mem::transmute(rgattr.as_ptr())) }
}
#[inline]
pub unsafe fn CertFindCRLInStore(hcertstore: HCERTSTORE, dwcertencodingtype: CERT_QUERY_ENCODING_TYPE, dwfindflags: u32, dwfindtype: u32, pvfindpara: Option<*const core::ffi::c_void>, pprevcrlcontext: Option<*const CRL_CONTEXT>) -> *mut CRL_CONTEXT {
    windows_core::link!("crypt32.dll" "system" fn CertFindCRLInStore(hcertstore : HCERTSTORE, dwcertencodingtype : CERT_QUERY_ENCODING_TYPE, dwfindflags : u32, dwfindtype : u32, pvfindpara : *const core::ffi::c_void, pprevcrlcontext : *const CRL_CONTEXT) -> *mut CRL_CONTEXT);
    unsafe { CertFindCRLInStore(hcertstore, dwcertencodingtype, dwfindflags, dwfindtype, pvfindpara.unwrap_or(core::mem::zeroed()) as _, pprevcrlcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CertFindCTLInStore(hcertstore: HCERTSTORE, dwmsgandcertencodingtype: u32, dwfindflags: u32, dwfindtype: CERT_FIND_TYPE, pvfindpara: Option<*const core::ffi::c_void>, pprevctlcontext: Option<*const CTL_CONTEXT>) -> *mut CTL_CONTEXT {
    windows_core::link!("crypt32.dll" "system" fn CertFindCTLInStore(hcertstore : HCERTSTORE, dwmsgandcertencodingtype : u32, dwfindflags : u32, dwfindtype : CERT_FIND_TYPE, pvfindpara : *const core::ffi::c_void, pprevctlcontext : *const CTL_CONTEXT) -> *mut CTL_CONTEXT);
    unsafe { CertFindCTLInStore(hcertstore, dwmsgandcertencodingtype, dwfindflags, dwfindtype, pvfindpara.unwrap_or(core::mem::zeroed()) as _, pprevctlcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CertFindCertificateInCRL(pcert: *const CERT_CONTEXT, pcrlcontext: *const CRL_CONTEXT, dwflags: u32, pvreserved: Option<*const core::ffi::c_void>, ppcrlentry: *mut *mut CRL_ENTRY) -> windows_core::BOOL {
    windows_core::link!("crypt32.dll" "system" fn CertFindCertificateInCRL(pcert : *const CERT_CONTEXT, pcrlcontext : *const CRL_CONTEXT, dwflags : u32, pvreserved : *const core::ffi::c_void, ppcrlentry : *mut *mut CRL_ENTRY) -> windows_core::BOOL);
    unsafe { CertFindCertificateInCRL(pcert, pcrlcontext, dwflags, pvreserved.unwrap_or(core::mem::zeroed()) as _, ppcrlentry as _) }
}
#[inline]
pub unsafe fn CertFindCertificateInStore(hcertstore: HCERTSTORE, dwcertencodingtype: CERT_QUERY_ENCODING_TYPE, dwfindflags: u32, dwfindtype: CERT_FIND_FLAGS, pvfindpara: Option<*const core::ffi::c_void>, pprevcertcontext: Option<*const CERT_CONTEXT>) -> *mut CERT_CONTEXT {
    windows_core::link!("crypt32.dll" "system" fn CertFindCertificateInStore(hcertstore : HCERTSTORE, dwcertencodingtype : CERT_QUERY_ENCODING_TYPE, dwfindflags : u32, dwfindtype : CERT_FIND_FLAGS, pvfindpara : *const core::ffi::c_void, pprevcertcontext : *const CERT_CONTEXT) -> *mut CERT_CONTEXT);
    unsafe { CertFindCertificateInStore(hcertstore, dwcertencodingtype, dwfindflags, dwfindtype, pvfindpara.unwrap_or(core::mem::zeroed()) as _, pprevcertcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CertFindChainInStore(hcertstore: HCERTSTORE, dwcertencodingtype: CERT_QUERY_ENCODING_TYPE, dwfindflags: CERT_FIND_CHAIN_IN_STORE_FLAGS, dwfindtype: u32, pvfindpara: Option<*const core::ffi::c_void>, pprevchaincontext: Option<*const CERT_CHAIN_CONTEXT>) -> *mut CERT_CHAIN_CONTEXT {
    windows_core::link!("crypt32.dll" "system" fn CertFindChainInStore(hcertstore : HCERTSTORE, dwcertencodingtype : CERT_QUERY_ENCODING_TYPE, dwfindflags : CERT_FIND_CHAIN_IN_STORE_FLAGS, dwfindtype : u32, pvfindpara : *const core::ffi::c_void, pprevchaincontext : *const CERT_CHAIN_CONTEXT) -> *mut CERT_CHAIN_CONTEXT);
    unsafe { CertFindChainInStore(hcertstore, dwcertencodingtype, dwfindflags, dwfindtype, pvfindpara.unwrap_or(core::mem::zeroed()) as _, pprevchaincontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CertFindExtension<P0>(pszobjid: P0, rgextensions: &[CERT_EXTENSION]) -> *mut CERT_EXTENSION
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("crypt32.dll" "system" fn CertFindExtension(pszobjid : windows_core::PCSTR, cextensions : u32, rgextensions : *const CERT_EXTENSION) -> *mut CERT_EXTENSION);
    unsafe { CertFindExtension(pszobjid.param().abi(), rgextensions.len().try_into().unwrap(), core::mem::transmute(rgextensions.as_ptr())) }
}
#[inline]
pub unsafe fn CertFindRDNAttr<P0>(pszobjid: P0, pname: *const CERT_NAME_INFO) -> *mut CERT_RDN_ATTR
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("crypt32.dll" "system" fn CertFindRDNAttr(pszobjid : windows_core::PCSTR, pname : *const CERT_NAME_INFO) -> *mut CERT_RDN_ATTR);
    unsafe { CertFindRDNAttr(pszobjid.param().abi(), pname) }
}
#[inline]
pub unsafe fn CertFindSubjectInCTL(dwencodingtype: u32, dwsubjecttype: u32, pvsubject: *const core::ffi::c_void, pctlcontext: *const CTL_CONTEXT, dwflags: u32) -> *mut CTL_ENTRY {
    windows_core::link!("crypt32.dll" "system" fn CertFindSubjectInCTL(dwencodingtype : u32, dwsubjecttype : u32, pvsubject : *const core::ffi::c_void, pctlcontext : *const CTL_CONTEXT, dwflags : u32) -> *mut CTL_ENTRY);
    unsafe { CertFindSubjectInCTL(dwencodingtype, dwsubjecttype, pvsubject, pctlcontext, dwflags) }
}
#[inline]
pub unsafe fn CertFindSubjectInSortedCTL(psubjectidentifier: *const CRYPT_INTEGER_BLOB, pctlcontext: *const CTL_CONTEXT, dwflags: u32, pvreserved: Option<*const core::ffi::c_void>, pencodedattributes: Option<*mut CRYPT_INTEGER_BLOB>) -> windows_core::BOOL {
    windows_core::link!("crypt32.dll" "system" fn CertFindSubjectInSortedCTL(psubjectidentifier : *const CRYPT_INTEGER_BLOB, pctlcontext : *const CTL_CONTEXT, dwflags : u32, pvreserved : *const core::ffi::c_void, pencodedattributes : *mut CRYPT_INTEGER_BLOB) -> windows_core::BOOL);
    unsafe { CertFindSubjectInSortedCTL(psubjectidentifier, pctlcontext, dwflags, pvreserved.unwrap_or(core::mem::zeroed()) as _, pencodedattributes.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CertFreeCRLContext(pcrlcontext: Option<*const CRL_CONTEXT>) -> windows_core::BOOL {
    windows_core::link!("crypt32.dll" "system" fn CertFreeCRLContext(pcrlcontext : *const CRL_CONTEXT) -> windows_core::BOOL);
    unsafe { CertFreeCRLContext(pcrlcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CertFreeCTLContext(pctlcontext: Option<*const CTL_CONTEXT>) -> windows_core::BOOL {
    windows_core::link!("crypt32.dll" "system" fn CertFreeCTLContext(pctlcontext : *const CTL_CONTEXT) -> windows_core::BOOL);
    unsafe { CertFreeCTLContext(pctlcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CertFreeCertificateChain(pchaincontext: *const CERT_CHAIN_CONTEXT) {
    windows_core::link!("crypt32.dll" "system" fn CertFreeCertificateChain(pchaincontext : *const CERT_CHAIN_CONTEXT));
    unsafe { CertFreeCertificateChain(pchaincontext) }
}
#[inline]
pub unsafe fn CertFreeCertificateChainEngine(hchainengine: Option<HCERTCHAINENGINE>) {
    windows_core::link!("crypt32.dll" "system" fn CertFreeCertificateChainEngine(hchainengine : HCERTCHAINENGINE));
    unsafe { CertFreeCertificateChainEngine(hchainengine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CertFreeCertificateChainList(prgpselection: *const *const CERT_CHAIN_CONTEXT) {
    windows_core::link!("crypt32.dll" "system" fn CertFreeCertificateChainList(prgpselection : *const *const CERT_CHAIN_CONTEXT));
    unsafe { CertFreeCertificateChainList(prgpselection) }
}
#[inline]
pub unsafe fn CertFreeCertificateContext(pcertcontext: Option<*const CERT_CONTEXT>) -> windows_core::BOOL {
    windows_core::link!("crypt32.dll" "system" fn CertFreeCertificateContext(pcertcontext : *const CERT_CONTEXT) -> windows_core::BOOL);
    unsafe { CertFreeCertificateContext(pcertcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CertFreeServerOcspResponseContext(pserverocspresponsecontext: Option<*const CERT_SERVER_OCSP_RESPONSE_CONTEXT>) {
    windows_core::link!("crypt32.dll" "system" fn CertFreeServerOcspResponseContext(pserverocspresponsecontext : *const CERT_SERVER_OCSP_RESPONSE_CONTEXT));
    unsafe { CertFreeServerOcspResponseContext(pserverocspresponsecontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CertGetCRLContextProperty(pcrlcontext: *const CRL_CONTEXT, dwpropid: u32, pvdata: Option<*mut core::ffi::c_void>, pcbdata: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CertGetCRLContextProperty(pcrlcontext : *const CRL_CONTEXT, dwpropid : u32, pvdata : *mut core::ffi::c_void, pcbdata : *mut u32) -> windows_core::BOOL);
    unsafe { CertGetCRLContextProperty(pcrlcontext, dwpropid, pvdata.unwrap_or(core::mem::zeroed()) as _, pcbdata as _).ok() }
}
#[inline]
pub unsafe fn CertGetCRLFromStore(hcertstore: HCERTSTORE, pissuercontext: Option<*const CERT_CONTEXT>, pprevcrlcontext: Option<*const CRL_CONTEXT>, pdwflags: *mut u32) -> *mut CRL_CONTEXT {
    windows_core::link!("crypt32.dll" "system" fn CertGetCRLFromStore(hcertstore : HCERTSTORE, pissuercontext : *const CERT_CONTEXT, pprevcrlcontext : *const CRL_CONTEXT, pdwflags : *mut u32) -> *mut CRL_CONTEXT);
    unsafe { CertGetCRLFromStore(hcertstore, pissuercontext.unwrap_or(core::mem::zeroed()) as _, pprevcrlcontext.unwrap_or(core::mem::zeroed()) as _, pdwflags as _) }
}
#[inline]
pub unsafe fn CertGetCTLContextProperty(pctlcontext: *const CTL_CONTEXT, dwpropid: u32, pvdata: Option<*mut core::ffi::c_void>, pcbdata: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CertGetCTLContextProperty(pctlcontext : *const CTL_CONTEXT, dwpropid : u32, pvdata : *mut core::ffi::c_void, pcbdata : *mut u32) -> windows_core::BOOL);
    unsafe { CertGetCTLContextProperty(pctlcontext, dwpropid, pvdata.unwrap_or(core::mem::zeroed()) as _, pcbdata as _).ok() }
}
#[inline]
pub unsafe fn CertGetCertificateChain(hchainengine: Option<HCERTCHAINENGINE>, pcertcontext: *const CERT_CONTEXT, ptime: Option<*const super::super::Foundation::FILETIME>, hadditionalstore: Option<HCERTSTORE>, pchainpara: *const CERT_CHAIN_PARA, dwflags: u32, pvreserved: Option<*const core::ffi::c_void>, ppchaincontext: *mut *mut CERT_CHAIN_CONTEXT) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CertGetCertificateChain(hchainengine : HCERTCHAINENGINE, pcertcontext : *const CERT_CONTEXT, ptime : *const super::super::Foundation:: FILETIME, hadditionalstore : HCERTSTORE, pchainpara : *const CERT_CHAIN_PARA, dwflags : u32, pvreserved : *const core::ffi::c_void, ppchaincontext : *mut *mut CERT_CHAIN_CONTEXT) -> windows_core::BOOL);
    unsafe { CertGetCertificateChain(hchainengine.unwrap_or(core::mem::zeroed()) as _, pcertcontext, ptime.unwrap_or(core::mem::zeroed()) as _, hadditionalstore.unwrap_or(core::mem::zeroed()) as _, pchainpara, dwflags, pvreserved.unwrap_or(core::mem::zeroed()) as _, ppchaincontext as _).ok() }
}
#[inline]
pub unsafe fn CertGetCertificateContextProperty(pcertcontext: *const CERT_CONTEXT, dwpropid: u32, pvdata: Option<*mut core::ffi::c_void>, pcbdata: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CertGetCertificateContextProperty(pcertcontext : *const CERT_CONTEXT, dwpropid : u32, pvdata : *mut core::ffi::c_void, pcbdata : *mut u32) -> windows_core::BOOL);
    unsafe { CertGetCertificateContextProperty(pcertcontext, dwpropid, pvdata.unwrap_or(core::mem::zeroed()) as _, pcbdata as _).ok() }
}
#[inline]
pub unsafe fn CertGetEnhancedKeyUsage(pcertcontext: *const CERT_CONTEXT, dwflags: u32, pusage: Option<*mut CTL_USAGE>, pcbusage: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CertGetEnhancedKeyUsage(pcertcontext : *const CERT_CONTEXT, dwflags : u32, pusage : *mut CTL_USAGE, pcbusage : *mut u32) -> windows_core::BOOL);
    unsafe { CertGetEnhancedKeyUsage(pcertcontext, dwflags, pusage.unwrap_or(core::mem::zeroed()) as _, pcbusage as _).ok() }
}
#[inline]
pub unsafe fn CertGetIntendedKeyUsage(dwcertencodingtype: CERT_QUERY_ENCODING_TYPE, pcertinfo: *const CERT_INFO, pbkeyusage: &mut [u8]) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CertGetIntendedKeyUsage(dwcertencodingtype : CERT_QUERY_ENCODING_TYPE, pcertinfo : *const CERT_INFO, pbkeyusage : *mut u8, cbkeyusage : u32) -> windows_core::BOOL);
    unsafe { CertGetIntendedKeyUsage(dwcertencodingtype, pcertinfo, core::mem::transmute(pbkeyusage.as_ptr()), pbkeyusage.len().try_into().unwrap()).ok() }
}
#[inline]
pub unsafe fn CertGetIssuerCertificateFromStore(hcertstore: HCERTSTORE, psubjectcontext: *const CERT_CONTEXT, pprevissuercontext: Option<*const CERT_CONTEXT>, pdwflags: *mut u32) -> *mut CERT_CONTEXT {
    windows_core::link!("crypt32.dll" "system" fn CertGetIssuerCertificateFromStore(hcertstore : HCERTSTORE, psubjectcontext : *const CERT_CONTEXT, pprevissuercontext : *const CERT_CONTEXT, pdwflags : *mut u32) -> *mut CERT_CONTEXT);
    unsafe { CertGetIssuerCertificateFromStore(hcertstore, psubjectcontext, pprevissuercontext.unwrap_or(core::mem::zeroed()) as _, pdwflags as _) }
}
#[inline]
pub unsafe fn CertGetNameStringA(pcertcontext: *const CERT_CONTEXT, dwtype: u32, dwflags: u32, pvtypepara: Option<*const core::ffi::c_void>, psznamestring: Option<&mut [u8]>) -> u32 {
    windows_core::link!("crypt32.dll" "system" fn CertGetNameStringA(pcertcontext : *const CERT_CONTEXT, dwtype : u32, dwflags : u32, pvtypepara : *const core::ffi::c_void, psznamestring : windows_core::PSTR, cchnamestring : u32) -> u32);
    unsafe { CertGetNameStringA(pcertcontext, dwtype, dwflags, pvtypepara.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(psznamestring.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), psznamestring.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn CertGetNameStringW(pcertcontext: *const CERT_CONTEXT, dwtype: u32, dwflags: u32, pvtypepara: Option<*const core::ffi::c_void>, psznamestring: Option<&mut [u16]>) -> u32 {
    windows_core::link!("crypt32.dll" "system" fn CertGetNameStringW(pcertcontext : *const CERT_CONTEXT, dwtype : u32, dwflags : u32, pvtypepara : *const core::ffi::c_void, psznamestring : windows_core::PWSTR, cchnamestring : u32) -> u32);
    unsafe { CertGetNameStringW(pcertcontext, dwtype, dwflags, pvtypepara.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(psznamestring.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), psznamestring.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn CertGetPublicKeyLength(dwcertencodingtype: CERT_QUERY_ENCODING_TYPE, ppublickey: *const CERT_PUBLIC_KEY_INFO) -> u32 {
    windows_core::link!("crypt32.dll" "system" fn CertGetPublicKeyLength(dwcertencodingtype : CERT_QUERY_ENCODING_TYPE, ppublickey : *const CERT_PUBLIC_KEY_INFO) -> u32);
    unsafe { CertGetPublicKeyLength(dwcertencodingtype, ppublickey) }
}
#[inline]
pub unsafe fn CertGetServerOcspResponseContext(hserverocspresponse: *const core::ffi::c_void, dwflags: u32, pvreserved: Option<*const core::ffi::c_void>) -> *mut CERT_SERVER_OCSP_RESPONSE_CONTEXT {
    windows_core::link!("crypt32.dll" "system" fn CertGetServerOcspResponseContext(hserverocspresponse : *const core::ffi::c_void, dwflags : u32, pvreserved : *const core::ffi::c_void) -> *mut CERT_SERVER_OCSP_RESPONSE_CONTEXT);
    unsafe { CertGetServerOcspResponseContext(hserverocspresponse, dwflags, pvreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CertGetStoreProperty(hcertstore: HCERTSTORE, dwpropid: u32, pvdata: Option<*mut core::ffi::c_void>, pcbdata: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CertGetStoreProperty(hcertstore : HCERTSTORE, dwpropid : u32, pvdata : *mut core::ffi::c_void, pcbdata : *mut u32) -> windows_core::BOOL);
    unsafe { CertGetStoreProperty(hcertstore, dwpropid, pvdata.unwrap_or(core::mem::zeroed()) as _, pcbdata as _).ok() }
}
#[inline]
pub unsafe fn CertGetSubjectCertificateFromStore(hcertstore: HCERTSTORE, dwcertencodingtype: CERT_QUERY_ENCODING_TYPE, pcertid: *const CERT_INFO) -> *mut CERT_CONTEXT {
    windows_core::link!("crypt32.dll" "system" fn CertGetSubjectCertificateFromStore(hcertstore : HCERTSTORE, dwcertencodingtype : CERT_QUERY_ENCODING_TYPE, pcertid : *const CERT_INFO) -> *mut CERT_CONTEXT);
    unsafe { CertGetSubjectCertificateFromStore(hcertstore, dwcertencodingtype, pcertid) }
}
#[inline]
pub unsafe fn CertGetValidUsages(rghcerts: &[*const CERT_CONTEXT], cnumoids: *mut i32, rghoids: Option<*mut windows_core::PSTR>, pcboids: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CertGetValidUsages(ccerts : u32, rghcerts : *const *const CERT_CONTEXT, cnumoids : *mut i32, rghoids : *mut windows_core::PSTR, pcboids : *mut u32) -> windows_core::BOOL);
    unsafe { CertGetValidUsages(rghcerts.len().try_into().unwrap(), core::mem::transmute(rghcerts.as_ptr()), cnumoids as _, rghoids.unwrap_or(core::mem::zeroed()) as _, pcboids as _).ok() }
}
#[inline]
pub unsafe fn CertIsRDNAttrsInCertificateName(dwcertencodingtype: CERT_QUERY_ENCODING_TYPE, dwflags: u32, pcertname: *const CRYPT_INTEGER_BLOB, prdn: *const CERT_RDN) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CertIsRDNAttrsInCertificateName(dwcertencodingtype : CERT_QUERY_ENCODING_TYPE, dwflags : u32, pcertname : *const CRYPT_INTEGER_BLOB, prdn : *const CERT_RDN) -> windows_core::BOOL);
    unsafe { CertIsRDNAttrsInCertificateName(dwcertencodingtype, dwflags, pcertname, prdn).ok() }
}
#[inline]
pub unsafe fn CertIsStrongHashToSign<P1>(pstrongsignpara: *const CERT_STRONG_SIGN_PARA, pwszcnghashalgid: P1, psigningcert: Option<*const CERT_CONTEXT>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("crypt32.dll" "system" fn CertIsStrongHashToSign(pstrongsignpara : *const CERT_STRONG_SIGN_PARA, pwszcnghashalgid : windows_core::PCWSTR, psigningcert : *const CERT_CONTEXT) -> windows_core::BOOL);
    unsafe { CertIsStrongHashToSign(pstrongsignpara, pwszcnghashalgid.param().abi(), psigningcert.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CertIsValidCRLForCertificate(pcert: *const CERT_CONTEXT, pcrl: *const CRL_CONTEXT, dwflags: u32, pvreserved: Option<*const core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("crypt32.dll" "system" fn CertIsValidCRLForCertificate(pcert : *const CERT_CONTEXT, pcrl : *const CRL_CONTEXT, dwflags : u32, pvreserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { CertIsValidCRLForCertificate(pcert, pcrl, dwflags, pvreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CertIsWeakHash<P1, P5>(dwhashusetype: u32, pwszcnghashalgid: P1, dwchainflags: u32, psignerchaincontext: Option<*const CERT_CHAIN_CONTEXT>, ptimestamp: Option<*const super::super::Foundation::FILETIME>, pwszfilename: P5) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("crypt32.dll" "system" fn CertIsWeakHash(dwhashusetype : u32, pwszcnghashalgid : windows_core::PCWSTR, dwchainflags : u32, psignerchaincontext : *const CERT_CHAIN_CONTEXT, ptimestamp : *const super::super::Foundation:: FILETIME, pwszfilename : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { CertIsWeakHash(dwhashusetype, pwszcnghashalgid.param().abi(), dwchainflags, psignerchaincontext.unwrap_or(core::mem::zeroed()) as _, ptimestamp.unwrap_or(core::mem::zeroed()) as _, pwszfilename.param().abi()) }
}
#[inline]
pub unsafe fn CertNameToStrA(dwcertencodingtype: CERT_QUERY_ENCODING_TYPE, pname: *const CRYPT_INTEGER_BLOB, dwstrtype: CERT_STRING_TYPE, psz: Option<&mut [u8]>) -> u32 {
    windows_core::link!("crypt32.dll" "system" fn CertNameToStrA(dwcertencodingtype : CERT_QUERY_ENCODING_TYPE, pname : *const CRYPT_INTEGER_BLOB, dwstrtype : CERT_STRING_TYPE, psz : windows_core::PSTR, csz : u32) -> u32);
    unsafe { CertNameToStrA(dwcertencodingtype, pname, dwstrtype, core::mem::transmute(psz.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), psz.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn CertNameToStrW(dwcertencodingtype: CERT_QUERY_ENCODING_TYPE, pname: *const CRYPT_INTEGER_BLOB, dwstrtype: CERT_STRING_TYPE, psz: Option<&mut [u16]>) -> u32 {
    windows_core::link!("crypt32.dll" "system" fn CertNameToStrW(dwcertencodingtype : CERT_QUERY_ENCODING_TYPE, pname : *const CRYPT_INTEGER_BLOB, dwstrtype : CERT_STRING_TYPE, psz : windows_core::PWSTR, csz : u32) -> u32);
    unsafe { CertNameToStrW(dwcertencodingtype, pname, dwstrtype, core::mem::transmute(psz.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), psz.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn CertOIDToAlgId<P0>(pszobjid: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("crypt32.dll" "system" fn CertOIDToAlgId(pszobjid : windows_core::PCSTR) -> u32);
    unsafe { CertOIDToAlgId(pszobjid.param().abi()) }
}
#[inline]
pub unsafe fn CertOpenServerOcspResponse(pchaincontext: *const CERT_CHAIN_CONTEXT, dwflags: u32, popenpara: Option<*const CERT_SERVER_OCSP_RESPONSE_OPEN_PARA>) -> *mut core::ffi::c_void {
    windows_core::link!("crypt32.dll" "system" fn CertOpenServerOcspResponse(pchaincontext : *const CERT_CHAIN_CONTEXT, dwflags : u32, popenpara : *const CERT_SERVER_OCSP_RESPONSE_OPEN_PARA) -> *mut core::ffi::c_void);
    unsafe { CertOpenServerOcspResponse(pchaincontext, dwflags, popenpara.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CertOpenStore<P0>(lpszstoreprovider: P0, dwencodingtype: CERT_QUERY_ENCODING_TYPE, hcryptprov: Option<HCRYPTPROV_LEGACY>, dwflags: CERT_OPEN_STORE_FLAGS, pvpara: Option<*const core::ffi::c_void>) -> windows_core::Result<HCERTSTORE>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("crypt32.dll" "system" fn CertOpenStore(lpszstoreprovider : windows_core::PCSTR, dwencodingtype : CERT_QUERY_ENCODING_TYPE, hcryptprov : HCRYPTPROV_LEGACY, dwflags : CERT_OPEN_STORE_FLAGS, pvpara : *const core::ffi::c_void) -> HCERTSTORE);
    let result__ = unsafe { CertOpenStore(lpszstoreprovider.param().abi(), dwencodingtype, hcryptprov.unwrap_or(core::mem::zeroed()) as _, dwflags, pvpara.unwrap_or(core::mem::zeroed()) as _) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[inline]
pub unsafe fn CertOpenSystemStoreA<P1>(hprov: Option<HCRYPTPROV_LEGACY>, szsubsystemprotocol: P1) -> windows_core::Result<HCERTSTORE>
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("crypt32.dll" "system" fn CertOpenSystemStoreA(hprov : HCRYPTPROV_LEGACY, szsubsystemprotocol : windows_core::PCSTR) -> HCERTSTORE);
    let result__ = unsafe { CertOpenSystemStoreA(hprov.unwrap_or(core::mem::zeroed()) as _, szsubsystemprotocol.param().abi()) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[inline]
pub unsafe fn CertOpenSystemStoreW<P1>(hprov: Option<HCRYPTPROV_LEGACY>, szsubsystemprotocol: P1) -> windows_core::Result<HCERTSTORE>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("crypt32.dll" "system" fn CertOpenSystemStoreW(hprov : HCRYPTPROV_LEGACY, szsubsystemprotocol : windows_core::PCWSTR) -> HCERTSTORE);
    let result__ = unsafe { CertOpenSystemStoreW(hprov.unwrap_or(core::mem::zeroed()) as _, szsubsystemprotocol.param().abi()) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[inline]
pub unsafe fn CertRDNValueToStrA(dwvaluetype: u32, pvalue: *const CRYPT_INTEGER_BLOB, psz: Option<&mut [u8]>) -> u32 {
    windows_core::link!("crypt32.dll" "system" fn CertRDNValueToStrA(dwvaluetype : u32, pvalue : *const CRYPT_INTEGER_BLOB, psz : windows_core::PSTR, csz : u32) -> u32);
    unsafe { CertRDNValueToStrA(dwvaluetype, pvalue, core::mem::transmute(psz.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), psz.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn CertRDNValueToStrW(dwvaluetype: u32, pvalue: *const CRYPT_INTEGER_BLOB, psz: Option<&mut [u16]>) -> u32 {
    windows_core::link!("crypt32.dll" "system" fn CertRDNValueToStrW(dwvaluetype : u32, pvalue : *const CRYPT_INTEGER_BLOB, psz : windows_core::PWSTR, csz : u32) -> u32);
    unsafe { CertRDNValueToStrW(dwvaluetype, pvalue, core::mem::transmute(psz.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), psz.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn CertRegisterPhysicalStore<P2>(pvsystemstore: *const core::ffi::c_void, dwflags: u32, pwszstorename: P2, pstoreinfo: *const CERT_PHYSICAL_STORE_INFO, pvreserved: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("crypt32.dll" "system" fn CertRegisterPhysicalStore(pvsystemstore : *const core::ffi::c_void, dwflags : u32, pwszstorename : windows_core::PCWSTR, pstoreinfo : *const CERT_PHYSICAL_STORE_INFO, pvreserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { CertRegisterPhysicalStore(pvsystemstore, dwflags, pwszstorename.param().abi(), pstoreinfo, pvreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CertRegisterSystemStore(pvsystemstore: *const core::ffi::c_void, dwflags: u32, pstoreinfo: Option<*const CERT_SYSTEM_STORE_INFO>, pvreserved: Option<*const core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("crypt32.dll" "system" fn CertRegisterSystemStore(pvsystemstore : *const core::ffi::c_void, dwflags : u32, pstoreinfo : *const CERT_SYSTEM_STORE_INFO, pvreserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { CertRegisterSystemStore(pvsystemstore, dwflags, pstoreinfo.unwrap_or(core::mem::zeroed()) as _, pvreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CertRemoveEnhancedKeyUsageIdentifier<P1>(pcertcontext: *const CERT_CONTEXT, pszusageidentifier: P1) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("crypt32.dll" "system" fn CertRemoveEnhancedKeyUsageIdentifier(pcertcontext : *const CERT_CONTEXT, pszusageidentifier : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { CertRemoveEnhancedKeyUsageIdentifier(pcertcontext, pszusageidentifier.param().abi()).ok() }
}
#[inline]
pub unsafe fn CertRemoveStoreFromCollection(hcollectionstore: HCERTSTORE, hsiblingstore: HCERTSTORE) {
    windows_core::link!("crypt32.dll" "system" fn CertRemoveStoreFromCollection(hcollectionstore : HCERTSTORE, hsiblingstore : HCERTSTORE));
    unsafe { CertRemoveStoreFromCollection(hcollectionstore, hsiblingstore) }
}
#[inline]
pub unsafe fn CertResyncCertificateChainEngine(hchainengine: Option<HCERTCHAINENGINE>) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CertResyncCertificateChainEngine(hchainengine : HCERTCHAINENGINE) -> windows_core::BOOL);
    unsafe { CertResyncCertificateChainEngine(hchainengine.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CertRetrieveLogoOrBiometricInfo<P1>(pcertcontext: *const CERT_CONTEXT, lpszlogoorbiometrictype: P1, dwretrievalflags: u32, dwtimeout: u32, dwflags: u32, pvreserved: Option<*const core::ffi::c_void>, ppbdata: *mut *mut u8, pcbdata: *mut u32, ppwszmimetype: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("crypt32.dll" "system" fn CertRetrieveLogoOrBiometricInfo(pcertcontext : *const CERT_CONTEXT, lpszlogoorbiometrictype : windows_core::PCSTR, dwretrievalflags : u32, dwtimeout : u32, dwflags : u32, pvreserved : *const core::ffi::c_void, ppbdata : *mut *mut u8, pcbdata : *mut u32, ppwszmimetype : *mut windows_core::PWSTR) -> windows_core::BOOL);
    unsafe { CertRetrieveLogoOrBiometricInfo(pcertcontext, lpszlogoorbiometrictype.param().abi(), dwretrievalflags, dwtimeout, dwflags, pvreserved.unwrap_or(core::mem::zeroed()) as _, ppbdata as _, pcbdata as _, ppwszmimetype.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CertSaveStore(hcertstore: HCERTSTORE, dwencodingtype: CERT_QUERY_ENCODING_TYPE, dwsaveas: CERT_STORE_SAVE_AS, dwsaveto: CERT_STORE_SAVE_TO, pvsavetopara: *mut core::ffi::c_void, dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CertSaveStore(hcertstore : HCERTSTORE, dwencodingtype : CERT_QUERY_ENCODING_TYPE, dwsaveas : CERT_STORE_SAVE_AS, dwsaveto : CERT_STORE_SAVE_TO, pvsavetopara : *mut core::ffi::c_void, dwflags : u32) -> windows_core::BOOL);
    unsafe { CertSaveStore(hcertstore, dwencodingtype, dwsaveas, dwsaveto, pvsavetopara as _, dwflags).ok() }
}
#[inline]
pub unsafe fn CertSelectCertificateChains(pselectioncontext: Option<*const windows_core::GUID>, dwflags: u32, pchainparameters: Option<*const CERT_SELECT_CHAIN_PARA>, rgpcriteria: Option<&[CERT_SELECT_CRITERIA]>, hstore: HCERTSTORE, pcselection: *mut u32, pprgpselection: *mut *mut *mut CERT_CHAIN_CONTEXT) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CertSelectCertificateChains(pselectioncontext : *const windows_core::GUID, dwflags : u32, pchainparameters : *const CERT_SELECT_CHAIN_PARA, ccriteria : u32, rgpcriteria : *const CERT_SELECT_CRITERIA, hstore : HCERTSTORE, pcselection : *mut u32, pprgpselection : *mut *mut *mut CERT_CHAIN_CONTEXT) -> windows_core::BOOL);
    unsafe { CertSelectCertificateChains(pselectioncontext.unwrap_or(core::mem::zeroed()) as _, dwflags, pchainparameters.unwrap_or(core::mem::zeroed()) as _, rgpcriteria.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(rgpcriteria.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), hstore, pcselection as _, pprgpselection as _).ok() }
}
#[inline]
pub unsafe fn CertSerializeCRLStoreElement(pcrlcontext: *const CRL_CONTEXT, dwflags: u32, pbelement: Option<*mut u8>, pcbelement: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CertSerializeCRLStoreElement(pcrlcontext : *const CRL_CONTEXT, dwflags : u32, pbelement : *mut u8, pcbelement : *mut u32) -> windows_core::BOOL);
    unsafe { CertSerializeCRLStoreElement(pcrlcontext, dwflags, pbelement.unwrap_or(core::mem::zeroed()) as _, pcbelement as _).ok() }
}
#[inline]
pub unsafe fn CertSerializeCTLStoreElement(pctlcontext: *const CTL_CONTEXT, dwflags: u32, pbelement: Option<*mut u8>, pcbelement: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CertSerializeCTLStoreElement(pctlcontext : *const CTL_CONTEXT, dwflags : u32, pbelement : *mut u8, pcbelement : *mut u32) -> windows_core::BOOL);
    unsafe { CertSerializeCTLStoreElement(pctlcontext, dwflags, pbelement.unwrap_or(core::mem::zeroed()) as _, pcbelement as _).ok() }
}
#[inline]
pub unsafe fn CertSerializeCertificateStoreElement(pcertcontext: *const CERT_CONTEXT, dwflags: u32, pbelement: Option<*mut u8>, pcbelement: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CertSerializeCertificateStoreElement(pcertcontext : *const CERT_CONTEXT, dwflags : u32, pbelement : *mut u8, pcbelement : *mut u32) -> windows_core::BOOL);
    unsafe { CertSerializeCertificateStoreElement(pcertcontext, dwflags, pbelement.unwrap_or(core::mem::zeroed()) as _, pcbelement as _).ok() }
}
#[inline]
pub unsafe fn CertSetCRLContextProperty(pcrlcontext: *const CRL_CONTEXT, dwpropid: u32, dwflags: u32, pvdata: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CertSetCRLContextProperty(pcrlcontext : *const CRL_CONTEXT, dwpropid : u32, dwflags : u32, pvdata : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { CertSetCRLContextProperty(pcrlcontext, dwpropid, dwflags, pvdata.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CertSetCTLContextProperty(pctlcontext: *const CTL_CONTEXT, dwpropid: u32, dwflags: u32, pvdata: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CertSetCTLContextProperty(pctlcontext : *const CTL_CONTEXT, dwpropid : u32, dwflags : u32, pvdata : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { CertSetCTLContextProperty(pctlcontext, dwpropid, dwflags, pvdata.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CertSetCertificateContextPropertiesFromCTLEntry(pcertcontext: *const CERT_CONTEXT, pctlentry: *const CTL_ENTRY, dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CertSetCertificateContextPropertiesFromCTLEntry(pcertcontext : *const CERT_CONTEXT, pctlentry : *const CTL_ENTRY, dwflags : u32) -> windows_core::BOOL);
    unsafe { CertSetCertificateContextPropertiesFromCTLEntry(pcertcontext, pctlentry, dwflags).ok() }
}
#[inline]
pub unsafe fn CertSetCertificateContextProperty(pcertcontext: *const CERT_CONTEXT, dwpropid: u32, dwflags: u32, pvdata: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CertSetCertificateContextProperty(pcertcontext : *const CERT_CONTEXT, dwpropid : u32, dwflags : u32, pvdata : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { CertSetCertificateContextProperty(pcertcontext, dwpropid, dwflags, pvdata.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CertSetEnhancedKeyUsage(pcertcontext: *const CERT_CONTEXT, pusage: Option<*const CTL_USAGE>) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CertSetEnhancedKeyUsage(pcertcontext : *const CERT_CONTEXT, pusage : *const CTL_USAGE) -> windows_core::BOOL);
    unsafe { CertSetEnhancedKeyUsage(pcertcontext, pusage.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CertSetStoreProperty(hcertstore: HCERTSTORE, dwpropid: u32, dwflags: u32, pvdata: Option<*const core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("crypt32.dll" "system" fn CertSetStoreProperty(hcertstore : HCERTSTORE, dwpropid : u32, dwflags : u32, pvdata : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { CertSetStoreProperty(hcertstore, dwpropid, dwflags, pvdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CertStrToNameA<P1>(dwcertencodingtype: CERT_QUERY_ENCODING_TYPE, pszx500: P1, dwstrtype: CERT_STRING_TYPE, pvreserved: Option<*const core::ffi::c_void>, pbencoded: Option<*mut u8>, pcbencoded: *mut u32, ppszerror: Option<*mut windows_core::PCSTR>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("crypt32.dll" "system" fn CertStrToNameA(dwcertencodingtype : CERT_QUERY_ENCODING_TYPE, pszx500 : windows_core::PCSTR, dwstrtype : CERT_STRING_TYPE, pvreserved : *const core::ffi::c_void, pbencoded : *mut u8, pcbencoded : *mut u32, ppszerror : *mut windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { CertStrToNameA(dwcertencodingtype, pszx500.param().abi(), dwstrtype, pvreserved.unwrap_or(core::mem::zeroed()) as _, pbencoded.unwrap_or(core::mem::zeroed()) as _, pcbencoded as _, ppszerror.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CertStrToNameW<P1>(dwcertencodingtype: CERT_QUERY_ENCODING_TYPE, pszx500: P1, dwstrtype: CERT_STRING_TYPE, pvreserved: Option<*const core::ffi::c_void>, pbencoded: Option<*mut u8>, pcbencoded: *mut u32, ppszerror: Option<*mut windows_core::PCWSTR>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("crypt32.dll" "system" fn CertStrToNameW(dwcertencodingtype : CERT_QUERY_ENCODING_TYPE, pszx500 : windows_core::PCWSTR, dwstrtype : CERT_STRING_TYPE, pvreserved : *const core::ffi::c_void, pbencoded : *mut u8, pcbencoded : *mut u32, ppszerror : *mut windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { CertStrToNameW(dwcertencodingtype, pszx500.param().abi(), dwstrtype, pvreserved.unwrap_or(core::mem::zeroed()) as _, pbencoded.unwrap_or(core::mem::zeroed()) as _, pcbencoded as _, ppszerror.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CertUnregisterPhysicalStore<P2>(pvsystemstore: *const core::ffi::c_void, dwflags: u32, pwszstorename: P2) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("crypt32.dll" "system" fn CertUnregisterPhysicalStore(pvsystemstore : *const core::ffi::c_void, dwflags : u32, pwszstorename : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { CertUnregisterPhysicalStore(pvsystemstore, dwflags, pwszstorename.param().abi()) }
}
#[inline]
pub unsafe fn CertUnregisterSystemStore(pvsystemstore: *const core::ffi::c_void, dwflags: u32) -> windows_core::BOOL {
    windows_core::link!("crypt32.dll" "system" fn CertUnregisterSystemStore(pvsystemstore : *const core::ffi::c_void, dwflags : u32) -> windows_core::BOOL);
    unsafe { CertUnregisterSystemStore(pvsystemstore, dwflags) }
}
#[inline]
pub unsafe fn CertVerifyCRLRevocation(dwcertencodingtype: CERT_QUERY_ENCODING_TYPE, pcertid: *const CERT_INFO, rgpcrlinfo: &[*const CRL_INFO]) -> windows_core::BOOL {
    windows_core::link!("crypt32.dll" "system" fn CertVerifyCRLRevocation(dwcertencodingtype : CERT_QUERY_ENCODING_TYPE, pcertid : *const CERT_INFO, ccrlinfo : u32, rgpcrlinfo : *const *const CRL_INFO) -> windows_core::BOOL);
    unsafe { CertVerifyCRLRevocation(dwcertencodingtype, pcertid, rgpcrlinfo.len().try_into().unwrap(), core::mem::transmute(rgpcrlinfo.as_ptr())) }
}
#[inline]
pub unsafe fn CertVerifyCRLTimeValidity(ptimetoverify: Option<*const super::super::Foundation::FILETIME>, pcrlinfo: *const CRL_INFO) -> i32 {
    windows_core::link!("crypt32.dll" "system" fn CertVerifyCRLTimeValidity(ptimetoverify : *const super::super::Foundation:: FILETIME, pcrlinfo : *const CRL_INFO) -> i32);
    unsafe { CertVerifyCRLTimeValidity(ptimetoverify.unwrap_or(core::mem::zeroed()) as _, pcrlinfo) }
}
#[inline]
pub unsafe fn CertVerifyCTLUsage(dwencodingtype: u32, dwsubjecttype: u32, pvsubject: *const core::ffi::c_void, psubjectusage: *const CTL_USAGE, dwflags: u32, pverifyusagepara: Option<*const CTL_VERIFY_USAGE_PARA>, pverifyusagestatus: *mut CTL_VERIFY_USAGE_STATUS) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CertVerifyCTLUsage(dwencodingtype : u32, dwsubjecttype : u32, pvsubject : *const core::ffi::c_void, psubjectusage : *const CTL_USAGE, dwflags : u32, pverifyusagepara : *const CTL_VERIFY_USAGE_PARA, pverifyusagestatus : *mut CTL_VERIFY_USAGE_STATUS) -> windows_core::BOOL);
    unsafe { CertVerifyCTLUsage(dwencodingtype, dwsubjecttype, pvsubject, psubjectusage, dwflags, pverifyusagepara.unwrap_or(core::mem::zeroed()) as _, pverifyusagestatus as _).ok() }
}
#[inline]
pub unsafe fn CertVerifyCertificateChainPolicy<P0>(pszpolicyoid: P0, pchaincontext: *const CERT_CHAIN_CONTEXT, ppolicypara: *const CERT_CHAIN_POLICY_PARA, ppolicystatus: *mut CERT_CHAIN_POLICY_STATUS) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("crypt32.dll" "system" fn CertVerifyCertificateChainPolicy(pszpolicyoid : windows_core::PCSTR, pchaincontext : *const CERT_CHAIN_CONTEXT, ppolicypara : *const CERT_CHAIN_POLICY_PARA, ppolicystatus : *mut CERT_CHAIN_POLICY_STATUS) -> windows_core::BOOL);
    unsafe { CertVerifyCertificateChainPolicy(pszpolicyoid.param().abi(), pchaincontext, ppolicypara, ppolicystatus as _) }
}
#[inline]
pub unsafe fn CertVerifyRevocation(dwencodingtype: u32, dwrevtype: u32, rgpvcontext: &[*const core::ffi::c_void], dwflags: u32, prevpara: Option<*const CERT_REVOCATION_PARA>, prevstatus: *mut CERT_REVOCATION_STATUS) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CertVerifyRevocation(dwencodingtype : u32, dwrevtype : u32, ccontext : u32, rgpvcontext : *const *const core::ffi::c_void, dwflags : u32, prevpara : *const CERT_REVOCATION_PARA, prevstatus : *mut CERT_REVOCATION_STATUS) -> windows_core::BOOL);
    unsafe { CertVerifyRevocation(dwencodingtype, dwrevtype, rgpvcontext.len().try_into().unwrap(), core::mem::transmute(rgpvcontext.as_ptr()), dwflags, prevpara.unwrap_or(core::mem::zeroed()) as _, prevstatus as _).ok() }
}
#[inline]
pub unsafe fn CertVerifySubjectCertificateContext(psubject: *const CERT_CONTEXT, pissuer: Option<*const CERT_CONTEXT>, pdwflags: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CertVerifySubjectCertificateContext(psubject : *const CERT_CONTEXT, pissuer : *const CERT_CONTEXT, pdwflags : *mut u32) -> windows_core::BOOL);
    unsafe { CertVerifySubjectCertificateContext(psubject, pissuer.unwrap_or(core::mem::zeroed()) as _, pdwflags as _).ok() }
}
#[inline]
pub unsafe fn CertVerifyTimeValidity(ptimetoverify: Option<*const super::super::Foundation::FILETIME>, pcertinfo: *const CERT_INFO) -> i32 {
    windows_core::link!("crypt32.dll" "system" fn CertVerifyTimeValidity(ptimetoverify : *const super::super::Foundation:: FILETIME, pcertinfo : *const CERT_INFO) -> i32);
    unsafe { CertVerifyTimeValidity(ptimetoverify.unwrap_or(core::mem::zeroed()) as _, pcertinfo) }
}
#[inline]
pub unsafe fn CertVerifyValidityNesting(psubjectinfo: *const CERT_INFO, pissuerinfo: *const CERT_INFO) -> windows_core::BOOL {
    windows_core::link!("crypt32.dll" "system" fn CertVerifyValidityNesting(psubjectinfo : *const CERT_INFO, pissuerinfo : *const CERT_INFO) -> windows_core::BOOL);
    unsafe { CertVerifyValidityNesting(psubjectinfo, pissuerinfo) }
}
#[inline]
pub unsafe fn CloseCryptoHandle(hcrypto: *const INFORMATIONCARD_CRYPTO_HANDLE) -> windows_core::Result<()> {
    windows_core::link!("infocardapi.dll" "system" fn CloseCryptoHandle(hcrypto : *const INFORMATIONCARD_CRYPTO_HANDLE) -> windows_core::HRESULT);
    unsafe { CloseCryptoHandle(hcrypto).ok() }
}
#[inline]
pub unsafe fn CryptAcquireCertificatePrivateKey(pcert: *const CERT_CONTEXT, dwflags: CRYPT_ACQUIRE_FLAGS, pvparameters: Option<*const core::ffi::c_void>, phcryptprovorncryptkey: *mut HCRYPTPROV_OR_NCRYPT_KEY_HANDLE, pdwkeyspec: Option<*mut CERT_KEY_SPEC>, pfcallerfreeprovorncryptkey: Option<*mut windows_core::BOOL>) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CryptAcquireCertificatePrivateKey(pcert : *const CERT_CONTEXT, dwflags : CRYPT_ACQUIRE_FLAGS, pvparameters : *const core::ffi::c_void, phcryptprovorncryptkey : *mut HCRYPTPROV_OR_NCRYPT_KEY_HANDLE, pdwkeyspec : *mut CERT_KEY_SPEC, pfcallerfreeprovorncryptkey : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { CryptAcquireCertificatePrivateKey(pcert, dwflags, pvparameters.unwrap_or(core::mem::zeroed()) as _, phcryptprovorncryptkey as _, pdwkeyspec.unwrap_or(core::mem::zeroed()) as _, pfcallerfreeprovorncryptkey.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CryptAcquireContextA<P1, P2>(phprov: *mut usize, szcontainer: P1, szprovider: P2, dwprovtype: u32, dwflags: u32) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn CryptAcquireContextA(phprov : *mut usize, szcontainer : windows_core::PCSTR, szprovider : windows_core::PCSTR, dwprovtype : u32, dwflags : u32) -> windows_core::BOOL);
    unsafe { CryptAcquireContextA(phprov as _, szcontainer.param().abi(), szprovider.param().abi(), dwprovtype, dwflags).ok() }
}
#[inline]
pub unsafe fn CryptAcquireContextW<P1, P2>(phprov: *mut usize, szcontainer: P1, szprovider: P2, dwprovtype: u32, dwflags: u32) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn CryptAcquireContextW(phprov : *mut usize, szcontainer : windows_core::PCWSTR, szprovider : windows_core::PCWSTR, dwprovtype : u32, dwflags : u32) -> windows_core::BOOL);
    unsafe { CryptAcquireContextW(phprov as _, szcontainer.param().abi(), szprovider.param().abi(), dwprovtype, dwflags).ok() }
}
#[inline]
pub unsafe fn CryptBinaryToStringA(pbbinary: &[u8], dwflags: CRYPT_STRING, pszstring: Option<windows_core::PSTR>, pcchstring: *mut u32) -> windows_core::BOOL {
    windows_core::link!("crypt32.dll" "system" fn CryptBinaryToStringA(pbbinary : *const u8, cbbinary : u32, dwflags : CRYPT_STRING, pszstring : windows_core::PSTR, pcchstring : *mut u32) -> windows_core::BOOL);
    unsafe { CryptBinaryToStringA(core::mem::transmute(pbbinary.as_ptr()), pbbinary.len().try_into().unwrap(), dwflags, pszstring.unwrap_or(core::mem::zeroed()) as _, pcchstring as _) }
}
#[inline]
pub unsafe fn CryptBinaryToStringW(pbbinary: &[u8], dwflags: CRYPT_STRING, pszstring: Option<windows_core::PWSTR>, pcchstring: *mut u32) -> windows_core::BOOL {
    windows_core::link!("crypt32.dll" "system" fn CryptBinaryToStringW(pbbinary : *const u8, cbbinary : u32, dwflags : CRYPT_STRING, pszstring : windows_core::PWSTR, pcchstring : *mut u32) -> windows_core::BOOL);
    unsafe { CryptBinaryToStringW(core::mem::transmute(pbbinary.as_ptr()), pbbinary.len().try_into().unwrap(), dwflags, pszstring.unwrap_or(core::mem::zeroed()) as _, pcchstring as _) }
}
#[inline]
pub unsafe fn CryptCloseAsyncHandle(hasync: Option<HCRYPTASYNC>) -> windows_core::BOOL {
    windows_core::link!("crypt32.dll" "system" fn CryptCloseAsyncHandle(hasync : HCRYPTASYNC) -> windows_core::BOOL);
    unsafe { CryptCloseAsyncHandle(hasync.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CryptContextAddRef(hprov: usize, pdwreserved: Option<*const u32>, dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("advapi32.dll" "system" fn CryptContextAddRef(hprov : usize, pdwreserved : *const u32, dwflags : u32) -> windows_core::BOOL);
    unsafe { CryptContextAddRef(hprov, pdwreserved.unwrap_or(core::mem::zeroed()) as _, dwflags).ok() }
}
#[inline]
pub unsafe fn CryptCreateAsyncHandle(dwflags: u32, phasync: *mut HCRYPTASYNC) -> windows_core::BOOL {
    windows_core::link!("crypt32.dll" "system" fn CryptCreateAsyncHandle(dwflags : u32, phasync : *mut HCRYPTASYNC) -> windows_core::BOOL);
    unsafe { CryptCreateAsyncHandle(dwflags, phasync as _) }
}
#[inline]
pub unsafe fn CryptCreateHash(hprov: usize, algid: ALG_ID, hkey: usize, dwflags: u32, phhash: *mut usize) -> windows_core::Result<()> {
    windows_core::link!("advapi32.dll" "system" fn CryptCreateHash(hprov : usize, algid : ALG_ID, hkey : usize, dwflags : u32, phhash : *mut usize) -> windows_core::BOOL);
    unsafe { CryptCreateHash(hprov, algid, hkey, dwflags, phhash as _).ok() }
}
#[inline]
pub unsafe fn CryptCreateKeyIdentifierFromCSP<P1>(dwcertencodingtype: CERT_QUERY_ENCODING_TYPE, pszpubkeyoid: P1, ppubkeystruc: *const PUBLICKEYSTRUC, cbpubkeystruc: u32, dwflags: u32, pvreserved: Option<*const core::ffi::c_void>, pbhash: Option<*mut u8>, pcbhash: *mut u32) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("crypt32.dll" "system" fn CryptCreateKeyIdentifierFromCSP(dwcertencodingtype : CERT_QUERY_ENCODING_TYPE, pszpubkeyoid : windows_core::PCSTR, ppubkeystruc : *const PUBLICKEYSTRUC, cbpubkeystruc : u32, dwflags : u32, pvreserved : *const core::ffi::c_void, pbhash : *mut u8, pcbhash : *mut u32) -> windows_core::BOOL);
    unsafe { CryptCreateKeyIdentifierFromCSP(dwcertencodingtype, pszpubkeyoid.param().abi(), ppubkeystruc, cbpubkeystruc, dwflags, pvreserved.unwrap_or(core::mem::zeroed()) as _, pbhash.unwrap_or(core::mem::zeroed()) as _, pcbhash as _).ok() }
}
#[inline]
pub unsafe fn CryptDecodeMessage(dwmsgtypeflags: u32, pdecryptpara: Option<*const CRYPT_DECRYPT_MESSAGE_PARA>, pverifypara: Option<*const CRYPT_VERIFY_MESSAGE_PARA>, dwsignerindex: u32, pbencodedblob: &[u8], dwprevinnercontenttype: u32, pdwmsgtype: Option<*mut u32>, pdwinnercontenttype: Option<*mut u32>, pbdecoded: Option<*mut u8>, pcbdecoded: Option<*mut u32>, ppxchgcert: Option<*mut *mut CERT_CONTEXT>, ppsignercert: Option<*mut *mut CERT_CONTEXT>) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CryptDecodeMessage(dwmsgtypeflags : u32, pdecryptpara : *const CRYPT_DECRYPT_MESSAGE_PARA, pverifypara : *const CRYPT_VERIFY_MESSAGE_PARA, dwsignerindex : u32, pbencodedblob : *const u8, cbencodedblob : u32, dwprevinnercontenttype : u32, pdwmsgtype : *mut u32, pdwinnercontenttype : *mut u32, pbdecoded : *mut u8, pcbdecoded : *mut u32, ppxchgcert : *mut *mut CERT_CONTEXT, ppsignercert : *mut *mut CERT_CONTEXT) -> windows_core::BOOL);
    unsafe {
        CryptDecodeMessage(
            dwmsgtypeflags,
            pdecryptpara.unwrap_or(core::mem::zeroed()) as _,
            pverifypara.unwrap_or(core::mem::zeroed()) as _,
            dwsignerindex,
            core::mem::transmute(pbencodedblob.as_ptr()),
            pbencodedblob.len().try_into().unwrap(),
            dwprevinnercontenttype,
            pdwmsgtype.unwrap_or(core::mem::zeroed()) as _,
            pdwinnercontenttype.unwrap_or(core::mem::zeroed()) as _,
            pbdecoded.unwrap_or(core::mem::zeroed()) as _,
            pcbdecoded.unwrap_or(core::mem::zeroed()) as _,
            ppxchgcert.unwrap_or(core::mem::zeroed()) as _,
            ppsignercert.unwrap_or(core::mem::zeroed()) as _,
        )
        .ok()
    }
}
#[inline]
pub unsafe fn CryptDecodeObject<P1>(dwcertencodingtype: CERT_QUERY_ENCODING_TYPE, lpszstructtype: P1, pbencoded: &[u8], dwflags: u32, pvstructinfo: Option<*mut core::ffi::c_void>, pcbstructinfo: *mut u32) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("crypt32.dll" "system" fn CryptDecodeObject(dwcertencodingtype : CERT_QUERY_ENCODING_TYPE, lpszstructtype : windows_core::PCSTR, pbencoded : *const u8, cbencoded : u32, dwflags : u32, pvstructinfo : *mut core::ffi::c_void, pcbstructinfo : *mut u32) -> windows_core::BOOL);
    unsafe { CryptDecodeObject(dwcertencodingtype, lpszstructtype.param().abi(), core::mem::transmute(pbencoded.as_ptr()), pbencoded.len().try_into().unwrap(), dwflags, pvstructinfo.unwrap_or(core::mem::zeroed()) as _, pcbstructinfo as _).ok() }
}
#[inline]
pub unsafe fn CryptDecodeObjectEx<P1>(dwcertencodingtype: CERT_QUERY_ENCODING_TYPE, lpszstructtype: P1, pbencoded: &[u8], dwflags: u32, pdecodepara: Option<*const CRYPT_DECODE_PARA>, pvstructinfo: Option<*mut core::ffi::c_void>, pcbstructinfo: *mut u32) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("crypt32.dll" "system" fn CryptDecodeObjectEx(dwcertencodingtype : CERT_QUERY_ENCODING_TYPE, lpszstructtype : windows_core::PCSTR, pbencoded : *const u8, cbencoded : u32, dwflags : u32, pdecodepara : *const CRYPT_DECODE_PARA, pvstructinfo : *mut core::ffi::c_void, pcbstructinfo : *mut u32) -> windows_core::BOOL);
    unsafe { CryptDecodeObjectEx(dwcertencodingtype, lpszstructtype.param().abi(), core::mem::transmute(pbencoded.as_ptr()), pbencoded.len().try_into().unwrap(), dwflags, pdecodepara.unwrap_or(core::mem::zeroed()) as _, pvstructinfo.unwrap_or(core::mem::zeroed()) as _, pcbstructinfo as _).ok() }
}
#[inline]
pub unsafe fn CryptDecrypt(hkey: usize, hhash: usize, r#final: bool, dwflags: u32, pbdata: *mut u8, pdwdatalen: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("advapi32.dll" "system" fn CryptDecrypt(hkey : usize, hhash : usize, r#final : windows_core::BOOL, dwflags : u32, pbdata : *mut u8, pdwdatalen : *mut u32) -> windows_core::BOOL);
    unsafe { CryptDecrypt(hkey, hhash, r#final.into(), dwflags, pbdata as _, pdwdatalen as _).ok() }
}
#[inline]
pub unsafe fn CryptDecryptAndVerifyMessageSignature(pdecryptpara: *const CRYPT_DECRYPT_MESSAGE_PARA, pverifypara: *const CRYPT_VERIFY_MESSAGE_PARA, dwsignerindex: u32, pbencryptedblob: &[u8], pbdecrypted: Option<*mut u8>, pcbdecrypted: Option<*mut u32>, ppxchgcert: Option<*mut *mut CERT_CONTEXT>, ppsignercert: Option<*mut *mut CERT_CONTEXT>) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CryptDecryptAndVerifyMessageSignature(pdecryptpara : *const CRYPT_DECRYPT_MESSAGE_PARA, pverifypara : *const CRYPT_VERIFY_MESSAGE_PARA, dwsignerindex : u32, pbencryptedblob : *const u8, cbencryptedblob : u32, pbdecrypted : *mut u8, pcbdecrypted : *mut u32, ppxchgcert : *mut *mut CERT_CONTEXT, ppsignercert : *mut *mut CERT_CONTEXT) -> windows_core::BOOL);
    unsafe { CryptDecryptAndVerifyMessageSignature(pdecryptpara, pverifypara, dwsignerindex, core::mem::transmute(pbencryptedblob.as_ptr()), pbencryptedblob.len().try_into().unwrap(), pbdecrypted.unwrap_or(core::mem::zeroed()) as _, pcbdecrypted.unwrap_or(core::mem::zeroed()) as _, ppxchgcert.unwrap_or(core::mem::zeroed()) as _, ppsignercert.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CryptDecryptMessage(pdecryptpara: *const CRYPT_DECRYPT_MESSAGE_PARA, pbencryptedblob: &[u8], pbdecrypted: Option<*mut u8>, pcbdecrypted: Option<*mut u32>, ppxchgcert: Option<*mut *mut CERT_CONTEXT>) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CryptDecryptMessage(pdecryptpara : *const CRYPT_DECRYPT_MESSAGE_PARA, pbencryptedblob : *const u8, cbencryptedblob : u32, pbdecrypted : *mut u8, pcbdecrypted : *mut u32, ppxchgcert : *mut *mut CERT_CONTEXT) -> windows_core::BOOL);
    unsafe { CryptDecryptMessage(pdecryptpara, core::mem::transmute(pbencryptedblob.as_ptr()), pbencryptedblob.len().try_into().unwrap(), pbdecrypted.unwrap_or(core::mem::zeroed()) as _, pcbdecrypted.unwrap_or(core::mem::zeroed()) as _, ppxchgcert.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CryptDeriveKey(hprov: usize, algid: ALG_ID, hbasedata: usize, dwflags: u32, phkey: *mut usize) -> windows_core::Result<()> {
    windows_core::link!("advapi32.dll" "system" fn CryptDeriveKey(hprov : usize, algid : ALG_ID, hbasedata : usize, dwflags : u32, phkey : *mut usize) -> windows_core::BOOL);
    unsafe { CryptDeriveKey(hprov, algid, hbasedata, dwflags, phkey as _).ok() }
}
#[inline]
pub unsafe fn CryptDestroyHash(hhash: usize) -> windows_core::Result<()> {
    windows_core::link!("advapi32.dll" "system" fn CryptDestroyHash(hhash : usize) -> windows_core::BOOL);
    unsafe { CryptDestroyHash(hhash).ok() }
}
#[inline]
pub unsafe fn CryptDestroyKey(hkey: usize) -> windows_core::Result<()> {
    windows_core::link!("advapi32.dll" "system" fn CryptDestroyKey(hkey : usize) -> windows_core::BOOL);
    unsafe { CryptDestroyKey(hkey).ok() }
}
#[inline]
pub unsafe fn CryptDuplicateHash(hhash: usize, pdwreserved: Option<*const u32>, dwflags: u32, phhash: *mut usize) -> windows_core::Result<()> {
    windows_core::link!("advapi32.dll" "system" fn CryptDuplicateHash(hhash : usize, pdwreserved : *const u32, dwflags : u32, phhash : *mut usize) -> windows_core::BOOL);
    unsafe { CryptDuplicateHash(hhash, pdwreserved.unwrap_or(core::mem::zeroed()) as _, dwflags, phhash as _).ok() }
}
#[inline]
pub unsafe fn CryptDuplicateKey(hkey: usize, pdwreserved: Option<*const u32>, dwflags: u32, phkey: *mut usize) -> windows_core::Result<()> {
    windows_core::link!("advapi32.dll" "system" fn CryptDuplicateKey(hkey : usize, pdwreserved : *const u32, dwflags : u32, phkey : *mut usize) -> windows_core::BOOL);
    unsafe { CryptDuplicateKey(hkey, pdwreserved.unwrap_or(core::mem::zeroed()) as _, dwflags, phkey as _).ok() }
}
#[inline]
pub unsafe fn CryptEncodeObject<P1>(dwcertencodingtype: CERT_QUERY_ENCODING_TYPE, lpszstructtype: P1, pvstructinfo: *const core::ffi::c_void, pbencoded: Option<*mut u8>, pcbencoded: *mut u32) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("crypt32.dll" "system" fn CryptEncodeObject(dwcertencodingtype : CERT_QUERY_ENCODING_TYPE, lpszstructtype : windows_core::PCSTR, pvstructinfo : *const core::ffi::c_void, pbencoded : *mut u8, pcbencoded : *mut u32) -> windows_core::BOOL);
    unsafe { CryptEncodeObject(dwcertencodingtype, lpszstructtype.param().abi(), pvstructinfo, pbencoded.unwrap_or(core::mem::zeroed()) as _, pcbencoded as _).ok() }
}
#[inline]
pub unsafe fn CryptEncodeObjectEx<P1>(dwcertencodingtype: CERT_QUERY_ENCODING_TYPE, lpszstructtype: P1, pvstructinfo: *const core::ffi::c_void, dwflags: CRYPT_ENCODE_OBJECT_FLAGS, pencodepara: Option<*const CRYPT_ENCODE_PARA>, pvencoded: Option<*mut core::ffi::c_void>, pcbencoded: *mut u32) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("crypt32.dll" "system" fn CryptEncodeObjectEx(dwcertencodingtype : CERT_QUERY_ENCODING_TYPE, lpszstructtype : windows_core::PCSTR, pvstructinfo : *const core::ffi::c_void, dwflags : CRYPT_ENCODE_OBJECT_FLAGS, pencodepara : *const CRYPT_ENCODE_PARA, pvencoded : *mut core::ffi::c_void, pcbencoded : *mut u32) -> windows_core::BOOL);
    unsafe { CryptEncodeObjectEx(dwcertencodingtype, lpszstructtype.param().abi(), pvstructinfo, dwflags, pencodepara.unwrap_or(core::mem::zeroed()) as _, pvencoded.unwrap_or(core::mem::zeroed()) as _, pcbencoded as _).ok() }
}
#[inline]
pub unsafe fn CryptEncrypt(hkey: usize, hhash: usize, r#final: bool, dwflags: u32, pbdata: Option<&mut [u8]>, pdwdatalen: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("advapi32.dll" "system" fn CryptEncrypt(hkey : usize, hhash : usize, r#final : windows_core::BOOL, dwflags : u32, pbdata : *mut u8, pdwdatalen : *mut u32, dwbuflen : u32) -> windows_core::BOOL);
    unsafe { CryptEncrypt(hkey, hhash, r#final.into(), dwflags, core::mem::transmute(pbdata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pdwdatalen as _, pbdata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())).ok() }
}
#[inline]
pub unsafe fn CryptEncryptMessage(pencryptpara: *const CRYPT_ENCRYPT_MESSAGE_PARA, rgprecipientcert: &[*const CERT_CONTEXT], pbtobeencrypted: Option<&[u8]>, pbencryptedblob: Option<*mut u8>, pcbencryptedblob: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CryptEncryptMessage(pencryptpara : *const CRYPT_ENCRYPT_MESSAGE_PARA, crecipientcert : u32, rgprecipientcert : *const *const CERT_CONTEXT, pbtobeencrypted : *const u8, cbtobeencrypted : u32, pbencryptedblob : *mut u8, pcbencryptedblob : *mut u32) -> windows_core::BOOL);
    unsafe { CryptEncryptMessage(pencryptpara, rgprecipientcert.len().try_into().unwrap(), core::mem::transmute(rgprecipientcert.as_ptr()), core::mem::transmute(pbtobeencrypted.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pbtobeencrypted.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pbencryptedblob.unwrap_or(core::mem::zeroed()) as _, pcbencryptedblob as _).ok() }
}
#[inline]
pub unsafe fn CryptEnumKeyIdentifierProperties<P3>(pkeyidentifier: Option<*const CRYPT_INTEGER_BLOB>, dwpropid: u32, dwflags: u32, pwszcomputername: P3, pvreserved: Option<*const core::ffi::c_void>, pvarg: Option<*mut core::ffi::c_void>, pfnenum: PFN_CRYPT_ENUM_KEYID_PROP) -> windows_core::Result<()>
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("crypt32.dll" "system" fn CryptEnumKeyIdentifierProperties(pkeyidentifier : *const CRYPT_INTEGER_BLOB, dwpropid : u32, dwflags : u32, pwszcomputername : windows_core::PCWSTR, pvreserved : *const core::ffi::c_void, pvarg : *mut core::ffi::c_void, pfnenum : PFN_CRYPT_ENUM_KEYID_PROP) -> windows_core::BOOL);
    unsafe { CryptEnumKeyIdentifierProperties(pkeyidentifier.unwrap_or(core::mem::zeroed()) as _, dwpropid, dwflags, pwszcomputername.param().abi(), pvreserved.unwrap_or(core::mem::zeroed()) as _, pvarg.unwrap_or(core::mem::zeroed()) as _, pfnenum).ok() }
}
#[inline]
pub unsafe fn CryptEnumOIDFunction<P1, P2>(dwencodingtype: u32, pszfuncname: P1, pszoid: P2, dwflags: u32, pvarg: Option<*mut core::ffi::c_void>, pfnenumoidfunc: PFN_CRYPT_ENUM_OID_FUNC) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("crypt32.dll" "system" fn CryptEnumOIDFunction(dwencodingtype : u32, pszfuncname : windows_core::PCSTR, pszoid : windows_core::PCSTR, dwflags : u32, pvarg : *mut core::ffi::c_void, pfnenumoidfunc : PFN_CRYPT_ENUM_OID_FUNC) -> windows_core::BOOL);
    unsafe { CryptEnumOIDFunction(dwencodingtype, pszfuncname.param().abi(), pszoid.param().abi(), dwflags, pvarg.unwrap_or(core::mem::zeroed()) as _, pfnenumoidfunc).ok() }
}
#[inline]
pub unsafe fn CryptEnumOIDInfo(dwgroupid: u32, dwflags: u32, pvarg: Option<*mut core::ffi::c_void>, pfnenumoidinfo: PFN_CRYPT_ENUM_OID_INFO) -> windows_core::BOOL {
    windows_core::link!("crypt32.dll" "system" fn CryptEnumOIDInfo(dwgroupid : u32, dwflags : u32, pvarg : *mut core::ffi::c_void, pfnenumoidinfo : PFN_CRYPT_ENUM_OID_INFO) -> windows_core::BOOL);
    unsafe { CryptEnumOIDInfo(dwgroupid, dwflags, pvarg.unwrap_or(core::mem::zeroed()) as _, pfnenumoidinfo) }
}
#[inline]
pub unsafe fn CryptEnumProviderTypesA(dwindex: u32, pdwreserved: Option<*const u32>, dwflags: u32, pdwprovtype: *mut u32, sztypename: Option<windows_core::PSTR>, pcbtypename: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("advapi32.dll" "system" fn CryptEnumProviderTypesA(dwindex : u32, pdwreserved : *const u32, dwflags : u32, pdwprovtype : *mut u32, sztypename : windows_core::PSTR, pcbtypename : *mut u32) -> windows_core::BOOL);
    unsafe { CryptEnumProviderTypesA(dwindex, pdwreserved.unwrap_or(core::mem::zeroed()) as _, dwflags, pdwprovtype as _, sztypename.unwrap_or(core::mem::zeroed()) as _, pcbtypename as _).ok() }
}
#[inline]
pub unsafe fn CryptEnumProviderTypesW(dwindex: u32, pdwreserved: Option<*const u32>, dwflags: u32, pdwprovtype: *mut u32, sztypename: Option<windows_core::PWSTR>, pcbtypename: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("advapi32.dll" "system" fn CryptEnumProviderTypesW(dwindex : u32, pdwreserved : *const u32, dwflags : u32, pdwprovtype : *mut u32, sztypename : windows_core::PWSTR, pcbtypename : *mut u32) -> windows_core::BOOL);
    unsafe { CryptEnumProviderTypesW(dwindex, pdwreserved.unwrap_or(core::mem::zeroed()) as _, dwflags, pdwprovtype as _, sztypename.unwrap_or(core::mem::zeroed()) as _, pcbtypename as _).ok() }
}
#[inline]
pub unsafe fn CryptEnumProvidersA(dwindex: u32, pdwreserved: Option<*const u32>, dwflags: u32, pdwprovtype: *mut u32, szprovname: Option<windows_core::PSTR>, pcbprovname: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("advapi32.dll" "system" fn CryptEnumProvidersA(dwindex : u32, pdwreserved : *const u32, dwflags : u32, pdwprovtype : *mut u32, szprovname : windows_core::PSTR, pcbprovname : *mut u32) -> windows_core::BOOL);
    unsafe { CryptEnumProvidersA(dwindex, pdwreserved.unwrap_or(core::mem::zeroed()) as _, dwflags, pdwprovtype as _, szprovname.unwrap_or(core::mem::zeroed()) as _, pcbprovname as _).ok() }
}
#[inline]
pub unsafe fn CryptEnumProvidersW(dwindex: u32, pdwreserved: Option<*const u32>, dwflags: u32, pdwprovtype: *mut u32, szprovname: Option<windows_core::PWSTR>, pcbprovname: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("advapi32.dll" "system" fn CryptEnumProvidersW(dwindex : u32, pdwreserved : *const u32, dwflags : u32, pdwprovtype : *mut u32, szprovname : windows_core::PWSTR, pcbprovname : *mut u32) -> windows_core::BOOL);
    unsafe { CryptEnumProvidersW(dwindex, pdwreserved.unwrap_or(core::mem::zeroed()) as _, dwflags, pdwprovtype as _, szprovname.unwrap_or(core::mem::zeroed()) as _, pcbprovname as _).ok() }
}
#[inline]
pub unsafe fn CryptExportKey(hkey: usize, hexpkey: usize, dwblobtype: u32, dwflags: CRYPT_KEY_FLAGS, pbdata: Option<*mut u8>, pdwdatalen: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("advapi32.dll" "system" fn CryptExportKey(hkey : usize, hexpkey : usize, dwblobtype : u32, dwflags : CRYPT_KEY_FLAGS, pbdata : *mut u8, pdwdatalen : *mut u32) -> windows_core::BOOL);
    unsafe { CryptExportKey(hkey, hexpkey, dwblobtype, dwflags, pbdata.unwrap_or(core::mem::zeroed()) as _, pdwdatalen as _).ok() }
}
#[inline]
pub unsafe fn CryptExportPKCS8<P2>(hcryptprov: usize, dwkeyspec: u32, pszprivatekeyobjid: P2, dwflags: u32, pvauxinfo: Option<*const core::ffi::c_void>, pbprivatekeyblob: Option<*mut u8>, pcbprivatekeyblob: *mut u32) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("crypt32.dll" "system" fn CryptExportPKCS8(hcryptprov : usize, dwkeyspec : u32, pszprivatekeyobjid : windows_core::PCSTR, dwflags : u32, pvauxinfo : *const core::ffi::c_void, pbprivatekeyblob : *mut u8, pcbprivatekeyblob : *mut u32) -> windows_core::BOOL);
    unsafe { CryptExportPKCS8(hcryptprov, dwkeyspec, pszprivatekeyobjid.param().abi(), dwflags, pvauxinfo.unwrap_or(core::mem::zeroed()) as _, pbprivatekeyblob.unwrap_or(core::mem::zeroed()) as _, pcbprivatekeyblob as _).ok() }
}
#[inline]
pub unsafe fn CryptExportPublicKeyInfo(hcryptprovorncryptkey: HCRYPTPROV_OR_NCRYPT_KEY_HANDLE, dwkeyspec: Option<u32>, dwcertencodingtype: CERT_QUERY_ENCODING_TYPE, pinfo: Option<*mut CERT_PUBLIC_KEY_INFO>, pcbinfo: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CryptExportPublicKeyInfo(hcryptprovorncryptkey : HCRYPTPROV_OR_NCRYPT_KEY_HANDLE, dwkeyspec : u32, dwcertencodingtype : CERT_QUERY_ENCODING_TYPE, pinfo : *mut CERT_PUBLIC_KEY_INFO, pcbinfo : *mut u32) -> windows_core::BOOL);
    unsafe { CryptExportPublicKeyInfo(hcryptprovorncryptkey, dwkeyspec.unwrap_or(core::mem::zeroed()) as _, dwcertencodingtype, pinfo.unwrap_or(core::mem::zeroed()) as _, pcbinfo as _).ok() }
}
#[inline]
pub unsafe fn CryptExportPublicKeyInfoEx<P3>(hcryptprovorncryptkey: HCRYPTPROV_OR_NCRYPT_KEY_HANDLE, dwkeyspec: Option<u32>, dwcertencodingtype: CERT_QUERY_ENCODING_TYPE, pszpublickeyobjid: P3, dwflags: u32, pvauxinfo: Option<*const core::ffi::c_void>, pinfo: Option<*mut CERT_PUBLIC_KEY_INFO>, pcbinfo: *mut u32) -> windows_core::Result<()>
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("crypt32.dll" "system" fn CryptExportPublicKeyInfoEx(hcryptprovorncryptkey : HCRYPTPROV_OR_NCRYPT_KEY_HANDLE, dwkeyspec : u32, dwcertencodingtype : CERT_QUERY_ENCODING_TYPE, pszpublickeyobjid : windows_core::PCSTR, dwflags : u32, pvauxinfo : *const core::ffi::c_void, pinfo : *mut CERT_PUBLIC_KEY_INFO, pcbinfo : *mut u32) -> windows_core::BOOL);
    unsafe { CryptExportPublicKeyInfoEx(hcryptprovorncryptkey, dwkeyspec.unwrap_or(core::mem::zeroed()) as _, dwcertencodingtype, pszpublickeyobjid.param().abi(), dwflags, pvauxinfo.unwrap_or(core::mem::zeroed()) as _, pinfo.unwrap_or(core::mem::zeroed()) as _, pcbinfo as _).ok() }
}
#[inline]
pub unsafe fn CryptExportPublicKeyInfoFromBCryptKeyHandle<P2>(hbcryptkey: BCRYPT_KEY_HANDLE, dwcertencodingtype: CERT_QUERY_ENCODING_TYPE, pszpublickeyobjid: P2, dwflags: u32, pvauxinfo: Option<*const core::ffi::c_void>, pinfo: Option<*mut CERT_PUBLIC_KEY_INFO>, pcbinfo: *mut u32) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("crypt32.dll" "system" fn CryptExportPublicKeyInfoFromBCryptKeyHandle(hbcryptkey : BCRYPT_KEY_HANDLE, dwcertencodingtype : CERT_QUERY_ENCODING_TYPE, pszpublickeyobjid : windows_core::PCSTR, dwflags : u32, pvauxinfo : *const core::ffi::c_void, pinfo : *mut CERT_PUBLIC_KEY_INFO, pcbinfo : *mut u32) -> windows_core::BOOL);
    unsafe { CryptExportPublicKeyInfoFromBCryptKeyHandle(hbcryptkey, dwcertencodingtype, pszpublickeyobjid.param().abi(), dwflags, pvauxinfo.unwrap_or(core::mem::zeroed()) as _, pinfo.unwrap_or(core::mem::zeroed()) as _, pcbinfo as _) }
}
#[inline]
pub unsafe fn CryptFindCertificateKeyProvInfo(pcert: *const CERT_CONTEXT, dwflags: CRYPT_FIND_FLAGS, pvreserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CryptFindCertificateKeyProvInfo(pcert : *const CERT_CONTEXT, dwflags : CRYPT_FIND_FLAGS, pvreserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { CryptFindCertificateKeyProvInfo(pcert, dwflags, pvreserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CryptFindLocalizedName<P0>(pwszcryptname: P0) -> windows_core::PCWSTR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("crypt32.dll" "system" fn CryptFindLocalizedName(pwszcryptname : windows_core::PCWSTR) -> windows_core::PCWSTR);
    unsafe { CryptFindLocalizedName(pwszcryptname.param().abi()) }
}
#[inline]
pub unsafe fn CryptFindOIDInfo(dwkeytype: u32, pvkey: *const core::ffi::c_void, dwgroupid: u32) -> *mut CRYPT_OID_INFO {
    windows_core::link!("crypt32.dll" "system" fn CryptFindOIDInfo(dwkeytype : u32, pvkey : *const core::ffi::c_void, dwgroupid : u32) -> *mut CRYPT_OID_INFO);
    unsafe { CryptFindOIDInfo(dwkeytype, pvkey, dwgroupid) }
}
#[inline]
pub unsafe fn CryptFormatObject<P4>(dwcertencodingtype: CERT_QUERY_ENCODING_TYPE, dwformattype: u32, dwformatstrtype: u32, pformatstruct: Option<*const core::ffi::c_void>, lpszstructtype: P4, pbencoded: &[u8], pbformat: Option<*mut core::ffi::c_void>, pcbformat: *mut u32) -> windows_core::Result<()>
where
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("crypt32.dll" "system" fn CryptFormatObject(dwcertencodingtype : CERT_QUERY_ENCODING_TYPE, dwformattype : u32, dwformatstrtype : u32, pformatstruct : *const core::ffi::c_void, lpszstructtype : windows_core::PCSTR, pbencoded : *const u8, cbencoded : u32, pbformat : *mut core::ffi::c_void, pcbformat : *mut u32) -> windows_core::BOOL);
    unsafe { CryptFormatObject(dwcertencodingtype, dwformattype, dwformatstrtype, pformatstruct.unwrap_or(core::mem::zeroed()) as _, lpszstructtype.param().abi(), core::mem::transmute(pbencoded.as_ptr()), pbencoded.len().try_into().unwrap(), pbformat.unwrap_or(core::mem::zeroed()) as _, pcbformat as _).ok() }
}
#[inline]
pub unsafe fn CryptFreeOIDFunctionAddress(hfuncaddr: *const core::ffi::c_void, dwflags: u32) -> windows_core::BOOL {
    windows_core::link!("crypt32.dll" "system" fn CryptFreeOIDFunctionAddress(hfuncaddr : *const core::ffi::c_void, dwflags : u32) -> windows_core::BOOL);
    unsafe { CryptFreeOIDFunctionAddress(hfuncaddr, dwflags) }
}
#[inline]
pub unsafe fn CryptGenKey(hprov: usize, algid: ALG_ID, dwflags: CRYPT_KEY_FLAGS, phkey: *mut usize) -> windows_core::Result<()> {
    windows_core::link!("advapi32.dll" "system" fn CryptGenKey(hprov : usize, algid : ALG_ID, dwflags : CRYPT_KEY_FLAGS, phkey : *mut usize) -> windows_core::BOOL);
    unsafe { CryptGenKey(hprov, algid, dwflags, phkey as _).ok() }
}
#[inline]
pub unsafe fn CryptGenRandom(hprov: usize, pbbuffer: &mut [u8]) -> windows_core::Result<()> {
    windows_core::link!("advapi32.dll" "system" fn CryptGenRandom(hprov : usize, dwlen : u32, pbbuffer : *mut u8) -> windows_core::BOOL);
    unsafe { CryptGenRandom(hprov, pbbuffer.len().try_into().unwrap(), core::mem::transmute(pbbuffer.as_ptr())).ok() }
}
#[inline]
pub unsafe fn CryptGetAsyncParam<P1>(hasync: HCRYPTASYNC, pszparamoid: P1, ppvparam: Option<*mut *mut core::ffi::c_void>, ppfnfree: Option<*mut PFN_CRYPT_ASYNC_PARAM_FREE_FUNC>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("crypt32.dll" "system" fn CryptGetAsyncParam(hasync : HCRYPTASYNC, pszparamoid : windows_core::PCSTR, ppvparam : *mut *mut core::ffi::c_void, ppfnfree : *mut PFN_CRYPT_ASYNC_PARAM_FREE_FUNC) -> windows_core::BOOL);
    unsafe { CryptGetAsyncParam(hasync, pszparamoid.param().abi(), ppvparam.unwrap_or(core::mem::zeroed()) as _, ppfnfree.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CryptGetDefaultOIDDllList(hfuncset: *const core::ffi::c_void, dwencodingtype: u32, pwszdlllist: Option<windows_core::PWSTR>, pcchdlllist: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CryptGetDefaultOIDDllList(hfuncset : *const core::ffi::c_void, dwencodingtype : u32, pwszdlllist : windows_core::PWSTR, pcchdlllist : *mut u32) -> windows_core::BOOL);
    unsafe { CryptGetDefaultOIDDllList(hfuncset, dwencodingtype, pwszdlllist.unwrap_or(core::mem::zeroed()) as _, pcchdlllist as _).ok() }
}
#[inline]
pub unsafe fn CryptGetDefaultOIDFunctionAddress<P2>(hfuncset: *const core::ffi::c_void, dwencodingtype: u32, pwszdll: P2, dwflags: u32, ppvfuncaddr: *mut *mut core::ffi::c_void, phfuncaddr: *mut *mut core::ffi::c_void) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("crypt32.dll" "system" fn CryptGetDefaultOIDFunctionAddress(hfuncset : *const core::ffi::c_void, dwencodingtype : u32, pwszdll : windows_core::PCWSTR, dwflags : u32, ppvfuncaddr : *mut *mut core::ffi::c_void, phfuncaddr : *mut *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { CryptGetDefaultOIDFunctionAddress(hfuncset, dwencodingtype, pwszdll.param().abi(), dwflags, ppvfuncaddr as _, phfuncaddr as _) }
}
#[inline]
pub unsafe fn CryptGetDefaultProviderA(dwprovtype: u32, pdwreserved: Option<*const u32>, dwflags: u32, pszprovname: Option<windows_core::PSTR>, pcbprovname: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("advapi32.dll" "system" fn CryptGetDefaultProviderA(dwprovtype : u32, pdwreserved : *const u32, dwflags : u32, pszprovname : windows_core::PSTR, pcbprovname : *mut u32) -> windows_core::BOOL);
    unsafe { CryptGetDefaultProviderA(dwprovtype, pdwreserved.unwrap_or(core::mem::zeroed()) as _, dwflags, pszprovname.unwrap_or(core::mem::zeroed()) as _, pcbprovname as _).ok() }
}
#[inline]
pub unsafe fn CryptGetDefaultProviderW(dwprovtype: u32, pdwreserved: Option<*const u32>, dwflags: u32, pszprovname: Option<windows_core::PWSTR>, pcbprovname: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("advapi32.dll" "system" fn CryptGetDefaultProviderW(dwprovtype : u32, pdwreserved : *const u32, dwflags : u32, pszprovname : windows_core::PWSTR, pcbprovname : *mut u32) -> windows_core::BOOL);
    unsafe { CryptGetDefaultProviderW(dwprovtype, pdwreserved.unwrap_or(core::mem::zeroed()) as _, dwflags, pszprovname.unwrap_or(core::mem::zeroed()) as _, pcbprovname as _).ok() }
}
#[inline]
pub unsafe fn CryptGetHashParam(hhash: usize, dwparam: u32, pbdata: Option<*mut u8>, pdwdatalen: *mut u32, dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("advapi32.dll" "system" fn CryptGetHashParam(hhash : usize, dwparam : u32, pbdata : *mut u8, pdwdatalen : *mut u32, dwflags : u32) -> windows_core::BOOL);
    unsafe { CryptGetHashParam(hhash, dwparam, pbdata.unwrap_or(core::mem::zeroed()) as _, pdwdatalen as _, dwflags).ok() }
}
#[inline]
pub unsafe fn CryptGetKeyIdentifierProperty<P3>(pkeyidentifier: *const CRYPT_INTEGER_BLOB, dwpropid: u32, dwflags: u32, pwszcomputername: P3, pvreserved: Option<*const core::ffi::c_void>, pvdata: Option<*mut core::ffi::c_void>, pcbdata: *mut u32) -> windows_core::Result<()>
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("crypt32.dll" "system" fn CryptGetKeyIdentifierProperty(pkeyidentifier : *const CRYPT_INTEGER_BLOB, dwpropid : u32, dwflags : u32, pwszcomputername : windows_core::PCWSTR, pvreserved : *const core::ffi::c_void, pvdata : *mut core::ffi::c_void, pcbdata : *mut u32) -> windows_core::BOOL);
    unsafe { CryptGetKeyIdentifierProperty(pkeyidentifier, dwpropid, dwflags, pwszcomputername.param().abi(), pvreserved.unwrap_or(core::mem::zeroed()) as _, pvdata.unwrap_or(core::mem::zeroed()) as _, pcbdata as _).ok() }
}
#[inline]
pub unsafe fn CryptGetKeyParam(hkey: usize, dwparam: CRYPT_KEY_PARAM_ID, pbdata: Option<*mut u8>, pdwdatalen: *mut u32, dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("advapi32.dll" "system" fn CryptGetKeyParam(hkey : usize, dwparam : CRYPT_KEY_PARAM_ID, pbdata : *mut u8, pdwdatalen : *mut u32, dwflags : u32) -> windows_core::BOOL);
    unsafe { CryptGetKeyParam(hkey, dwparam, pbdata.unwrap_or(core::mem::zeroed()) as _, pdwdatalen as _, dwflags).ok() }
}
#[inline]
pub unsafe fn CryptGetMessageCertificates(dwmsgandcertencodingtype: u32, hcryptprov: Option<HCRYPTPROV_LEGACY>, dwflags: u32, pbsignedblob: &[u8]) -> windows_core::Result<HCERTSTORE> {
    windows_core::link!("crypt32.dll" "system" fn CryptGetMessageCertificates(dwmsgandcertencodingtype : u32, hcryptprov : HCRYPTPROV_LEGACY, dwflags : u32, pbsignedblob : *const u8, cbsignedblob : u32) -> HCERTSTORE);
    let result__ = unsafe { CryptGetMessageCertificates(dwmsgandcertencodingtype, hcryptprov.unwrap_or(core::mem::zeroed()) as _, dwflags, core::mem::transmute(pbsignedblob.as_ptr()), pbsignedblob.len().try_into().unwrap()) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[inline]
pub unsafe fn CryptGetMessageSignerCount(dwmsgencodingtype: u32, pbsignedblob: &[u8]) -> i32 {
    windows_core::link!("crypt32.dll" "system" fn CryptGetMessageSignerCount(dwmsgencodingtype : u32, pbsignedblob : *const u8, cbsignedblob : u32) -> i32);
    unsafe { CryptGetMessageSignerCount(dwmsgencodingtype, core::mem::transmute(pbsignedblob.as_ptr()), pbsignedblob.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn CryptGetOIDFunctionAddress<P2>(hfuncset: *const core::ffi::c_void, dwencodingtype: u32, pszoid: P2, dwflags: u32, ppvfuncaddr: *mut *mut core::ffi::c_void, phfuncaddr: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("crypt32.dll" "system" fn CryptGetOIDFunctionAddress(hfuncset : *const core::ffi::c_void, dwencodingtype : u32, pszoid : windows_core::PCSTR, dwflags : u32, ppvfuncaddr : *mut *mut core::ffi::c_void, phfuncaddr : *mut *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { CryptGetOIDFunctionAddress(hfuncset, dwencodingtype, pszoid.param().abi(), dwflags, ppvfuncaddr as _, phfuncaddr as _).ok() }
}
#[inline]
pub unsafe fn CryptGetOIDFunctionValue<P1, P2, P3>(dwencodingtype: u32, pszfuncname: P1, pszoid: P2, pwszvaluename: P3, pdwvaluetype: Option<*mut u32>, pbvaluedata: Option<*mut u8>, pcbvaluedata: Option<*mut u32>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("crypt32.dll" "system" fn CryptGetOIDFunctionValue(dwencodingtype : u32, pszfuncname : windows_core::PCSTR, pszoid : windows_core::PCSTR, pwszvaluename : windows_core::PCWSTR, pdwvaluetype : *mut u32, pbvaluedata : *mut u8, pcbvaluedata : *mut u32) -> windows_core::BOOL);
    unsafe { CryptGetOIDFunctionValue(dwencodingtype, pszfuncname.param().abi(), pszoid.param().abi(), pwszvaluename.param().abi(), pdwvaluetype.unwrap_or(core::mem::zeroed()) as _, pbvaluedata.unwrap_or(core::mem::zeroed()) as _, pcbvaluedata.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CryptGetObjectUrl<P0>(pszurloid: P0, pvpara: *const core::ffi::c_void, dwflags: CRYPT_GET_URL_FLAGS, purlarray: Option<*mut CRYPT_URL_ARRAY>, pcburlarray: *mut u32, purlinfo: Option<*mut CRYPT_URL_INFO>, pcburlinfo: Option<*mut u32>, pvreserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("cryptnet.dll" "system" fn CryptGetObjectUrl(pszurloid : windows_core::PCSTR, pvpara : *const core::ffi::c_void, dwflags : CRYPT_GET_URL_FLAGS, purlarray : *mut CRYPT_URL_ARRAY, pcburlarray : *mut u32, purlinfo : *mut CRYPT_URL_INFO, pcburlinfo : *mut u32, pvreserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { CryptGetObjectUrl(pszurloid.param().abi(), pvpara, dwflags, purlarray.unwrap_or(core::mem::zeroed()) as _, pcburlarray as _, purlinfo.unwrap_or(core::mem::zeroed()) as _, pcburlinfo.unwrap_or(core::mem::zeroed()) as _, pvreserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CryptGetProvParam(hprov: usize, dwparam: u32, pbdata: Option<*mut u8>, pdwdatalen: *mut u32, dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("advapi32.dll" "system" fn CryptGetProvParam(hprov : usize, dwparam : u32, pbdata : *mut u8, pdwdatalen : *mut u32, dwflags : u32) -> windows_core::BOOL);
    unsafe { CryptGetProvParam(hprov, dwparam, pbdata.unwrap_or(core::mem::zeroed()) as _, pdwdatalen as _, dwflags).ok() }
}
#[inline]
pub unsafe fn CryptGetUserKey(hprov: usize, dwkeyspec: u32, phuserkey: *mut usize) -> windows_core::Result<()> {
    windows_core::link!("advapi32.dll" "system" fn CryptGetUserKey(hprov : usize, dwkeyspec : u32, phuserkey : *mut usize) -> windows_core::BOOL);
    unsafe { CryptGetUserKey(hprov, dwkeyspec, phuserkey as _).ok() }
}
#[inline]
pub unsafe fn CryptHashCertificate(hcryptprov: Option<HCRYPTPROV_LEGACY>, algid: ALG_ID, dwflags: u32, pbencoded: &[u8], pbcomputedhash: Option<*mut u8>, pcbcomputedhash: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CryptHashCertificate(hcryptprov : HCRYPTPROV_LEGACY, algid : ALG_ID, dwflags : u32, pbencoded : *const u8, cbencoded : u32, pbcomputedhash : *mut u8, pcbcomputedhash : *mut u32) -> windows_core::BOOL);
    unsafe { CryptHashCertificate(hcryptprov.unwrap_or(core::mem::zeroed()) as _, algid, dwflags, core::mem::transmute(pbencoded.as_ptr()), pbencoded.len().try_into().unwrap(), pbcomputedhash.unwrap_or(core::mem::zeroed()) as _, pcbcomputedhash as _).ok() }
}
#[inline]
pub unsafe fn CryptHashCertificate2<P0>(pwszcnghashalgid: P0, dwflags: u32, pvreserved: Option<*const core::ffi::c_void>, pbencoded: Option<&[u8]>, pbcomputedhash: Option<*mut u8>, pcbcomputedhash: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("crypt32.dll" "system" fn CryptHashCertificate2(pwszcnghashalgid : windows_core::PCWSTR, dwflags : u32, pvreserved : *const core::ffi::c_void, pbencoded : *const u8, cbencoded : u32, pbcomputedhash : *mut u8, pcbcomputedhash : *mut u32) -> windows_core::BOOL);
    unsafe { CryptHashCertificate2(pwszcnghashalgid.param().abi(), dwflags, pvreserved.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pbencoded.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pbencoded.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pbcomputedhash.unwrap_or(core::mem::zeroed()) as _, pcbcomputedhash as _).ok() }
}
#[inline]
pub unsafe fn CryptHashData(hhash: usize, pbdata: &[u8], dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("advapi32.dll" "system" fn CryptHashData(hhash : usize, pbdata : *const u8, dwdatalen : u32, dwflags : u32) -> windows_core::BOOL);
    unsafe { CryptHashData(hhash, core::mem::transmute(pbdata.as_ptr()), pbdata.len().try_into().unwrap(), dwflags).ok() }
}
#[inline]
pub unsafe fn CryptHashMessage(phashpara: *const CRYPT_HASH_MESSAGE_PARA, fdetachedhash: bool, ctobehashed: u32, rgpbtobehashed: *const *const u8, rgcbtobehashed: *const u32, pbhashedblob: Option<*mut u8>, pcbhashedblob: Option<*mut u32>, pbcomputedhash: Option<*mut u8>, pcbcomputedhash: Option<*mut u32>) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CryptHashMessage(phashpara : *const CRYPT_HASH_MESSAGE_PARA, fdetachedhash : windows_core::BOOL, ctobehashed : u32, rgpbtobehashed : *const *const u8, rgcbtobehashed : *const u32, pbhashedblob : *mut u8, pcbhashedblob : *mut u32, pbcomputedhash : *mut u8, pcbcomputedhash : *mut u32) -> windows_core::BOOL);
    unsafe { CryptHashMessage(phashpara, fdetachedhash.into(), ctobehashed, rgpbtobehashed, rgcbtobehashed, pbhashedblob.unwrap_or(core::mem::zeroed()) as _, pcbhashedblob.unwrap_or(core::mem::zeroed()) as _, pbcomputedhash.unwrap_or(core::mem::zeroed()) as _, pcbcomputedhash.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CryptHashPublicKeyInfo(hcryptprov: Option<HCRYPTPROV_LEGACY>, algid: ALG_ID, dwflags: u32, dwcertencodingtype: CERT_QUERY_ENCODING_TYPE, pinfo: *const CERT_PUBLIC_KEY_INFO, pbcomputedhash: Option<*mut u8>, pcbcomputedhash: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CryptHashPublicKeyInfo(hcryptprov : HCRYPTPROV_LEGACY, algid : ALG_ID, dwflags : u32, dwcertencodingtype : CERT_QUERY_ENCODING_TYPE, pinfo : *const CERT_PUBLIC_KEY_INFO, pbcomputedhash : *mut u8, pcbcomputedhash : *mut u32) -> windows_core::BOOL);
    unsafe { CryptHashPublicKeyInfo(hcryptprov.unwrap_or(core::mem::zeroed()) as _, algid, dwflags, dwcertencodingtype, pinfo, pbcomputedhash.unwrap_or(core::mem::zeroed()) as _, pcbcomputedhash as _).ok() }
}
#[inline]
pub unsafe fn CryptHashSessionKey(hhash: usize, hkey: usize, dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("advapi32.dll" "system" fn CryptHashSessionKey(hhash : usize, hkey : usize, dwflags : u32) -> windows_core::BOOL);
    unsafe { CryptHashSessionKey(hhash, hkey, dwflags).ok() }
}
#[inline]
pub unsafe fn CryptHashToBeSigned(hcryptprov: Option<HCRYPTPROV_LEGACY>, dwcertencodingtype: CERT_QUERY_ENCODING_TYPE, pbencoded: &[u8], pbcomputedhash: Option<*mut u8>, pcbcomputedhash: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CryptHashToBeSigned(hcryptprov : HCRYPTPROV_LEGACY, dwcertencodingtype : CERT_QUERY_ENCODING_TYPE, pbencoded : *const u8, cbencoded : u32, pbcomputedhash : *mut u8, pcbcomputedhash : *mut u32) -> windows_core::BOOL);
    unsafe { CryptHashToBeSigned(hcryptprov.unwrap_or(core::mem::zeroed()) as _, dwcertencodingtype, core::mem::transmute(pbencoded.as_ptr()), pbencoded.len().try_into().unwrap(), pbcomputedhash.unwrap_or(core::mem::zeroed()) as _, pcbcomputedhash as _).ok() }
}
#[inline]
pub unsafe fn CryptImportKey(hprov: usize, pbdata: &[u8], hpubkey: usize, dwflags: CRYPT_KEY_FLAGS, phkey: *mut usize) -> windows_core::Result<()> {
    windows_core::link!("advapi32.dll" "system" fn CryptImportKey(hprov : usize, pbdata : *const u8, dwdatalen : u32, hpubkey : usize, dwflags : CRYPT_KEY_FLAGS, phkey : *mut usize) -> windows_core::BOOL);
    unsafe { CryptImportKey(hprov, core::mem::transmute(pbdata.as_ptr()), pbdata.len().try_into().unwrap(), hpubkey, dwflags, phkey as _).ok() }
}
#[inline]
pub unsafe fn CryptImportPKCS8(sprivatekeyandparams: CRYPT_PKCS8_IMPORT_PARAMS, dwflags: CRYPT_KEY_FLAGS, phcryptprov: Option<*mut usize>, pvauxinfo: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CryptImportPKCS8(sprivatekeyandparams : CRYPT_PKCS8_IMPORT_PARAMS, dwflags : CRYPT_KEY_FLAGS, phcryptprov : *mut usize, pvauxinfo : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { CryptImportPKCS8(core::mem::transmute(sprivatekeyandparams), dwflags, phcryptprov.unwrap_or(core::mem::zeroed()) as _, pvauxinfo.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CryptImportPublicKeyInfo(hcryptprov: usize, dwcertencodingtype: CERT_QUERY_ENCODING_TYPE, pinfo: *const CERT_PUBLIC_KEY_INFO, phkey: *mut usize) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CryptImportPublicKeyInfo(hcryptprov : usize, dwcertencodingtype : CERT_QUERY_ENCODING_TYPE, pinfo : *const CERT_PUBLIC_KEY_INFO, phkey : *mut usize) -> windows_core::BOOL);
    unsafe { CryptImportPublicKeyInfo(hcryptprov, dwcertencodingtype, pinfo, phkey as _).ok() }
}
#[inline]
pub unsafe fn CryptImportPublicKeyInfoEx(hcryptprov: usize, dwcertencodingtype: CERT_QUERY_ENCODING_TYPE, pinfo: *const CERT_PUBLIC_KEY_INFO, aikeyalg: ALG_ID, dwflags: u32, pvauxinfo: Option<*const core::ffi::c_void>, phkey: *mut usize) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CryptImportPublicKeyInfoEx(hcryptprov : usize, dwcertencodingtype : CERT_QUERY_ENCODING_TYPE, pinfo : *const CERT_PUBLIC_KEY_INFO, aikeyalg : ALG_ID, dwflags : u32, pvauxinfo : *const core::ffi::c_void, phkey : *mut usize) -> windows_core::BOOL);
    unsafe { CryptImportPublicKeyInfoEx(hcryptprov, dwcertencodingtype, pinfo, aikeyalg, dwflags, pvauxinfo.unwrap_or(core::mem::zeroed()) as _, phkey as _).ok() }
}
#[inline]
pub unsafe fn CryptImportPublicKeyInfoEx2(dwcertencodingtype: CERT_QUERY_ENCODING_TYPE, pinfo: *const CERT_PUBLIC_KEY_INFO, dwflags: CRYPT_IMPORT_PUBLIC_KEY_FLAGS, pvauxinfo: Option<*const core::ffi::c_void>, phkey: *mut BCRYPT_KEY_HANDLE) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CryptImportPublicKeyInfoEx2(dwcertencodingtype : CERT_QUERY_ENCODING_TYPE, pinfo : *const CERT_PUBLIC_KEY_INFO, dwflags : CRYPT_IMPORT_PUBLIC_KEY_FLAGS, pvauxinfo : *const core::ffi::c_void, phkey : *mut BCRYPT_KEY_HANDLE) -> windows_core::BOOL);
    unsafe { CryptImportPublicKeyInfoEx2(dwcertencodingtype, pinfo, dwflags, pvauxinfo.unwrap_or(core::mem::zeroed()) as _, phkey as _).ok() }
}
#[inline]
pub unsafe fn CryptInitOIDFunctionSet<P0>(pszfuncname: P0, dwflags: u32) -> *mut core::ffi::c_void
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("crypt32.dll" "system" fn CryptInitOIDFunctionSet(pszfuncname : windows_core::PCSTR, dwflags : u32) -> *mut core::ffi::c_void);
    unsafe { CryptInitOIDFunctionSet(pszfuncname.param().abi(), dwflags) }
}
#[inline]
pub unsafe fn CryptInstallCancelRetrieval(pfncancel: PFN_CRYPT_CANCEL_RETRIEVAL, pvarg: Option<*const core::ffi::c_void>, dwflags: u32, pvreserved: Option<*const core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("cryptnet.dll" "system" fn CryptInstallCancelRetrieval(pfncancel : PFN_CRYPT_CANCEL_RETRIEVAL, pvarg : *const core::ffi::c_void, dwflags : u32, pvreserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { CryptInstallCancelRetrieval(pfncancel, pvarg.unwrap_or(core::mem::zeroed()) as _, dwflags, pvreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CryptInstallDefaultContext(hcryptprov: usize, dwdefaulttype: CRYPT_DEFAULT_CONTEXT_TYPE, pvdefaultpara: Option<*const core::ffi::c_void>, dwflags: CRYPT_DEFAULT_CONTEXT_FLAGS, pvreserved: Option<*const core::ffi::c_void>, phdefaultcontext: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CryptInstallDefaultContext(hcryptprov : usize, dwdefaulttype : CRYPT_DEFAULT_CONTEXT_TYPE, pvdefaultpara : *const core::ffi::c_void, dwflags : CRYPT_DEFAULT_CONTEXT_FLAGS, pvreserved : *const core::ffi::c_void, phdefaultcontext : *mut *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { CryptInstallDefaultContext(hcryptprov, dwdefaulttype, pvdefaultpara.unwrap_or(core::mem::zeroed()) as _, dwflags, pvreserved.unwrap_or(core::mem::zeroed()) as _, phdefaultcontext as _).ok() }
}
#[inline]
pub unsafe fn CryptInstallOIDFunctionAddress<P2>(hmodule: Option<super::super::Foundation::HMODULE>, dwencodingtype: u32, pszfuncname: P2, rgfuncentry: &[CRYPT_OID_FUNC_ENTRY], dwflags: u32) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("crypt32.dll" "system" fn CryptInstallOIDFunctionAddress(hmodule : super::super::Foundation:: HMODULE, dwencodingtype : u32, pszfuncname : windows_core::PCSTR, cfuncentry : u32, rgfuncentry : *const CRYPT_OID_FUNC_ENTRY, dwflags : u32) -> windows_core::BOOL);
    unsafe { CryptInstallOIDFunctionAddress(hmodule.unwrap_or(core::mem::zeroed()) as _, dwencodingtype, pszfuncname.param().abi(), rgfuncentry.len().try_into().unwrap(), core::mem::transmute(rgfuncentry.as_ptr()), dwflags) }
}
#[inline]
pub unsafe fn CryptMemAlloc(cbsize: u32) -> *mut core::ffi::c_void {
    windows_core::link!("crypt32.dll" "system" fn CryptMemAlloc(cbsize : u32) -> *mut core::ffi::c_void);
    unsafe { CryptMemAlloc(cbsize) }
}
#[inline]
pub unsafe fn CryptMemFree(pv: Option<*const core::ffi::c_void>) {
    windows_core::link!("crypt32.dll" "system" fn CryptMemFree(pv : *const core::ffi::c_void));
    unsafe { CryptMemFree(pv.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CryptMemRealloc(pv: Option<*const core::ffi::c_void>, cbsize: u32) -> *mut core::ffi::c_void {
    windows_core::link!("crypt32.dll" "system" fn CryptMemRealloc(pv : *const core::ffi::c_void, cbsize : u32) -> *mut core::ffi::c_void);
    unsafe { CryptMemRealloc(pv.unwrap_or(core::mem::zeroed()) as _, cbsize) }
}
#[inline]
pub unsafe fn CryptMsgCalculateEncodedLength<P4>(dwmsgencodingtype: u32, dwflags: u32, dwmsgtype: u32, pvmsgencodeinfo: *const core::ffi::c_void, pszinnercontentobjid: P4, cbdata: u32) -> u32
where
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("crypt32.dll" "system" fn CryptMsgCalculateEncodedLength(dwmsgencodingtype : u32, dwflags : u32, dwmsgtype : u32, pvmsgencodeinfo : *const core::ffi::c_void, pszinnercontentobjid : windows_core::PCSTR, cbdata : u32) -> u32);
    unsafe { CryptMsgCalculateEncodedLength(dwmsgencodingtype, dwflags, dwmsgtype, pvmsgencodeinfo, pszinnercontentobjid.param().abi(), cbdata) }
}
#[inline]
pub unsafe fn CryptMsgClose(hcryptmsg: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CryptMsgClose(hcryptmsg : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { CryptMsgClose(hcryptmsg.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CryptMsgControl(hcryptmsg: *const core::ffi::c_void, dwflags: u32, dwctrltype: u32, pvctrlpara: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CryptMsgControl(hcryptmsg : *const core::ffi::c_void, dwflags : u32, dwctrltype : u32, pvctrlpara : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { CryptMsgControl(hcryptmsg, dwflags, dwctrltype, pvctrlpara.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CryptMsgCountersign(hcryptmsg: *const core::ffi::c_void, dwindex: u32, rgcountersigners: &[CMSG_SIGNER_ENCODE_INFO]) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CryptMsgCountersign(hcryptmsg : *const core::ffi::c_void, dwindex : u32, ccountersigners : u32, rgcountersigners : *const CMSG_SIGNER_ENCODE_INFO) -> windows_core::BOOL);
    unsafe { CryptMsgCountersign(hcryptmsg, dwindex, rgcountersigners.len().try_into().unwrap(), core::mem::transmute(rgcountersigners.as_ptr())).ok() }
}
#[inline]
pub unsafe fn CryptMsgCountersignEncoded(dwencodingtype: u32, pbsignerinfo: &[u8], rgcountersigners: &[CMSG_SIGNER_ENCODE_INFO], pbcountersignature: Option<*mut u8>, pcbcountersignature: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CryptMsgCountersignEncoded(dwencodingtype : u32, pbsignerinfo : *const u8, cbsignerinfo : u32, ccountersigners : u32, rgcountersigners : *const CMSG_SIGNER_ENCODE_INFO, pbcountersignature : *mut u8, pcbcountersignature : *mut u32) -> windows_core::BOOL);
    unsafe { CryptMsgCountersignEncoded(dwencodingtype, core::mem::transmute(pbsignerinfo.as_ptr()), pbsignerinfo.len().try_into().unwrap(), rgcountersigners.len().try_into().unwrap(), core::mem::transmute(rgcountersigners.as_ptr()), pbcountersignature.unwrap_or(core::mem::zeroed()) as _, pcbcountersignature as _).ok() }
}
#[inline]
pub unsafe fn CryptMsgDuplicate(hcryptmsg: Option<*const core::ffi::c_void>) -> *mut core::ffi::c_void {
    windows_core::link!("crypt32.dll" "system" fn CryptMsgDuplicate(hcryptmsg : *const core::ffi::c_void) -> *mut core::ffi::c_void);
    unsafe { CryptMsgDuplicate(hcryptmsg.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CryptMsgEncodeAndSignCTL(dwmsgencodingtype: u32, pctlinfo: *const CTL_INFO, psigninfo: *const CMSG_SIGNED_ENCODE_INFO, dwflags: u32, pbencoded: Option<*mut u8>, pcbencoded: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CryptMsgEncodeAndSignCTL(dwmsgencodingtype : u32, pctlinfo : *const CTL_INFO, psigninfo : *const CMSG_SIGNED_ENCODE_INFO, dwflags : u32, pbencoded : *mut u8, pcbencoded : *mut u32) -> windows_core::BOOL);
    unsafe { CryptMsgEncodeAndSignCTL(dwmsgencodingtype, pctlinfo, psigninfo, dwflags, pbencoded.unwrap_or(core::mem::zeroed()) as _, pcbencoded as _).ok() }
}
#[inline]
pub unsafe fn CryptMsgGetAndVerifySigner(hcryptmsg: *const core::ffi::c_void, rghsignerstore: Option<&[HCERTSTORE]>, dwflags: u32, ppsigner: Option<*mut *mut CERT_CONTEXT>, pdwsignerindex: Option<*mut u32>) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CryptMsgGetAndVerifySigner(hcryptmsg : *const core::ffi::c_void, csignerstore : u32, rghsignerstore : *const HCERTSTORE, dwflags : u32, ppsigner : *mut *mut CERT_CONTEXT, pdwsignerindex : *mut u32) -> windows_core::BOOL);
    unsafe { CryptMsgGetAndVerifySigner(hcryptmsg, rghsignerstore.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(rghsignerstore.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), dwflags, ppsigner.unwrap_or(core::mem::zeroed()) as _, pdwsignerindex.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CryptMsgGetParam(hcryptmsg: *const core::ffi::c_void, dwparamtype: u32, dwindex: u32, pvdata: Option<*mut core::ffi::c_void>, pcbdata: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CryptMsgGetParam(hcryptmsg : *const core::ffi::c_void, dwparamtype : u32, dwindex : u32, pvdata : *mut core::ffi::c_void, pcbdata : *mut u32) -> windows_core::BOOL);
    unsafe { CryptMsgGetParam(hcryptmsg, dwparamtype, dwindex, pvdata.unwrap_or(core::mem::zeroed()) as _, pcbdata as _).ok() }
}
#[inline]
pub unsafe fn CryptMsgOpenToDecode(dwmsgencodingtype: u32, dwflags: u32, dwmsgtype: u32, hcryptprov: Option<HCRYPTPROV_LEGACY>, precipientinfo: Option<*const CERT_INFO>, pstreaminfo: Option<*const CMSG_STREAM_INFO>) -> *mut core::ffi::c_void {
    windows_core::link!("crypt32.dll" "system" fn CryptMsgOpenToDecode(dwmsgencodingtype : u32, dwflags : u32, dwmsgtype : u32, hcryptprov : HCRYPTPROV_LEGACY, precipientinfo : *const CERT_INFO, pstreaminfo : *const CMSG_STREAM_INFO) -> *mut core::ffi::c_void);
    unsafe { CryptMsgOpenToDecode(dwmsgencodingtype, dwflags, dwmsgtype, hcryptprov.unwrap_or(core::mem::zeroed()) as _, precipientinfo.unwrap_or(core::mem::zeroed()) as _, pstreaminfo.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CryptMsgOpenToEncode<P4>(dwmsgencodingtype: u32, dwflags: u32, dwmsgtype: CRYPT_MSG_TYPE, pvmsgencodeinfo: *const core::ffi::c_void, pszinnercontentobjid: P4, pstreaminfo: Option<*const CMSG_STREAM_INFO>) -> *mut core::ffi::c_void
where
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("crypt32.dll" "system" fn CryptMsgOpenToEncode(dwmsgencodingtype : u32, dwflags : u32, dwmsgtype : CRYPT_MSG_TYPE, pvmsgencodeinfo : *const core::ffi::c_void, pszinnercontentobjid : windows_core::PCSTR, pstreaminfo : *const CMSG_STREAM_INFO) -> *mut core::ffi::c_void);
    unsafe { CryptMsgOpenToEncode(dwmsgencodingtype, dwflags, dwmsgtype, pvmsgencodeinfo, pszinnercontentobjid.param().abi(), pstreaminfo.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CryptMsgSignCTL(dwmsgencodingtype: u32, pbctlcontent: &[u8], psigninfo: *const CMSG_SIGNED_ENCODE_INFO, dwflags: u32, pbencoded: Option<*mut u8>, pcbencoded: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CryptMsgSignCTL(dwmsgencodingtype : u32, pbctlcontent : *const u8, cbctlcontent : u32, psigninfo : *const CMSG_SIGNED_ENCODE_INFO, dwflags : u32, pbencoded : *mut u8, pcbencoded : *mut u32) -> windows_core::BOOL);
    unsafe { CryptMsgSignCTL(dwmsgencodingtype, core::mem::transmute(pbctlcontent.as_ptr()), pbctlcontent.len().try_into().unwrap(), psigninfo, dwflags, pbencoded.unwrap_or(core::mem::zeroed()) as _, pcbencoded as _).ok() }
}
#[inline]
pub unsafe fn CryptMsgUpdate(hcryptmsg: *const core::ffi::c_void, pbdata: Option<&[u8]>, ffinal: bool) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CryptMsgUpdate(hcryptmsg : *const core::ffi::c_void, pbdata : *const u8, cbdata : u32, ffinal : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { CryptMsgUpdate(hcryptmsg, core::mem::transmute(pbdata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pbdata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), ffinal.into()).ok() }
}
#[inline]
pub unsafe fn CryptMsgVerifyCountersignatureEncoded(hcryptprov: Option<HCRYPTPROV_LEGACY>, dwencodingtype: u32, pbsignerinfo: &[u8], pbsignerinfocountersignature: &[u8], pcicountersigner: *const CERT_INFO) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CryptMsgVerifyCountersignatureEncoded(hcryptprov : HCRYPTPROV_LEGACY, dwencodingtype : u32, pbsignerinfo : *const u8, cbsignerinfo : u32, pbsignerinfocountersignature : *const u8, cbsignerinfocountersignature : u32, pcicountersigner : *const CERT_INFO) -> windows_core::BOOL);
    unsafe { CryptMsgVerifyCountersignatureEncoded(hcryptprov.unwrap_or(core::mem::zeroed()) as _, dwencodingtype, core::mem::transmute(pbsignerinfo.as_ptr()), pbsignerinfo.len().try_into().unwrap(), core::mem::transmute(pbsignerinfocountersignature.as_ptr()), pbsignerinfocountersignature.len().try_into().unwrap(), pcicountersigner).ok() }
}
#[inline]
pub unsafe fn CryptMsgVerifyCountersignatureEncodedEx(hcryptprov: Option<HCRYPTPROV_LEGACY>, dwencodingtype: u32, pbsignerinfo: &[u8], pbsignerinfocountersignature: &[u8], dwsignertype: u32, pvsigner: *const core::ffi::c_void, dwflags: u32, pvextra: Option<*mut core::ffi::c_void>) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CryptMsgVerifyCountersignatureEncodedEx(hcryptprov : HCRYPTPROV_LEGACY, dwencodingtype : u32, pbsignerinfo : *const u8, cbsignerinfo : u32, pbsignerinfocountersignature : *const u8, cbsignerinfocountersignature : u32, dwsignertype : u32, pvsigner : *const core::ffi::c_void, dwflags : u32, pvextra : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { CryptMsgVerifyCountersignatureEncodedEx(hcryptprov.unwrap_or(core::mem::zeroed()) as _, dwencodingtype, core::mem::transmute(pbsignerinfo.as_ptr()), pbsignerinfo.len().try_into().unwrap(), core::mem::transmute(pbsignerinfocountersignature.as_ptr()), pbsignerinfocountersignature.len().try_into().unwrap(), dwsignertype, pvsigner, dwflags, pvextra.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CryptProtectData<P1>(pdatain: *const CRYPT_INTEGER_BLOB, szdatadescr: P1, poptionalentropy: Option<*const CRYPT_INTEGER_BLOB>, pvreserved: Option<*const core::ffi::c_void>, ppromptstruct: Option<*const CRYPTPROTECT_PROMPTSTRUCT>, dwflags: u32, pdataout: *mut CRYPT_INTEGER_BLOB) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("crypt32.dll" "system" fn CryptProtectData(pdatain : *const CRYPT_INTEGER_BLOB, szdatadescr : windows_core::PCWSTR, poptionalentropy : *const CRYPT_INTEGER_BLOB, pvreserved : *const core::ffi::c_void, ppromptstruct : *const CRYPTPROTECT_PROMPTSTRUCT, dwflags : u32, pdataout : *mut CRYPT_INTEGER_BLOB) -> windows_core::BOOL);
    unsafe { CryptProtectData(pdatain, szdatadescr.param().abi(), poptionalentropy.unwrap_or(core::mem::zeroed()) as _, pvreserved.unwrap_or(core::mem::zeroed()) as _, ppromptstruct.unwrap_or(core::mem::zeroed()) as _, dwflags, pdataout as _).ok() }
}
#[inline]
pub unsafe fn CryptProtectMemory(pdatain: *mut core::ffi::c_void, cbdatain: u32, dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CryptProtectMemory(pdatain : *mut core::ffi::c_void, cbdatain : u32, dwflags : u32) -> windows_core::BOOL);
    unsafe { CryptProtectMemory(pdatain as _, cbdatain, dwflags).ok() }
}
#[inline]
pub unsafe fn CryptQueryObject(dwobjecttype: CERT_QUERY_OBJECT_TYPE, pvobject: *const core::ffi::c_void, dwexpectedcontenttypeflags: CERT_QUERY_CONTENT_TYPE_FLAGS, dwexpectedformattypeflags: CERT_QUERY_FORMAT_TYPE_FLAGS, dwflags: u32, pdwmsgandcertencodingtype: Option<*mut CERT_QUERY_ENCODING_TYPE>, pdwcontenttype: Option<*mut CERT_QUERY_CONTENT_TYPE>, pdwformattype: Option<*mut CERT_QUERY_FORMAT_TYPE>, phcertstore: Option<*mut HCERTSTORE>, phmsg: Option<*mut *mut core::ffi::c_void>, ppvcontext: Option<*mut *mut core::ffi::c_void>) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CryptQueryObject(dwobjecttype : CERT_QUERY_OBJECT_TYPE, pvobject : *const core::ffi::c_void, dwexpectedcontenttypeflags : CERT_QUERY_CONTENT_TYPE_FLAGS, dwexpectedformattypeflags : CERT_QUERY_FORMAT_TYPE_FLAGS, dwflags : u32, pdwmsgandcertencodingtype : *mut CERT_QUERY_ENCODING_TYPE, pdwcontenttype : *mut CERT_QUERY_CONTENT_TYPE, pdwformattype : *mut CERT_QUERY_FORMAT_TYPE, phcertstore : *mut HCERTSTORE, phmsg : *mut *mut core::ffi::c_void, ppvcontext : *mut *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { CryptQueryObject(dwobjecttype, pvobject, dwexpectedcontenttypeflags, dwexpectedformattypeflags, dwflags, pdwmsgandcertencodingtype.unwrap_or(core::mem::zeroed()) as _, pdwcontenttype.unwrap_or(core::mem::zeroed()) as _, pdwformattype.unwrap_or(core::mem::zeroed()) as _, phcertstore.unwrap_or(core::mem::zeroed()) as _, phmsg.unwrap_or(core::mem::zeroed()) as _, ppvcontext.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CryptRegisterDefaultOIDFunction<P1, P3>(dwencodingtype: u32, pszfuncname: P1, dwindex: u32, pwszdll: P3) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("crypt32.dll" "system" fn CryptRegisterDefaultOIDFunction(dwencodingtype : u32, pszfuncname : windows_core::PCSTR, dwindex : u32, pwszdll : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { CryptRegisterDefaultOIDFunction(dwencodingtype, pszfuncname.param().abi(), dwindex, pwszdll.param().abi()) }
}
#[inline]
pub unsafe fn CryptRegisterOIDFunction<P1, P2, P3, P4>(dwencodingtype: u32, pszfuncname: P1, pszoid: P2, pwszdll: P3, pszoverridefuncname: P4) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("crypt32.dll" "system" fn CryptRegisterOIDFunction(dwencodingtype : u32, pszfuncname : windows_core::PCSTR, pszoid : windows_core::PCSTR, pwszdll : windows_core::PCWSTR, pszoverridefuncname : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { CryptRegisterOIDFunction(dwencodingtype, pszfuncname.param().abi(), pszoid.param().abi(), pwszdll.param().abi(), pszoverridefuncname.param().abi()) }
}
#[inline]
pub unsafe fn CryptRegisterOIDInfo(pinfo: *const CRYPT_OID_INFO, dwflags: u32) -> windows_core::BOOL {
    windows_core::link!("crypt32.dll" "system" fn CryptRegisterOIDInfo(pinfo : *const CRYPT_OID_INFO, dwflags : u32) -> windows_core::BOOL);
    unsafe { CryptRegisterOIDInfo(pinfo, dwflags) }
}
#[inline]
pub unsafe fn CryptReleaseContext(hprov: usize, dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("advapi32.dll" "system" fn CryptReleaseContext(hprov : usize, dwflags : u32) -> windows_core::BOOL);
    unsafe { CryptReleaseContext(hprov, dwflags).ok() }
}
#[inline]
pub unsafe fn CryptRetrieveObjectByUrlA<P0, P1>(pszurl: P0, pszobjectoid: P1, dwretrievalflags: u32, dwtimeout: u32, ppvobject: *mut *mut core::ffi::c_void, hasyncretrieve: Option<HCRYPTASYNC>, pcredentials: Option<*const CRYPT_CREDENTIALS>, pvverify: Option<*const core::ffi::c_void>, pauxinfo: Option<*mut CRYPT_RETRIEVE_AUX_INFO>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("cryptnet.dll" "system" fn CryptRetrieveObjectByUrlA(pszurl : windows_core::PCSTR, pszobjectoid : windows_core::PCSTR, dwretrievalflags : u32, dwtimeout : u32, ppvobject : *mut *mut core::ffi::c_void, hasyncretrieve : HCRYPTASYNC, pcredentials : *const CRYPT_CREDENTIALS, pvverify : *const core::ffi::c_void, pauxinfo : *mut CRYPT_RETRIEVE_AUX_INFO) -> windows_core::BOOL);
    unsafe { CryptRetrieveObjectByUrlA(pszurl.param().abi(), pszobjectoid.param().abi(), dwretrievalflags, dwtimeout, ppvobject as _, hasyncretrieve.unwrap_or(core::mem::zeroed()) as _, pcredentials.unwrap_or(core::mem::zeroed()) as _, pvverify.unwrap_or(core::mem::zeroed()) as _, pauxinfo.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CryptRetrieveObjectByUrlW<P0, P1>(pszurl: P0, pszobjectoid: P1, dwretrievalflags: u32, dwtimeout: u32, ppvobject: *mut *mut core::ffi::c_void, hasyncretrieve: Option<HCRYPTASYNC>, pcredentials: Option<*const CRYPT_CREDENTIALS>, pvverify: Option<*const core::ffi::c_void>, pauxinfo: Option<*mut CRYPT_RETRIEVE_AUX_INFO>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("cryptnet.dll" "system" fn CryptRetrieveObjectByUrlW(pszurl : windows_core::PCWSTR, pszobjectoid : windows_core::PCSTR, dwretrievalflags : u32, dwtimeout : u32, ppvobject : *mut *mut core::ffi::c_void, hasyncretrieve : HCRYPTASYNC, pcredentials : *const CRYPT_CREDENTIALS, pvverify : *const core::ffi::c_void, pauxinfo : *mut CRYPT_RETRIEVE_AUX_INFO) -> windows_core::BOOL);
    unsafe { CryptRetrieveObjectByUrlW(pszurl.param().abi(), pszobjectoid.param().abi(), dwretrievalflags, dwtimeout, ppvobject as _, hasyncretrieve.unwrap_or(core::mem::zeroed()) as _, pcredentials.unwrap_or(core::mem::zeroed()) as _, pvverify.unwrap_or(core::mem::zeroed()) as _, pauxinfo.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CryptRetrieveTimeStamp<P0, P3>(wszurl: P0, dwretrievalflags: u32, dwtimeout: u32, pszhashid: P3, ppara: Option<*const CRYPT_TIMESTAMP_PARA>, pbdata: &[u8], pptscontext: *mut *mut CRYPT_TIMESTAMP_CONTEXT, pptssigner: *mut *mut CERT_CONTEXT, phstore: Option<*mut HCERTSTORE>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("crypt32.dll" "system" fn CryptRetrieveTimeStamp(wszurl : windows_core::PCWSTR, dwretrievalflags : u32, dwtimeout : u32, pszhashid : windows_core::PCSTR, ppara : *const CRYPT_TIMESTAMP_PARA, pbdata : *const u8, cbdata : u32, pptscontext : *mut *mut CRYPT_TIMESTAMP_CONTEXT, pptssigner : *mut *mut CERT_CONTEXT, phstore : *mut HCERTSTORE) -> windows_core::BOOL);
    unsafe { CryptRetrieveTimeStamp(wszurl.param().abi(), dwretrievalflags, dwtimeout, pszhashid.param().abi(), ppara.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pbdata.as_ptr()), pbdata.len().try_into().unwrap(), pptscontext as _, pptssigner as _, phstore.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CryptSetAsyncParam<P1>(hasync: HCRYPTASYNC, pszparamoid: P1, pvparam: Option<*const core::ffi::c_void>, pfnfree: PFN_CRYPT_ASYNC_PARAM_FREE_FUNC) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("crypt32.dll" "system" fn CryptSetAsyncParam(hasync : HCRYPTASYNC, pszparamoid : windows_core::PCSTR, pvparam : *const core::ffi::c_void, pfnfree : PFN_CRYPT_ASYNC_PARAM_FREE_FUNC) -> windows_core::BOOL);
    unsafe { CryptSetAsyncParam(hasync, pszparamoid.param().abi(), pvparam.unwrap_or(core::mem::zeroed()) as _, pfnfree) }
}
#[inline]
pub unsafe fn CryptSetHashParam(hhash: usize, dwparam: CRYPT_SET_HASH_PARAM, pbdata: *const u8, dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("advapi32.dll" "system" fn CryptSetHashParam(hhash : usize, dwparam : CRYPT_SET_HASH_PARAM, pbdata : *const u8, dwflags : u32) -> windows_core::BOOL);
    unsafe { CryptSetHashParam(hhash, dwparam, pbdata, dwflags).ok() }
}
#[inline]
pub unsafe fn CryptSetKeyIdentifierProperty<P3>(pkeyidentifier: *const CRYPT_INTEGER_BLOB, dwpropid: u32, dwflags: u32, pwszcomputername: P3, pvreserved: Option<*const core::ffi::c_void>, pvdata: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("crypt32.dll" "system" fn CryptSetKeyIdentifierProperty(pkeyidentifier : *const CRYPT_INTEGER_BLOB, dwpropid : u32, dwflags : u32, pwszcomputername : windows_core::PCWSTR, pvreserved : *const core::ffi::c_void, pvdata : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { CryptSetKeyIdentifierProperty(pkeyidentifier, dwpropid, dwflags, pwszcomputername.param().abi(), pvreserved.unwrap_or(core::mem::zeroed()) as _, pvdata.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CryptSetKeyParam(hkey: usize, dwparam: CRYPT_KEY_PARAM_ID, pbdata: *const u8, dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("advapi32.dll" "system" fn CryptSetKeyParam(hkey : usize, dwparam : CRYPT_KEY_PARAM_ID, pbdata : *const u8, dwflags : u32) -> windows_core::BOOL);
    unsafe { CryptSetKeyParam(hkey, dwparam, pbdata, dwflags).ok() }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn CryptSetOIDFunctionValue<P1, P2, P3>(dwencodingtype: u32, pszfuncname: P1, pszoid: P2, pwszvaluename: P3, dwvaluetype: super::super::System::Registry::REG_VALUE_TYPE, pbvaluedata: Option<&[u8]>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("crypt32.dll" "system" fn CryptSetOIDFunctionValue(dwencodingtype : u32, pszfuncname : windows_core::PCSTR, pszoid : windows_core::PCSTR, pwszvaluename : windows_core::PCWSTR, dwvaluetype : super::super::System::Registry:: REG_VALUE_TYPE, pbvaluedata : *const u8, cbvaluedata : u32) -> windows_core::BOOL);
    unsafe { CryptSetOIDFunctionValue(dwencodingtype, pszfuncname.param().abi(), pszoid.param().abi(), pwszvaluename.param().abi(), dwvaluetype, core::mem::transmute(pbvaluedata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pbvaluedata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn CryptSetProvParam(hprov: usize, dwparam: CRYPT_SET_PROV_PARAM_ID, pbdata: *const u8, dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("advapi32.dll" "system" fn CryptSetProvParam(hprov : usize, dwparam : CRYPT_SET_PROV_PARAM_ID, pbdata : *const u8, dwflags : u32) -> windows_core::BOOL);
    unsafe { CryptSetProvParam(hprov, dwparam, pbdata, dwflags).ok() }
}
#[inline]
pub unsafe fn CryptSetProviderA<P0>(pszprovname: P0, dwprovtype: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn CryptSetProviderA(pszprovname : windows_core::PCSTR, dwprovtype : u32) -> windows_core::BOOL);
    unsafe { CryptSetProviderA(pszprovname.param().abi(), dwprovtype).ok() }
}
#[inline]
pub unsafe fn CryptSetProviderExA<P0>(pszprovname: P0, dwprovtype: u32, pdwreserved: Option<*const u32>, dwflags: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn CryptSetProviderExA(pszprovname : windows_core::PCSTR, dwprovtype : u32, pdwreserved : *const u32, dwflags : u32) -> windows_core::BOOL);
    unsafe { CryptSetProviderExA(pszprovname.param().abi(), dwprovtype, pdwreserved.unwrap_or(core::mem::zeroed()) as _, dwflags).ok() }
}
#[inline]
pub unsafe fn CryptSetProviderExW<P0>(pszprovname: P0, dwprovtype: u32, pdwreserved: Option<*const u32>, dwflags: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn CryptSetProviderExW(pszprovname : windows_core::PCWSTR, dwprovtype : u32, pdwreserved : *const u32, dwflags : u32) -> windows_core::BOOL);
    unsafe { CryptSetProviderExW(pszprovname.param().abi(), dwprovtype, pdwreserved.unwrap_or(core::mem::zeroed()) as _, dwflags).ok() }
}
#[inline]
pub unsafe fn CryptSetProviderW<P0>(pszprovname: P0, dwprovtype: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn CryptSetProviderW(pszprovname : windows_core::PCWSTR, dwprovtype : u32) -> windows_core::BOOL);
    unsafe { CryptSetProviderW(pszprovname.param().abi(), dwprovtype).ok() }
}
#[inline]
pub unsafe fn CryptSignAndEncodeCertificate<P3>(hcryptprovorncryptkey: Option<HCRYPTPROV_OR_NCRYPT_KEY_HANDLE>, dwkeyspec: Option<CERT_KEY_SPEC>, dwcertencodingtype: CERT_QUERY_ENCODING_TYPE, lpszstructtype: P3, pvstructinfo: *const core::ffi::c_void, psignaturealgorithm: *const CRYPT_ALGORITHM_IDENTIFIER, pvhashauxinfo: Option<*const core::ffi::c_void>, pbencoded: Option<*mut u8>, pcbencoded: *mut u32) -> windows_core::Result<()>
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("crypt32.dll" "system" fn CryptSignAndEncodeCertificate(hcryptprovorncryptkey : HCRYPTPROV_OR_NCRYPT_KEY_HANDLE, dwkeyspec : CERT_KEY_SPEC, dwcertencodingtype : CERT_QUERY_ENCODING_TYPE, lpszstructtype : windows_core::PCSTR, pvstructinfo : *const core::ffi::c_void, psignaturealgorithm : *const CRYPT_ALGORITHM_IDENTIFIER, pvhashauxinfo : *const core::ffi::c_void, pbencoded : *mut u8, pcbencoded : *mut u32) -> windows_core::BOOL);
    unsafe { CryptSignAndEncodeCertificate(hcryptprovorncryptkey.unwrap_or(core::mem::zeroed()) as _, dwkeyspec.unwrap_or(core::mem::zeroed()) as _, dwcertencodingtype, lpszstructtype.param().abi(), pvstructinfo, psignaturealgorithm, pvhashauxinfo.unwrap_or(core::mem::zeroed()) as _, pbencoded.unwrap_or(core::mem::zeroed()) as _, pcbencoded as _).ok() }
}
#[inline]
pub unsafe fn CryptSignAndEncryptMessage(psignpara: *const CRYPT_SIGN_MESSAGE_PARA, pencryptpara: *const CRYPT_ENCRYPT_MESSAGE_PARA, rgprecipientcert: &[*const CERT_CONTEXT], pbtobesignedandencrypted: &[u8], pbsignedandencryptedblob: Option<*mut u8>, pcbsignedandencryptedblob: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CryptSignAndEncryptMessage(psignpara : *const CRYPT_SIGN_MESSAGE_PARA, pencryptpara : *const CRYPT_ENCRYPT_MESSAGE_PARA, crecipientcert : u32, rgprecipientcert : *const *const CERT_CONTEXT, pbtobesignedandencrypted : *const u8, cbtobesignedandencrypted : u32, pbsignedandencryptedblob : *mut u8, pcbsignedandencryptedblob : *mut u32) -> windows_core::BOOL);
    unsafe { CryptSignAndEncryptMessage(psignpara, pencryptpara, rgprecipientcert.len().try_into().unwrap(), core::mem::transmute(rgprecipientcert.as_ptr()), core::mem::transmute(pbtobesignedandencrypted.as_ptr()), pbtobesignedandencrypted.len().try_into().unwrap(), pbsignedandencryptedblob.unwrap_or(core::mem::zeroed()) as _, pcbsignedandencryptedblob as _).ok() }
}
#[inline]
pub unsafe fn CryptSignCertificate(hcryptprovorncryptkey: Option<HCRYPTPROV_OR_NCRYPT_KEY_HANDLE>, dwkeyspec: Option<u32>, dwcertencodingtype: CERT_QUERY_ENCODING_TYPE, pbencodedtobesigned: &[u8], psignaturealgorithm: *const CRYPT_ALGORITHM_IDENTIFIER, pvhashauxinfo: Option<*const core::ffi::c_void>, pbsignature: Option<*mut u8>, pcbsignature: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CryptSignCertificate(hcryptprovorncryptkey : HCRYPTPROV_OR_NCRYPT_KEY_HANDLE, dwkeyspec : u32, dwcertencodingtype : CERT_QUERY_ENCODING_TYPE, pbencodedtobesigned : *const u8, cbencodedtobesigned : u32, psignaturealgorithm : *const CRYPT_ALGORITHM_IDENTIFIER, pvhashauxinfo : *const core::ffi::c_void, pbsignature : *mut u8, pcbsignature : *mut u32) -> windows_core::BOOL);
    unsafe { CryptSignCertificate(hcryptprovorncryptkey.unwrap_or(core::mem::zeroed()) as _, dwkeyspec.unwrap_or(core::mem::zeroed()) as _, dwcertencodingtype, core::mem::transmute(pbencodedtobesigned.as_ptr()), pbencodedtobesigned.len().try_into().unwrap(), psignaturealgorithm, pvhashauxinfo.unwrap_or(core::mem::zeroed()) as _, pbsignature.unwrap_or(core::mem::zeroed()) as _, pcbsignature as _).ok() }
}
#[inline]
pub unsafe fn CryptSignHashA<P2>(hhash: usize, dwkeyspec: u32, szdescription: P2, dwflags: u32, pbsignature: Option<*mut u8>, pdwsiglen: *mut u32) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn CryptSignHashA(hhash : usize, dwkeyspec : u32, szdescription : windows_core::PCSTR, dwflags : u32, pbsignature : *mut u8, pdwsiglen : *mut u32) -> windows_core::BOOL);
    unsafe { CryptSignHashA(hhash, dwkeyspec, szdescription.param().abi(), dwflags, pbsignature.unwrap_or(core::mem::zeroed()) as _, pdwsiglen as _).ok() }
}
#[inline]
pub unsafe fn CryptSignHashW<P2>(hhash: usize, dwkeyspec: u32, szdescription: P2, dwflags: u32, pbsignature: Option<*mut u8>, pdwsiglen: *mut u32) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn CryptSignHashW(hhash : usize, dwkeyspec : u32, szdescription : windows_core::PCWSTR, dwflags : u32, pbsignature : *mut u8, pdwsiglen : *mut u32) -> windows_core::BOOL);
    unsafe { CryptSignHashW(hhash, dwkeyspec, szdescription.param().abi(), dwflags, pbsignature.unwrap_or(core::mem::zeroed()) as _, pdwsiglen as _).ok() }
}
#[inline]
pub unsafe fn CryptSignMessage(psignpara: *const CRYPT_SIGN_MESSAGE_PARA, fdetachedsignature: bool, ctobesigned: u32, rgpbtobesigned: Option<*const *const u8>, rgcbtobesigned: *const u32, pbsignedblob: Option<*mut u8>, pcbsignedblob: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CryptSignMessage(psignpara : *const CRYPT_SIGN_MESSAGE_PARA, fdetachedsignature : windows_core::BOOL, ctobesigned : u32, rgpbtobesigned : *const *const u8, rgcbtobesigned : *const u32, pbsignedblob : *mut u8, pcbsignedblob : *mut u32) -> windows_core::BOOL);
    unsafe { CryptSignMessage(psignpara, fdetachedsignature.into(), ctobesigned, rgpbtobesigned.unwrap_or(core::mem::zeroed()) as _, rgcbtobesigned, pbsignedblob.unwrap_or(core::mem::zeroed()) as _, pcbsignedblob as _).ok() }
}
#[inline]
pub unsafe fn CryptSignMessageWithKey(psignpara: *const CRYPT_KEY_SIGN_MESSAGE_PARA, pbtobesigned: &[u8], pbsignedblob: Option<*mut u8>, pcbsignedblob: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CryptSignMessageWithKey(psignpara : *const CRYPT_KEY_SIGN_MESSAGE_PARA, pbtobesigned : *const u8, cbtobesigned : u32, pbsignedblob : *mut u8, pcbsignedblob : *mut u32) -> windows_core::BOOL);
    unsafe { CryptSignMessageWithKey(psignpara, core::mem::transmute(pbtobesigned.as_ptr()), pbtobesigned.len().try_into().unwrap(), pbsignedblob.unwrap_or(core::mem::zeroed()) as _, pcbsignedblob as _).ok() }
}
#[inline]
pub unsafe fn CryptStringToBinaryA(pszstring: &[u8], dwflags: CRYPT_STRING, pbbinary: Option<*mut u8>, pcbbinary: *mut u32, pdwskip: Option<*mut u32>, pdwflags: Option<*mut u32>) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CryptStringToBinaryA(pszstring : windows_core::PCSTR, cchstring : u32, dwflags : CRYPT_STRING, pbbinary : *mut u8, pcbbinary : *mut u32, pdwskip : *mut u32, pdwflags : *mut u32) -> windows_core::BOOL);
    unsafe { CryptStringToBinaryA(core::mem::transmute(pszstring.as_ptr()), pszstring.len().try_into().unwrap(), dwflags, pbbinary.unwrap_or(core::mem::zeroed()) as _, pcbbinary as _, pdwskip.unwrap_or(core::mem::zeroed()) as _, pdwflags.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CryptStringToBinaryW(pszstring: &[u16], dwflags: CRYPT_STRING, pbbinary: Option<*mut u8>, pcbbinary: *mut u32, pdwskip: Option<*mut u32>, pdwflags: Option<*mut u32>) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CryptStringToBinaryW(pszstring : windows_core::PCWSTR, cchstring : u32, dwflags : CRYPT_STRING, pbbinary : *mut u8, pcbbinary : *mut u32, pdwskip : *mut u32, pdwflags : *mut u32) -> windows_core::BOOL);
    unsafe { CryptStringToBinaryW(core::mem::transmute(pszstring.as_ptr()), pszstring.len().try_into().unwrap(), dwflags, pbbinary.unwrap_or(core::mem::zeroed()) as _, pcbbinary as _, pdwskip.unwrap_or(core::mem::zeroed()) as _, pdwflags.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CryptUninstallCancelRetrieval(dwflags: u32, pvreserved: Option<*const core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("cryptnet.dll" "system" fn CryptUninstallCancelRetrieval(dwflags : u32, pvreserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { CryptUninstallCancelRetrieval(dwflags, pvreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CryptUninstallDefaultContext(hdefaultcontext: Option<*const core::ffi::c_void>, dwflags: u32, pvreserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CryptUninstallDefaultContext(hdefaultcontext : *const core::ffi::c_void, dwflags : u32, pvreserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { CryptUninstallDefaultContext(hdefaultcontext.unwrap_or(core::mem::zeroed()) as _, dwflags, pvreserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CryptUnprotectData(pdatain: *const CRYPT_INTEGER_BLOB, ppszdatadescr: Option<*mut windows_core::PWSTR>, poptionalentropy: Option<*const CRYPT_INTEGER_BLOB>, pvreserved: Option<*const core::ffi::c_void>, ppromptstruct: Option<*const CRYPTPROTECT_PROMPTSTRUCT>, dwflags: u32, pdataout: *mut CRYPT_INTEGER_BLOB) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CryptUnprotectData(pdatain : *const CRYPT_INTEGER_BLOB, ppszdatadescr : *mut windows_core::PWSTR, poptionalentropy : *const CRYPT_INTEGER_BLOB, pvreserved : *const core::ffi::c_void, ppromptstruct : *const CRYPTPROTECT_PROMPTSTRUCT, dwflags : u32, pdataout : *mut CRYPT_INTEGER_BLOB) -> windows_core::BOOL);
    unsafe { CryptUnprotectData(pdatain, ppszdatadescr.unwrap_or(core::mem::zeroed()) as _, poptionalentropy.unwrap_or(core::mem::zeroed()) as _, pvreserved.unwrap_or(core::mem::zeroed()) as _, ppromptstruct.unwrap_or(core::mem::zeroed()) as _, dwflags, pdataout as _).ok() }
}
#[inline]
pub unsafe fn CryptUnprotectMemory(pdatain: *mut core::ffi::c_void, cbdatain: u32, dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CryptUnprotectMemory(pdatain : *mut core::ffi::c_void, cbdatain : u32, dwflags : u32) -> windows_core::BOOL);
    unsafe { CryptUnprotectMemory(pdatain as _, cbdatain, dwflags).ok() }
}
#[inline]
pub unsafe fn CryptUnregisterDefaultOIDFunction<P1, P2>(dwencodingtype: u32, pszfuncname: P1, pwszdll: P2) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("crypt32.dll" "system" fn CryptUnregisterDefaultOIDFunction(dwencodingtype : u32, pszfuncname : windows_core::PCSTR, pwszdll : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { CryptUnregisterDefaultOIDFunction(dwencodingtype, pszfuncname.param().abi(), pwszdll.param().abi()) }
}
#[inline]
pub unsafe fn CryptUnregisterOIDFunction<P1, P2>(dwencodingtype: u32, pszfuncname: P1, pszoid: P2) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("crypt32.dll" "system" fn CryptUnregisterOIDFunction(dwencodingtype : u32, pszfuncname : windows_core::PCSTR, pszoid : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { CryptUnregisterOIDFunction(dwencodingtype, pszfuncname.param().abi(), pszoid.param().abi()) }
}
#[inline]
pub unsafe fn CryptUnregisterOIDInfo(pinfo: *const CRYPT_OID_INFO) -> windows_core::BOOL {
    windows_core::link!("crypt32.dll" "system" fn CryptUnregisterOIDInfo(pinfo : *const CRYPT_OID_INFO) -> windows_core::BOOL);
    unsafe { CryptUnregisterOIDInfo(pinfo) }
}
#[inline]
pub unsafe fn CryptUpdateProtectedState<P1>(poldsid: Option<super::PSID>, pwszoldpassword: P1, dwflags: u32, pdwsuccesscount: Option<*mut u32>, pdwfailurecount: Option<*mut u32>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("crypt32.dll" "system" fn CryptUpdateProtectedState(poldsid : super:: PSID, pwszoldpassword : windows_core::PCWSTR, dwflags : u32, pdwsuccesscount : *mut u32, pdwfailurecount : *mut u32) -> windows_core::BOOL);
    unsafe { CryptUpdateProtectedState(poldsid.unwrap_or(core::mem::zeroed()) as _, pwszoldpassword.param().abi(), dwflags, pdwsuccesscount.unwrap_or(core::mem::zeroed()) as _, pdwfailurecount.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CryptVerifyCertificateSignature(hcryptprov: Option<HCRYPTPROV_LEGACY>, dwcertencodingtype: CERT_QUERY_ENCODING_TYPE, pbencoded: &[u8], ppublickey: *const CERT_PUBLIC_KEY_INFO) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CryptVerifyCertificateSignature(hcryptprov : HCRYPTPROV_LEGACY, dwcertencodingtype : CERT_QUERY_ENCODING_TYPE, pbencoded : *const u8, cbencoded : u32, ppublickey : *const CERT_PUBLIC_KEY_INFO) -> windows_core::BOOL);
    unsafe { CryptVerifyCertificateSignature(hcryptprov.unwrap_or(core::mem::zeroed()) as _, dwcertencodingtype, core::mem::transmute(pbencoded.as_ptr()), pbencoded.len().try_into().unwrap(), ppublickey).ok() }
}
#[inline]
pub unsafe fn CryptVerifyCertificateSignatureEx(hcryptprov: Option<HCRYPTPROV_LEGACY>, dwcertencodingtype: CERT_QUERY_ENCODING_TYPE, dwsubjecttype: u32, pvsubject: *const core::ffi::c_void, dwissuertype: u32, pvissuer: Option<*const core::ffi::c_void>, dwflags: CRYPT_VERIFY_CERT_FLAGS, pvextra: Option<*mut core::ffi::c_void>) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CryptVerifyCertificateSignatureEx(hcryptprov : HCRYPTPROV_LEGACY, dwcertencodingtype : CERT_QUERY_ENCODING_TYPE, dwsubjecttype : u32, pvsubject : *const core::ffi::c_void, dwissuertype : u32, pvissuer : *const core::ffi::c_void, dwflags : CRYPT_VERIFY_CERT_FLAGS, pvextra : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { CryptVerifyCertificateSignatureEx(hcryptprov.unwrap_or(core::mem::zeroed()) as _, dwcertencodingtype, dwsubjecttype, pvsubject, dwissuertype, pvissuer.unwrap_or(core::mem::zeroed()) as _, dwflags, pvextra.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CryptVerifyDetachedMessageHash(phashpara: *const CRYPT_HASH_MESSAGE_PARA, pbdetachedhashblob: &[u8], ctobehashed: u32, rgpbtobehashed: *const *const u8, rgcbtobehashed: *const u32, pbcomputedhash: Option<*mut u8>, pcbcomputedhash: Option<*mut u32>) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CryptVerifyDetachedMessageHash(phashpara : *const CRYPT_HASH_MESSAGE_PARA, pbdetachedhashblob : *const u8, cbdetachedhashblob : u32, ctobehashed : u32, rgpbtobehashed : *const *const u8, rgcbtobehashed : *const u32, pbcomputedhash : *mut u8, pcbcomputedhash : *mut u32) -> windows_core::BOOL);
    unsafe { CryptVerifyDetachedMessageHash(phashpara, core::mem::transmute(pbdetachedhashblob.as_ptr()), pbdetachedhashblob.len().try_into().unwrap(), ctobehashed, rgpbtobehashed, rgcbtobehashed, pbcomputedhash.unwrap_or(core::mem::zeroed()) as _, pcbcomputedhash.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CryptVerifyDetachedMessageSignature(pverifypara: *const CRYPT_VERIFY_MESSAGE_PARA, dwsignerindex: u32, pbdetachedsignblob: &[u8], ctobesigned: u32, rgpbtobesigned: *const *const u8, rgcbtobesigned: *const u32, ppsignercert: Option<*mut *mut CERT_CONTEXT>) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CryptVerifyDetachedMessageSignature(pverifypara : *const CRYPT_VERIFY_MESSAGE_PARA, dwsignerindex : u32, pbdetachedsignblob : *const u8, cbdetachedsignblob : u32, ctobesigned : u32, rgpbtobesigned : *const *const u8, rgcbtobesigned : *const u32, ppsignercert : *mut *mut CERT_CONTEXT) -> windows_core::BOOL);
    unsafe { CryptVerifyDetachedMessageSignature(pverifypara, dwsignerindex, core::mem::transmute(pbdetachedsignblob.as_ptr()), pbdetachedsignblob.len().try_into().unwrap(), ctobesigned, rgpbtobesigned, rgcbtobesigned, ppsignercert.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CryptVerifyMessageHash(phashpara: *const CRYPT_HASH_MESSAGE_PARA, pbhashedblob: &[u8], pbtobehashed: Option<*mut u8>, pcbtobehashed: Option<*mut u32>, pbcomputedhash: Option<*mut u8>, pcbcomputedhash: Option<*mut u32>) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CryptVerifyMessageHash(phashpara : *const CRYPT_HASH_MESSAGE_PARA, pbhashedblob : *const u8, cbhashedblob : u32, pbtobehashed : *mut u8, pcbtobehashed : *mut u32, pbcomputedhash : *mut u8, pcbcomputedhash : *mut u32) -> windows_core::BOOL);
    unsafe { CryptVerifyMessageHash(phashpara, core::mem::transmute(pbhashedblob.as_ptr()), pbhashedblob.len().try_into().unwrap(), pbtobehashed.unwrap_or(core::mem::zeroed()) as _, pcbtobehashed.unwrap_or(core::mem::zeroed()) as _, pbcomputedhash.unwrap_or(core::mem::zeroed()) as _, pcbcomputedhash.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CryptVerifyMessageSignature(pverifypara: *const CRYPT_VERIFY_MESSAGE_PARA, dwsignerindex: u32, pbsignedblob: &[u8], pbdecoded: Option<*mut u8>, pcbdecoded: Option<*mut u32>, ppsignercert: Option<*mut *mut CERT_CONTEXT>) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CryptVerifyMessageSignature(pverifypara : *const CRYPT_VERIFY_MESSAGE_PARA, dwsignerindex : u32, pbsignedblob : *const u8, cbsignedblob : u32, pbdecoded : *mut u8, pcbdecoded : *mut u32, ppsignercert : *mut *mut CERT_CONTEXT) -> windows_core::BOOL);
    unsafe { CryptVerifyMessageSignature(pverifypara, dwsignerindex, core::mem::transmute(pbsignedblob.as_ptr()), pbsignedblob.len().try_into().unwrap(), pbdecoded.unwrap_or(core::mem::zeroed()) as _, pcbdecoded.unwrap_or(core::mem::zeroed()) as _, ppsignercert.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CryptVerifyMessageSignatureWithKey(pverifypara: *const CRYPT_KEY_VERIFY_MESSAGE_PARA, ppublickeyinfo: Option<*const CERT_PUBLIC_KEY_INFO>, pbsignedblob: &[u8], pbdecoded: Option<*mut u8>, pcbdecoded: Option<*mut u32>) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CryptVerifyMessageSignatureWithKey(pverifypara : *const CRYPT_KEY_VERIFY_MESSAGE_PARA, ppublickeyinfo : *const CERT_PUBLIC_KEY_INFO, pbsignedblob : *const u8, cbsignedblob : u32, pbdecoded : *mut u8, pcbdecoded : *mut u32) -> windows_core::BOOL);
    unsafe { CryptVerifyMessageSignatureWithKey(pverifypara, ppublickeyinfo.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pbsignedblob.as_ptr()), pbsignedblob.len().try_into().unwrap(), pbdecoded.unwrap_or(core::mem::zeroed()) as _, pcbdecoded.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CryptVerifySignatureA<P4>(hhash: usize, pbsignature: &[u8], hpubkey: usize, szdescription: P4, dwflags: u32) -> windows_core::Result<()>
where
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn CryptVerifySignatureA(hhash : usize, pbsignature : *const u8, dwsiglen : u32, hpubkey : usize, szdescription : windows_core::PCSTR, dwflags : u32) -> windows_core::BOOL);
    unsafe { CryptVerifySignatureA(hhash, core::mem::transmute(pbsignature.as_ptr()), pbsignature.len().try_into().unwrap(), hpubkey, szdescription.param().abi(), dwflags).ok() }
}
#[inline]
pub unsafe fn CryptVerifySignatureW<P4>(hhash: usize, pbsignature: &[u8], hpubkey: usize, szdescription: P4, dwflags: u32) -> windows_core::Result<()>
where
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn CryptVerifySignatureW(hhash : usize, pbsignature : *const u8, dwsiglen : u32, hpubkey : usize, szdescription : windows_core::PCWSTR, dwflags : u32) -> windows_core::BOOL);
    unsafe { CryptVerifySignatureW(hhash, core::mem::transmute(pbsignature.as_ptr()), pbsignature.len().try_into().unwrap(), hpubkey, szdescription.param().abi(), dwflags).ok() }
}
#[inline]
pub unsafe fn CryptVerifyTimeStampSignature(pbtscontentinfo: &[u8], pbdata: Option<&[u8]>, hadditionalstore: Option<HCERTSTORE>, pptscontext: *mut *mut CRYPT_TIMESTAMP_CONTEXT, pptssigner: *mut *mut CERT_CONTEXT, phstore: Option<*mut HCERTSTORE>) -> windows_core::Result<()> {
    windows_core::link!("crypt32.dll" "system" fn CryptVerifyTimeStampSignature(pbtscontentinfo : *const u8, cbtscontentinfo : u32, pbdata : *const u8, cbdata : u32, hadditionalstore : HCERTSTORE, pptscontext : *mut *mut CRYPT_TIMESTAMP_CONTEXT, pptssigner : *mut *mut CERT_CONTEXT, phstore : *mut HCERTSTORE) -> windows_core::BOOL);
    unsafe { CryptVerifyTimeStampSignature(core::mem::transmute(pbtscontentinfo.as_ptr()), pbtscontentinfo.len().try_into().unwrap(), core::mem::transmute(pbdata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pbdata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), hadditionalstore.unwrap_or(core::mem::zeroed()) as _, pptscontext as _, pptssigner as _, phstore.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CryptXmlAddObject(hsignatureorobject: *const core::ffi::c_void, dwflags: u32, rgproperty: Option<&[CRYPT_XML_PROPERTY]>, pencoded: *const CRYPT_XML_BLOB) -> windows_core::Result<*mut CRYPT_XML_OBJECT> {
    windows_core::link!("cryptxml.dll" "system" fn CryptXmlAddObject(hsignatureorobject : *const core::ffi::c_void, dwflags : u32, rgproperty : *const CRYPT_XML_PROPERTY, cproperty : u32, pencoded : *const CRYPT_XML_BLOB, ppobject : *mut *mut CRYPT_XML_OBJECT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CryptXmlAddObject(hsignatureorobject, dwflags, core::mem::transmute(rgproperty.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), rgproperty.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pencoded, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn CryptXmlClose(hcryptxml: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_core::link!("cryptxml.dll" "system" fn CryptXmlClose(hcryptxml : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CryptXmlClose(hcryptxml).ok() }
}
#[inline]
pub unsafe fn CryptXmlCreateReference<P2, P3, P4>(hcryptxml: *const core::ffi::c_void, dwflags: u32, wszid: P2, wszuri: P3, wsztype: P4, pdigestmethod: *const CRYPT_XML_ALGORITHM, rgtransform: Option<&[CRYPT_XML_ALGORITHM]>, phreference: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cryptxml.dll" "system" fn CryptXmlCreateReference(hcryptxml : *const core::ffi::c_void, dwflags : u32, wszid : windows_core::PCWSTR, wszuri : windows_core::PCWSTR, wsztype : windows_core::PCWSTR, pdigestmethod : *const CRYPT_XML_ALGORITHM, ctransform : u32, rgtransform : *const CRYPT_XML_ALGORITHM, phreference : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CryptXmlCreateReference(hcryptxml, dwflags, wszid.param().abi(), wszuri.param().abi(), wsztype.param().abi(), pdigestmethod, rgtransform.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(rgtransform.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), phreference as _).ok() }
}
#[inline]
pub unsafe fn CryptXmlDigestReference(hreference: *const core::ffi::c_void, dwflags: u32, pdataproviderin: *const CRYPT_XML_DATA_PROVIDER) -> windows_core::Result<()> {
    windows_core::link!("cryptxml.dll" "system" fn CryptXmlDigestReference(hreference : *const core::ffi::c_void, dwflags : u32, pdataproviderin : *const CRYPT_XML_DATA_PROVIDER) -> windows_core::HRESULT);
    unsafe { CryptXmlDigestReference(hreference, dwflags, pdataproviderin).ok() }
}
#[inline]
pub unsafe fn CryptXmlEncode(hcryptxml: *const core::ffi::c_void, dwcharset: CRYPT_XML_CHARSET, rgproperty: Option<&[CRYPT_XML_PROPERTY]>, pvcallbackstate: *mut core::ffi::c_void, pfnwrite: PFN_CRYPT_XML_WRITE_CALLBACK) -> windows_core::Result<()> {
    windows_core::link!("cryptxml.dll" "system" fn CryptXmlEncode(hcryptxml : *const core::ffi::c_void, dwcharset : CRYPT_XML_CHARSET, rgproperty : *const CRYPT_XML_PROPERTY, cproperty : u32, pvcallbackstate : *mut core::ffi::c_void, pfnwrite : PFN_CRYPT_XML_WRITE_CALLBACK) -> windows_core::HRESULT);
    unsafe { CryptXmlEncode(hcryptxml, dwcharset, core::mem::transmute(rgproperty.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), rgproperty.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pvcallbackstate as _, pfnwrite).ok() }
}
#[inline]
pub unsafe fn CryptXmlEnumAlgorithmInfo(dwgroupid: u32, dwflags: u32, pvarg: Option<*mut core::ffi::c_void>, pfnenumalginfo: PFN_CRYPT_XML_ENUM_ALG_INFO) -> windows_core::Result<()> {
    windows_core::link!("cryptxml.dll" "system" fn CryptXmlEnumAlgorithmInfo(dwgroupid : u32, dwflags : u32, pvarg : *mut core::ffi::c_void, pfnenumalginfo : PFN_CRYPT_XML_ENUM_ALG_INFO) -> windows_core::HRESULT);
    unsafe { CryptXmlEnumAlgorithmInfo(dwgroupid, dwflags, pvarg.unwrap_or(core::mem::zeroed()) as _, pfnenumalginfo).ok() }
}
#[inline]
pub unsafe fn CryptXmlFindAlgorithmInfo(dwfindbytype: u32, pvfindby: *const core::ffi::c_void, dwgroupid: u32, dwflags: u32) -> *mut CRYPT_XML_ALGORITHM_INFO {
    windows_core::link!("cryptxml.dll" "system" fn CryptXmlFindAlgorithmInfo(dwfindbytype : u32, pvfindby : *const core::ffi::c_void, dwgroupid : u32, dwflags : u32) -> *mut CRYPT_XML_ALGORITHM_INFO);
    unsafe { CryptXmlFindAlgorithmInfo(dwfindbytype, pvfindby, dwgroupid, dwflags) }
}
#[inline]
pub unsafe fn CryptXmlGetAlgorithmInfo(pxmlalgorithm: *const CRYPT_XML_ALGORITHM, dwflags: CRYPT_XML_FLAGS) -> windows_core::Result<*mut CRYPT_XML_ALGORITHM_INFO> {
    windows_core::link!("cryptxml.dll" "system" fn CryptXmlGetAlgorithmInfo(pxmlalgorithm : *const CRYPT_XML_ALGORITHM, dwflags : CRYPT_XML_FLAGS, ppalginfo : *mut *mut CRYPT_XML_ALGORITHM_INFO) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CryptXmlGetAlgorithmInfo(pxmlalgorithm, dwflags, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn CryptXmlGetDocContext(hcryptxml: *const core::ffi::c_void) -> windows_core::Result<*mut CRYPT_XML_DOC_CTXT> {
    windows_core::link!("cryptxml.dll" "system" fn CryptXmlGetDocContext(hcryptxml : *const core::ffi::c_void, ppstruct : *mut *mut CRYPT_XML_DOC_CTXT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CryptXmlGetDocContext(hcryptxml, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn CryptXmlGetReference(hcryptxml: *const core::ffi::c_void) -> windows_core::Result<*mut CRYPT_XML_REFERENCE> {
    windows_core::link!("cryptxml.dll" "system" fn CryptXmlGetReference(hcryptxml : *const core::ffi::c_void, ppstruct : *mut *mut CRYPT_XML_REFERENCE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CryptXmlGetReference(hcryptxml, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn CryptXmlGetSignature(hcryptxml: *const core::ffi::c_void) -> windows_core::Result<*mut CRYPT_XML_SIGNATURE> {
    windows_core::link!("cryptxml.dll" "system" fn CryptXmlGetSignature(hcryptxml : *const core::ffi::c_void, ppstruct : *mut *mut CRYPT_XML_SIGNATURE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CryptXmlGetSignature(hcryptxml, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn CryptXmlGetStatus(hcryptxml: *const core::ffi::c_void) -> windows_core::Result<CRYPT_XML_STATUS> {
    windows_core::link!("cryptxml.dll" "system" fn CryptXmlGetStatus(hcryptxml : *const core::ffi::c_void, pstatus : *mut CRYPT_XML_STATUS) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CryptXmlGetStatus(hcryptxml, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn CryptXmlGetTransforms() -> windows_core::Result<*mut CRYPT_XML_TRANSFORM_CHAIN_CONFIG> {
    windows_core::link!("cryptxml.dll" "system" fn CryptXmlGetTransforms(ppconfig : *mut *mut CRYPT_XML_TRANSFORM_CHAIN_CONFIG) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CryptXmlGetTransforms(&mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn CryptXmlImportPublicKey(dwflags: CRYPT_XML_FLAGS, pkeyvalue: *const CRYPT_XML_KEY_VALUE) -> windows_core::Result<BCRYPT_KEY_HANDLE> {
    windows_core::link!("cryptxml.dll" "system" fn CryptXmlImportPublicKey(dwflags : CRYPT_XML_FLAGS, pkeyvalue : *const CRYPT_XML_KEY_VALUE, phkey : *mut BCRYPT_KEY_HANDLE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CryptXmlImportPublicKey(dwflags, pkeyvalue, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn CryptXmlOpenToDecode(pconfig: Option<*const CRYPT_XML_TRANSFORM_CHAIN_CONFIG>, dwflags: CRYPT_XML_FLAGS, rgproperty: Option<&[CRYPT_XML_PROPERTY]>, pencoded: *const CRYPT_XML_BLOB, phcryptxml: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_core::link!("cryptxml.dll" "system" fn CryptXmlOpenToDecode(pconfig : *const CRYPT_XML_TRANSFORM_CHAIN_CONFIG, dwflags : CRYPT_XML_FLAGS, rgproperty : *const CRYPT_XML_PROPERTY, cproperty : u32, pencoded : *const CRYPT_XML_BLOB, phcryptxml : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CryptXmlOpenToDecode(pconfig.unwrap_or(core::mem::zeroed()) as _, dwflags, core::mem::transmute(rgproperty.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), rgproperty.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pencoded, phcryptxml as _).ok() }
}
#[inline]
pub unsafe fn CryptXmlOpenToEncode<P2>(pconfig: Option<*const CRYPT_XML_TRANSFORM_CHAIN_CONFIG>, dwflags: CRYPT_XML_FLAGS, wszid: P2, rgproperty: Option<&[CRYPT_XML_PROPERTY]>, pencoded: Option<*const CRYPT_XML_BLOB>, phsignature: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cryptxml.dll" "system" fn CryptXmlOpenToEncode(pconfig : *const CRYPT_XML_TRANSFORM_CHAIN_CONFIG, dwflags : CRYPT_XML_FLAGS, wszid : windows_core::PCWSTR, rgproperty : *const CRYPT_XML_PROPERTY, cproperty : u32, pencoded : *const CRYPT_XML_BLOB, phsignature : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CryptXmlOpenToEncode(pconfig.unwrap_or(core::mem::zeroed()) as _, dwflags, wszid.param().abi(), core::mem::transmute(rgproperty.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), rgproperty.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pencoded.unwrap_or(core::mem::zeroed()) as _, phsignature as _).ok() }
}
#[inline]
pub unsafe fn CryptXmlSetHMACSecret(hsignature: *const core::ffi::c_void, pbsecret: &[u8]) -> windows_core::Result<()> {
    windows_core::link!("cryptxml.dll" "system" fn CryptXmlSetHMACSecret(hsignature : *const core::ffi::c_void, pbsecret : *const u8, cbsecret : u32) -> windows_core::HRESULT);
    unsafe { CryptXmlSetHMACSecret(hsignature, core::mem::transmute(pbsecret.as_ptr()), pbsecret.len().try_into().unwrap()).ok() }
}
#[inline]
pub unsafe fn CryptXmlSign(hsignature: *const core::ffi::c_void, hkey: Option<HCRYPTPROV_OR_NCRYPT_KEY_HANDLE>, dwkeyspec: CERT_KEY_SPEC, dwflags: CRYPT_XML_FLAGS, dwkeyinfospec: CRYPT_XML_KEYINFO_SPEC, pvkeyinfospec: Option<*const core::ffi::c_void>, psignaturemethod: *const CRYPT_XML_ALGORITHM, pcanonicalization: *const CRYPT_XML_ALGORITHM) -> windows_core::Result<()> {
    windows_core::link!("cryptxml.dll" "system" fn CryptXmlSign(hsignature : *const core::ffi::c_void, hkey : HCRYPTPROV_OR_NCRYPT_KEY_HANDLE, dwkeyspec : CERT_KEY_SPEC, dwflags : CRYPT_XML_FLAGS, dwkeyinfospec : CRYPT_XML_KEYINFO_SPEC, pvkeyinfospec : *const core::ffi::c_void, psignaturemethod : *const CRYPT_XML_ALGORITHM, pcanonicalization : *const CRYPT_XML_ALGORITHM) -> windows_core::HRESULT);
    unsafe { CryptXmlSign(hsignature, hkey.unwrap_or(core::mem::zeroed()) as _, dwkeyspec, dwflags, dwkeyinfospec, pvkeyinfospec.unwrap_or(core::mem::zeroed()) as _, psignaturemethod, pcanonicalization).ok() }
}
#[inline]
pub unsafe fn CryptXmlVerifySignature(hsignature: *const core::ffi::c_void, hkey: Option<BCRYPT_KEY_HANDLE>, dwflags: CRYPT_XML_FLAGS) -> windows_core::Result<()> {
    windows_core::link!("cryptxml.dll" "system" fn CryptXmlVerifySignature(hsignature : *const core::ffi::c_void, hkey : BCRYPT_KEY_HANDLE, dwflags : CRYPT_XML_FLAGS) -> windows_core::HRESULT);
    unsafe { CryptXmlVerifySignature(hsignature, hkey.unwrap_or(core::mem::zeroed()) as _, dwflags).ok() }
}
#[inline]
pub unsafe fn Decrypt(hcrypto: *const INFORMATIONCARD_CRYPTO_HANDLE, foaep: bool, pindata: &[u8], pcboutdata: *mut u32, ppoutdata: *mut *mut u8) -> windows_core::Result<()> {
    windows_core::link!("infocardapi.dll" "system" fn Decrypt(hcrypto : *const INFORMATIONCARD_CRYPTO_HANDLE, foaep : windows_core::BOOL, cbindata : u32, pindata : *const u8, pcboutdata : *mut u32, ppoutdata : *mut *mut u8) -> windows_core::HRESULT);
    unsafe { Decrypt(hcrypto, foaep.into(), pindata.len().try_into().unwrap(), core::mem::transmute(pindata.as_ptr()), pcboutdata as _, ppoutdata as _).ok() }
}
#[inline]
pub unsafe fn Encrypt(hcrypto: *const INFORMATIONCARD_CRYPTO_HANDLE, foaep: bool, pindata: &[u8], pcboutdata: *mut u32, ppoutdata: *mut *mut u8) -> windows_core::Result<()> {
    windows_core::link!("infocardapi.dll" "system" fn Encrypt(hcrypto : *const INFORMATIONCARD_CRYPTO_HANDLE, foaep : windows_core::BOOL, cbindata : u32, pindata : *const u8, pcboutdata : *mut u32, ppoutdata : *mut *mut u8) -> windows_core::HRESULT);
    unsafe { Encrypt(hcrypto, foaep.into(), pindata.len().try_into().unwrap(), core::mem::transmute(pindata.as_ptr()), pcboutdata as _, ppoutdata as _).ok() }
}
#[inline]
pub unsafe fn FindCertsByIssuer<P5>(pcertchains: Option<*mut CERT_CHAIN>, pcbcertchains: *mut u32, pccertchains: *mut u32, pbencodedissuername: Option<&[u8]>, pwszpurpose: P5, dwkeyspec: u32) -> windows_core::Result<()>
where
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wintrust.dll" "system" fn FindCertsByIssuer(pcertchains : *mut CERT_CHAIN, pcbcertchains : *mut u32, pccertchains : *mut u32, pbencodedissuername : *const u8, cbencodedissuername : u32, pwszpurpose : windows_core::PCWSTR, dwkeyspec : u32) -> windows_core::HRESULT);
    unsafe { FindCertsByIssuer(pcertchains.unwrap_or(core::mem::zeroed()) as _, pcbcertchains as _, pccertchains as _, core::mem::transmute(pbencodedissuername.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pbencodedissuername.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pwszpurpose.param().abi(), dwkeyspec).ok() }
}
#[inline]
pub unsafe fn FreeToken(pallocmemory: *const GENERIC_XML_TOKEN) -> windows_core::BOOL {
    windows_core::link!("infocardapi.dll" "system" fn FreeToken(pallocmemory : *const GENERIC_XML_TOKEN) -> windows_core::BOOL);
    unsafe { FreeToken(pallocmemory) }
}
#[inline]
pub unsafe fn GenerateDerivedKey<P7>(hcrypto: *const INFORMATIONCARD_CRYPTO_HANDLE, plabel: &[u8], pnonce: &[u8], derivedkeylength: u32, offset: u32, algid: P7, pcbkey: *mut u32, ppkey: *mut *mut u8) -> windows_core::Result<()>
where
    P7: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("infocardapi.dll" "system" fn GenerateDerivedKey(hcrypto : *const INFORMATIONCARD_CRYPTO_HANDLE, cblabel : u32, plabel : *const u8, cbnonce : u32, pnonce : *const u8, derivedkeylength : u32, offset : u32, algid : windows_core::PCWSTR, pcbkey : *mut u32, ppkey : *mut *mut u8) -> windows_core::HRESULT);
    unsafe { GenerateDerivedKey(hcrypto, plabel.len().try_into().unwrap(), core::mem::transmute(plabel.as_ptr()), pnonce.len().try_into().unwrap(), core::mem::transmute(pnonce.as_ptr()), derivedkeylength, offset, algid.param().abi(), pcbkey as _, ppkey as _).ok() }
}
#[inline]
pub unsafe fn GetAsymmetricEncryptionInterface<P0, P1>(pszprovidername: P0, pszalgid: P1, ppfunctiontable: *mut *mut BCRYPT_ASYMMETRIC_ENCRYPTION_FUNCTION_TABLE, dwflags: u32) -> super::super::Foundation::NTSTATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("bcryptprimitives.dll" "system" fn GetAsymmetricEncryptionInterface(pszprovidername : windows_core::PCWSTR, pszalgid : windows_core::PCWSTR, ppfunctiontable : *mut *mut BCRYPT_ASYMMETRIC_ENCRYPTION_FUNCTION_TABLE, dwflags : u32) -> super::super::Foundation:: NTSTATUS);
    unsafe { GetAsymmetricEncryptionInterface(pszprovidername.param().abi(), pszalgid.param().abi(), ppfunctiontable as _, dwflags) }
}
#[inline]
pub unsafe fn GetBrowserToken(dwparamtype: u32, pparam: *const core::ffi::c_void, pcbtoken: Option<*mut u32>, pptoken: Option<*mut *mut u8>) -> windows_core::Result<()> {
    windows_core::link!("infocardapi.dll" "system" fn GetBrowserToken(dwparamtype : u32, pparam : *const core::ffi::c_void, pcbtoken : *mut u32, pptoken : *mut *mut u8) -> windows_core::HRESULT);
    unsafe { GetBrowserToken(dwparamtype, pparam, pcbtoken.unwrap_or(core::mem::zeroed()) as _, pptoken.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn GetCipherInterface<P0, P1>(pszprovidername: P0, pszalgid: P1, ppfunctiontable: *mut *mut BCRYPT_CIPHER_FUNCTION_TABLE, dwflags: u32) -> super::super::Foundation::NTSTATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("bcryptprimitives.dll" "system" fn GetCipherInterface(pszprovidername : windows_core::PCWSTR, pszalgid : windows_core::PCWSTR, ppfunctiontable : *mut *mut BCRYPT_CIPHER_FUNCTION_TABLE, dwflags : u32) -> super::super::Foundation:: NTSTATUS);
    unsafe { GetCipherInterface(pszprovidername.param().abi(), pszalgid.param().abi(), ppfunctiontable as _, dwflags) }
}
#[inline]
pub unsafe fn GetCryptoTransform(hsymmetriccrypto: *const INFORMATIONCARD_CRYPTO_HANDLE, mode: u32, padding: PaddingMode, feedbacksize: u32, direction: Direction, piv: &[u8]) -> windows_core::Result<*mut INFORMATIONCARD_CRYPTO_HANDLE> {
    windows_core::link!("infocardapi.dll" "system" fn GetCryptoTransform(hsymmetriccrypto : *const INFORMATIONCARD_CRYPTO_HANDLE, mode : u32, padding : PaddingMode, feedbacksize : u32, direction : Direction, cbiv : u32, piv : *const u8, pphtransform : *mut *mut INFORMATIONCARD_CRYPTO_HANDLE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetCryptoTransform(hsymmetriccrypto, mode, padding, feedbacksize, direction, piv.len().try_into().unwrap(), core::mem::transmute(piv.as_ptr()), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn GetHashInterface<P0, P1>(pszprovidername: P0, pszalgid: P1, ppfunctiontable: *mut *mut BCRYPT_HASH_FUNCTION_TABLE, dwflags: u32) -> super::super::Foundation::NTSTATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("bcryptprimitives.dll" "system" fn GetHashInterface(pszprovidername : windows_core::PCWSTR, pszalgid : windows_core::PCWSTR, ppfunctiontable : *mut *mut BCRYPT_HASH_FUNCTION_TABLE, dwflags : u32) -> super::super::Foundation:: NTSTATUS);
    unsafe { GetHashInterface(pszprovidername.param().abi(), pszalgid.param().abi(), ppfunctiontable as _, dwflags) }
}
#[inline]
pub unsafe fn GetKeyDerivationInterface<P0, P1>(pszprovidername: P0, pszalgid: P1, ppfunctiontable: *mut *mut BCRYPT_KEY_DERIVATION_FUNCTION_TABLE, dwflags: u32) -> super::super::Foundation::NTSTATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("bcryptprimitives.dll" "system" fn GetKeyDerivationInterface(pszprovidername : windows_core::PCWSTR, pszalgid : windows_core::PCWSTR, ppfunctiontable : *mut *mut BCRYPT_KEY_DERIVATION_FUNCTION_TABLE, dwflags : u32) -> super::super::Foundation:: NTSTATUS);
    unsafe { GetKeyDerivationInterface(pszprovidername.param().abi(), pszalgid.param().abi(), ppfunctiontable as _, dwflags) }
}
#[inline]
pub unsafe fn GetKeyStorageInterface<P0>(pszprovidername: P0, ppfunctiontable: *mut *mut NCRYPT_KEY_STORAGE_FUNCTION_TABLE, dwflags: u32) -> super::super::Foundation::NTSTATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ncrypt.dll" "system" fn GetKeyStorageInterface(pszprovidername : windows_core::PCWSTR, ppfunctiontable : *mut *mut NCRYPT_KEY_STORAGE_FUNCTION_TABLE, dwflags : u32) -> super::super::Foundation:: NTSTATUS);
    unsafe { GetKeyStorageInterface(pszprovidername.param().abi(), ppfunctiontable as _, dwflags) }
}
#[inline]
pub unsafe fn GetKeyedHash(hsymmetriccrypto: *const INFORMATIONCARD_CRYPTO_HANDLE) -> windows_core::Result<*mut INFORMATIONCARD_CRYPTO_HANDLE> {
    windows_core::link!("infocardapi.dll" "system" fn GetKeyedHash(hsymmetriccrypto : *const INFORMATIONCARD_CRYPTO_HANDLE, pphhash : *mut *mut INFORMATIONCARD_CRYPTO_HANDLE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetKeyedHash(hsymmetriccrypto, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn GetRngInterface<P0>(pszprovidername: P0, ppfunctiontable: *mut *mut BCRYPT_RNG_FUNCTION_TABLE, dwflags: u32) -> super::super::Foundation::NTSTATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("bcryptprimitives.dll" "system" fn GetRngInterface(pszprovidername : windows_core::PCWSTR, ppfunctiontable : *mut *mut BCRYPT_RNG_FUNCTION_TABLE, dwflags : u32) -> super::super::Foundation:: NTSTATUS);
    unsafe { GetRngInterface(pszprovidername.param().abi(), ppfunctiontable as _, dwflags) }
}
#[inline]
pub unsafe fn GetSChannelInterface<P0>(pszprovidername: P0, ppfunctiontable: *mut *mut NCRYPT_SSL_FUNCTION_TABLE, dwflags: u32) -> super::super::Foundation::NTSTATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ncrypt.dll" "system" fn GetSChannelInterface(pszprovidername : windows_core::PCWSTR, ppfunctiontable : *mut *mut NCRYPT_SSL_FUNCTION_TABLE, dwflags : u32) -> super::super::Foundation:: NTSTATUS);
    unsafe { GetSChannelInterface(pszprovidername.param().abi(), ppfunctiontable as _, dwflags) }
}
#[inline]
pub unsafe fn GetSecretAgreementInterface<P0, P1>(pszprovidername: P0, pszalgid: P1, ppfunctiontable: *mut *mut BCRYPT_SECRET_AGREEMENT_FUNCTION_TABLE, dwflags: u32) -> super::super::Foundation::NTSTATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("bcryptprimitives.dll" "system" fn GetSecretAgreementInterface(pszprovidername : windows_core::PCWSTR, pszalgid : windows_core::PCWSTR, ppfunctiontable : *mut *mut BCRYPT_SECRET_AGREEMENT_FUNCTION_TABLE, dwflags : u32) -> super::super::Foundation:: NTSTATUS);
    unsafe { GetSecretAgreementInterface(pszprovidername.param().abi(), pszalgid.param().abi(), ppfunctiontable as _, dwflags) }
}
#[inline]
pub unsafe fn GetSignatureInterface<P0, P1>(pszprovidername: P0, pszalgid: P1, ppfunctiontable: *mut *mut BCRYPT_SIGNATURE_FUNCTION_TABLE, dwflags: u32) -> super::super::Foundation::NTSTATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("bcryptprimitives.dll" "system" fn GetSignatureInterface(pszprovidername : windows_core::PCWSTR, pszalgid : windows_core::PCWSTR, ppfunctiontable : *mut *mut BCRYPT_SIGNATURE_FUNCTION_TABLE, dwflags : u32) -> super::super::Foundation:: NTSTATUS);
    unsafe { GetSignatureInterface(pszprovidername.param().abi(), pszalgid.param().abi(), ppfunctiontable as _, dwflags) }
}
#[inline]
pub unsafe fn GetToken(ppolicychain: &[POLICY_ELEMENT], securitytoken: *mut *mut GENERIC_XML_TOKEN, phprooftokencrypto: *mut *mut INFORMATIONCARD_CRYPTO_HANDLE) -> windows_core::Result<()> {
    windows_core::link!("infocardapi.dll" "system" fn GetToken(cpolicychain : u32, ppolicychain : *const POLICY_ELEMENT, securitytoken : *mut *mut GENERIC_XML_TOKEN, phprooftokencrypto : *mut *mut INFORMATIONCARD_CRYPTO_HANDLE) -> windows_core::HRESULT);
    unsafe { GetToken(ppolicychain.len().try_into().unwrap(), core::mem::transmute(ppolicychain.as_ptr()), securitytoken as _, phprooftokencrypto as _).ok() }
}
#[inline]
pub unsafe fn HashCore(hcrypto: *const INFORMATIONCARD_CRYPTO_HANDLE, pindata: &[u8]) -> windows_core::Result<()> {
    windows_core::link!("infocardapi.dll" "system" fn HashCore(hcrypto : *const INFORMATIONCARD_CRYPTO_HANDLE, cbindata : u32, pindata : *const u8) -> windows_core::HRESULT);
    unsafe { HashCore(hcrypto, pindata.len().try_into().unwrap(), core::mem::transmute(pindata.as_ptr())).ok() }
}
#[inline]
pub unsafe fn HashFinal(hcrypto: *const INFORMATIONCARD_CRYPTO_HANDLE, pindata: &[u8], pcboutdata: *mut u32, ppoutdata: *mut *mut u8) -> windows_core::Result<()> {
    windows_core::link!("infocardapi.dll" "system" fn HashFinal(hcrypto : *const INFORMATIONCARD_CRYPTO_HANDLE, cbindata : u32, pindata : *const u8, pcboutdata : *mut u32, ppoutdata : *mut *mut u8) -> windows_core::HRESULT);
    unsafe { HashFinal(hcrypto, pindata.len().try_into().unwrap(), core::mem::transmute(pindata.as_ptr()), pcboutdata as _, ppoutdata as _).ok() }
}
#[inline]
pub unsafe fn ImportInformationCard<P0>(filename: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("infocardapi.dll" "system" fn ImportInformationCard(filename : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { ImportInformationCard(filename.param().abi()).ok() }
}
#[inline]
pub unsafe fn ManageCardSpace() -> windows_core::Result<()> {
    windows_core::link!("infocardapi.dll" "system" fn ManageCardSpace() -> windows_core::HRESULT);
    unsafe { ManageCardSpace().ok() }
}
#[inline]
pub unsafe fn NCryptCloseProtectionDescriptor(hdescriptor: super::NCRYPT_DESCRIPTOR_HANDLE) -> windows_core::Result<()> {
    windows_core::link!("ncrypt.dll" "system" fn NCryptCloseProtectionDescriptor(hdescriptor : super:: NCRYPT_DESCRIPTOR_HANDLE) -> windows_core::HRESULT);
    unsafe { NCryptCloseProtectionDescriptor(hdescriptor).ok() }
}
#[inline]
pub unsafe fn NCryptCreateClaim(hsubjectkey: Option<NCRYPT_KEY_HANDLE>, hauthoritykey: Option<NCRYPT_KEY_HANDLE>, dwclaimtype: u32, pparameterlist: Option<*const BCryptBufferDesc>, pbclaimblob: Option<&mut [u8]>, pcbresult: *mut u32, dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("ncrypt.dll" "system" fn NCryptCreateClaim(hsubjectkey : NCRYPT_KEY_HANDLE, hauthoritykey : NCRYPT_KEY_HANDLE, dwclaimtype : u32, pparameterlist : *const BCryptBufferDesc, pbclaimblob : *mut u8, cbclaimblob : u32, pcbresult : *mut u32, dwflags : u32) -> windows_core::HRESULT);
    unsafe { NCryptCreateClaim(hsubjectkey.unwrap_or(core::mem::zeroed()) as _, hauthoritykey.unwrap_or(core::mem::zeroed()) as _, dwclaimtype, pparameterlist.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pbclaimblob.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pbclaimblob.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbresult as _, dwflags).ok() }
}
#[inline]
pub unsafe fn NCryptCreatePersistedKey<P2, P3>(hprovider: NCRYPT_PROV_HANDLE, phkey: *mut NCRYPT_KEY_HANDLE, pszalgid: P2, pszkeyname: P3, dwlegacykeyspec: CERT_KEY_SPEC, dwflags: NCRYPT_FLAGS) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ncrypt.dll" "system" fn NCryptCreatePersistedKey(hprovider : NCRYPT_PROV_HANDLE, phkey : *mut NCRYPT_KEY_HANDLE, pszalgid : windows_core::PCWSTR, pszkeyname : windows_core::PCWSTR, dwlegacykeyspec : CERT_KEY_SPEC, dwflags : NCRYPT_FLAGS) -> windows_core::HRESULT);
    unsafe { NCryptCreatePersistedKey(hprovider, phkey as _, pszalgid.param().abi(), pszkeyname.param().abi(), dwlegacykeyspec, dwflags).ok() }
}
#[inline]
pub unsafe fn NCryptCreateProtectionDescriptor<P0>(pwszdescriptorstring: P0, dwflags: u32) -> windows_core::Result<super::NCRYPT_DESCRIPTOR_HANDLE>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ncrypt.dll" "system" fn NCryptCreateProtectionDescriptor(pwszdescriptorstring : windows_core::PCWSTR, dwflags : u32, phdescriptor : *mut super:: NCRYPT_DESCRIPTOR_HANDLE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        NCryptCreateProtectionDescriptor(pwszdescriptorstring.param().abi(), dwflags, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn NCryptDecrypt(hkey: NCRYPT_KEY_HANDLE, pbinput: Option<&[u8]>, ppaddinginfo: Option<*const core::ffi::c_void>, pboutput: Option<&mut [u8]>, pcbresult: *mut u32, dwflags: NCRYPT_FLAGS) -> windows_core::Result<()> {
    windows_core::link!("ncrypt.dll" "system" fn NCryptDecrypt(hkey : NCRYPT_KEY_HANDLE, pbinput : *const u8, cbinput : u32, ppaddinginfo : *const core::ffi::c_void, pboutput : *mut u8, cboutput : u32, pcbresult : *mut u32, dwflags : NCRYPT_FLAGS) -> windows_core::HRESULT);
    unsafe { NCryptDecrypt(hkey, core::mem::transmute(pbinput.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pbinput.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), ppaddinginfo.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pboutput.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pboutput.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbresult as _, dwflags).ok() }
}
#[inline]
pub unsafe fn NCryptDeleteKey(hkey: NCRYPT_KEY_HANDLE, dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("ncrypt.dll" "system" fn NCryptDeleteKey(hkey : NCRYPT_KEY_HANDLE, dwflags : u32) -> windows_core::HRESULT);
    unsafe { NCryptDeleteKey(hkey, dwflags).ok() }
}
#[inline]
pub unsafe fn NCryptDeriveKey<P1>(hsharedsecret: NCRYPT_SECRET_HANDLE, pwszkdf: P1, pparameterlist: Option<*const BCryptBufferDesc>, pbderivedkey: Option<&mut [u8]>, pcbresult: *mut u32, dwflags: u32) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ncrypt.dll" "system" fn NCryptDeriveKey(hsharedsecret : NCRYPT_SECRET_HANDLE, pwszkdf : windows_core::PCWSTR, pparameterlist : *const BCryptBufferDesc, pbderivedkey : *mut u8, cbderivedkey : u32, pcbresult : *mut u32, dwflags : u32) -> windows_core::HRESULT);
    unsafe { NCryptDeriveKey(hsharedsecret, pwszkdf.param().abi(), pparameterlist.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pbderivedkey.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pbderivedkey.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbresult as _, dwflags).ok() }
}
#[inline]
pub unsafe fn NCryptEncrypt(hkey: NCRYPT_KEY_HANDLE, pbinput: Option<&[u8]>, ppaddinginfo: Option<*const core::ffi::c_void>, pboutput: Option<&mut [u8]>, pcbresult: *mut u32, dwflags: NCRYPT_FLAGS) -> windows_core::Result<()> {
    windows_core::link!("ncrypt.dll" "system" fn NCryptEncrypt(hkey : NCRYPT_KEY_HANDLE, pbinput : *const u8, cbinput : u32, ppaddinginfo : *const core::ffi::c_void, pboutput : *mut u8, cboutput : u32, pcbresult : *mut u32, dwflags : NCRYPT_FLAGS) -> windows_core::HRESULT);
    unsafe { NCryptEncrypt(hkey, core::mem::transmute(pbinput.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pbinput.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), ppaddinginfo.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pboutput.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pboutput.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbresult as _, dwflags).ok() }
}
#[inline]
pub unsafe fn NCryptEnumAlgorithms(hprovider: NCRYPT_PROV_HANDLE, dwalgoperations: NCRYPT_OPERATION, pdwalgcount: *mut u32, ppalglist: *mut *mut NCryptAlgorithmName, dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("ncrypt.dll" "system" fn NCryptEnumAlgorithms(hprovider : NCRYPT_PROV_HANDLE, dwalgoperations : NCRYPT_OPERATION, pdwalgcount : *mut u32, ppalglist : *mut *mut NCryptAlgorithmName, dwflags : u32) -> windows_core::HRESULT);
    unsafe { NCryptEnumAlgorithms(hprovider, dwalgoperations, pdwalgcount as _, ppalglist as _, dwflags).ok() }
}
#[inline]
pub unsafe fn NCryptEnumKeys<P1>(hprovider: NCRYPT_PROV_HANDLE, pszscope: P1, ppkeyname: *mut *mut NCryptKeyName, ppenumstate: *mut *mut core::ffi::c_void, dwflags: NCRYPT_FLAGS) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ncrypt.dll" "system" fn NCryptEnumKeys(hprovider : NCRYPT_PROV_HANDLE, pszscope : windows_core::PCWSTR, ppkeyname : *mut *mut NCryptKeyName, ppenumstate : *mut *mut core::ffi::c_void, dwflags : NCRYPT_FLAGS) -> windows_core::HRESULT);
    unsafe { NCryptEnumKeys(hprovider, pszscope.param().abi(), ppkeyname as _, ppenumstate as _, dwflags).ok() }
}
#[inline]
pub unsafe fn NCryptEnumStorageProviders(pdwprovidercount: *mut u32, ppproviderlist: *mut *mut NCryptProviderName, dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("ncrypt.dll" "system" fn NCryptEnumStorageProviders(pdwprovidercount : *mut u32, ppproviderlist : *mut *mut NCryptProviderName, dwflags : u32) -> windows_core::HRESULT);
    unsafe { NCryptEnumStorageProviders(pdwprovidercount as _, ppproviderlist as _, dwflags).ok() }
}
#[inline]
pub unsafe fn NCryptExportKey<P2>(hkey: NCRYPT_KEY_HANDLE, hexportkey: Option<NCRYPT_KEY_HANDLE>, pszblobtype: P2, pparameterlist: Option<*const BCryptBufferDesc>, pboutput: Option<&mut [u8]>, pcbresult: *mut u32, dwflags: NCRYPT_FLAGS) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ncrypt.dll" "system" fn NCryptExportKey(hkey : NCRYPT_KEY_HANDLE, hexportkey : NCRYPT_KEY_HANDLE, pszblobtype : windows_core::PCWSTR, pparameterlist : *const BCryptBufferDesc, pboutput : *mut u8, cboutput : u32, pcbresult : *mut u32, dwflags : NCRYPT_FLAGS) -> windows_core::HRESULT);
    unsafe { NCryptExportKey(hkey, hexportkey.unwrap_or(core::mem::zeroed()) as _, pszblobtype.param().abi(), pparameterlist.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pboutput.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pboutput.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbresult as _, dwflags).ok() }
}
#[inline]
pub unsafe fn NCryptFinalizeKey(hkey: NCRYPT_KEY_HANDLE, dwflags: NCRYPT_FLAGS) -> windows_core::Result<()> {
    windows_core::link!("ncrypt.dll" "system" fn NCryptFinalizeKey(hkey : NCRYPT_KEY_HANDLE, dwflags : NCRYPT_FLAGS) -> windows_core::HRESULT);
    unsafe { NCryptFinalizeKey(hkey, dwflags).ok() }
}
#[inline]
pub unsafe fn NCryptFreeBuffer(pvinput: *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_core::link!("ncrypt.dll" "system" fn NCryptFreeBuffer(pvinput : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { NCryptFreeBuffer(pvinput as _).ok() }
}
#[inline]
pub unsafe fn NCryptFreeObject(hobject: NCRYPT_HANDLE) -> windows_core::Result<()> {
    windows_core::link!("ncrypt.dll" "system" fn NCryptFreeObject(hobject : NCRYPT_HANDLE) -> windows_core::HRESULT);
    unsafe { NCryptFreeObject(hobject).ok() }
}
#[inline]
pub unsafe fn NCryptGetProperty<P1>(hobject: NCRYPT_HANDLE, pszproperty: P1, pboutput: Option<&mut [u8]>, pcbresult: *mut u32, dwflags: super::OBJECT_SECURITY_INFORMATION) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ncrypt.dll" "system" fn NCryptGetProperty(hobject : NCRYPT_HANDLE, pszproperty : windows_core::PCWSTR, pboutput : *mut u8, cboutput : u32, pcbresult : *mut u32, dwflags : super:: OBJECT_SECURITY_INFORMATION) -> windows_core::HRESULT);
    unsafe { NCryptGetProperty(hobject, pszproperty.param().abi(), core::mem::transmute(pboutput.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pboutput.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbresult as _, dwflags).ok() }
}
#[inline]
pub unsafe fn NCryptGetProtectionDescriptorInfo(hdescriptor: super::NCRYPT_DESCRIPTOR_HANDLE, pmempara: Option<*const NCRYPT_ALLOC_PARA>, dwinfotype: u32, ppvinfo: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_core::link!("ncrypt.dll" "system" fn NCryptGetProtectionDescriptorInfo(hdescriptor : super:: NCRYPT_DESCRIPTOR_HANDLE, pmempara : *const NCRYPT_ALLOC_PARA, dwinfotype : u32, ppvinfo : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { NCryptGetProtectionDescriptorInfo(hdescriptor, pmempara.unwrap_or(core::mem::zeroed()) as _, dwinfotype, ppvinfo as _).ok() }
}
#[inline]
pub unsafe fn NCryptImportKey<P2>(hprovider: NCRYPT_PROV_HANDLE, himportkey: Option<NCRYPT_KEY_HANDLE>, pszblobtype: P2, pparameterlist: Option<*const BCryptBufferDesc>, phkey: *mut NCRYPT_KEY_HANDLE, pbdata: &[u8], dwflags: NCRYPT_FLAGS) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ncrypt.dll" "system" fn NCryptImportKey(hprovider : NCRYPT_PROV_HANDLE, himportkey : NCRYPT_KEY_HANDLE, pszblobtype : windows_core::PCWSTR, pparameterlist : *const BCryptBufferDesc, phkey : *mut NCRYPT_KEY_HANDLE, pbdata : *const u8, cbdata : u32, dwflags : NCRYPT_FLAGS) -> windows_core::HRESULT);
    unsafe { NCryptImportKey(hprovider, himportkey.unwrap_or(core::mem::zeroed()) as _, pszblobtype.param().abi(), pparameterlist.unwrap_or(core::mem::zeroed()) as _, phkey as _, core::mem::transmute(pbdata.as_ptr()), pbdata.len().try_into().unwrap(), dwflags).ok() }
}
#[inline]
pub unsafe fn NCryptIsAlgSupported<P1>(hprovider: NCRYPT_PROV_HANDLE, pszalgid: P1, dwflags: u32) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ncrypt.dll" "system" fn NCryptIsAlgSupported(hprovider : NCRYPT_PROV_HANDLE, pszalgid : windows_core::PCWSTR, dwflags : u32) -> windows_core::HRESULT);
    unsafe { NCryptIsAlgSupported(hprovider, pszalgid.param().abi(), dwflags).ok() }
}
#[inline]
pub unsafe fn NCryptIsKeyHandle(hkey: NCRYPT_KEY_HANDLE) -> windows_core::BOOL {
    windows_core::link!("ncrypt.dll" "system" fn NCryptIsKeyHandle(hkey : NCRYPT_KEY_HANDLE) -> windows_core::BOOL);
    unsafe { NCryptIsKeyHandle(hkey) }
}
#[inline]
pub unsafe fn NCryptKeyDerivation(hkey: NCRYPT_KEY_HANDLE, pparameterlist: Option<*const BCryptBufferDesc>, pbderivedkey: &mut [u8], pcbresult: *mut u32, dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("ncrypt.dll" "system" fn NCryptKeyDerivation(hkey : NCRYPT_KEY_HANDLE, pparameterlist : *const BCryptBufferDesc, pbderivedkey : *mut u8, cbderivedkey : u32, pcbresult : *mut u32, dwflags : u32) -> windows_core::HRESULT);
    unsafe { NCryptKeyDerivation(hkey, pparameterlist.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pbderivedkey.as_ptr()), pbderivedkey.len().try_into().unwrap(), pcbresult as _, dwflags).ok() }
}
#[inline]
pub unsafe fn NCryptNotifyChangeKey(hprovider: NCRYPT_PROV_HANDLE, phevent: *mut super::super::Foundation::HANDLE, dwflags: NCRYPT_FLAGS) -> windows_core::Result<()> {
    windows_core::link!("ncrypt.dll" "system" fn NCryptNotifyChangeKey(hprovider : NCRYPT_PROV_HANDLE, phevent : *mut super::super::Foundation:: HANDLE, dwflags : NCRYPT_FLAGS) -> windows_core::HRESULT);
    unsafe { NCryptNotifyChangeKey(hprovider, phevent as _, dwflags).ok() }
}
#[inline]
pub unsafe fn NCryptOpenKey<P2>(hprovider: NCRYPT_PROV_HANDLE, phkey: *mut NCRYPT_KEY_HANDLE, pszkeyname: P2, dwlegacykeyspec: CERT_KEY_SPEC, dwflags: NCRYPT_FLAGS) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ncrypt.dll" "system" fn NCryptOpenKey(hprovider : NCRYPT_PROV_HANDLE, phkey : *mut NCRYPT_KEY_HANDLE, pszkeyname : windows_core::PCWSTR, dwlegacykeyspec : CERT_KEY_SPEC, dwflags : NCRYPT_FLAGS) -> windows_core::HRESULT);
    unsafe { NCryptOpenKey(hprovider, phkey as _, pszkeyname.param().abi(), dwlegacykeyspec, dwflags).ok() }
}
#[inline]
pub unsafe fn NCryptOpenStorageProvider<P1>(phprovider: *mut NCRYPT_PROV_HANDLE, pszprovidername: P1, dwflags: u32) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ncrypt.dll" "system" fn NCryptOpenStorageProvider(phprovider : *mut NCRYPT_PROV_HANDLE, pszprovidername : windows_core::PCWSTR, dwflags : u32) -> windows_core::HRESULT);
    unsafe { NCryptOpenStorageProvider(phprovider as _, pszprovidername.param().abi(), dwflags).ok() }
}
#[inline]
pub unsafe fn NCryptProtectSecret(hdescriptor: super::NCRYPT_DESCRIPTOR_HANDLE, dwflags: u32, pbdata: &[u8], pmempara: Option<*const NCRYPT_ALLOC_PARA>, hwnd: Option<super::super::Foundation::HWND>, ppbprotectedblob: *mut *mut u8, pcbprotectedblob: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("ncrypt.dll" "system" fn NCryptProtectSecret(hdescriptor : super:: NCRYPT_DESCRIPTOR_HANDLE, dwflags : u32, pbdata : *const u8, cbdata : u32, pmempara : *const NCRYPT_ALLOC_PARA, hwnd : super::super::Foundation:: HWND, ppbprotectedblob : *mut *mut u8, pcbprotectedblob : *mut u32) -> windows_core::HRESULT);
    unsafe { NCryptProtectSecret(hdescriptor, dwflags, core::mem::transmute(pbdata.as_ptr()), pbdata.len().try_into().unwrap(), pmempara.unwrap_or(core::mem::zeroed()) as _, hwnd.unwrap_or(core::mem::zeroed()) as _, ppbprotectedblob as _, pcbprotectedblob as _).ok() }
}
#[inline]
pub unsafe fn NCryptQueryProtectionDescriptorName<P0>(pwszname: P0, pwszdescriptorstring: Option<windows_core::PWSTR>, pcdescriptorstring: *mut usize, dwflags: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ncrypt.dll" "system" fn NCryptQueryProtectionDescriptorName(pwszname : windows_core::PCWSTR, pwszdescriptorstring : windows_core::PWSTR, pcdescriptorstring : *mut usize, dwflags : u32) -> windows_core::HRESULT);
    unsafe { NCryptQueryProtectionDescriptorName(pwszname.param().abi(), pwszdescriptorstring.unwrap_or(core::mem::zeroed()) as _, pcdescriptorstring as _, dwflags).ok() }
}
#[inline]
pub unsafe fn NCryptRegisterProtectionDescriptorName<P0, P1>(pwszname: P0, pwszdescriptorstring: P1, dwflags: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ncrypt.dll" "system" fn NCryptRegisterProtectionDescriptorName(pwszname : windows_core::PCWSTR, pwszdescriptorstring : windows_core::PCWSTR, dwflags : u32) -> windows_core::HRESULT);
    unsafe { NCryptRegisterProtectionDescriptorName(pwszname.param().abi(), pwszdescriptorstring.param().abi(), dwflags).ok() }
}
#[inline]
pub unsafe fn NCryptSecretAgreement(hprivkey: NCRYPT_KEY_HANDLE, hpubkey: NCRYPT_KEY_HANDLE, phagreedsecret: *mut NCRYPT_SECRET_HANDLE, dwflags: NCRYPT_FLAGS) -> windows_core::Result<()> {
    windows_core::link!("ncrypt.dll" "system" fn NCryptSecretAgreement(hprivkey : NCRYPT_KEY_HANDLE, hpubkey : NCRYPT_KEY_HANDLE, phagreedsecret : *mut NCRYPT_SECRET_HANDLE, dwflags : NCRYPT_FLAGS) -> windows_core::HRESULT);
    unsafe { NCryptSecretAgreement(hprivkey, hpubkey, phagreedsecret as _, dwflags).ok() }
}
#[inline]
pub unsafe fn NCryptSetProperty<P1>(hobject: NCRYPT_HANDLE, pszproperty: P1, pbinput: &[u8], dwflags: NCRYPT_FLAGS) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ncrypt.dll" "system" fn NCryptSetProperty(hobject : NCRYPT_HANDLE, pszproperty : windows_core::PCWSTR, pbinput : *const u8, cbinput : u32, dwflags : NCRYPT_FLAGS) -> windows_core::HRESULT);
    unsafe { NCryptSetProperty(hobject, pszproperty.param().abi(), core::mem::transmute(pbinput.as_ptr()), pbinput.len().try_into().unwrap(), dwflags).ok() }
}
#[inline]
pub unsafe fn NCryptSignHash(hkey: NCRYPT_KEY_HANDLE, ppaddinginfo: Option<*const core::ffi::c_void>, pbhashvalue: &[u8], pbsignature: Option<&mut [u8]>, pcbresult: *mut u32, dwflags: NCRYPT_FLAGS) -> windows_core::Result<()> {
    windows_core::link!("ncrypt.dll" "system" fn NCryptSignHash(hkey : NCRYPT_KEY_HANDLE, ppaddinginfo : *const core::ffi::c_void, pbhashvalue : *const u8, cbhashvalue : u32, pbsignature : *mut u8, cbsignature : u32, pcbresult : *mut u32, dwflags : NCRYPT_FLAGS) -> windows_core::HRESULT);
    unsafe { NCryptSignHash(hkey, ppaddinginfo.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pbhashvalue.as_ptr()), pbhashvalue.len().try_into().unwrap(), core::mem::transmute(pbsignature.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pbsignature.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbresult as _, dwflags).ok() }
}
#[inline]
pub unsafe fn NCryptStreamClose(hstream: super::NCRYPT_STREAM_HANDLE) -> windows_core::Result<()> {
    windows_core::link!("ncrypt.dll" "system" fn NCryptStreamClose(hstream : super:: NCRYPT_STREAM_HANDLE) -> windows_core::HRESULT);
    unsafe { NCryptStreamClose(hstream).ok() }
}
#[inline]
pub unsafe fn NCryptStreamOpenToProtect(hdescriptor: super::NCRYPT_DESCRIPTOR_HANDLE, dwflags: u32, hwnd: Option<super::super::Foundation::HWND>, pstreaminfo: *const NCRYPT_PROTECT_STREAM_INFO) -> windows_core::Result<super::NCRYPT_STREAM_HANDLE> {
    windows_core::link!("ncrypt.dll" "system" fn NCryptStreamOpenToProtect(hdescriptor : super:: NCRYPT_DESCRIPTOR_HANDLE, dwflags : u32, hwnd : super::super::Foundation:: HWND, pstreaminfo : *const NCRYPT_PROTECT_STREAM_INFO, phstream : *mut super:: NCRYPT_STREAM_HANDLE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        NCryptStreamOpenToProtect(hdescriptor, dwflags, hwnd.unwrap_or(core::mem::zeroed()) as _, pstreaminfo, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn NCryptStreamOpenToUnprotect(pstreaminfo: *const NCRYPT_PROTECT_STREAM_INFO, dwflags: u32, hwnd: Option<super::super::Foundation::HWND>) -> windows_core::Result<super::NCRYPT_STREAM_HANDLE> {
    windows_core::link!("ncrypt.dll" "system" fn NCryptStreamOpenToUnprotect(pstreaminfo : *const NCRYPT_PROTECT_STREAM_INFO, dwflags : u32, hwnd : super::super::Foundation:: HWND, phstream : *mut super:: NCRYPT_STREAM_HANDLE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        NCryptStreamOpenToUnprotect(pstreaminfo, dwflags, hwnd.unwrap_or(core::mem::zeroed()) as _, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn NCryptStreamOpenToUnprotectEx(pstreaminfo: *const NCRYPT_PROTECT_STREAM_INFO_EX, dwflags: u32, hwnd: Option<super::super::Foundation::HWND>) -> windows_core::Result<super::NCRYPT_STREAM_HANDLE> {
    windows_core::link!("ncrypt.dll" "system" fn NCryptStreamOpenToUnprotectEx(pstreaminfo : *const NCRYPT_PROTECT_STREAM_INFO_EX, dwflags : u32, hwnd : super::super::Foundation:: HWND, phstream : *mut super:: NCRYPT_STREAM_HANDLE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        NCryptStreamOpenToUnprotectEx(pstreaminfo, dwflags, hwnd.unwrap_or(core::mem::zeroed()) as _, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn NCryptStreamUpdate(hstream: super::NCRYPT_STREAM_HANDLE, pbdata: &[u8], ffinal: bool) -> windows_core::Result<()> {
    windows_core::link!("ncrypt.dll" "system" fn NCryptStreamUpdate(hstream : super:: NCRYPT_STREAM_HANDLE, pbdata : *const u8, cbdata : usize, ffinal : windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { NCryptStreamUpdate(hstream, core::mem::transmute(pbdata.as_ptr()), pbdata.len().try_into().unwrap(), ffinal.into()).ok() }
}
#[inline]
pub unsafe fn NCryptTranslateHandle(phprovider: Option<*mut NCRYPT_PROV_HANDLE>, phkey: *mut NCRYPT_KEY_HANDLE, hlegacyprov: usize, hlegacykey: Option<usize>, dwlegacykeyspec: Option<CERT_KEY_SPEC>, dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("ncrypt.dll" "system" fn NCryptTranslateHandle(phprovider : *mut NCRYPT_PROV_HANDLE, phkey : *mut NCRYPT_KEY_HANDLE, hlegacyprov : usize, hlegacykey : usize, dwlegacykeyspec : CERT_KEY_SPEC, dwflags : u32) -> windows_core::HRESULT);
    unsafe { NCryptTranslateHandle(phprovider.unwrap_or(core::mem::zeroed()) as _, phkey as _, hlegacyprov, hlegacykey.unwrap_or(core::mem::zeroed()) as _, dwlegacykeyspec.unwrap_or(core::mem::zeroed()) as _, dwflags).ok() }
}
#[inline]
pub unsafe fn NCryptUnprotectSecret(phdescriptor: Option<*mut super::NCRYPT_DESCRIPTOR_HANDLE>, dwflags: NCRYPT_FLAGS, pbprotectedblob: &[u8], pmempara: Option<*const NCRYPT_ALLOC_PARA>, hwnd: Option<super::super::Foundation::HWND>, ppbdata: *mut *mut u8, pcbdata: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("ncrypt.dll" "system" fn NCryptUnprotectSecret(phdescriptor : *mut super:: NCRYPT_DESCRIPTOR_HANDLE, dwflags : NCRYPT_FLAGS, pbprotectedblob : *const u8, cbprotectedblob : u32, pmempara : *const NCRYPT_ALLOC_PARA, hwnd : super::super::Foundation:: HWND, ppbdata : *mut *mut u8, pcbdata : *mut u32) -> windows_core::HRESULT);
    unsafe { NCryptUnprotectSecret(phdescriptor.unwrap_or(core::mem::zeroed()) as _, dwflags, core::mem::transmute(pbprotectedblob.as_ptr()), pbprotectedblob.len().try_into().unwrap(), pmempara.unwrap_or(core::mem::zeroed()) as _, hwnd.unwrap_or(core::mem::zeroed()) as _, ppbdata as _, pcbdata as _).ok() }
}
#[inline]
pub unsafe fn NCryptVerifyClaim(hsubjectkey: NCRYPT_KEY_HANDLE, hauthoritykey: Option<NCRYPT_KEY_HANDLE>, dwclaimtype: u32, pparameterlist: Option<*const BCryptBufferDesc>, pbclaimblob: &[u8], poutput: *mut BCryptBufferDesc, dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("ncrypt.dll" "system" fn NCryptVerifyClaim(hsubjectkey : NCRYPT_KEY_HANDLE, hauthoritykey : NCRYPT_KEY_HANDLE, dwclaimtype : u32, pparameterlist : *const BCryptBufferDesc, pbclaimblob : *const u8, cbclaimblob : u32, poutput : *mut BCryptBufferDesc, dwflags : u32) -> windows_core::HRESULT);
    unsafe { NCryptVerifyClaim(hsubjectkey, hauthoritykey.unwrap_or(core::mem::zeroed()) as _, dwclaimtype, pparameterlist.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pbclaimblob.as_ptr()), pbclaimblob.len().try_into().unwrap(), poutput as _, dwflags).ok() }
}
#[inline]
pub unsafe fn NCryptVerifySignature(hkey: NCRYPT_KEY_HANDLE, ppaddinginfo: Option<*const core::ffi::c_void>, pbhashvalue: &[u8], pbsignature: &[u8], dwflags: NCRYPT_FLAGS) -> windows_core::Result<()> {
    windows_core::link!("ncrypt.dll" "system" fn NCryptVerifySignature(hkey : NCRYPT_KEY_HANDLE, ppaddinginfo : *const core::ffi::c_void, pbhashvalue : *const u8, cbhashvalue : u32, pbsignature : *const u8, cbsignature : u32, dwflags : NCRYPT_FLAGS) -> windows_core::HRESULT);
    unsafe { NCryptVerifySignature(hkey, ppaddinginfo.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pbhashvalue.as_ptr()), pbhashvalue.len().try_into().unwrap(), core::mem::transmute(pbsignature.as_ptr()), pbsignature.len().try_into().unwrap(), dwflags).ok() }
}
#[inline]
pub unsafe fn PFXExportCertStore<P2>(hstore: HCERTSTORE, ppfx: *mut CRYPT_INTEGER_BLOB, szpassword: P2, dwflags: u32) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("crypt32.dll" "system" fn PFXExportCertStore(hstore : HCERTSTORE, ppfx : *mut CRYPT_INTEGER_BLOB, szpassword : windows_core::PCWSTR, dwflags : u32) -> windows_core::BOOL);
    unsafe { PFXExportCertStore(hstore, ppfx as _, szpassword.param().abi(), dwflags).ok() }
}
#[inline]
pub unsafe fn PFXExportCertStoreEx<P2>(hstore: HCERTSTORE, ppfx: *mut CRYPT_INTEGER_BLOB, szpassword: P2, pvpara: *const core::ffi::c_void, dwflags: u32) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("crypt32.dll" "system" fn PFXExportCertStoreEx(hstore : HCERTSTORE, ppfx : *mut CRYPT_INTEGER_BLOB, szpassword : windows_core::PCWSTR, pvpara : *const core::ffi::c_void, dwflags : u32) -> windows_core::BOOL);
    unsafe { PFXExportCertStoreEx(hstore, ppfx as _, szpassword.param().abi(), pvpara, dwflags).ok() }
}
#[inline]
pub unsafe fn PFXImportCertStore<P1>(ppfx: *const CRYPT_INTEGER_BLOB, szpassword: P1, dwflags: CRYPT_KEY_FLAGS) -> windows_core::Result<HCERTSTORE>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("crypt32.dll" "system" fn PFXImportCertStore(ppfx : *const CRYPT_INTEGER_BLOB, szpassword : windows_core::PCWSTR, dwflags : CRYPT_KEY_FLAGS) -> HCERTSTORE);
    let result__ = unsafe { PFXImportCertStore(ppfx, szpassword.param().abi(), dwflags) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[inline]
pub unsafe fn PFXIsPFXBlob(ppfx: *const CRYPT_INTEGER_BLOB) -> windows_core::BOOL {
    windows_core::link!("crypt32.dll" "system" fn PFXIsPFXBlob(ppfx : *const CRYPT_INTEGER_BLOB) -> windows_core::BOOL);
    unsafe { PFXIsPFXBlob(ppfx) }
}
#[inline]
pub unsafe fn PFXVerifyPassword<P1>(ppfx: *const CRYPT_INTEGER_BLOB, szpassword: P1, dwflags: u32) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("crypt32.dll" "system" fn PFXVerifyPassword(ppfx : *const CRYPT_INTEGER_BLOB, szpassword : windows_core::PCWSTR, dwflags : u32) -> windows_core::BOOL);
    unsafe { PFXVerifyPassword(ppfx, szpassword.param().abi(), dwflags) }
}
#[inline]
pub unsafe fn ProcessPrng(pbdata: &mut [u8]) -> windows_core::BOOL {
    windows_core::link!("bcryptprimitives.dll" "system" fn ProcessPrng(pbdata : *mut u8, cbdata : usize) -> windows_core::BOOL);
    unsafe { ProcessPrng(core::mem::transmute(pbdata.as_ptr()), pbdata.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn SignError() -> windows_core::Result<()> {
    windows_core::link!("mssign32.dll" "system" fn SignError() -> windows_core::HRESULT);
    unsafe { SignError().ok() }
}
#[inline]
pub unsafe fn SignHash<P3>(hcrypto: *const INFORMATIONCARD_CRYPTO_HANDLE, phash: &[u8], hashalgoid: P3, pcbsig: *mut u32, ppsig: *mut *mut u8) -> windows_core::Result<()>
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("infocardapi.dll" "system" fn SignHash(hcrypto : *const INFORMATIONCARD_CRYPTO_HANDLE, cbhash : u32, phash : *const u8, hashalgoid : windows_core::PCWSTR, pcbsig : *mut u32, ppsig : *mut *mut u8) -> windows_core::HRESULT);
    unsafe { SignHash(hcrypto, phash.len().try_into().unwrap(), core::mem::transmute(phash.as_ptr()), hashalgoid.param().abi(), pcbsig as _, ppsig as _).ok() }
}
#[inline]
pub unsafe fn SignerFreeSignerContext(psignercontext: *const SIGNER_CONTEXT) -> windows_core::Result<()> {
    windows_core::link!("mssign32.dll" "system" fn SignerFreeSignerContext(psignercontext : *const SIGNER_CONTEXT) -> windows_core::HRESULT);
    unsafe { SignerFreeSignerContext(psignercontext).ok() }
}
#[inline]
pub unsafe fn SignerSign<P4>(psubjectinfo: *const SIGNER_SUBJECT_INFO, psignercert: *const SIGNER_CERT, psignatureinfo: *const SIGNER_SIGNATURE_INFO, pproviderinfo: Option<*const SIGNER_PROVIDER_INFO>, pwszhttptimestamp: P4, psrequest: Option<*const CRYPT_ATTRIBUTES>, psipdata: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mssign32.dll" "system" fn SignerSign(psubjectinfo : *const SIGNER_SUBJECT_INFO, psignercert : *const SIGNER_CERT, psignatureinfo : *const SIGNER_SIGNATURE_INFO, pproviderinfo : *const SIGNER_PROVIDER_INFO, pwszhttptimestamp : windows_core::PCWSTR, psrequest : *const CRYPT_ATTRIBUTES, psipdata : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { SignerSign(psubjectinfo, psignercert, psignatureinfo, pproviderinfo.unwrap_or(core::mem::zeroed()) as _, pwszhttptimestamp.param().abi(), psrequest.unwrap_or(core::mem::zeroed()) as _, psipdata.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SignerSignEx<P5>(dwflags: SIGNER_SIGN_FLAGS, psubjectinfo: *const SIGNER_SUBJECT_INFO, psignercert: *const SIGNER_CERT, psignatureinfo: *const SIGNER_SIGNATURE_INFO, pproviderinfo: Option<*const SIGNER_PROVIDER_INFO>, pwszhttptimestamp: P5, psrequest: Option<*const CRYPT_ATTRIBUTES>, psipdata: Option<*const core::ffi::c_void>) -> windows_core::Result<*mut SIGNER_CONTEXT>
where
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mssign32.dll" "system" fn SignerSignEx(dwflags : SIGNER_SIGN_FLAGS, psubjectinfo : *const SIGNER_SUBJECT_INFO, psignercert : *const SIGNER_CERT, psignatureinfo : *const SIGNER_SIGNATURE_INFO, pproviderinfo : *const SIGNER_PROVIDER_INFO, pwszhttptimestamp : windows_core::PCWSTR, psrequest : *const CRYPT_ATTRIBUTES, psipdata : *const core::ffi::c_void, ppsignercontext : *mut *mut SIGNER_CONTEXT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SignerSignEx(dwflags, psubjectinfo, psignercert, psignatureinfo, pproviderinfo.unwrap_or(core::mem::zeroed()) as _, pwszhttptimestamp.param().abi(), psrequest.unwrap_or(core::mem::zeroed()) as _, psipdata.unwrap_or(core::mem::zeroed()) as _, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn SignerSignEx2<P6, P7>(dwflags: SIGNER_SIGN_FLAGS, psubjectinfo: *const SIGNER_SUBJECT_INFO, psignercert: *const SIGNER_CERT, psignatureinfo: *const SIGNER_SIGNATURE_INFO, pproviderinfo: Option<*const SIGNER_PROVIDER_INFO>, dwtimestampflags: Option<SIGNER_TIMESTAMP_FLAGS>, psztimestampalgorithmoid: P6, pwszhttptimestamp: P7, psrequest: Option<*const CRYPT_ATTRIBUTES>, psipdata: Option<*const core::ffi::c_void>, ppsignercontext: *mut *mut SIGNER_CONTEXT, pcryptopolicy: Option<*const CERT_STRONG_SIGN_PARA>, preserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P6: windows_core::Param<windows_core::PCSTR>,
    P7: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mssign32.dll" "system" fn SignerSignEx2(dwflags : SIGNER_SIGN_FLAGS, psubjectinfo : *const SIGNER_SUBJECT_INFO, psignercert : *const SIGNER_CERT, psignatureinfo : *const SIGNER_SIGNATURE_INFO, pproviderinfo : *const SIGNER_PROVIDER_INFO, dwtimestampflags : SIGNER_TIMESTAMP_FLAGS, psztimestampalgorithmoid : windows_core::PCSTR, pwszhttptimestamp : windows_core::PCWSTR, psrequest : *const CRYPT_ATTRIBUTES, psipdata : *const core::ffi::c_void, ppsignercontext : *mut *mut SIGNER_CONTEXT, pcryptopolicy : *const CERT_STRONG_SIGN_PARA, preserved : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { SignerSignEx2(dwflags, psubjectinfo, psignercert, psignatureinfo, pproviderinfo.unwrap_or(core::mem::zeroed()) as _, dwtimestampflags.unwrap_or(core::mem::zeroed()) as _, psztimestampalgorithmoid.param().abi(), pwszhttptimestamp.param().abi(), psrequest.unwrap_or(core::mem::zeroed()) as _, psipdata.unwrap_or(core::mem::zeroed()) as _, ppsignercontext as _, pcryptopolicy.unwrap_or(core::mem::zeroed()) as _, preserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SignerSignEx3<P6, P7>(dwflags: SIGNER_SIGN_FLAGS, psubjectinfo: *const SIGNER_SUBJECT_INFO, psignercert: *const SIGNER_CERT, psignatureinfo: *const SIGNER_SIGNATURE_INFO, pproviderinfo: Option<*const SIGNER_PROVIDER_INFO>, dwtimestampflags: Option<SIGNER_TIMESTAMP_FLAGS>, psztimestampalgorithmoid: P6, pwszhttptimestamp: P7, psrequest: Option<*const CRYPT_ATTRIBUTES>, psipdata: Option<*const core::ffi::c_void>, ppsignercontext: *mut *mut SIGNER_CONTEXT, pcryptopolicy: Option<*const CERT_STRONG_SIGN_PARA>, pdigestsigninfo: Option<*const SIGNER_DIGEST_SIGN_INFO>, preserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P6: windows_core::Param<windows_core::PCSTR>,
    P7: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mssign32.dll" "system" fn SignerSignEx3(dwflags : SIGNER_SIGN_FLAGS, psubjectinfo : *const SIGNER_SUBJECT_INFO, psignercert : *const SIGNER_CERT, psignatureinfo : *const SIGNER_SIGNATURE_INFO, pproviderinfo : *const SIGNER_PROVIDER_INFO, dwtimestampflags : SIGNER_TIMESTAMP_FLAGS, psztimestampalgorithmoid : windows_core::PCSTR, pwszhttptimestamp : windows_core::PCWSTR, psrequest : *const CRYPT_ATTRIBUTES, psipdata : *const core::ffi::c_void, ppsignercontext : *mut *mut SIGNER_CONTEXT, pcryptopolicy : *const CERT_STRONG_SIGN_PARA, pdigestsigninfo : *const SIGNER_DIGEST_SIGN_INFO, preserved : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        SignerSignEx3(
            dwflags,
            psubjectinfo,
            psignercert,
            psignatureinfo,
            pproviderinfo.unwrap_or(core::mem::zeroed()) as _,
            dwtimestampflags.unwrap_or(core::mem::zeroed()) as _,
            psztimestampalgorithmoid.param().abi(),
            pwszhttptimestamp.param().abi(),
            psrequest.unwrap_or(core::mem::zeroed()) as _,
            psipdata.unwrap_or(core::mem::zeroed()) as _,
            ppsignercontext as _,
            pcryptopolicy.unwrap_or(core::mem::zeroed()) as _,
            pdigestsigninfo.unwrap_or(core::mem::zeroed()) as _,
            preserved.unwrap_or(core::mem::zeroed()) as _,
        )
        .ok()
    }
}
#[inline]
pub unsafe fn SignerTimeStamp<P1>(psubjectinfo: *const SIGNER_SUBJECT_INFO, pwszhttptimestamp: P1, psrequest: Option<*const CRYPT_ATTRIBUTES>, psipdata: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mssign32.dll" "system" fn SignerTimeStamp(psubjectinfo : *const SIGNER_SUBJECT_INFO, pwszhttptimestamp : windows_core::PCWSTR, psrequest : *const CRYPT_ATTRIBUTES, psipdata : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { SignerTimeStamp(psubjectinfo, pwszhttptimestamp.param().abi(), psrequest.unwrap_or(core::mem::zeroed()) as _, psipdata.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SignerTimeStampEx<P2>(dwflags: Option<u32>, psubjectinfo: *const SIGNER_SUBJECT_INFO, pwszhttptimestamp: P2, psrequest: *const CRYPT_ATTRIBUTES, psipdata: *const core::ffi::c_void) -> windows_core::Result<*mut SIGNER_CONTEXT>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mssign32.dll" "system" fn SignerTimeStampEx(dwflags : u32, psubjectinfo : *const SIGNER_SUBJECT_INFO, pwszhttptimestamp : windows_core::PCWSTR, psrequest : *const CRYPT_ATTRIBUTES, psipdata : *const core::ffi::c_void, ppsignercontext : *mut *mut SIGNER_CONTEXT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SignerTimeStampEx(dwflags.unwrap_or(core::mem::zeroed()) as _, psubjectinfo, pwszhttptimestamp.param().abi(), psrequest, psipdata, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn SignerTimeStampEx2<P2>(dwflags: Option<SIGNER_TIMESTAMP_FLAGS>, psubjectinfo: *const SIGNER_SUBJECT_INFO, pwszhttptimestamp: P2, dwalgid: ALG_ID, psrequest: *const CRYPT_ATTRIBUTES, psipdata: *const core::ffi::c_void) -> windows_core::Result<*mut SIGNER_CONTEXT>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mssign32.dll" "system" fn SignerTimeStampEx2(dwflags : SIGNER_TIMESTAMP_FLAGS, psubjectinfo : *const SIGNER_SUBJECT_INFO, pwszhttptimestamp : windows_core::PCWSTR, dwalgid : ALG_ID, psrequest : *const CRYPT_ATTRIBUTES, psipdata : *const core::ffi::c_void, ppsignercontext : *mut *mut SIGNER_CONTEXT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SignerTimeStampEx2(dwflags.unwrap_or(core::mem::zeroed()) as _, psubjectinfo, pwszhttptimestamp.param().abi(), dwalgid, psrequest, psipdata, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn SignerTimeStampEx3<P3, P4>(dwflags: SIGNER_TIMESTAMP_FLAGS, dwindex: u32, psubjectinfo: *const SIGNER_SUBJECT_INFO, pwszhttptimestamp: P3, pszalgorithmoid: P4, psrequest: Option<*const CRYPT_ATTRIBUTES>, psipdata: Option<*const core::ffi::c_void>, ppsignercontext: *mut *mut SIGNER_CONTEXT, pcryptopolicy: Option<*const CERT_STRONG_SIGN_PARA>, preserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mssign32.dll" "system" fn SignerTimeStampEx3(dwflags : SIGNER_TIMESTAMP_FLAGS, dwindex : u32, psubjectinfo : *const SIGNER_SUBJECT_INFO, pwszhttptimestamp : windows_core::PCWSTR, pszalgorithmoid : windows_core::PCWSTR, psrequest : *const CRYPT_ATTRIBUTES, psipdata : *const core::ffi::c_void, ppsignercontext : *mut *mut SIGNER_CONTEXT, pcryptopolicy : *const CERT_STRONG_SIGN_PARA, preserved : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { SignerTimeStampEx3(dwflags, dwindex, psubjectinfo, pwszhttptimestamp.param().abi(), pszalgorithmoid.param().abi(), psrequest.unwrap_or(core::mem::zeroed()) as _, psipdata.unwrap_or(core::mem::zeroed()) as _, ppsignercontext as _, pcryptopolicy.unwrap_or(core::mem::zeroed()) as _, preserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SslChangeNotify(hevent: super::super::Foundation::HANDLE, dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("ncrypt.dll" "system" fn SslChangeNotify(hevent : super::super::Foundation:: HANDLE, dwflags : u32) -> windows_core::HRESULT);
    unsafe { SslChangeNotify(hevent, dwflags).ok() }
}
#[inline]
pub unsafe fn SslComputeClientAuthHash<P3>(hsslprovider: NCRYPT_PROV_HANDLE, hmasterkey: NCRYPT_KEY_HANDLE, hhandshakehash: NCRYPT_HASH_HANDLE, pszalgid: P3, pboutput: Option<&mut [u8]>, pcbresult: *mut u32, dwflags: u32) -> windows_core::Result<()>
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ncrypt.dll" "system" fn SslComputeClientAuthHash(hsslprovider : NCRYPT_PROV_HANDLE, hmasterkey : NCRYPT_KEY_HANDLE, hhandshakehash : NCRYPT_HASH_HANDLE, pszalgid : windows_core::PCWSTR, pboutput : *mut u8, cboutput : u32, pcbresult : *mut u32, dwflags : u32) -> windows_core::HRESULT);
    unsafe { SslComputeClientAuthHash(hsslprovider, hmasterkey, hhandshakehash, pszalgid.param().abi(), core::mem::transmute(pboutput.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pboutput.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbresult as _, dwflags).ok() }
}
#[inline]
pub unsafe fn SslComputeEapKeyBlock(hsslprovider: NCRYPT_PROV_HANDLE, hmasterkey: NCRYPT_KEY_HANDLE, pbrandoms: &[u8], pboutput: Option<&mut [u8]>, pcbresult: *mut u32, dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("ncrypt.dll" "system" fn SslComputeEapKeyBlock(hsslprovider : NCRYPT_PROV_HANDLE, hmasterkey : NCRYPT_KEY_HANDLE, pbrandoms : *const u8, cbrandoms : u32, pboutput : *mut u8, cboutput : u32, pcbresult : *mut u32, dwflags : u32) -> windows_core::HRESULT);
    unsafe { SslComputeEapKeyBlock(hsslprovider, hmasterkey, core::mem::transmute(pbrandoms.as_ptr()), pbrandoms.len().try_into().unwrap(), core::mem::transmute(pboutput.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pboutput.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbresult as _, dwflags).ok() }
}
#[inline]
pub unsafe fn SslComputeFinishedHash(hsslprovider: NCRYPT_PROV_HANDLE, hmasterkey: NCRYPT_KEY_HANDLE, hhandshakehash: NCRYPT_HASH_HANDLE, pboutput: &mut [u8], dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("ncrypt.dll" "system" fn SslComputeFinishedHash(hsslprovider : NCRYPT_PROV_HANDLE, hmasterkey : NCRYPT_KEY_HANDLE, hhandshakehash : NCRYPT_HASH_HANDLE, pboutput : *mut u8, cboutput : u32, dwflags : u32) -> windows_core::HRESULT);
    unsafe { SslComputeFinishedHash(hsslprovider, hmasterkey, hhandshakehash, core::mem::transmute(pboutput.as_ptr()), pboutput.len().try_into().unwrap(), dwflags).ok() }
}
#[inline]
pub unsafe fn SslComputeSessionHash(hsslprovider: NCRYPT_PROV_HANDLE, hhandshakehash: NCRYPT_HASH_HANDLE, dwprotocol: u32, pboutput: Option<&mut [u8]>, pcbresult: *mut u32, dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("ncrypt.dll" "system" fn SslComputeSessionHash(hsslprovider : NCRYPT_PROV_HANDLE, hhandshakehash : NCRYPT_HASH_HANDLE, dwprotocol : u32, pboutput : *mut u8, cboutput : u32, pcbresult : *mut u32, dwflags : u32) -> windows_core::HRESULT);
    unsafe { SslComputeSessionHash(hsslprovider, hhandshakehash, dwprotocol, core::mem::transmute(pboutput.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pboutput.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbresult as _, dwflags).ok() }
}
#[inline]
pub unsafe fn SslCreateClientAuthHash<P4>(hsslprovider: NCRYPT_PROV_HANDLE, phhandshakehash: *mut NCRYPT_HASH_HANDLE, dwprotocol: u32, dwciphersuite: u32, pszhashalgid: P4, dwflags: u32) -> windows_core::Result<()>
where
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ncrypt.dll" "system" fn SslCreateClientAuthHash(hsslprovider : NCRYPT_PROV_HANDLE, phhandshakehash : *mut NCRYPT_HASH_HANDLE, dwprotocol : u32, dwciphersuite : u32, pszhashalgid : windows_core::PCWSTR, dwflags : u32) -> windows_core::HRESULT);
    unsafe { SslCreateClientAuthHash(hsslprovider, phhandshakehash as _, dwprotocol, dwciphersuite, pszhashalgid.param().abi(), dwflags).ok() }
}
#[inline]
pub unsafe fn SslCreateEphemeralKey(hsslprovider: NCRYPT_PROV_HANDLE, phephemeralkey: *mut NCRYPT_KEY_HANDLE, dwprotocol: u32, dwciphersuite: u32, dwkeytype: u32, dwkeybitlen: u32, pbparams: Option<&[u8]>, dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("ncrypt.dll" "system" fn SslCreateEphemeralKey(hsslprovider : NCRYPT_PROV_HANDLE, phephemeralkey : *mut NCRYPT_KEY_HANDLE, dwprotocol : u32, dwciphersuite : u32, dwkeytype : u32, dwkeybitlen : u32, pbparams : *const u8, cbparams : u32, dwflags : u32) -> windows_core::HRESULT);
    unsafe { SslCreateEphemeralKey(hsslprovider, phephemeralkey as _, dwprotocol, dwciphersuite, dwkeytype, dwkeybitlen, core::mem::transmute(pbparams.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pbparams.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), dwflags).ok() }
}
#[inline]
pub unsafe fn SslCreateHandshakeHash(hsslprovider: NCRYPT_PROV_HANDLE, phhandshakehash: *mut NCRYPT_HASH_HANDLE, dwprotocol: u32, dwciphersuite: u32, dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("ncrypt.dll" "system" fn SslCreateHandshakeHash(hsslprovider : NCRYPT_PROV_HANDLE, phhandshakehash : *mut NCRYPT_HASH_HANDLE, dwprotocol : u32, dwciphersuite : u32, dwflags : u32) -> windows_core::HRESULT);
    unsafe { SslCreateHandshakeHash(hsslprovider, phhandshakehash as _, dwprotocol, dwciphersuite, dwflags).ok() }
}
#[inline]
pub unsafe fn SslDecrementProviderReferenceCount(hsslprovider: NCRYPT_PROV_HANDLE) -> windows_core::Result<()> {
    windows_core::link!("ncrypt.dll" "system" fn SslDecrementProviderReferenceCount(hsslprovider : NCRYPT_PROV_HANDLE) -> windows_core::HRESULT);
    unsafe { SslDecrementProviderReferenceCount(hsslprovider).ok() }
}
#[inline]
pub unsafe fn SslDecryptPacket(hsslprovider: NCRYPT_PROV_HANDLE, hkey: NCRYPT_KEY_HANDLE, pbinput: &[u8], pboutput: Option<&mut [u8]>, pcbresult: *mut u32, sequencenumber: u64, dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("ncrypt.dll" "system" fn SslDecryptPacket(hsslprovider : NCRYPT_PROV_HANDLE, hkey : NCRYPT_KEY_HANDLE, pbinput : *const u8, cbinput : u32, pboutput : *mut u8, cboutput : u32, pcbresult : *mut u32, sequencenumber : u64, dwflags : u32) -> windows_core::HRESULT);
    unsafe { SslDecryptPacket(hsslprovider, hkey, core::mem::transmute(pbinput.as_ptr()), pbinput.len().try_into().unwrap(), core::mem::transmute(pboutput.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pboutput.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbresult as _, sequencenumber, dwflags).ok() }
}
#[inline]
pub unsafe fn SslDuplicateTranscriptHash(hsslprovider: NCRYPT_PROV_HANDLE, htranscripthash: NCRYPT_HASH_HANDLE, phtranscripthash: *mut NCRYPT_HASH_HANDLE, dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("ncrypt.dll" "system" fn SslDuplicateTranscriptHash(hsslprovider : NCRYPT_PROV_HANDLE, htranscripthash : NCRYPT_HASH_HANDLE, phtranscripthash : *mut NCRYPT_HASH_HANDLE, dwflags : u32) -> windows_core::HRESULT);
    unsafe { SslDuplicateTranscriptHash(hsslprovider, htranscripthash, phtranscripthash as _, dwflags).ok() }
}
#[inline]
pub unsafe fn SslEncryptPacket(hsslprovider: NCRYPT_PROV_HANDLE, hkey: NCRYPT_KEY_HANDLE, pbinput: &[u8], pboutput: Option<&mut [u8]>, pcbresult: *mut u32, sequencenumber: u64, dwcontenttype: u32, dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("ncrypt.dll" "system" fn SslEncryptPacket(hsslprovider : NCRYPT_PROV_HANDLE, hkey : NCRYPT_KEY_HANDLE, pbinput : *const u8, cbinput : u32, pboutput : *mut u8, cboutput : u32, pcbresult : *mut u32, sequencenumber : u64, dwcontenttype : u32, dwflags : u32) -> windows_core::HRESULT);
    unsafe { SslEncryptPacket(hsslprovider, hkey, core::mem::transmute(pbinput.as_ptr()), pbinput.len().try_into().unwrap(), core::mem::transmute(pboutput.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pboutput.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbresult as _, sequencenumber, dwcontenttype, dwflags).ok() }
}
#[inline]
pub unsafe fn SslEnumCipherSuites(hsslprovider: NCRYPT_PROV_HANDLE, hprivatekey: Option<NCRYPT_KEY_HANDLE>, ppciphersuite: *mut *mut NCRYPT_SSL_CIPHER_SUITE, ppenumstate: *mut *mut core::ffi::c_void, dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("ncrypt.dll" "system" fn SslEnumCipherSuites(hsslprovider : NCRYPT_PROV_HANDLE, hprivatekey : NCRYPT_KEY_HANDLE, ppciphersuite : *mut *mut NCRYPT_SSL_CIPHER_SUITE, ppenumstate : *mut *mut core::ffi::c_void, dwflags : u32) -> windows_core::HRESULT);
    unsafe { SslEnumCipherSuites(hsslprovider, hprivatekey.unwrap_or(core::mem::zeroed()) as _, ppciphersuite as _, ppenumstate as _, dwflags).ok() }
}
#[inline]
pub unsafe fn SslEnumCipherSuitesEx(hsslprovider: NCRYPT_PROV_HANDLE, hprivatekey: Option<NCRYPT_KEY_HANDLE>, ppciphersuite: *mut *mut NCRYPT_SSL_CIPHER_SUITE_EX, ppenumstate: *mut *mut core::ffi::c_void, dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("ncrypt.dll" "system" fn SslEnumCipherSuitesEx(hsslprovider : NCRYPT_PROV_HANDLE, hprivatekey : NCRYPT_KEY_HANDLE, ppciphersuite : *mut *mut NCRYPT_SSL_CIPHER_SUITE_EX, ppenumstate : *mut *mut core::ffi::c_void, dwflags : u32) -> windows_core::HRESULT);
    unsafe { SslEnumCipherSuitesEx(hsslprovider, hprivatekey.unwrap_or(core::mem::zeroed()) as _, ppciphersuite as _, ppenumstate as _, dwflags).ok() }
}
#[inline]
pub unsafe fn SslEnumEccCurves(hsslprovider: NCRYPT_PROV_HANDLE, pecccurvecount: *mut u32, ppecccurve: *mut *mut NCRYPT_SSL_ECC_CURVE, dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("ncrypt.dll" "system" fn SslEnumEccCurves(hsslprovider : NCRYPT_PROV_HANDLE, pecccurvecount : *mut u32, ppecccurve : *mut *mut NCRYPT_SSL_ECC_CURVE, dwflags : u32) -> windows_core::HRESULT);
    unsafe { SslEnumEccCurves(hsslprovider, pecccurvecount as _, ppecccurve as _, dwflags).ok() }
}
#[inline]
pub unsafe fn SslEnumProtocolProviders(pdwprovidercount: *mut u32, ppproviderlist: *mut *mut NCryptProviderName, dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("ncrypt.dll" "system" fn SslEnumProtocolProviders(pdwprovidercount : *mut u32, ppproviderlist : *mut *mut NCryptProviderName, dwflags : u32) -> windows_core::HRESULT);
    unsafe { SslEnumProtocolProviders(pdwprovidercount as _, ppproviderlist as _, dwflags).ok() }
}
#[inline]
pub unsafe fn SslExpandBinderKey(hsslprovider: NCRYPT_PROV_HANDLE, hearlykey: NCRYPT_KEY_HANDLE, phbinderkey: *mut NCRYPT_KEY_HANDLE, pparameterlist: Option<*const BCryptBufferDesc>, dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("ncrypt.dll" "system" fn SslExpandBinderKey(hsslprovider : NCRYPT_PROV_HANDLE, hearlykey : NCRYPT_KEY_HANDLE, phbinderkey : *mut NCRYPT_KEY_HANDLE, pparameterlist : *const BCryptBufferDesc, dwflags : u32) -> windows_core::HRESULT);
    unsafe { SslExpandBinderKey(hsslprovider, hearlykey, phbinderkey as _, pparameterlist.unwrap_or(core::mem::zeroed()) as _, dwflags).ok() }
}
#[inline]
pub unsafe fn SslExpandExporterMasterKey(hsslprovider: NCRYPT_PROV_HANDLE, hbasekey: NCRYPT_KEY_HANDLE, hhashvalue: NCRYPT_HASH_HANDLE, phexportermasterkey: *mut NCRYPT_KEY_HANDLE, pparameterlist: Option<*const BCryptBufferDesc>, dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("ncrypt.dll" "system" fn SslExpandExporterMasterKey(hsslprovider : NCRYPT_PROV_HANDLE, hbasekey : NCRYPT_KEY_HANDLE, hhashvalue : NCRYPT_HASH_HANDLE, phexportermasterkey : *mut NCRYPT_KEY_HANDLE, pparameterlist : *const BCryptBufferDesc, dwflags : u32) -> windows_core::HRESULT);
    unsafe { SslExpandExporterMasterKey(hsslprovider, hbasekey, hhashvalue, phexportermasterkey as _, pparameterlist.unwrap_or(core::mem::zeroed()) as _, dwflags).ok() }
}
#[inline]
pub unsafe fn SslExpandPreSharedKey(hsslprovider: NCRYPT_PROV_HANDLE, hresumptionmasterkey: NCRYPT_KEY_HANDLE, pbticketnonce: Option<&[u8]>, phpresharedkey: *mut NCRYPT_KEY_HANDLE, pparameterlist: Option<*const BCryptBufferDesc>, dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("ncrypt.dll" "system" fn SslExpandPreSharedKey(hsslprovider : NCRYPT_PROV_HANDLE, hresumptionmasterkey : NCRYPT_KEY_HANDLE, pbticketnonce : *const u8, cbticketnonce : u32, phpresharedkey : *mut NCRYPT_KEY_HANDLE, pparameterlist : *const BCryptBufferDesc, dwflags : u32) -> windows_core::HRESULT);
    unsafe { SslExpandPreSharedKey(hsslprovider, hresumptionmasterkey, core::mem::transmute(pbticketnonce.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pbticketnonce.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), phpresharedkey as _, pparameterlist.unwrap_or(core::mem::zeroed()) as _, dwflags).ok() }
}
#[inline]
pub unsafe fn SslExpandResumptionMasterKey(hsslprovider: NCRYPT_PROV_HANDLE, hmasterkey: NCRYPT_KEY_HANDLE, hhashvalue: NCRYPT_HASH_HANDLE, phresumptionmasterkey: *mut NCRYPT_KEY_HANDLE, pparameterlist: Option<*const BCryptBufferDesc>, dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("ncrypt.dll" "system" fn SslExpandResumptionMasterKey(hsslprovider : NCRYPT_PROV_HANDLE, hmasterkey : NCRYPT_KEY_HANDLE, hhashvalue : NCRYPT_HASH_HANDLE, phresumptionmasterkey : *mut NCRYPT_KEY_HANDLE, pparameterlist : *const BCryptBufferDesc, dwflags : u32) -> windows_core::HRESULT);
    unsafe { SslExpandResumptionMasterKey(hsslprovider, hmasterkey, hhashvalue, phresumptionmasterkey as _, pparameterlist.unwrap_or(core::mem::zeroed()) as _, dwflags).ok() }
}
#[inline]
pub unsafe fn SslExpandTrafficKeys(hsslprovider: NCRYPT_PROV_HANDLE, hbasekey: NCRYPT_KEY_HANDLE, hhashvalue: NCRYPT_HASH_HANDLE, phclienttraffickey: Option<*mut NCRYPT_KEY_HANDLE>, phservertraffickey: Option<*mut NCRYPT_KEY_HANDLE>, pparameterlist: Option<*const BCryptBufferDesc>, dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("ncrypt.dll" "system" fn SslExpandTrafficKeys(hsslprovider : NCRYPT_PROV_HANDLE, hbasekey : NCRYPT_KEY_HANDLE, hhashvalue : NCRYPT_HASH_HANDLE, phclienttraffickey : *mut NCRYPT_KEY_HANDLE, phservertraffickey : *mut NCRYPT_KEY_HANDLE, pparameterlist : *const BCryptBufferDesc, dwflags : u32) -> windows_core::HRESULT);
    unsafe { SslExpandTrafficKeys(hsslprovider, hbasekey, hhashvalue, phclienttraffickey.unwrap_or(core::mem::zeroed()) as _, phservertraffickey.unwrap_or(core::mem::zeroed()) as _, pparameterlist.unwrap_or(core::mem::zeroed()) as _, dwflags).ok() }
}
#[inline]
pub unsafe fn SslExpandWriteKey(hsslprovider: NCRYPT_PROV_HANDLE, hbasetraffickey: NCRYPT_KEY_HANDLE, phwritekey: *mut NCRYPT_KEY_HANDLE, pparameterlist: Option<*const BCryptBufferDesc>, dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("ncrypt.dll" "system" fn SslExpandWriteKey(hsslprovider : NCRYPT_PROV_HANDLE, hbasetraffickey : NCRYPT_KEY_HANDLE, phwritekey : *mut NCRYPT_KEY_HANDLE, pparameterlist : *const BCryptBufferDesc, dwflags : u32) -> windows_core::HRESULT);
    unsafe { SslExpandWriteKey(hsslprovider, hbasetraffickey, phwritekey as _, pparameterlist.unwrap_or(core::mem::zeroed()) as _, dwflags).ok() }
}
#[inline]
pub unsafe fn SslExportKey<P2>(hsslprovider: NCRYPT_PROV_HANDLE, hkey: NCRYPT_KEY_HANDLE, pszblobtype: P2, pboutput: Option<&mut [u8]>, pcbresult: *mut u32, dwflags: u32) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ncrypt.dll" "system" fn SslExportKey(hsslprovider : NCRYPT_PROV_HANDLE, hkey : NCRYPT_KEY_HANDLE, pszblobtype : windows_core::PCWSTR, pboutput : *mut u8, cboutput : u32, pcbresult : *mut u32, dwflags : u32) -> windows_core::HRESULT);
    unsafe { SslExportKey(hsslprovider, hkey, pszblobtype.param().abi(), core::mem::transmute(pboutput.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pboutput.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbresult as _, dwflags).ok() }
}
#[inline]
pub unsafe fn SslExportKeyingMaterial<P2>(hsslprovider: NCRYPT_PROV_HANDLE, hmasterkey: NCRYPT_KEY_HANDLE, slabel: P2, pbrandoms: Option<&[u8]>, pbcontextvalue: Option<&[u8]>, pboutput: &mut [u8], dwflags: u32) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("ncrypt.dll" "system" fn SslExportKeyingMaterial(hsslprovider : NCRYPT_PROV_HANDLE, hmasterkey : NCRYPT_KEY_HANDLE, slabel : windows_core::PCSTR, pbrandoms : *const u8, cbrandoms : u32, pbcontextvalue : *const u8, cbcontextvalue : u16, pboutput : *mut u8, cboutput : u32, dwflags : u32) -> windows_core::HRESULT);
    unsafe { SslExportKeyingMaterial(hsslprovider, hmasterkey, slabel.param().abi(), core::mem::transmute(pbrandoms.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pbrandoms.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pbcontextvalue.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pbcontextvalue.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pboutput.as_ptr()), pboutput.len().try_into().unwrap(), dwflags).ok() }
}
#[inline]
pub unsafe fn SslExtractEarlyKey(hsslprovider: NCRYPT_PROV_HANDLE, hpresharedkey: Option<NCRYPT_KEY_HANDLE>, phearlykey: *mut NCRYPT_KEY_HANDLE, dwprotocol: u32, dwciphersuite: u32, pparameterlist: Option<*const BCryptBufferDesc>, dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("ncrypt.dll" "system" fn SslExtractEarlyKey(hsslprovider : NCRYPT_PROV_HANDLE, hpresharedkey : NCRYPT_KEY_HANDLE, phearlykey : *mut NCRYPT_KEY_HANDLE, dwprotocol : u32, dwciphersuite : u32, pparameterlist : *const BCryptBufferDesc, dwflags : u32) -> windows_core::HRESULT);
    unsafe { SslExtractEarlyKey(hsslprovider, hpresharedkey.unwrap_or(core::mem::zeroed()) as _, phearlykey as _, dwprotocol, dwciphersuite, pparameterlist.unwrap_or(core::mem::zeroed()) as _, dwflags).ok() }
}
#[inline]
pub unsafe fn SslExtractHandshakeKey(hsslprovider: NCRYPT_PROV_HANDLE, hprivatekey: NCRYPT_KEY_HANDLE, hpublickey: NCRYPT_KEY_HANDLE, hearlykey: NCRYPT_KEY_HANDLE, phhandshakekey: *mut NCRYPT_KEY_HANDLE, pparameterlist: Option<*const BCryptBufferDesc>, dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("ncrypt.dll" "system" fn SslExtractHandshakeKey(hsslprovider : NCRYPT_PROV_HANDLE, hprivatekey : NCRYPT_KEY_HANDLE, hpublickey : NCRYPT_KEY_HANDLE, hearlykey : NCRYPT_KEY_HANDLE, phhandshakekey : *mut NCRYPT_KEY_HANDLE, pparameterlist : *const BCryptBufferDesc, dwflags : u32) -> windows_core::HRESULT);
    unsafe { SslExtractHandshakeKey(hsslprovider, hprivatekey, hpublickey, hearlykey, phhandshakekey as _, pparameterlist.unwrap_or(core::mem::zeroed()) as _, dwflags).ok() }
}
#[inline]
pub unsafe fn SslExtractMasterKey(hsslprovider: NCRYPT_PROV_HANDLE, hhandshakekey: NCRYPT_KEY_HANDLE, phmasterkey: *mut NCRYPT_KEY_HANDLE, pparameterlist: Option<*const BCryptBufferDesc>, dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("ncrypt.dll" "system" fn SslExtractMasterKey(hsslprovider : NCRYPT_PROV_HANDLE, hhandshakekey : NCRYPT_KEY_HANDLE, phmasterkey : *mut NCRYPT_KEY_HANDLE, pparameterlist : *const BCryptBufferDesc, dwflags : u32) -> windows_core::HRESULT);
    unsafe { SslExtractMasterKey(hsslprovider, hhandshakekey, phmasterkey as _, pparameterlist.unwrap_or(core::mem::zeroed()) as _, dwflags).ok() }
}
#[inline]
pub unsafe fn SslFreeBuffer(pvinput: *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_core::link!("ncrypt.dll" "system" fn SslFreeBuffer(pvinput : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { SslFreeBuffer(pvinput as _).ok() }
}
#[inline]
pub unsafe fn SslFreeObject(hobject: NCRYPT_HANDLE, dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("ncrypt.dll" "system" fn SslFreeObject(hobject : NCRYPT_HANDLE, dwflags : u32) -> windows_core::HRESULT);
    unsafe { SslFreeObject(hobject, dwflags).ok() }
}
#[inline]
pub unsafe fn SslGenerateMasterKey(hsslprovider: NCRYPT_PROV_HANDLE, hprivatekey: Option<NCRYPT_KEY_HANDLE>, hpublickey: NCRYPT_KEY_HANDLE, phmasterkey: *mut NCRYPT_KEY_HANDLE, dwprotocol: u32, dwciphersuite: u32, pparameterlist: *const BCryptBufferDesc, pboutput: Option<&mut [u8]>, pcbresult: *mut u32, dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("ncrypt.dll" "system" fn SslGenerateMasterKey(hsslprovider : NCRYPT_PROV_HANDLE, hprivatekey : NCRYPT_KEY_HANDLE, hpublickey : NCRYPT_KEY_HANDLE, phmasterkey : *mut NCRYPT_KEY_HANDLE, dwprotocol : u32, dwciphersuite : u32, pparameterlist : *const BCryptBufferDesc, pboutput : *mut u8, cboutput : u32, pcbresult : *mut u32, dwflags : u32) -> windows_core::HRESULT);
    unsafe { SslGenerateMasterKey(hsslprovider, hprivatekey.unwrap_or(core::mem::zeroed()) as _, hpublickey, phmasterkey as _, dwprotocol, dwciphersuite, pparameterlist, core::mem::transmute(pboutput.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pboutput.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbresult as _, dwflags).ok() }
}
#[inline]
pub unsafe fn SslGeneratePreMasterKey(hsslprovider: NCRYPT_PROV_HANDLE, hpublickey: NCRYPT_KEY_HANDLE, phpremasterkey: *mut NCRYPT_KEY_HANDLE, dwprotocol: u32, dwciphersuite: u32, pparameterlist: *const BCryptBufferDesc, pboutput: Option<&mut [u8]>, pcbresult: *mut u32, dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("ncrypt.dll" "system" fn SslGeneratePreMasterKey(hsslprovider : NCRYPT_PROV_HANDLE, hpublickey : NCRYPT_KEY_HANDLE, phpremasterkey : *mut NCRYPT_KEY_HANDLE, dwprotocol : u32, dwciphersuite : u32, pparameterlist : *const BCryptBufferDesc, pboutput : *mut u8, cboutput : u32, pcbresult : *mut u32, dwflags : u32) -> windows_core::HRESULT);
    unsafe { SslGeneratePreMasterKey(hsslprovider, hpublickey, phpremasterkey as _, dwprotocol, dwciphersuite, pparameterlist, core::mem::transmute(pboutput.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pboutput.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbresult as _, dwflags).ok() }
}
#[inline]
pub unsafe fn SslGenerateSessionKeys(hsslprovider: NCRYPT_PROV_HANDLE, hmasterkey: NCRYPT_KEY_HANDLE, phreadkey: *mut NCRYPT_KEY_HANDLE, phwritekey: *mut NCRYPT_KEY_HANDLE, pparameterlist: *const BCryptBufferDesc, dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("ncrypt.dll" "system" fn SslGenerateSessionKeys(hsslprovider : NCRYPT_PROV_HANDLE, hmasterkey : NCRYPT_KEY_HANDLE, phreadkey : *mut NCRYPT_KEY_HANDLE, phwritekey : *mut NCRYPT_KEY_HANDLE, pparameterlist : *const BCryptBufferDesc, dwflags : u32) -> windows_core::HRESULT);
    unsafe { SslGenerateSessionKeys(hsslprovider, hmasterkey, phreadkey as _, phwritekey as _, pparameterlist, dwflags).ok() }
}
#[inline]
pub unsafe fn SslGetCipherSuitePRFHashAlgorithm(hsslprovider: NCRYPT_PROV_HANDLE, dwprotocol: u32, dwciphersuite: u32, dwkeytype: u32, szprfhash: &mut [u16; 64], dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("ncrypt.dll" "system" fn SslGetCipherSuitePRFHashAlgorithm(hsslprovider : NCRYPT_PROV_HANDLE, dwprotocol : u32, dwciphersuite : u32, dwkeytype : u32, szprfhash : windows_core::PWSTR, dwflags : u32) -> windows_core::HRESULT);
    unsafe { SslGetCipherSuitePRFHashAlgorithm(hsslprovider, dwprotocol, dwciphersuite, dwkeytype, core::mem::transmute(szprfhash.as_ptr()), dwflags).ok() }
}
#[inline]
pub unsafe fn SslGetKeyProperty<P1>(hkey: NCRYPT_KEY_HANDLE, pszproperty: P1, ppboutput: *mut *mut u8, pcboutput: *mut u32, dwflags: u32) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ncrypt.dll" "system" fn SslGetKeyProperty(hkey : NCRYPT_KEY_HANDLE, pszproperty : windows_core::PCWSTR, ppboutput : *mut *mut u8, pcboutput : *mut u32, dwflags : u32) -> windows_core::HRESULT);
    unsafe { SslGetKeyProperty(hkey, pszproperty.param().abi(), ppboutput as _, pcboutput as _, dwflags).ok() }
}
#[inline]
pub unsafe fn SslGetProviderProperty<P1>(hsslprovider: NCRYPT_PROV_HANDLE, pszproperty: P1, ppboutput: *mut *mut u8, pcboutput: *mut u32, ppenumstate: Option<*mut *mut core::ffi::c_void>, dwflags: u32) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ncrypt.dll" "system" fn SslGetProviderProperty(hsslprovider : NCRYPT_PROV_HANDLE, pszproperty : windows_core::PCWSTR, ppboutput : *mut *mut u8, pcboutput : *mut u32, ppenumstate : *mut *mut core::ffi::c_void, dwflags : u32) -> windows_core::HRESULT);
    unsafe { SslGetProviderProperty(hsslprovider, pszproperty.param().abi(), ppboutput as _, pcboutput as _, ppenumstate.unwrap_or(core::mem::zeroed()) as _, dwflags).ok() }
}
#[inline]
pub unsafe fn SslHashHandshake(hsslprovider: NCRYPT_PROV_HANDLE, hhandshakehash: NCRYPT_HASH_HANDLE, pbinput: &[u8], dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("ncrypt.dll" "system" fn SslHashHandshake(hsslprovider : NCRYPT_PROV_HANDLE, hhandshakehash : NCRYPT_HASH_HANDLE, pbinput : *const u8, cbinput : u32, dwflags : u32) -> windows_core::HRESULT);
    unsafe { SslHashHandshake(hsslprovider, hhandshakehash, core::mem::transmute(pbinput.as_ptr()), pbinput.len().try_into().unwrap(), dwflags).ok() }
}
#[inline]
pub unsafe fn SslImportKey<P2>(hsslprovider: NCRYPT_PROV_HANDLE, phkey: *mut NCRYPT_KEY_HANDLE, pszblobtype: P2, pbkeyblob: &[u8], dwflags: u32) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ncrypt.dll" "system" fn SslImportKey(hsslprovider : NCRYPT_PROV_HANDLE, phkey : *mut NCRYPT_KEY_HANDLE, pszblobtype : windows_core::PCWSTR, pbkeyblob : *const u8, cbkeyblob : u32, dwflags : u32) -> windows_core::HRESULT);
    unsafe { SslImportKey(hsslprovider, phkey as _, pszblobtype.param().abi(), core::mem::transmute(pbkeyblob.as_ptr()), pbkeyblob.len().try_into().unwrap(), dwflags).ok() }
}
#[inline]
pub unsafe fn SslImportMasterKey(hsslprovider: NCRYPT_PROV_HANDLE, hprivatekey: NCRYPT_KEY_HANDLE, phmasterkey: *mut NCRYPT_KEY_HANDLE, dwprotocol: u32, dwciphersuite: u32, pparameterlist: *const BCryptBufferDesc, pbencryptedkey: &[u8], dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("ncrypt.dll" "system" fn SslImportMasterKey(hsslprovider : NCRYPT_PROV_HANDLE, hprivatekey : NCRYPT_KEY_HANDLE, phmasterkey : *mut NCRYPT_KEY_HANDLE, dwprotocol : u32, dwciphersuite : u32, pparameterlist : *const BCryptBufferDesc, pbencryptedkey : *const u8, cbencryptedkey : u32, dwflags : u32) -> windows_core::HRESULT);
    unsafe { SslImportMasterKey(hsslprovider, hprivatekey, phmasterkey as _, dwprotocol, dwciphersuite, pparameterlist, core::mem::transmute(pbencryptedkey.as_ptr()), pbencryptedkey.len().try_into().unwrap(), dwflags).ok() }
}
#[inline]
pub unsafe fn SslIncrementProviderReferenceCount(hsslprovider: NCRYPT_PROV_HANDLE) -> windows_core::Result<()> {
    windows_core::link!("ncrypt.dll" "system" fn SslIncrementProviderReferenceCount(hsslprovider : NCRYPT_PROV_HANDLE) -> windows_core::HRESULT);
    unsafe { SslIncrementProviderReferenceCount(hsslprovider).ok() }
}
#[inline]
pub unsafe fn SslLookupCipherLengths(hsslprovider: NCRYPT_PROV_HANDLE, dwprotocol: u32, dwciphersuite: u32, dwkeytype: u32, pcipherlengths: *mut NCRYPT_SSL_CIPHER_LENGTHS, cbcipherlengths: u32, dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("ncrypt.dll" "system" fn SslLookupCipherLengths(hsslprovider : NCRYPT_PROV_HANDLE, dwprotocol : u32, dwciphersuite : u32, dwkeytype : u32, pcipherlengths : *mut NCRYPT_SSL_CIPHER_LENGTHS, cbcipherlengths : u32, dwflags : u32) -> windows_core::HRESULT);
    unsafe { SslLookupCipherLengths(hsslprovider, dwprotocol, dwciphersuite, dwkeytype, pcipherlengths as _, cbcipherlengths, dwflags).ok() }
}
#[inline]
pub unsafe fn SslLookupCipherSuiteInfo(hsslprovider: NCRYPT_PROV_HANDLE, dwprotocol: u32, dwciphersuite: u32, dwkeytype: u32, pciphersuite: *mut NCRYPT_SSL_CIPHER_SUITE, dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("ncrypt.dll" "system" fn SslLookupCipherSuiteInfo(hsslprovider : NCRYPT_PROV_HANDLE, dwprotocol : u32, dwciphersuite : u32, dwkeytype : u32, pciphersuite : *mut NCRYPT_SSL_CIPHER_SUITE, dwflags : u32) -> windows_core::HRESULT);
    unsafe { SslLookupCipherSuiteInfo(hsslprovider, dwprotocol, dwciphersuite, dwkeytype, pciphersuite as _, dwflags).ok() }
}
#[inline]
pub unsafe fn SslOpenPrivateKey(hsslprovider: NCRYPT_PROV_HANDLE, phprivatekey: *mut NCRYPT_KEY_HANDLE, pcertcontext: *const CERT_CONTEXT, dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("ncrypt.dll" "system" fn SslOpenPrivateKey(hsslprovider : NCRYPT_PROV_HANDLE, phprivatekey : *mut NCRYPT_KEY_HANDLE, pcertcontext : *const CERT_CONTEXT, dwflags : u32) -> windows_core::HRESULT);
    unsafe { SslOpenPrivateKey(hsslprovider, phprivatekey as _, pcertcontext, dwflags).ok() }
}
#[inline]
pub unsafe fn SslOpenProvider<P1>(phsslprovider: *mut NCRYPT_PROV_HANDLE, pszprovidername: P1, dwflags: u32) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ncrypt.dll" "system" fn SslOpenProvider(phsslprovider : *mut NCRYPT_PROV_HANDLE, pszprovidername : windows_core::PCWSTR, dwflags : u32) -> windows_core::HRESULT);
    unsafe { SslOpenProvider(phsslprovider as _, pszprovidername.param().abi(), dwflags).ok() }
}
#[inline]
pub unsafe fn SslSignHash(hsslprovider: NCRYPT_PROV_HANDLE, hprivatekey: NCRYPT_KEY_HANDLE, pbhashvalue: &[u8], pbsignature: Option<&mut [u8]>, pcbresult: *mut u32, dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("ncrypt.dll" "system" fn SslSignHash(hsslprovider : NCRYPT_PROV_HANDLE, hprivatekey : NCRYPT_KEY_HANDLE, pbhashvalue : *const u8, cbhashvalue : u32, pbsignature : *mut u8, cbsignature : u32, pcbresult : *mut u32, dwflags : u32) -> windows_core::HRESULT);
    unsafe { SslSignHash(hsslprovider, hprivatekey, core::mem::transmute(pbhashvalue.as_ptr()), pbhashvalue.len().try_into().unwrap(), core::mem::transmute(pbsignature.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pbsignature.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbresult as _, dwflags).ok() }
}
#[inline]
pub unsafe fn SslVerifySignature(hsslprovider: NCRYPT_PROV_HANDLE, hpublickey: NCRYPT_KEY_HANDLE, pbhashvalue: &[u8], pbsignature: &[u8], dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("ncrypt.dll" "system" fn SslVerifySignature(hsslprovider : NCRYPT_PROV_HANDLE, hpublickey : NCRYPT_KEY_HANDLE, pbhashvalue : *const u8, cbhashvalue : u32, pbsignature : *const u8, cbsignature : u32, dwflags : u32) -> windows_core::HRESULT);
    unsafe { SslVerifySignature(hsslprovider, hpublickey, core::mem::transmute(pbhashvalue.as_ptr()), pbhashvalue.len().try_into().unwrap(), core::mem::transmute(pbsignature.as_ptr()), pbsignature.len().try_into().unwrap(), dwflags).ok() }
}
#[inline]
pub unsafe fn SystemPrng(pbrandomdata: &mut [u8]) -> windows_core::BOOL {
    windows_core::link!("bcryptprimitives.dll" "system" fn SystemPrng(pbrandomdata : *mut u8, cbrandomdata : usize) -> windows_core::BOOL);
    unsafe { SystemPrng(core::mem::transmute(pbrandomdata.as_ptr()), pbrandomdata.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn TransformBlock(hcrypto: *const INFORMATIONCARD_CRYPTO_HANDLE, pindata: &[u8], pcboutdata: *mut u32, ppoutdata: *mut *mut u8) -> windows_core::Result<()> {
    windows_core::link!("infocardapi.dll" "system" fn TransformBlock(hcrypto : *const INFORMATIONCARD_CRYPTO_HANDLE, cbindata : u32, pindata : *const u8, pcboutdata : *mut u32, ppoutdata : *mut *mut u8) -> windows_core::HRESULT);
    unsafe { TransformBlock(hcrypto, pindata.len().try_into().unwrap(), core::mem::transmute(pindata.as_ptr()), pcboutdata as _, ppoutdata as _).ok() }
}
#[inline]
pub unsafe fn TransformFinalBlock(hcrypto: *const INFORMATIONCARD_CRYPTO_HANDLE, pindata: &[u8], pcboutdata: *mut u32, ppoutdata: *mut *mut u8) -> windows_core::Result<()> {
    windows_core::link!("infocardapi.dll" "system" fn TransformFinalBlock(hcrypto : *const INFORMATIONCARD_CRYPTO_HANDLE, cbindata : u32, pindata : *const u8, pcboutdata : *mut u32, ppoutdata : *mut *mut u8) -> windows_core::HRESULT);
    unsafe { TransformFinalBlock(hcrypto, pindata.len().try_into().unwrap(), core::mem::transmute(pindata.as_ptr()), pcboutdata as _, ppoutdata as _).ok() }
}
#[inline]
pub unsafe fn VerifyHash<P3>(hcrypto: *const INFORMATIONCARD_CRYPTO_HANDLE, phash: &[u8], hashalgoid: P3, psig: &[u8]) -> windows_core::Result<windows_core::BOOL>
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("infocardapi.dll" "system" fn VerifyHash(hcrypto : *const INFORMATIONCARD_CRYPTO_HANDLE, cbhash : u32, phash : *const u8, hashalgoid : windows_core::PCWSTR, cbsig : u32, psig : *const u8, pfverified : *mut windows_core::BOOL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VerifyHash(hcrypto, phash.len().try_into().unwrap(), core::mem::transmute(phash.as_ptr()), hashalgoid.param().abi(), psig.len().try_into().unwrap(), core::mem::transmute(psig.as_ptr()), &mut result__).map(|| result__)
    }
}
pub const ALG_CLASS_ALL: u32 = 57344u32;
pub const ALG_CLASS_ANY: u32 = 0u32;
pub const ALG_CLASS_DATA_ENCRYPT: u32 = 24576u32;
pub const ALG_CLASS_HASH: u32 = 32768u32;
pub const ALG_CLASS_KEY_EXCHANGE: u32 = 40960u32;
pub const ALG_CLASS_MSG_ENCRYPT: u32 = 16384u32;
pub const ALG_CLASS_SIGNATURE: u32 = 8192u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ALG_ID(pub u32);
pub const ALG_SID_3DES: u32 = 3u32;
pub const ALG_SID_3DES_112: u32 = 9u32;
pub const ALG_SID_AES: u32 = 17u32;
pub const ALG_SID_AES_128: u32 = 14u32;
pub const ALG_SID_AES_192: u32 = 15u32;
pub const ALG_SID_AES_256: u32 = 16u32;
pub const ALG_SID_AGREED_KEY_ANY: u32 = 3u32;
pub const ALG_SID_ANY: u32 = 0u32;
pub const ALG_SID_CAST: u32 = 6u32;
pub const ALG_SID_CYLINK_MEK: u32 = 12u32;
pub const ALG_SID_DES: u32 = 1u32;
pub const ALG_SID_DESX: u32 = 4u32;
pub const ALG_SID_DH_EPHEM: u32 = 2u32;
pub const ALG_SID_DH_SANDF: u32 = 1u32;
pub const ALG_SID_DSS_ANY: u32 = 0u32;
pub const ALG_SID_DSS_DMS: u32 = 2u32;
pub const ALG_SID_DSS_PKCS: u32 = 1u32;
pub const ALG_SID_ECDH: u32 = 5u32;
pub const ALG_SID_ECDH_EPHEM: u32 = 6u32;
pub const ALG_SID_ECDSA: u32 = 3u32;
pub const ALG_SID_ECMQV: u32 = 1u32;
pub const ALG_SID_EXAMPLE: u32 = 80u32;
pub const ALG_SID_HASH_REPLACE_OWF: u32 = 11u32;
pub const ALG_SID_HMAC: u32 = 9u32;
pub const ALG_SID_IDEA: u32 = 5u32;
pub const ALG_SID_KEA: u32 = 4u32;
pub const ALG_SID_MAC: u32 = 5u32;
pub const ALG_SID_MD2: u32 = 1u32;
pub const ALG_SID_MD4: u32 = 2u32;
pub const ALG_SID_MD5: u32 = 3u32;
pub const ALG_SID_PCT1_MASTER: u32 = 4u32;
pub const ALG_SID_RC2: u32 = 2u32;
pub const ALG_SID_RC4: u32 = 1u32;
pub const ALG_SID_RC5: u32 = 13u32;
pub const ALG_SID_RIPEMD: u32 = 6u32;
pub const ALG_SID_RIPEMD160: u32 = 7u32;
pub const ALG_SID_RSA_ANY: u32 = 0u32;
pub const ALG_SID_RSA_ENTRUST: u32 = 3u32;
pub const ALG_SID_RSA_MSATWORK: u32 = 2u32;
pub const ALG_SID_RSA_PGP: u32 = 4u32;
pub const ALG_SID_RSA_PKCS: u32 = 1u32;
pub const ALG_SID_SAFERSK128: u32 = 8u32;
pub const ALG_SID_SAFERSK64: u32 = 7u32;
pub const ALG_SID_SCHANNEL_ENC_KEY: u32 = 7u32;
pub const ALG_SID_SCHANNEL_MAC_KEY: u32 = 3u32;
pub const ALG_SID_SCHANNEL_MASTER_HASH: u32 = 2u32;
pub const ALG_SID_SEAL: u32 = 2u32;
pub const ALG_SID_SHA: u32 = 4u32;
pub const ALG_SID_SHA1: u32 = 4u32;
pub const ALG_SID_SHA_256: u32 = 12u32;
pub const ALG_SID_SHA_384: u32 = 13u32;
pub const ALG_SID_SHA_512: u32 = 14u32;
pub const ALG_SID_SKIPJACK: u32 = 10u32;
pub const ALG_SID_SSL2_MASTER: u32 = 5u32;
pub const ALG_SID_SSL3SHAMD5: u32 = 8u32;
pub const ALG_SID_SSL3_MASTER: u32 = 1u32;
pub const ALG_SID_TEK: u32 = 11u32;
pub const ALG_SID_THIRDPARTY_ANY: u32 = 0u32;
pub const ALG_SID_TLS1PRF: u32 = 10u32;
pub const ALG_SID_TLS1_MASTER: u32 = 6u32;
pub const ALG_TYPE_ANY: u32 = 0u32;
pub const ALG_TYPE_BLOCK: u32 = 1536u32;
pub const ALG_TYPE_DH: u32 = 2560u32;
pub const ALG_TYPE_DSS: u32 = 512u32;
pub const ALG_TYPE_ECDH: u32 = 3584u32;
pub const ALG_TYPE_RSA: u32 = 1024u32;
pub const ALG_TYPE_SECURECHANNEL: u32 = 3072u32;
pub const ALG_TYPE_STREAM: u32 = 2048u32;
pub const ALG_TYPE_THIRDPARTY: u32 = 4096u32;
pub const AT_ECDHE_P256: u32 = 6u32;
pub const AT_ECDHE_P384: u32 = 7u32;
pub const AT_ECDHE_P521: u32 = 8u32;
pub const AT_ECDSA_P256: u32 = 3u32;
pub const AT_ECDSA_P384: u32 = 4u32;
pub const AT_ECDSA_P521: u32 = 5u32;
pub const AT_KEYEXCHANGE: CERT_KEY_SPEC = CERT_KEY_SPEC(1u32);
pub const AT_SIGNATURE: CERT_KEY_SPEC = CERT_KEY_SPEC(2u32);
pub const AUDIT_CARD_DELETE: windows_core::HRESULT = windows_core::HRESULT(0x40050201_u32 as _);
pub const AUDIT_CARD_IMPORT: windows_core::HRESULT = windows_core::HRESULT(0x40050202_u32 as _);
pub const AUDIT_CARD_WRITTEN: windows_core::HRESULT = windows_core::HRESULT(0x40050200_u32 as _);
pub const AUDIT_SERVICE_IDLE_STOP: windows_core::HRESULT = windows_core::HRESULT(0x40050206_u32 as _);
pub const AUDIT_STORE_DELETE: windows_core::HRESULT = windows_core::HRESULT(0x40050205_u32 as _);
pub const AUDIT_STORE_EXPORT: windows_core::HRESULT = windows_core::HRESULT(0x40050204_u32 as _);
pub const AUDIT_STORE_IMPORT: windows_core::HRESULT = windows_core::HRESULT(0x40050203_u32 as _);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AUTHENTICODE_EXTRA_CERT_CHAIN_POLICY_PARA {
    pub cbSize: u32,
    pub dwRegPolicySettings: u32,
    pub pSignerInfo: *mut CMSG_SIGNER_INFO,
}
impl Default for AUTHENTICODE_EXTRA_CERT_CHAIN_POLICY_PARA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AUTHENTICODE_EXTRA_CERT_CHAIN_POLICY_STATUS {
    pub cbSize: u32,
    pub fCommercial: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AUTHENTICODE_TS_EXTRA_CERT_CHAIN_POLICY_PARA {
    pub cbSize: u32,
    pub dwRegPolicySettings: u32,
    pub fCommercial: windows_core::BOOL,
}
pub const AUTHTYPE_CLIENT: HTTPSPOLICY_CALLBACK_DATA_AUTH_TYPE = HTTPSPOLICY_CALLBACK_DATA_AUTH_TYPE(1u32);
pub const AUTHTYPE_SERVER: HTTPSPOLICY_CALLBACK_DATA_AUTH_TYPE = HTTPSPOLICY_CALLBACK_DATA_AUTH_TYPE(2u32);
pub const AdminCreateDeleteDirAc: CARD_DIRECTORY_ACCESS_CONDITION = CARD_DIRECTORY_ACCESS_CONDITION(2i32);
pub const AdminReadWriteAc: CARD_FILE_ACCESS_CONDITION = CARD_FILE_ACCESS_CONDITION(6i32);
pub const AdministratorPin: SECRET_PURPOSE = SECRET_PURPOSE(4i32);
pub const AlphaNumericPinType: SECRET_TYPE = SECRET_TYPE(0i32);
pub const AuthenticationPin: SECRET_PURPOSE = SECRET_PURPOSE(0i32);
pub const BASIC_CONSTRAINTS_CERT_CHAIN_POLICY_CA_FLAG: CERT_CHAIN_POLICY_FLAGS = CERT_CHAIN_POLICY_FLAGS(2147483648u32);
pub const BASIC_CONSTRAINTS_CERT_CHAIN_POLICY_END_ENTITY_FLAG: CERT_CHAIN_POLICY_FLAGS = CERT_CHAIN_POLICY_FLAGS(1073741824u32);
pub const BCRYPTBUFFER_VERSION: u32 = 0u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BCRYPTGENRANDOM_FLAGS(pub u32);
pub const BCRYPT_3DES_112_ALGORITHM: windows_core::PCWSTR = windows_core::w!("3DES_112");
pub const BCRYPT_3DES_112_CBC_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(369u32 as _);
pub const BCRYPT_3DES_112_CFB_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(401u32 as _);
pub const BCRYPT_3DES_112_ECB_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(385u32 as _);
pub const BCRYPT_3DES_ALGORITHM: windows_core::PCWSTR = windows_core::w!("3DES");
pub const BCRYPT_3DES_CBC_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(321u32 as _);
pub const BCRYPT_3DES_CFB_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(353u32 as _);
pub const BCRYPT_3DES_ECB_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(337u32 as _);
pub const BCRYPT_AES_ALGORITHM: windows_core::PCWSTR = windows_core::w!("AES");
pub const BCRYPT_AES_CBC_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(417u32 as _);
pub const BCRYPT_AES_CCM_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(465u32 as _);
pub const BCRYPT_AES_CFB_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(449u32 as _);
pub const BCRYPT_AES_CMAC_ALGORITHM: windows_core::PCWSTR = windows_core::w!("AES-CMAC");
pub const BCRYPT_AES_CMAC_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(257u32 as _);
pub const BCRYPT_AES_ECB_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(433u32 as _);
pub const BCRYPT_AES_GCM_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(481u32 as _);
pub const BCRYPT_AES_GMAC_ALGORITHM: windows_core::PCWSTR = windows_core::w!("AES-GMAC");
pub const BCRYPT_AES_GMAC_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(273u32 as _);
pub const BCRYPT_AES_WRAP_KEY_BLOB: windows_core::PCWSTR = windows_core::w!("Rfc3565KeyWrapBlob");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BCRYPT_ALGORITHM_IDENTIFIER {
    pub pszName: windows_core::PWSTR,
    pub dwClass: u32,
    pub dwFlags: u32,
}
pub const BCRYPT_ALGORITHM_NAME: windows_core::PCWSTR = windows_core::w!("AlgorithmName");
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BCRYPT_ALG_HANDLE(pub *mut core::ffi::c_void);
impl BCRYPT_ALG_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl windows_core::Free for BCRYPT_ALG_HANDLE {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_core::link!("bcrypt.dll" "system" fn BCryptCloseAlgorithmProvider(halgorithm : *mut core::ffi::c_void, dwflags : u32) -> i32);
            unsafe {
                BCryptCloseAlgorithmProvider(self.0, 0);
            }
        }
    }
}
impl Default for BCRYPT_ALG_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::imp::CanInto<BCRYPT_HANDLE> for BCRYPT_ALG_HANDLE {}
impl From<BCRYPT_ALG_HANDLE> for BCRYPT_HANDLE {
    fn from(value: BCRYPT_ALG_HANDLE) -> Self {
        Self(value.0)
    }
}
pub const BCRYPT_ALG_HANDLE_HMAC_FLAG: BCRYPT_OPEN_ALGORITHM_PROVIDER_FLAGS = BCRYPT_OPEN_ALGORITHM_PROVIDER_FLAGS(8u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct BCRYPT_ASYMMETRIC_ENCRYPTION_FUNCTION_TABLE {
    pub Version: BCRYPT_INTERFACE_VERSION,
    pub OpenAlgorithmProvider: BCryptOpenAlgorithmProviderFn,
    pub GetProperty: BCryptGetPropertyFn,
    pub SetProperty: BCryptSetPropertyFn,
    pub CloseAlgorithmProvider: BCryptCloseAlgorithmProviderFn,
    pub GenerateKeyPair: BCryptGenerateKeyPairFn,
    pub FinalizeKeyPair: BCryptFinalizeKeyPairFn,
    pub Encrypt: BCryptEncryptFn,
    pub Decrypt: BCryptDecryptFn,
    pub ImportKeyPair: BCryptImportKeyPairFn,
    pub ExportKey: BCryptExportKeyFn,
    pub DestroyKey: BCryptDestroyKeyFn,
    pub SignHash: BCryptSignHashFn,
    pub VerifySignature: BCryptVerifySignatureFn,
}
pub const BCRYPT_ASYMMETRIC_ENCRYPTION_INTERFACE: BCRYPT_INTERFACE = BCRYPT_INTERFACE(3u32);
pub const BCRYPT_ASYMMETRIC_ENCRYPTION_OPERATION: BCRYPT_OPERATION = BCRYPT_OPERATION(4u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BCRYPT_AUTHENTICATED_CIPHER_MODE_INFO {
    pub cbSize: u32,
    pub dwInfoVersion: u32,
    pub pbNonce: *mut u8,
    pub cbNonce: u32,
    pub pbAuthData: *mut u8,
    pub cbAuthData: u32,
    pub pbTag: *mut u8,
    pub cbTag: u32,
    pub pbMacContext: *mut u8,
    pub cbMacContext: u32,
    pub cbAAD: u32,
    pub cbData: u64,
    pub dwFlags: u32,
}
impl Default for BCRYPT_AUTHENTICATED_CIPHER_MODE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const BCRYPT_AUTHENTICATED_CIPHER_MODE_INFO_VERSION: u32 = 1u32;
pub const BCRYPT_AUTH_MODE_CHAIN_CALLS_FLAG: u32 = 1u32;
pub const BCRYPT_AUTH_MODE_IN_PROGRESS_FLAG: u32 = 2u32;
pub const BCRYPT_AUTH_TAG_LENGTH: windows_core::PCWSTR = windows_core::w!("AuthTagLength");
pub const BCRYPT_BLOCK_LENGTH: windows_core::PCWSTR = windows_core::w!("BlockLength");
pub const BCRYPT_BLOCK_PADDING: BCRYPT_FLAGS = BCRYPT_FLAGS(1u32);
pub const BCRYPT_BLOCK_SIZE_LIST: windows_core::PCWSTR = windows_core::w!("BlockSizeList");
pub const BCRYPT_BUFFERS_LOCKED_FLAG: u32 = 64u32;
pub const BCRYPT_CAPI_AES_FLAG: u32 = 16u32;
pub const BCRYPT_CAPI_KDF_ALGORITHM: windows_core::PCWSTR = windows_core::w!("CAPI_KDF");
pub const BCRYPT_CAPI_KDF_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(801u32 as _);
pub const BCRYPT_CHACHA20_POLY1305_ALGORITHM: windows_core::PCWSTR = windows_core::w!("CHACHA20_POLY1305");
pub const BCRYPT_CHACHA20_POLY1305_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(929u32 as _);
pub const BCRYPT_CHAINING_MODE: windows_core::PCWSTR = windows_core::w!("ChainingMode");
pub const BCRYPT_CHAIN_MODE_CBC: windows_core::PCWSTR = windows_core::w!("ChainingModeCBC");
pub const BCRYPT_CHAIN_MODE_CCM: windows_core::PCWSTR = windows_core::w!("ChainingModeCCM");
pub const BCRYPT_CHAIN_MODE_CFB: windows_core::PCWSTR = windows_core::w!("ChainingModeCFB");
pub const BCRYPT_CHAIN_MODE_ECB: windows_core::PCWSTR = windows_core::w!("ChainingModeECB");
pub const BCRYPT_CHAIN_MODE_GCM: windows_core::PCWSTR = windows_core::w!("ChainingModeGCM");
pub const BCRYPT_CHAIN_MODE_NA: windows_core::PCWSTR = windows_core::w!("ChainingModeN/A");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct BCRYPT_CIPHER_FUNCTION_TABLE {
    pub Version: BCRYPT_INTERFACE_VERSION,
    pub OpenAlgorithmProvider: BCryptOpenAlgorithmProviderFn,
    pub GetProperty: BCryptGetPropertyFn,
    pub SetProperty: BCryptSetPropertyFn,
    pub CloseAlgorithmProvider: BCryptCloseAlgorithmProviderFn,
    pub GenerateKey: BCryptGenerateSymmetricKeyFn,
    pub Encrypt: BCryptEncryptFn,
    pub Decrypt: BCryptDecryptFn,
    pub ImportKey: BCryptImportKeyFn,
    pub ExportKey: BCryptExportKeyFn,
    pub DuplicateKey: BCryptDuplicateKeyFn,
    pub DestroyKey: BCryptDestroyKeyFn,
}
pub const BCRYPT_CIPHER_INTERFACE: BCRYPT_INTERFACE = BCRYPT_INTERFACE(1u32);
pub const BCRYPT_CIPHER_OPERATION: BCRYPT_OPERATION = BCRYPT_OPERATION(1u32);
pub const BCRYPT_COPY_AFTER_PADDING_CHECK_FAILURE_FLAG: u32 = 256u32;
pub const BCRYPT_DESX_ALGORITHM: windows_core::PCWSTR = windows_core::w!("DESX");
pub const BCRYPT_DESX_CBC_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(545u32 as _);
pub const BCRYPT_DESX_CFB_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(577u32 as _);
pub const BCRYPT_DESX_ECB_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(561u32 as _);
pub const BCRYPT_DES_ALGORITHM: windows_core::PCWSTR = windows_core::w!("DES");
pub const BCRYPT_DES_CBC_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(497u32 as _);
pub const BCRYPT_DES_CFB_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(529u32 as _);
pub const BCRYPT_DES_ECB_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(513u32 as _);
pub const BCRYPT_DH_ALGORITHM: windows_core::PCWSTR = windows_core::w!("DH");
pub const BCRYPT_DH_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(641u32 as _);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BCRYPT_DH_KEY_BLOB {
    pub dwMagic: BCRYPT_DH_KEY_BLOB_MAGIC,
    pub cbKey: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BCRYPT_DH_KEY_BLOB_MAGIC(pub u32);
pub const BCRYPT_DH_PARAMETERS: windows_core::PCWSTR = windows_core::w!("DHParameters");
pub const BCRYPT_DH_PARAMETERS_MAGIC: u32 = 1297107012u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BCRYPT_DH_PARAMETER_HEADER {
    pub cbLength: u32,
    pub dwMagic: u32,
    pub cbKeyLength: u32,
}
pub const BCRYPT_DH_PRIVATE_BLOB: windows_core::PCWSTR = windows_core::w!("DHPRIVATEBLOB");
pub const BCRYPT_DH_PRIVATE_MAGIC: BCRYPT_DH_KEY_BLOB_MAGIC = BCRYPT_DH_KEY_BLOB_MAGIC(1448101956u32);
pub const BCRYPT_DH_PUBLIC_BLOB: windows_core::PCWSTR = windows_core::w!("DHPUBLICBLOB");
pub const BCRYPT_DH_PUBLIC_MAGIC: BCRYPT_DH_KEY_BLOB_MAGIC = BCRYPT_DH_KEY_BLOB_MAGIC(1112557636u32);
pub const BCRYPT_DSA_ALGORITHM: windows_core::PCWSTR = windows_core::w!("DSA");
pub const BCRYPT_DSA_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(721u32 as _);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BCRYPT_DSA_KEY_BLOB {
    pub dwMagic: BCRYPT_DSA_MAGIC,
    pub cbKey: u32,
    pub Count: [u8; 4],
    pub Seed: [u8; 20],
    pub q: [u8; 20],
}
impl Default for BCRYPT_DSA_KEY_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BCRYPT_DSA_KEY_BLOB_V2 {
    pub dwMagic: BCRYPT_DSA_MAGIC,
    pub cbKey: u32,
    pub hashAlgorithm: HASHALGORITHM_ENUM,
    pub standardVersion: DSAFIPSVERSION_ENUM,
    pub cbSeedLength: u32,
    pub cbGroupSize: u32,
    pub Count: [u8; 4],
}
impl Default for BCRYPT_DSA_KEY_BLOB_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BCRYPT_DSA_MAGIC(pub u32);
pub const BCRYPT_DSA_PARAMETERS: windows_core::PCWSTR = windows_core::w!("DSAParameters");
pub const BCRYPT_DSA_PARAMETERS_MAGIC: u32 = 1297109828u32;
pub const BCRYPT_DSA_PARAMETERS_MAGIC_V2: u32 = 843927620u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BCRYPT_DSA_PARAMETER_HEADER {
    pub cbLength: u32,
    pub dwMagic: u32,
    pub cbKeyLength: u32,
    pub Count: [u8; 4],
    pub Seed: [u8; 20],
    pub q: [u8; 20],
}
impl Default for BCRYPT_DSA_PARAMETER_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BCRYPT_DSA_PARAMETER_HEADER_V2 {
    pub cbLength: u32,
    pub dwMagic: u32,
    pub cbKeyLength: u32,
    pub hashAlgorithm: HASHALGORITHM_ENUM,
    pub standardVersion: DSAFIPSVERSION_ENUM,
    pub cbSeedLength: u32,
    pub cbGroupSize: u32,
    pub Count: [u8; 4],
}
impl Default for BCRYPT_DSA_PARAMETER_HEADER_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const BCRYPT_DSA_PRIVATE_BLOB: windows_core::PCWSTR = windows_core::w!("DSAPRIVATEBLOB");
pub const BCRYPT_DSA_PRIVATE_MAGIC: BCRYPT_DSA_MAGIC = BCRYPT_DSA_MAGIC(1448104772u32);
pub const BCRYPT_DSA_PRIVATE_MAGIC_V2: u32 = 844517444u32;
pub const BCRYPT_DSA_PUBLIC_BLOB: windows_core::PCWSTR = windows_core::w!("DSAPUBLICBLOB");
pub const BCRYPT_DSA_PUBLIC_MAGIC: BCRYPT_DSA_MAGIC = BCRYPT_DSA_MAGIC(1112560452u32);
pub const BCRYPT_DSA_PUBLIC_MAGIC_V2: u32 = 843206724u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BCRYPT_ECCFULLKEY_BLOB {
    pub dwMagic: u32,
    pub dwVersion: u32,
    pub dwCurveType: ECC_CURVE_TYPE_ENUM,
    pub dwCurveGenerationAlgId: ECC_CURVE_ALG_ID_ENUM,
    pub cbFieldLength: u32,
    pub cbSubgroupOrder: u32,
    pub cbCofactor: u32,
    pub cbSeed: u32,
}
pub const BCRYPT_ECCFULLPRIVATE_BLOB: windows_core::PCWSTR = windows_core::w!("ECCFULLPRIVATEBLOB");
pub const BCRYPT_ECCFULLPUBLIC_BLOB: windows_core::PCWSTR = windows_core::w!("ECCFULLPUBLICBLOB");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BCRYPT_ECCKEY_BLOB {
    pub dwMagic: u32,
    pub cbKey: u32,
}
pub const BCRYPT_ECCPRIVATE_BLOB: windows_core::PCWSTR = windows_core::w!("ECCPRIVATEBLOB");
pub const BCRYPT_ECCPUBLIC_BLOB: windows_core::PCWSTR = windows_core::w!("ECCPUBLICBLOB");
pub const BCRYPT_ECC_CURVE_25519: windows_core::PCWSTR = windows_core::w!("curve25519");
pub const BCRYPT_ECC_CURVE_BRAINPOOLP160R1: windows_core::PCWSTR = windows_core::w!("brainpoolP160r1");
pub const BCRYPT_ECC_CURVE_BRAINPOOLP160T1: windows_core::PCWSTR = windows_core::w!("brainpoolP160t1");
pub const BCRYPT_ECC_CURVE_BRAINPOOLP192R1: windows_core::PCWSTR = windows_core::w!("brainpoolP192r1");
pub const BCRYPT_ECC_CURVE_BRAINPOOLP192T1: windows_core::PCWSTR = windows_core::w!("brainpoolP192t1");
pub const BCRYPT_ECC_CURVE_BRAINPOOLP224R1: windows_core::PCWSTR = windows_core::w!("brainpoolP224r1");
pub const BCRYPT_ECC_CURVE_BRAINPOOLP224T1: windows_core::PCWSTR = windows_core::w!("brainpoolP224t1");
pub const BCRYPT_ECC_CURVE_BRAINPOOLP256R1: windows_core::PCWSTR = windows_core::w!("brainpoolP256r1");
pub const BCRYPT_ECC_CURVE_BRAINPOOLP256T1: windows_core::PCWSTR = windows_core::w!("brainpoolP256t1");
pub const BCRYPT_ECC_CURVE_BRAINPOOLP320R1: windows_core::PCWSTR = windows_core::w!("brainpoolP320r1");
pub const BCRYPT_ECC_CURVE_BRAINPOOLP320T1: windows_core::PCWSTR = windows_core::w!("brainpoolP320t1");
pub const BCRYPT_ECC_CURVE_BRAINPOOLP384R1: windows_core::PCWSTR = windows_core::w!("brainpoolP384r1");
pub const BCRYPT_ECC_CURVE_BRAINPOOLP384T1: windows_core::PCWSTR = windows_core::w!("brainpoolP384t1");
pub const BCRYPT_ECC_CURVE_BRAINPOOLP512R1: windows_core::PCWSTR = windows_core::w!("brainpoolP512r1");
pub const BCRYPT_ECC_CURVE_BRAINPOOLP512T1: windows_core::PCWSTR = windows_core::w!("brainpoolP512t1");
pub const BCRYPT_ECC_CURVE_EC192WAPI: windows_core::PCWSTR = windows_core::w!("ec192wapi");
pub const BCRYPT_ECC_CURVE_NAME: windows_core::PCWSTR = windows_core::w!("ECCCurveName");
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BCRYPT_ECC_CURVE_NAMES {
    pub dwEccCurveNames: u32,
    pub pEccCurveNames: *mut windows_core::PWSTR,
}
impl Default for BCRYPT_ECC_CURVE_NAMES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const BCRYPT_ECC_CURVE_NAME_LIST: windows_core::PCWSTR = windows_core::w!("ECCCurveNameList");
pub const BCRYPT_ECC_CURVE_NISTP192: windows_core::PCWSTR = windows_core::w!("nistP192");
pub const BCRYPT_ECC_CURVE_NISTP224: windows_core::PCWSTR = windows_core::w!("nistP224");
pub const BCRYPT_ECC_CURVE_NISTP256: windows_core::PCWSTR = windows_core::w!("nistP256");
pub const BCRYPT_ECC_CURVE_NISTP384: windows_core::PCWSTR = windows_core::w!("nistP384");
pub const BCRYPT_ECC_CURVE_NISTP521: windows_core::PCWSTR = windows_core::w!("nistP521");
pub const BCRYPT_ECC_CURVE_NUMSP256T1: windows_core::PCWSTR = windows_core::w!("numsP256t1");
pub const BCRYPT_ECC_CURVE_NUMSP384T1: windows_core::PCWSTR = windows_core::w!("numsP384t1");
pub const BCRYPT_ECC_CURVE_NUMSP512T1: windows_core::PCWSTR = windows_core::w!("numsP512t1");
pub const BCRYPT_ECC_CURVE_SECP160K1: windows_core::PCWSTR = windows_core::w!("secP160k1");
pub const BCRYPT_ECC_CURVE_SECP160R1: windows_core::PCWSTR = windows_core::w!("secP160r1");
pub const BCRYPT_ECC_CURVE_SECP160R2: windows_core::PCWSTR = windows_core::w!("secP160r2");
pub const BCRYPT_ECC_CURVE_SECP192K1: windows_core::PCWSTR = windows_core::w!("secP192k1");
pub const BCRYPT_ECC_CURVE_SECP192R1: windows_core::PCWSTR = windows_core::w!("secP192r1");
pub const BCRYPT_ECC_CURVE_SECP224K1: windows_core::PCWSTR = windows_core::w!("secP224k1");
pub const BCRYPT_ECC_CURVE_SECP224R1: windows_core::PCWSTR = windows_core::w!("secP224r1");
pub const BCRYPT_ECC_CURVE_SECP256K1: windows_core::PCWSTR = windows_core::w!("secP256k1");
pub const BCRYPT_ECC_CURVE_SECP256R1: windows_core::PCWSTR = windows_core::w!("secP256r1");
pub const BCRYPT_ECC_CURVE_SECP384R1: windows_core::PCWSTR = windows_core::w!("secP384r1");
pub const BCRYPT_ECC_CURVE_SECP521R1: windows_core::PCWSTR = windows_core::w!("secP521r1");
pub const BCRYPT_ECC_CURVE_WTLS12: windows_core::PCWSTR = windows_core::w!("wtls12");
pub const BCRYPT_ECC_CURVE_WTLS7: windows_core::PCWSTR = windows_core::w!("wtls7");
pub const BCRYPT_ECC_CURVE_WTLS9: windows_core::PCWSTR = windows_core::w!("wtls9");
pub const BCRYPT_ECC_CURVE_X962P192V1: windows_core::PCWSTR = windows_core::w!("x962P192v1");
pub const BCRYPT_ECC_CURVE_X962P192V2: windows_core::PCWSTR = windows_core::w!("x962P192v2");
pub const BCRYPT_ECC_CURVE_X962P192V3: windows_core::PCWSTR = windows_core::w!("x962P192v3");
pub const BCRYPT_ECC_CURVE_X962P239V1: windows_core::PCWSTR = windows_core::w!("x962P239v1");
pub const BCRYPT_ECC_CURVE_X962P239V2: windows_core::PCWSTR = windows_core::w!("x962P239v2");
pub const BCRYPT_ECC_CURVE_X962P239V3: windows_core::PCWSTR = windows_core::w!("x962P239v3");
pub const BCRYPT_ECC_CURVE_X962P256V1: windows_core::PCWSTR = windows_core::w!("x962P256v1");
pub const BCRYPT_ECC_FULLKEY_BLOB_V1: u32 = 1u32;
pub const BCRYPT_ECC_PARAMETERS: windows_core::PCWSTR = windows_core::w!("ECCParameters");
pub const BCRYPT_ECC_PARAMETERS_MAGIC: u32 = 1346585413u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BCRYPT_ECC_PARAMETER_HEADER {
    pub dwVersion: u32,
    pub dwCurveType: ECC_CURVE_TYPE_ENUM,
    pub dwCurveGenerationAlgId: ECC_CURVE_ALG_ID_ENUM,
    pub cbFieldLength: u32,
    pub cbSubgroupOrder: u32,
    pub cbCofactor: u32,
    pub cbSeed: u32,
}
pub const BCRYPT_ECC_PARAMETER_HEADER_V1: u32 = 1u32;
pub const BCRYPT_ECC_PRIME_MONTGOMERY_CURVE: ECC_CURVE_TYPE_ENUM = ECC_CURVE_TYPE_ENUM(3i32);
pub const BCRYPT_ECC_PRIME_SHORT_WEIERSTRASS_CURVE: ECC_CURVE_TYPE_ENUM = ECC_CURVE_TYPE_ENUM(1i32);
pub const BCRYPT_ECC_PRIME_TWISTED_EDWARDS_CURVE: ECC_CURVE_TYPE_ENUM = ECC_CURVE_TYPE_ENUM(2i32);
pub const BCRYPT_ECDH_ALGORITHM: windows_core::PCWSTR = windows_core::w!("ECDH");
pub const BCRYPT_ECDH_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(657u32 as _);
pub const BCRYPT_ECDH_P256_ALGORITHM: windows_core::PCWSTR = windows_core::w!("ECDH_P256");
pub const BCRYPT_ECDH_P256_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(673u32 as _);
pub const BCRYPT_ECDH_P384_ALGORITHM: windows_core::PCWSTR = windows_core::w!("ECDH_P384");
pub const BCRYPT_ECDH_P384_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(689u32 as _);
pub const BCRYPT_ECDH_P521_ALGORITHM: windows_core::PCWSTR = windows_core::w!("ECDH_P521");
pub const BCRYPT_ECDH_P521_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(705u32 as _);
pub const BCRYPT_ECDH_PRIVATE_GENERIC_MAGIC: u32 = 1447772997u32;
pub const BCRYPT_ECDH_PRIVATE_P256_MAGIC: u32 = 843793221u32;
pub const BCRYPT_ECDH_PRIVATE_P384_MAGIC: u32 = 877347653u32;
pub const BCRYPT_ECDH_PRIVATE_P521_MAGIC: u32 = 910902085u32;
pub const BCRYPT_ECDH_PUBLIC_GENERIC_MAGIC: u32 = 1347109701u32;
pub const BCRYPT_ECDH_PUBLIC_P256_MAGIC: u32 = 827016005u32;
pub const BCRYPT_ECDH_PUBLIC_P384_MAGIC: u32 = 860570437u32;
pub const BCRYPT_ECDH_PUBLIC_P521_MAGIC: u32 = 894124869u32;
pub const BCRYPT_ECDSA_ALGORITHM: windows_core::PCWSTR = windows_core::w!("ECDSA");
pub const BCRYPT_ECDSA_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(241u32 as _);
pub const BCRYPT_ECDSA_P256_ALGORITHM: windows_core::PCWSTR = windows_core::w!("ECDSA_P256");
pub const BCRYPT_ECDSA_P256_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(737u32 as _);
pub const BCRYPT_ECDSA_P384_ALGORITHM: windows_core::PCWSTR = windows_core::w!("ECDSA_P384");
pub const BCRYPT_ECDSA_P384_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(753u32 as _);
pub const BCRYPT_ECDSA_P521_ALGORITHM: windows_core::PCWSTR = windows_core::w!("ECDSA_P521");
pub const BCRYPT_ECDSA_P521_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(769u32 as _);
pub const BCRYPT_ECDSA_PRIVATE_GENERIC_MAGIC: u32 = 1447314245u32;
pub const BCRYPT_ECDSA_PRIVATE_P256_MAGIC: u32 = 844317509u32;
pub const BCRYPT_ECDSA_PRIVATE_P384_MAGIC: u32 = 877871941u32;
pub const BCRYPT_ECDSA_PRIVATE_P521_MAGIC: u32 = 911426373u32;
pub const BCRYPT_ECDSA_PUBLIC_GENERIC_MAGIC: u32 = 1346650949u32;
pub const BCRYPT_ECDSA_PUBLIC_P256_MAGIC: u32 = 827540293u32;
pub const BCRYPT_ECDSA_PUBLIC_P384_MAGIC: u32 = 861094725u32;
pub const BCRYPT_ECDSA_PUBLIC_P521_MAGIC: u32 = 894649157u32;
pub const BCRYPT_EFFECTIVE_KEY_LENGTH: windows_core::PCWSTR = windows_core::w!("EffectiveKeyLength");
pub const BCRYPT_ENABLE_INCOMPATIBLE_FIPS_CHECKS: u32 = 256u32;
pub const BCRYPT_EXTENDED_KEYSIZE: u32 = 128u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BCRYPT_FLAGS(pub u32);
impl BCRYPT_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for BCRYPT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for BCRYPT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for BCRYPT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for BCRYPT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for BCRYPT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const BCRYPT_GENERATE_IV: u32 = 32u32;
pub const BCRYPT_GLOBAL_PARAMETERS: windows_core::PCWSTR = windows_core::w!("SecretAgreementParam");
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BCRYPT_HANDLE(pub *mut core::ffi::c_void);
impl BCRYPT_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for BCRYPT_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const BCRYPT_HASH_BLOCK_LENGTH: windows_core::PCWSTR = windows_core::w!("HashBlockLength");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct BCRYPT_HASH_FUNCTION_TABLE {
    pub Version: BCRYPT_INTERFACE_VERSION,
    pub OpenAlgorithmProvider: BCryptOpenAlgorithmProviderFn,
    pub GetProperty: BCryptGetPropertyFn,
    pub SetProperty: BCryptSetPropertyFn,
    pub CloseAlgorithmProvider: BCryptCloseAlgorithmProviderFn,
    pub CreateHash: BCryptCreateHashFn,
    pub HashData: BCryptHashDataFn,
    pub FinishHash: BCryptFinishHashFn,
    pub DuplicateHash: BCryptDuplicateHashFn,
    pub DestroyHash: BCryptDestroyHashFn,
    pub CreateMultiHash: BCryptCreateMultiHashFn,
    pub ProcessMultiOperations: BCryptProcessMultiOperationsFn,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BCRYPT_HASH_HANDLE(pub *mut core::ffi::c_void);
impl BCRYPT_HASH_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl windows_core::Free for BCRYPT_HASH_HANDLE {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_core::link!("bcrypt.dll" "system" fn BCryptDestroyHash(hhash : *mut core::ffi::c_void) -> i32);
            unsafe {
                BCryptDestroyHash(self.0);
            }
        }
    }
}
impl Default for BCRYPT_HASH_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::imp::CanInto<BCRYPT_HANDLE> for BCRYPT_HASH_HANDLE {}
impl From<BCRYPT_HASH_HANDLE> for BCRYPT_HANDLE {
    fn from(value: BCRYPT_HASH_HANDLE) -> Self {
        Self(value.0)
    }
}
pub const BCRYPT_HASH_INTERFACE: BCRYPT_INTERFACE = BCRYPT_INTERFACE(2u32);
pub const BCRYPT_HASH_INTERFACE_MAJORVERSION_2: u32 = 2u32;
pub const BCRYPT_HASH_LENGTH: windows_core::PCWSTR = windows_core::w!("HashDigestLength");
pub const BCRYPT_HASH_OID_LIST: windows_core::PCWSTR = windows_core::w!("HashOIDList");
pub const BCRYPT_HASH_OPERATION: BCRYPT_OPERATION = BCRYPT_OPERATION(2u32);
pub const BCRYPT_HASH_OPERATION_FINISH_HASH: BCRYPT_HASH_OPERATION_TYPE = BCRYPT_HASH_OPERATION_TYPE(2i32);
pub const BCRYPT_HASH_OPERATION_HASH_DATA: BCRYPT_HASH_OPERATION_TYPE = BCRYPT_HASH_OPERATION_TYPE(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BCRYPT_HASH_OPERATION_TYPE(pub i32);
pub const BCRYPT_HASH_REUSABLE_FLAG: BCRYPT_OPEN_ALGORITHM_PROVIDER_FLAGS = BCRYPT_OPEN_ALGORITHM_PROVIDER_FLAGS(32u32);
pub const BCRYPT_HKDF_ALGORITHM: windows_core::PCWSTR = windows_core::w!("HKDF");
pub const BCRYPT_HKDF_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(913u32 as _);
pub const BCRYPT_HKDF_HASH_ALGORITHM: windows_core::PCWSTR = windows_core::w!("HkdfHashAlgorithm");
pub const BCRYPT_HKDF_PRK_AND_FINALIZE: windows_core::PCWSTR = windows_core::w!("HkdfPrkAndFinalize");
pub const BCRYPT_HKDF_SALT_AND_FINALIZE: windows_core::PCWSTR = windows_core::w!("HkdfSaltAndFinalize");
pub const BCRYPT_HMAC_MD2_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(289u32 as _);
pub const BCRYPT_HMAC_MD4_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(305u32 as _);
pub const BCRYPT_HMAC_MD5_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(145u32 as _);
pub const BCRYPT_HMAC_SHA1_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(161u32 as _);
pub const BCRYPT_HMAC_SHA256_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(177u32 as _);
pub const BCRYPT_HMAC_SHA384_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(193u32 as _);
pub const BCRYPT_HMAC_SHA512_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(209u32 as _);
pub const BCRYPT_INITIALIZATION_VECTOR: windows_core::PCWSTR = windows_core::w!("IV");
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BCRYPT_INTERFACE(pub u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BCRYPT_INTERFACE_VERSION {
    pub MajorVersion: u16,
    pub MinorVersion: u16,
}
pub const BCRYPT_IS_IFX_TPM_WEAK_KEY: windows_core::PCWSTR = windows_core::w!("IsIfxTpmWeakKey");
pub const BCRYPT_IS_KEYED_HASH: windows_core::PCWSTR = windows_core::w!("IsKeyedHash");
pub const BCRYPT_IS_REUSABLE_HASH: windows_core::PCWSTR = windows_core::w!("IsReusableHash");
pub const BCRYPT_KDF_HASH: windows_core::PCWSTR = windows_core::w!("HASH");
pub const BCRYPT_KDF_HKDF: windows_core::PCWSTR = windows_core::w!("HKDF");
pub const BCRYPT_KDF_HMAC: windows_core::PCWSTR = windows_core::w!("HMAC");
pub const BCRYPT_KDF_RAW_SECRET: windows_core::PCWSTR = windows_core::w!("TRUNCATE");
pub const BCRYPT_KDF_SP80056A_CONCAT: windows_core::PCWSTR = windows_core::w!("SP800_56A_CONCAT");
pub const BCRYPT_KDF_TLS_PRF: windows_core::PCWSTR = windows_core::w!("TLS_PRF");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BCRYPT_KEY_BLOB {
    pub Magic: u32,
}
pub const BCRYPT_KEY_DATA_BLOB: windows_core::PCWSTR = windows_core::w!("KeyDataBlob");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BCRYPT_KEY_DATA_BLOB_HEADER {
    pub dwMagic: u32,
    pub dwVersion: u32,
    pub cbKeyData: u32,
}
pub const BCRYPT_KEY_DATA_BLOB_MAGIC: u32 = 1296188491u32;
pub const BCRYPT_KEY_DATA_BLOB_VERSION1: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct BCRYPT_KEY_DERIVATION_FUNCTION_TABLE {
    pub Version: BCRYPT_INTERFACE_VERSION,
    pub OpenAlgorithmProvider: BCryptOpenAlgorithmProviderFn,
    pub GetProperty: BCryptGetPropertyFn,
    pub SetProperty: BCryptSetPropertyFn,
    pub CloseAlgorithmProvider: BCryptCloseAlgorithmProviderFn,
    pub GenerateKey: BCryptGenerateSymmetricKeyFn,
    pub DestroyKey: BCryptDestroyKeyFn,
    pub KeyDerivation: BCryptKeyDerivationFn,
    pub ExportKey: BCryptExportKeyFn,
    pub ImportKey: BCryptImportKeyFn,
    pub DuplicateKey: BCryptDuplicateKeyFn,
}
pub const BCRYPT_KEY_DERIVATION_INTERFACE: u32 = 7u32;
pub const BCRYPT_KEY_DERIVATION_OPERATION: u32 = 64u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BCRYPT_KEY_HANDLE(pub *mut core::ffi::c_void);
impl BCRYPT_KEY_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl windows_core::Free for BCRYPT_KEY_HANDLE {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_core::link!("bcrypt.dll" "system" fn BCryptDestroyKey(hkey : *mut core::ffi::c_void) -> i32);
            unsafe {
                BCryptDestroyKey(self.0);
            }
        }
    }
}
impl Default for BCRYPT_KEY_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::imp::CanInto<BCRYPT_HANDLE> for BCRYPT_KEY_HANDLE {}
impl From<BCRYPT_KEY_HANDLE> for BCRYPT_HANDLE {
    fn from(value: BCRYPT_KEY_HANDLE) -> Self {
        Self(value.0)
    }
}
pub const BCRYPT_KEY_LENGTH: windows_core::PCWSTR = windows_core::w!("KeyLength");
pub const BCRYPT_KEY_LENGTHS: windows_core::PCWSTR = windows_core::w!("KeyLengths");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BCRYPT_KEY_LENGTHS_STRUCT {
    pub dwMinLength: u32,
    pub dwMaxLength: u32,
    pub dwIncrement: u32,
}
pub const BCRYPT_KEY_OBJECT_LENGTH: windows_core::PCWSTR = windows_core::w!("KeyObjectLength");
pub const BCRYPT_KEY_STRENGTH: windows_core::PCWSTR = windows_core::w!("KeyStrength");
pub const BCRYPT_KEY_VALIDATION_RANGE: u32 = 16u32;
pub const BCRYPT_KEY_VALIDATION_RANGE_AND_ORDER: u32 = 24u32;
pub const BCRYPT_KEY_VALIDATION_REGENERATE: u32 = 32u32;
pub const BCRYPT_MD2_ALGORITHM: windows_core::PCWSTR = windows_core::w!("MD2");
pub const BCRYPT_MD2_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(1u32 as _);
pub const BCRYPT_MD4_ALGORITHM: windows_core::PCWSTR = windows_core::w!("MD4");
pub const BCRYPT_MD4_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(17u32 as _);
pub const BCRYPT_MD5_ALGORITHM: windows_core::PCWSTR = windows_core::w!("MD5");
pub const BCRYPT_MD5_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(33u32 as _);
pub const BCRYPT_MESSAGE_BLOCK_LENGTH: windows_core::PCWSTR = windows_core::w!("MessageBlockLength");
pub const BCRYPT_MULTI_FLAG: u32 = 64u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BCRYPT_MULTI_HASH_OPERATION {
    pub iHash: u32,
    pub hashOperation: BCRYPT_HASH_OPERATION_TYPE,
    pub pbBuffer: *mut u8,
    pub cbBuffer: u32,
}
impl Default for BCRYPT_MULTI_HASH_OPERATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const BCRYPT_MULTI_OBJECT_LENGTH: windows_core::PCWSTR = windows_core::w!("MultiObjectLength");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BCRYPT_MULTI_OBJECT_LENGTH_STRUCT {
    pub cbPerObject: u32,
    pub cbPerElement: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BCRYPT_MULTI_OPERATION_TYPE(pub i32);
pub const BCRYPT_NO_CURVE_GENERATION_ALG_ID: ECC_CURVE_ALG_ID_ENUM = ECC_CURVE_ALG_ID_ENUM(0i32);
pub const BCRYPT_NO_KEY_VALIDATION: u32 = 8u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BCRYPT_OAEP_PADDING_INFO {
    pub pszAlgId: windows_core::PCWSTR,
    pub pbLabel: *mut u8,
    pub cbLabel: u32,
}
impl Default for BCRYPT_OAEP_PADDING_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const BCRYPT_OBJECT_ALIGNMENT: u32 = 16u32;
pub const BCRYPT_OBJECT_LENGTH: windows_core::PCWSTR = windows_core::w!("ObjectLength");
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BCRYPT_OID {
    pub cbOID: u32,
    pub pbOID: *mut u8,
}
impl Default for BCRYPT_OID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BCRYPT_OID_LIST {
    pub dwOIDCount: u32,
    pub pOIDs: *mut BCRYPT_OID,
}
impl Default for BCRYPT_OID_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const BCRYPT_OPAQUE_KEY_BLOB: windows_core::PCWSTR = windows_core::w!("OpaqueKeyBlob");
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BCRYPT_OPEN_ALGORITHM_PROVIDER_FLAGS(pub u32);
impl BCRYPT_OPEN_ALGORITHM_PROVIDER_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for BCRYPT_OPEN_ALGORITHM_PROVIDER_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for BCRYPT_OPEN_ALGORITHM_PROVIDER_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for BCRYPT_OPEN_ALGORITHM_PROVIDER_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for BCRYPT_OPEN_ALGORITHM_PROVIDER_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for BCRYPT_OPEN_ALGORITHM_PROVIDER_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BCRYPT_OPERATION(pub u32);
impl BCRYPT_OPERATION {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for BCRYPT_OPERATION {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for BCRYPT_OPERATION {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for BCRYPT_OPERATION {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for BCRYPT_OPERATION {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for BCRYPT_OPERATION {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const BCRYPT_OPERATION_TYPE_HASH: BCRYPT_MULTI_OPERATION_TYPE = BCRYPT_MULTI_OPERATION_TYPE(1i32);
pub const BCRYPT_PADDING_SCHEMES: windows_core::PCWSTR = windows_core::w!("PaddingSchemes");
pub const BCRYPT_PAD_NONE: BCRYPT_FLAGS = BCRYPT_FLAGS(1u32);
pub const BCRYPT_PAD_OAEP: BCRYPT_FLAGS = BCRYPT_FLAGS(4u32);
pub const BCRYPT_PAD_PKCS1: BCRYPT_FLAGS = BCRYPT_FLAGS(2u32);
pub const BCRYPT_PAD_PKCS1_OPTIONAL_HASH_OID: u32 = 16u32;
pub const BCRYPT_PAD_PSS: BCRYPT_FLAGS = BCRYPT_FLAGS(8u32);
pub const BCRYPT_PBKDF2_ALGORITHM: windows_core::PCWSTR = windows_core::w!("PBKDF2");
pub const BCRYPT_PBKDF2_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(817u32 as _);
pub const BCRYPT_PCP_PLATFORM_TYPE_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_PLATFORM_TYPE");
pub const BCRYPT_PCP_PROVIDER_VERSION_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_PROVIDER_VERSION");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BCRYPT_PKCS1_PADDING_INFO {
    pub pszAlgId: windows_core::PCWSTR,
}
pub const BCRYPT_PRIMITIVE_TYPE: windows_core::PCWSTR = windows_core::w!("PrimitiveType");
pub const BCRYPT_PRIVATE_KEY: windows_core::PCWSTR = windows_core::w!("PrivKeyVal");
pub const BCRYPT_PRIVATE_KEY_BLOB: windows_core::PCWSTR = windows_core::w!("PRIVATEBLOB");
pub const BCRYPT_PRIVATE_KEY_FLAG: u32 = 2u32;
pub const BCRYPT_PROVIDER_HANDLE: windows_core::PCWSTR = windows_core::w!("ProviderHandle");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BCRYPT_PROVIDER_NAME {
    pub pszProviderName: windows_core::PWSTR,
}
pub const BCRYPT_PROV_DISPATCH: BCRYPT_OPEN_ALGORITHM_PROVIDER_FLAGS = BCRYPT_OPEN_ALGORITHM_PROVIDER_FLAGS(1u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BCRYPT_PSS_PADDING_INFO {
    pub pszAlgId: windows_core::PCWSTR,
    pub cbSalt: u32,
}
pub const BCRYPT_PUBLIC_KEY_BLOB: windows_core::PCWSTR = windows_core::w!("PUBLICBLOB");
pub const BCRYPT_PUBLIC_KEY_FLAG: u32 = 1u32;
pub const BCRYPT_PUBLIC_KEY_LENGTH: windows_core::PCWSTR = windows_core::w!("PublicKeyLength");
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BCRYPT_QUERY_PROVIDER_MODE(pub u32);
pub const BCRYPT_RC2_ALGORITHM: windows_core::PCWSTR = windows_core::w!("RC2");
pub const BCRYPT_RC2_CBC_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(593u32 as _);
pub const BCRYPT_RC2_CFB_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(625u32 as _);
pub const BCRYPT_RC2_ECB_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(609u32 as _);
pub const BCRYPT_RC4_ALGORITHM: windows_core::PCWSTR = windows_core::w!("RC4");
pub const BCRYPT_RC4_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(113u32 as _);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BCRYPT_RESOLVE_PROVIDERS_FLAGS(pub u32);
impl BCRYPT_RESOLVE_PROVIDERS_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for BCRYPT_RESOLVE_PROVIDERS_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for BCRYPT_RESOLVE_PROVIDERS_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for BCRYPT_RESOLVE_PROVIDERS_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for BCRYPT_RESOLVE_PROVIDERS_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for BCRYPT_RESOLVE_PROVIDERS_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const BCRYPT_RNG_ALGORITHM: windows_core::PCWSTR = windows_core::w!("RNG");
pub const BCRYPT_RNG_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(129u32 as _);
pub const BCRYPT_RNG_DUAL_EC_ALGORITHM: windows_core::PCWSTR = windows_core::w!("DUALECRNG");
pub const BCRYPT_RNG_FIPS186_DSA_ALGORITHM: windows_core::PCWSTR = windows_core::w!("FIPS186DSARNG");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct BCRYPT_RNG_FUNCTION_TABLE {
    pub Version: BCRYPT_INTERFACE_VERSION,
    pub OpenAlgorithmProvider: BCryptOpenAlgorithmProviderFn,
    pub GetProperty: BCryptGetPropertyFn,
    pub SetProperty: BCryptSetPropertyFn,
    pub CloseAlgorithmProvider: BCryptCloseAlgorithmProviderFn,
    pub GenRandom: BCryptGenRandomFn,
}
pub const BCRYPT_RNG_INTERFACE: BCRYPT_INTERFACE = BCRYPT_INTERFACE(6u32);
pub const BCRYPT_RNG_OPERATION: BCRYPT_OPERATION = BCRYPT_OPERATION(32u32);
pub const BCRYPT_RNG_USE_ENTROPY_IN_BUFFER: BCRYPTGENRANDOM_FLAGS = BCRYPTGENRANDOM_FLAGS(1u32);
pub const BCRYPT_RSAFULLPRIVATE_BLOB: windows_core::PCWSTR = windows_core::w!("RSAFULLPRIVATEBLOB");
pub const BCRYPT_RSAFULLPRIVATE_MAGIC: BCRYPT_RSAKEY_BLOB_MAGIC = BCRYPT_RSAKEY_BLOB_MAGIC(859919186u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BCRYPT_RSAKEY_BLOB {
    pub Magic: BCRYPT_RSAKEY_BLOB_MAGIC,
    pub BitLength: u32,
    pub cbPublicExp: u32,
    pub cbModulus: u32,
    pub cbPrime1: u32,
    pub cbPrime2: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BCRYPT_RSAKEY_BLOB_MAGIC(pub u32);
pub const BCRYPT_RSAPRIVATE_BLOB: windows_core::PCWSTR = windows_core::w!("RSAPRIVATEBLOB");
pub const BCRYPT_RSAPRIVATE_MAGIC: BCRYPT_RSAKEY_BLOB_MAGIC = BCRYPT_RSAKEY_BLOB_MAGIC(843141970u32);
pub const BCRYPT_RSAPUBLIC_BLOB: windows_core::PCWSTR = windows_core::w!("RSAPUBLICBLOB");
pub const BCRYPT_RSAPUBLIC_MAGIC: BCRYPT_RSAKEY_BLOB_MAGIC = BCRYPT_RSAKEY_BLOB_MAGIC(826364754u32);
pub const BCRYPT_RSA_ALGORITHM: windows_core::PCWSTR = windows_core::w!("RSA");
pub const BCRYPT_RSA_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(225u32 as _);
pub const BCRYPT_RSA_SIGN_ALGORITHM: windows_core::PCWSTR = windows_core::w!("RSA_SIGN");
pub const BCRYPT_RSA_SIGN_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(785u32 as _);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct BCRYPT_SECRET_AGREEMENT_FUNCTION_TABLE {
    pub Version: BCRYPT_INTERFACE_VERSION,
    pub OpenAlgorithmProvider: BCryptOpenAlgorithmProviderFn,
    pub GetProperty: BCryptGetPropertyFn,
    pub SetProperty: BCryptSetPropertyFn,
    pub CloseAlgorithmProvider: BCryptCloseAlgorithmProviderFn,
    pub SecretAgreement: BCryptSecretAgreementFn,
    pub DeriveKey: BCryptDeriveKeyFn,
    pub DestroySecret: BCryptDestroySecretFn,
    pub GenerateKeyPair: BCryptGenerateKeyPairFn,
    pub FinalizeKeyPair: BCryptFinalizeKeyPairFn,
    pub ImportKeyPair: BCryptImportKeyPairFn,
    pub ExportKey: BCryptExportKeyFn,
    pub DestroyKey: BCryptDestroyKeyFn,
}
pub const BCRYPT_SECRET_AGREEMENT_INTERFACE: BCRYPT_INTERFACE = BCRYPT_INTERFACE(4u32);
pub const BCRYPT_SECRET_AGREEMENT_OPERATION: BCRYPT_OPERATION = BCRYPT_OPERATION(8u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BCRYPT_SECRET_HANDLE(pub *mut core::ffi::c_void);
impl BCRYPT_SECRET_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl windows_core::Free for BCRYPT_SECRET_HANDLE {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_core::link!("bcrypt.dll" "system" fn BCryptDestroySecret(hsecret : *mut core::ffi::c_void) -> i32);
            unsafe {
                BCryptDestroySecret(self.0);
            }
        }
    }
}
impl Default for BCRYPT_SECRET_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::imp::CanInto<BCRYPT_HANDLE> for BCRYPT_SECRET_HANDLE {}
impl From<BCRYPT_SECRET_HANDLE> for BCRYPT_HANDLE {
    fn from(value: BCRYPT_SECRET_HANDLE) -> Self {
        Self(value.0)
    }
}
pub const BCRYPT_SHA1_ALGORITHM: windows_core::PCWSTR = windows_core::w!("SHA1");
pub const BCRYPT_SHA1_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(49u32 as _);
pub const BCRYPT_SHA256_ALGORITHM: windows_core::PCWSTR = windows_core::w!("SHA256");
pub const BCRYPT_SHA256_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(65u32 as _);
pub const BCRYPT_SHA384_ALGORITHM: windows_core::PCWSTR = windows_core::w!("SHA384");
pub const BCRYPT_SHA384_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(81u32 as _);
pub const BCRYPT_SHA512_ALGORITHM: windows_core::PCWSTR = windows_core::w!("SHA512");
pub const BCRYPT_SHA512_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(97u32 as _);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct BCRYPT_SIGNATURE_FUNCTION_TABLE {
    pub Version: BCRYPT_INTERFACE_VERSION,
    pub OpenAlgorithmProvider: BCryptOpenAlgorithmProviderFn,
    pub GetProperty: BCryptGetPropertyFn,
    pub SetProperty: BCryptSetPropertyFn,
    pub CloseAlgorithmProvider: BCryptCloseAlgorithmProviderFn,
    pub GenerateKeyPair: BCryptGenerateKeyPairFn,
    pub FinalizeKeyPair: BCryptFinalizeKeyPairFn,
    pub SignHash: BCryptSignHashFn,
    pub VerifySignature: BCryptVerifySignatureFn,
    pub ImportKeyPair: BCryptImportKeyPairFn,
    pub ExportKey: BCryptExportKeyFn,
    pub DestroyKey: BCryptDestroyKeyFn,
}
pub const BCRYPT_SIGNATURE_INTERFACE: BCRYPT_INTERFACE = BCRYPT_INTERFACE(5u32);
pub const BCRYPT_SIGNATURE_LENGTH: windows_core::PCWSTR = windows_core::w!("SignatureLength");
pub const BCRYPT_SIGNATURE_OPERATION: BCRYPT_OPERATION = BCRYPT_OPERATION(16u32);
pub const BCRYPT_SP800108_CTR_HMAC_ALGORITHM: windows_core::PCWSTR = windows_core::w!("SP800_108_CTR_HMAC");
pub const BCRYPT_SP800108_CTR_HMAC_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(833u32 as _);
pub const BCRYPT_SP80056A_CONCAT_ALGORITHM: windows_core::PCWSTR = windows_core::w!("SP800_56A_CONCAT");
pub const BCRYPT_SP80056A_CONCAT_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(849u32 as _);
pub const BCRYPT_SUPPORTED_PAD_OAEP: u32 = 8u32;
pub const BCRYPT_SUPPORTED_PAD_PKCS1_ENC: u32 = 2u32;
pub const BCRYPT_SUPPORTED_PAD_PKCS1_SIG: u32 = 4u32;
pub const BCRYPT_SUPPORTED_PAD_PSS: u32 = 16u32;
pub const BCRYPT_SUPPORTED_PAD_ROUTER: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BCRYPT_TABLE(pub u32);
pub const BCRYPT_TLS1_1_KDF_ALGORITHM: windows_core::PCWSTR = windows_core::w!("TLS1_1_KDF");
pub const BCRYPT_TLS1_1_KDF_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(865u32 as _);
pub const BCRYPT_TLS1_2_KDF_ALGORITHM: windows_core::PCWSTR = windows_core::w!("TLS1_2_KDF");
pub const BCRYPT_TLS1_2_KDF_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(881u32 as _);
pub const BCRYPT_TLS_CBC_HMAC_VERIFY_FLAG: u32 = 4u32;
pub const BCRYPT_USE_SYSTEM_PREFERRED_RNG: BCRYPTGENRANDOM_FLAGS = BCRYPTGENRANDOM_FLAGS(2u32);
pub const BCRYPT_XTS_AES_ALGORITHM: windows_core::PCWSTR = windows_core::w!("XTS-AES");
pub const BCRYPT_XTS_AES_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(897u32 as _);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BCryptBuffer {
    pub cbBuffer: u32,
    pub BufferType: u32,
    pub pvBuffer: *mut core::ffi::c_void,
}
impl Default for BCryptBuffer {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BCryptBufferDesc {
    pub ulVersion: u32,
    pub cBuffers: u32,
    pub pBuffers: *mut BCryptBuffer,
}
impl Default for BCryptBufferDesc {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type BCryptCloseAlgorithmProviderFn = Option<unsafe extern "system" fn(halgorithm: BCRYPT_ALG_HANDLE, dwflags: u32) -> super::super::Foundation::NTSTATUS>;
pub type BCryptCreateHashFn = Option<unsafe extern "system" fn(halgorithm: BCRYPT_ALG_HANDLE, phhash: *mut BCRYPT_HASH_HANDLE, pbhashobject: *mut u8, cbhashobject: u32, pbsecret: *const u8, cbsecret: u32, dwflags: u32) -> super::super::Foundation::NTSTATUS>;
pub type BCryptCreateMultiHashFn = Option<unsafe extern "system" fn(halgorithm: BCRYPT_ALG_HANDLE, phhash: *mut BCRYPT_HASH_HANDLE, nhashes: u32, pbhashobject: *mut u8, cbhashobject: u32, pbsecret: *const u8, cbsecret: u32, dwflags: u32) -> super::super::Foundation::NTSTATUS>;
pub type BCryptDecryptFn = Option<unsafe extern "system" fn(hkey: BCRYPT_KEY_HANDLE, pbinput: *const u8, cbinput: u32, ppaddinginfo: *const core::ffi::c_void, pbiv: *mut u8, cbiv: u32, pboutput: *mut u8, cboutput: u32, pcbresult: *mut u32, dwflags: u32) -> super::super::Foundation::NTSTATUS>;
pub type BCryptDeriveKeyCapiFn = Option<unsafe extern "system" fn(hhash: BCRYPT_HASH_HANDLE, htargetalg: BCRYPT_ALG_HANDLE, pbderivedkey: *mut u8, cbderivedkey: u32, dwflags: u32) -> super::super::Foundation::NTSTATUS>;
pub type BCryptDeriveKeyFn = Option<unsafe extern "system" fn(hsharedsecret: BCRYPT_SECRET_HANDLE, pwszkdf: windows_core::PCWSTR, pparameterlist: *const BCryptBufferDesc, pbderivedkey: *mut u8, cbderivedkey: u32, pcbresult: *mut u32, dwflags: u32) -> super::super::Foundation::NTSTATUS>;
pub type BCryptDeriveKeyPBKDF2Fn = Option<unsafe extern "system" fn(hprf: BCRYPT_ALG_HANDLE, pbpassword: *const u8, cbpassword: u32, pbsalt: *const u8, cbsalt: u32, citerations: u64, pbderivedkey: *mut u8, cbderivedkey: u32, dwflags: u32) -> super::super::Foundation::NTSTATUS>;
pub type BCryptDestroyHashFn = Option<unsafe extern "system" fn(hhash: BCRYPT_HASH_HANDLE) -> super::super::Foundation::NTSTATUS>;
pub type BCryptDestroyKeyFn = Option<unsafe extern "system" fn(hkey: BCRYPT_KEY_HANDLE) -> super::super::Foundation::NTSTATUS>;
pub type BCryptDestroySecretFn = Option<unsafe extern "system" fn(hsecret: BCRYPT_SECRET_HANDLE) -> super::super::Foundation::NTSTATUS>;
pub type BCryptDuplicateHashFn = Option<unsafe extern "system" fn(hhash: BCRYPT_HASH_HANDLE, phnewhash: *mut BCRYPT_HASH_HANDLE, pbhashobject: *mut u8, cbhashobject: u32, dwflags: u32) -> super::super::Foundation::NTSTATUS>;
pub type BCryptDuplicateKeyFn = Option<unsafe extern "system" fn(hkey: BCRYPT_KEY_HANDLE, phnewkey: *mut BCRYPT_KEY_HANDLE, pbkeyobject: *mut u8, cbkeyobject: u32, dwflags: u32) -> super::super::Foundation::NTSTATUS>;
pub type BCryptEncryptFn = Option<unsafe extern "system" fn(hkey: BCRYPT_KEY_HANDLE, pbinput: *const u8, cbinput: u32, ppaddinginfo: *const core::ffi::c_void, pbiv: *mut u8, cbiv: u32, pboutput: *mut u8, cboutput: u32, pcbresult: *mut u32, dwflags: u32) -> super::super::Foundation::NTSTATUS>;
pub type BCryptExportKeyFn = Option<unsafe extern "system" fn(hkey: BCRYPT_KEY_HANDLE, hexportkey: BCRYPT_KEY_HANDLE, pszblobtype: windows_core::PCWSTR, pboutput: *mut u8, cboutput: u32, pcbresult: *mut u32, dwflags: u32) -> super::super::Foundation::NTSTATUS>;
pub type BCryptFinalizeKeyPairFn = Option<unsafe extern "system" fn(hkey: BCRYPT_KEY_HANDLE, dwflags: u32) -> super::super::Foundation::NTSTATUS>;
pub type BCryptFinishHashFn = Option<unsafe extern "system" fn(hhash: BCRYPT_HASH_HANDLE, pboutput: *mut u8, cboutput: u32, dwflags: u32) -> super::super::Foundation::NTSTATUS>;
pub type BCryptFreeBufferFn = Option<unsafe extern "system" fn(pvbuffer: *const core::ffi::c_void)>;
pub type BCryptGenRandomFn = Option<unsafe extern "system" fn(halgorithm: BCRYPT_ALG_HANDLE, pbbuffer: *mut u8, cbbuffer: u32, dwflags: u32) -> super::super::Foundation::NTSTATUS>;
pub type BCryptGenerateKeyPairFn = Option<unsafe extern "system" fn(halgorithm: BCRYPT_ALG_HANDLE, phkey: *mut BCRYPT_KEY_HANDLE, dwlength: u32, dwflags: u32) -> super::super::Foundation::NTSTATUS>;
pub type BCryptGenerateSymmetricKeyFn = Option<unsafe extern "system" fn(halgorithm: BCRYPT_ALG_HANDLE, phkey: *mut BCRYPT_KEY_HANDLE, pbkeyobject: *mut u8, cbkeyobject: u32, pbsecret: *const u8, cbsecret: u32, dwflags: u32) -> super::super::Foundation::NTSTATUS>;
pub type BCryptGetPropertyFn = Option<unsafe extern "system" fn(hobject: BCRYPT_HANDLE, pszproperty: windows_core::PCWSTR, pboutput: *mut u8, cboutput: u32, pcbresult: *mut u32, dwflags: u32) -> super::super::Foundation::NTSTATUS>;
pub type BCryptHashDataFn = Option<unsafe extern "system" fn(hhash: BCRYPT_HASH_HANDLE, pbinput: *const u8, cbinput: u32, dwflags: u32) -> super::super::Foundation::NTSTATUS>;
pub type BCryptImportKeyFn = Option<unsafe extern "system" fn(halgorithm: BCRYPT_ALG_HANDLE, himportkey: BCRYPT_KEY_HANDLE, pszblobtype: windows_core::PCWSTR, phkey: *mut BCRYPT_KEY_HANDLE, pbkeyobject: *mut u8, cbkeyobject: u32, pbinput: *const u8, cbinput: u32, dwflags: u32) -> super::super::Foundation::NTSTATUS>;
pub type BCryptImportKeyPairFn = Option<unsafe extern "system" fn(halgorithm: BCRYPT_ALG_HANDLE, himportkey: BCRYPT_KEY_HANDLE, pszblobtype: windows_core::PCWSTR, phkey: *mut BCRYPT_KEY_HANDLE, pbinput: *const u8, cbinput: u32, dwflags: u32) -> super::super::Foundation::NTSTATUS>;
pub type BCryptKeyDerivationFn = Option<unsafe extern "system" fn(hkey: BCRYPT_KEY_HANDLE, pparameterlist: *const BCryptBufferDesc, pbderivedkey: *mut u8, cbderivedkey: u32, pcbresult: *mut u32, dwflags: u32) -> super::super::Foundation::NTSTATUS>;
pub type BCryptOpenAlgorithmProviderFn = Option<unsafe extern "system" fn(phalgorithm: *mut BCRYPT_ALG_HANDLE, pszalgid: windows_core::PCWSTR, dwflags: u32) -> super::super::Foundation::NTSTATUS>;
pub type BCryptProcessMultiOperationsFn = Option<unsafe extern "system" fn(hobject: BCRYPT_HANDLE, operationtype: BCRYPT_MULTI_OPERATION_TYPE, poperations: *const core::ffi::c_void, cboperations: u32, dwflags: u32) -> super::super::Foundation::NTSTATUS>;
pub type BCryptSecretAgreementFn = Option<unsafe extern "system" fn(hprivkey: BCRYPT_KEY_HANDLE, hpubkey: BCRYPT_KEY_HANDLE, phagreedsecret: *mut BCRYPT_SECRET_HANDLE, dwflags: u32) -> super::super::Foundation::NTSTATUS>;
pub type BCryptSetPropertyFn = Option<unsafe extern "system" fn(hobject: BCRYPT_HANDLE, pszproperty: windows_core::PCWSTR, pbinput: *const u8, cbinput: u32, dwflags: u32) -> super::super::Foundation::NTSTATUS>;
pub type BCryptSignHashFn = Option<unsafe extern "system" fn(hkey: BCRYPT_KEY_HANDLE, ppaddinginfo: *const core::ffi::c_void, pbinput: *const u8, cbinput: u32, pboutput: *mut u8, cboutput: u32, pcbresult: *mut u32, dwflags: u32) -> super::super::Foundation::NTSTATUS>;
pub type BCryptVerifySignatureFn = Option<unsafe extern "system" fn(hkey: BCRYPT_KEY_HANDLE, ppaddinginfo: *const core::ffi::c_void, pbhash: *const u8, cbhash: u32, pbsignature: *const u8, cbsignature: u32, dwflags: u32) -> super::super::Foundation::NTSTATUS>;
pub const CALG_3DES: ALG_ID = ALG_ID(26115u32);
pub const CALG_3DES_112: ALG_ID = ALG_ID(26121u32);
pub const CALG_AES: ALG_ID = ALG_ID(26129u32);
pub const CALG_AES_128: ALG_ID = ALG_ID(26126u32);
pub const CALG_AES_192: ALG_ID = ALG_ID(26127u32);
pub const CALG_AES_256: ALG_ID = ALG_ID(26128u32);
pub const CALG_AGREEDKEY_ANY: ALG_ID = ALG_ID(43523u32);
pub const CALG_CYLINK_MEK: ALG_ID = ALG_ID(26124u32);
pub const CALG_DES: ALG_ID = ALG_ID(26113u32);
pub const CALG_DESX: ALG_ID = ALG_ID(26116u32);
pub const CALG_DH_EPHEM: ALG_ID = ALG_ID(43522u32);
pub const CALG_DH_SF: ALG_ID = ALG_ID(43521u32);
pub const CALG_DSS_SIGN: ALG_ID = ALG_ID(8704u32);
pub const CALG_ECDH: ALG_ID = ALG_ID(43525u32);
pub const CALG_ECDH_EPHEM: ALG_ID = ALG_ID(44550u32);
pub const CALG_ECDSA: ALG_ID = ALG_ID(8707u32);
pub const CALG_ECMQV: ALG_ID = ALG_ID(40961u32);
pub const CALG_HASH_REPLACE_OWF: ALG_ID = ALG_ID(32779u32);
pub const CALG_HMAC: ALG_ID = ALG_ID(32777u32);
pub const CALG_HUGHES_MD5: ALG_ID = ALG_ID(40963u32);
pub const CALG_KEA_KEYX: ALG_ID = ALG_ID(43524u32);
pub const CALG_MAC: ALG_ID = ALG_ID(32773u32);
pub const CALG_MD2: ALG_ID = ALG_ID(32769u32);
pub const CALG_MD4: ALG_ID = ALG_ID(32770u32);
pub const CALG_MD5: ALG_ID = ALG_ID(32771u32);
pub const CALG_NO_SIGN: ALG_ID = ALG_ID(8192u32);
pub const CALG_NULLCIPHER: ALG_ID = ALG_ID(24576u32);
pub const CALG_OID_INFO_CNG_ONLY: u32 = 4294967295u32;
pub const CALG_OID_INFO_PARAMETERS: u32 = 4294967294u32;
pub const CALG_PCT1_MASTER: ALG_ID = ALG_ID(19460u32);
pub const CALG_RC2: ALG_ID = ALG_ID(26114u32);
pub const CALG_RC4: ALG_ID = ALG_ID(26625u32);
pub const CALG_RC5: ALG_ID = ALG_ID(26125u32);
pub const CALG_RSA_KEYX: ALG_ID = ALG_ID(41984u32);
pub const CALG_RSA_SIGN: ALG_ID = ALG_ID(9216u32);
pub const CALG_SCHANNEL_ENC_KEY: ALG_ID = ALG_ID(19463u32);
pub const CALG_SCHANNEL_MAC_KEY: ALG_ID = ALG_ID(19459u32);
pub const CALG_SCHANNEL_MASTER_HASH: ALG_ID = ALG_ID(19458u32);
pub const CALG_SEAL: ALG_ID = ALG_ID(26626u32);
pub const CALG_SHA: ALG_ID = ALG_ID(32772u32);
pub const CALG_SHA1: ALG_ID = ALG_ID(32772u32);
pub const CALG_SHA_256: ALG_ID = ALG_ID(32780u32);
pub const CALG_SHA_384: ALG_ID = ALG_ID(32781u32);
pub const CALG_SHA_512: ALG_ID = ALG_ID(32782u32);
pub const CALG_SKIPJACK: ALG_ID = ALG_ID(26122u32);
pub const CALG_SSL2_MASTER: ALG_ID = ALG_ID(19461u32);
pub const CALG_SSL3_MASTER: ALG_ID = ALG_ID(19457u32);
pub const CALG_SSL3_SHAMD5: ALG_ID = ALG_ID(32776u32);
pub const CALG_TEK: ALG_ID = ALG_ID(26123u32);
pub const CALG_THIRDPARTY_CIPHER: ALG_ID = ALG_ID(28672u32);
pub const CALG_THIRDPARTY_HASH: ALG_ID = ALG_ID(36864u32);
pub const CALG_THIRDPARTY_KEY_EXCHANGE: ALG_ID = ALG_ID(45056u32);
pub const CALG_THIRDPARTY_SIGNATURE: ALG_ID = ALG_ID(12288u32);
pub const CALG_TLS1PRF: ALG_ID = ALG_ID(32778u32);
pub const CALG_TLS1_MASTER: ALG_ID = ALG_ID(19462u32);
pub const CARD_3DES_112_ALGORITHM: windows_core::PCWSTR = windows_core::w!("3DES_112");
pub const CARD_3DES_ALGORITHM: windows_core::PCWSTR = windows_core::w!("3DES");
pub const CARD_AES_ALGORITHM: windows_core::PCWSTR = windows_core::w!("AES");
pub const CARD_ASYMMETRIC_OPERATION: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CARD_AUTHENTICATE {
    pub dwVersion: u32,
    pub dwFlags: u32,
    pub PinId: u32,
    pub cbPinData: u32,
    pub pbPinData: [u8; 1],
}
impl Default for CARD_AUTHENTICATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CARD_AUTHENTICATE_CURRENT_VERSION: u32 = 7u32;
pub const CARD_AUTHENTICATE_GENERATE_SESSION_PIN: u32 = 268435456u32;
pub const CARD_AUTHENTICATE_PIN_CHALLENGE_RESPONSE: u32 = 1u32;
pub const CARD_AUTHENTICATE_PIN_PIN: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CARD_AUTHENTICATE_RESPONSE {
    pub dwVersion: u32,
    pub cbSessionPin: u32,
    pub cAttemptsRemaining: u32,
    pub pbSessionPin: [u8; 1],
}
impl Default for CARD_AUTHENTICATE_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CARD_AUTHENTICATE_RESPONSE_CURRENT_VERSION: u32 = 7u32;
pub const CARD_AUTHENTICATE_RESPONSE_VERSION_SEVEN: u32 = 7u32;
pub const CARD_AUTHENTICATE_SESSION_PIN: u32 = 536870912u32;
pub const CARD_AUTHENTICATE_VERSION_SEVEN: u32 = 7u32;
pub const CARD_BUFFER_SIZE_ONLY: u32 = 536870912u32;
pub const CARD_CACHE_FILE_CURRENT_VERSION: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CARD_CACHE_FILE_FORMAT {
    pub bVersion: u8,
    pub bPinsFreshness: u8,
    pub wContainersFreshness: u16,
    pub wFilesFreshness: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CARD_CAPABILITIES {
    pub dwVersion: u32,
    pub fCertificateCompression: windows_core::BOOL,
    pub fKeyGen: windows_core::BOOL,
}
pub const CARD_CAPABILITIES_CURRENT_VERSION: u32 = 1u32;
pub const CARD_CHAIN_MODE_CBC: windows_core::PCWSTR = windows_core::w!("ChainingModeCBC");
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CARD_CHANGE_AUTHENTICATOR {
    pub dwVersion: u32,
    pub dwFlags: u32,
    pub dwAuthenticatingPinId: u32,
    pub cbAuthenticatingPinData: u32,
    pub dwTargetPinId: u32,
    pub cbTargetData: u32,
    pub cRetryCount: u32,
    pub pbData: [u8; 1],
}
impl Default for CARD_CHANGE_AUTHENTICATOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CARD_CHANGE_AUTHENTICATOR_CURRENT_VERSION: u32 = 7u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CARD_CHANGE_AUTHENTICATOR_RESPONSE {
    pub dwVersion: u32,
    pub cAttemptsRemaining: u32,
}
pub const CARD_CHANGE_AUTHENTICATOR_RESPONSE_CURRENT_VERSION: u32 = 7u32;
pub const CARD_CHANGE_AUTHENTICATOR_RESPONSE_VERSION_SEVEN: u32 = 7u32;
pub const CARD_CHANGE_AUTHENTICATOR_VERSION_SEVEN: u32 = 7u32;
pub const CARD_CIPHER_OPERATION: u32 = 1u32;
pub const CARD_CREATE_CONTAINER_KEY_GEN: u32 = 1u32;
pub const CARD_CREATE_CONTAINER_KEY_IMPORT: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CARD_DATA {
    pub dwVersion: u32,
    pub pbAtr: *mut u8,
    pub cbAtr: u32,
    pub pwszCardName: windows_core::PWSTR,
    pub pfnCspAlloc: PFN_CSP_ALLOC,
    pub pfnCspReAlloc: PFN_CSP_REALLOC,
    pub pfnCspFree: PFN_CSP_FREE,
    pub pfnCspCacheAddFile: PFN_CSP_CACHE_ADD_FILE,
    pub pfnCspCacheLookupFile: PFN_CSP_CACHE_LOOKUP_FILE,
    pub pfnCspCacheDeleteFile: PFN_CSP_CACHE_DELETE_FILE,
    pub pvCacheContext: *mut core::ffi::c_void,
    pub pfnCspPadData: PFN_CSP_PAD_DATA,
    pub hSCardCtx: usize,
    pub hScard: usize,
    pub pvVendorSpecific: *mut core::ffi::c_void,
    pub pfnCardDeleteContext: PFN_CARD_DELETE_CONTEXT,
    pub pfnCardQueryCapabilities: PFN_CARD_QUERY_CAPABILITIES,
    pub pfnCardDeleteContainer: PFN_CARD_DELETE_CONTAINER,
    pub pfnCardCreateContainer: PFN_CARD_CREATE_CONTAINER,
    pub pfnCardGetContainerInfo: PFN_CARD_GET_CONTAINER_INFO,
    pub pfnCardAuthenticatePin: PFN_CARD_AUTHENTICATE_PIN,
    pub pfnCardGetChallenge: PFN_CARD_GET_CHALLENGE,
    pub pfnCardAuthenticateChallenge: PFN_CARD_AUTHENTICATE_CHALLENGE,
    pub pfnCardUnblockPin: PFN_CARD_UNBLOCK_PIN,
    pub pfnCardChangeAuthenticator: PFN_CARD_CHANGE_AUTHENTICATOR,
    pub pfnCardDeauthenticate: PFN_CARD_DEAUTHENTICATE,
    pub pfnCardCreateDirectory: PFN_CARD_CREATE_DIRECTORY,
    pub pfnCardDeleteDirectory: PFN_CARD_DELETE_DIRECTORY,
    pub pvUnused3: *mut core::ffi::c_void,
    pub pvUnused4: *mut core::ffi::c_void,
    pub pfnCardCreateFile: PFN_CARD_CREATE_FILE,
    pub pfnCardReadFile: PFN_CARD_READ_FILE,
    pub pfnCardWriteFile: PFN_CARD_WRITE_FILE,
    pub pfnCardDeleteFile: PFN_CARD_DELETE_FILE,
    pub pfnCardEnumFiles: PFN_CARD_ENUM_FILES,
    pub pfnCardGetFileInfo: PFN_CARD_GET_FILE_INFO,
    pub pfnCardQueryFreeSpace: PFN_CARD_QUERY_FREE_SPACE,
    pub pfnCardQueryKeySizes: PFN_CARD_QUERY_KEY_SIZES,
    pub pfnCardSignData: PFN_CARD_SIGN_DATA,
    pub pfnCardRSADecrypt: PFN_CARD_RSA_DECRYPT,
    pub pfnCardConstructDHAgreement: PFN_CARD_CONSTRUCT_DH_AGREEMENT,
    pub pfnCardDeriveKey: PFN_CARD_DERIVE_KEY,
    pub pfnCardDestroyDHAgreement: PFN_CARD_DESTROY_DH_AGREEMENT,
    pub pfnCspGetDHAgreement: PFN_CSP_GET_DH_AGREEMENT,
    pub pfnCardGetChallengeEx: PFN_CARD_GET_CHALLENGE_EX,
    pub pfnCardAuthenticateEx: PFN_CARD_AUTHENTICATE_EX,
    pub pfnCardChangeAuthenticatorEx: PFN_CARD_CHANGE_AUTHENTICATOR_EX,
    pub pfnCardDeauthenticateEx: PFN_CARD_DEAUTHENTICATE_EX,
    pub pfnCardGetContainerProperty: PFN_CARD_GET_CONTAINER_PROPERTY,
    pub pfnCardSetContainerProperty: PFN_CARD_SET_CONTAINER_PROPERTY,
    pub pfnCardGetProperty: PFN_CARD_GET_PROPERTY,
    pub pfnCardSetProperty: PFN_CARD_SET_PROPERTY,
    pub pfnCspUnpadData: PFN_CSP_UNPAD_DATA,
    pub pfnMDImportSessionKey: PFN_MD_IMPORT_SESSION_KEY,
    pub pfnMDEncryptData: PFN_MD_ENCRYPT_DATA,
    pub pfnCardImportSessionKey: PFN_CARD_IMPORT_SESSION_KEY,
    pub pfnCardGetSharedKeyHandle: PFN_CARD_GET_SHARED_KEY_HANDLE,
    pub pfnCardGetAlgorithmProperty: PFN_CARD_GET_ALGORITHM_PROPERTY,
    pub pfnCardGetKeyProperty: PFN_CARD_GET_KEY_PROPERTY,
    pub pfnCardSetKeyProperty: PFN_CARD_SET_KEY_PROPERTY,
    pub pfnCardDestroyKey: PFN_CARD_DESTROY_KEY,
    pub pfnCardProcessEncryptedData: PFN_CARD_PROCESS_ENCRYPTED_DATA,
    pub pfnCardCreateContainerEx: PFN_CARD_CREATE_CONTAINER_EX,
}
impl Default for CARD_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CARD_DATA_CURRENT_VERSION: u32 = 7u32;
pub const CARD_DATA_VALUE_UNKNOWN: u32 = 4294967295u32;
pub const CARD_DATA_VERSION_FIVE: u32 = 5u32;
pub const CARD_DATA_VERSION_FOUR: u32 = 4u32;
pub const CARD_DATA_VERSION_SEVEN: u32 = 7u32;
pub const CARD_DATA_VERSION_SIX: u32 = 6u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CARD_DERIVE_KEY {
    pub dwVersion: u32,
    pub dwFlags: u32,
    pub pwszKDF: windows_core::PWSTR,
    pub bSecretAgreementIndex: u8,
    pub pParameterList: *mut core::ffi::c_void,
    pub pbDerivedKey: *mut u8,
    pub cbDerivedKey: u32,
    pub pwszAlgId: windows_core::PWSTR,
    pub dwKeyLen: u32,
    pub hKey: usize,
}
impl Default for CARD_DERIVE_KEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CARD_DERIVE_KEY_CURRENT_VERSION: u32 = 2u32;
pub const CARD_DERIVE_KEY_VERSION: u32 = 1u32;
pub const CARD_DERIVE_KEY_VERSION_TWO: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CARD_DH_AGREEMENT_INFO {
    pub dwVersion: u32,
    pub bContainerIndex: u8,
    pub dwFlags: u32,
    pub dwPublicKey: u32,
    pub pbPublicKey: *mut u8,
    pub pbReserved: *mut u8,
    pub cbReserved: u32,
    pub bSecretAgreementIndex: u8,
}
impl Default for CARD_DH_AGREEMENT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CARD_DH_AGREEMENT_INFO_VERSION: u32 = 2u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CARD_DIRECTORY_ACCESS_CONDITION(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CARD_ENCRYPTED_DATA {
    pub pbEncryptedData: *mut u8,
    pub cbEncryptedData: u32,
}
impl Default for CARD_ENCRYPTED_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CARD_FILE_ACCESS_CONDITION(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CARD_FILE_INFO {
    pub dwVersion: u32,
    pub cbFileSize: u32,
    pub AccessCondition: CARD_FILE_ACCESS_CONDITION,
}
pub const CARD_FILE_INFO_CURRENT_VERSION: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CARD_FREE_SPACE_INFO {
    pub dwVersion: u32,
    pub dwBytesAvailable: u32,
    pub dwKeyContainersAvailable: u32,
    pub dwMaxKeyContainers: u32,
}
pub const CARD_FREE_SPACE_INFO_CURRENT_VERSION: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CARD_IMPORT_KEYPAIR {
    pub dwVersion: u32,
    pub bContainerIndex: u8,
    pub PinId: u32,
    pub dwKeySpec: u32,
    pub dwKeySize: u32,
    pub cbInput: u32,
    pub pbInput: [u8; 1],
}
impl Default for CARD_IMPORT_KEYPAIR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CARD_IMPORT_KEYPAIR_CURRENT_VERSION: u32 = 7u32;
pub const CARD_IMPORT_KEYPAIR_VERSION_SEVEN: u32 = 7u32;
pub const CARD_KEY_IMPORT_ECC_KEYEST: u32 = 4u32;
pub const CARD_KEY_IMPORT_PLAIN_TEXT: u32 = 1u32;
pub const CARD_KEY_IMPORT_RSA_KEYEST: u32 = 2u32;
pub const CARD_KEY_IMPORT_SHARED_SYMMETRIC: u32 = 8u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CARD_KEY_SIZES {
    pub dwVersion: u32,
    pub dwMinimumBitlen: u32,
    pub dwDefaultBitlen: u32,
    pub dwMaximumBitlen: u32,
    pub dwIncrementalBitlen: u32,
}
pub const CARD_KEY_SIZES_CURRENT_VERSION: u32 = 1u32;
pub const CARD_PADDING_INFO_PRESENT: u32 = 1073741824u32;
pub const CARD_PADDING_NONE: u32 = 1u32;
pub const CARD_PADDING_OAEP: u32 = 8u32;
pub const CARD_PADDING_PKCS1: u32 = 2u32;
pub const CARD_PADDING_PSS: u32 = 4u32;
pub const CARD_PIN_SILENT_CONTEXT: u32 = 64u32;
pub const CARD_PIN_STRENGTH_PLAINTEXT: u32 = 1u32;
pub const CARD_PIN_STRENGTH_SESSION_PIN: u32 = 2u32;
pub const CARD_RETURN_KEY_HANDLE: u32 = 16777216u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CARD_RSA_DECRYPT_INFO {
    pub dwVersion: u32,
    pub bContainerIndex: u8,
    pub dwKeySpec: u32,
    pub pbData: *mut u8,
    pub cbData: u32,
    pub pPaddingInfo: *mut core::ffi::c_void,
    pub dwPaddingType: u32,
}
impl Default for CARD_RSA_DECRYPT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CARD_RSA_KEY_DECRYPT_INFO_CURRENT_VERSION: u32 = 2u32;
pub const CARD_RSA_KEY_DECRYPT_INFO_VERSION_ONE: u32 = 1u32;
pub const CARD_RSA_KEY_DECRYPT_INFO_VERSION_TWO: u32 = 2u32;
pub const CARD_SECURE_KEY_INJECTION_NO_CARD_MODE: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CARD_SIGNING_INFO {
    pub dwVersion: u32,
    pub bContainerIndex: u8,
    pub dwKeySpec: u32,
    pub dwSigningFlags: u32,
    pub aiHashAlg: ALG_ID,
    pub pbData: *mut u8,
    pub cbData: u32,
    pub pbSignedData: *mut u8,
    pub cbSignedData: u32,
    pub pPaddingInfo: *mut core::ffi::c_void,
    pub dwPaddingType: u32,
}
impl Default for CARD_SIGNING_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CARD_SIGNING_INFO_BASIC_VERSION: u32 = 1u32;
pub const CARD_SIGNING_INFO_CURRENT_VERSION: u32 = 2u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CASetupProperty(pub i32);
pub const CCP_ASSOCIATED_ECDH_KEY: windows_core::PCWSTR = windows_core::w!("Associated ECDH Key");
pub const CCP_CONTAINER_INFO: windows_core::PCWSTR = windows_core::w!("Container Info");
pub const CCP_PIN_IDENTIFIER: windows_core::PCWSTR = windows_core::w!("PIN Identifier");
pub const CCertSrvSetup: windows_core::GUID = windows_core::GUID::from_u128(0x961f180f_f55c_413d_a9b3_7d2af4d8e42f);
pub const CCertSrvSetupKeyInformation: windows_core::GUID = windows_core::GUID::from_u128(0x38373906_5433_4633_b0fb_29b7e78262e1);
pub const CCertificateEnrollmentPolicyServerSetup: windows_core::GUID = windows_core::GUID::from_u128(0xafe2fa32_41b1_459d_a5de_49add8a72182);
pub const CCertificateEnrollmentServerSetup: windows_core::GUID = windows_core::GUID::from_u128(0x9902f3bc_88af_4cf8_ae62_7140531552b6);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CEPSetupProperty(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CERTIFICATE_CHAIN_BLOB {
    pub certCount: u32,
    pub rawCertificates: *mut CRYPT_INTEGER_BLOB,
}
impl Default for CERTIFICATE_CHAIN_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_ACCESS_DESCRIPTION {
    pub pszAccessMethod: windows_core::PSTR,
    pub AccessLocation: CERT_ALT_NAME_ENTRY,
}
impl Default for CERT_ACCESS_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_ACCESS_STATE_GP_SYSTEM_STORE_FLAG: u32 = 8u32;
pub const CERT_ACCESS_STATE_LM_SYSTEM_STORE_FLAG: u32 = 4u32;
pub const CERT_ACCESS_STATE_PROP_ID: u32 = 14u32;
pub const CERT_ACCESS_STATE_SHARED_USER_FLAG: u32 = 16u32;
pub const CERT_ACCESS_STATE_SYSTEM_STORE_FLAG: u32 = 2u32;
pub const CERT_ACCESS_STATE_WRITE_PERSIST_FLAG: u32 = 1u32;
pub const CERT_AIA_URL_RETRIEVED_PROP_ID: u32 = 67u32;
pub const CERT_ALT_NAME_EDI_PARTY_NAME: u32 = 6u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_ALT_NAME_ENTRY {
    pub dwAltNameChoice: u32,
    pub Anonymous: CERT_ALT_NAME_ENTRY_0,
}
impl Default for CERT_ALT_NAME_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CERT_ALT_NAME_ENTRY_0 {
    pub pOtherName: *mut CERT_OTHER_NAME,
    pub pwszRfc822Name: windows_core::PWSTR,
    pub pwszDNSName: windows_core::PWSTR,
    pub DirectoryName: CRYPT_INTEGER_BLOB,
    pub pwszURL: windows_core::PWSTR,
    pub IPAddress: CRYPT_INTEGER_BLOB,
    pub pszRegisteredID: windows_core::PSTR,
}
impl Default for CERT_ALT_NAME_ENTRY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_ALT_NAME_ENTRY_ERR_INDEX_MASK: u32 = 255u32;
pub const CERT_ALT_NAME_ENTRY_ERR_INDEX_SHIFT: u32 = 16u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CERT_ALT_NAME_INFO {
    pub cAltEntry: u32,
    pub rgAltEntry: *mut CERT_ALT_NAME_ENTRY,
}
impl Default for CERT_ALT_NAME_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_ALT_NAME_VALUE_ERR_INDEX_MASK: u32 = 65535u32;
pub const CERT_ALT_NAME_VALUE_ERR_INDEX_SHIFT: u32 = 0u32;
pub const CERT_ALT_NAME_X400_ADDRESS: u32 = 4u32;
pub const CERT_ARCHIVED_KEY_HASH_PROP_ID: u32 = 65u32;
pub const CERT_ARCHIVED_PROP_ID: u32 = 19u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CERT_AUTHORITY_INFO_ACCESS {
    pub cAccDescr: u32,
    pub rgAccDescr: *mut CERT_ACCESS_DESCRIPTION,
}
impl Default for CERT_AUTHORITY_INFO_ACCESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_AUTHORITY_INFO_ACCESS_PROP_ID: u32 = 68u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CERT_AUTHORITY_KEY_ID2_INFO {
    pub KeyId: CRYPT_INTEGER_BLOB,
    pub AuthorityCertIssuer: CERT_ALT_NAME_INFO,
    pub AuthorityCertSerialNumber: CRYPT_INTEGER_BLOB,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CERT_AUTHORITY_KEY_ID_INFO {
    pub KeyId: CRYPT_INTEGER_BLOB,
    pub CertIssuer: CRYPT_INTEGER_BLOB,
    pub CertSerialNumber: CRYPT_INTEGER_BLOB,
}
pub const CERT_AUTH_ROOT_AUTO_UPDATE_DISABLE_PARTIAL_CHAIN_LOGGING_FLAG: u32 = 2u32;
pub const CERT_AUTH_ROOT_AUTO_UPDATE_DISABLE_UNTRUSTED_ROOT_LOGGING_FLAG: u32 = 1u32;
pub const CERT_AUTH_ROOT_AUTO_UPDATE_ENCODED_CTL_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("EncodedCtl");
pub const CERT_AUTH_ROOT_AUTO_UPDATE_FLAGS_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("Flags");
pub const CERT_AUTH_ROOT_AUTO_UPDATE_LAST_SYNC_TIME_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("LastSyncTime");
pub const CERT_AUTH_ROOT_AUTO_UPDATE_ROOT_DIR_URL_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("RootDirUrl");
pub const CERT_AUTH_ROOT_AUTO_UPDATE_SYNC_DELTA_TIME_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("SyncDeltaTime");
pub const CERT_AUTH_ROOT_CAB_FILENAME: windows_core::PCWSTR = windows_core::w!("authrootstl.cab");
pub const CERT_AUTH_ROOT_CERT_EXT: windows_core::PCWSTR = windows_core::w!(".crt");
pub const CERT_AUTH_ROOT_CTL_FILENAME: windows_core::PCWSTR = windows_core::w!("authroot.stl");
pub const CERT_AUTH_ROOT_CTL_FILENAME_A: windows_core::PCSTR = windows_core::s!("authroot.stl");
pub const CERT_AUTH_ROOT_SEQ_FILENAME: windows_core::PCWSTR = windows_core::w!("authrootseq.txt");
pub const CERT_AUTH_ROOT_SHA256_HASH_PROP_ID: u32 = 98u32;
pub const CERT_AUTO_ENROLL_PROP_ID: u32 = 21u32;
pub const CERT_AUTO_ENROLL_RETRY_PROP_ID: u32 = 66u32;
pub const CERT_AUTO_UPDATE_DISABLE_RANDOM_QUERY_STRING_FLAG: u32 = 4u32;
pub const CERT_AUTO_UPDATE_ROOT_DIR_URL_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("RootDirUrl");
pub const CERT_AUTO_UPDATE_SYNC_FROM_DIR_URL_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("SyncFromDirUrl");
pub const CERT_BACKED_UP_PROP_ID: u32 = 69u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CERT_BASIC_CONSTRAINTS2_INFO {
    pub fCA: windows_core::BOOL,
    pub fPathLenConstraint: windows_core::BOOL,
    pub dwPathLenConstraint: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CERT_BASIC_CONSTRAINTS_INFO {
    pub SubjectType: CRYPT_BIT_BLOB,
    pub fPathLenConstraint: windows_core::BOOL,
    pub dwPathLenConstraint: u32,
    pub cSubtreesConstraint: u32,
    pub rgSubtreesConstraint: *mut CRYPT_INTEGER_BLOB,
}
impl Default for CERT_BASIC_CONSTRAINTS_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_BIOMETRIC_DATA {
    pub dwTypeOfBiometricDataChoice: CERT_BIOMETRIC_DATA_TYPE,
    pub Anonymous: CERT_BIOMETRIC_DATA_0,
    pub HashedUrl: CERT_HASHED_URL,
}
impl Default for CERT_BIOMETRIC_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CERT_BIOMETRIC_DATA_0 {
    pub dwPredefined: u32,
    pub pszObjId: windows_core::PSTR,
}
impl Default for CERT_BIOMETRIC_DATA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CERT_BIOMETRIC_DATA_TYPE(pub u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CERT_BIOMETRIC_EXT_INFO {
    pub cBiometricData: u32,
    pub rgBiometricData: *mut CERT_BIOMETRIC_DATA,
}
impl Default for CERT_BIOMETRIC_EXT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_BIOMETRIC_OID_DATA_CHOICE: CERT_BIOMETRIC_DATA_TYPE = CERT_BIOMETRIC_DATA_TYPE(2u32);
pub const CERT_BIOMETRIC_PICTURE_TYPE: u32 = 0u32;
pub const CERT_BIOMETRIC_PREDEFINED_DATA_CHOICE: CERT_BIOMETRIC_DATA_TYPE = CERT_BIOMETRIC_DATA_TYPE(1u32);
pub const CERT_BIOMETRIC_SIGNATURE_TYPE: u32 = 1u32;
pub const CERT_BUNDLE_CERTIFICATE: u32 = 0u32;
pub const CERT_BUNDLE_CRL: u32 = 1u32;
pub const CERT_CASE_INSENSITIVE_IS_RDN_ATTRS_FLAG: u32 = 2u32;
pub const CERT_CA_DISABLE_CRL_PROP_ID: u32 = 82u32;
pub const CERT_CA_OCSP_AUTHORITY_INFO_ACCESS_PROP_ID: u32 = 81u32;
pub const CERT_CA_SUBJECT_FLAG: u32 = 128u32;
pub const CERT_CEP_PROP_ID: u32 = 87u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CERT_CHAIN {
    pub cCerts: u32,
    pub certs: *mut CRYPT_INTEGER_BLOB,
    pub keyLocatorInfo: CRYPT_KEY_PROV_INFO,
}
impl Default for CERT_CHAIN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_CHAIN_AUTO_CURRENT_USER: u32 = 1u32;
pub const CERT_CHAIN_AUTO_FLAGS_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("AutoFlags");
pub const CERT_CHAIN_AUTO_FLUSH_DISABLE_FLAG: u32 = 1u32;
pub const CERT_CHAIN_AUTO_FLUSH_FIRST_DELTA_SECONDS_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("AutoFlushFirstDeltaSeconds");
pub const CERT_CHAIN_AUTO_FLUSH_NEXT_DELTA_SECONDS_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("AutoFlushNextDeltaSeconds");
pub const CERT_CHAIN_AUTO_HPKP_RULE_INFO: u32 = 8u32;
pub const CERT_CHAIN_AUTO_IMPERSONATED: u32 = 3u32;
pub const CERT_CHAIN_AUTO_LOCAL_MACHINE: u32 = 2u32;
pub const CERT_CHAIN_AUTO_LOG_CREATE_FLAG: u32 = 2u32;
pub const CERT_CHAIN_AUTO_LOG_FILE_NAME_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("AutoLogFileName");
pub const CERT_CHAIN_AUTO_LOG_FLUSH_FLAG: u32 = 8u32;
pub const CERT_CHAIN_AUTO_LOG_FREE_FLAG: u32 = 4u32;
pub const CERT_CHAIN_AUTO_NETWORK_INFO: u32 = 6u32;
pub const CERT_CHAIN_AUTO_PINRULE_INFO: u32 = 5u32;
pub const CERT_CHAIN_AUTO_PROCESS_INFO: u32 = 4u32;
pub const CERT_CHAIN_AUTO_SERIAL_LOCAL_MACHINE: u32 = 7u32;
pub const CERT_CHAIN_CACHE_END_CERT: u32 = 1u32;
pub const CERT_CHAIN_CACHE_ONLY_URL_RETRIEVAL: u32 = 4u32;
pub const CERT_CHAIN_CACHE_RESYNC_FILETIME_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("ChainCacheResyncFiletime");
pub const CERT_CHAIN_CONFIG_REGPATH: windows_core::PCWSTR = windows_core::w!("Software\\Microsoft\\Cryptography\\OID\\EncodingType 0\\CertDllCreateCertificateChainEngine\\Config");
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CERT_CHAIN_CONTEXT {
    pub cbSize: u32,
    pub TrustStatus: CERT_TRUST_STATUS,
    pub cChain: u32,
    pub rgpChain: *mut *mut CERT_SIMPLE_CHAIN,
    pub cLowerQualityChainContext: u32,
    pub rgpLowerQualityChainContext: *mut *mut CERT_CHAIN_CONTEXT,
    pub fHasRevocationFreshnessTime: windows_core::BOOL,
    pub dwRevocationFreshnessTime: u32,
    pub dwCreateFlags: u32,
    pub ChainId: windows_core::GUID,
}
impl Default for CERT_CHAIN_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_CHAIN_CRL_VALIDITY_EXT_PERIOD_HOURS_DEFAULT: u32 = 12u32;
pub const CERT_CHAIN_CRL_VALIDITY_EXT_PERIOD_HOURS_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("CRLValidityExtensionPeriod");
pub const CERT_CHAIN_CROSS_CERT_DOWNLOAD_INTERVAL_HOURS_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("CrossCertDownloadIntervalHours");
pub const CERT_CHAIN_DEFAULT_CONFIG_SUBDIR: windows_core::PCWSTR = windows_core::w!("Default");
pub const CERT_CHAIN_DISABLE_AIA: u32 = 8192u32;
pub const CERT_CHAIN_DISABLE_AIA_URL_RETRIEVAL_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("DisableAIAUrlRetrieval");
pub const CERT_CHAIN_DISABLE_ALL_EKU_WEAK_FLAG: u32 = 65536u32;
pub const CERT_CHAIN_DISABLE_AUTH_ROOT_AUTO_UPDATE: u32 = 256u32;
pub const CERT_CHAIN_DISABLE_AUTO_FLUSH_PROCESS_NAME_LIST_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("DisableAutoFlushProcessNameList");
pub const CERT_CHAIN_DISABLE_CA_NAME_CONSTRAINTS_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("DisableCANameConstraints");
pub const CERT_CHAIN_DISABLE_CODE_SIGNING_WEAK_FLAG: u32 = 4194304u32;
pub const CERT_CHAIN_DISABLE_ECC_PARA_FLAG: u32 = 16u32;
pub const CERT_CHAIN_DISABLE_FILE_HASH_WEAK_FLAG: u32 = 4096u32;
pub const CERT_CHAIN_DISABLE_MANDATORY_BASIC_CONSTRAINTS_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("DisableMandatoryBasicConstraints");
pub const CERT_CHAIN_DISABLE_MD2_MD4: u32 = 4096u32;
pub const CERT_CHAIN_DISABLE_MOTW_CODE_SIGNING_WEAK_FLAG: u32 = 8388608u32;
pub const CERT_CHAIN_DISABLE_MOTW_FILE_HASH_WEAK_FLAG: u32 = 8192u32;
pub const CERT_CHAIN_DISABLE_MOTW_TIMESTAMP_HASH_WEAK_FLAG: u32 = 32768u32;
pub const CERT_CHAIN_DISABLE_MOTW_TIMESTAMP_WEAK_FLAG: u32 = 134217728u32;
pub const CERT_CHAIN_DISABLE_MY_PEER_TRUST: u32 = 2048u32;
pub const CERT_CHAIN_DISABLE_OPT_IN_SERVER_AUTH_WEAK_FLAG: u32 = 262144u32;
pub const CERT_CHAIN_DISABLE_PASS1_QUALITY_FILTERING: u32 = 64u32;
pub const CERT_CHAIN_DISABLE_SERIAL_CHAIN_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("DisableSerialChain");
pub const CERT_CHAIN_DISABLE_SERVER_AUTH_WEAK_FLAG: u32 = 1048576u32;
pub const CERT_CHAIN_DISABLE_SYNC_WITH_SSL_TIME_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("DisableSyncWithSslTime");
pub const CERT_CHAIN_DISABLE_TIMESTAMP_HASH_WEAK_FLAG: u32 = 16384u32;
pub const CERT_CHAIN_DISABLE_TIMESTAMP_WEAK_FLAG: u32 = 67108864u32;
pub const CERT_CHAIN_DISABLE_UNSUPPORTED_CRITICAL_EXTENSIONS_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("DisableUnsupportedCriticalExtensions");
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CERT_CHAIN_ELEMENT {
    pub cbSize: u32,
    pub pCertContext: *const CERT_CONTEXT,
    pub TrustStatus: CERT_TRUST_STATUS,
    pub pRevocationInfo: *mut CERT_REVOCATION_INFO,
    pub pIssuanceUsage: *mut CTL_USAGE,
    pub pApplicationUsage: *mut CTL_USAGE,
    pub pwszExtendedErrorInfo: windows_core::PCWSTR,
}
impl Default for CERT_CHAIN_ELEMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_CHAIN_ENABLE_ALL_EKU_HYGIENE_FLAG: u32 = 131072u32;
pub const CERT_CHAIN_ENABLE_CACHE_AUTO_UPDATE: u32 = 16u32;
pub const CERT_CHAIN_ENABLE_CODE_SIGNING_HYGIENE_FLAG: u32 = 16777216u32;
pub const CERT_CHAIN_ENABLE_DISALLOWED_CA: u32 = 131072u32;
pub const CERT_CHAIN_ENABLE_MD2_MD4_FLAG: u32 = 1u32;
pub const CERT_CHAIN_ENABLE_MOTW_CODE_SIGNING_HYGIENE_FLAG: u32 = 33554432u32;
pub const CERT_CHAIN_ENABLE_MOTW_TIMESTAMP_HYGIENE_FLAG: u32 = 536870912u32;
pub const CERT_CHAIN_ENABLE_ONLY_WEAK_LOGGING_FLAG: u32 = 8u32;
pub const CERT_CHAIN_ENABLE_PEER_TRUST: u32 = 1024u32;
pub const CERT_CHAIN_ENABLE_SERVER_AUTH_HYGIENE_FLAG: u32 = 2097152u32;
pub const CERT_CHAIN_ENABLE_SHARE_STORE: u32 = 32u32;
pub const CERT_CHAIN_ENABLE_TIMESTAMP_HYGIENE_FLAG: u32 = 268435456u32;
pub const CERT_CHAIN_ENABLE_WEAK_LOGGING_FLAG: u32 = 4u32;
pub const CERT_CHAIN_ENABLE_WEAK_RSA_ROOT_FLAG: u32 = 2u32;
pub const CERT_CHAIN_ENABLE_WEAK_SETTINGS_FLAG: u32 = 2147483648u32;
pub const CERT_CHAIN_ENABLE_WEAK_SIGNATURE_FLAGS_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("EnableWeakSignatureFlags");
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CERT_CHAIN_ENGINE_CONFIG {
    pub cbSize: u32,
    pub hRestrictedRoot: HCERTSTORE,
    pub hRestrictedTrust: HCERTSTORE,
    pub hRestrictedOther: HCERTSTORE,
    pub cAdditionalStore: u32,
    pub rghAdditionalStore: *mut HCERTSTORE,
    pub dwFlags: u32,
    pub dwUrlRetrievalTimeout: u32,
    pub MaximumCachedCertificates: u32,
    pub CycleDetectionModulus: u32,
    pub hExclusiveRoot: HCERTSTORE,
    pub hExclusiveTrustedPeople: HCERTSTORE,
    pub dwExclusiveFlags: u32,
}
impl Default for CERT_CHAIN_ENGINE_CONFIG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_CHAIN_EXCLUSIVE_ENABLE_CA_FLAG: u32 = 1u32;
pub const CERT_CHAIN_FIND_BY_ISSUER: u32 = 1u32;
pub const CERT_CHAIN_FIND_BY_ISSUER_CACHE_ONLY_FLAG: CERT_FIND_CHAIN_IN_STORE_FLAGS = CERT_FIND_CHAIN_IN_STORE_FLAGS(32768u32);
pub const CERT_CHAIN_FIND_BY_ISSUER_CACHE_ONLY_URL_FLAG: CERT_FIND_CHAIN_IN_STORE_FLAGS = CERT_FIND_CHAIN_IN_STORE_FLAGS(4u32);
pub const CERT_CHAIN_FIND_BY_ISSUER_COMPARE_KEY_FLAG: CERT_FIND_CHAIN_IN_STORE_FLAGS = CERT_FIND_CHAIN_IN_STORE_FLAGS(1u32);
pub const CERT_CHAIN_FIND_BY_ISSUER_COMPLEX_CHAIN_FLAG: CERT_FIND_CHAIN_IN_STORE_FLAGS = CERT_FIND_CHAIN_IN_STORE_FLAGS(2u32);
pub const CERT_CHAIN_FIND_BY_ISSUER_LOCAL_MACHINE_FLAG: CERT_FIND_CHAIN_IN_STORE_FLAGS = CERT_FIND_CHAIN_IN_STORE_FLAGS(8u32);
pub const CERT_CHAIN_FIND_BY_ISSUER_NO_KEY_FLAG: CERT_FIND_CHAIN_IN_STORE_FLAGS = CERT_FIND_CHAIN_IN_STORE_FLAGS(16384u32);
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CERT_CHAIN_FIND_BY_ISSUER_PARA {
    pub cbSize: u32,
    pub pszUsageIdentifier: windows_core::PCSTR,
    pub dwKeySpec: u32,
    pub dwAcquirePrivateKeyFlags: u32,
    pub cIssuer: u32,
    pub rgIssuer: *mut CRYPT_INTEGER_BLOB,
    pub pfnFindCallback: PFN_CERT_CHAIN_FIND_BY_ISSUER_CALLBACK,
    pub pvFindArg: *mut core::ffi::c_void,
}
impl Default for CERT_CHAIN_FIND_BY_ISSUER_PARA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_CHAIN_HAS_MOTW: u32 = 16384u32;
pub const CERT_CHAIN_MAX_AIA_URL_COUNT_IN_CERT_DEFAULT: u32 = 5u32;
pub const CERT_CHAIN_MAX_AIA_URL_COUNT_IN_CERT_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("MaxAIAUrlCountInCert");
pub const CERT_CHAIN_MAX_AIA_URL_RETRIEVAL_BYTE_COUNT_DEFAULT: u32 = 100000u32;
pub const CERT_CHAIN_MAX_AIA_URL_RETRIEVAL_BYTE_COUNT_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("MaxAIAUrlRetrievalByteCount");
pub const CERT_CHAIN_MAX_AIA_URL_RETRIEVAL_CERT_COUNT_DEFAULT: u32 = 10u32;
pub const CERT_CHAIN_MAX_AIA_URL_RETRIEVAL_CERT_COUNT_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("MaxAIAUrlRetrievalCertCount");
pub const CERT_CHAIN_MAX_AIA_URL_RETRIEVAL_COUNT_PER_CHAIN_DEFAULT: u32 = 3u32;
pub const CERT_CHAIN_MAX_AIA_URL_RETRIEVAL_COUNT_PER_CHAIN_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("MaxAIAUrlRetrievalCountPerChain");
pub const CERT_CHAIN_MAX_SSL_TIME_UPDATED_EVENT_COUNT_DEFAULT: u32 = 5u32;
pub const CERT_CHAIN_MAX_SSL_TIME_UPDATED_EVENT_COUNT_DISABLE: u32 = 4294967295u32;
pub const CERT_CHAIN_MAX_SSL_TIME_UPDATED_EVENT_COUNT_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("MaxSslTimeUpdatedEventCount");
pub const CERT_CHAIN_MAX_URL_RETRIEVAL_BYTE_COUNT_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("MaxUrlRetrievalByteCount");
pub const CERT_CHAIN_MIN_PUB_KEY_BIT_LENGTH_DISABLE: u32 = 4294967295u32;
pub const CERT_CHAIN_MIN_RSA_PUB_KEY_BIT_LENGTH_DEFAULT: u32 = 1023u32;
pub const CERT_CHAIN_MIN_RSA_PUB_KEY_BIT_LENGTH_DISABLE: u32 = 4294967295u32;
pub const CERT_CHAIN_MIN_RSA_PUB_KEY_BIT_LENGTH_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("MinRsaPubKeyBitLength");
pub const CERT_CHAIN_MOTW_IGNORE_AFTER_TIME_WEAK_FLAG: u32 = 1073741824u32;
pub const CERT_CHAIN_OCSP_VALIDITY_SECONDS_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("OcspValiditySeconds");
pub const CERT_CHAIN_ONLY_ADDITIONAL_AND_AUTH_ROOT: u32 = 32768u32;
pub const CERT_CHAIN_OPTIONS_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("Options");
pub const CERT_CHAIN_OPTION_DISABLE_AIA_URL_RETRIEVAL: u32 = 2u32;
pub const CERT_CHAIN_OPTION_ENABLE_SIA_URL_RETRIEVAL: u32 = 4u32;
pub const CERT_CHAIN_OPT_IN_WEAK_FLAGS: u32 = 262144u32;
pub const CERT_CHAIN_OPT_IN_WEAK_SIGNATURE: u32 = 65536u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CERT_CHAIN_PARA {
    pub cbSize: u32,
    pub RequestedUsage: CERT_USAGE_MATCH,
    pub RequestedIssuancePolicy: CERT_USAGE_MATCH,
    pub dwUrlRetrievalTimeout: u32,
    pub fCheckRevocationFreshnessTime: windows_core::BOOL,
    pub dwRevocationFreshnessTime: u32,
    pub pftCacheResync: *mut super::super::Foundation::FILETIME,
    pub pStrongSignPara: *mut CERT_STRONG_SIGN_PARA,
    pub dwStrongSignFlags: u32,
}
impl Default for CERT_CHAIN_PARA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_CHAIN_POLICY_ALLOW_TESTROOT_FLAG: CERT_CHAIN_POLICY_FLAGS = CERT_CHAIN_POLICY_FLAGS(32768u32);
pub const CERT_CHAIN_POLICY_ALLOW_UNKNOWN_CA_FLAG: CERT_CHAIN_POLICY_FLAGS = CERT_CHAIN_POLICY_FLAGS(16u32);
pub const CERT_CHAIN_POLICY_AUTHENTICODE: windows_core::PCSTR = windows_core::PCSTR(2i32 as _);
pub const CERT_CHAIN_POLICY_AUTHENTICODE_TS: windows_core::PCSTR = windows_core::PCSTR(3i32 as _);
pub const CERT_CHAIN_POLICY_BASE: windows_core::PCSTR = windows_core::PCSTR(1i32 as _);
pub const CERT_CHAIN_POLICY_BASIC_CONSTRAINTS: windows_core::PCSTR = windows_core::PCSTR(5i32 as _);
pub const CERT_CHAIN_POLICY_EV: windows_core::PCSTR = windows_core::PCSTR(8i32 as _);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CERT_CHAIN_POLICY_FLAGS(pub u32);
impl CERT_CHAIN_POLICY_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CERT_CHAIN_POLICY_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CERT_CHAIN_POLICY_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CERT_CHAIN_POLICY_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CERT_CHAIN_POLICY_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CERT_CHAIN_POLICY_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CERT_CHAIN_POLICY_IGNORE_ALL_NOT_TIME_VALID_FLAGS: CERT_CHAIN_POLICY_FLAGS = CERT_CHAIN_POLICY_FLAGS(7u32);
pub const CERT_CHAIN_POLICY_IGNORE_ALL_REV_UNKNOWN_FLAGS: CERT_CHAIN_POLICY_FLAGS = CERT_CHAIN_POLICY_FLAGS(3840u32);
pub const CERT_CHAIN_POLICY_IGNORE_CA_REV_UNKNOWN_FLAG: CERT_CHAIN_POLICY_FLAGS = CERT_CHAIN_POLICY_FLAGS(1024u32);
pub const CERT_CHAIN_POLICY_IGNORE_CTL_NOT_TIME_VALID_FLAG: CERT_CHAIN_POLICY_FLAGS = CERT_CHAIN_POLICY_FLAGS(2u32);
pub const CERT_CHAIN_POLICY_IGNORE_CTL_SIGNER_REV_UNKNOWN_FLAG: CERT_CHAIN_POLICY_FLAGS = CERT_CHAIN_POLICY_FLAGS(512u32);
pub const CERT_CHAIN_POLICY_IGNORE_END_REV_UNKNOWN_FLAG: CERT_CHAIN_POLICY_FLAGS = CERT_CHAIN_POLICY_FLAGS(256u32);
pub const CERT_CHAIN_POLICY_IGNORE_INVALID_BASIC_CONSTRAINTS_FLAG: CERT_CHAIN_POLICY_FLAGS = CERT_CHAIN_POLICY_FLAGS(8u32);
pub const CERT_CHAIN_POLICY_IGNORE_INVALID_NAME_FLAG: CERT_CHAIN_POLICY_FLAGS = CERT_CHAIN_POLICY_FLAGS(64u32);
pub const CERT_CHAIN_POLICY_IGNORE_INVALID_POLICY_FLAG: CERT_CHAIN_POLICY_FLAGS = CERT_CHAIN_POLICY_FLAGS(128u32);
pub const CERT_CHAIN_POLICY_IGNORE_NOT_SUPPORTED_CRITICAL_EXT_FLAG: CERT_CHAIN_POLICY_FLAGS = CERT_CHAIN_POLICY_FLAGS(8192u32);
pub const CERT_CHAIN_POLICY_IGNORE_NOT_TIME_NESTED_FLAG: CERT_CHAIN_POLICY_FLAGS = CERT_CHAIN_POLICY_FLAGS(4u32);
pub const CERT_CHAIN_POLICY_IGNORE_NOT_TIME_VALID_FLAG: CERT_CHAIN_POLICY_FLAGS = CERT_CHAIN_POLICY_FLAGS(1u32);
pub const CERT_CHAIN_POLICY_IGNORE_PEER_TRUST_FLAG: CERT_CHAIN_POLICY_FLAGS = CERT_CHAIN_POLICY_FLAGS(4096u32);
pub const CERT_CHAIN_POLICY_IGNORE_ROOT_REV_UNKNOWN_FLAG: CERT_CHAIN_POLICY_FLAGS = CERT_CHAIN_POLICY_FLAGS(2048u32);
pub const CERT_CHAIN_POLICY_IGNORE_WEAK_SIGNATURE_FLAG: CERT_CHAIN_POLICY_FLAGS = CERT_CHAIN_POLICY_FLAGS(134217728u32);
pub const CERT_CHAIN_POLICY_IGNORE_WRONG_USAGE_FLAG: CERT_CHAIN_POLICY_FLAGS = CERT_CHAIN_POLICY_FLAGS(32u32);
pub const CERT_CHAIN_POLICY_MICROSOFT_ROOT: windows_core::PCSTR = windows_core::PCSTR(7i32 as _);
pub const CERT_CHAIN_POLICY_NT_AUTH: windows_core::PCSTR = windows_core::PCSTR(6i32 as _);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CERT_CHAIN_POLICY_PARA {
    pub cbSize: u32,
    pub dwFlags: CERT_CHAIN_POLICY_FLAGS,
    pub pvExtraPolicyPara: *mut core::ffi::c_void,
}
impl Default for CERT_CHAIN_POLICY_PARA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_CHAIN_POLICY_SSL: windows_core::PCSTR = windows_core::PCSTR(4i32 as _);
pub const CERT_CHAIN_POLICY_SSL_F12: windows_core::PCSTR = windows_core::PCSTR(9i32 as _);
pub const CERT_CHAIN_POLICY_SSL_F12_ERROR_LEVEL: u32 = 2u32;
pub const CERT_CHAIN_POLICY_SSL_F12_NONE_CATEGORY: u32 = 0u32;
pub const CERT_CHAIN_POLICY_SSL_F12_ROOT_PROGRAM_CATEGORY: u32 = 2u32;
pub const CERT_CHAIN_POLICY_SSL_F12_SUCCESS_LEVEL: u32 = 0u32;
pub const CERT_CHAIN_POLICY_SSL_F12_WARNING_LEVEL: u32 = 1u32;
pub const CERT_CHAIN_POLICY_SSL_F12_WEAK_CRYPTO_CATEGORY: u32 = 1u32;
pub const CERT_CHAIN_POLICY_SSL_HPKP_HEADER: windows_core::PCSTR = windows_core::PCSTR(10i32 as _);
pub const CERT_CHAIN_POLICY_SSL_KEY_PIN: windows_core::PCSTR = windows_core::PCSTR(12i32 as _);
pub const CERT_CHAIN_POLICY_SSL_KEY_PIN_MISMATCH_ERROR: i32 = -2i32;
pub const CERT_CHAIN_POLICY_SSL_KEY_PIN_MISMATCH_WARNING: u32 = 2u32;
pub const CERT_CHAIN_POLICY_SSL_KEY_PIN_MITM_ERROR: i32 = -1i32;
pub const CERT_CHAIN_POLICY_SSL_KEY_PIN_MITM_WARNING: u32 = 1u32;
pub const CERT_CHAIN_POLICY_SSL_KEY_PIN_SUCCESS: u32 = 0u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CERT_CHAIN_POLICY_STATUS {
    pub cbSize: u32,
    pub dwError: u32,
    pub lChainIndex: i32,
    pub lElementIndex: i32,
    pub pvExtraPolicyStatus: *mut core::ffi::c_void,
}
impl Default for CERT_CHAIN_POLICY_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_CHAIN_POLICY_THIRD_PARTY_ROOT: windows_core::PCSTR = windows_core::PCSTR(11i32 as _);
pub const CERT_CHAIN_POLICY_TRUST_TESTROOT_FLAG: CERT_CHAIN_POLICY_FLAGS = CERT_CHAIN_POLICY_FLAGS(16384u32);
pub const CERT_CHAIN_RETURN_LOWER_QUALITY_CONTEXTS: u32 = 128u32;
pub const CERT_CHAIN_REVOCATION_ACCUMULATIVE_TIMEOUT: u32 = 134217728u32;
pub const CERT_CHAIN_REVOCATION_CHECK_CACHE_ONLY: u32 = 2147483648u32;
pub const CERT_CHAIN_REVOCATION_CHECK_CHAIN: u32 = 536870912u32;
pub const CERT_CHAIN_REVOCATION_CHECK_CHAIN_EXCLUDE_ROOT: u32 = 1073741824u32;
pub const CERT_CHAIN_REVOCATION_CHECK_END_CERT: u32 = 268435456u32;
pub const CERT_CHAIN_REVOCATION_CHECK_OCSP_CERT: u32 = 67108864u32;
pub const CERT_CHAIN_REV_ACCUMULATIVE_URL_RETRIEVAL_TIMEOUT_MILLISECONDS_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("ChainRevAccumulativeUrlRetrievalTimeoutMilliseconds");
pub const CERT_CHAIN_SERIAL_CHAIN_LOG_FILE_NAME_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("SerialChainLogFileName");
pub const CERT_CHAIN_SSL_HANDSHAKE_LOG_FILE_NAME_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("SslHandshakeLogFileName");
pub const CERT_CHAIN_STRONG_SIGN_DISABLE_END_CHECK_FLAG: u32 = 1u32;
pub const CERT_CHAIN_THREAD_STORE_SYNC: u32 = 2u32;
pub const CERT_CHAIN_TIMESTAMP_TIME: u32 = 512u32;
pub const CERT_CHAIN_URL_RETRIEVAL_TIMEOUT_MILLISECONDS_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("ChainUrlRetrievalTimeoutMilliseconds");
pub const CERT_CHAIN_USE_LOCAL_MACHINE_STORE: u32 = 8u32;
pub const CERT_CHAIN_WEAK_AFTER_TIME_NAME: windows_core::PCWSTR = windows_core::w!("AfterTime");
pub const CERT_CHAIN_WEAK_ALL_CONFIG_NAME: windows_core::PCWSTR = windows_core::w!("All");
pub const CERT_CHAIN_WEAK_FILE_HASH_AFTER_TIME_NAME: windows_core::PCWSTR = windows_core::w!("FileHashAfterTime");
pub const CERT_CHAIN_WEAK_FLAGS_NAME: windows_core::PCWSTR = windows_core::w!("Flags");
pub const CERT_CHAIN_WEAK_HYGIENE_NAME: windows_core::PCWSTR = windows_core::w!("Hygiene");
pub const CERT_CHAIN_WEAK_MIN_BIT_LENGTH_NAME: windows_core::PCWSTR = windows_core::w!("MinBitLength");
pub const CERT_CHAIN_WEAK_PREFIX_NAME: windows_core::PCWSTR = windows_core::w!("Weak");
pub const CERT_CHAIN_WEAK_RSA_PUB_KEY_TIME_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("WeakRsaPubKeyTime");
pub const CERT_CHAIN_WEAK_SHA256_ALLOW_NAME: windows_core::PCWSTR = windows_core::w!("Sha256Allow");
pub const CERT_CHAIN_WEAK_SIGNATURE_LOG_DIR_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("WeakSignatureLogDir");
pub const CERT_CHAIN_WEAK_THIRD_PARTY_CONFIG_NAME: windows_core::PCWSTR = windows_core::w!("ThirdParty");
pub const CERT_CHAIN_WEAK_TIMESTAMP_HASH_AFTER_TIME_NAME: windows_core::PCWSTR = windows_core::w!("TimestampHashAfterTime");
pub const CERT_CLOSE_STORE_CHECK_FLAG: u32 = 2u32;
pub const CERT_CLOSE_STORE_FORCE_FLAG: u32 = 1u32;
pub const CERT_CLR_DELETE_KEY_PROP_ID: u32 = 125u32;
pub const CERT_COMPARE_ANY: u32 = 0u32;
pub const CERT_COMPARE_ATTR: u32 = 3u32;
pub const CERT_COMPARE_CERT_ID: u32 = 16u32;
pub const CERT_COMPARE_CROSS_CERT_DIST_POINTS: u32 = 17u32;
pub const CERT_COMPARE_CTL_USAGE: u32 = 10u32;
pub const CERT_COMPARE_ENHKEY_USAGE: u32 = 10u32;
pub const CERT_COMPARE_EXISTING: u32 = 13u32;
pub const CERT_COMPARE_HASH: u32 = 1u32;
pub const CERT_COMPARE_HASH_STR: u32 = 20u32;
pub const CERT_COMPARE_HAS_PRIVATE_KEY: u32 = 21u32;
pub const CERT_COMPARE_ISSUER_OF: u32 = 12u32;
pub const CERT_COMPARE_KEY_IDENTIFIER: u32 = 15u32;
pub const CERT_COMPARE_KEY_SPEC: u32 = 9u32;
pub const CERT_COMPARE_MASK: u32 = 65535u32;
pub const CERT_COMPARE_MD5_HASH: u32 = 4u32;
pub const CERT_COMPARE_NAME: u32 = 2u32;
pub const CERT_COMPARE_NAME_STR_A: u32 = 7u32;
pub const CERT_COMPARE_NAME_STR_W: u32 = 8u32;
pub const CERT_COMPARE_PROPERTY: u32 = 5u32;
pub const CERT_COMPARE_PUBKEY_MD5_HASH: u32 = 18u32;
pub const CERT_COMPARE_PUBLIC_KEY: u32 = 6u32;
pub const CERT_COMPARE_SHA1_HASH: u32 = 1u32;
pub const CERT_COMPARE_SHIFT: i32 = 16i32;
pub const CERT_COMPARE_SIGNATURE_HASH: u32 = 14u32;
pub const CERT_COMPARE_SUBJECT_CERT: u32 = 11u32;
pub const CERT_COMPARE_SUBJECT_INFO_ACCESS: u32 = 19u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CERT_CONTEXT {
    pub dwCertEncodingType: CERT_QUERY_ENCODING_TYPE,
    pub pbCertEncoded: *mut u8,
    pub cbCertEncoded: u32,
    pub pCertInfo: *mut CERT_INFO,
    pub hCertStore: HCERTSTORE,
}
impl Default for CERT_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_CONTEXT_REVOCATION_TYPE: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CERT_CONTROL_STORE_FLAGS(pub u32);
pub const CERT_CREATE_CONTEXT_NOCOPY_FLAG: u32 = 1u32;
pub const CERT_CREATE_CONTEXT_NO_ENTRY_FLAG: u32 = 8u32;
pub const CERT_CREATE_CONTEXT_NO_HCRYPTMSG_FLAG: u32 = 4u32;
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CERT_CREATE_CONTEXT_PARA {
    pub cbSize: u32,
    pub pfnFree: PFN_CRYPT_FREE,
    pub pvFree: *mut core::ffi::c_void,
    pub pfnSort: PFN_CERT_CREATE_CONTEXT_SORT_FUNC,
    pub pvSort: *mut core::ffi::c_void,
}
impl Default for CERT_CREATE_CONTEXT_PARA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_CREATE_CONTEXT_SORTED_FLAG: u32 = 2u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CERT_CREATE_SELFSIGN_FLAGS(pub u32);
impl CERT_CREATE_SELFSIGN_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CERT_CREATE_SELFSIGN_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CERT_CREATE_SELFSIGN_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CERT_CREATE_SELFSIGN_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CERT_CREATE_SELFSIGN_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CERT_CREATE_SELFSIGN_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CERT_CREATE_SELFSIGN_NO_KEY_INFO: CERT_CREATE_SELFSIGN_FLAGS = CERT_CREATE_SELFSIGN_FLAGS(2u32);
pub const CERT_CREATE_SELFSIGN_NO_SIGN: CERT_CREATE_SELFSIGN_FLAGS = CERT_CREATE_SELFSIGN_FLAGS(1u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CERT_CRL_CONTEXT_PAIR {
    pub pCertContext: *const CERT_CONTEXT,
    pub pCrlContext: *mut CRL_CONTEXT,
}
impl Default for CERT_CRL_CONTEXT_PAIR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_CRL_SIGN_KEY_USAGE: u32 = 2u32;
pub const CERT_CROSS_CERT_DIST_POINTS_PROP_ID: u32 = 23u32;
pub const CERT_CTL_USAGE_PROP_ID: u32 = 9u32;
pub const CERT_DATA_ENCIPHERMENT_KEY_USAGE: u32 = 16u32;
pub const CERT_DATE_STAMP_PROP_ID: u32 = 27u32;
pub const CERT_DECIPHER_ONLY_KEY_USAGE: u32 = 128u32;
pub const CERT_DEFAULT_OID_PUBLIC_KEY_SIGN: windows_core::PCWSTR = windows_core::w!("1.2.840.113549.1.1.1");
pub const CERT_DEFAULT_OID_PUBLIC_KEY_XCHG: windows_core::PCWSTR = windows_core::w!("1.2.840.113549.1.1.1");
pub const CERT_DESCRIPTION_PROP_ID: u32 = 13u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CERT_DH_PARAMETERS {
    pub p: CRYPT_INTEGER_BLOB,
    pub g: CRYPT_INTEGER_BLOB,
}
pub const CERT_DIGITAL_SIGNATURE_KEY_USAGE: u32 = 128u32;
pub const CERT_DISABLE_PIN_RULES_AUTO_UPDATE_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("DisablePinRulesAutoUpdate");
pub const CERT_DISABLE_ROOT_AUTO_UPDATE_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("DisableRootAutoUpdate");
pub const CERT_DISALLOWED_CA_FILETIME_PROP_ID: u32 = 128u32;
pub const CERT_DISALLOWED_CERT_AUTO_UPDATE_ENCODED_CTL_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("DisallowedCertEncodedCtl");
pub const CERT_DISALLOWED_CERT_AUTO_UPDATE_LAST_SYNC_TIME_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("DisallowedCertLastSyncTime");
pub const CERT_DISALLOWED_CERT_AUTO_UPDATE_LIST_IDENTIFIER: windows_core::PCWSTR = windows_core::w!("DisallowedCert_AutoUpdate_1");
pub const CERT_DISALLOWED_CERT_AUTO_UPDATE_SYNC_DELTA_TIME_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("DisallowedCertSyncDeltaTime");
pub const CERT_DISALLOWED_CERT_CAB_FILENAME: windows_core::PCWSTR = windows_core::w!("disallowedcertstl.cab");
pub const CERT_DISALLOWED_CERT_CTL_FILENAME: windows_core::PCWSTR = windows_core::w!("disallowedcert.stl");
pub const CERT_DISALLOWED_CERT_CTL_FILENAME_A: windows_core::PCSTR = windows_core::s!("disallowedcert.stl");
pub const CERT_DISALLOWED_ENHKEY_USAGE_PROP_ID: u32 = 122u32;
pub const CERT_DISALLOWED_FILETIME_PROP_ID: u32 = 104u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CERT_DSS_PARAMETERS {
    pub p: CRYPT_INTEGER_BLOB,
    pub q: CRYPT_INTEGER_BLOB,
    pub g: CRYPT_INTEGER_BLOB,
}
pub const CERT_DSS_R_LEN: u32 = 20u32;
pub const CERT_DSS_S_LEN: u32 = 20u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CERT_ECC_SIGNATURE {
    pub r: CRYPT_INTEGER_BLOB,
    pub s: CRYPT_INTEGER_BLOB,
}
pub const CERT_EFSBLOB_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("EFSBlob");
pub const CERT_EFS_PROP_ID: u32 = 17u32;
pub const CERT_ENABLE_DISALLOWED_CERT_AUTO_UPDATE_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("EnableDisallowedCertAutoUpdate");
pub const CERT_ENCIPHER_ONLY_KEY_USAGE: u32 = 1u32;
pub const CERT_ENCODING_TYPE_MASK: u32 = 65535u32;
pub const CERT_END_ENTITY_SUBJECT_FLAG: u32 = 64u32;
pub const CERT_ENHKEY_USAGE_PROP_ID: u32 = 9u32;
pub const CERT_ENROLLMENT_PROP_ID: u32 = 26u32;
pub const CERT_EXCLUDED_SUBTREE_BIT: i32 = -2147483648i32;
pub const CERT_EXTENDED_ERROR_INFO_PROP_ID: u32 = 30u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CERT_EXTENSION {
    pub pszObjId: windows_core::PSTR,
    pub fCritical: windows_core::BOOL,
    pub Value: CRYPT_INTEGER_BLOB,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CERT_EXTENSIONS {
    pub cExtension: u32,
    pub rgExtension: *mut CERT_EXTENSION,
}
impl Default for CERT_EXTENSIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_FILE_HASH_USE_TYPE: u32 = 1u32;
pub const CERT_FILE_STORE_COMMIT_ENABLE_FLAG: u32 = 65536u32;
pub const CERT_FIND_ANY: CERT_FIND_FLAGS = CERT_FIND_FLAGS(0u32);
pub const CERT_FIND_CERT_ID: CERT_FIND_FLAGS = CERT_FIND_FLAGS(1048576u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CERT_FIND_CHAIN_IN_STORE_FLAGS(pub u32);
impl CERT_FIND_CHAIN_IN_STORE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CERT_FIND_CHAIN_IN_STORE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CERT_FIND_CHAIN_IN_STORE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CERT_FIND_CHAIN_IN_STORE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CERT_FIND_CHAIN_IN_STORE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CERT_FIND_CHAIN_IN_STORE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CERT_FIND_CROSS_CERT_DIST_POINTS: CERT_FIND_FLAGS = CERT_FIND_FLAGS(1114112u32);
pub const CERT_FIND_CTL_USAGE: CERT_FIND_FLAGS = CERT_FIND_FLAGS(655360u32);
pub const CERT_FIND_ENHKEY_USAGE: CERT_FIND_FLAGS = CERT_FIND_FLAGS(655360u32);
pub const CERT_FIND_EXISTING: CERT_FIND_FLAGS = CERT_FIND_FLAGS(851968u32);
pub const CERT_FIND_EXT_ONLY_CTL_USAGE_FLAG: CERT_FIND_FLAGS = CERT_FIND_FLAGS(2u32);
pub const CERT_FIND_EXT_ONLY_ENHKEY_USAGE_FLAG: CERT_FIND_FLAGS = CERT_FIND_FLAGS(2u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CERT_FIND_FLAGS(pub u32);
impl CERT_FIND_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CERT_FIND_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CERT_FIND_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CERT_FIND_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CERT_FIND_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CERT_FIND_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CERT_FIND_HASH: CERT_FIND_FLAGS = CERT_FIND_FLAGS(65536u32);
pub const CERT_FIND_HASH_STR: CERT_FIND_FLAGS = CERT_FIND_FLAGS(1310720u32);
pub const CERT_FIND_HAS_PRIVATE_KEY: CERT_FIND_FLAGS = CERT_FIND_FLAGS(1376256u32);
pub const CERT_FIND_ISSUER_ATTR: CERT_FIND_FLAGS = CERT_FIND_FLAGS(196612u32);
pub const CERT_FIND_ISSUER_NAME: CERT_FIND_FLAGS = CERT_FIND_FLAGS(131076u32);
pub const CERT_FIND_ISSUER_OF: CERT_FIND_FLAGS = CERT_FIND_FLAGS(786432u32);
pub const CERT_FIND_ISSUER_STR: CERT_FIND_FLAGS = CERT_FIND_FLAGS(524292u32);
pub const CERT_FIND_ISSUER_STR_A: CERT_FIND_FLAGS = CERT_FIND_FLAGS(458756u32);
pub const CERT_FIND_ISSUER_STR_W: CERT_FIND_FLAGS = CERT_FIND_FLAGS(524292u32);
pub const CERT_FIND_KEY_IDENTIFIER: CERT_FIND_FLAGS = CERT_FIND_FLAGS(983040u32);
pub const CERT_FIND_KEY_SPEC: CERT_FIND_FLAGS = CERT_FIND_FLAGS(589824u32);
pub const CERT_FIND_MD5_HASH: CERT_FIND_FLAGS = CERT_FIND_FLAGS(262144u32);
pub const CERT_FIND_NO_CTL_USAGE_FLAG: CERT_FIND_FLAGS = CERT_FIND_FLAGS(8u32);
pub const CERT_FIND_NO_ENHKEY_USAGE_FLAG: CERT_FIND_FLAGS = CERT_FIND_FLAGS(8u32);
pub const CERT_FIND_OPTIONAL_CTL_USAGE_FLAG: CERT_FIND_FLAGS = CERT_FIND_FLAGS(1u32);
pub const CERT_FIND_OPTIONAL_ENHKEY_USAGE_FLAG: CERT_FIND_FLAGS = CERT_FIND_FLAGS(1u32);
pub const CERT_FIND_OR_CTL_USAGE_FLAG: CERT_FIND_FLAGS = CERT_FIND_FLAGS(16u32);
pub const CERT_FIND_OR_ENHKEY_USAGE_FLAG: CERT_FIND_FLAGS = CERT_FIND_FLAGS(16u32);
pub const CERT_FIND_PROPERTY: CERT_FIND_FLAGS = CERT_FIND_FLAGS(327680u32);
pub const CERT_FIND_PROP_ONLY_CTL_USAGE_FLAG: CERT_FIND_FLAGS = CERT_FIND_FLAGS(4u32);
pub const CERT_FIND_PROP_ONLY_ENHKEY_USAGE_FLAG: CERT_FIND_FLAGS = CERT_FIND_FLAGS(4u32);
pub const CERT_FIND_PUBKEY_MD5_HASH: CERT_FIND_FLAGS = CERT_FIND_FLAGS(1179648u32);
pub const CERT_FIND_PUBLIC_KEY: CERT_FIND_FLAGS = CERT_FIND_FLAGS(393216u32);
pub const CERT_FIND_SHA1_HASH: CERT_FIND_FLAGS = CERT_FIND_FLAGS(65536u32);
pub const CERT_FIND_SIGNATURE_HASH: CERT_FIND_FLAGS = CERT_FIND_FLAGS(917504u32);
pub const CERT_FIND_SUBJECT_ATTR: CERT_FIND_FLAGS = CERT_FIND_FLAGS(196615u32);
pub const CERT_FIND_SUBJECT_CERT: CERT_FIND_FLAGS = CERT_FIND_FLAGS(720896u32);
pub const CERT_FIND_SUBJECT_INFO_ACCESS: CERT_FIND_FLAGS = CERT_FIND_FLAGS(1245184u32);
pub const CERT_FIND_SUBJECT_NAME: CERT_FIND_FLAGS = CERT_FIND_FLAGS(131079u32);
pub const CERT_FIND_SUBJECT_STR: CERT_FIND_FLAGS = CERT_FIND_FLAGS(524295u32);
pub const CERT_FIND_SUBJECT_STR_A: CERT_FIND_FLAGS = CERT_FIND_FLAGS(458759u32);
pub const CERT_FIND_SUBJECT_STR_W: CERT_FIND_FLAGS = CERT_FIND_FLAGS(524295u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CERT_FIND_TYPE(pub u32);
pub const CERT_FIND_VALID_CTL_USAGE_FLAG: CERT_FIND_FLAGS = CERT_FIND_FLAGS(32u32);
pub const CERT_FIND_VALID_ENHKEY_USAGE_FLAG: CERT_FIND_FLAGS = CERT_FIND_FLAGS(32u32);
pub const CERT_FIRST_RESERVED_PROP_ID: u32 = 129u32;
pub const CERT_FIRST_USER_PROP_ID: u32 = 32768u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CERT_FORTEZZA_DATA_PROP {
    pub SerialNumber: [u8; 8],
    pub CertIndex: i32,
    pub CertLabel: [u8; 36],
}
impl Default for CERT_FORTEZZA_DATA_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_FORTEZZA_DATA_PROP_ID: u32 = 18u32;
pub const CERT_FRIENDLY_NAME_PROP_ID: u32 = 11u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_GENERAL_SUBTREE {
    pub Base: CERT_ALT_NAME_ENTRY,
    pub dwMinimum: u32,
    pub fMaximum: windows_core::BOOL,
    pub dwMaximum: u32,
}
impl Default for CERT_GENERAL_SUBTREE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_GROUP_POLICY_SYSTEM_STORE_REGPATH: windows_core::PCWSTR = windows_core::w!("Software\\Policies\\Microsoft\\SystemCertificates");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CERT_HASHED_URL {
    pub HashAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub Hash: CRYPT_INTEGER_BLOB,
    pub pwszUrl: windows_core::PWSTR,
}
pub const CERT_HASH_PROP_ID: u32 = 3u32;
pub const CERT_HCRYPTPROV_OR_NCRYPT_KEY_HANDLE_PROP_ID: u32 = 79u32;
pub const CERT_HCRYPTPROV_TRANSFER_PROP_ID: u32 = 100u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_ID {
    pub dwIdChoice: CERT_ID_OPTION,
    pub Anonymous: CERT_ID_0,
}
impl Default for CERT_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CERT_ID_0 {
    pub IssuerSerialNumber: CERT_ISSUER_SERIAL_NUMBER,
    pub KeyId: CRYPT_INTEGER_BLOB,
    pub HashId: CRYPT_INTEGER_BLOB,
}
impl Default for CERT_ID_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_ID_ISSUER_SERIAL_NUMBER: CERT_ID_OPTION = CERT_ID_OPTION(1u32);
pub const CERT_ID_KEY_IDENTIFIER: CERT_ID_OPTION = CERT_ID_OPTION(2u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CERT_ID_OPTION(pub u32);
pub const CERT_ID_SHA1_HASH: CERT_ID_OPTION = CERT_ID_OPTION(3u32);
pub const CERT_IE30_RESERVED_PROP_ID: u32 = 7u32;
pub const CERT_IE_DIRTY_FLAGS_REGPATH: windows_core::PCWSTR = windows_core::w!("Software\\Microsoft\\Cryptography\\IEDirtyFlags");
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CERT_INFO {
    pub dwVersion: u32,
    pub SerialNumber: CRYPT_INTEGER_BLOB,
    pub SignatureAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub Issuer: CRYPT_INTEGER_BLOB,
    pub NotBefore: super::super::Foundation::FILETIME,
    pub NotAfter: super::super::Foundation::FILETIME,
    pub Subject: CRYPT_INTEGER_BLOB,
    pub SubjectPublicKeyInfo: CERT_PUBLIC_KEY_INFO,
    pub IssuerUniqueId: CRYPT_BIT_BLOB,
    pub SubjectUniqueId: CRYPT_BIT_BLOB,
    pub cExtension: u32,
    pub rgExtension: *mut CERT_EXTENSION,
}
impl Default for CERT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_INFO_EXTENSION_FLAG: u32 = 11u32;
pub const CERT_INFO_ISSUER_FLAG: u32 = 4u32;
pub const CERT_INFO_ISSUER_UNIQUE_ID_FLAG: u32 = 9u32;
pub const CERT_INFO_NOT_AFTER_FLAG: u32 = 6u32;
pub const CERT_INFO_NOT_BEFORE_FLAG: u32 = 5u32;
pub const CERT_INFO_SERIAL_NUMBER_FLAG: u32 = 2u32;
pub const CERT_INFO_SIGNATURE_ALGORITHM_FLAG: u32 = 3u32;
pub const CERT_INFO_SUBJECT_FLAG: u32 = 7u32;
pub const CERT_INFO_SUBJECT_PUBLIC_KEY_INFO_FLAG: u32 = 8u32;
pub const CERT_INFO_SUBJECT_UNIQUE_ID_FLAG: u32 = 10u32;
pub const CERT_INFO_VERSION_FLAG: u32 = 1u32;
pub const CERT_ISOLATED_KEY_PROP_ID: u32 = 118u32;
pub const CERT_ISSUER_CHAIN_PUB_KEY_CNG_ALG_BIT_LENGTH_PROP_ID: u32 = 96u32;
pub const CERT_ISSUER_CHAIN_SIGN_HASH_CNG_ALG_PROP_ID: u32 = 95u32;
pub const CERT_ISSUER_PUBLIC_KEY_MD5_HASH_PROP_ID: u32 = 24u32;
pub const CERT_ISSUER_PUB_KEY_BIT_LENGTH_PROP_ID: u32 = 94u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CERT_ISSUER_SERIAL_NUMBER {
    pub Issuer: CRYPT_INTEGER_BLOB,
    pub SerialNumber: CRYPT_INTEGER_BLOB,
}
pub const CERT_ISSUER_SERIAL_NUMBER_MD5_HASH_PROP_ID: u32 = 28u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CERT_KEYGEN_REQUEST_INFO {
    pub dwVersion: u32,
    pub SubjectPublicKeyInfo: CERT_PUBLIC_KEY_INFO,
    pub pwszChallengeString: windows_core::PWSTR,
}
pub const CERT_KEYGEN_REQUEST_V1: u32 = 0u32;
pub const CERT_KEY_AGREEMENT_KEY_USAGE: u32 = 8u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CERT_KEY_ATTRIBUTES_INFO {
    pub KeyId: CRYPT_INTEGER_BLOB,
    pub IntendedKeyUsage: CRYPT_BIT_BLOB,
    pub pPrivateKeyUsagePeriod: *mut CERT_PRIVATE_KEY_VALIDITY,
}
impl Default for CERT_KEY_ATTRIBUTES_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_KEY_CERT_SIGN_KEY_USAGE: u32 = 4u32;
pub const CERT_KEY_CLASSIFICATION_PROP_ID: u32 = 120u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_KEY_CONTEXT {
    pub cbSize: u32,
    pub Anonymous: CERT_KEY_CONTEXT_0,
    pub dwKeySpec: u32,
}
impl Default for CERT_KEY_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CERT_KEY_CONTEXT_0 {
    pub hCryptProv: usize,
    pub hNCryptKey: NCRYPT_KEY_HANDLE,
}
impl Default for CERT_KEY_CONTEXT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_KEY_CONTEXT_PROP_ID: u32 = 5u32;
pub const CERT_KEY_ENCIPHERMENT_KEY_USAGE: u32 = 32u32;
pub const CERT_KEY_IDENTIFIER_PROP_ID: u32 = 20u32;
pub const CERT_KEY_PROV_HANDLE_PROP_ID: u32 = 1u32;
pub const CERT_KEY_PROV_INFO_PROP_ID: u32 = 2u32;
pub const CERT_KEY_REPAIR_ATTEMPTED_PROP_ID: u32 = 103u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CERT_KEY_SPEC(pub u32);
pub const CERT_KEY_SPEC_PROP_ID: u32 = 6u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CERT_KEY_USAGE_RESTRICTION_INFO {
    pub cCertPolicyId: u32,
    pub rgCertPolicyId: *mut CERT_POLICY_ID,
    pub RestrictedKeyUsage: CRYPT_BIT_BLOB,
}
impl Default for CERT_KEY_USAGE_RESTRICTION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_LAST_RESERVED_PROP_ID: u32 = 32767u32;
pub const CERT_LAST_USER_PROP_ID: u32 = 65535u32;
pub const CERT_LDAP_STORE_AREC_EXCLUSIVE_FLAG: u32 = 131072u32;
pub const CERT_LDAP_STORE_OPENED_FLAG: u32 = 262144u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CERT_LDAP_STORE_OPENED_PARA {
    pub pvLdapSessionHandle: *mut core::ffi::c_void,
    pub pwszLdapUrl: windows_core::PCWSTR,
}
impl Default for CERT_LDAP_STORE_OPENED_PARA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_LDAP_STORE_SIGN_FLAG: u32 = 65536u32;
pub const CERT_LDAP_STORE_UNBIND_FLAG: u32 = 524288u32;
pub const CERT_LOCAL_MACHINE_SYSTEM_STORE_REGPATH: windows_core::PCWSTR = windows_core::w!("Software\\Microsoft\\SystemCertificates");
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CERT_LOGOTYPE_AUDIO {
    pub LogotypeDetails: CERT_LOGOTYPE_DETAILS,
    pub pLogotypeAudioInfo: *mut CERT_LOGOTYPE_AUDIO_INFO,
}
impl Default for CERT_LOGOTYPE_AUDIO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CERT_LOGOTYPE_AUDIO_INFO {
    pub dwFileSize: u32,
    pub dwPlayTime: u32,
    pub dwChannels: u32,
    pub dwSampleRate: u32,
    pub pwszLanguage: windows_core::PWSTR,
}
pub const CERT_LOGOTYPE_BITS_IMAGE_RESOLUTION_CHOICE: CERT_LOGOTYPE_CHOICE = CERT_LOGOTYPE_CHOICE(1u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CERT_LOGOTYPE_CHOICE(pub u32);
pub const CERT_LOGOTYPE_COLOR_IMAGE_INFO_CHOICE: CERT_LOGOTYPE_IMAGE_INFO_TYPE = CERT_LOGOTYPE_IMAGE_INFO_TYPE(2u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CERT_LOGOTYPE_DATA {
    pub cLogotypeImage: u32,
    pub rgLogotypeImage: *mut CERT_LOGOTYPE_IMAGE,
    pub cLogotypeAudio: u32,
    pub rgLogotypeAudio: *mut CERT_LOGOTYPE_AUDIO,
}
impl Default for CERT_LOGOTYPE_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CERT_LOGOTYPE_DETAILS {
    pub pwszMimeType: windows_core::PWSTR,
    pub cHashedUrl: u32,
    pub rgHashedUrl: *mut CERT_HASHED_URL,
}
impl Default for CERT_LOGOTYPE_DETAILS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_LOGOTYPE_DIRECT_INFO_CHOICE: CERT_LOGOTYPE_OPTION = CERT_LOGOTYPE_OPTION(1u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CERT_LOGOTYPE_EXT_INFO {
    pub cCommunityLogo: u32,
    pub rgCommunityLogo: *mut CERT_LOGOTYPE_INFO,
    pub pIssuerLogo: *mut CERT_LOGOTYPE_INFO,
    pub pSubjectLogo: *mut CERT_LOGOTYPE_INFO,
    pub cOtherLogo: u32,
    pub rgOtherLogo: *mut CERT_OTHER_LOGOTYPE_INFO,
}
impl Default for CERT_LOGOTYPE_EXT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_LOGOTYPE_GRAY_SCALE_IMAGE_INFO_CHOICE: CERT_LOGOTYPE_IMAGE_INFO_TYPE = CERT_LOGOTYPE_IMAGE_INFO_TYPE(1u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CERT_LOGOTYPE_IMAGE {
    pub LogotypeDetails: CERT_LOGOTYPE_DETAILS,
    pub pLogotypeImageInfo: *mut CERT_LOGOTYPE_IMAGE_INFO,
}
impl Default for CERT_LOGOTYPE_IMAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_LOGOTYPE_IMAGE_INFO {
    pub dwLogotypeImageInfoChoice: CERT_LOGOTYPE_IMAGE_INFO_TYPE,
    pub dwFileSize: u32,
    pub dwXSize: u32,
    pub dwYSize: u32,
    pub dwLogotypeImageResolutionChoice: CERT_LOGOTYPE_CHOICE,
    pub Anonymous: CERT_LOGOTYPE_IMAGE_INFO_0,
    pub pwszLanguage: windows_core::PWSTR,
}
impl Default for CERT_LOGOTYPE_IMAGE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CERT_LOGOTYPE_IMAGE_INFO_0 {
    pub dwNumBits: u32,
    pub dwTableSize: u32,
}
impl Default for CERT_LOGOTYPE_IMAGE_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CERT_LOGOTYPE_IMAGE_INFO_TYPE(pub u32);
pub const CERT_LOGOTYPE_INDIRECT_INFO_CHOICE: CERT_LOGOTYPE_OPTION = CERT_LOGOTYPE_OPTION(2u32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_LOGOTYPE_INFO {
    pub dwLogotypeInfoChoice: CERT_LOGOTYPE_OPTION,
    pub Anonymous: CERT_LOGOTYPE_INFO_0,
}
impl Default for CERT_LOGOTYPE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CERT_LOGOTYPE_INFO_0 {
    pub pLogotypeDirectInfo: *mut CERT_LOGOTYPE_DATA,
    pub pLogotypeIndirectInfo: *mut CERT_LOGOTYPE_REFERENCE,
}
impl Default for CERT_LOGOTYPE_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_LOGOTYPE_NO_IMAGE_RESOLUTION_CHOICE: CERT_LOGOTYPE_CHOICE = CERT_LOGOTYPE_CHOICE(0u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CERT_LOGOTYPE_OPTION(pub u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CERT_LOGOTYPE_REFERENCE {
    pub cHashedUrl: u32,
    pub rgHashedUrl: *mut CERT_HASHED_URL,
}
impl Default for CERT_LOGOTYPE_REFERENCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_LOGOTYPE_TABLE_SIZE_IMAGE_RESOLUTION_CHOICE: CERT_LOGOTYPE_CHOICE = CERT_LOGOTYPE_CHOICE(2u32);
pub const CERT_MD5_HASH_PROP_ID: u32 = 4u32;
pub const CERT_NAME_ATTR_TYPE: u32 = 3u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CERT_NAME_CONSTRAINTS_INFO {
    pub cPermittedSubtree: u32,
    pub rgPermittedSubtree: *mut CERT_GENERAL_SUBTREE,
    pub cExcludedSubtree: u32,
    pub rgExcludedSubtree: *mut CERT_GENERAL_SUBTREE,
}
impl Default for CERT_NAME_CONSTRAINTS_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_NAME_DISABLE_IE4_UTF8_FLAG: u32 = 65536u32;
pub const CERT_NAME_DNS_TYPE: u32 = 6u32;
pub const CERT_NAME_EMAIL_TYPE: u32 = 1u32;
pub const CERT_NAME_FRIENDLY_DISPLAY_TYPE: u32 = 5u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CERT_NAME_INFO {
    pub cRDN: u32,
    pub rgRDN: *mut CERT_RDN,
}
impl Default for CERT_NAME_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_NAME_ISSUER_FLAG: u32 = 1u32;
pub const CERT_NAME_RDN_TYPE: u32 = 2u32;
pub const CERT_NAME_SEARCH_ALL_NAMES_FLAG: u32 = 2u32;
pub const CERT_NAME_SIMPLE_DISPLAY_TYPE: u32 = 4u32;
pub const CERT_NAME_STR_COMMA_FLAG: u32 = 67108864u32;
pub const CERT_NAME_STR_CRLF_FLAG: u32 = 134217728u32;
pub const CERT_NAME_STR_DISABLE_IE4_UTF8_FLAG: u32 = 65536u32;
pub const CERT_NAME_STR_DISABLE_UTF8_DIR_STR_FLAG: u32 = 1048576u32;
pub const CERT_NAME_STR_ENABLE_PUNYCODE_FLAG: u32 = 2097152u32;
pub const CERT_NAME_STR_ENABLE_T61_UNICODE_FLAG: u32 = 131072u32;
pub const CERT_NAME_STR_ENABLE_UTF8_UNICODE_FLAG: u32 = 262144u32;
pub const CERT_NAME_STR_FORCE_UTF8_DIR_STR_FLAG: u32 = 524288u32;
pub const CERT_NAME_STR_FORWARD_FLAG: u32 = 16777216u32;
pub const CERT_NAME_STR_NO_PLUS_FLAG: u32 = 536870912u32;
pub const CERT_NAME_STR_NO_QUOTING_FLAG: u32 = 268435456u32;
pub const CERT_NAME_STR_REVERSE_FLAG: u32 = 33554432u32;
pub const CERT_NAME_STR_SEMICOLON_FLAG: u32 = 1073741824u32;
pub const CERT_NAME_UPN_TYPE: u32 = 8u32;
pub const CERT_NAME_URL_TYPE: u32 = 7u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CERT_NAME_VALUE {
    pub dwValueType: u32,
    pub Value: CRYPT_INTEGER_BLOB,
}
pub const CERT_NCRYPT_KEY_HANDLE_PROP_ID: u32 = 78u32;
pub const CERT_NCRYPT_KEY_HANDLE_TRANSFER_PROP_ID: u32 = 99u32;
pub const CERT_NCRYPT_KEY_SPEC: CERT_KEY_SPEC = CERT_KEY_SPEC(4294967295u32);
pub const CERT_NEW_KEY_PROP_ID: u32 = 74u32;
pub const CERT_NEXT_UPDATE_LOCATION_PROP_ID: u32 = 10u32;
pub const CERT_NONCOMPLIANT_ROOT_URL_PROP_ID: u32 = 123u32;
pub const CERT_NON_REPUDIATION_KEY_USAGE: u32 = 64u32;
pub const CERT_NOT_BEFORE_ENHKEY_USAGE_PROP_ID: u32 = 127u32;
pub const CERT_NOT_BEFORE_FILETIME_PROP_ID: u32 = 126u32;
pub const CERT_NO_AUTO_EXPIRE_CHECK_PROP_ID: u32 = 77u32;
pub const CERT_NO_EXPIRE_NOTIFICATION_PROP_ID: u32 = 97u32;
pub const CERT_OCM_SUBCOMPONENTS_LOCAL_MACHINE_REGPATH: windows_core::PCWSTR = windows_core::w!("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Setup\\OC Manager\\Subcomponents");
pub const CERT_OCM_SUBCOMPONENTS_ROOT_AUTO_UPDATE_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("RootAutoUpdate");
pub const CERT_OCSP_CACHE_PREFIX_PROP_ID: u32 = 75u32;
pub const CERT_OCSP_MUST_STAPLE_PROP_ID: u32 = 121u32;
pub const CERT_OCSP_RESPONSE_PROP_ID: u32 = 70u32;
pub const CERT_OFFLINE_CRL_SIGN_KEY_USAGE: u32 = 2u32;
pub const CERT_OID_NAME_STR: CERT_STRING_TYPE = CERT_STRING_TYPE(2u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CERT_OPEN_STORE_FLAGS(pub u32);
impl CERT_OPEN_STORE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CERT_OPEN_STORE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CERT_OPEN_STORE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CERT_OPEN_STORE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CERT_OPEN_STORE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CERT_OPEN_STORE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CERT_OR_CRL_BLOB {
    pub dwChoice: u32,
    pub cbEncoded: u32,
    pub pbEncoded: *mut u8,
}
impl Default for CERT_OR_CRL_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CERT_OR_CRL_BUNDLE {
    pub cItem: u32,
    pub rgItem: *mut CERT_OR_CRL_BLOB,
}
impl Default for CERT_OR_CRL_BUNDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_OTHER_LOGOTYPE_INFO {
    pub pszObjId: windows_core::PSTR,
    pub LogotypeInfo: CERT_LOGOTYPE_INFO,
}
impl Default for CERT_OTHER_LOGOTYPE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CERT_OTHER_NAME {
    pub pszObjId: windows_core::PSTR,
    pub Value: CRYPT_INTEGER_BLOB,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CERT_PAIR {
    pub Forward: CRYPT_INTEGER_BLOB,
    pub Reverse: CRYPT_INTEGER_BLOB,
}
pub const CERT_PHYSICAL_STORE_ADD_ENABLE_FLAG: u32 = 1u32;
pub const CERT_PHYSICAL_STORE_AUTH_ROOT_NAME: windows_core::PCWSTR = windows_core::w!(".AuthRoot");
pub const CERT_PHYSICAL_STORE_DEFAULT_NAME: windows_core::PCWSTR = windows_core::w!(".Default");
pub const CERT_PHYSICAL_STORE_DS_USER_CERTIFICATE_NAME: windows_core::PCWSTR = windows_core::w!(".UserCertificate");
pub const CERT_PHYSICAL_STORE_ENTERPRISE_NAME: windows_core::PCWSTR = windows_core::w!(".Enterprise");
pub const CERT_PHYSICAL_STORE_GROUP_POLICY_NAME: windows_core::PCWSTR = windows_core::w!(".GroupPolicy");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CERT_PHYSICAL_STORE_INFO {
    pub cbSize: u32,
    pub pszOpenStoreProvider: windows_core::PSTR,
    pub dwOpenEncodingType: u32,
    pub dwOpenFlags: u32,
    pub OpenParameters: CRYPT_INTEGER_BLOB,
    pub dwFlags: u32,
    pub dwPriority: u32,
}
pub const CERT_PHYSICAL_STORE_INSERT_COMPUTER_NAME_ENABLE_FLAG: u32 = 8u32;
pub const CERT_PHYSICAL_STORE_LOCAL_MACHINE_GROUP_POLICY_NAME: windows_core::PCWSTR = windows_core::w!(".LocalMachineGroupPolicy");
pub const CERT_PHYSICAL_STORE_LOCAL_MACHINE_NAME: windows_core::PCWSTR = windows_core::w!(".LocalMachine");
pub const CERT_PHYSICAL_STORE_OPEN_DISABLE_FLAG: u32 = 2u32;
pub const CERT_PHYSICAL_STORE_PREDEFINED_ENUM_FLAG: u32 = 1u32;
pub const CERT_PHYSICAL_STORE_REMOTE_OPEN_DISABLE_FLAG: u32 = 4u32;
pub const CERT_PHYSICAL_STORE_SMART_CARD_NAME: windows_core::PCWSTR = windows_core::w!(".SmartCard");
pub const CERT_PIN_RULES_AUTO_UPDATE_ENCODED_CTL_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("PinRulesEncodedCtl");
pub const CERT_PIN_RULES_AUTO_UPDATE_LAST_SYNC_TIME_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("PinRulesLastSyncTime");
pub const CERT_PIN_RULES_AUTO_UPDATE_LIST_IDENTIFIER: windows_core::PCWSTR = windows_core::w!("PinRules_AutoUpdate_1");
pub const CERT_PIN_RULES_AUTO_UPDATE_SYNC_DELTA_TIME_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("PinRulesSyncDeltaTime");
pub const CERT_PIN_RULES_CAB_FILENAME: windows_core::PCWSTR = windows_core::w!("pinrulesstl.cab");
pub const CERT_PIN_RULES_CTL_FILENAME: windows_core::PCWSTR = windows_core::w!("pinrules.stl");
pub const CERT_PIN_RULES_CTL_FILENAME_A: windows_core::PCSTR = windows_core::s!("pinrules.stl");
pub const CERT_PIN_SHA256_HASH_PROP_ID: u32 = 124u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CERT_POLICIES_INFO {
    pub cPolicyInfo: u32,
    pub rgPolicyInfo: *mut CERT_POLICY_INFO,
}
impl Default for CERT_POLICIES_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CERT_POLICY95_QUALIFIER1 {
    pub pszPracticesReference: windows_core::PWSTR,
    pub pszNoticeIdentifier: windows_core::PSTR,
    pub pszNSINoticeIdentifier: windows_core::PSTR,
    pub cCPSURLs: u32,
    pub rgCPSURLs: *mut CPS_URLS,
}
impl Default for CERT_POLICY95_QUALIFIER1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CERT_POLICY_CONSTRAINTS_INFO {
    pub fRequireExplicitPolicy: windows_core::BOOL,
    pub dwRequireExplicitPolicySkipCerts: u32,
    pub fInhibitPolicyMapping: windows_core::BOOL,
    pub dwInhibitPolicyMappingSkipCerts: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CERT_POLICY_ID {
    pub cCertPolicyElementId: u32,
    pub rgpszCertPolicyElementId: *mut windows_core::PSTR,
}
impl Default for CERT_POLICY_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CERT_POLICY_INFO {
    pub pszPolicyIdentifier: windows_core::PSTR,
    pub cPolicyQualifier: u32,
    pub rgPolicyQualifier: *mut CERT_POLICY_QUALIFIER_INFO,
}
impl Default for CERT_POLICY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CERT_POLICY_MAPPING {
    pub pszIssuerDomainPolicy: windows_core::PSTR,
    pub pszSubjectDomainPolicy: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CERT_POLICY_MAPPINGS_INFO {
    pub cPolicyMapping: u32,
    pub rgPolicyMapping: *mut CERT_POLICY_MAPPING,
}
impl Default for CERT_POLICY_MAPPINGS_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CERT_POLICY_QUALIFIER_INFO {
    pub pszPolicyQualifierId: windows_core::PSTR,
    pub Qualifier: CRYPT_INTEGER_BLOB,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CERT_POLICY_QUALIFIER_NOTICE_REFERENCE {
    pub pszOrganization: windows_core::PSTR,
    pub cNoticeNumbers: u32,
    pub rgNoticeNumbers: *mut i32,
}
impl Default for CERT_POLICY_QUALIFIER_NOTICE_REFERENCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CERT_POLICY_QUALIFIER_USER_NOTICE {
    pub pNoticeReference: *mut CERT_POLICY_QUALIFIER_NOTICE_REFERENCE,
    pub pszDisplayText: windows_core::PWSTR,
}
impl Default for CERT_POLICY_QUALIFIER_USER_NOTICE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CERT_PRIVATE_KEY_VALIDITY {
    pub NotBefore: super::super::Foundation::FILETIME,
    pub NotAfter: super::super::Foundation::FILETIME,
}
pub const CERT_PROT_ROOT_DISABLE_CURRENT_USER_FLAG: u32 = 1u32;
pub const CERT_PROT_ROOT_DISABLE_LM_AUTH_FLAG: u32 = 8u32;
pub const CERT_PROT_ROOT_DISABLE_NOT_DEFINED_NAME_CONSTRAINT_FLAG: u32 = 32u32;
pub const CERT_PROT_ROOT_DISABLE_NT_AUTH_REQUIRED_FLAG: u32 = 16u32;
pub const CERT_PROT_ROOT_DISABLE_PEER_TRUST: u32 = 65536u32;
pub const CERT_PROT_ROOT_FLAGS_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("Flags");
pub const CERT_PROT_ROOT_INHIBIT_ADD_AT_INIT_FLAG: u32 = 2u32;
pub const CERT_PROT_ROOT_INHIBIT_PURGE_LM_FLAG: u32 = 4u32;
pub const CERT_PROT_ROOT_ONLY_LM_GPT_FLAG: u32 = 8u32;
pub const CERT_PROT_ROOT_PEER_USAGES_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("PeerUsages");
pub const CERT_PROT_ROOT_PEER_USAGES_VALUE_NAME_A: windows_core::PCSTR = windows_core::s!("PeerUsages");
pub const CERT_PUBKEY_ALG_PARA_PROP_ID: u32 = 22u32;
pub const CERT_PUBKEY_HASH_RESERVED_PROP_ID: u32 = 8u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CERT_PUBLIC_KEY_INFO {
    pub Algorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub PublicKey: CRYPT_BIT_BLOB,
}
pub const CERT_PUB_KEY_CNG_ALG_BIT_LENGTH_PROP_ID: u32 = 93u32;
pub const CERT_PVK_FILE_PROP_ID: u32 = 12u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CERT_QC_STATEMENT {
    pub pszStatementId: windows_core::PSTR,
    pub StatementInfo: CRYPT_INTEGER_BLOB,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CERT_QC_STATEMENTS_EXT_INFO {
    pub cStatement: u32,
    pub rgStatement: *mut CERT_QC_STATEMENT,
}
impl Default for CERT_QC_STATEMENTS_EXT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_QUERY_CONTENT_CERT: CERT_QUERY_CONTENT_TYPE = CERT_QUERY_CONTENT_TYPE(1u32);
pub const CERT_QUERY_CONTENT_CERT_PAIR: CERT_QUERY_CONTENT_TYPE = CERT_QUERY_CONTENT_TYPE(13u32);
pub const CERT_QUERY_CONTENT_CRL: CERT_QUERY_CONTENT_TYPE = CERT_QUERY_CONTENT_TYPE(3u32);
pub const CERT_QUERY_CONTENT_CTL: CERT_QUERY_CONTENT_TYPE = CERT_QUERY_CONTENT_TYPE(2u32);
pub const CERT_QUERY_CONTENT_FLAG_ALL: CERT_QUERY_CONTENT_TYPE_FLAGS = CERT_QUERY_CONTENT_TYPE_FLAGS(16382u32);
pub const CERT_QUERY_CONTENT_FLAG_ALL_ISSUER_CERT: CERT_QUERY_CONTENT_TYPE_FLAGS = CERT_QUERY_CONTENT_TYPE_FLAGS(818u32);
pub const CERT_QUERY_CONTENT_FLAG_CERT: CERT_QUERY_CONTENT_TYPE_FLAGS = CERT_QUERY_CONTENT_TYPE_FLAGS(2u32);
pub const CERT_QUERY_CONTENT_FLAG_CERT_PAIR: CERT_QUERY_CONTENT_TYPE_FLAGS = CERT_QUERY_CONTENT_TYPE_FLAGS(8192u32);
pub const CERT_QUERY_CONTENT_FLAG_CRL: CERT_QUERY_CONTENT_TYPE_FLAGS = CERT_QUERY_CONTENT_TYPE_FLAGS(8u32);
pub const CERT_QUERY_CONTENT_FLAG_CTL: CERT_QUERY_CONTENT_TYPE_FLAGS = CERT_QUERY_CONTENT_TYPE_FLAGS(4u32);
pub const CERT_QUERY_CONTENT_FLAG_PFX: CERT_QUERY_CONTENT_TYPE_FLAGS = CERT_QUERY_CONTENT_TYPE_FLAGS(4096u32);
pub const CERT_QUERY_CONTENT_FLAG_PFX_AND_LOAD: CERT_QUERY_CONTENT_TYPE_FLAGS = CERT_QUERY_CONTENT_TYPE_FLAGS(16384u32);
pub const CERT_QUERY_CONTENT_FLAG_PKCS10: CERT_QUERY_CONTENT_TYPE_FLAGS = CERT_QUERY_CONTENT_TYPE_FLAGS(2048u32);
pub const CERT_QUERY_CONTENT_FLAG_PKCS7_SIGNED: CERT_QUERY_CONTENT_TYPE_FLAGS = CERT_QUERY_CONTENT_TYPE_FLAGS(256u32);
pub const CERT_QUERY_CONTENT_FLAG_PKCS7_SIGNED_EMBED: CERT_QUERY_CONTENT_TYPE_FLAGS = CERT_QUERY_CONTENT_TYPE_FLAGS(1024u32);
pub const CERT_QUERY_CONTENT_FLAG_PKCS7_UNSIGNED: CERT_QUERY_CONTENT_TYPE_FLAGS = CERT_QUERY_CONTENT_TYPE_FLAGS(512u32);
pub const CERT_QUERY_CONTENT_FLAG_SERIALIZED_CERT: CERT_QUERY_CONTENT_TYPE_FLAGS = CERT_QUERY_CONTENT_TYPE_FLAGS(32u32);
pub const CERT_QUERY_CONTENT_FLAG_SERIALIZED_CRL: CERT_QUERY_CONTENT_TYPE_FLAGS = CERT_QUERY_CONTENT_TYPE_FLAGS(128u32);
pub const CERT_QUERY_CONTENT_FLAG_SERIALIZED_CTL: CERT_QUERY_CONTENT_TYPE_FLAGS = CERT_QUERY_CONTENT_TYPE_FLAGS(64u32);
pub const CERT_QUERY_CONTENT_FLAG_SERIALIZED_STORE: CERT_QUERY_CONTENT_TYPE_FLAGS = CERT_QUERY_CONTENT_TYPE_FLAGS(16u32);
pub const CERT_QUERY_CONTENT_PFX: CERT_QUERY_CONTENT_TYPE = CERT_QUERY_CONTENT_TYPE(12u32);
pub const CERT_QUERY_CONTENT_PFX_AND_LOAD: CERT_QUERY_CONTENT_TYPE = CERT_QUERY_CONTENT_TYPE(14u32);
pub const CERT_QUERY_CONTENT_PKCS10: CERT_QUERY_CONTENT_TYPE = CERT_QUERY_CONTENT_TYPE(11u32);
pub const CERT_QUERY_CONTENT_PKCS7_SIGNED: CERT_QUERY_CONTENT_TYPE = CERT_QUERY_CONTENT_TYPE(8u32);
pub const CERT_QUERY_CONTENT_PKCS7_SIGNED_EMBED: CERT_QUERY_CONTENT_TYPE = CERT_QUERY_CONTENT_TYPE(10u32);
pub const CERT_QUERY_CONTENT_PKCS7_UNSIGNED: CERT_QUERY_CONTENT_TYPE = CERT_QUERY_CONTENT_TYPE(9u32);
pub const CERT_QUERY_CONTENT_SERIALIZED_CERT: CERT_QUERY_CONTENT_TYPE = CERT_QUERY_CONTENT_TYPE(5u32);
pub const CERT_QUERY_CONTENT_SERIALIZED_CRL: CERT_QUERY_CONTENT_TYPE = CERT_QUERY_CONTENT_TYPE(7u32);
pub const CERT_QUERY_CONTENT_SERIALIZED_CTL: CERT_QUERY_CONTENT_TYPE = CERT_QUERY_CONTENT_TYPE(6u32);
pub const CERT_QUERY_CONTENT_SERIALIZED_STORE: CERT_QUERY_CONTENT_TYPE = CERT_QUERY_CONTENT_TYPE(4u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CERT_QUERY_CONTENT_TYPE(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CERT_QUERY_CONTENT_TYPE_FLAGS(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CERT_QUERY_ENCODING_TYPE(pub u32);
impl CERT_QUERY_ENCODING_TYPE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CERT_QUERY_ENCODING_TYPE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CERT_QUERY_ENCODING_TYPE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CERT_QUERY_ENCODING_TYPE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CERT_QUERY_ENCODING_TYPE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CERT_QUERY_ENCODING_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CERT_QUERY_FORMAT_ASN_ASCII_HEX_ENCODED: CERT_QUERY_FORMAT_TYPE = CERT_QUERY_FORMAT_TYPE(3u32);
pub const CERT_QUERY_FORMAT_BASE64_ENCODED: CERT_QUERY_FORMAT_TYPE = CERT_QUERY_FORMAT_TYPE(2u32);
pub const CERT_QUERY_FORMAT_BINARY: CERT_QUERY_FORMAT_TYPE = CERT_QUERY_FORMAT_TYPE(1u32);
pub const CERT_QUERY_FORMAT_FLAG_ALL: CERT_QUERY_FORMAT_TYPE_FLAGS = CERT_QUERY_FORMAT_TYPE_FLAGS(14u32);
pub const CERT_QUERY_FORMAT_FLAG_ASN_ASCII_HEX_ENCODED: CERT_QUERY_FORMAT_TYPE_FLAGS = CERT_QUERY_FORMAT_TYPE_FLAGS(8u32);
pub const CERT_QUERY_FORMAT_FLAG_BASE64_ENCODED: CERT_QUERY_FORMAT_TYPE_FLAGS = CERT_QUERY_FORMAT_TYPE_FLAGS(4u32);
pub const CERT_QUERY_FORMAT_FLAG_BINARY: CERT_QUERY_FORMAT_TYPE_FLAGS = CERT_QUERY_FORMAT_TYPE_FLAGS(2u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CERT_QUERY_FORMAT_TYPE(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CERT_QUERY_FORMAT_TYPE_FLAGS(pub u32);
pub const CERT_QUERY_OBJECT_BLOB: CERT_QUERY_OBJECT_TYPE = CERT_QUERY_OBJECT_TYPE(2u32);
pub const CERT_QUERY_OBJECT_FILE: CERT_QUERY_OBJECT_TYPE = CERT_QUERY_OBJECT_TYPE(1u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CERT_QUERY_OBJECT_TYPE(pub u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CERT_RDN {
    pub cRDNAttr: u32,
    pub rgRDNAttr: *mut CERT_RDN_ATTR,
}
impl Default for CERT_RDN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_RDN_ANY_TYPE: CERT_RDN_ATTR_VALUE_TYPE = CERT_RDN_ATTR_VALUE_TYPE(0i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CERT_RDN_ATTR {
    pub pszObjId: windows_core::PSTR,
    pub dwValueType: u32,
    pub Value: CRYPT_INTEGER_BLOB,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CERT_RDN_ATTR_VALUE_TYPE(pub i32);
pub const CERT_RDN_BMP_STRING: CERT_RDN_ATTR_VALUE_TYPE = CERT_RDN_ATTR_VALUE_TYPE(12i32);
pub const CERT_RDN_DISABLE_CHECK_TYPE_FLAG: u32 = 1073741824u32;
pub const CERT_RDN_DISABLE_IE4_UTF8_FLAG: u32 = 16777216u32;
pub const CERT_RDN_ENABLE_PUNYCODE_FLAG: u32 = 33554432u32;
pub const CERT_RDN_ENABLE_T61_UNICODE_FLAG: u32 = 2147483648u32;
pub const CERT_RDN_ENABLE_UTF8_UNICODE_FLAG: u32 = 536870912u32;
pub const CERT_RDN_ENCODED_BLOB: CERT_RDN_ATTR_VALUE_TYPE = CERT_RDN_ATTR_VALUE_TYPE(1i32);
pub const CERT_RDN_FLAGS_MASK: u32 = 4278190080u32;
pub const CERT_RDN_FORCE_UTF8_UNICODE_FLAG: u32 = 268435456u32;
pub const CERT_RDN_GENERAL_STRING: CERT_RDN_ATTR_VALUE_TYPE = CERT_RDN_ATTR_VALUE_TYPE(10i32);
pub const CERT_RDN_GRAPHIC_STRING: CERT_RDN_ATTR_VALUE_TYPE = CERT_RDN_ATTR_VALUE_TYPE(8i32);
pub const CERT_RDN_IA5_STRING: CERT_RDN_ATTR_VALUE_TYPE = CERT_RDN_ATTR_VALUE_TYPE(7i32);
pub const CERT_RDN_INT4_STRING: CERT_RDN_ATTR_VALUE_TYPE = CERT_RDN_ATTR_VALUE_TYPE(11i32);
pub const CERT_RDN_ISO646_STRING: CERT_RDN_ATTR_VALUE_TYPE = CERT_RDN_ATTR_VALUE_TYPE(9i32);
pub const CERT_RDN_NUMERIC_STRING: CERT_RDN_ATTR_VALUE_TYPE = CERT_RDN_ATTR_VALUE_TYPE(3i32);
pub const CERT_RDN_OCTET_STRING: CERT_RDN_ATTR_VALUE_TYPE = CERT_RDN_ATTR_VALUE_TYPE(2i32);
pub const CERT_RDN_PRINTABLE_STRING: CERT_RDN_ATTR_VALUE_TYPE = CERT_RDN_ATTR_VALUE_TYPE(4i32);
pub const CERT_RDN_T61_STRING: CERT_RDN_ATTR_VALUE_TYPE = CERT_RDN_ATTR_VALUE_TYPE(5i32);
pub const CERT_RDN_TELETEX_STRING: CERT_RDN_ATTR_VALUE_TYPE = CERT_RDN_ATTR_VALUE_TYPE(5i32);
pub const CERT_RDN_TYPE_MASK: u32 = 255u32;
pub const CERT_RDN_UNICODE_STRING: CERT_RDN_ATTR_VALUE_TYPE = CERT_RDN_ATTR_VALUE_TYPE(12i32);
pub const CERT_RDN_UNIVERSAL_STRING: CERT_RDN_ATTR_VALUE_TYPE = CERT_RDN_ATTR_VALUE_TYPE(11i32);
pub const CERT_RDN_UTF8_STRING: CERT_RDN_ATTR_VALUE_TYPE = CERT_RDN_ATTR_VALUE_TYPE(13i32);
pub const CERT_RDN_VIDEOTEX_STRING: CERT_RDN_ATTR_VALUE_TYPE = CERT_RDN_ATTR_VALUE_TYPE(6i32);
pub const CERT_RDN_VISIBLE_STRING: CERT_RDN_ATTR_VALUE_TYPE = CERT_RDN_ATTR_VALUE_TYPE(9i32);
pub const CERT_REGISTRY_STORE_CLIENT_GPT_FLAG: u32 = 2147483648u32;
#[repr(C)]
#[cfg(feature = "Win32_System_Registry")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CERT_REGISTRY_STORE_CLIENT_GPT_PARA {
    pub hKeyBase: super::super::System::Registry::HKEY,
    pub pwszRegPath: windows_core::PWSTR,
}
pub const CERT_REGISTRY_STORE_EXTERNAL_FLAG: u32 = 1048576u32;
pub const CERT_REGISTRY_STORE_LM_GPT_FLAG: u32 = 16777216u32;
pub const CERT_REGISTRY_STORE_MY_IE_DIRTY_FLAG: u32 = 524288u32;
pub const CERT_REGISTRY_STORE_REMOTE_FLAG: u32 = 65536u32;
pub const CERT_REGISTRY_STORE_ROAMING_FLAG: u32 = 262144u32;
#[repr(C)]
#[cfg(feature = "Win32_System_Registry")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CERT_REGISTRY_STORE_ROAMING_PARA {
    pub hKey: super::super::System::Registry::HKEY,
    pub pwszStoreDirectory: windows_core::PWSTR,
}
pub const CERT_REGISTRY_STORE_SERIALIZED_FLAG: u32 = 131072u32;
pub const CERT_RENEWAL_PROP_ID: u32 = 64u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CERT_REQUEST_INFO {
    pub dwVersion: u32,
    pub Subject: CRYPT_INTEGER_BLOB,
    pub SubjectPublicKeyInfo: CERT_PUBLIC_KEY_INFO,
    pub cAttribute: u32,
    pub rgAttribute: *mut CRYPT_ATTRIBUTE,
}
impl Default for CERT_REQUEST_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_REQUEST_ORIGINATOR_PROP_ID: u32 = 71u32;
pub const CERT_REQUEST_V1: u32 = 0u32;
pub const CERT_RETRIEVE_BIOMETRIC_PREDEFINED_BASE_TYPE: windows_core::PCSTR = windows_core::PCSTR(1000i32 as _);
pub const CERT_RETRIEVE_COMMUNITY_LOGO: windows_core::PCSTR = windows_core::PCSTR(3i32 as _);
pub const CERT_RETRIEVE_ISSUER_LOGO: windows_core::PCSTR = windows_core::PCSTR(1i32 as _);
pub const CERT_RETRIEVE_SUBJECT_LOGO: windows_core::PCSTR = windows_core::PCSTR(2i32 as _);
pub const CERT_RETR_BEHAVIOR_FILE_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("AllowFileUrlScheme");
pub const CERT_RETR_BEHAVIOR_INET_AUTH_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("EnableInetUnknownAuth");
pub const CERT_RETR_BEHAVIOR_INET_STATUS_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("EnableInetLocal");
pub const CERT_RETR_BEHAVIOR_LDAP_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("DisableLDAPSignAndEncrypt");
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CERT_REVOCATION_CHAIN_PARA {
    pub cbSize: u32,
    pub hChainEngine: HCERTCHAINENGINE,
    pub hAdditionalStore: HCERTSTORE,
    pub dwChainFlags: u32,
    pub dwUrlRetrievalTimeout: u32,
    pub pftCurrentTime: *mut super::super::Foundation::FILETIME,
    pub pftCacheResync: *mut super::super::Foundation::FILETIME,
    pub cbMaxUrlRetrievalByteCount: u32,
}
impl Default for CERT_REVOCATION_CHAIN_PARA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CERT_REVOCATION_CRL_INFO {
    pub cbSize: u32,
    pub pBaseCrlContext: *mut CRL_CONTEXT,
    pub pDeltaCrlContext: *mut CRL_CONTEXT,
    pub pCrlEntry: *mut CRL_ENTRY,
    pub fDeltaCrlEntry: windows_core::BOOL,
}
impl Default for CERT_REVOCATION_CRL_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CERT_REVOCATION_INFO {
    pub cbSize: u32,
    pub dwRevocationResult: u32,
    pub pszRevocationOid: windows_core::PCSTR,
    pub pvOidSpecificInfo: *mut core::ffi::c_void,
    pub fHasFreshnessTime: windows_core::BOOL,
    pub dwFreshnessTime: u32,
    pub pCrlInfo: *mut CERT_REVOCATION_CRL_INFO,
}
impl Default for CERT_REVOCATION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CERT_REVOCATION_PARA {
    pub cbSize: u32,
    pub pIssuerCert: *const CERT_CONTEXT,
    pub cCertStore: u32,
    pub rgCertStore: *mut HCERTSTORE,
    pub hCrlStore: HCERTSTORE,
    pub pftTimeToUse: *mut super::super::Foundation::FILETIME,
}
impl Default for CERT_REVOCATION_PARA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CERT_REVOCATION_STATUS {
    pub cbSize: u32,
    pub dwIndex: u32,
    pub dwError: u32,
    pub dwReason: CERT_REVOCATION_STATUS_REASON,
    pub fHasFreshnessTime: windows_core::BOOL,
    pub dwFreshnessTime: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CERT_REVOCATION_STATUS_REASON(pub u32);
pub const CERT_ROOT_PROGRAM_CERT_POLICIES_PROP_ID: u32 = 83u32;
pub const CERT_ROOT_PROGRAM_CHAIN_POLICIES_PROP_ID: u32 = 105u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CERT_ROOT_PROGRAM_FLAGS(pub u32);
impl CERT_ROOT_PROGRAM_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CERT_ROOT_PROGRAM_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CERT_ROOT_PROGRAM_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CERT_ROOT_PROGRAM_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CERT_ROOT_PROGRAM_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CERT_ROOT_PROGRAM_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CERT_ROOT_PROGRAM_FLAG_ADDRESS: u32 = 8u32;
pub const CERT_ROOT_PROGRAM_FLAG_LSC: CERT_ROOT_PROGRAM_FLAGS = CERT_ROOT_PROGRAM_FLAGS(64u32);
pub const CERT_ROOT_PROGRAM_FLAG_ORG: CERT_ROOT_PROGRAM_FLAGS = CERT_ROOT_PROGRAM_FLAGS(128u32);
pub const CERT_ROOT_PROGRAM_FLAG_OU: u32 = 16u32;
pub const CERT_ROOT_PROGRAM_FLAG_SUBJECT_LOGO: CERT_ROOT_PROGRAM_FLAGS = CERT_ROOT_PROGRAM_FLAGS(32u32);
pub const CERT_ROOT_PROGRAM_NAME_CONSTRAINTS_PROP_ID: u32 = 84u32;
pub const CERT_RSA_PUBLIC_KEY_OBJID: windows_core::PCWSTR = windows_core::w!("1.2.840.113549.1.1.1");
pub const CERT_SCARD_PIN_ID_PROP_ID: u32 = 90u32;
pub const CERT_SCARD_PIN_INFO_PROP_ID: u32 = 91u32;
pub const CERT_SCEP_CA_CERT_PROP_ID: u32 = 111u32;
pub const CERT_SCEP_ENCRYPT_HASH_CNG_ALG_PROP_ID: u32 = 114u32;
pub const CERT_SCEP_FLAGS_PROP_ID: u32 = 115u32;
pub const CERT_SCEP_GUID_PROP_ID: u32 = 116u32;
pub const CERT_SCEP_NONCE_PROP_ID: u32 = 113u32;
pub const CERT_SCEP_RA_ENCRYPTION_CERT_PROP_ID: u32 = 110u32;
pub const CERT_SCEP_RA_SIGNATURE_CERT_PROP_ID: u32 = 109u32;
pub const CERT_SCEP_SERVER_CERTS_PROP_ID: u32 = 108u32;
pub const CERT_SCEP_SIGNER_CERT_PROP_ID: u32 = 112u32;
pub const CERT_SELECT_ALLOW_DUPLICATES: u32 = 128u32;
pub const CERT_SELECT_ALLOW_EXPIRED: u32 = 1u32;
pub const CERT_SELECT_BY_ENHKEY_USAGE: CERT_SELECT_CRITERIA_TYPE = CERT_SELECT_CRITERIA_TYPE(1u32);
pub const CERT_SELECT_BY_EXTENSION: CERT_SELECT_CRITERIA_TYPE = CERT_SELECT_CRITERIA_TYPE(5u32);
pub const CERT_SELECT_BY_FRIENDLYNAME: u32 = 13u32;
pub const CERT_SELECT_BY_ISSUER_ATTR: CERT_SELECT_CRITERIA_TYPE = CERT_SELECT_CRITERIA_TYPE(7u32);
pub const CERT_SELECT_BY_ISSUER_DISPLAYNAME: u32 = 12u32;
pub const CERT_SELECT_BY_ISSUER_NAME: CERT_SELECT_CRITERIA_TYPE = CERT_SELECT_CRITERIA_TYPE(9u32);
pub const CERT_SELECT_BY_KEY_USAGE: CERT_SELECT_CRITERIA_TYPE = CERT_SELECT_CRITERIA_TYPE(2u32);
pub const CERT_SELECT_BY_POLICY_OID: CERT_SELECT_CRITERIA_TYPE = CERT_SELECT_CRITERIA_TYPE(3u32);
pub const CERT_SELECT_BY_PROV_NAME: CERT_SELECT_CRITERIA_TYPE = CERT_SELECT_CRITERIA_TYPE(4u32);
pub const CERT_SELECT_BY_PUBLIC_KEY: CERT_SELECT_CRITERIA_TYPE = CERT_SELECT_CRITERIA_TYPE(10u32);
pub const CERT_SELECT_BY_SUBJECT_ATTR: CERT_SELECT_CRITERIA_TYPE = CERT_SELECT_CRITERIA_TYPE(8u32);
pub const CERT_SELECT_BY_SUBJECT_HOST_NAME: CERT_SELECT_CRITERIA_TYPE = CERT_SELECT_CRITERIA_TYPE(6u32);
pub const CERT_SELECT_BY_THUMBPRINT: u32 = 14u32;
pub const CERT_SELECT_BY_TLS_SIGNATURES: CERT_SELECT_CRITERIA_TYPE = CERT_SELECT_CRITERIA_TYPE(11u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CERT_SELECT_CHAIN_PARA {
    pub hChainEngine: HCERTCHAINENGINE,
    pub pTime: *mut super::super::Foundation::FILETIME,
    pub hAdditionalStore: HCERTSTORE,
    pub pChainPara: *mut CERT_CHAIN_PARA,
    pub dwFlags: u32,
}
impl Default for CERT_SELECT_CHAIN_PARA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CERT_SELECT_CRITERIA {
    pub dwType: CERT_SELECT_CRITERIA_TYPE,
    pub cPara: u32,
    pub ppPara: *mut *mut core::ffi::c_void,
}
impl Default for CERT_SELECT_CRITERIA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CERT_SELECT_CRITERIA_TYPE(pub u32);
pub const CERT_SELECT_DISALLOW_SELFSIGNED: u32 = 4u32;
pub const CERT_SELECT_HARDWARE_ONLY: u32 = 64u32;
pub const CERT_SELECT_HAS_KEY_FOR_KEY_EXCHANGE: u32 = 32u32;
pub const CERT_SELECT_HAS_KEY_FOR_SIGNATURE: u32 = 16u32;
pub const CERT_SELECT_HAS_PRIVATE_KEY: u32 = 8u32;
pub const CERT_SELECT_IGNORE_AUTOSELECT: u32 = 256u32;
pub const CERT_SELECT_MAX_PARA: u32 = 500u32;
pub const CERT_SELECT_TRUSTED_ROOT: u32 = 2u32;
pub const CERT_SEND_AS_TRUSTED_ISSUER_PROP_ID: u32 = 102u32;
pub const CERT_SERIALIZABLE_KEY_CONTEXT_PROP_ID: u32 = 117u32;
pub const CERT_SERIAL_CHAIN_PROP_ID: u32 = 119u32;
pub const CERT_SERVER_OCSP_RESPONSE_ASYNC_FLAG: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CERT_SERVER_OCSP_RESPONSE_CONTEXT {
    pub cbSize: u32,
    pub pbEncodedOcspResponse: *mut u8,
    pub cbEncodedOcspResponse: u32,
}
impl Default for CERT_SERVER_OCSP_RESPONSE_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CERT_SERVER_OCSP_RESPONSE_OPEN_PARA {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub pcbUsedSize: *mut u32,
    pub pwszOcspDirectory: windows_core::PWSTR,
    pub pfnUpdateCallback: PFN_CERT_SERVER_OCSP_RESPONSE_UPDATE_CALLBACK,
    pub pvUpdateCallbackArg: *mut core::ffi::c_void,
}
impl Default for CERT_SERVER_OCSP_RESPONSE_OPEN_PARA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_SERVER_OCSP_RESPONSE_OPEN_PARA_READ_FLAG: u32 = 1u32;
pub const CERT_SERVER_OCSP_RESPONSE_OPEN_PARA_WRITE_FLAG: u32 = 2u32;
pub const CERT_SET_KEY_CONTEXT_PROP_ID: CRYPT_KEY_FLAGS = CRYPT_KEY_FLAGS(1u32);
pub const CERT_SET_KEY_PROV_HANDLE_PROP_ID: CRYPT_KEY_FLAGS = CRYPT_KEY_FLAGS(1u32);
pub const CERT_SET_PROPERTY_IGNORE_PERSIST_ERROR_FLAG: u32 = 2147483648u32;
pub const CERT_SET_PROPERTY_INHIBIT_PERSIST_FLAG: u32 = 1073741824u32;
pub const CERT_SHA1_HASH_PROP_ID: u32 = 3u32;
pub const CERT_SHA256_HASH_PROP_ID: u32 = 107u32;
pub const CERT_SIGNATURE_HASH_PROP_ID: u32 = 15u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CERT_SIGNED_CONTENT_INFO {
    pub ToBeSigned: CRYPT_INTEGER_BLOB,
    pub SignatureAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub Signature: CRYPT_BIT_BLOB,
}
pub const CERT_SIGN_HASH_CNG_ALG_PROP_ID: u32 = 89u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CERT_SIMPLE_CHAIN {
    pub cbSize: u32,
    pub TrustStatus: CERT_TRUST_STATUS,
    pub cElement: u32,
    pub rgpElement: *mut *mut CERT_CHAIN_ELEMENT,
    pub pTrustListInfo: *mut CERT_TRUST_LIST_INFO,
    pub fHasRevocationFreshnessTime: windows_core::BOOL,
    pub dwRevocationFreshnessTime: u32,
}
impl Default for CERT_SIMPLE_CHAIN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_SIMPLE_NAME_STR: CERT_STRING_TYPE = CERT_STRING_TYPE(1u32);
pub const CERT_SMART_CARD_DATA_PROP_ID: u32 = 16u32;
pub const CERT_SMART_CARD_READER_NON_REMOVABLE_PROP_ID: u32 = 106u32;
pub const CERT_SMART_CARD_READER_PROP_ID: u32 = 101u32;
pub const CERT_SMART_CARD_ROOT_INFO_PROP_ID: u32 = 76u32;
pub const CERT_SOURCE_LOCATION_PROP_ID: u32 = 72u32;
pub const CERT_SOURCE_URL_PROP_ID: u32 = 73u32;
pub const CERT_SRV_OCSP_RESP_MAX_BEFORE_NEXT_UPDATE_SECONDS_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("SrvOcspRespMaxBeforeNextUpdateSeconds");
pub const CERT_SRV_OCSP_RESP_MAX_SYNC_CERT_FILE_SECONDS_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("SrvOcspRespMaxSyncCertFileSeconds");
pub const CERT_SRV_OCSP_RESP_MIN_AFTER_NEXT_UPDATE_SECONDS_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("SrvOcspRespMinAfterNextUpdateSeconds");
pub const CERT_SRV_OCSP_RESP_MIN_BEFORE_NEXT_UPDATE_SECONDS_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("SrvOcspRespMinBeforeNextUpdateSeconds");
pub const CERT_SRV_OCSP_RESP_MIN_SYNC_CERT_FILE_SECONDS_DEFAULT: u32 = 5u32;
pub const CERT_SRV_OCSP_RESP_MIN_SYNC_CERT_FILE_SECONDS_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("SrvOcspRespMinSyncCertFileSeconds");
pub const CERT_SRV_OCSP_RESP_MIN_VALIDITY_SECONDS_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("SrvOcspRespMinValiditySeconds");
pub const CERT_SRV_OCSP_RESP_URL_RETRIEVAL_TIMEOUT_MILLISECONDS_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("SrvOcspRespUrlRetrievalTimeoutMilliseconds");
pub const CERT_STORE_ADD_ALWAYS: u32 = 4u32;
pub const CERT_STORE_ADD_NEW: u32 = 1u32;
pub const CERT_STORE_ADD_NEWER: u32 = 6u32;
pub const CERT_STORE_ADD_NEWER_INHERIT_PROPERTIES: u32 = 7u32;
pub const CERT_STORE_ADD_REPLACE_EXISTING: u32 = 3u32;
pub const CERT_STORE_ADD_REPLACE_EXISTING_INHERIT_PROPERTIES: u32 = 5u32;
pub const CERT_STORE_ADD_USE_EXISTING: u32 = 2u32;
pub const CERT_STORE_BACKUP_RESTORE_FLAG: CERT_OPEN_STORE_FLAGS = CERT_OPEN_STORE_FLAGS(2048u32);
pub const CERT_STORE_BASE_CRL_FLAG: u32 = 256u32;
pub const CERT_STORE_CERTIFICATE_CONTEXT: u32 = 1u32;
pub const CERT_STORE_CREATE_NEW_FLAG: CERT_OPEN_STORE_FLAGS = CERT_OPEN_STORE_FLAGS(8192u32);
pub const CERT_STORE_CRL_CONTEXT: u32 = 2u32;
pub const CERT_STORE_CTL_CONTEXT: u32 = 3u32;
pub const CERT_STORE_CTRL_AUTO_RESYNC: u32 = 4u32;
pub const CERT_STORE_CTRL_CANCEL_NOTIFY: u32 = 5u32;
pub const CERT_STORE_CTRL_COMMIT: u32 = 3u32;
pub const CERT_STORE_CTRL_COMMIT_CLEAR_FLAG: CERT_CONTROL_STORE_FLAGS = CERT_CONTROL_STORE_FLAGS(2u32);
pub const CERT_STORE_CTRL_COMMIT_FORCE_FLAG: CERT_CONTROL_STORE_FLAGS = CERT_CONTROL_STORE_FLAGS(1u32);
pub const CERT_STORE_CTRL_INHIBIT_DUPLICATE_HANDLE_FLAG: CERT_CONTROL_STORE_FLAGS = CERT_CONTROL_STORE_FLAGS(1u32);
pub const CERT_STORE_CTRL_NOTIFY_CHANGE: u32 = 2u32;
pub const CERT_STORE_CTRL_RESYNC: u32 = 1u32;
pub const CERT_STORE_DEFER_CLOSE_UNTIL_LAST_FREE_FLAG: CERT_OPEN_STORE_FLAGS = CERT_OPEN_STORE_FLAGS(4u32);
pub const CERT_STORE_DELETE_FLAG: CERT_OPEN_STORE_FLAGS = CERT_OPEN_STORE_FLAGS(16u32);
pub const CERT_STORE_DELTA_CRL_FLAG: u32 = 512u32;
pub const CERT_STORE_ENUM_ARCHIVED_FLAG: CERT_OPEN_STORE_FLAGS = CERT_OPEN_STORE_FLAGS(512u32);
pub const CERT_STORE_LOCALIZED_NAME_PROP_ID: u32 = 4096u32;
pub const CERT_STORE_MANIFOLD_FLAG: u32 = 256u32;
pub const CERT_STORE_MAXIMUM_ALLOWED_FLAG: CERT_OPEN_STORE_FLAGS = CERT_OPEN_STORE_FLAGS(4096u32);
pub const CERT_STORE_NO_CRL_FLAG: u32 = 65536u32;
pub const CERT_STORE_NO_CRYPT_RELEASE_FLAG: CERT_OPEN_STORE_FLAGS = CERT_OPEN_STORE_FLAGS(1u32);
pub const CERT_STORE_NO_ISSUER_FLAG: u32 = 131072u32;
pub const CERT_STORE_OPEN_EXISTING_FLAG: CERT_OPEN_STORE_FLAGS = CERT_OPEN_STORE_FLAGS(16384u32);
pub const CERT_STORE_PROV_CLOSE_FUNC: u32 = 0u32;
pub const CERT_STORE_PROV_COLLECTION: windows_core::PCSTR = windows_core::PCSTR(11i32 as _);
pub const CERT_STORE_PROV_CONTROL_FUNC: u32 = 13u32;
pub const CERT_STORE_PROV_DELETED_FLAG: CERT_STORE_PROV_FLAGS = CERT_STORE_PROV_FLAGS(2u32);
pub const CERT_STORE_PROV_DELETE_CERT_FUNC: u32 = 3u32;
pub const CERT_STORE_PROV_DELETE_CRL_FUNC: u32 = 7u32;
pub const CERT_STORE_PROV_DELETE_CTL_FUNC: u32 = 11u32;
pub const CERT_STORE_PROV_EXTERNAL_FLAG: CERT_STORE_PROV_FLAGS = CERT_STORE_PROV_FLAGS(1u32);
pub const CERT_STORE_PROV_FILE: windows_core::PCSTR = windows_core::PCSTR(3i32 as _);
pub const CERT_STORE_PROV_FILENAME: i32 = 8i32;
pub const CERT_STORE_PROV_FILENAME_A: windows_core::PCSTR = windows_core::PCSTR(7i32 as _);
pub const CERT_STORE_PROV_FILENAME_W: windows_core::PCSTR = windows_core::PCSTR(8i32 as _);
pub const CERT_STORE_PROV_FIND_CERT_FUNC: u32 = 14u32;
pub const CERT_STORE_PROV_FIND_CRL_FUNC: u32 = 17u32;
pub const CERT_STORE_PROV_FIND_CTL_FUNC: u32 = 20u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CERT_STORE_PROV_FIND_INFO {
    pub cbSize: u32,
    pub dwMsgAndCertEncodingType: u32,
    pub dwFindFlags: u32,
    pub dwFindType: u32,
    pub pvFindPara: *const core::ffi::c_void,
}
impl Default for CERT_STORE_PROV_FIND_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CERT_STORE_PROV_FLAGS(pub u32);
impl CERT_STORE_PROV_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CERT_STORE_PROV_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CERT_STORE_PROV_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CERT_STORE_PROV_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CERT_STORE_PROV_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CERT_STORE_PROV_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CERT_STORE_PROV_FREE_FIND_CERT_FUNC: u32 = 15u32;
pub const CERT_STORE_PROV_FREE_FIND_CRL_FUNC: u32 = 18u32;
pub const CERT_STORE_PROV_FREE_FIND_CTL_FUNC: u32 = 21u32;
pub const CERT_STORE_PROV_GET_CERT_PROPERTY_FUNC: u32 = 16u32;
pub const CERT_STORE_PROV_GET_CRL_PROPERTY_FUNC: u32 = 19u32;
pub const CERT_STORE_PROV_GET_CTL_PROPERTY_FUNC: u32 = 22u32;
pub const CERT_STORE_PROV_GP_SYSTEM_STORE_FLAG: u32 = 32u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CERT_STORE_PROV_INFO {
    pub cbSize: u32,
    pub cStoreProvFunc: u32,
    pub rgpvStoreProvFunc: *mut *mut core::ffi::c_void,
    pub hStoreProv: HCERTSTOREPROV,
    pub dwStoreProvFlags: CERT_STORE_PROV_FLAGS,
    pub hStoreProvFuncAddr2: *mut core::ffi::c_void,
}
impl Default for CERT_STORE_PROV_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_STORE_PROV_LDAP: i32 = 16i32;
pub const CERT_STORE_PROV_LDAP_W: windows_core::PCSTR = windows_core::PCSTR(16i32 as _);
pub const CERT_STORE_PROV_LM_SYSTEM_STORE_FLAG: CERT_STORE_PROV_FLAGS = CERT_STORE_PROV_FLAGS(16u32);
pub const CERT_STORE_PROV_MEMORY: windows_core::PCSTR = windows_core::PCSTR(2i32 as _);
pub const CERT_STORE_PROV_MSG: windows_core::PCSTR = windows_core::PCSTR(1i32 as _);
pub const CERT_STORE_PROV_NO_PERSIST_FLAG: CERT_STORE_PROV_FLAGS = CERT_STORE_PROV_FLAGS(4u32);
pub const CERT_STORE_PROV_PHYSICAL: i32 = 14i32;
pub const CERT_STORE_PROV_PHYSICAL_W: windows_core::PCSTR = windows_core::PCSTR(14i32 as _);
pub const CERT_STORE_PROV_PKCS12: windows_core::PCSTR = windows_core::PCSTR(17i32 as _);
pub const CERT_STORE_PROV_PKCS7: windows_core::PCSTR = windows_core::PCSTR(5i32 as _);
pub const CERT_STORE_PROV_READ_CERT_FUNC: u32 = 1u32;
pub const CERT_STORE_PROV_READ_CRL_FUNC: u32 = 5u32;
pub const CERT_STORE_PROV_READ_CTL_FUNC: u32 = 9u32;
pub const CERT_STORE_PROV_REG: windows_core::PCSTR = windows_core::PCSTR(4i32 as _);
pub const CERT_STORE_PROV_SERIALIZED: windows_core::PCSTR = windows_core::PCSTR(6i32 as _);
pub const CERT_STORE_PROV_SET_CERT_PROPERTY_FUNC: u32 = 4u32;
pub const CERT_STORE_PROV_SET_CRL_PROPERTY_FUNC: u32 = 8u32;
pub const CERT_STORE_PROV_SET_CTL_PROPERTY_FUNC: u32 = 12u32;
pub const CERT_STORE_PROV_SHARED_USER_FLAG: u32 = 64u32;
pub const CERT_STORE_PROV_SMART_CARD: i32 = 15i32;
pub const CERT_STORE_PROV_SMART_CARD_W: windows_core::PCSTR = windows_core::PCSTR(15i32 as _);
pub const CERT_STORE_PROV_SYSTEM: i32 = 10i32;
pub const CERT_STORE_PROV_SYSTEM_A: windows_core::PCSTR = windows_core::PCSTR(9i32 as _);
pub const CERT_STORE_PROV_SYSTEM_REGISTRY: i32 = 13i32;
pub const CERT_STORE_PROV_SYSTEM_REGISTRY_A: windows_core::PCSTR = windows_core::PCSTR(12i32 as _);
pub const CERT_STORE_PROV_SYSTEM_REGISTRY_W: windows_core::PCSTR = windows_core::PCSTR(13i32 as _);
pub const CERT_STORE_PROV_SYSTEM_STORE_FLAG: CERT_STORE_PROV_FLAGS = CERT_STORE_PROV_FLAGS(8u32);
pub const CERT_STORE_PROV_SYSTEM_W: windows_core::PCSTR = windows_core::PCSTR(10i32 as _);
pub const CERT_STORE_PROV_WRITE_ADD_FLAG: u32 = 1u32;
pub const CERT_STORE_PROV_WRITE_CERT_FUNC: u32 = 2u32;
pub const CERT_STORE_PROV_WRITE_CRL_FUNC: u32 = 6u32;
pub const CERT_STORE_PROV_WRITE_CTL_FUNC: u32 = 10u32;
pub const CERT_STORE_READONLY_FLAG: CERT_OPEN_STORE_FLAGS = CERT_OPEN_STORE_FLAGS(32768u32);
pub const CERT_STORE_REVOCATION_FLAG: u32 = 4u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CERT_STORE_SAVE_AS(pub u32);
pub const CERT_STORE_SAVE_AS_PKCS12: u32 = 3u32;
pub const CERT_STORE_SAVE_AS_PKCS7: CERT_STORE_SAVE_AS = CERT_STORE_SAVE_AS(2u32);
pub const CERT_STORE_SAVE_AS_STORE: CERT_STORE_SAVE_AS = CERT_STORE_SAVE_AS(1u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CERT_STORE_SAVE_TO(pub u32);
pub const CERT_STORE_SAVE_TO_FILE: CERT_STORE_SAVE_TO = CERT_STORE_SAVE_TO(1u32);
pub const CERT_STORE_SAVE_TO_FILENAME: CERT_STORE_SAVE_TO = CERT_STORE_SAVE_TO(4u32);
pub const CERT_STORE_SAVE_TO_FILENAME_A: CERT_STORE_SAVE_TO = CERT_STORE_SAVE_TO(3u32);
pub const CERT_STORE_SAVE_TO_FILENAME_W: CERT_STORE_SAVE_TO = CERT_STORE_SAVE_TO(4u32);
pub const CERT_STORE_SAVE_TO_MEMORY: CERT_STORE_SAVE_TO = CERT_STORE_SAVE_TO(2u32);
pub const CERT_STORE_SET_LOCALIZED_NAME_FLAG: CERT_OPEN_STORE_FLAGS = CERT_OPEN_STORE_FLAGS(2u32);
pub const CERT_STORE_SHARE_CONTEXT_FLAG: CERT_OPEN_STORE_FLAGS = CERT_OPEN_STORE_FLAGS(128u32);
pub const CERT_STORE_SHARE_STORE_FLAG: u32 = 64u32;
pub const CERT_STORE_SIGNATURE_FLAG: u32 = 1u32;
pub const CERT_STORE_TIME_VALIDITY_FLAG: u32 = 2u32;
pub const CERT_STORE_UNSAFE_PHYSICAL_FLAG: u32 = 32u32;
pub const CERT_STORE_UPDATE_KEYID_FLAG: CERT_OPEN_STORE_FLAGS = CERT_OPEN_STORE_FLAGS(1024u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CERT_STRING_TYPE(pub u32);
pub const CERT_STRONG_SIGN_ECDSA_ALGORITHM: windows_core::PCWSTR = windows_core::w!("ECDSA");
pub const CERT_STRONG_SIGN_ENABLE_CRL_CHECK: CERT_STRONG_SIGN_FLAGS = CERT_STRONG_SIGN_FLAGS(1u32);
pub const CERT_STRONG_SIGN_ENABLE_OCSP_CHECK: CERT_STRONG_SIGN_FLAGS = CERT_STRONG_SIGN_FLAGS(2u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CERT_STRONG_SIGN_FLAGS(pub u32);
impl CERT_STRONG_SIGN_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CERT_STRONG_SIGN_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CERT_STRONG_SIGN_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CERT_STRONG_SIGN_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CERT_STRONG_SIGN_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CERT_STRONG_SIGN_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CERT_STRONG_SIGN_OID_INFO_CHOICE: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_STRONG_SIGN_PARA {
    pub cbSize: u32,
    pub dwInfoChoice: u32,
    pub Anonymous: CERT_STRONG_SIGN_PARA_0,
}
impl Default for CERT_STRONG_SIGN_PARA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CERT_STRONG_SIGN_PARA_0 {
    pub pvInfo: *mut core::ffi::c_void,
    pub pSerializedInfo: *mut CERT_STRONG_SIGN_SERIALIZED_INFO,
    pub pszOID: windows_core::PSTR,
}
impl Default for CERT_STRONG_SIGN_PARA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CERT_STRONG_SIGN_SERIALIZED_INFO {
    pub dwFlags: CERT_STRONG_SIGN_FLAGS,
    pub pwszCNGSignHashAlgids: windows_core::PWSTR,
    pub pwszCNGPubKeyMinBitLengths: windows_core::PWSTR,
}
pub const CERT_STRONG_SIGN_SERIALIZED_INFO_CHOICE: u32 = 1u32;
pub const CERT_SUBJECT_DISABLE_CRL_PROP_ID: u32 = 86u32;
pub const CERT_SUBJECT_INFO_ACCESS_PROP_ID: u32 = 80u32;
pub const CERT_SUBJECT_NAME_MD5_HASH_PROP_ID: u32 = 29u32;
pub const CERT_SUBJECT_OCSP_AUTHORITY_INFO_ACCESS_PROP_ID: u32 = 85u32;
pub const CERT_SUBJECT_PUBLIC_KEY_MD5_HASH_PROP_ID: u32 = 25u32;
pub const CERT_SUBJECT_PUB_KEY_BIT_LENGTH_PROP_ID: u32 = 92u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CERT_SUPPORTED_ALGORITHM_INFO {
    pub Algorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub IntendedKeyUsage: CRYPT_BIT_BLOB,
    pub IntendedCertPolicies: CERT_POLICIES_INFO,
}
pub const CERT_SYSTEM_STORE_CURRENT_SERVICE_ID: u32 = 4u32;
pub const CERT_SYSTEM_STORE_CURRENT_USER: u32 = 65536u32;
pub const CERT_SYSTEM_STORE_CURRENT_USER_GROUP_POLICY_ID: u32 = 7u32;
pub const CERT_SYSTEM_STORE_CURRENT_USER_ID: u32 = 1u32;
pub const CERT_SYSTEM_STORE_DEFER_READ_FLAG: u32 = 536870912u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CERT_SYSTEM_STORE_FLAGS(pub u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CERT_SYSTEM_STORE_INFO {
    pub cbSize: u32,
}
pub const CERT_SYSTEM_STORE_LOCAL_MACHINE: u32 = 131072u32;
pub const CERT_SYSTEM_STORE_LOCAL_MACHINE_ENTERPRISE_ID: u32 = 9u32;
pub const CERT_SYSTEM_STORE_LOCAL_MACHINE_GROUP_POLICY_ID: u32 = 8u32;
pub const CERT_SYSTEM_STORE_LOCAL_MACHINE_ID: u32 = 2u32;
pub const CERT_SYSTEM_STORE_LOCAL_MACHINE_WCOS_ID: u32 = 10u32;
pub const CERT_SYSTEM_STORE_LOCATION_MASK: CERT_SYSTEM_STORE_FLAGS = CERT_SYSTEM_STORE_FLAGS(16711680u32);
pub const CERT_SYSTEM_STORE_LOCATION_SHIFT: u32 = 16u32;
pub const CERT_SYSTEM_STORE_MASK: u32 = 4294901760u32;
pub const CERT_SYSTEM_STORE_RELOCATE_FLAG: CERT_SYSTEM_STORE_FLAGS = CERT_SYSTEM_STORE_FLAGS(2147483648u32);
#[repr(C)]
#[cfg(feature = "Win32_System_Registry")]
#[derive(Clone, Copy)]
pub struct CERT_SYSTEM_STORE_RELOCATE_PARA {
    pub Anonymous1: CERT_SYSTEM_STORE_RELOCATE_PARA_0,
    pub Anonymous2: CERT_SYSTEM_STORE_RELOCATE_PARA_1,
}
#[cfg(feature = "Win32_System_Registry")]
impl Default for CERT_SYSTEM_STORE_RELOCATE_PARA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Registry")]
#[derive(Clone, Copy)]
pub union CERT_SYSTEM_STORE_RELOCATE_PARA_0 {
    pub hKeyBase: super::super::System::Registry::HKEY,
    pub pvBase: *mut core::ffi::c_void,
}
#[cfg(feature = "Win32_System_Registry")]
impl Default for CERT_SYSTEM_STORE_RELOCATE_PARA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Registry")]
#[derive(Clone, Copy)]
pub union CERT_SYSTEM_STORE_RELOCATE_PARA_1 {
    pub pvSystemStore: *mut core::ffi::c_void,
    pub pszSystemStore: windows_core::PCSTR,
    pub pwszSystemStore: windows_core::PCWSTR,
}
#[cfg(feature = "Win32_System_Registry")]
impl Default for CERT_SYSTEM_STORE_RELOCATE_PARA_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_SYSTEM_STORE_SERVICES_ID: u32 = 5u32;
pub const CERT_SYSTEM_STORE_UNPROTECTED_FLAG: u32 = 1073741824u32;
pub const CERT_SYSTEM_STORE_USERS_ID: u32 = 6u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CERT_TEMPLATE_EXT {
    pub pszObjId: windows_core::PSTR,
    pub dwMajorVersion: u32,
    pub fMinorVersion: windows_core::BOOL,
    pub dwMinorVersion: u32,
}
pub const CERT_TIMESTAMP_HASH_USE_TYPE: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CERT_TPM_SPECIFICATION_INFO {
    pub pwszFamily: windows_core::PWSTR,
    pub dwLevel: u32,
    pub dwRevision: u32,
}
pub const CERT_TRUST_AUTO_UPDATE_CA_REVOCATION: u32 = 16u32;
pub const CERT_TRUST_AUTO_UPDATE_END_REVOCATION: u32 = 32u32;
pub const CERT_TRUST_BEFORE_DISALLOWED_CA_FILETIME: u32 = 2097152u32;
pub const CERT_TRUST_CTL_IS_NOT_SIGNATURE_VALID: u32 = 262144u32;
pub const CERT_TRUST_CTL_IS_NOT_TIME_VALID: u32 = 131072u32;
pub const CERT_TRUST_CTL_IS_NOT_VALID_FOR_USAGE: u32 = 524288u32;
pub const CERT_TRUST_HAS_ALLOW_WEAK_SIGNATURE: u32 = 131072u32;
pub const CERT_TRUST_HAS_AUTO_UPDATE_WEAK_SIGNATURE: u32 = 32768u32;
pub const CERT_TRUST_HAS_CRL_VALIDITY_EXTENDED: u32 = 4096u32;
pub const CERT_TRUST_HAS_EXACT_MATCH_ISSUER: u32 = 1u32;
pub const CERT_TRUST_HAS_EXCLUDED_NAME_CONSTRAINT: u32 = 32768u32;
pub const CERT_TRUST_HAS_ISSUANCE_CHAIN_POLICY: u32 = 512u32;
pub const CERT_TRUST_HAS_KEY_MATCH_ISSUER: u32 = 2u32;
pub const CERT_TRUST_HAS_NAME_MATCH_ISSUER: u32 = 4u32;
pub const CERT_TRUST_HAS_NOT_DEFINED_NAME_CONSTRAINT: u32 = 8192u32;
pub const CERT_TRUST_HAS_NOT_PERMITTED_NAME_CONSTRAINT: u32 = 16384u32;
pub const CERT_TRUST_HAS_NOT_SUPPORTED_CRITICAL_EXT: u32 = 134217728u32;
pub const CERT_TRUST_HAS_NOT_SUPPORTED_NAME_CONSTRAINT: u32 = 4096u32;
pub const CERT_TRUST_HAS_PREFERRED_ISSUER: u32 = 256u32;
pub const CERT_TRUST_HAS_VALID_NAME_CONSTRAINTS: u32 = 1024u32;
pub const CERT_TRUST_HAS_WEAK_HYGIENE: u32 = 2097152u32;
pub const CERT_TRUST_HAS_WEAK_SIGNATURE: u32 = 1048576u32;
pub const CERT_TRUST_INVALID_BASIC_CONSTRAINTS: u32 = 1024u32;
pub const CERT_TRUST_INVALID_EXTENSION: u32 = 256u32;
pub const CERT_TRUST_INVALID_NAME_CONSTRAINTS: u32 = 2048u32;
pub const CERT_TRUST_INVALID_POLICY_CONSTRAINTS: u32 = 512u32;
pub const CERT_TRUST_IS_CA_TRUSTED: u32 = 16384u32;
pub const CERT_TRUST_IS_COMPLEX_CHAIN: u32 = 65536u32;
pub const CERT_TRUST_IS_CYCLIC: u32 = 128u32;
pub const CERT_TRUST_IS_EXPLICIT_DISTRUST: u32 = 67108864u32;
pub const CERT_TRUST_IS_FROM_EXCLUSIVE_TRUST_STORE: u32 = 8192u32;
pub const CERT_TRUST_IS_KEY_ROLLOVER: u32 = 128u32;
pub const CERT_TRUST_IS_NOT_SIGNATURE_VALID: u32 = 8u32;
pub const CERT_TRUST_IS_NOT_TIME_NESTED: u32 = 2u32;
pub const CERT_TRUST_IS_NOT_TIME_VALID: u32 = 1u32;
pub const CERT_TRUST_IS_NOT_VALID_FOR_USAGE: u32 = 16u32;
pub const CERT_TRUST_IS_OFFLINE_REVOCATION: u32 = 16777216u32;
pub const CERT_TRUST_IS_PARTIAL_CHAIN: u32 = 65536u32;
pub const CERT_TRUST_IS_PEER_TRUSTED: u32 = 2048u32;
pub const CERT_TRUST_IS_REVOKED: u32 = 4u32;
pub const CERT_TRUST_IS_SELF_SIGNED: u32 = 8u32;
pub const CERT_TRUST_IS_UNTRUSTED_ROOT: u32 = 32u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CERT_TRUST_LIST_INFO {
    pub cbSize: u32,
    pub pCtlEntry: *mut CTL_ENTRY,
    pub pCtlContext: *mut CTL_CONTEXT,
}
impl Default for CERT_TRUST_LIST_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_TRUST_NO_ERROR: u32 = 0u32;
pub const CERT_TRUST_NO_ISSUANCE_CHAIN_POLICY: u32 = 33554432u32;
pub const CERT_TRUST_NO_OCSP_FAILOVER_TO_CRL: u32 = 64u32;
pub const CERT_TRUST_NO_TIME_CHECK: u32 = 33554432u32;
pub const CERT_TRUST_PUB_ALLOW_END_USER_TRUST: u32 = 0u32;
pub const CERT_TRUST_PUB_ALLOW_ENTERPRISE_ADMIN_TRUST: u32 = 2u32;
pub const CERT_TRUST_PUB_ALLOW_MACHINE_ADMIN_TRUST: u32 = 1u32;
pub const CERT_TRUST_PUB_ALLOW_TRUST_MASK: u32 = 3u32;
pub const CERT_TRUST_PUB_AUTHENTICODE_FLAGS_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("AuthenticodeFlags");
pub const CERT_TRUST_PUB_CHECK_PUBLISHER_REV_FLAG: u32 = 256u32;
pub const CERT_TRUST_PUB_CHECK_TIMESTAMP_REV_FLAG: u32 = 512u32;
pub const CERT_TRUST_REVOCATION_STATUS_UNKNOWN: u32 = 64u32;
pub const CERT_TRUST_SSL_HANDSHAKE_OCSP: u32 = 262144u32;
pub const CERT_TRUST_SSL_RECONNECT_OCSP: u32 = 1048576u32;
pub const CERT_TRUST_SSL_TIME_VALID: u32 = 16777216u32;
pub const CERT_TRUST_SSL_TIME_VALID_OCSP: u32 = 524288u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CERT_TRUST_STATUS {
    pub dwErrorStatus: u32,
    pub dwInfoStatus: u32,
}
pub const CERT_UNICODE_ATTR_ERR_INDEX_MASK: u32 = 63u32;
pub const CERT_UNICODE_ATTR_ERR_INDEX_SHIFT: u32 = 16u32;
pub const CERT_UNICODE_IS_RDN_ATTRS_FLAG: u32 = 1u32;
pub const CERT_UNICODE_RDN_ERR_INDEX_MASK: u32 = 1023u32;
pub const CERT_UNICODE_RDN_ERR_INDEX_SHIFT: u32 = 22u32;
pub const CERT_UNICODE_VALUE_ERR_INDEX_MASK: u32 = 65535u32;
pub const CERT_UNICODE_VALUE_ERR_INDEX_SHIFT: u32 = 0u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CERT_USAGE_MATCH {
    pub dwType: u32,
    pub Usage: CTL_USAGE,
}
pub const CERT_V1: u32 = 0u32;
pub const CERT_V2: u32 = 1u32;
pub const CERT_V3: u32 = 2u32;
pub const CERT_VERIFY_ALLOW_MORE_USAGE_FLAG: u32 = 8u32;
pub const CERT_VERIFY_CACHE_ONLY_BASED_REVOCATION: u32 = 2u32;
pub const CERT_VERIFY_INHIBIT_CTL_UPDATE_FLAG: u32 = 1u32;
pub const CERT_VERIFY_NO_TIME_CHECK_FLAG: u32 = 4u32;
pub const CERT_VERIFY_REV_ACCUMULATIVE_TIMEOUT_FLAG: u32 = 4u32;
pub const CERT_VERIFY_REV_CHAIN_FLAG: u32 = 1u32;
pub const CERT_VERIFY_REV_NO_OCSP_FAILOVER_TO_CRL_FLAG: u32 = 16u32;
pub const CERT_VERIFY_REV_SERVER_OCSP_FLAG: u32 = 8u32;
pub const CERT_VERIFY_REV_SERVER_OCSP_WIRE_ONLY_FLAG: u32 = 32u32;
pub const CERT_VERIFY_TRUSTED_SIGNERS_FLAG: u32 = 2u32;
pub const CERT_VERIFY_UPDATED_CTL_FLAG: u32 = 1u32;
pub const CERT_X500_NAME_STR: CERT_STRING_TYPE = CERT_STRING_TYPE(3u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CERT_X942_DH_PARAMETERS {
    pub p: CRYPT_INTEGER_BLOB,
    pub g: CRYPT_INTEGER_BLOB,
    pub q: CRYPT_INTEGER_BLOB,
    pub j: CRYPT_INTEGER_BLOB,
    pub pValidationParams: *mut CERT_X942_DH_VALIDATION_PARAMS,
}
impl Default for CERT_X942_DH_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CERT_X942_DH_VALIDATION_PARAMS {
    pub seed: CRYPT_BIT_BLOB,
    pub pgenCounter: u32,
}
pub const CERT_XML_NAME_STR: u32 = 4u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CESSetupProperty(pub i32);
pub const CKP_BLOCK_LENGTH: windows_core::PCWSTR = windows_core::w!("BlockLength");
pub const CKP_CHAINING_MODE: windows_core::PCWSTR = windows_core::w!("ChainingMode");
pub const CKP_INITIALIZATION_VECTOR: windows_core::PCWSTR = windows_core::w!("IV");
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLAIMLIST {
    pub count: u32,
    pub claims: *const windows_core::PCWSTR,
}
impl Default for CLAIMLIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CLMD_FILE_TAG_CARD_AUTH_CERT: u32 = 6275329u32;
pub const CLMD_FILE_TAG_CARD_CAPABILITY_CONTAINER: u32 = 6275335u32;
pub const CLMD_FILE_TAG_CHUID: u32 = 6275330u32;
pub const CLMD_FILE_TAG_FACIAL_IMAGE: u32 = 6275336u32;
pub const CLMD_FILE_TAG_FINGERPRINT: u32 = 6275331u32;
pub const CLMD_FILE_TAG_FIRST_RETIRED_KEY_MGMT_KEY: u32 = 6275341u32;
pub const CLMD_FILE_TAG_KEY_HISTORY: u32 = 6275340u32;
pub const CLMD_FILE_TAG_KEY_MGMT_CERT: u32 = 6275339u32;
pub const CLMD_FILE_TAG_LAST_RETIRED_KEY_MGMT_KEY: u32 = 6275360u32;
pub const CLMD_FILE_TAG_PIV_AUTH_CERT: u32 = 6275333u32;
pub const CLMD_FILE_TAG_PRINTED_INFORMATION: u32 = 6275337u32;
pub const CLMD_FILE_TAG_SECURITY_OBJECT: u32 = 6275334u32;
pub const CLMD_FILE_TAG_SIG_CERT: u32 = 6275338u32;
pub const CLMD_FILE_TAG_UNSIGNED_CHUID: u32 = 6275332u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLMD_PIV_CERT_DATA {
    pub dwVersion: u32,
    pub dwCertTag: u32,
    pub pbCert: *mut u8,
    pub cbCert: u32,
}
impl Default for CLMD_PIV_CERT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CLMD_PIV_CERT_DATA_CURRENT_VERSION: u32 = 0u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLMD_PIV_GENERATE_ASYMMETRIC_KEY {
    pub dwVersion: u32,
    pub bAlgorithmId: u8,
    pub bKeyId: u8,
    pub pbKey: *mut u8,
    pub cbKey: u32,
}
impl Default for CLMD_PIV_GENERATE_ASYMMETRIC_KEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CLMD_PIV_GENERATE_ASYMMETRIC_KEY_CURRENT_VERSION: u32 = 0u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLMD_PIV_PUBLIC_KEY_DATA {
    pub dwVersion: u32,
    pub bKeyId: u8,
    pub pbPublicKey: *mut u8,
    pub cbPublicKey: u32,
}
impl Default for CLMD_PIV_PUBLIC_KEY_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CLMD_PIV_PUBLIC_KEY_DATA_CURRENT_VERSION: u32 = 0u32;
pub const CMC_ADD_ATTRIBUTES: windows_core::PCSTR = windows_core::PCSTR(63i32 as _);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CMC_ADD_ATTRIBUTES_INFO {
    pub dwCmcDataReference: u32,
    pub cCertReference: u32,
    pub rgdwCertReference: *mut u32,
    pub cAttribute: u32,
    pub rgAttribute: *mut CRYPT_ATTRIBUTE,
}
impl Default for CMC_ADD_ATTRIBUTES_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CMC_ADD_EXTENSIONS: windows_core::PCSTR = windows_core::PCSTR(62i32 as _);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CMC_ADD_EXTENSIONS_INFO {
    pub dwCmcDataReference: u32,
    pub cCertReference: u32,
    pub rgdwCertReference: *mut u32,
    pub cExtension: u32,
    pub rgExtension: *mut CERT_EXTENSION,
}
impl Default for CMC_ADD_EXTENSIONS_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CMC_DATA: windows_core::PCSTR = windows_core::PCSTR(59i32 as _);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CMC_DATA_INFO {
    pub cTaggedAttribute: u32,
    pub rgTaggedAttribute: *mut CMC_TAGGED_ATTRIBUTE,
    pub cTaggedRequest: u32,
    pub rgTaggedRequest: *mut CMC_TAGGED_REQUEST,
    pub cTaggedContentInfo: u32,
    pub rgTaggedContentInfo: *mut CMC_TAGGED_CONTENT_INFO,
    pub cTaggedOtherMsg: u32,
    pub rgTaggedOtherMsg: *mut CMC_TAGGED_OTHER_MSG,
}
impl Default for CMC_DATA_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CMC_FAIL_BAD_ALG: u32 = 0u32;
pub const CMC_FAIL_BAD_CERT_ID: u32 = 4u32;
pub const CMC_FAIL_BAD_IDENTITY: u32 = 7u32;
pub const CMC_FAIL_BAD_MESSAGE_CHECK: u32 = 1u32;
pub const CMC_FAIL_BAD_REQUEST: u32 = 2u32;
pub const CMC_FAIL_BAD_TIME: u32 = 3u32;
pub const CMC_FAIL_INTERNAL_CA_ERROR: u32 = 11u32;
pub const CMC_FAIL_MUST_ARCHIVE_KEYS: u32 = 6u32;
pub const CMC_FAIL_NO_KEY_REUSE: u32 = 10u32;
pub const CMC_FAIL_POP_FAILED: u32 = 9u32;
pub const CMC_FAIL_POP_REQUIRED: u32 = 8u32;
pub const CMC_FAIL_TRY_LATER: u32 = 12u32;
pub const CMC_FAIL_UNSUPORTED_EXT: u32 = 5u32;
pub const CMC_OTHER_INFO_FAIL_CHOICE: u32 = 1u32;
pub const CMC_OTHER_INFO_NO_CHOICE: u32 = 0u32;
pub const CMC_OTHER_INFO_PEND_CHOICE: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CMC_PEND_INFO {
    pub PendToken: CRYPT_INTEGER_BLOB,
    pub PendTime: super::super::Foundation::FILETIME,
}
pub const CMC_RESPONSE: windows_core::PCSTR = windows_core::PCSTR(60i32 as _);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CMC_RESPONSE_INFO {
    pub cTaggedAttribute: u32,
    pub rgTaggedAttribute: *mut CMC_TAGGED_ATTRIBUTE,
    pub cTaggedContentInfo: u32,
    pub rgTaggedContentInfo: *mut CMC_TAGGED_CONTENT_INFO,
    pub cTaggedOtherMsg: u32,
    pub rgTaggedOtherMsg: *mut CMC_TAGGED_OTHER_MSG,
}
impl Default for CMC_RESPONSE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CMC_STATUS: windows_core::PCSTR = windows_core::PCSTR(61i32 as _);
pub const CMC_STATUS_CONFIRM_REQUIRED: u32 = 5u32;
pub const CMC_STATUS_FAILED: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CMC_STATUS_INFO {
    pub dwStatus: u32,
    pub cBodyList: u32,
    pub rgdwBodyList: *mut u32,
    pub pwszStatusString: windows_core::PWSTR,
    pub dwOtherInfoChoice: u32,
    pub Anonymous: CMC_STATUS_INFO_0,
}
impl Default for CMC_STATUS_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CMC_STATUS_INFO_0 {
    pub dwFailInfo: u32,
    pub pPendInfo: *mut CMC_PEND_INFO,
}
impl Default for CMC_STATUS_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CMC_STATUS_NO_SUPPORT: u32 = 4u32;
pub const CMC_STATUS_PENDING: u32 = 3u32;
pub const CMC_STATUS_SUCCESS: u32 = 0u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CMC_TAGGED_ATTRIBUTE {
    pub dwBodyPartID: u32,
    pub Attribute: CRYPT_ATTRIBUTE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CMC_TAGGED_CERT_REQUEST {
    pub dwBodyPartID: u32,
    pub SignedCertRequest: CRYPT_INTEGER_BLOB,
}
pub const CMC_TAGGED_CERT_REQUEST_CHOICE: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CMC_TAGGED_CONTENT_INFO {
    pub dwBodyPartID: u32,
    pub EncodedContentInfo: CRYPT_INTEGER_BLOB,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CMC_TAGGED_OTHER_MSG {
    pub dwBodyPartID: u32,
    pub pszObjId: windows_core::PSTR,
    pub Value: CRYPT_INTEGER_BLOB,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CMC_TAGGED_REQUEST {
    pub dwTaggedRequestChoice: u32,
    pub Anonymous: CMC_TAGGED_REQUEST_0,
}
impl Default for CMC_TAGGED_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CMC_TAGGED_REQUEST_0 {
    pub pTaggedCertRequest: *mut CMC_TAGGED_CERT_REQUEST,
}
impl Default for CMC_TAGGED_REQUEST_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CMSCEPSetup: windows_core::GUID = windows_core::GUID::from_u128(0xaa4f5c02_8e7c_49c4_94fa_67a5cc5eadb4);
pub const CMSG_ATTR_CERT_COUNT_PARAM: u32 = 31u32;
pub const CMSG_ATTR_CERT_PARAM: u32 = 32u32;
pub const CMSG_AUTHENTICATED_ATTRIBUTES_FLAG: u32 = 8u32;
pub const CMSG_BARE_CONTENT_FLAG: u32 = 1u32;
pub const CMSG_BARE_CONTENT_PARAM: u32 = 3u32;
pub const CMSG_CERT_COUNT_PARAM: u32 = 11u32;
pub const CMSG_CERT_PARAM: u32 = 12u32;
pub const CMSG_CMS_ENCAPSULATED_CONTENT_FLAG: u32 = 64u32;
pub const CMSG_CMS_ENCAPSULATED_CTL_FLAG: u32 = 32768u32;
pub const CMSG_CMS_RECIPIENT_COUNT_PARAM: u32 = 33u32;
pub const CMSG_CMS_RECIPIENT_ENCRYPTED_KEY_INDEX_PARAM: u32 = 35u32;
pub const CMSG_CMS_RECIPIENT_INDEX_PARAM: u32 = 34u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CMSG_CMS_RECIPIENT_INFO {
    pub dwRecipientChoice: u32,
    pub Anonymous: CMSG_CMS_RECIPIENT_INFO_0,
}
impl Default for CMSG_CMS_RECIPIENT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CMSG_CMS_RECIPIENT_INFO_0 {
    pub pKeyTrans: *mut CMSG_KEY_TRANS_RECIPIENT_INFO,
    pub pKeyAgree: *mut CMSG_KEY_AGREE_RECIPIENT_INFO,
    pub pMailList: *mut CMSG_MAIL_LIST_RECIPIENT_INFO,
}
impl Default for CMSG_CMS_RECIPIENT_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CMSG_CMS_RECIPIENT_INFO_PARAM: u32 = 36u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CMSG_CMS_SIGNER_INFO {
    pub dwVersion: u32,
    pub SignerId: CERT_ID,
    pub HashAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub HashEncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub EncryptedHash: CRYPT_INTEGER_BLOB,
    pub AuthAttrs: CRYPT_ATTRIBUTES,
    pub UnauthAttrs: CRYPT_ATTRIBUTES,
}
impl Default for CMSG_CMS_SIGNER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CMSG_CMS_SIGNER_INFO_PARAM: u32 = 39u32;
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CMSG_CNG_CONTENT_DECRYPT_INFO {
    pub cbSize: u32,
    pub ContentEncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub pfnAlloc: PFN_CMSG_ALLOC,
    pub pfnFree: PFN_CMSG_FREE,
    pub hNCryptKey: NCRYPT_KEY_HANDLE,
    pub pbContentEncryptKey: *mut u8,
    pub cbContentEncryptKey: u32,
    pub hCNGContentEncryptKey: BCRYPT_KEY_HANDLE,
    pub pbCNGContentEncryptKeyObject: *mut u8,
}
impl Default for CMSG_CNG_CONTENT_DECRYPT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CMSG_COMPUTED_HASH_PARAM: u32 = 22u32;
pub const CMSG_CONTENTS_OCTETS_FLAG: u32 = 16u32;
pub const CMSG_CONTENT_ENCRYPT_FREE_OBJID_FLAG: u32 = 2u32;
pub const CMSG_CONTENT_ENCRYPT_FREE_PARA_FLAG: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CMSG_CONTENT_ENCRYPT_INFO {
    pub cbSize: u32,
    pub hCryptProv: HCRYPTPROV_LEGACY,
    pub ContentEncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub pvEncryptionAuxInfo: *mut core::ffi::c_void,
    pub cRecipients: u32,
    pub rgCmsRecipients: *mut CMSG_RECIPIENT_ENCODE_INFO,
    pub pfnAlloc: PFN_CMSG_ALLOC,
    pub pfnFree: PFN_CMSG_FREE,
    pub dwEncryptFlags: u32,
    pub Anonymous: CMSG_CONTENT_ENCRYPT_INFO_0,
    pub dwFlags: u32,
    pub fCNG: windows_core::BOOL,
    pub pbCNGContentEncryptKeyObject: *mut u8,
    pub pbContentEncryptKey: *mut u8,
    pub cbContentEncryptKey: u32,
}
impl Default for CMSG_CONTENT_ENCRYPT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CMSG_CONTENT_ENCRYPT_INFO_0 {
    pub hContentEncryptKey: usize,
    pub hCNGContentEncryptKey: BCRYPT_KEY_HANDLE,
}
impl Default for CMSG_CONTENT_ENCRYPT_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CMSG_CONTENT_ENCRYPT_PAD_ENCODED_LEN_FLAG: u32 = 1u32;
pub const CMSG_CONTENT_ENCRYPT_RELEASE_CONTEXT_FLAG: u32 = 32768u32;
pub const CMSG_CONTENT_PARAM: u32 = 2u32;
pub const CMSG_CRL_COUNT_PARAM: u32 = 13u32;
pub const CMSG_CRL_PARAM: u32 = 14u32;
pub const CMSG_CRYPT_RELEASE_CONTEXT_FLAG: u32 = 32768u32;
pub const CMSG_CTRL_ADD_ATTR_CERT: u32 = 14u32;
pub const CMSG_CTRL_ADD_CERT: u32 = 10u32;
pub const CMSG_CTRL_ADD_CMS_SIGNER_INFO: u32 = 20u32;
pub const CMSG_CTRL_ADD_CRL: u32 = 12u32;
pub const CMSG_CTRL_ADD_SIGNER: u32 = 6u32;
pub const CMSG_CTRL_ADD_SIGNER_UNAUTH_ATTR: u32 = 8u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CMSG_CTRL_ADD_SIGNER_UNAUTH_ATTR_PARA {
    pub cbSize: u32,
    pub dwSignerIndex: u32,
    pub blob: CRYPT_INTEGER_BLOB,
}
pub const CMSG_CTRL_DECRYPT: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CMSG_CTRL_DECRYPT_PARA {
    pub cbSize: u32,
    pub Anonymous: CMSG_CTRL_DECRYPT_PARA_0,
    pub dwKeySpec: u32,
    pub dwRecipientIndex: u32,
}
impl Default for CMSG_CTRL_DECRYPT_PARA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CMSG_CTRL_DECRYPT_PARA_0 {
    pub hCryptProv: usize,
    pub hNCryptKey: NCRYPT_KEY_HANDLE,
}
impl Default for CMSG_CTRL_DECRYPT_PARA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CMSG_CTRL_DEL_ATTR_CERT: u32 = 15u32;
pub const CMSG_CTRL_DEL_CERT: u32 = 11u32;
pub const CMSG_CTRL_DEL_CRL: u32 = 13u32;
pub const CMSG_CTRL_DEL_SIGNER: u32 = 7u32;
pub const CMSG_CTRL_DEL_SIGNER_UNAUTH_ATTR: u32 = 9u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CMSG_CTRL_DEL_SIGNER_UNAUTH_ATTR_PARA {
    pub cbSize: u32,
    pub dwSignerIndex: u32,
    pub dwUnauthAttrIndex: u32,
}
pub const CMSG_CTRL_ENABLE_STRONG_SIGNATURE: u32 = 21u32;
pub const CMSG_CTRL_KEY_AGREE_DECRYPT: u32 = 17u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CMSG_CTRL_KEY_AGREE_DECRYPT_PARA {
    pub cbSize: u32,
    pub Anonymous: CMSG_CTRL_KEY_AGREE_DECRYPT_PARA_0,
    pub dwKeySpec: u32,
    pub pKeyAgree: *mut CMSG_KEY_AGREE_RECIPIENT_INFO,
    pub dwRecipientIndex: u32,
    pub dwRecipientEncryptedKeyIndex: u32,
    pub OriginatorPublicKey: CRYPT_BIT_BLOB,
}
impl Default for CMSG_CTRL_KEY_AGREE_DECRYPT_PARA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CMSG_CTRL_KEY_AGREE_DECRYPT_PARA_0 {
    pub hCryptProv: usize,
    pub hNCryptKey: NCRYPT_KEY_HANDLE,
}
impl Default for CMSG_CTRL_KEY_AGREE_DECRYPT_PARA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CMSG_CTRL_KEY_TRANS_DECRYPT: u32 = 16u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CMSG_CTRL_KEY_TRANS_DECRYPT_PARA {
    pub cbSize: u32,
    pub Anonymous: CMSG_CTRL_KEY_TRANS_DECRYPT_PARA_0,
    pub dwKeySpec: u32,
    pub pKeyTrans: *mut CMSG_KEY_TRANS_RECIPIENT_INFO,
    pub dwRecipientIndex: u32,
}
impl Default for CMSG_CTRL_KEY_TRANS_DECRYPT_PARA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CMSG_CTRL_KEY_TRANS_DECRYPT_PARA_0 {
    pub hCryptProv: usize,
    pub hNCryptKey: NCRYPT_KEY_HANDLE,
}
impl Default for CMSG_CTRL_KEY_TRANS_DECRYPT_PARA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CMSG_CTRL_MAIL_LIST_DECRYPT: u32 = 18u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CMSG_CTRL_MAIL_LIST_DECRYPT_PARA {
    pub cbSize: u32,
    pub hCryptProv: usize,
    pub pMailList: *mut CMSG_MAIL_LIST_RECIPIENT_INFO,
    pub dwRecipientIndex: u32,
    pub dwKeyChoice: u32,
    pub Anonymous: CMSG_CTRL_MAIL_LIST_DECRYPT_PARA_0,
}
impl Default for CMSG_CTRL_MAIL_LIST_DECRYPT_PARA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CMSG_CTRL_MAIL_LIST_DECRYPT_PARA_0 {
    pub hKeyEncryptionKey: usize,
    pub pvKeyEncryptionKey: *mut core::ffi::c_void,
}
impl Default for CMSG_CTRL_MAIL_LIST_DECRYPT_PARA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CMSG_CTRL_VERIFY_HASH: u32 = 5u32;
pub const CMSG_CTRL_VERIFY_SIGNATURE: u32 = 1u32;
pub const CMSG_CTRL_VERIFY_SIGNATURE_EX: u32 = 19u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CMSG_CTRL_VERIFY_SIGNATURE_EX_PARA {
    pub cbSize: u32,
    pub hCryptProv: HCRYPTPROV_LEGACY,
    pub dwSignerIndex: u32,
    pub dwSignerType: u32,
    pub pvSigner: *mut core::ffi::c_void,
}
impl Default for CMSG_CTRL_VERIFY_SIGNATURE_EX_PARA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CMSG_DATA: CRYPT_MSG_TYPE = CRYPT_MSG_TYPE(1u32);
pub const CMSG_DEFAULT_INSTALLABLE_FUNC_OID: windows_core::PCSTR = windows_core::PCSTR(1i32 as _);
pub const CMSG_DETACHED_FLAG: u32 = 4u32;
pub const CMSG_ENCODED_MESSAGE: u32 = 29u32;
pub const CMSG_ENCODED_SIGNER: u32 = 28u32;
pub const CMSG_ENCODE_HASHED_SUBJECT_IDENTIFIER_FLAG: u32 = 2u32;
pub const CMSG_ENCODE_SORTED_CTL_FLAG: u32 = 1u32;
pub const CMSG_ENCODING_TYPE_MASK: u32 = 4294901760u32;
pub const CMSG_ENCRYPTED: u32 = 6u32;
pub const CMSG_ENCRYPTED_DIGEST: u32 = 27u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CMSG_ENCRYPTED_ENCODE_INFO {
    pub cbSize: u32,
    pub ContentEncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub pvEncryptionAuxInfo: *mut core::ffi::c_void,
}
impl Default for CMSG_ENCRYPTED_ENCODE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CMSG_ENCRYPT_PARAM: u32 = 26u32;
pub const CMSG_ENVELOPED: CRYPT_MSG_TYPE = CRYPT_MSG_TYPE(3u32);
pub const CMSG_ENVELOPED_DATA_CMS_VERSION: u32 = 2u32;
pub const CMSG_ENVELOPED_DATA_PKCS_1_5_VERSION: u32 = 0u32;
pub const CMSG_ENVELOPED_DATA_V0: u32 = 0u32;
pub const CMSG_ENVELOPED_DATA_V2: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CMSG_ENVELOPED_ENCODE_INFO {
    pub cbSize: u32,
    pub hCryptProv: HCRYPTPROV_LEGACY,
    pub ContentEncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub pvEncryptionAuxInfo: *mut core::ffi::c_void,
    pub cRecipients: u32,
    pub rgpRecipients: *mut *mut CERT_INFO,
}
impl Default for CMSG_ENVELOPED_ENCODE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CMSG_ENVELOPED_RECIPIENT_V0: u32 = 0u32;
pub const CMSG_ENVELOPED_RECIPIENT_V2: u32 = 2u32;
pub const CMSG_ENVELOPED_RECIPIENT_V3: u32 = 3u32;
pub const CMSG_ENVELOPED_RECIPIENT_V4: u32 = 4u32;
pub const CMSG_ENVELOPE_ALGORITHM_PARAM: u32 = 15u32;
pub const CMSG_HASHED: CRYPT_MSG_TYPE = CRYPT_MSG_TYPE(5u32);
pub const CMSG_HASHED_DATA_CMS_VERSION: u32 = 2u32;
pub const CMSG_HASHED_DATA_PKCS_1_5_VERSION: u32 = 0u32;
pub const CMSG_HASHED_DATA_V0: u32 = 0u32;
pub const CMSG_HASHED_DATA_V2: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CMSG_HASHED_ENCODE_INFO {
    pub cbSize: u32,
    pub hCryptProv: HCRYPTPROV_LEGACY,
    pub HashAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub pvHashAuxInfo: *mut core::ffi::c_void,
}
impl Default for CMSG_HASHED_ENCODE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CMSG_HASH_ALGORITHM_PARAM: u32 = 20u32;
pub const CMSG_HASH_DATA_PARAM: u32 = 21u32;
pub const CMSG_INDEFINITE_LENGTH: u32 = 4294967295u32;
pub const CMSG_INNER_CONTENT_TYPE_PARAM: u32 = 4u32;
pub const CMSG_KEY_AGREE_ENCRYPT_FREE_MATERIAL_FLAG: u32 = 2u32;
pub const CMSG_KEY_AGREE_ENCRYPT_FREE_OBJID_FLAG: u32 = 32u32;
pub const CMSG_KEY_AGREE_ENCRYPT_FREE_PARA_FLAG: u32 = 1u32;
pub const CMSG_KEY_AGREE_ENCRYPT_FREE_PUBKEY_ALG_FLAG: u32 = 4u32;
pub const CMSG_KEY_AGREE_ENCRYPT_FREE_PUBKEY_BITS_FLAG: u32 = 16u32;
pub const CMSG_KEY_AGREE_ENCRYPT_FREE_PUBKEY_PARA_FLAG: u32 = 8u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CMSG_KEY_AGREE_ENCRYPT_INFO {
    pub cbSize: u32,
    pub dwRecipientIndex: u32,
    pub KeyEncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub UserKeyingMaterial: CRYPT_INTEGER_BLOB,
    pub dwOriginatorChoice: CMSG_KEY_AGREE_ORIGINATOR,
    pub Anonymous: CMSG_KEY_AGREE_ENCRYPT_INFO_0,
    pub cKeyAgreeKeyEncryptInfo: u32,
    pub rgpKeyAgreeKeyEncryptInfo: *mut *mut CMSG_KEY_AGREE_KEY_ENCRYPT_INFO,
    pub dwFlags: u32,
}
impl Default for CMSG_KEY_AGREE_ENCRYPT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CMSG_KEY_AGREE_ENCRYPT_INFO_0 {
    pub OriginatorCertId: CERT_ID,
    pub OriginatorPublicKeyInfo: CERT_PUBLIC_KEY_INFO,
}
impl Default for CMSG_KEY_AGREE_ENCRYPT_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CMSG_KEY_AGREE_EPHEMERAL_KEY_CHOICE: CMSG_KEY_AGREE_OPTION = CMSG_KEY_AGREE_OPTION(1u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CMSG_KEY_AGREE_KEY_ENCRYPT_INFO {
    pub cbSize: u32,
    pub EncryptedKey: CRYPT_INTEGER_BLOB,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CMSG_KEY_AGREE_OPTION(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CMSG_KEY_AGREE_ORIGINATOR(pub u32);
pub const CMSG_KEY_AGREE_ORIGINATOR_CERT: CMSG_KEY_AGREE_ORIGINATOR = CMSG_KEY_AGREE_ORIGINATOR(1u32);
pub const CMSG_KEY_AGREE_ORIGINATOR_PUBLIC_KEY: CMSG_KEY_AGREE_ORIGINATOR = CMSG_KEY_AGREE_ORIGINATOR(2u32);
pub const CMSG_KEY_AGREE_RECIPIENT: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CMSG_KEY_AGREE_RECIPIENT_ENCODE_INFO {
    pub cbSize: u32,
    pub KeyEncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub pvKeyEncryptionAuxInfo: *mut core::ffi::c_void,
    pub KeyWrapAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub pvKeyWrapAuxInfo: *mut core::ffi::c_void,
    pub hCryptProv: HCRYPTPROV_LEGACY,
    pub dwKeySpec: u32,
    pub dwKeyChoice: CMSG_KEY_AGREE_OPTION,
    pub Anonymous: CMSG_KEY_AGREE_RECIPIENT_ENCODE_INFO_0,
    pub UserKeyingMaterial: CRYPT_INTEGER_BLOB,
    pub cRecipientEncryptedKeys: u32,
    pub rgpRecipientEncryptedKeys: *mut *mut CMSG_RECIPIENT_ENCRYPTED_KEY_ENCODE_INFO,
}
impl Default for CMSG_KEY_AGREE_RECIPIENT_ENCODE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CMSG_KEY_AGREE_RECIPIENT_ENCODE_INFO_0 {
    pub pEphemeralAlgorithm: *mut CRYPT_ALGORITHM_IDENTIFIER,
    pub pSenderId: *mut CERT_ID,
}
impl Default for CMSG_KEY_AGREE_RECIPIENT_ENCODE_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CMSG_KEY_AGREE_RECIPIENT_INFO {
    pub dwVersion: u32,
    pub dwOriginatorChoice: CMSG_KEY_AGREE_ORIGINATOR,
    pub Anonymous: CMSG_KEY_AGREE_RECIPIENT_INFO_0,
    pub UserKeyingMaterial: CRYPT_INTEGER_BLOB,
    pub KeyEncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub cRecipientEncryptedKeys: u32,
    pub rgpRecipientEncryptedKeys: *mut *mut CMSG_RECIPIENT_ENCRYPTED_KEY_INFO,
}
impl Default for CMSG_KEY_AGREE_RECIPIENT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CMSG_KEY_AGREE_RECIPIENT_INFO_0 {
    pub OriginatorCertId: CERT_ID,
    pub OriginatorPublicKeyInfo: CERT_PUBLIC_KEY_INFO,
}
impl Default for CMSG_KEY_AGREE_RECIPIENT_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CMSG_KEY_AGREE_STATIC_KEY_CHOICE: CMSG_KEY_AGREE_OPTION = CMSG_KEY_AGREE_OPTION(2u32);
pub const CMSG_KEY_AGREE_VERSION: u32 = 3u32;
pub const CMSG_KEY_TRANS_CMS_VERSION: u32 = 2u32;
pub const CMSG_KEY_TRANS_ENCRYPT_FREE_OBJID_FLAG: u32 = 2u32;
pub const CMSG_KEY_TRANS_ENCRYPT_FREE_PARA_FLAG: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CMSG_KEY_TRANS_ENCRYPT_INFO {
    pub cbSize: u32,
    pub dwRecipientIndex: u32,
    pub KeyEncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub EncryptedKey: CRYPT_INTEGER_BLOB,
    pub dwFlags: u32,
}
pub const CMSG_KEY_TRANS_PKCS_1_5_VERSION: u32 = 0u32;
pub const CMSG_KEY_TRANS_RECIPIENT: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CMSG_KEY_TRANS_RECIPIENT_ENCODE_INFO {
    pub cbSize: u32,
    pub KeyEncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub pvKeyEncryptionAuxInfo: *mut core::ffi::c_void,
    pub hCryptProv: HCRYPTPROV_LEGACY,
    pub RecipientPublicKey: CRYPT_BIT_BLOB,
    pub RecipientId: CERT_ID,
}
impl Default for CMSG_KEY_TRANS_RECIPIENT_ENCODE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CMSG_KEY_TRANS_RECIPIENT_INFO {
    pub dwVersion: u32,
    pub RecipientId: CERT_ID,
    pub KeyEncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub EncryptedKey: CRYPT_INTEGER_BLOB,
}
impl Default for CMSG_KEY_TRANS_RECIPIENT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CMSG_LENGTH_ONLY_FLAG: u32 = 2u32;
pub const CMSG_MAIL_LIST_ENCRYPT_FREE_OBJID_FLAG: u32 = 2u32;
pub const CMSG_MAIL_LIST_ENCRYPT_FREE_PARA_FLAG: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CMSG_MAIL_LIST_ENCRYPT_INFO {
    pub cbSize: u32,
    pub dwRecipientIndex: u32,
    pub KeyEncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub EncryptedKey: CRYPT_INTEGER_BLOB,
    pub dwFlags: u32,
}
pub const CMSG_MAIL_LIST_HANDLE_KEY_CHOICE: u32 = 1u32;
pub const CMSG_MAIL_LIST_RECIPIENT: u32 = 3u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CMSG_MAIL_LIST_RECIPIENT_ENCODE_INFO {
    pub cbSize: u32,
    pub KeyEncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub pvKeyEncryptionAuxInfo: *mut core::ffi::c_void,
    pub hCryptProv: usize,
    pub dwKeyChoice: u32,
    pub Anonymous: CMSG_MAIL_LIST_RECIPIENT_ENCODE_INFO_0,
    pub KeyId: CRYPT_INTEGER_BLOB,
    pub Date: super::super::Foundation::FILETIME,
    pub pOtherAttr: *mut CRYPT_ATTRIBUTE_TYPE_VALUE,
}
impl Default for CMSG_MAIL_LIST_RECIPIENT_ENCODE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CMSG_MAIL_LIST_RECIPIENT_ENCODE_INFO_0 {
    pub hKeyEncryptionKey: usize,
    pub pvKeyEncryptionKey: *mut core::ffi::c_void,
}
impl Default for CMSG_MAIL_LIST_RECIPIENT_ENCODE_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CMSG_MAIL_LIST_RECIPIENT_INFO {
    pub dwVersion: u32,
    pub KeyId: CRYPT_INTEGER_BLOB,
    pub KeyEncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub EncryptedKey: CRYPT_INTEGER_BLOB,
    pub Date: super::super::Foundation::FILETIME,
    pub pOtherAttr: *mut CRYPT_ATTRIBUTE_TYPE_VALUE,
}
impl Default for CMSG_MAIL_LIST_RECIPIENT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CMSG_MAIL_LIST_VERSION: u32 = 4u32;
pub const CMSG_MAX_LENGTH_FLAG: u32 = 32u32;
pub const CMSG_OID_CAPI1_EXPORT_KEY_AGREE_FUNC: windows_core::PCWSTR = windows_core::w!("CryptMsgDllExportKeyAgree");
pub const CMSG_OID_CAPI1_EXPORT_KEY_TRANS_FUNC: windows_core::PCWSTR = windows_core::w!("CryptMsgDllExportKeyTrans");
pub const CMSG_OID_CAPI1_EXPORT_MAIL_LIST_FUNC: windows_core::PCWSTR = windows_core::w!("CryptMsgDllExportMailList");
pub const CMSG_OID_CAPI1_GEN_CONTENT_ENCRYPT_KEY_FUNC: windows_core::PCWSTR = windows_core::w!("CryptMsgDllGenContentEncryptKey");
pub const CMSG_OID_CAPI1_IMPORT_KEY_AGREE_FUNC: windows_core::PCWSTR = windows_core::w!("CryptMsgDllImportKeyAgree");
pub const CMSG_OID_CAPI1_IMPORT_KEY_TRANS_FUNC: windows_core::PCWSTR = windows_core::w!("CryptMsgDllImportKeyTrans");
pub const CMSG_OID_CAPI1_IMPORT_MAIL_LIST_FUNC: windows_core::PCWSTR = windows_core::w!("CryptMsgDllImportMailList");
pub const CMSG_OID_CNG_EXPORT_KEY_AGREE_FUNC: windows_core::PCSTR = windows_core::s!("CryptMsgDllCNGExportKeyAgree");
pub const CMSG_OID_CNG_EXPORT_KEY_TRANS_FUNC: windows_core::PCSTR = windows_core::s!("CryptMsgDllCNGExportKeyTrans");
pub const CMSG_OID_CNG_GEN_CONTENT_ENCRYPT_KEY_FUNC: windows_core::PCSTR = windows_core::s!("CryptMsgDllCNGGenContentEncryptKey");
pub const CMSG_OID_CNG_IMPORT_CONTENT_ENCRYPT_KEY_FUNC: windows_core::PCSTR = windows_core::s!("CryptMsgDllCNGImportContentEncryptKey");
pub const CMSG_OID_CNG_IMPORT_KEY_AGREE_FUNC: windows_core::PCSTR = windows_core::s!("CryptMsgDllCNGImportKeyAgree");
pub const CMSG_OID_CNG_IMPORT_KEY_TRANS_FUNC: windows_core::PCSTR = windows_core::s!("CryptMsgDllCNGImportKeyTrans");
pub const CMSG_OID_EXPORT_ENCRYPT_KEY_FUNC: windows_core::PCSTR = windows_core::s!("CryptMsgDllExportEncryptKey");
pub const CMSG_OID_EXPORT_KEY_AGREE_FUNC: windows_core::PCSTR = windows_core::s!("CryptMsgDllExportKeyAgree");
pub const CMSG_OID_EXPORT_KEY_TRANS_FUNC: windows_core::PCSTR = windows_core::s!("CryptMsgDllExportKeyTrans");
pub const CMSG_OID_EXPORT_MAIL_LIST_FUNC: windows_core::PCSTR = windows_core::s!("CryptMsgDllExportMailList");
pub const CMSG_OID_GEN_CONTENT_ENCRYPT_KEY_FUNC: windows_core::PCSTR = windows_core::s!("CryptMsgDllGenContentEncryptKey");
pub const CMSG_OID_GEN_ENCRYPT_KEY_FUNC: windows_core::PCSTR = windows_core::s!("CryptMsgDllGenEncryptKey");
pub const CMSG_OID_IMPORT_ENCRYPT_KEY_FUNC: windows_core::PCSTR = windows_core::s!("CryptMsgDllImportEncryptKey");
pub const CMSG_OID_IMPORT_KEY_AGREE_FUNC: windows_core::PCSTR = windows_core::s!("CryptMsgDllImportKeyAgree");
pub const CMSG_OID_IMPORT_KEY_TRANS_FUNC: windows_core::PCSTR = windows_core::s!("CryptMsgDllImportKeyTrans");
pub const CMSG_OID_IMPORT_MAIL_LIST_FUNC: windows_core::PCSTR = windows_core::s!("CryptMsgDllImportMailList");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CMSG_RC2_AUX_INFO {
    pub cbSize: u32,
    pub dwBitLen: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CMSG_RC4_AUX_INFO {
    pub cbSize: u32,
    pub dwBitLen: u32,
}
pub const CMSG_RC4_NO_SALT_FLAG: u32 = 1073741824u32;
pub const CMSG_RECIPIENT_COUNT_PARAM: u32 = 17u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CMSG_RECIPIENT_ENCODE_INFO {
    pub dwRecipientChoice: u32,
    pub Anonymous: CMSG_RECIPIENT_ENCODE_INFO_0,
}
impl Default for CMSG_RECIPIENT_ENCODE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CMSG_RECIPIENT_ENCODE_INFO_0 {
    pub pKeyTrans: *mut CMSG_KEY_TRANS_RECIPIENT_ENCODE_INFO,
    pub pKeyAgree: *mut CMSG_KEY_AGREE_RECIPIENT_ENCODE_INFO,
    pub pMailList: *mut CMSG_MAIL_LIST_RECIPIENT_ENCODE_INFO,
}
impl Default for CMSG_RECIPIENT_ENCODE_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CMSG_RECIPIENT_ENCRYPTED_KEY_ENCODE_INFO {
    pub cbSize: u32,
    pub RecipientPublicKey: CRYPT_BIT_BLOB,
    pub RecipientId: CERT_ID,
    pub Date: super::super::Foundation::FILETIME,
    pub pOtherAttr: *mut CRYPT_ATTRIBUTE_TYPE_VALUE,
}
impl Default for CMSG_RECIPIENT_ENCRYPTED_KEY_ENCODE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CMSG_RECIPIENT_ENCRYPTED_KEY_INFO {
    pub RecipientId: CERT_ID,
    pub EncryptedKey: CRYPT_INTEGER_BLOB,
    pub Date: super::super::Foundation::FILETIME,
    pub pOtherAttr: *mut CRYPT_ATTRIBUTE_TYPE_VALUE,
}
impl Default for CMSG_RECIPIENT_ENCRYPTED_KEY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CMSG_RECIPIENT_INDEX_PARAM: u32 = 18u32;
pub const CMSG_RECIPIENT_INFO_PARAM: u32 = 19u32;
pub const CMSG_SIGNED: CRYPT_MSG_TYPE = CRYPT_MSG_TYPE(2u32);
pub const CMSG_SIGNED_AND_ENVELOPED: CRYPT_MSG_TYPE = CRYPT_MSG_TYPE(4u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CMSG_SIGNED_AND_ENVELOPED_ENCODE_INFO {
    pub cbSize: u32,
    pub SignedInfo: CMSG_SIGNED_ENCODE_INFO,
    pub EnvelopedInfo: CMSG_ENVELOPED_ENCODE_INFO,
}
pub const CMSG_SIGNED_DATA_CMS_VERSION: u32 = 3u32;
pub const CMSG_SIGNED_DATA_NO_SIGN_FLAG: u32 = 128u32;
pub const CMSG_SIGNED_DATA_PKCS_1_5_VERSION: u32 = 1u32;
pub const CMSG_SIGNED_DATA_V1: u32 = 1u32;
pub const CMSG_SIGNED_DATA_V3: u32 = 3u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CMSG_SIGNED_ENCODE_INFO {
    pub cbSize: u32,
    pub cSigners: u32,
    pub rgSigners: *mut CMSG_SIGNER_ENCODE_INFO,
    pub cCertEncoded: u32,
    pub rgCertEncoded: *mut CRYPT_INTEGER_BLOB,
    pub cCrlEncoded: u32,
    pub rgCrlEncoded: *mut CRYPT_INTEGER_BLOB,
}
impl Default for CMSG_SIGNED_ENCODE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CMSG_SIGNER_AUTH_ATTR_PARAM: u32 = 9u32;
pub const CMSG_SIGNER_CERT_ID_PARAM: u32 = 38u32;
pub const CMSG_SIGNER_CERT_INFO_PARAM: u32 = 7u32;
pub const CMSG_SIGNER_COUNT_PARAM: u32 = 5u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CMSG_SIGNER_ENCODE_INFO {
    pub cbSize: u32,
    pub pCertInfo: *mut CERT_INFO,
    pub Anonymous: CMSG_SIGNER_ENCODE_INFO_0,
    pub dwKeySpec: u32,
    pub HashAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub pvHashAuxInfo: *mut core::ffi::c_void,
    pub cAuthAttr: u32,
    pub rgAuthAttr: *mut CRYPT_ATTRIBUTE,
    pub cUnauthAttr: u32,
    pub rgUnauthAttr: *mut CRYPT_ATTRIBUTE,
}
impl Default for CMSG_SIGNER_ENCODE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CMSG_SIGNER_ENCODE_INFO_0 {
    pub hCryptProv: usize,
    pub hNCryptKey: NCRYPT_KEY_HANDLE,
}
impl Default for CMSG_SIGNER_ENCODE_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CMSG_SIGNER_HASH_ALGORITHM_PARAM: u32 = 8u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CMSG_SIGNER_INFO {
    pub dwVersion: u32,
    pub Issuer: CRYPT_INTEGER_BLOB,
    pub SerialNumber: CRYPT_INTEGER_BLOB,
    pub HashAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub HashEncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub EncryptedHash: CRYPT_INTEGER_BLOB,
    pub AuthAttrs: CRYPT_ATTRIBUTES,
    pub UnauthAttrs: CRYPT_ATTRIBUTES,
}
pub const CMSG_SIGNER_INFO_CMS_VERSION: u32 = 3u32;
pub const CMSG_SIGNER_INFO_PARAM: u32 = 6u32;
pub const CMSG_SIGNER_INFO_PKCS_1_5_VERSION: u32 = 1u32;
pub const CMSG_SIGNER_INFO_V1: u32 = 1u32;
pub const CMSG_SIGNER_INFO_V3: u32 = 3u32;
pub const CMSG_SIGNER_ONLY_FLAG: u32 = 2u32;
pub const CMSG_SIGNER_UNAUTH_ATTR_PARAM: u32 = 10u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CMSG_SP3_COMPATIBLE_AUX_INFO {
    pub cbSize: u32,
    pub dwFlags: u32,
}
pub const CMSG_SP3_COMPATIBLE_ENCRYPT_FLAG: u32 = 2147483648u32;
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CMSG_STREAM_INFO {
    pub cbContent: u32,
    pub pfnStreamOutput: PFN_CMSG_STREAM_OUTPUT,
    pub pvArg: *mut core::ffi::c_void,
}
impl Default for CMSG_STREAM_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CMSG_TRUSTED_SIGNER_FLAG: u32 = 1u32;
pub const CMSG_TYPE_PARAM: u32 = 1u32;
pub const CMSG_UNPROTECTED_ATTR_PARAM: u32 = 37u32;
pub const CMSG_USE_SIGNER_INDEX_FLAG: u32 = 4u32;
pub const CMSG_VERIFY_COUNTER_SIGN_ENABLE_STRONG_FLAG: u32 = 1u32;
pub const CMSG_VERIFY_SIGNER_CERT: u32 = 2u32;
pub const CMSG_VERIFY_SIGNER_CHAIN: u32 = 3u32;
pub const CMSG_VERIFY_SIGNER_NULL: u32 = 4u32;
pub const CMSG_VERIFY_SIGNER_PUBKEY: u32 = 1u32;
pub const CMSG_VERSION_PARAM: u32 = 30u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CMS_DH_KEY_INFO {
    pub dwVersion: u32,
    pub Algid: ALG_ID,
    pub pszContentEncObjId: windows_core::PSTR,
    pub PubInfo: CRYPT_INTEGER_BLOB,
    pub pReserved: *mut core::ffi::c_void,
}
impl Default for CMS_DH_KEY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CMS_KEY_INFO {
    pub dwVersion: u32,
    pub Algid: ALG_ID,
    pub pbOID: *mut u8,
    pub cbOID: u32,
}
impl Default for CMS_KEY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CMS_SIGNER_INFO: windows_core::PCSTR = windows_core::PCSTR(501i32 as _);
pub const CNG_RSA_PRIVATE_KEY_BLOB: windows_core::PCSTR = windows_core::PCSTR(83i32 as _);
pub const CNG_RSA_PUBLIC_KEY_BLOB: windows_core::PCSTR = windows_core::PCSTR(72i32 as _);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CONTAINER_INFO {
    pub dwVersion: u32,
    pub dwReserved: u32,
    pub cbSigPublicKey: u32,
    pub pbSigPublicKey: *mut u8,
    pub cbKeyExPublicKey: u32,
    pub pbKeyExPublicKey: *mut u8,
}
impl Default for CONTAINER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CONTAINER_INFO_CURRENT_VERSION: u32 = 1u32;
pub const CONTAINER_MAP_DEFAULT_CONTAINER: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CONTAINER_MAP_RECORD {
    pub wszGuid: [u16; 40],
    pub bFlags: u8,
    pub bReserved: u8,
    pub wSigKeySizeBits: u16,
    pub wKeyExchangeKeySizeBits: u16,
}
impl Default for CONTAINER_MAP_RECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CONTAINER_MAP_VALID_CONTAINER: u32 = 1u32;
pub const CONTEXT_OID_CAPI2_ANY: windows_core::PCSTR = windows_core::PCSTR(5i32 as _);
pub const CONTEXT_OID_CERTIFICATE: windows_core::PCSTR = windows_core::PCSTR(1i32 as _);
pub const CONTEXT_OID_CREATE_OBJECT_CONTEXT_FUNC: windows_core::PCSTR = windows_core::s!("ContextDllCreateObjectContext");
pub const CONTEXT_OID_CRL: windows_core::PCSTR = windows_core::PCSTR(2i32 as _);
pub const CONTEXT_OID_CTL: windows_core::PCSTR = windows_core::PCSTR(3i32 as _);
pub const CONTEXT_OID_OCSP_RESP: windows_core::PCSTR = windows_core::PCSTR(6i32 as _);
pub const CONTEXT_OID_PKCS7: windows_core::PCSTR = windows_core::PCSTR(4i32 as _);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CPS_URLS {
    pub pszURL: windows_core::PWSTR,
    pub pAlgorithm: *mut CRYPT_ALGORITHM_IDENTIFIER,
    pub pDigest: *mut CRYPT_INTEGER_BLOB,
}
impl Default for CPS_URLS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CP_CACHE_MODE_GLOBAL_CACHE: u32 = 1u32;
pub const CP_CACHE_MODE_NO_CACHE: u32 = 3u32;
pub const CP_CACHE_MODE_SESSION_ONLY: u32 = 2u32;
pub const CP_CARD_AUTHENTICATED_STATE: windows_core::PCWSTR = windows_core::w!("Authenticated State");
pub const CP_CARD_CACHE_MODE: windows_core::PCWSTR = windows_core::w!("Cache Mode");
pub const CP_CARD_CAPABILITIES: windows_core::PCWSTR = windows_core::w!("Capabilities");
pub const CP_CARD_FREE_SPACE: windows_core::PCWSTR = windows_core::w!("Free Space");
pub const CP_CARD_GUID: windows_core::PCWSTR = windows_core::w!("Card Identifier");
pub const CP_CARD_KEYSIZES: windows_core::PCWSTR = windows_core::w!("Key Sizes");
pub const CP_CARD_LIST_PINS: windows_core::PCWSTR = windows_core::w!("PIN List");
pub const CP_CARD_PIN_INFO: windows_core::PCWSTR = windows_core::w!("PIN Information");
pub const CP_CARD_PIN_STRENGTH_CHANGE: windows_core::PCWSTR = windows_core::w!("PIN Strength Change");
pub const CP_CARD_PIN_STRENGTH_UNBLOCK: windows_core::PCWSTR = windows_core::w!("PIN Strength Unblock");
pub const CP_CARD_PIN_STRENGTH_VERIFY: windows_core::PCWSTR = windows_core::w!("PIN Strength Verify");
pub const CP_CARD_PIV: windows_core::PCWSTR = windows_core::w!("PIV Card");
pub const CP_CARD_READ_ONLY: windows_core::PCWSTR = windows_core::w!("Read Only Mode");
pub const CP_CARD_SERIAL_NO: windows_core::PCWSTR = windows_core::w!("Card Serial Number");
pub const CP_CHAINING_MODES: windows_core::PCWSTR = windows_core::w!("Chaining Modes");
pub const CP_ENUM_ALGORITHMS: windows_core::PCWSTR = windows_core::w!("Algorithms");
pub const CP_KEY_IMPORT_SUPPORT: windows_core::PCWSTR = windows_core::w!("Key Import Support");
pub const CP_PADDING_SCHEMES: windows_core::PCWSTR = windows_core::w!("Padding Schemes");
pub const CP_PARENT_WINDOW: windows_core::PCWSTR = windows_core::w!("Parent Window");
pub const CP_PIN_CONTEXT_STRING: windows_core::PCWSTR = windows_core::w!("PIN Context String");
pub const CP_PIV_CARD_CAPABILITY_CONTAINER: windows_core::PCWSTR = windows_core::w!("PIV CCC");
pub const CP_PIV_CARD_HOLDER_UNIQUE_IDENTIFIER: windows_core::PCWSTR = windows_core::w!("PIV CHUID");
pub const CP_PIV_CARD_HOLDER_UNSIGNED_UNIQUE_IDENTIFIER: windows_core::PCWSTR = windows_core::w!("PIV UCHUID");
pub const CP_PIV_CERTIFICATE: windows_core::PCWSTR = windows_core::w!("PIV Certificate");
pub const CP_PIV_FACIAL_IMAGE: windows_core::PCWSTR = windows_core::w!("PIV Facial Image");
pub const CP_PIV_FINGERPRINT: windows_core::PCWSTR = windows_core::w!("PIV Fingerprint");
pub const CP_PIV_GENERATE_KEY: windows_core::PCWSTR = windows_core::w!("PIV Generate Key");
pub const CP_PIV_KEY_HISTORY_OBJECT: windows_core::PCWSTR = windows_core::w!("PIV Key History Object");
pub const CP_PIV_PRINTED_INFORMATION: windows_core::PCWSTR = windows_core::w!("PIV Printed Information");
pub const CP_PIV_PUBLIC_KEY: windows_core::PCWSTR = windows_core::w!("PIV Public Key");
pub const CP_PIV_SECURITY_OBJECT: windows_core::PCWSTR = windows_core::w!("PIV Security Object");
pub const CP_SUPPORTS_WIN_X509_ENROLLMENT: windows_core::PCWSTR = windows_core::w!("Supports Windows x.509 Enrollment");
pub const CREDENTIAL_OID_PASSWORD_CREDENTIALS: i32 = 2i32;
pub const CREDENTIAL_OID_PASSWORD_CREDENTIALS_A: windows_core::PCSTR = windows_core::PCSTR(1i32 as _);
pub const CREDENTIAL_OID_PASSWORD_CREDENTIALS_W: windows_core::PCSTR = windows_core::PCSTR(2i32 as _);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRL_CONTEXT {
    pub dwCertEncodingType: CERT_QUERY_ENCODING_TYPE,
    pub pbCrlEncoded: *mut u8,
    pub cbCrlEncoded: u32,
    pub pCrlInfo: *mut CRL_INFO,
    pub hCertStore: HCERTSTORE,
}
impl Default for CRL_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRL_DIST_POINT {
    pub DistPointName: CRL_DIST_POINT_NAME,
    pub ReasonFlags: CRYPT_BIT_BLOB,
    pub CRLIssuer: CERT_ALT_NAME_INFO,
}
impl Default for CRL_DIST_POINT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRL_DIST_POINTS_INFO {
    pub cDistPoint: u32,
    pub rgDistPoint: *mut CRL_DIST_POINT,
}
impl Default for CRL_DIST_POINTS_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRL_DIST_POINT_ERR_CRL_ISSUER_BIT: i32 = -2147483648i32;
pub const CRL_DIST_POINT_ERR_INDEX_MASK: u32 = 127u32;
pub const CRL_DIST_POINT_ERR_INDEX_SHIFT: u32 = 24u32;
pub const CRL_DIST_POINT_FULL_NAME: u32 = 1u32;
pub const CRL_DIST_POINT_ISSUER_RDN_NAME: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRL_DIST_POINT_NAME {
    pub dwDistPointNameChoice: u32,
    pub Anonymous: CRL_DIST_POINT_NAME_0,
}
impl Default for CRL_DIST_POINT_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CRL_DIST_POINT_NAME_0 {
    pub FullName: CERT_ALT_NAME_INFO,
}
impl Default for CRL_DIST_POINT_NAME_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRL_DIST_POINT_NO_NAME: u32 = 0u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRL_ENTRY {
    pub SerialNumber: CRYPT_INTEGER_BLOB,
    pub RevocationDate: super::super::Foundation::FILETIME,
    pub cExtension: u32,
    pub rgExtension: *mut CERT_EXTENSION,
}
impl Default for CRL_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRL_FIND_ANY: u32 = 0u32;
pub const CRL_FIND_EXISTING: u32 = 2u32;
pub const CRL_FIND_ISSUED_BY: u32 = 1u32;
pub const CRL_FIND_ISSUED_BY_AKI_FLAG: u32 = 1u32;
pub const CRL_FIND_ISSUED_BY_BASE_FLAG: u32 = 8u32;
pub const CRL_FIND_ISSUED_BY_DELTA_FLAG: u32 = 4u32;
pub const CRL_FIND_ISSUED_BY_SIGNATURE_FLAG: u32 = 2u32;
pub const CRL_FIND_ISSUED_FOR: u32 = 3u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRL_FIND_ISSUED_FOR_PARA {
    pub pSubjectCert: *const CERT_CONTEXT,
    pub pIssuerCert: *const CERT_CONTEXT,
}
impl Default for CRL_FIND_ISSUED_FOR_PARA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRL_FIND_ISSUED_FOR_SET_STRONG_PROPERTIES_FLAG: u32 = 16u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRL_INFO {
    pub dwVersion: u32,
    pub SignatureAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub Issuer: CRYPT_INTEGER_BLOB,
    pub ThisUpdate: super::super::Foundation::FILETIME,
    pub NextUpdate: super::super::Foundation::FILETIME,
    pub cCRLEntry: u32,
    pub rgCRLEntry: *mut CRL_ENTRY,
    pub cExtension: u32,
    pub rgExtension: *mut CERT_EXTENSION,
}
impl Default for CRL_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRL_ISSUING_DIST_POINT {
    pub DistPointName: CRL_DIST_POINT_NAME,
    pub fOnlyContainsUserCerts: windows_core::BOOL,
    pub fOnlyContainsCACerts: windows_core::BOOL,
    pub OnlySomeReasonFlags: CRYPT_BIT_BLOB,
    pub fIndirectCRL: windows_core::BOOL,
}
impl Default for CRL_ISSUING_DIST_POINT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRL_REASON_AA_COMPROMISE: u32 = 10u32;
pub const CRL_REASON_AA_COMPROMISE_FLAG: u32 = 128u32;
pub const CRL_REASON_AFFILIATION_CHANGED: CERT_REVOCATION_STATUS_REASON = CERT_REVOCATION_STATUS_REASON(3u32);
pub const CRL_REASON_AFFILIATION_CHANGED_FLAG: u32 = 16u32;
pub const CRL_REASON_CA_COMPROMISE: CERT_REVOCATION_STATUS_REASON = CERT_REVOCATION_STATUS_REASON(2u32);
pub const CRL_REASON_CA_COMPROMISE_FLAG: u32 = 32u32;
pub const CRL_REASON_CERTIFICATE_HOLD: CERT_REVOCATION_STATUS_REASON = CERT_REVOCATION_STATUS_REASON(6u32);
pub const CRL_REASON_CERTIFICATE_HOLD_FLAG: u32 = 2u32;
pub const CRL_REASON_CESSATION_OF_OPERATION: CERT_REVOCATION_STATUS_REASON = CERT_REVOCATION_STATUS_REASON(5u32);
pub const CRL_REASON_CESSATION_OF_OPERATION_FLAG: u32 = 4u32;
pub const CRL_REASON_KEY_COMPROMISE: CERT_REVOCATION_STATUS_REASON = CERT_REVOCATION_STATUS_REASON(1u32);
pub const CRL_REASON_KEY_COMPROMISE_FLAG: u32 = 64u32;
pub const CRL_REASON_PRIVILEGE_WITHDRAWN: u32 = 9u32;
pub const CRL_REASON_PRIVILEGE_WITHDRAWN_FLAG: u32 = 1u32;
pub const CRL_REASON_REMOVE_FROM_CRL: CERT_REVOCATION_STATUS_REASON = CERT_REVOCATION_STATUS_REASON(8u32);
pub const CRL_REASON_SUPERSEDED: CERT_REVOCATION_STATUS_REASON = CERT_REVOCATION_STATUS_REASON(4u32);
pub const CRL_REASON_SUPERSEDED_FLAG: u32 = 8u32;
pub const CRL_REASON_UNSPECIFIED: CERT_REVOCATION_STATUS_REASON = CERT_REVOCATION_STATUS_REASON(0u32);
pub const CRL_REASON_UNUSED_FLAG: u32 = 128u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRL_REVOCATION_INFO {
    pub pCrlEntry: *mut CRL_ENTRY,
    pub pCrlContext: *mut CRL_CONTEXT,
    pub pCrlIssuerChain: *mut CERT_CHAIN_CONTEXT,
}
impl Default for CRL_REVOCATION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRL_V1: u32 = 0u32;
pub const CRL_V2: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CROSS_CERT_DIST_POINTS_INFO {
    pub dwSyncDeltaTime: u32,
    pub cDistPoint: u32,
    pub rgDistPoint: *mut CERT_ALT_NAME_INFO,
}
impl Default for CROSS_CERT_DIST_POINTS_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CROSS_CERT_DIST_POINT_ERR_INDEX_MASK: u32 = 255u32;
pub const CROSS_CERT_DIST_POINT_ERR_INDEX_SHIFT: u32 = 24u32;
pub const CRYPTNET_CACHED_OCSP_SWITCH_TO_CRL_COUNT_DEFAULT: u32 = 50u32;
pub const CRYPTNET_CACHED_OCSP_SWITCH_TO_CRL_COUNT_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("CryptnetCachedOcspSwitchToCrlCount");
pub const CRYPTNET_CRL_BEFORE_OCSP_ENABLE: u32 = 4294967295u32;
pub const CRYPTNET_CRL_PRE_FETCH_DISABLE_INFORMATION_EVENTS_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("DisableInformationEvents");
pub const CRYPTNET_CRL_PRE_FETCH_LOG_FILE_NAME_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("LogFileName");
pub const CRYPTNET_CRL_PRE_FETCH_MAX_AGE_SECONDS_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("MaxAgeSeconds");
pub const CRYPTNET_CRL_PRE_FETCH_MIN_AFTER_NEXT_UPDATE_SECONDS_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("MinAfterNextUpdateSeconds");
pub const CRYPTNET_CRL_PRE_FETCH_MIN_BEFORE_NEXT_UPDATE_SECONDS_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("MinBeforeNextUpdateSeconds");
pub const CRYPTNET_CRL_PRE_FETCH_PROCESS_NAME_LIST_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("ProcessNameList");
pub const CRYPTNET_CRL_PRE_FETCH_PUBLISH_BEFORE_NEXT_UPDATE_SECONDS_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("PublishBeforeNextUpdateSeconds");
pub const CRYPTNET_CRL_PRE_FETCH_PUBLISH_RANDOM_INTERVAL_SECONDS_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("PublishRandomIntervalSeconds");
pub const CRYPTNET_CRL_PRE_FETCH_TIMEOUT_SECONDS_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("TimeoutSeconds");
pub const CRYPTNET_CRL_PRE_FETCH_URL_LIST_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("PreFetchUrlList");
pub const CRYPTNET_MAX_CACHED_OCSP_PER_CRL_COUNT_DEFAULT: u32 = 500u32;
pub const CRYPTNET_MAX_CACHED_OCSP_PER_CRL_COUNT_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("CryptnetMaxCachedOcspPerCrlCount");
pub const CRYPTNET_OCSP_AFTER_CRL_DISABLE: u32 = 4294967295u32;
pub const CRYPTNET_PRE_FETCH_AFTER_CURRENT_TIME_PRE_FETCH_PERIOD_SECONDS_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("CryptnetPreFetchAfterCurrentTimePreFetchPeriodSeconds");
pub const CRYPTNET_PRE_FETCH_AFTER_PUBLISH_PRE_FETCH_DIVISOR_DEFAULT: u32 = 10u32;
pub const CRYPTNET_PRE_FETCH_AFTER_PUBLISH_PRE_FETCH_DIVISOR_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("CryptnetPreFetchAfterPublishPreFetchDivisor");
pub const CRYPTNET_PRE_FETCH_BEFORE_NEXT_UPDATE_PRE_FETCH_DIVISOR_DEFAULT: u32 = 20u32;
pub const CRYPTNET_PRE_FETCH_BEFORE_NEXT_UPDATE_PRE_FETCH_DIVISOR_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("CryptnetPreFetchBeforeNextUpdatePreFetchDivisor");
pub const CRYPTNET_PRE_FETCH_MAX_AFTER_NEXT_UPDATE_PRE_FETCH_PERIOD_SECONDS_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("CryptnetPreFetchMaxAfterNextUpdatePreFetchPeriodSeconds");
pub const CRYPTNET_PRE_FETCH_MAX_MAX_AGE_SECONDS_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("CryptnetPreFetchMaxMaxAgeSeconds");
pub const CRYPTNET_PRE_FETCH_MIN_AFTER_NEXT_UPDATE_PRE_FETCH_PERIOD_SECONDS_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("CryptnetPreFetchMinAfterNextUpdatePreFetchPeriodSeconds");
pub const CRYPTNET_PRE_FETCH_MIN_BEFORE_NEXT_UPDATE_PRE_FETCH_PERIOD_SECONDS_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("CryptnetPreFetchMinBeforeNextUpdatePreFetchSeconds");
pub const CRYPTNET_PRE_FETCH_MIN_MAX_AGE_SECONDS_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("CryptnetPreFetchMinMaxAgeSeconds");
pub const CRYPTNET_PRE_FETCH_MIN_OCSP_VALIDITY_PERIOD_SECONDS_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("CryptnetPreFetchMinOcspValidityPeriodSeconds");
pub const CRYPTNET_PRE_FETCH_RETRIEVAL_TIMEOUT_SECONDS_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("CryptnetPreFetchRetrievalTimeoutSeconds");
pub const CRYPTNET_PRE_FETCH_SCAN_AFTER_TRIGGER_DELAY_SECONDS_DEFAULT: u32 = 60u32;
pub const CRYPTNET_PRE_FETCH_SCAN_AFTER_TRIGGER_DELAY_SECONDS_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("CryptnetPreFetchScanAfterTriggerDelaySeconds");
pub const CRYPTNET_PRE_FETCH_TRIGGER_DISABLE: u32 = 4294967295u32;
pub const CRYPTNET_PRE_FETCH_TRIGGER_PERIOD_SECONDS_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("CryptnetPreFetchTriggerPeriodSeconds");
pub const CRYPTNET_PRE_FETCH_VALIDITY_PERIOD_AFTER_NEXT_UPDATE_PRE_FETCH_DIVISOR_DEFAULT: u32 = 10u32;
pub const CRYPTNET_PRE_FETCH_VALIDITY_PERIOD_AFTER_NEXT_UPDATE_PRE_FETCH_DIVISOR_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("CryptnetPreFetchValidityPeriodAfterNextUpdatePreFetchDivisor");
pub const CRYPTNET_URL_CACHE_DEFAULT_FLUSH: u32 = 0u32;
pub const CRYPTNET_URL_CACHE_DEFAULT_FLUSH_EXEMPT_SECONDS_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("CryptnetDefaultFlushExemptSeconds");
pub const CRYPTNET_URL_CACHE_DISABLE_FLUSH: u32 = 4294967295u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CRYPTNET_URL_CACHE_FLUSH_INFO {
    pub cbSize: u32,
    pub dwExemptSeconds: u32,
    pub ExpireTime: super::super::Foundation::FILETIME,
}
pub const CRYPTNET_URL_CACHE_PRE_FETCH_AUTOROOT_CAB: u32 = 5u32;
pub const CRYPTNET_URL_CACHE_PRE_FETCH_BLOB: u32 = 1u32;
pub const CRYPTNET_URL_CACHE_PRE_FETCH_CRL: u32 = 2u32;
pub const CRYPTNET_URL_CACHE_PRE_FETCH_DISALLOWED_CERT_CAB: u32 = 6u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CRYPTNET_URL_CACHE_PRE_FETCH_INFO {
    pub cbSize: u32,
    pub dwObjectType: u32,
    pub dwError: u32,
    pub dwReserved: u32,
    pub ThisUpdateTime: super::super::Foundation::FILETIME,
    pub NextUpdateTime: super::super::Foundation::FILETIME,
    pub PublishTime: super::super::Foundation::FILETIME,
}
pub const CRYPTNET_URL_CACHE_PRE_FETCH_NONE: u32 = 0u32;
pub const CRYPTNET_URL_CACHE_PRE_FETCH_OCSP: u32 = 3u32;
pub const CRYPTNET_URL_CACHE_PRE_FETCH_PIN_RULES_CAB: u32 = 7u32;
pub const CRYPTNET_URL_CACHE_RESPONSE_HTTP: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CRYPTNET_URL_CACHE_RESPONSE_INFO {
    pub cbSize: u32,
    pub wResponseType: u16,
    pub wResponseFlags: u16,
    pub LastModifiedTime: super::super::Foundation::FILETIME,
    pub dwMaxAge: u32,
    pub pwszETag: windows_core::PCWSTR,
    pub dwProxyId: u32,
}
pub const CRYPTNET_URL_CACHE_RESPONSE_NONE: u32 = 0u32;
pub const CRYPTNET_URL_CACHE_RESPONSE_VALIDATED: u32 = 32768u32;
pub const CRYPTPROTECTMEMORY_BLOCK_SIZE: u32 = 16u32;
pub const CRYPTPROTECTMEMORY_CROSS_PROCESS: u32 = 1u32;
pub const CRYPTPROTECTMEMORY_SAME_LOGON: u32 = 2u32;
pub const CRYPTPROTECTMEMORY_SAME_PROCESS: u32 = 0u32;
pub const CRYPTPROTECT_AUDIT: u32 = 16u32;
pub const CRYPTPROTECT_CRED_REGENERATE: u32 = 128u32;
pub const CRYPTPROTECT_CRED_SYNC: u32 = 8u32;
pub const CRYPTPROTECT_DEFAULT_PROVIDER: windows_core::GUID = windows_core::GUID::from_u128(0xdf9d8cd0_1501_11d1_8c7a_00c04fc297eb);
pub const CRYPTPROTECT_FIRST_RESERVED_FLAGVAL: u32 = 268435455u32;
pub const CRYPTPROTECT_LAST_RESERVED_FLAGVAL: u32 = 4294967295u32;
pub const CRYPTPROTECT_LOCAL_MACHINE: u32 = 4u32;
pub const CRYPTPROTECT_NO_RECOVERY: u32 = 32u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CRYPTPROTECT_PROMPTSTRUCT {
    pub cbSize: u32,
    pub dwPromptFlags: u32,
    pub hwndApp: super::super::Foundation::HWND,
    pub szPrompt: windows_core::PCWSTR,
}
pub const CRYPTPROTECT_PROMPT_ON_PROTECT: u32 = 2u32;
pub const CRYPTPROTECT_PROMPT_ON_UNPROTECT: u32 = 1u32;
pub const CRYPTPROTECT_PROMPT_REQUIRE_STRONG: u32 = 16u32;
pub const CRYPTPROTECT_PROMPT_RESERVED: u32 = 4u32;
pub const CRYPTPROTECT_PROMPT_STRONG: u32 = 8u32;
pub const CRYPTPROTECT_UI_FORBIDDEN: u32 = 1u32;
pub const CRYPTPROTECT_VERIFY_PROTECTION: u32 = 64u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPT_3DES_KEY_STATE {
    pub Key: [u8; 24],
    pub IV: [u8; 8],
    pub Feedback: [u8; 8],
}
impl Default for CRYPT_3DES_KEY_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_ACCUMULATIVE_TIMEOUT: u32 = 2048u32;
pub const CRYPT_ACQUIRE_ALLOW_NCRYPT_KEY_FLAG: CRYPT_ACQUIRE_FLAGS = CRYPT_ACQUIRE_FLAGS(65536u32);
pub const CRYPT_ACQUIRE_CACHE_FLAG: CRYPT_ACQUIRE_FLAGS = CRYPT_ACQUIRE_FLAGS(1u32);
pub const CRYPT_ACQUIRE_COMPARE_KEY_FLAG: CRYPT_ACQUIRE_FLAGS = CRYPT_ACQUIRE_FLAGS(4u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_ACQUIRE_FLAGS(pub u32);
impl CRYPT_ACQUIRE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CRYPT_ACQUIRE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CRYPT_ACQUIRE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CRYPT_ACQUIRE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CRYPT_ACQUIRE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CRYPT_ACQUIRE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CRYPT_ACQUIRE_NCRYPT_KEY_FLAGS_MASK: CRYPT_ACQUIRE_FLAGS = CRYPT_ACQUIRE_FLAGS(458752u32);
pub const CRYPT_ACQUIRE_NO_HEALING: CRYPT_ACQUIRE_FLAGS = CRYPT_ACQUIRE_FLAGS(8u32);
pub const CRYPT_ACQUIRE_ONLY_NCRYPT_KEY_FLAG: CRYPT_ACQUIRE_FLAGS = CRYPT_ACQUIRE_FLAGS(262144u32);
pub const CRYPT_ACQUIRE_PREFER_NCRYPT_KEY_FLAG: CRYPT_ACQUIRE_FLAGS = CRYPT_ACQUIRE_FLAGS(131072u32);
pub const CRYPT_ACQUIRE_SILENT_FLAG: CRYPT_ACQUIRE_FLAGS = CRYPT_ACQUIRE_FLAGS(64u32);
pub const CRYPT_ACQUIRE_USE_PROV_INFO_FLAG: CRYPT_ACQUIRE_FLAGS = CRYPT_ACQUIRE_FLAGS(2u32);
pub const CRYPT_ACQUIRE_WINDOW_HANDLE_FLAG: CRYPT_ACQUIRE_FLAGS = CRYPT_ACQUIRE_FLAGS(128u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPT_AES_128_KEY_STATE {
    pub Key: [u8; 16],
    pub IV: [u8; 16],
    pub EncryptionState: [u8; 176],
    pub DecryptionState: [u8; 176],
    pub Feedback: [u8; 16],
}
impl Default for CRYPT_AES_128_KEY_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPT_AES_256_KEY_STATE {
    pub Key: [u8; 32],
    pub IV: [u8; 16],
    pub EncryptionState: [u8; 240],
    pub DecryptionState: [u8; 240],
    pub Feedback: [u8; 16],
}
impl Default for CRYPT_AES_256_KEY_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_AIA_RETRIEVAL: u32 = 524288u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CRYPT_ALGORITHM_IDENTIFIER {
    pub pszObjId: windows_core::PSTR,
    pub Parameters: CRYPT_INTEGER_BLOB,
}
pub const CRYPT_ALL_FUNCTIONS: BCRYPT_RESOLVE_PROVIDERS_FLAGS = BCRYPT_RESOLVE_PROVIDERS_FLAGS(1u32);
pub const CRYPT_ALL_PROVIDERS: BCRYPT_RESOLVE_PROVIDERS_FLAGS = BCRYPT_RESOLVE_PROVIDERS_FLAGS(2u32);
pub const CRYPT_ANY: BCRYPT_QUERY_PROVIDER_MODE = BCRYPT_QUERY_PROVIDER_MODE(4u32);
pub const CRYPT_ARCHIVABLE: CRYPT_KEY_FLAGS = CRYPT_KEY_FLAGS(16384u32);
pub const CRYPT_ARCHIVE: u32 = 256u32;
pub const CRYPT_ASN_ENCODING: u32 = 1u32;
pub const CRYPT_ASYNC_RETRIEVAL: u32 = 16u32;
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CRYPT_ASYNC_RETRIEVAL_COMPLETION {
    pub pfnCompletion: PFN_CRYPT_ASYNC_RETRIEVAL_COMPLETION_FUNC,
    pub pvCompletion: *mut core::ffi::c_void,
}
impl Default for CRYPT_ASYNC_RETRIEVAL_COMPLETION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPT_ATTRIBUTE {
    pub pszObjId: windows_core::PSTR,
    pub cValue: u32,
    pub rgValue: *mut CRYPT_INTEGER_BLOB,
}
impl Default for CRYPT_ATTRIBUTE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPT_ATTRIBUTES {
    pub cAttr: u32,
    pub rgAttr: *mut CRYPT_ATTRIBUTE,
}
impl Default for CRYPT_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CRYPT_ATTRIBUTE_TYPE_VALUE {
    pub pszObjId: windows_core::PSTR,
    pub Value: CRYPT_INTEGER_BLOB,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPT_BIT_BLOB {
    pub cbData: u32,
    pub pbData: *mut u8,
    pub cUnusedBits: u32,
}
impl Default for CRYPT_BIT_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPT_BLOB_ARRAY {
    pub cBlob: u32,
    pub rgBlob: *mut CRYPT_INTEGER_BLOB,
}
impl Default for CRYPT_BLOB_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_BLOB_VER3: CRYPT_KEY_FLAGS = CRYPT_KEY_FLAGS(128u32);
pub const CRYPT_CACHE_ONLY_RETRIEVAL: u32 = 2u32;
pub const CRYPT_CHECK_FRESHNESS_TIME_VALIDITY: u32 = 1024u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CRYPT_CONTENT_INFO {
    pub pszObjId: windows_core::PSTR,
    pub Content: CRYPT_INTEGER_BLOB,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPT_CONTENT_INFO_SEQUENCE_OF_ANY {
    pub pszObjId: windows_core::PSTR,
    pub cValue: u32,
    pub rgValue: *mut CRYPT_INTEGER_BLOB,
}
impl Default for CRYPT_CONTENT_INFO_SEQUENCE_OF_ANY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPT_CONTEXTS {
    pub cContexts: u32,
    pub rgpszContexts: *mut windows_core::PWSTR,
}
impl Default for CRYPT_CONTEXTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CRYPT_CONTEXT_CONFIG {
    pub dwFlags: CRYPT_CONTEXT_CONFIG_FLAGS,
    pub dwReserved: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_CONTEXT_CONFIG_FLAGS(pub u32);
impl CRYPT_CONTEXT_CONFIG_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CRYPT_CONTEXT_CONFIG_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CRYPT_CONTEXT_CONFIG_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CRYPT_CONTEXT_CONFIG_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CRYPT_CONTEXT_CONFIG_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CRYPT_CONTEXT_CONFIG_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPT_CONTEXT_FUNCTIONS {
    pub cFunctions: u32,
    pub rgpszFunctions: *mut windows_core::PWSTR,
}
impl Default for CRYPT_CONTEXT_FUNCTIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CRYPT_CONTEXT_FUNCTION_CONFIG {
    pub dwFlags: u32,
    pub dwReserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPT_CONTEXT_FUNCTION_PROVIDERS {
    pub cProviders: u32,
    pub rgpszProviders: *mut windows_core::PWSTR,
}
impl Default for CRYPT_CONTEXT_FUNCTION_PROVIDERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_CREATE_IV: CRYPT_KEY_FLAGS = CRYPT_KEY_FLAGS(512u32);
pub const CRYPT_CREATE_NEW_FLUSH_ENTRY: u32 = 268435456u32;
pub const CRYPT_CREATE_SALT: CRYPT_KEY_FLAGS = CRYPT_KEY_FLAGS(4u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPT_CREDENTIALS {
    pub cbSize: u32,
    pub pszCredentialsOid: windows_core::PCSTR,
    pub pvCredentials: *mut core::ffi::c_void,
}
impl Default for CRYPT_CREDENTIALS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CRYPT_CSP_PROVIDER {
    pub dwKeySpec: u32,
    pub pwszProviderName: windows_core::PWSTR,
    pub Signature: CRYPT_BIT_BLOB,
}
pub const CRYPT_DATA_KEY: CRYPT_KEY_FLAGS = CRYPT_KEY_FLAGS(2048u32);
pub const CRYPT_DECODE_ALLOC_FLAG: u32 = 32768u32;
pub const CRYPT_DECODE_ENABLE_PUNYCODE_FLAG: u32 = 33554432u32;
pub const CRYPT_DECODE_ENABLE_UTF8PERCENT_FLAG: u32 = 67108864u32;
pub const CRYPT_DECODE_NOCOPY_FLAG: u32 = 1u32;
pub const CRYPT_DECODE_NO_SIGNATURE_BYTE_REVERSAL_FLAG: u32 = 8u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct CRYPT_DECODE_PARA {
    pub cbSize: u32,
    pub pfnAlloc: PFN_CRYPT_ALLOC,
    pub pfnFree: PFN_CRYPT_FREE,
}
pub const CRYPT_DECODE_SHARE_OID_STRING_FLAG: u32 = 4u32;
pub const CRYPT_DECODE_TO_BE_SIGNED_FLAG: u32 = 2u32;
pub const CRYPT_DECRYPT: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPT_DECRYPT_MESSAGE_PARA {
    pub cbSize: u32,
    pub dwMsgAndCertEncodingType: u32,
    pub cCertStore: u32,
    pub rghCertStore: *mut HCERTSTORE,
}
impl Default for CRYPT_DECRYPT_MESSAGE_PARA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_DECRYPT_RSA_NO_PADDING_CHECK: u32 = 32u32;
pub const CRYPT_DEFAULT_CONTAINER_OPTIONAL: u32 = 128u32;
pub const CRYPT_DEFAULT_CONTEXT: windows_core::PCWSTR = windows_core::w!("Default");
pub const CRYPT_DEFAULT_CONTEXT_AUTO_RELEASE_FLAG: CRYPT_DEFAULT_CONTEXT_FLAGS = CRYPT_DEFAULT_CONTEXT_FLAGS(1u32);
pub const CRYPT_DEFAULT_CONTEXT_CERT_SIGN_OID: CRYPT_DEFAULT_CONTEXT_TYPE = CRYPT_DEFAULT_CONTEXT_TYPE(1u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_DEFAULT_CONTEXT_FLAGS(pub u32);
impl CRYPT_DEFAULT_CONTEXT_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CRYPT_DEFAULT_CONTEXT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CRYPT_DEFAULT_CONTEXT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CRYPT_DEFAULT_CONTEXT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CRYPT_DEFAULT_CONTEXT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CRYPT_DEFAULT_CONTEXT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CRYPT_DEFAULT_CONTEXT_MULTI_CERT_SIGN_OID: CRYPT_DEFAULT_CONTEXT_TYPE = CRYPT_DEFAULT_CONTEXT_TYPE(2u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPT_DEFAULT_CONTEXT_MULTI_OID_PARA {
    pub cOID: u32,
    pub rgpszOID: *mut windows_core::PSTR,
}
impl Default for CRYPT_DEFAULT_CONTEXT_MULTI_OID_PARA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_DEFAULT_CONTEXT_PROCESS_FLAG: CRYPT_DEFAULT_CONTEXT_FLAGS = CRYPT_DEFAULT_CONTEXT_FLAGS(2u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_DEFAULT_CONTEXT_TYPE(pub u32);
pub const CRYPT_DEFAULT_OID: windows_core::PCSTR = windows_core::s!("DEFAULT");
pub const CRYPT_DELETEKEYSET: u32 = 16u32;
pub const CRYPT_DELETE_DEFAULT: u32 = 4u32;
pub const CRYPT_DELETE_KEYSET: u32 = 16u32;
pub const CRYPT_DESTROYKEY: CRYPT_KEY_FLAGS = CRYPT_KEY_FLAGS(4u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPT_DES_KEY_STATE {
    pub Key: [u8; 8],
    pub IV: [u8; 8],
    pub Feedback: [u8; 8],
}
impl Default for CRYPT_DES_KEY_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_DOMAIN: BCRYPT_TABLE = BCRYPT_TABLE(2u32);
pub const CRYPT_DONT_CACHE_RESULT: u32 = 8u32;
pub const CRYPT_DONT_CHECK_TIME_VALIDITY: u32 = 512u32;
pub const CRYPT_DONT_VERIFY_SIGNATURE: u32 = 256u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPT_ECC_CMS_SHARED_INFO {
    pub Algorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub EntityUInfo: CRYPT_INTEGER_BLOB,
    pub rgbSuppPubInfo: [u8; 4],
}
impl Default for CRYPT_ECC_CMS_SHARED_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_ECC_CMS_SHARED_INFO_SUPPPUBINFO_BYTE_LENGTH: u32 = 4u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CRYPT_ECC_PRIVATE_KEY_INFO {
    pub dwVersion: u32,
    pub PrivateKey: CRYPT_INTEGER_BLOB,
    pub szCurveOid: windows_core::PSTR,
    pub PublicKey: CRYPT_BIT_BLOB,
}
pub const CRYPT_ECC_PRIVATE_KEY_INFO_v1: u32 = 1u32;
pub const CRYPT_ENABLE_FILE_RETRIEVAL: u32 = 134217728u32;
pub const CRYPT_ENABLE_SSL_REVOCATION_RETRIEVAL: u32 = 8388608u32;
pub const CRYPT_ENCODE_ALLOC_FLAG: CRYPT_ENCODE_OBJECT_FLAGS = CRYPT_ENCODE_OBJECT_FLAGS(32768u32);
pub const CRYPT_ENCODE_DECODE_NONE: u32 = 0u32;
pub const CRYPT_ENCODE_ENABLE_PUNYCODE_FLAG: CRYPT_ENCODE_OBJECT_FLAGS = CRYPT_ENCODE_OBJECT_FLAGS(131072u32);
pub const CRYPT_ENCODE_ENABLE_UTF8PERCENT_FLAG: u32 = 262144u32;
pub const CRYPT_ENCODE_NO_SIGNATURE_BYTE_REVERSAL_FLAG: u32 = 8u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_ENCODE_OBJECT_FLAGS(pub u32);
impl CRYPT_ENCODE_OBJECT_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CRYPT_ENCODE_OBJECT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CRYPT_ENCODE_OBJECT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CRYPT_ENCODE_OBJECT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CRYPT_ENCODE_OBJECT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CRYPT_ENCODE_OBJECT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct CRYPT_ENCODE_PARA {
    pub cbSize: u32,
    pub pfnAlloc: PFN_CRYPT_ALLOC,
    pub pfnFree: PFN_CRYPT_FREE,
}
pub const CRYPT_ENCRYPT: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CRYPT_ENCRYPTED_PRIVATE_KEY_INFO {
    pub EncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub EncryptedPrivateKey: CRYPT_INTEGER_BLOB,
}
pub const CRYPT_ENCRYPT_ALG_OID_GROUP_ID: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPT_ENCRYPT_MESSAGE_PARA {
    pub cbSize: u32,
    pub dwMsgEncodingType: u32,
    pub hCryptProv: HCRYPTPROV_LEGACY,
    pub ContentEncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub pvEncryptionAuxInfo: *mut core::ffi::c_void,
    pub dwFlags: u32,
    pub dwInnerContentType: u32,
}
impl Default for CRYPT_ENCRYPT_MESSAGE_PARA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_ENHKEY_USAGE_OID_GROUP_ID: u32 = 7u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CRYPT_ENROLLMENT_NAME_VALUE_PAIR {
    pub pwszName: windows_core::PWSTR,
    pub pwszValue: windows_core::PWSTR,
}
pub const CRYPT_EXCLUSIVE: CRYPT_CONTEXT_CONFIG_FLAGS = CRYPT_CONTEXT_CONFIG_FLAGS(1u32);
pub const CRYPT_EXPORT: u32 = 4u32;
pub const CRYPT_EXPORTABLE: CRYPT_KEY_FLAGS = CRYPT_KEY_FLAGS(1u32);
pub const CRYPT_EXPORT_KEY: u32 = 64u32;
pub const CRYPT_EXTERNAL_SIGNATURE_LENGTH: u32 = 136u32;
pub const CRYPT_EXT_OR_ATTR_OID_GROUP_ID: u32 = 6u32;
pub const CRYPT_FAILED: u32 = 0u32;
pub const CRYPT_FASTSGC: u32 = 2u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_FIND_FLAGS(pub u32);
pub const CRYPT_FIND_MACHINE_KEYSET_FLAG: CRYPT_FIND_FLAGS = CRYPT_FIND_FLAGS(2u32);
pub const CRYPT_FIND_SILENT_KEYSET_FLAG: CRYPT_FIND_FLAGS = CRYPT_FIND_FLAGS(64u32);
pub const CRYPT_FIND_USER_KEYSET_FLAG: CRYPT_FIND_FLAGS = CRYPT_FIND_FLAGS(1u32);
pub const CRYPT_FIRST: u32 = 1u32;
pub const CRYPT_FIRST_ALG_OID_GROUP_ID: u32 = 1u32;
pub const CRYPT_FLAG_IPSEC: u32 = 16u32;
pub const CRYPT_FLAG_PCT1: u32 = 1u32;
pub const CRYPT_FLAG_SIGNING: u32 = 32u32;
pub const CRYPT_FLAG_SSL2: u32 = 2u32;
pub const CRYPT_FLAG_SSL3: u32 = 4u32;
pub const CRYPT_FLAG_TLS1: u32 = 8u32;
pub const CRYPT_FORCE_KEY_PROTECTION_HIGH: CRYPT_KEY_FLAGS = CRYPT_KEY_FLAGS(32768u32);
pub const CRYPT_FORMAT_COMMA: u32 = 4096u32;
pub const CRYPT_FORMAT_CRLF: u32 = 512u32;
pub const CRYPT_FORMAT_OID: u32 = 4u32;
pub const CRYPT_FORMAT_RDN_CRLF: u32 = 512u32;
pub const CRYPT_FORMAT_RDN_REVERSE: u32 = 2048u32;
pub const CRYPT_FORMAT_RDN_SEMICOLON: u32 = 256u32;
pub const CRYPT_FORMAT_RDN_UNQUOTE: u32 = 1024u32;
pub const CRYPT_FORMAT_SEMICOLON: u32 = 256u32;
pub const CRYPT_FORMAT_SIMPLE: u32 = 1u32;
pub const CRYPT_FORMAT_STR_MULTI_LINE: u32 = 1u32;
pub const CRYPT_FORMAT_STR_NO_HEX: u32 = 16u32;
pub const CRYPT_FORMAT_X509: u32 = 2u32;
pub const CRYPT_GET_INSTALLED_OID_FUNC_FLAG: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPT_GET_TIME_VALID_OBJECT_EXTRA_INFO {
    pub cbSize: u32,
    pub iDeltaCrlIndicator: i32,
    pub pftCacheResync: *mut super::super::Foundation::FILETIME,
    pub pLastSyncTime: *mut super::super::Foundation::FILETIME,
    pub pMaxAgeTime: *mut super::super::Foundation::FILETIME,
    pub pChainPara: *mut CERT_REVOCATION_CHAIN_PARA,
    pub pDeltaCrlIndicator: *mut CRYPT_INTEGER_BLOB,
}
impl Default for CRYPT_GET_TIME_VALID_OBJECT_EXTRA_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_GET_URL_FLAGS(pub u32);
impl CRYPT_GET_URL_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CRYPT_GET_URL_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CRYPT_GET_URL_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CRYPT_GET_URL_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CRYPT_GET_URL_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CRYPT_GET_URL_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CRYPT_GET_URL_FROM_AUTH_ATTRIBUTE: CRYPT_GET_URL_FLAGS = CRYPT_GET_URL_FLAGS(8u32);
pub const CRYPT_GET_URL_FROM_EXTENSION: CRYPT_GET_URL_FLAGS = CRYPT_GET_URL_FLAGS(2u32);
pub const CRYPT_GET_URL_FROM_PROPERTY: CRYPT_GET_URL_FLAGS = CRYPT_GET_URL_FLAGS(1u32);
pub const CRYPT_GET_URL_FROM_UNAUTH_ATTRIBUTE: CRYPT_GET_URL_FLAGS = CRYPT_GET_URL_FLAGS(4u32);
pub const CRYPT_HASH_ALG_OID_GROUP_ID: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CRYPT_HASH_INFO {
    pub HashAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub Hash: CRYPT_INTEGER_BLOB,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPT_HASH_MESSAGE_PARA {
    pub cbSize: u32,
    pub dwMsgEncodingType: u32,
    pub hCryptProv: HCRYPTPROV_LEGACY,
    pub HashAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub pvHashAuxInfo: *mut core::ffi::c_void,
}
impl Default for CRYPT_HASH_MESSAGE_PARA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_HTTP_POST_RETRIEVAL: u32 = 1048576u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CRYPT_IMAGE_REF {
    pub pszImage: windows_core::PWSTR,
    pub dwFlags: CRYPT_IMAGE_REF_FLAGS,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_IMAGE_REF_FLAGS(pub u32);
impl CRYPT_IMAGE_REF_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CRYPT_IMAGE_REF_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CRYPT_IMAGE_REF_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CRYPT_IMAGE_REF_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CRYPT_IMAGE_REF_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CRYPT_IMAGE_REF_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPT_IMAGE_REG {
    pub pszImage: windows_core::PWSTR,
    pub cInterfaces: u32,
    pub rgpInterfaces: *mut *mut CRYPT_INTERFACE_REG,
}
impl Default for CRYPT_IMAGE_REG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_IMPL_HARDWARE: u32 = 1u32;
pub const CRYPT_IMPL_MIXED: u32 = 3u32;
pub const CRYPT_IMPL_REMOVABLE: u32 = 8u32;
pub const CRYPT_IMPL_SOFTWARE: u32 = 2u32;
pub const CRYPT_IMPL_UNKNOWN: u32 = 4u32;
pub const CRYPT_IMPORT_KEY: u32 = 128u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_IMPORT_PUBLIC_KEY_FLAGS(pub u32);
impl CRYPT_IMPORT_PUBLIC_KEY_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CRYPT_IMPORT_PUBLIC_KEY_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CRYPT_IMPORT_PUBLIC_KEY_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CRYPT_IMPORT_PUBLIC_KEY_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CRYPT_IMPORT_PUBLIC_KEY_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CRYPT_IMPORT_PUBLIC_KEY_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CRYPT_INITIATOR: CRYPT_KEY_FLAGS = CRYPT_KEY_FLAGS(64u32);
pub const CRYPT_INSTALL_OID_FUNC_BEFORE_FLAG: u32 = 1u32;
pub const CRYPT_INSTALL_OID_INFO_BEFORE_FLAG: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPT_INTEGER_BLOB {
    pub cbData: u32,
    pub pbData: *mut u8,
}
impl Default for CRYPT_INTEGER_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPT_INTERFACE_REG {
    pub dwInterface: BCRYPT_INTERFACE,
    pub dwFlags: BCRYPT_TABLE,
    pub cFunctions: u32,
    pub rgpszFunctions: *mut windows_core::PWSTR,
}
impl Default for CRYPT_INTERFACE_REG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_IPSEC_HMAC_KEY: CRYPT_KEY_FLAGS = CRYPT_KEY_FLAGS(256u32);
pub const CRYPT_KDF_OID_GROUP_ID: u32 = 10u32;
pub const CRYPT_KEEP_TIME_VALID: u32 = 128u32;
pub const CRYPT_KEK: CRYPT_KEY_FLAGS = CRYPT_KEY_FLAGS(1024u32);
pub const CRYPT_KEYID_ALLOC_FLAG: u32 = 32768u32;
pub const CRYPT_KEYID_DELETE_FLAG: u32 = 16u32;
pub const CRYPT_KEYID_MACHINE_FLAG: u32 = 32u32;
pub const CRYPT_KEYID_SET_NEW_FLAG: u32 = 8192u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_KEY_FLAGS(pub u32);
impl CRYPT_KEY_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CRYPT_KEY_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CRYPT_KEY_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CRYPT_KEY_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CRYPT_KEY_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CRYPT_KEY_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_KEY_PARAM_ID(pub u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPT_KEY_PROV_INFO {
    pub pwszContainerName: windows_core::PWSTR,
    pub pwszProvName: windows_core::PWSTR,
    pub dwProvType: u32,
    pub dwFlags: CRYPT_KEY_FLAGS,
    pub cProvParam: u32,
    pub rgProvParam: *mut CRYPT_KEY_PROV_PARAM,
    pub dwKeySpec: u32,
}
impl Default for CRYPT_KEY_PROV_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPT_KEY_PROV_PARAM {
    pub dwParam: u32,
    pub pbData: *mut u8,
    pub cbData: u32,
    pub dwFlags: u32,
}
impl Default for CRYPT_KEY_PROV_PARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPT_KEY_SIGN_MESSAGE_PARA {
    pub cbSize: u32,
    pub dwMsgAndCertEncodingType: CERT_QUERY_ENCODING_TYPE,
    pub Anonymous: CRYPT_KEY_SIGN_MESSAGE_PARA_0,
    pub dwKeySpec: CERT_KEY_SPEC,
    pub HashAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub pvHashAuxInfo: *mut core::ffi::c_void,
    pub PubKeyAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
}
impl Default for CRYPT_KEY_SIGN_MESSAGE_PARA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CRYPT_KEY_SIGN_MESSAGE_PARA_0 {
    pub hCryptProv: usize,
    pub hNCryptKey: NCRYPT_KEY_HANDLE,
}
impl Default for CRYPT_KEY_SIGN_MESSAGE_PARA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CRYPT_KEY_VERIFY_MESSAGE_PARA {
    pub cbSize: u32,
    pub dwMsgEncodingType: u32,
    pub hCryptProv: HCRYPTPROV_LEGACY,
}
pub const CRYPT_KM: BCRYPT_QUERY_PROVIDER_MODE = BCRYPT_QUERY_PROVIDER_MODE(2u32);
pub const CRYPT_LAST_ALG_OID_GROUP_ID: u32 = 4u32;
pub const CRYPT_LAST_OID_GROUP_ID: u32 = 10u32;
pub const CRYPT_LDAP_AREC_EXCLUSIVE_RETRIEVAL: u32 = 262144u32;
pub const CRYPT_LDAP_INSERT_ENTRY_ATTRIBUTE: u32 = 32768u32;
pub const CRYPT_LDAP_SCOPE_BASE_ONLY_RETRIEVAL: u32 = 8192u32;
pub const CRYPT_LDAP_SIGN_RETRIEVAL: u32 = 65536u32;
pub const CRYPT_LITTLE_ENDIAN: u32 = 1u32;
pub const CRYPT_LOCAL: BCRYPT_TABLE = BCRYPT_TABLE(1u32);
pub const CRYPT_LOCALIZED_NAME_ENCODING_TYPE: u32 = 0u32;
pub const CRYPT_LOCALIZED_NAME_OID: windows_core::PCSTR = windows_core::s!("LocalizedNames");
pub const CRYPT_MAC: u32 = 32u32;
pub const CRYPT_MACHINE_DEFAULT: u32 = 1u32;
pub const CRYPT_MACHINE_KEYSET: CRYPT_KEY_FLAGS = CRYPT_KEY_FLAGS(32u32);
pub const CRYPT_MAC_RESOURCE: windows_core::PCWSTR = windows_core::w!("#667");
pub const CRYPT_MAC_RESOURCE_NUMBER: u32 = 667u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CRYPT_MASK_GEN_ALGORITHM {
    pub pszObjId: windows_core::PSTR,
    pub HashAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
}
pub const CRYPT_MATCH_ANY_ENCODING_TYPE: u32 = 4294967295u32;
pub const CRYPT_MAX_PROVIDER_ID: u32 = 999u32;
pub const CRYPT_MESSAGE_BARE_CONTENT_OUT_FLAG: u32 = 1u32;
pub const CRYPT_MESSAGE_ENCAPSULATED_CONTENT_OUT_FLAG: u32 = 2u32;
pub const CRYPT_MESSAGE_KEYID_RECIPIENT_FLAG: u32 = 4u32;
pub const CRYPT_MESSAGE_KEYID_SIGNER_FLAG: u32 = 4u32;
pub const CRYPT_MESSAGE_SILENT_KEYSET_FLAG: u32 = 64u32;
pub const CRYPT_MIN_DEPENDENCIES: CRYPT_IMAGE_REF_FLAGS = CRYPT_IMAGE_REF_FLAGS(1u32);
pub const CRYPT_MM: BCRYPT_QUERY_PROVIDER_MODE = BCRYPT_QUERY_PROVIDER_MODE(3u32);
pub const CRYPT_MODE_CBC: u32 = 1u32;
pub const CRYPT_MODE_CBCI: u32 = 6u32;
pub const CRYPT_MODE_CBCOFM: u32 = 9u32;
pub const CRYPT_MODE_CBCOFMI: u32 = 10u32;
pub const CRYPT_MODE_CFB: u32 = 4u32;
pub const CRYPT_MODE_CFBP: u32 = 7u32;
pub const CRYPT_MODE_CTS: u32 = 5u32;
pub const CRYPT_MODE_ECB: u32 = 2u32;
pub const CRYPT_MODE_OFB: u32 = 3u32;
pub const CRYPT_MODE_OFBP: u32 = 8u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_MSG_TYPE(pub u32);
pub const CRYPT_NDR_ENCODING: u32 = 2u32;
pub const CRYPT_NEWKEYSET: u32 = 8u32;
pub const CRYPT_NEXT: u32 = 2u32;
pub const CRYPT_NOHASHOID: u32 = 1u32;
pub const CRYPT_NOT_MODIFIED_RETRIEVAL: u32 = 4194304u32;
pub const CRYPT_NO_AUTH_RETRIEVAL: u32 = 131072u32;
pub const CRYPT_NO_OCSP_FAILOVER_TO_CRL_RETRIEVAL: u32 = 33554432u32;
pub const CRYPT_NO_SALT: CRYPT_KEY_FLAGS = CRYPT_KEY_FLAGS(16u32);
pub const CRYPT_OAEP: CRYPT_KEY_FLAGS = CRYPT_KEY_FLAGS(64u32);
pub const CRYPT_OBJECT_LOCATOR_FIRST_RESERVED_USER_NAME_TYPE: u32 = 33u32;
pub const CRYPT_OBJECT_LOCATOR_LAST_RESERVED_NAME_TYPE: u32 = 32u32;
pub const CRYPT_OBJECT_LOCATOR_LAST_RESERVED_USER_NAME_TYPE: u32 = 65535u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct CRYPT_OBJECT_LOCATOR_PROVIDER_TABLE {
    pub cbSize: u32,
    pub pfnGet: PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_GET,
    pub pfnRelease: PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_RELEASE,
    pub pfnFreePassword: PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_FREE_PASSWORD,
    pub pfnFree: PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_FREE,
    pub pfnFreeIdentifier: PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_FREE_IDENTIFIER,
}
pub const CRYPT_OBJECT_LOCATOR_RELEASE_DLL_UNLOAD: CRYPT_OBJECT_LOCATOR_RELEASE_REASON = CRYPT_OBJECT_LOCATOR_RELEASE_REASON(4u32);
pub const CRYPT_OBJECT_LOCATOR_RELEASE_PROCESS_EXIT: CRYPT_OBJECT_LOCATOR_RELEASE_REASON = CRYPT_OBJECT_LOCATOR_RELEASE_REASON(3u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_OBJECT_LOCATOR_RELEASE_REASON(pub u32);
pub const CRYPT_OBJECT_LOCATOR_RELEASE_SERVICE_STOP: CRYPT_OBJECT_LOCATOR_RELEASE_REASON = CRYPT_OBJECT_LOCATOR_RELEASE_REASON(2u32);
pub const CRYPT_OBJECT_LOCATOR_RELEASE_SYSTEM_SHUTDOWN: CRYPT_OBJECT_LOCATOR_RELEASE_REASON = CRYPT_OBJECT_LOCATOR_RELEASE_REASON(1u32);
pub const CRYPT_OBJECT_LOCATOR_SPN_NAME_TYPE: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CRYPT_OBJID_TABLE {
    pub dwAlgId: u32,
    pub pszObjId: windows_core::PCSTR,
}
pub const CRYPT_OCSP_ONLY_RETRIEVAL: u32 = 16777216u32;
pub const CRYPT_OFFLINE_CHECK_RETRIEVAL: u32 = 16384u32;
pub const CRYPT_OID_CREATE_COM_OBJECT_FUNC: windows_core::PCSTR = windows_core::s!("CryptDllCreateCOMObject");
pub const CRYPT_OID_DECODE_OBJECT_EX_FUNC: windows_core::PCSTR = windows_core::s!("CryptDllDecodeObjectEx");
pub const CRYPT_OID_DECODE_OBJECT_FUNC: windows_core::PCSTR = windows_core::s!("CryptDllDecodeObject");
pub const CRYPT_OID_DISABLE_SEARCH_DS_FLAG: u32 = 2147483648u32;
pub const CRYPT_OID_ENCODE_OBJECT_EX_FUNC: windows_core::PCSTR = windows_core::s!("CryptDllEncodeObjectEx");
pub const CRYPT_OID_ENCODE_OBJECT_FUNC: windows_core::PCSTR = windows_core::s!("CryptDllEncodeObject");
pub const CRYPT_OID_ENUM_PHYSICAL_STORE_FUNC: windows_core::PCSTR = windows_core::s!("CertDllEnumPhysicalStore");
pub const CRYPT_OID_ENUM_SYSTEM_STORE_FUNC: windows_core::PCSTR = windows_core::s!("CertDllEnumSystemStore");
pub const CRYPT_OID_EXPORT_PRIVATE_KEY_INFO_FUNC: windows_core::PCSTR = windows_core::s!("CryptDllExportPrivateKeyInfoEx");
pub const CRYPT_OID_EXPORT_PUBLIC_KEY_INFO_EX2_FUNC: windows_core::PCSTR = windows_core::s!("CryptDllExportPublicKeyInfoEx2");
pub const CRYPT_OID_EXPORT_PUBLIC_KEY_INFO_FROM_BCRYPT_HANDLE_FUNC: windows_core::PCSTR = windows_core::s!("CryptDllExportPublicKeyInfoFromBCryptKeyHandle");
pub const CRYPT_OID_EXPORT_PUBLIC_KEY_INFO_FUNC: windows_core::PCSTR = windows_core::s!("CryptDllExportPublicKeyInfoEx");
pub const CRYPT_OID_EXTRACT_ENCODED_SIGNATURE_PARAMETERS_FUNC: windows_core::PCSTR = windows_core::s!("CryptDllExtractEncodedSignatureParameters");
pub const CRYPT_OID_FIND_LOCALIZED_NAME_FUNC: windows_core::PCSTR = windows_core::s!("CryptDllFindLocalizedName");
pub const CRYPT_OID_FIND_OID_INFO_FUNC: windows_core::PCSTR = windows_core::s!("CryptDllFindOIDInfo");
pub const CRYPT_OID_FORMAT_OBJECT_FUNC: windows_core::PCSTR = windows_core::s!("CryptDllFormatObject");
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPT_OID_FUNC_ENTRY {
    pub pszOID: windows_core::PCSTR,
    pub pvFuncAddr: *mut core::ffi::c_void,
}
impl Default for CRYPT_OID_FUNC_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_OID_IMPORT_PRIVATE_KEY_INFO_FUNC: windows_core::PCSTR = windows_core::s!("CryptDllImportPrivateKeyInfoEx");
pub const CRYPT_OID_IMPORT_PUBLIC_KEY_INFO_EX2_FUNC: windows_core::PCSTR = windows_core::s!("CryptDllImportPublicKeyInfoEx2");
pub const CRYPT_OID_IMPORT_PUBLIC_KEY_INFO_FUNC: windows_core::PCSTR = windows_core::s!("CryptDllImportPublicKeyInfoEx");
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPT_OID_INFO {
    pub cbSize: u32,
    pub pszOID: windows_core::PCSTR,
    pub pwszName: windows_core::PCWSTR,
    pub dwGroupId: u32,
    pub Anonymous: CRYPT_OID_INFO_0,
    pub ExtraInfo: CRYPT_INTEGER_BLOB,
}
impl Default for CRYPT_OID_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CRYPT_OID_INFO_0 {
    pub dwValue: u32,
    pub Algid: ALG_ID,
    pub dwLength: u32,
}
impl Default for CRYPT_OID_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_OID_INFO_ALGID_KEY: u32 = 3u32;
pub const CRYPT_OID_INFO_CNG_ALGID_KEY: u32 = 5u32;
pub const CRYPT_OID_INFO_CNG_SIGN_KEY: u32 = 6u32;
pub const CRYPT_OID_INFO_ECC_PARAMETERS_ALGORITHM: windows_core::PCWSTR = windows_core::w!("CryptOIDInfoECCParameters");
pub const CRYPT_OID_INFO_ECC_WRAP_PARAMETERS_ALGORITHM: windows_core::PCWSTR = windows_core::w!("CryptOIDInfoECCWrapParameters");
pub const CRYPT_OID_INFO_HASH_PARAMETERS_ALGORITHM: windows_core::PCWSTR = windows_core::w!("CryptOIDInfoHashParameters");
pub const CRYPT_OID_INFO_MGF1_PARAMETERS_ALGORITHM: windows_core::PCWSTR = windows_core::w!("CryptOIDInfoMgf1Parameters");
pub const CRYPT_OID_INFO_NAME_KEY: u32 = 2u32;
pub const CRYPT_OID_INFO_NO_PARAMETERS_ALGORITHM: windows_core::PCWSTR = windows_core::w!("CryptOIDInfoNoParameters");
pub const CRYPT_OID_INFO_NO_SIGN_ALGORITHM: windows_core::PCWSTR = windows_core::w!("CryptOIDInfoNoSign");
pub const CRYPT_OID_INFO_OAEP_PARAMETERS_ALGORITHM: windows_core::PCWSTR = windows_core::w!("CryptOIDInfoOAEPParameters");
pub const CRYPT_OID_INFO_OID_GROUP_BIT_LEN_MASK: u32 = 268369920u32;
pub const CRYPT_OID_INFO_OID_GROUP_BIT_LEN_SHIFT: u32 = 16u32;
pub const CRYPT_OID_INFO_OID_KEY: u32 = 1u32;
pub const CRYPT_OID_INFO_OID_KEY_FLAGS_MASK: u32 = 4294901760u32;
pub const CRYPT_OID_INFO_PUBKEY_ENCRYPT_KEY_FLAG: CRYPT_IMPORT_PUBLIC_KEY_FLAGS = CRYPT_IMPORT_PUBLIC_KEY_FLAGS(1073741824u32);
pub const CRYPT_OID_INFO_PUBKEY_SIGN_KEY_FLAG: CRYPT_IMPORT_PUBLIC_KEY_FLAGS = CRYPT_IMPORT_PUBLIC_KEY_FLAGS(2147483648u32);
pub const CRYPT_OID_INFO_SIGN_KEY: u32 = 4u32;
pub const CRYPT_OID_INHIBIT_SIGNATURE_FORMAT_FLAG: u32 = 1u32;
pub const CRYPT_OID_NO_NULL_ALGORITHM_PARA_FLAG: u32 = 4u32;
pub const CRYPT_OID_OPEN_STORE_PROV_FUNC: windows_core::PCSTR = windows_core::s!("CertDllOpenStoreProv");
pub const CRYPT_OID_OPEN_SYSTEM_STORE_PROV_FUNC: windows_core::PCSTR = windows_core::s!("CertDllOpenSystemStoreProv");
pub const CRYPT_OID_PREFER_CNG_ALGID_FLAG: u32 = 1073741824u32;
pub const CRYPT_OID_PUBKEY_ENCRYPT_ONLY_FLAG: u32 = 1073741824u32;
pub const CRYPT_OID_PUBKEY_SIGN_ONLY_FLAG: u32 = 2147483648u32;
pub const CRYPT_OID_REGISTER_PHYSICAL_STORE_FUNC: windows_core::PCSTR = windows_core::s!("CertDllRegisterPhysicalStore");
pub const CRYPT_OID_REGISTER_SYSTEM_STORE_FUNC: windows_core::PCSTR = windows_core::s!("CertDllRegisterSystemStore");
pub const CRYPT_OID_REGPATH: windows_core::PCSTR = windows_core::s!("Software\\Microsoft\\Cryptography\\OID");
pub const CRYPT_OID_REG_DLL_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("Dll");
pub const CRYPT_OID_REG_ENCODING_TYPE_PREFIX: windows_core::PCSTR = windows_core::s!("EncodingType ");
pub const CRYPT_OID_REG_FLAGS_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("CryptFlags");
pub const CRYPT_OID_REG_FUNC_NAME_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("FuncName");
pub const CRYPT_OID_REG_FUNC_NAME_VALUE_NAME_A: windows_core::PCSTR = windows_core::s!("FuncName");
pub const CRYPT_OID_SIGN_AND_ENCODE_HASH_FUNC: windows_core::PCSTR = windows_core::s!("CryptDllSignAndEncodeHash");
pub const CRYPT_OID_SYSTEM_STORE_LOCATION_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("SystemStoreLocation");
pub const CRYPT_OID_UNREGISTER_PHYSICAL_STORE_FUNC: windows_core::PCSTR = windows_core::s!("CertDllUnregisterPhysicalStore");
pub const CRYPT_OID_UNREGISTER_SYSTEM_STORE_FUNC: windows_core::PCSTR = windows_core::s!("CertDllUnregisterSystemStore");
pub const CRYPT_OID_USE_CURVE_NAME_FOR_ENCODE_FLAG: u32 = 536870912u32;
pub const CRYPT_OID_USE_CURVE_PARAMETERS_FOR_ENCODE_FLAG: u32 = 268435456u32;
pub const CRYPT_OID_USE_PUBKEY_PARA_FOR_PKCS7_FLAG: u32 = 2u32;
pub const CRYPT_OID_VERIFY_CERTIFICATE_CHAIN_POLICY_FUNC: windows_core::PCSTR = windows_core::s!("CertDllVerifyCertificateChainPolicy");
pub const CRYPT_OID_VERIFY_CTL_USAGE_FUNC: windows_core::PCSTR = windows_core::s!("CertDllVerifyCTLUsage");
pub const CRYPT_OID_VERIFY_ENCODED_SIGNATURE_FUNC: windows_core::PCSTR = windows_core::s!("CryptDllVerifyEncodedSignature");
pub const CRYPT_OID_VERIFY_REVOCATION_FUNC: windows_core::PCSTR = windows_core::s!("CertDllVerifyRevocation");
pub const CRYPT_ONLINE: CRYPT_KEY_FLAGS = CRYPT_KEY_FLAGS(128u32);
pub const CRYPT_OVERRIDE: CRYPT_CONTEXT_CONFIG_FLAGS = CRYPT_CONTEXT_CONFIG_FLAGS(65536u32);
pub const CRYPT_OVERWRITE: u32 = 1u32;
pub const CRYPT_OWF_REPL_LM_HASH: u32 = 1u32;
pub const CRYPT_PARAM_ASYNC_RETRIEVAL_COMPLETION: windows_core::PCSTR = windows_core::PCSTR(1i32 as _);
pub const CRYPT_PARAM_CANCEL_ASYNC_RETRIEVAL: windows_core::PCSTR = windows_core::PCSTR(2i32 as _);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CRYPT_PASSWORD_CREDENTIALSA {
    pub cbSize: u32,
    pub pszUsername: windows_core::PSTR,
    pub pszPassword: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CRYPT_PASSWORD_CREDENTIALSW {
    pub cbSize: u32,
    pub pszUsername: windows_core::PWSTR,
    pub pszPassword: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CRYPT_PKCS12_PBE_PARAMS {
    pub iIterations: i32,
    pub cbSalt: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CRYPT_PKCS8_EXPORT_PARAMS {
    pub hCryptProv: usize,
    pub dwKeySpec: u32,
    pub pszPrivateKeyObjId: windows_core::PSTR,
    pub pEncryptPrivateKeyFunc: PCRYPT_ENCRYPT_PRIVATE_KEY_FUNC,
    pub pVoidEncryptFunc: *mut core::ffi::c_void,
}
impl Default for CRYPT_PKCS8_EXPORT_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CRYPT_PKCS8_IMPORT_PARAMS {
    pub PrivateKey: CRYPT_INTEGER_BLOB,
    pub pResolvehCryptProvFunc: PCRYPT_RESOLVE_HCRYPTPROV_FUNC,
    pub pVoidResolveFunc: *mut core::ffi::c_void,
    pub pDecryptPrivateKeyFunc: PCRYPT_DECRYPT_PRIVATE_KEY_FUNC,
    pub pVoidDecryptFunc: *mut core::ffi::c_void,
}
impl Default for CRYPT_PKCS8_IMPORT_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_POLICY_OID_GROUP_ID: u32 = 8u32;
pub const CRYPT_PREGEN: CRYPT_KEY_FLAGS = CRYPT_KEY_FLAGS(64u32);
pub const CRYPT_PRIORITY_BOTTOM: u32 = 4294967295u32;
pub const CRYPT_PRIORITY_TOP: u32 = 0u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPT_PRIVATE_KEY_INFO {
    pub Version: u32,
    pub Algorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub PrivateKey: CRYPT_INTEGER_BLOB,
    pub pAttributes: *mut CRYPT_ATTRIBUTES,
}
impl Default for CRYPT_PRIVATE_KEY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_PROCESS_ISOLATE: CRYPT_IMAGE_REF_FLAGS = CRYPT_IMAGE_REF_FLAGS(65536u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPT_PROPERTY_REF {
    pub pszProperty: windows_core::PWSTR,
    pub cbValue: u32,
    pub pbValue: *mut u8,
}
impl Default for CRYPT_PROPERTY_REF {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPT_PROVIDERS {
    pub cProviders: u32,
    pub rgpszProviders: *mut windows_core::PWSTR,
}
impl Default for CRYPT_PROVIDERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_PROVIDER_IOCTL__GET_SCHANNEL_INTERFACE: u32 = 4145180u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPT_PROVIDER_REF {
    pub dwInterface: u32,
    pub pszFunction: windows_core::PWSTR,
    pub pszProvider: windows_core::PWSTR,
    pub cProperties: u32,
    pub rgpProperties: *mut *mut CRYPT_PROPERTY_REF,
    pub pUM: *mut CRYPT_IMAGE_REF,
    pub pKM: *mut CRYPT_IMAGE_REF,
}
impl Default for CRYPT_PROVIDER_REF {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPT_PROVIDER_REFS {
    pub cProviders: u32,
    pub rgpProviders: *mut *mut CRYPT_PROVIDER_REF,
}
impl Default for CRYPT_PROVIDER_REFS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPT_PROVIDER_REG {
    pub cAliases: u32,
    pub rgpszAliases: *mut windows_core::PWSTR,
    pub pUM: *mut CRYPT_IMAGE_REG,
    pub pKM: *mut CRYPT_IMAGE_REG,
}
impl Default for CRYPT_PROVIDER_REG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_PROVSTRUC_VERSION_V3: u32 = 3u32;
pub const CRYPT_PROXY_CACHE_RETRIEVAL: u32 = 2097152u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CRYPT_PSOURCE_ALGORITHM {
    pub pszObjId: windows_core::PSTR,
    pub EncodingParameters: CRYPT_INTEGER_BLOB,
}
pub const CRYPT_PSTORE: u32 = 2u32;
pub const CRYPT_PUBKEY_ALG_OID_GROUP_ID: u32 = 3u32;
pub const CRYPT_RANDOM_QUERY_STRING_RETRIEVAL: u32 = 67108864u32;
pub const CRYPT_RC2_128BIT_VERSION: u32 = 58u32;
pub const CRYPT_RC2_40BIT_VERSION: u32 = 160u32;
pub const CRYPT_RC2_56BIT_VERSION: u32 = 52u32;
pub const CRYPT_RC2_64BIT_VERSION: u32 = 120u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPT_RC2_CBC_PARAMETERS {
    pub dwVersion: u32,
    pub fIV: windows_core::BOOL,
    pub rgbIV: [u8; 8],
}
impl Default for CRYPT_RC2_CBC_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPT_RC4_KEY_STATE {
    pub Key: [u8; 16],
    pub SBox: [u8; 256],
    pub i: u8,
    pub j: u8,
}
impl Default for CRYPT_RC4_KEY_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_RDN_ATTR_OID_GROUP_ID: u32 = 5u32;
pub const CRYPT_READ: u32 = 8u32;
pub const CRYPT_RECIPIENT: CRYPT_KEY_FLAGS = CRYPT_KEY_FLAGS(16u32);
pub const CRYPT_REGISTER_FIRST_INDEX: u32 = 0u32;
pub const CRYPT_REGISTER_LAST_INDEX: u32 = 4294967295u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPT_RETRIEVE_AUX_INFO {
    pub cbSize: u32,
    pub pLastSyncTime: *mut super::super::Foundation::FILETIME,
    pub dwMaxUrlRetrievalByteCount: u32,
    pub pPreFetchInfo: *mut CRYPTNET_URL_CACHE_PRE_FETCH_INFO,
    pub pFlushInfo: *mut CRYPTNET_URL_CACHE_FLUSH_INFO,
    pub ppResponseInfo: *mut *mut CRYPTNET_URL_CACHE_RESPONSE_INFO,
    pub pwszCacheFileNamePrefix: windows_core::PWSTR,
    pub pftCacheResync: *mut super::super::Foundation::FILETIME,
    pub fProxyCacheRetrieval: windows_core::BOOL,
    pub dwHttpStatusCode: u32,
    pub ppwszErrorResponseHeaders: *mut windows_core::PWSTR,
    pub ppErrorContentBlob: *mut *mut CRYPT_INTEGER_BLOB,
}
impl Default for CRYPT_RETRIEVE_AUX_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_RETRIEVE_MAX_ERROR_CONTENT_LENGTH: u32 = 4096u32;
pub const CRYPT_RETRIEVE_MULTIPLE_OBJECTS: u32 = 1u32;
pub type CRYPT_RETURN_HWND = Option<unsafe extern "system" fn(phwnd: *mut super::super::Foundation::HWND)>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CRYPT_RSAES_OAEP_PARAMETERS {
    pub HashAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub MaskGenAlgorithm: CRYPT_MASK_GEN_ALGORITHM,
    pub PSourceAlgorithm: CRYPT_PSOURCE_ALGORITHM,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CRYPT_RSA_SSA_PSS_PARAMETERS {
    pub HashAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub MaskGenAlgorithm: CRYPT_MASK_GEN_ALGORITHM,
    pub dwSaltLength: u32,
    pub dwTrailerField: u32,
}
pub const CRYPT_SECRETDIGEST: u32 = 1u32;
pub const CRYPT_SEC_DESCR: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPT_SEQUENCE_OF_ANY {
    pub cValue: u32,
    pub rgValue: *mut CRYPT_INTEGER_BLOB,
}
impl Default for CRYPT_SEQUENCE_OF_ANY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_SERVER: u32 = 1024u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_SET_HASH_PARAM(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_SET_PROV_PARAM_ID(pub u32);
pub const CRYPT_SF: CRYPT_KEY_FLAGS = CRYPT_KEY_FLAGS(256u32);
pub const CRYPT_SGC: u32 = 1u32;
pub const CRYPT_SGCKEY: CRYPT_KEY_FLAGS = CRYPT_KEY_FLAGS(8192u32);
pub const CRYPT_SGC_ENUM: u32 = 4u32;
pub const CRYPT_SIGN_ALG_OID_GROUP_ID: u32 = 4u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPT_SIGN_MESSAGE_PARA {
    pub cbSize: u32,
    pub dwMsgEncodingType: u32,
    pub pSigningCert: *const CERT_CONTEXT,
    pub HashAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub pvHashAuxInfo: *mut core::ffi::c_void,
    pub cMsgCert: u32,
    pub rgpMsgCert: *mut *mut CERT_CONTEXT,
    pub cMsgCrl: u32,
    pub rgpMsgCrl: *mut *mut CRL_CONTEXT,
    pub cAuthAttr: u32,
    pub rgAuthAttr: *mut CRYPT_ATTRIBUTE,
    pub cUnauthAttr: u32,
    pub rgUnauthAttr: *mut CRYPT_ATTRIBUTE,
    pub dwFlags: u32,
    pub dwInnerContentType: u32,
}
impl Default for CRYPT_SIGN_MESSAGE_PARA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_SIG_RESOURCE: windows_core::PCWSTR = windows_core::w!("#666");
pub const CRYPT_SIG_RESOURCE_NUMBER: u32 = 666u32;
pub const CRYPT_SIG_RESOURCE_VERSION: u32 = 256u32;
pub const CRYPT_SILENT: u32 = 64u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPT_SMART_CARD_ROOT_INFO {
    pub rgbCardID: [u8; 16],
    pub luid: ROOT_INFO_LUID,
}
impl Default for CRYPT_SMART_CARD_ROOT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPT_SMIME_CAPABILITIES {
    pub cCapability: u32,
    pub rgCapability: *mut CRYPT_SMIME_CAPABILITY,
}
impl Default for CRYPT_SMIME_CAPABILITIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CRYPT_SMIME_CAPABILITY {
    pub pszObjId: windows_core::PSTR,
    pub Parameters: CRYPT_INTEGER_BLOB,
}
pub const CRYPT_SORTED_CTL_ENCODE_HASHED_SUBJECT_IDENTIFIER_FLAG: u32 = 65536u32;
pub const CRYPT_SSL2_FALLBACK: CRYPT_KEY_FLAGS = CRYPT_KEY_FLAGS(2u32);
pub const CRYPT_STICKY_CACHE_RETRIEVAL: u32 = 4096u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_STRING(pub u32);
pub const CRYPT_STRING_ANY: CRYPT_STRING = CRYPT_STRING(7u32);
pub const CRYPT_STRING_BASE64: CRYPT_STRING = CRYPT_STRING(1u32);
pub const CRYPT_STRING_BASE64HEADER: CRYPT_STRING = CRYPT_STRING(0u32);
pub const CRYPT_STRING_BASE64REQUESTHEADER: CRYPT_STRING = CRYPT_STRING(3u32);
pub const CRYPT_STRING_BASE64URI: u32 = 13u32;
pub const CRYPT_STRING_BASE64X509CRLHEADER: CRYPT_STRING = CRYPT_STRING(9u32);
pub const CRYPT_STRING_BASE64_ANY: CRYPT_STRING = CRYPT_STRING(6u32);
pub const CRYPT_STRING_BINARY: CRYPT_STRING = CRYPT_STRING(2u32);
pub const CRYPT_STRING_ENCODEMASK: u32 = 255u32;
pub const CRYPT_STRING_HASHDATA: u32 = 268435456u32;
pub const CRYPT_STRING_HEX: CRYPT_STRING = CRYPT_STRING(4u32);
pub const CRYPT_STRING_HEXADDR: CRYPT_STRING = CRYPT_STRING(10u32);
pub const CRYPT_STRING_HEXASCII: CRYPT_STRING = CRYPT_STRING(5u32);
pub const CRYPT_STRING_HEXASCIIADDR: CRYPT_STRING = CRYPT_STRING(11u32);
pub const CRYPT_STRING_HEXRAW: CRYPT_STRING = CRYPT_STRING(12u32);
pub const CRYPT_STRING_HEX_ANY: CRYPT_STRING = CRYPT_STRING(8u32);
pub const CRYPT_STRING_NOCR: u32 = 2147483648u32;
pub const CRYPT_STRING_NOCRLF: u32 = 1073741824u32;
pub const CRYPT_STRING_PERCENTESCAPE: u32 = 134217728u32;
pub const CRYPT_STRING_RESERVED100: u32 = 256u32;
pub const CRYPT_STRING_RESERVED200: u32 = 512u32;
pub const CRYPT_STRING_STRICT: CRYPT_STRING = CRYPT_STRING(536870912u32);
pub const CRYPT_SUCCEED: u32 = 1u32;
pub const CRYPT_TEMPLATE_OID_GROUP_ID: u32 = 9u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CRYPT_TIMESTAMP_ACCURACY {
    pub dwSeconds: u32,
    pub dwMillis: u32,
    pub dwMicros: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPT_TIMESTAMP_CONTEXT {
    pub cbEncoded: u32,
    pub pbEncoded: *mut u8,
    pub pTimeStamp: *mut CRYPT_TIMESTAMP_INFO,
}
impl Default for CRYPT_TIMESTAMP_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPT_TIMESTAMP_INFO {
    pub dwVersion: u32,
    pub pszTSAPolicyId: windows_core::PSTR,
    pub HashAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub HashedMessage: CRYPT_INTEGER_BLOB,
    pub SerialNumber: CRYPT_INTEGER_BLOB,
    pub ftTime: super::super::Foundation::FILETIME,
    pub pvAccuracy: *mut CRYPT_TIMESTAMP_ACCURACY,
    pub fOrdering: windows_core::BOOL,
    pub Nonce: CRYPT_INTEGER_BLOB,
    pub Tsa: CRYPT_INTEGER_BLOB,
    pub cExtension: u32,
    pub rgExtension: *mut CERT_EXTENSION,
}
impl Default for CRYPT_TIMESTAMP_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPT_TIMESTAMP_PARA {
    pub pszTSAPolicyId: windows_core::PCSTR,
    pub fRequestCerts: windows_core::BOOL,
    pub Nonce: CRYPT_INTEGER_BLOB,
    pub cExtension: u32,
    pub rgExtension: *mut CERT_EXTENSION,
}
impl Default for CRYPT_TIMESTAMP_PARA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPT_TIMESTAMP_REQUEST {
    pub dwVersion: CRYPT_TIMESTAMP_VERSION,
    pub HashAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub HashedMessage: CRYPT_INTEGER_BLOB,
    pub pszTSAPolicyId: windows_core::PSTR,
    pub Nonce: CRYPT_INTEGER_BLOB,
    pub fCertReq: windows_core::BOOL,
    pub cExtension: u32,
    pub rgExtension: *mut CERT_EXTENSION,
}
impl Default for CRYPT_TIMESTAMP_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPT_TIMESTAMP_RESPONSE {
    pub dwStatus: CRYPT_TIMESTAMP_RESPONSE_STATUS,
    pub cFreeText: u32,
    pub rgFreeText: *mut windows_core::PWSTR,
    pub FailureInfo: CRYPT_BIT_BLOB,
    pub ContentInfo: CRYPT_INTEGER_BLOB,
}
impl Default for CRYPT_TIMESTAMP_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_TIMESTAMP_RESPONSE_STATUS(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_TIMESTAMP_VERSION(pub u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPT_TIME_STAMP_REQUEST_INFO {
    pub pszTimeStampAlgorithm: windows_core::PSTR,
    pub pszContentType: windows_core::PSTR,
    pub Content: CRYPT_INTEGER_BLOB,
    pub cAttribute: u32,
    pub rgAttribute: *mut CRYPT_ATTRIBUTE,
}
impl Default for CRYPT_TIME_STAMP_REQUEST_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_TYPE2_FORMAT: u32 = 2u32;
pub const CRYPT_UI_PROMPT: u32 = 4u32;
pub const CRYPT_UM: BCRYPT_QUERY_PROVIDER_MODE = BCRYPT_QUERY_PROVIDER_MODE(1u32);
pub const CRYPT_UNICODE_NAME_DECODE_DISABLE_IE4_UTF8_FLAG: u32 = 16777216u32;
pub const CRYPT_UNICODE_NAME_ENCODE_DISABLE_CHECK_TYPE_FLAG: CRYPT_ENCODE_OBJECT_FLAGS = CRYPT_ENCODE_OBJECT_FLAGS(1073741824u32);
pub const CRYPT_UNICODE_NAME_ENCODE_ENABLE_T61_UNICODE_FLAG: CRYPT_ENCODE_OBJECT_FLAGS = CRYPT_ENCODE_OBJECT_FLAGS(2147483648u32);
pub const CRYPT_UNICODE_NAME_ENCODE_ENABLE_UTF8_UNICODE_FLAG: CRYPT_ENCODE_OBJECT_FLAGS = CRYPT_ENCODE_OBJECT_FLAGS(536870912u32);
pub const CRYPT_UNICODE_NAME_ENCODE_FORCE_UTF8_UNICODE_FLAG: u32 = 268435456u32;
pub const CRYPT_UPDATE_KEY: u32 = 8u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPT_URL_ARRAY {
    pub cUrl: u32,
    pub rgwszUrl: *mut windows_core::PWSTR,
}
impl Default for CRYPT_URL_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPT_URL_INFO {
    pub cbSize: u32,
    pub dwSyncDeltaTime: u32,
    pub cGroup: u32,
    pub rgcGroupEntry: *mut u32,
}
impl Default for CRYPT_URL_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_USERDATA: u32 = 1u32;
pub const CRYPT_USER_DEFAULT: u32 = 2u32;
pub const CRYPT_USER_KEYSET: CRYPT_KEY_FLAGS = CRYPT_KEY_FLAGS(4096u32);
pub const CRYPT_USER_PROTECTED: CRYPT_KEY_FLAGS = CRYPT_KEY_FLAGS(2u32);
pub const CRYPT_USER_PROTECTED_STRONG: u32 = 1048576u32;
pub const CRYPT_VERIFYCONTEXT: u32 = 4026531840u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_VERIFY_CERT_FLAGS(pub u32);
pub const CRYPT_VERIFY_CERT_SIGN_CHECK_WEAK_HASH_FLAG: u32 = 8u32;
pub const CRYPT_VERIFY_CERT_SIGN_DISABLE_MD2_MD4_FLAG: CRYPT_VERIFY_CERT_FLAGS = CRYPT_VERIFY_CERT_FLAGS(1u32);
pub const CRYPT_VERIFY_CERT_SIGN_ISSUER_CERT: u32 = 2u32;
pub const CRYPT_VERIFY_CERT_SIGN_ISSUER_CHAIN: u32 = 3u32;
pub const CRYPT_VERIFY_CERT_SIGN_ISSUER_NULL: u32 = 4u32;
pub const CRYPT_VERIFY_CERT_SIGN_ISSUER_PUBKEY: u32 = 1u32;
pub const CRYPT_VERIFY_CERT_SIGN_RETURN_STRONG_PROPERTIES_FLAG: CRYPT_VERIFY_CERT_FLAGS = CRYPT_VERIFY_CERT_FLAGS(4u32);
pub const CRYPT_VERIFY_CERT_SIGN_SET_STRONG_PROPERTIES_FLAG: CRYPT_VERIFY_CERT_FLAGS = CRYPT_VERIFY_CERT_FLAGS(2u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CRYPT_VERIFY_CERT_SIGN_STRONG_PROPERTIES_INFO {
    pub CertSignHashCNGAlgPropData: CRYPT_INTEGER_BLOB,
    pub CertIssuerPubKeyBitLengthPropData: CRYPT_INTEGER_BLOB,
}
pub const CRYPT_VERIFY_CERT_SIGN_SUBJECT_BLOB: u32 = 1u32;
pub const CRYPT_VERIFY_CERT_SIGN_SUBJECT_CERT: u32 = 2u32;
pub const CRYPT_VERIFY_CERT_SIGN_SUBJECT_CRL: u32 = 3u32;
pub const CRYPT_VERIFY_CERT_SIGN_SUBJECT_OCSP_BASIC_SIGNED_RESPONSE: u32 = 4u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPT_VERIFY_CERT_SIGN_WEAK_HASH_INFO {
    pub cCNGHashAlgid: u32,
    pub rgpwszCNGHashAlgid: *const windows_core::PCWSTR,
    pub dwWeakIndex: u32,
}
impl Default for CRYPT_VERIFY_CERT_SIGN_WEAK_HASH_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_VERIFY_CONTEXT_SIGNATURE: u32 = 32u32;
pub const CRYPT_VERIFY_DATA_HASH: u32 = 64u32;
pub type CRYPT_VERIFY_IMAGE_A = Option<unsafe extern "system" fn(szimage: windows_core::PCSTR, pbsigdata: *const u8) -> windows_core::BOOL>;
pub type CRYPT_VERIFY_IMAGE_W = Option<unsafe extern "system" fn(szimage: windows_core::PCWSTR, pbsigdata: *const u8) -> windows_core::BOOL>;
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CRYPT_VERIFY_MESSAGE_PARA {
    pub cbSize: u32,
    pub dwMsgAndCertEncodingType: u32,
    pub hCryptProv: HCRYPTPROV_LEGACY,
    pub pfnGetSignerCertificate: PFN_CRYPT_GET_SIGNER_CERTIFICATE,
    pub pvGetArg: *mut core::ffi::c_void,
}
impl Default for CRYPT_VERIFY_MESSAGE_PARA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_VOLATILE: CRYPT_KEY_FLAGS = CRYPT_KEY_FLAGS(4096u32);
pub const CRYPT_WIRE_ONLY_RETRIEVAL: u32 = 4u32;
pub const CRYPT_WRITE: u32 = 16u32;
pub const CRYPT_X931_FORMAT: u32 = 4u32;
pub const CRYPT_X942_COUNTER_BYTE_LENGTH: u32 = 4u32;
pub const CRYPT_X942_KEY_LENGTH_BYTE_LENGTH: u32 = 4u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPT_X942_OTHER_INFO {
    pub pszContentEncryptionObjId: windows_core::PSTR,
    pub rgbCounter: [u8; 4],
    pub rgbKeyLength: [u8; 4],
    pub PubInfo: CRYPT_INTEGER_BLOB,
}
impl Default for CRYPT_X942_OTHER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CRYPT_XML_ALGORITHM {
    pub cbSize: u32,
    pub wszAlgorithm: windows_core::PCWSTR,
    pub Encoded: CRYPT_XML_BLOB,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPT_XML_ALGORITHM_INFO {
    pub cbSize: u32,
    pub wszAlgorithmURI: windows_core::PWSTR,
    pub wszName: windows_core::PWSTR,
    pub dwGroupId: CRYPT_XML_GROUP_ID,
    pub wszCNGAlgid: windows_core::PWSTR,
    pub wszCNGExtraAlgid: windows_core::PWSTR,
    pub dwSignFlags: u32,
    pub dwVerifyFlags: u32,
    pub pvPaddingInfo: *mut core::ffi::c_void,
    pub pvExtraInfo: *mut core::ffi::c_void,
}
impl Default for CRYPT_XML_ALGORITHM_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_XML_ALGORITHM_INFO_FIND_BY_CNG_ALGID: u32 = 3u32;
pub const CRYPT_XML_ALGORITHM_INFO_FIND_BY_CNG_SIGN_ALGID: u32 = 4u32;
pub const CRYPT_XML_ALGORITHM_INFO_FIND_BY_NAME: u32 = 2u32;
pub const CRYPT_XML_ALGORITHM_INFO_FIND_BY_URI: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPT_XML_BLOB {
    pub dwCharset: CRYPT_XML_CHARSET,
    pub cbData: u32,
    pub pbData: *mut u8,
}
impl Default for CRYPT_XML_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_XML_BLOB_MAX: u32 = 2147483640u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_XML_CHARSET(pub i32);
pub const CRYPT_XML_CHARSET_AUTO: CRYPT_XML_CHARSET = CRYPT_XML_CHARSET(0i32);
pub const CRYPT_XML_CHARSET_UTF16BE: CRYPT_XML_CHARSET = CRYPT_XML_CHARSET(3i32);
pub const CRYPT_XML_CHARSET_UTF16LE: CRYPT_XML_CHARSET = CRYPT_XML_CHARSET(2i32);
pub const CRYPT_XML_CHARSET_UTF8: CRYPT_XML_CHARSET = CRYPT_XML_CHARSET(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct CRYPT_XML_CRYPTOGRAPHIC_INTERFACE {
    pub cbSize: u32,
    pub fpCryptXmlEncodeAlgorithm: CryptXmlDllEncodeAlgorithm,
    pub fpCryptXmlCreateDigest: CryptXmlDllCreateDigest,
    pub fpCryptXmlDigestData: CryptXmlDllDigestData,
    pub fpCryptXmlFinalizeDigest: CryptXmlDllFinalizeDigest,
    pub fpCryptXmlCloseDigest: CryptXmlDllCloseDigest,
    pub fpCryptXmlSignData: CryptXmlDllSignData,
    pub fpCryptXmlVerifySignature: CryptXmlDllVerifySignature,
    pub fpCryptXmlGetAlgorithmInfo: CryptXmlDllGetAlgorithmInfo,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPT_XML_DATA_BLOB {
    pub cbData: u32,
    pub pbData: *mut u8,
}
impl Default for CRYPT_XML_DATA_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CRYPT_XML_DATA_PROVIDER {
    pub pvCallbackState: *mut core::ffi::c_void,
    pub cbBufferSize: u32,
    pub pfnRead: PFN_CRYPT_XML_DATA_PROVIDER_READ,
    pub pfnClose: PFN_CRYPT_XML_DATA_PROVIDER_CLOSE,
}
impl Default for CRYPT_XML_DATA_PROVIDER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_XML_DIGEST_REFERENCE_DATA_TRANSFORMED: u32 = 1u32;
pub const CRYPT_XML_DIGEST_VALUE_MAX: u32 = 128u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPT_XML_DOC_CTXT {
    pub cbSize: u32,
    pub hDocCtxt: *mut core::ffi::c_void,
    pub pTransformsConfig: *mut CRYPT_XML_TRANSFORM_CHAIN_CONFIG,
    pub cSignature: u32,
    pub rgpSignature: *mut *mut CRYPT_XML_SIGNATURE,
}
impl Default for CRYPT_XML_DOC_CTXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_XML_E_ALGORITHM: windows_core::HRESULT = windows_core::HRESULT(0x80092104_u32 as _);
pub const CRYPT_XML_E_BASE: windows_core::HRESULT = windows_core::HRESULT(0x80092100_u32 as _);
pub const CRYPT_XML_E_ENCODING: windows_core::HRESULT = windows_core::HRESULT(0x80092103_u32 as _);
pub const CRYPT_XML_E_HANDLE: windows_core::HRESULT = windows_core::HRESULT(0x80092106_u32 as _);
pub const CRYPT_XML_E_HASH_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x8009210B_u32 as _);
pub const CRYPT_XML_E_INVALID_DIGEST: windows_core::HRESULT = windows_core::HRESULT(0x80092109_u32 as _);
pub const CRYPT_XML_E_INVALID_KEYVALUE: windows_core::HRESULT = windows_core::HRESULT(0x8009210F_u32 as _);
pub const CRYPT_XML_E_INVALID_SIGNATURE: windows_core::HRESULT = windows_core::HRESULT(0x8009210A_u32 as _);
pub const CRYPT_XML_E_LARGE: windows_core::HRESULT = windows_core::HRESULT(0x80092101_u32 as _);
pub const CRYPT_XML_E_LAST: windows_core::HRESULT = windows_core::HRESULT(0x80092112_u32 as _);
pub const CRYPT_XML_E_NON_UNIQUE_ID: windows_core::HRESULT = windows_core::HRESULT(0x80092112_u32 as _);
pub const CRYPT_XML_E_OPERATION: windows_core::HRESULT = windows_core::HRESULT(0x80092107_u32 as _);
pub const CRYPT_XML_E_SIGNER: windows_core::HRESULT = windows_core::HRESULT(0x80092111_u32 as _);
pub const CRYPT_XML_E_SIGN_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x8009210C_u32 as _);
pub const CRYPT_XML_E_TOO_MANY_SIGNATURES: windows_core::HRESULT = windows_core::HRESULT(0x8009210E_u32 as _);
pub const CRYPT_XML_E_TOO_MANY_TRANSFORMS: windows_core::HRESULT = windows_core::HRESULT(0x80092102_u32 as _);
pub const CRYPT_XML_E_TRANSFORM: windows_core::HRESULT = windows_core::HRESULT(0x80092105_u32 as _);
pub const CRYPT_XML_E_UNEXPECTED_XML: windows_core::HRESULT = windows_core::HRESULT(0x80092110_u32 as _);
pub const CRYPT_XML_E_UNRESOLVED_REFERENCE: windows_core::HRESULT = windows_core::HRESULT(0x80092108_u32 as _);
pub const CRYPT_XML_E_VERIFY_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x8009210D_u32 as _);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_XML_FLAGS(pub u32);
pub const CRYPT_XML_FLAG_ADD_OBJECT_CREATE_COPY: u32 = 1u32;
pub const CRYPT_XML_FLAG_ALWAYS_RETURN_ENCODED_OBJECT: u32 = 1073741824u32;
pub const CRYPT_XML_FLAG_CREATE_REFERENCE_AS_OBJECT: u32 = 1u32;
pub const CRYPT_XML_FLAG_DISABLE_EXTENSIONS: CRYPT_XML_FLAGS = CRYPT_XML_FLAGS(268435456u32);
pub const CRYPT_XML_FLAG_ECDSA_DSIG11: u32 = 67108864u32;
pub const CRYPT_XML_FLAG_ENFORCE_ID_NAME_FORMAT: u32 = 134217728u32;
pub const CRYPT_XML_FLAG_ENFORCE_ID_NCNAME_FORMAT: u32 = 536870912u32;
pub const CRYPT_XML_FLAG_NO_SERIALIZE: CRYPT_XML_FLAGS = CRYPT_XML_FLAGS(2147483648u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_XML_GROUP_ID(pub u32);
pub const CRYPT_XML_GROUP_ID_HASH: CRYPT_XML_GROUP_ID = CRYPT_XML_GROUP_ID(1u32);
pub const CRYPT_XML_GROUP_ID_SIGN: CRYPT_XML_GROUP_ID = CRYPT_XML_GROUP_ID(2u32);
pub const CRYPT_XML_ID_MAX: u32 = 256u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CRYPT_XML_ISSUER_SERIAL {
    pub wszIssuer: windows_core::PCWSTR,
    pub wszSerial: windows_core::PCWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPT_XML_KEYINFO_PARAM {
    pub wszId: windows_core::PCWSTR,
    pub wszKeyName: windows_core::PCWSTR,
    pub SKI: CRYPT_INTEGER_BLOB,
    pub wszSubjectName: windows_core::PCWSTR,
    pub cCertificate: u32,
    pub rgCertificate: *mut CRYPT_INTEGER_BLOB,
    pub cCRL: u32,
    pub rgCRL: *mut CRYPT_INTEGER_BLOB,
}
impl Default for CRYPT_XML_KEYINFO_PARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_XML_KEYINFO_SPEC(pub i32);
pub const CRYPT_XML_KEYINFO_SPEC_ENCODED: CRYPT_XML_KEYINFO_SPEC = CRYPT_XML_KEYINFO_SPEC(1i32);
pub const CRYPT_XML_KEYINFO_SPEC_NONE: CRYPT_XML_KEYINFO_SPEC = CRYPT_XML_KEYINFO_SPEC(0i32);
pub const CRYPT_XML_KEYINFO_SPEC_PARAM: CRYPT_XML_KEYINFO_SPEC = CRYPT_XML_KEYINFO_SPEC(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_XML_KEYINFO_TYPE(pub u32);
pub const CRYPT_XML_KEYINFO_TYPE_CUSTOM: CRYPT_XML_KEYINFO_TYPE = CRYPT_XML_KEYINFO_TYPE(5u32);
pub const CRYPT_XML_KEYINFO_TYPE_KEYNAME: CRYPT_XML_KEYINFO_TYPE = CRYPT_XML_KEYINFO_TYPE(1u32);
pub const CRYPT_XML_KEYINFO_TYPE_KEYVALUE: CRYPT_XML_KEYINFO_TYPE = CRYPT_XML_KEYINFO_TYPE(2u32);
pub const CRYPT_XML_KEYINFO_TYPE_RETRIEVAL: CRYPT_XML_KEYINFO_TYPE = CRYPT_XML_KEYINFO_TYPE(3u32);
pub const CRYPT_XML_KEYINFO_TYPE_X509DATA: CRYPT_XML_KEYINFO_TYPE = CRYPT_XML_KEYINFO_TYPE(4u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CRYPT_XML_KEY_DSA_KEY_VALUE {
    pub P: CRYPT_XML_DATA_BLOB,
    pub Q: CRYPT_XML_DATA_BLOB,
    pub G: CRYPT_XML_DATA_BLOB,
    pub Y: CRYPT_XML_DATA_BLOB,
    pub J: CRYPT_XML_DATA_BLOB,
    pub Seed: CRYPT_XML_DATA_BLOB,
    pub Counter: CRYPT_XML_DATA_BLOB,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CRYPT_XML_KEY_ECDSA_KEY_VALUE {
    pub wszNamedCurve: windows_core::PCWSTR,
    pub X: CRYPT_XML_DATA_BLOB,
    pub Y: CRYPT_XML_DATA_BLOB,
    pub ExplicitPara: CRYPT_XML_BLOB,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPT_XML_KEY_INFO {
    pub cbSize: u32,
    pub wszId: windows_core::PCWSTR,
    pub cKeyInfo: u32,
    pub rgKeyInfo: *mut CRYPT_XML_KEY_INFO_ITEM,
    pub hVerifyKey: BCRYPT_KEY_HANDLE,
}
impl Default for CRYPT_XML_KEY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPT_XML_KEY_INFO_ITEM {
    pub dwType: CRYPT_XML_KEYINFO_TYPE,
    pub Anonymous: CRYPT_XML_KEY_INFO_ITEM_0,
}
impl Default for CRYPT_XML_KEY_INFO_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CRYPT_XML_KEY_INFO_ITEM_0 {
    pub wszKeyName: windows_core::PCWSTR,
    pub KeyValue: CRYPT_XML_KEY_VALUE,
    pub RetrievalMethod: CRYPT_XML_BLOB,
    pub X509Data: CRYPT_XML_X509DATA,
    pub Custom: CRYPT_XML_BLOB,
}
impl Default for CRYPT_XML_KEY_INFO_ITEM_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CRYPT_XML_KEY_RSA_KEY_VALUE {
    pub Modulus: CRYPT_XML_DATA_BLOB,
    pub Exponent: CRYPT_XML_DATA_BLOB,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPT_XML_KEY_VALUE {
    pub dwType: CRYPT_XML_KEY_VALUE_TYPE,
    pub Anonymous: CRYPT_XML_KEY_VALUE_0,
}
impl Default for CRYPT_XML_KEY_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CRYPT_XML_KEY_VALUE_0 {
    pub DSAKeyValue: CRYPT_XML_KEY_DSA_KEY_VALUE,
    pub RSAKeyValue: CRYPT_XML_KEY_RSA_KEY_VALUE,
    pub ECDSAKeyValue: CRYPT_XML_KEY_ECDSA_KEY_VALUE,
    pub Custom: CRYPT_XML_BLOB,
}
impl Default for CRYPT_XML_KEY_VALUE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_XML_KEY_VALUE_TYPE(pub u32);
pub const CRYPT_XML_KEY_VALUE_TYPE_CUSTOM: CRYPT_XML_KEY_VALUE_TYPE = CRYPT_XML_KEY_VALUE_TYPE(4u32);
pub const CRYPT_XML_KEY_VALUE_TYPE_DSA: CRYPT_XML_KEY_VALUE_TYPE = CRYPT_XML_KEY_VALUE_TYPE(1u32);
pub const CRYPT_XML_KEY_VALUE_TYPE_ECDSA: CRYPT_XML_KEY_VALUE_TYPE = CRYPT_XML_KEY_VALUE_TYPE(3u32);
pub const CRYPT_XML_KEY_VALUE_TYPE_RSA: CRYPT_XML_KEY_VALUE_TYPE = CRYPT_XML_KEY_VALUE_TYPE(2u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPT_XML_OBJECT {
    pub cbSize: u32,
    pub hObject: *mut core::ffi::c_void,
    pub wszId: windows_core::PCWSTR,
    pub wszMimeType: windows_core::PCWSTR,
    pub wszEncoding: windows_core::PCWSTR,
    pub Manifest: CRYPT_XML_REFERENCES,
    pub Encoded: CRYPT_XML_BLOB,
}
impl Default for CRYPT_XML_OBJECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_XML_OBJECTS_MAX: u32 = 256u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPT_XML_PROPERTY {
    pub dwPropId: CRYPT_XML_PROPERTY_ID,
    pub pvValue: *const core::ffi::c_void,
    pub cbValue: u32,
}
impl Default for CRYPT_XML_PROPERTY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_XML_PROPERTY_DOC_DECLARATION: CRYPT_XML_PROPERTY_ID = CRYPT_XML_PROPERTY_ID(4i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_XML_PROPERTY_ID(pub i32);
pub const CRYPT_XML_PROPERTY_MAX_HEAP_SIZE: CRYPT_XML_PROPERTY_ID = CRYPT_XML_PROPERTY_ID(1i32);
pub const CRYPT_XML_PROPERTY_MAX_SIGNATURES: CRYPT_XML_PROPERTY_ID = CRYPT_XML_PROPERTY_ID(3i32);
pub const CRYPT_XML_PROPERTY_SIGNATURE_LOCATION: CRYPT_XML_PROPERTY_ID = CRYPT_XML_PROPERTY_ID(2i32);
pub const CRYPT_XML_PROPERTY_XML_OUTPUT_CHARSET: CRYPT_XML_PROPERTY_ID = CRYPT_XML_PROPERTY_ID(5i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPT_XML_REFERENCE {
    pub cbSize: u32,
    pub hReference: *mut core::ffi::c_void,
    pub wszId: windows_core::PCWSTR,
    pub wszUri: windows_core::PCWSTR,
    pub wszType: windows_core::PCWSTR,
    pub DigestMethod: CRYPT_XML_ALGORITHM,
    pub DigestValue: CRYPT_INTEGER_BLOB,
    pub cTransform: u32,
    pub rgTransform: *mut CRYPT_XML_ALGORITHM,
}
impl Default for CRYPT_XML_REFERENCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPT_XML_REFERENCES {
    pub cReference: u32,
    pub rgpReference: *mut *mut CRYPT_XML_REFERENCE,
}
impl Default for CRYPT_XML_REFERENCES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_XML_REFERENCES_MAX: u32 = 32760u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPT_XML_SIGNATURE {
    pub cbSize: u32,
    pub hSignature: *mut core::ffi::c_void,
    pub wszId: windows_core::PCWSTR,
    pub SignedInfo: CRYPT_XML_SIGNED_INFO,
    pub SignatureValue: CRYPT_INTEGER_BLOB,
    pub pKeyInfo: *mut CRYPT_XML_KEY_INFO,
    pub cObject: u32,
    pub rgpObject: *mut *mut CRYPT_XML_OBJECT,
}
impl Default for CRYPT_XML_SIGNATURE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_XML_SIGNATURES_MAX: u32 = 16u32;
pub const CRYPT_XML_SIGNATURE_VALUE_MAX: u32 = 2048u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPT_XML_SIGNED_INFO {
    pub cbSize: u32,
    pub wszId: windows_core::PCWSTR,
    pub Canonicalization: CRYPT_XML_ALGORITHM,
    pub SignatureMethod: CRYPT_XML_ALGORITHM,
    pub cReference: u32,
    pub rgpReference: *mut *mut CRYPT_XML_REFERENCE,
    pub Encoded: CRYPT_XML_BLOB,
}
impl Default for CRYPT_XML_SIGNED_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_XML_SIGN_ADD_KEYVALUE: CRYPT_XML_FLAGS = CRYPT_XML_FLAGS(1u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CRYPT_XML_STATUS {
    pub cbSize: u32,
    pub dwErrorStatus: CRYPT_XML_STATUS_ERROR_STATUS,
    pub dwInfoStatus: CRYPT_XML_STATUS_INFO_STATUS,
}
pub const CRYPT_XML_STATUS_DIGESTING: CRYPT_XML_STATUS_INFO_STATUS = CRYPT_XML_STATUS_INFO_STATUS(4u32);
pub const CRYPT_XML_STATUS_DIGEST_VALID: CRYPT_XML_STATUS_INFO_STATUS = CRYPT_XML_STATUS_INFO_STATUS(8u32);
pub const CRYPT_XML_STATUS_ERROR_DIGEST_INVALID: CRYPT_XML_STATUS_ERROR_STATUS = CRYPT_XML_STATUS_ERROR_STATUS(2u32);
pub const CRYPT_XML_STATUS_ERROR_KEYINFO_NOT_PARSED: CRYPT_XML_STATUS_ERROR_STATUS = CRYPT_XML_STATUS_ERROR_STATUS(131072u32);
pub const CRYPT_XML_STATUS_ERROR_NOT_RESOLVED: CRYPT_XML_STATUS_ERROR_STATUS = CRYPT_XML_STATUS_ERROR_STATUS(1u32);
pub const CRYPT_XML_STATUS_ERROR_NOT_SUPPORTED_ALGORITHM: CRYPT_XML_STATUS_ERROR_STATUS = CRYPT_XML_STATUS_ERROR_STATUS(5u32);
pub const CRYPT_XML_STATUS_ERROR_NOT_SUPPORTED_TRANSFORM: CRYPT_XML_STATUS_ERROR_STATUS = CRYPT_XML_STATUS_ERROR_STATUS(8u32);
pub const CRYPT_XML_STATUS_ERROR_SIGNATURE_INVALID: CRYPT_XML_STATUS_ERROR_STATUS = CRYPT_XML_STATUS_ERROR_STATUS(65536u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_XML_STATUS_ERROR_STATUS(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_XML_STATUS_INFO_STATUS(pub u32);
pub const CRYPT_XML_STATUS_INTERNAL_REFERENCE: CRYPT_XML_STATUS_INFO_STATUS = CRYPT_XML_STATUS_INFO_STATUS(1u32);
pub const CRYPT_XML_STATUS_KEY_AVAILABLE: CRYPT_XML_STATUS_INFO_STATUS = CRYPT_XML_STATUS_INFO_STATUS(2u32);
pub const CRYPT_XML_STATUS_NO_ERROR: u32 = 0u32;
pub const CRYPT_XML_STATUS_OPENED_TO_ENCODE: CRYPT_XML_STATUS_INFO_STATUS = CRYPT_XML_STATUS_INFO_STATUS(2147483648u32);
pub const CRYPT_XML_STATUS_SIGNATURE_VALID: CRYPT_XML_STATUS_INFO_STATUS = CRYPT_XML_STATUS_INFO_STATUS(65536u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPT_XML_TRANSFORM_CHAIN_CONFIG {
    pub cbSize: u32,
    pub cTransformInfo: u32,
    pub rgpTransformInfo: *mut *mut CRYPT_XML_TRANSFORM_INFO,
}
impl Default for CRYPT_XML_TRANSFORM_CHAIN_CONFIG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_XML_TRANSFORM_FLAGS(pub u32);
impl CRYPT_XML_TRANSFORM_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CRYPT_XML_TRANSFORM_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CRYPT_XML_TRANSFORM_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CRYPT_XML_TRANSFORM_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CRYPT_XML_TRANSFORM_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CRYPT_XML_TRANSFORM_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct CRYPT_XML_TRANSFORM_INFO {
    pub cbSize: u32,
    pub wszAlgorithm: windows_core::PCWSTR,
    pub cbBufferSize: u32,
    pub dwFlags: CRYPT_XML_TRANSFORM_FLAGS,
    pub pfnCreateTransform: PFN_CRYPT_XML_CREATE_TRANSFORM,
}
pub const CRYPT_XML_TRANSFORM_MAX: u32 = 16u32;
pub const CRYPT_XML_TRANSFORM_ON_NODESET: CRYPT_XML_TRANSFORM_FLAGS = CRYPT_XML_TRANSFORM_FLAGS(2u32);
pub const CRYPT_XML_TRANSFORM_ON_STREAM: CRYPT_XML_TRANSFORM_FLAGS = CRYPT_XML_TRANSFORM_FLAGS(1u32);
pub const CRYPT_XML_TRANSFORM_URI_QUERY_STRING: CRYPT_XML_TRANSFORM_FLAGS = CRYPT_XML_TRANSFORM_FLAGS(3u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPT_XML_X509DATA {
    pub cX509Data: u32,
    pub rgX509Data: *mut CRYPT_XML_X509DATA_ITEM,
}
impl Default for CRYPT_XML_X509DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPT_XML_X509DATA_ITEM {
    pub dwType: CRYPT_XML_X509DATA_TYPE,
    pub Anonymous: CRYPT_XML_X509DATA_ITEM_0,
}
impl Default for CRYPT_XML_X509DATA_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CRYPT_XML_X509DATA_ITEM_0 {
    pub IssuerSerial: CRYPT_XML_ISSUER_SERIAL,
    pub SKI: CRYPT_XML_DATA_BLOB,
    pub wszSubjectName: windows_core::PCWSTR,
    pub Certificate: CRYPT_XML_DATA_BLOB,
    pub CRL: CRYPT_XML_DATA_BLOB,
    pub Custom: CRYPT_XML_BLOB,
}
impl Default for CRYPT_XML_X509DATA_ITEM_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_XML_X509DATA_TYPE(pub u32);
pub const CRYPT_XML_X509DATA_TYPE_CERTIFICATE: CRYPT_XML_X509DATA_TYPE = CRYPT_XML_X509DATA_TYPE(4u32);
pub const CRYPT_XML_X509DATA_TYPE_CRL: CRYPT_XML_X509DATA_TYPE = CRYPT_XML_X509DATA_TYPE(5u32);
pub const CRYPT_XML_X509DATA_TYPE_CUSTOM: CRYPT_XML_X509DATA_TYPE = CRYPT_XML_X509DATA_TYPE(6u32);
pub const CRYPT_XML_X509DATA_TYPE_ISSUER_SERIAL: CRYPT_XML_X509DATA_TYPE = CRYPT_XML_X509DATA_TYPE(1u32);
pub const CRYPT_XML_X509DATA_TYPE_SKI: CRYPT_XML_X509DATA_TYPE = CRYPT_XML_X509DATA_TYPE(2u32);
pub const CRYPT_XML_X509DATA_TYPE_SUBJECT_NAME: CRYPT_XML_X509DATA_TYPE = CRYPT_XML_X509DATA_TYPE(3u32);
pub const CRYPT_Y_ONLY: CRYPT_KEY_FLAGS = CRYPT_KEY_FLAGS(1u32);
pub const CSF_AUTHENTICATE: windows_core::PCWSTR = windows_core::w!("Authenticate");
pub const CSF_CHANGE_AUTHENTICATOR: windows_core::PCWSTR = windows_core::w!("Change Authenticator");
pub const CSF_IMPORT_KEYPAIR: windows_core::PCWSTR = windows_core::w!("Import Key Pair");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CTL_ANY_SUBJECT_INFO {
    pub SubjectAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub SubjectIdentifier: CRYPT_INTEGER_BLOB,
}
pub const CTL_ANY_SUBJECT_TYPE: u32 = 1u32;
pub const CTL_CERT_SUBJECT_TYPE: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CTL_CONTEXT {
    pub dwMsgAndCertEncodingType: u32,
    pub pbCtlEncoded: *mut u8,
    pub cbCtlEncoded: u32,
    pub pCtlInfo: *mut CTL_INFO,
    pub hCertStore: HCERTSTORE,
    pub hCryptMsg: *mut core::ffi::c_void,
    pub pbCtlContent: *mut u8,
    pub cbCtlContent: u32,
}
impl Default for CTL_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CTL_ENTRY {
    pub SubjectIdentifier: CRYPT_INTEGER_BLOB,
    pub cAttribute: u32,
    pub rgAttribute: *mut CRYPT_ATTRIBUTE,
}
impl Default for CTL_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CTL_ENTRY_FROM_PROP_CHAIN_FLAG: u32 = 1u32;
pub const CTL_FIND_ANY: CERT_FIND_TYPE = CERT_FIND_TYPE(0u32);
pub const CTL_FIND_EXISTING: CERT_FIND_TYPE = CERT_FIND_TYPE(5u32);
pub const CTL_FIND_MD5_HASH: CERT_FIND_TYPE = CERT_FIND_TYPE(2u32);
pub const CTL_FIND_NO_LIST_ID_CBDATA: u32 = 4294967295u32;
pub const CTL_FIND_SAME_USAGE_FLAG: CERT_FIND_TYPE = CERT_FIND_TYPE(1u32);
pub const CTL_FIND_SHA1_HASH: CERT_FIND_TYPE = CERT_FIND_TYPE(1u32);
pub const CTL_FIND_SUBJECT: CERT_FIND_TYPE = CERT_FIND_TYPE(4u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CTL_FIND_SUBJECT_PARA {
    pub cbSize: u32,
    pub pUsagePara: *mut CTL_FIND_USAGE_PARA,
    pub dwSubjectType: u32,
    pub pvSubject: *mut core::ffi::c_void,
}
impl Default for CTL_FIND_SUBJECT_PARA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CTL_FIND_USAGE: CERT_FIND_TYPE = CERT_FIND_TYPE(3u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CTL_FIND_USAGE_PARA {
    pub cbSize: u32,
    pub SubjectUsage: CTL_USAGE,
    pub ListIdentifier: CRYPT_INTEGER_BLOB,
    pub pSigner: *mut CERT_INFO,
}
impl Default for CTL_FIND_USAGE_PARA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CTL_INFO {
    pub dwVersion: u32,
    pub SubjectUsage: CTL_USAGE,
    pub ListIdentifier: CRYPT_INTEGER_BLOB,
    pub SequenceNumber: CRYPT_INTEGER_BLOB,
    pub ThisUpdate: super::super::Foundation::FILETIME,
    pub NextUpdate: super::super::Foundation::FILETIME,
    pub SubjectAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub cCTLEntry: u32,
    pub rgCTLEntry: *mut CTL_ENTRY,
    pub cExtension: u32,
    pub rgExtension: *mut CERT_EXTENSION,
}
impl Default for CTL_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CTL_USAGE {
    pub cUsageIdentifier: u32,
    pub rgpszUsageIdentifier: *mut windows_core::PSTR,
}
impl Default for CTL_USAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CTL_USAGE_MATCH {
    pub dwType: u32,
    pub Usage: CTL_USAGE,
}
pub const CTL_V1: u32 = 0u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CTL_VERIFY_USAGE_PARA {
    pub cbSize: u32,
    pub ListIdentifier: CRYPT_INTEGER_BLOB,
    pub cCtlStore: u32,
    pub rghCtlStore: *mut HCERTSTORE,
    pub cSignerStore: u32,
    pub rghSignerStore: *mut HCERTSTORE,
}
impl Default for CTL_VERIFY_USAGE_PARA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CTL_VERIFY_USAGE_STATUS {
    pub cbSize: u32,
    pub dwError: u32,
    pub dwFlags: u32,
    pub ppCtl: *mut *mut CTL_CONTEXT,
    pub dwCtlEntryIndex: u32,
    pub ppSigner: *mut *mut CERT_CONTEXT,
    pub dwSignerIndex: u32,
}
impl Default for CTL_VERIFY_USAGE_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CUR_BLOB_VERSION: u32 = 2u32;
pub const CUR_OFFLOAD_VERSION: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CertKeyType(pub u32);
pub const ChallengeResponsePinType: SECRET_TYPE = SECRET_TYPE(2i32);
pub type CryptXmlDllCloseDigest = Option<unsafe extern "system" fn(hdigest: *const core::ffi::c_void) -> windows_core::HRESULT>;
pub type CryptXmlDllCreateDigest = Option<unsafe extern "system" fn(pdigestmethod: *const CRYPT_XML_ALGORITHM, pcbsize: *mut u32, phdigest: *mut *mut core::ffi::c_void) -> windows_core::HRESULT>;
pub type CryptXmlDllCreateKey = Option<unsafe extern "system" fn(pencoded: *const CRYPT_XML_BLOB, phkey: *mut BCRYPT_KEY_HANDLE) -> windows_core::HRESULT>;
pub type CryptXmlDllDigestData = Option<unsafe extern "system" fn(hdigest: *const core::ffi::c_void, pbdata: *const u8, cbdata: u32) -> windows_core::HRESULT>;
pub type CryptXmlDllEncodeAlgorithm = Option<unsafe extern "system" fn(palginfo: *const CRYPT_XML_ALGORITHM_INFO, dwcharset: CRYPT_XML_CHARSET, pvcallbackstate: *mut core::ffi::c_void, pfnwrite: PFN_CRYPT_XML_WRITE_CALLBACK) -> windows_core::HRESULT>;
pub type CryptXmlDllEncodeKeyValue = Option<unsafe extern "system" fn(hkey: NCRYPT_KEY_HANDLE, dwcharset: CRYPT_XML_CHARSET, pvcallbackstate: *mut core::ffi::c_void, pfnwrite: PFN_CRYPT_XML_WRITE_CALLBACK) -> windows_core::HRESULT>;
pub type CryptXmlDllFinalizeDigest = Option<unsafe extern "system" fn(hdigest: *const core::ffi::c_void, pbdigest: *mut u8, cbdigest: u32) -> windows_core::HRESULT>;
pub type CryptXmlDllGetAlgorithmInfo = Option<unsafe extern "system" fn(pxmlalgorithm: *const CRYPT_XML_ALGORITHM, ppalginfo: *mut *mut CRYPT_XML_ALGORITHM_INFO) -> windows_core::HRESULT>;
pub type CryptXmlDllGetInterface = Option<unsafe extern "system" fn(dwflags: u32, pmethod: *const CRYPT_XML_ALGORITHM_INFO, pinterface: *mut CRYPT_XML_CRYPTOGRAPHIC_INTERFACE) -> windows_core::HRESULT>;
pub type CryptXmlDllSignData = Option<unsafe extern "system" fn(psignaturemethod: *const CRYPT_XML_ALGORITHM, hcryptprovorncryptkey: HCRYPTPROV_OR_NCRYPT_KEY_HANDLE, dwkeyspec: u32, pbinput: *const u8, cbinput: u32, pboutput: *mut u8, cboutput: u32, pcbresult: *mut u32) -> windows_core::HRESULT>;
pub type CryptXmlDllVerifySignature = Option<unsafe extern "system" fn(psignaturemethod: *const CRYPT_XML_ALGORITHM, hkey: BCRYPT_KEY_HANDLE, pbinput: *const u8, cbinput: u32, pbsignature: *const u8, cbsignature: u32) -> windows_core::HRESULT>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DSAFIPSVERSION_ENUM(pub i32);
pub const DSA_FIPS186_2: DSAFIPSVERSION_ENUM = DSAFIPSVERSION_ENUM(0i32);
pub const DSA_FIPS186_3: DSAFIPSVERSION_ENUM = DSAFIPSVERSION_ENUM(1i32);
pub const DSA_HASH_ALGORITHM_SHA1: HASHALGORITHM_ENUM = HASHALGORITHM_ENUM(0i32);
pub const DSA_HASH_ALGORITHM_SHA256: HASHALGORITHM_ENUM = HASHALGORITHM_ENUM(1i32);
pub const DSA_HASH_ALGORITHM_SHA512: HASHALGORITHM_ENUM = HASHALGORITHM_ENUM(2i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DSSSEED {
    pub counter: u32,
    pub seed: [u8; 20],
}
impl Default for DSSSEED {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DTLS1_0_PROTOCOL_VERSION: u32 = 65279u32;
pub const DTLS1_2_PROTOCOL_VERSION: u32 = 65277u32;
pub const DigitalSignaturePin: SECRET_PURPOSE = SECRET_PURPOSE(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Direction(pub i32);
pub const DirectionDecrypt: Direction = Direction(2i32);
pub const DirectionEncrypt: Direction = Direction(1i32);
pub const ECC_CMS_SHARED_INFO: windows_core::PCSTR = windows_core::PCSTR(77i32 as _);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ECC_CURVE_ALG_ID_ENUM(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ECC_CURVE_TYPE_ENUM(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ENDPOINTADDRESS {
    pub serviceUrl: windows_core::PCWSTR,
    pub policyUrl: windows_core::PCWSTR,
    pub rawCertificate: CRYPT_INTEGER_BLOB,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ENDPOINTADDRESS2 {
    pub serviceUrl: windows_core::PCWSTR,
    pub policyUrl: windows_core::PCWSTR,
    pub identityType: u32,
    pub identityBytes: *mut core::ffi::c_void,
}
impl Default for ENDPOINTADDRESS2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ENUM_CEPSETUPPROP_AUTHENTICATION: CEPSetupProperty = CEPSetupProperty(0i32);
pub const ENUM_CEPSETUPPROP_CAINFORMATION: MSCEPSetupProperty = MSCEPSetupProperty(11i32);
pub const ENUM_CEPSETUPPROP_CHALLENGEURL: MSCEPSetupProperty = MSCEPSetupProperty(13i32);
pub const ENUM_CEPSETUPPROP_EXCHANGEKEYINFORMATION: MSCEPSetupProperty = MSCEPSetupProperty(10i32);
pub const ENUM_CEPSETUPPROP_KEYBASED_RENEWAL: CEPSetupProperty = CEPSetupProperty(3i32);
pub const ENUM_CEPSETUPPROP_MSCEPURL: MSCEPSetupProperty = MSCEPSetupProperty(12i32);
pub const ENUM_CEPSETUPPROP_RANAME_CITY: MSCEPSetupProperty = MSCEPSetupProperty(6i32);
pub const ENUM_CEPSETUPPROP_RANAME_CN: MSCEPSetupProperty = MSCEPSetupProperty(2i32);
pub const ENUM_CEPSETUPPROP_RANAME_COMPANY: MSCEPSetupProperty = MSCEPSetupProperty(4i32);
pub const ENUM_CEPSETUPPROP_RANAME_COUNTRY: MSCEPSetupProperty = MSCEPSetupProperty(8i32);
pub const ENUM_CEPSETUPPROP_RANAME_DEPT: MSCEPSetupProperty = MSCEPSetupProperty(5i32);
pub const ENUM_CEPSETUPPROP_RANAME_EMAIL: MSCEPSetupProperty = MSCEPSetupProperty(3i32);
pub const ENUM_CEPSETUPPROP_RANAME_STATE: MSCEPSetupProperty = MSCEPSetupProperty(7i32);
pub const ENUM_CEPSETUPPROP_SIGNINGKEYINFORMATION: MSCEPSetupProperty = MSCEPSetupProperty(9i32);
pub const ENUM_CEPSETUPPROP_SSLCERTHASH: CEPSetupProperty = CEPSetupProperty(1i32);
pub const ENUM_CEPSETUPPROP_URL: CEPSetupProperty = CEPSetupProperty(2i32);
pub const ENUM_CEPSETUPPROP_USECHALLENGE: MSCEPSetupProperty = MSCEPSetupProperty(1i32);
pub const ENUM_CEPSETUPPROP_USELOCALSYSTEM: MSCEPSetupProperty = MSCEPSetupProperty(0i32);
pub const ENUM_CESSETUPPROP_ALLOW_KEYBASED_RENEWAL: CESSetupProperty = CESSetupProperty(6i32);
pub const ENUM_CESSETUPPROP_AUTHENTICATION: CESSetupProperty = CESSetupProperty(2i32);
pub const ENUM_CESSETUPPROP_CACONFIG: CESSetupProperty = CESSetupProperty(1i32);
pub const ENUM_CESSETUPPROP_RENEWALONLY: CESSetupProperty = CESSetupProperty(5i32);
pub const ENUM_CESSETUPPROP_SSLCERTHASH: CESSetupProperty = CESSetupProperty(3i32);
pub const ENUM_CESSETUPPROP_URL: CESSetupProperty = CESSetupProperty(4i32);
pub const ENUM_CESSETUPPROP_USE_IISAPPPOOLIDENTITY: CESSetupProperty = CESSetupProperty(0i32);
pub const ENUM_SETUPPROP_CADSSUFFIX: CASetupProperty = CASetupProperty(4i32);
pub const ENUM_SETUPPROP_CAKEYINFORMATION: CASetupProperty = CASetupProperty(1i32);
pub const ENUM_SETUPPROP_CANAME: CASetupProperty = CASetupProperty(3i32);
pub const ENUM_SETUPPROP_CATYPE: CASetupProperty = CASetupProperty(0i32);
pub const ENUM_SETUPPROP_DATABASEDIRECTORY: CASetupProperty = CASetupProperty(9i32);
pub const ENUM_SETUPPROP_EXPIRATIONDATE: CASetupProperty = CASetupProperty(7i32);
pub const ENUM_SETUPPROP_INTERACTIVE: CASetupProperty = CASetupProperty(2i32);
pub const ENUM_SETUPPROP_INVALID: CASetupProperty = CASetupProperty(-1i32);
pub const ENUM_SETUPPROP_LOGDIRECTORY: CASetupProperty = CASetupProperty(10i32);
pub const ENUM_SETUPPROP_PARENTCAMACHINE: CASetupProperty = CASetupProperty(12i32);
pub const ENUM_SETUPPROP_PARENTCANAME: CASetupProperty = CASetupProperty(13i32);
pub const ENUM_SETUPPROP_PRESERVEDATABASE: CASetupProperty = CASetupProperty(8i32);
pub const ENUM_SETUPPROP_REQUESTFILE: CASetupProperty = CASetupProperty(14i32);
pub const ENUM_SETUPPROP_SHAREDFOLDER: CASetupProperty = CASetupProperty(11i32);
pub const ENUM_SETUPPROP_VALIDITYPERIOD: CASetupProperty = CASetupProperty(5i32);
pub const ENUM_SETUPPROP_VALIDITYPERIODUNIT: CASetupProperty = CASetupProperty(6i32);
pub const ENUM_SETUPPROP_WEBCAMACHINE: CASetupProperty = CASetupProperty(15i32);
pub const ENUM_SETUPPROP_WEBCANAME: CASetupProperty = CASetupProperty(16i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EV_EXTRA_CERT_CHAIN_POLICY_PARA {
    pub cbSize: u32,
    pub dwRootProgramQualifierFlags: CERT_ROOT_PROGRAM_FLAGS,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EV_EXTRA_CERT_CHAIN_POLICY_STATUS {
    pub cbSize: u32,
    pub dwQualifiers: u32,
    pub dwIssuanceUsageIndex: u32,
}
pub const EXPORT_PRIVATE_KEYS: u32 = 4u32;
pub const EXPO_OFFLOAD_FUNC_NAME: windows_core::PCSTR = windows_core::s!("OffloadModExpo");
pub const EXPO_OFFLOAD_REG_VALUE: windows_core::PCSTR = windows_core::s!("ExpoOffload");
pub const E_ICARD_ARGUMENT: windows_core::HRESULT = windows_core::HRESULT(0xC0050105_u32 as _);
pub const E_ICARD_COMMUNICATION: windows_core::HRESULT = windows_core::HRESULT(0xC0050100_u32 as _);
pub const E_ICARD_DATA_ACCESS: windows_core::HRESULT = windows_core::HRESULT(0xC0050101_u32 as _);
pub const E_ICARD_EXPORT: windows_core::HRESULT = windows_core::HRESULT(0xC0050102_u32 as _);
pub const E_ICARD_FAIL: windows_core::HRESULT = windows_core::HRESULT(0xC0050115_u32 as _);
pub const E_ICARD_FAILED_REQUIRED_CLAIMS: windows_core::HRESULT = windows_core::HRESULT(0xC0050184_u32 as _);
pub const E_ICARD_IDENTITY: windows_core::HRESULT = windows_core::HRESULT(0xC0050103_u32 as _);
pub const E_ICARD_IMPORT: windows_core::HRESULT = windows_core::HRESULT(0xC0050104_u32 as _);
pub const E_ICARD_INFORMATIONCARD: windows_core::HRESULT = windows_core::HRESULT(0xC0050107_u32 as _);
pub const E_ICARD_INVALID_PROOF_KEY: windows_core::HRESULT = windows_core::HRESULT(0xC0050182_u32 as _);
pub const E_ICARD_LOGOVALIDATION: windows_core::HRESULT = windows_core::HRESULT(0xC0050109_u32 as _);
pub const E_ICARD_MISSING_APPLIESTO: windows_core::HRESULT = windows_core::HRESULT(0xC0050181_u32 as _);
pub const E_ICARD_PASSWORDVALIDATION: windows_core::HRESULT = windows_core::HRESULT(0xC005010A_u32 as _);
pub const E_ICARD_POLICY: windows_core::HRESULT = windows_core::HRESULT(0xC005010B_u32 as _);
pub const E_ICARD_PROCESSDIED: windows_core::HRESULT = windows_core::HRESULT(0xC005010C_u32 as _);
pub const E_ICARD_REFRESH_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0xC0050180_u32 as _);
pub const E_ICARD_REQUEST: windows_core::HRESULT = windows_core::HRESULT(0xC0050106_u32 as _);
pub const E_ICARD_SERVICE: windows_core::HRESULT = windows_core::HRESULT(0xC005010E_u32 as _);
pub const E_ICARD_SERVICEBUSY: windows_core::HRESULT = windows_core::HRESULT(0xC005010D_u32 as _);
pub const E_ICARD_SHUTTINGDOWN: windows_core::HRESULT = windows_core::HRESULT(0xC005010F_u32 as _);
pub const E_ICARD_STOREKEY: windows_core::HRESULT = windows_core::HRESULT(0xC0050108_u32 as _);
pub const E_ICARD_STORE_IMPORT: windows_core::HRESULT = windows_core::HRESULT(0xC0050114_u32 as _);
pub const E_ICARD_TOKENCREATION: windows_core::HRESULT = windows_core::HRESULT(0xC0050110_u32 as _);
pub const E_ICARD_TRUSTEXCHANGE: windows_core::HRESULT = windows_core::HRESULT(0xC0050111_u32 as _);
pub const E_ICARD_UI_INITIALIZATION: windows_core::HRESULT = windows_core::HRESULT(0xC005011A_u32 as _);
pub const E_ICARD_UNKNOWN_REFERENCE: windows_core::HRESULT = windows_core::HRESULT(0xC0050183_u32 as _);
pub const E_ICARD_UNTRUSTED: windows_core::HRESULT = windows_core::HRESULT(0xC0050112_u32 as _);
pub const E_ICARD_USERCANCELLED: windows_core::HRESULT = windows_core::HRESULT(0xC0050113_u32 as _);
pub const EmptyPinType: SECRET_TYPE = SECRET_TYPE(3i32);
pub const EncryptionPin: SECRET_PURPOSE = SECRET_PURPOSE(2i32);
pub const EveryoneReadAdminWriteAc: CARD_FILE_ACCESS_CONDITION = CARD_FILE_ACCESS_CONDITION(3i32);
pub const EveryoneReadUserWriteAc: CARD_FILE_ACCESS_CONDITION = CARD_FILE_ACCESS_CONDITION(1i32);
pub const ExternalPinType: SECRET_TYPE = SECRET_TYPE(1i32);
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct GENERIC_XML_TOKEN {
    pub createDate: super::super::Foundation::FILETIME,
    pub expiryDate: super::super::Foundation::FILETIME,
    pub xmlToken: windows_core::PWSTR,
    pub internalTokenReference: windows_core::PWSTR,
    pub externalTokenReference: windows_core::PWSTR,
}
pub type GetAsymmetricEncryptionInterfaceFn = Option<unsafe extern "system" fn(pszprovidername: windows_core::PCWSTR, pszalgid: windows_core::PCWSTR, ppfunctiontable: *mut *mut BCRYPT_ASYMMETRIC_ENCRYPTION_FUNCTION_TABLE, dwflags: u32) -> super::super::Foundation::NTSTATUS>;
pub type GetCipherInterfaceFn = Option<unsafe extern "system" fn(pszprovidername: windows_core::PCWSTR, pszalgid: windows_core::PCWSTR, ppfunctiontable: *mut *mut BCRYPT_CIPHER_FUNCTION_TABLE, dwflags: u32) -> super::super::Foundation::NTSTATUS>;
pub type GetHashInterfaceFn = Option<unsafe extern "system" fn(pszprovidername: windows_core::PCWSTR, pszalgid: windows_core::PCWSTR, ppfunctiontable: *mut *mut BCRYPT_HASH_FUNCTION_TABLE, dwflags: u32) -> super::super::Foundation::NTSTATUS>;
pub type GetKeyDerivationInterfaceFn = Option<unsafe extern "system" fn(pszprovidername: windows_core::PCWSTR, pszalgid: windows_core::PCWSTR, ppfunctiontable: *mut *mut BCRYPT_KEY_DERIVATION_FUNCTION_TABLE, dwflags: u32) -> super::super::Foundation::NTSTATUS>;
pub type GetKeyStorageInterfaceFn = Option<unsafe extern "system" fn(pszprovidername: windows_core::PCWSTR, ppfunctiontable: *mut *mut NCRYPT_KEY_STORAGE_FUNCTION_TABLE, dwflags: u32) -> super::super::Foundation::NTSTATUS>;
pub type GetRngInterfaceFn = Option<unsafe extern "system" fn(pszprovidername: windows_core::PCWSTR, ppfunctiontable: *mut *mut BCRYPT_RNG_FUNCTION_TABLE, dwflags: u32) -> super::super::Foundation::NTSTATUS>;
pub type GetSChannelInterfaceFn = Option<unsafe extern "system" fn(pszprovidername: windows_core::PCWSTR, ppfunctiontable: *mut *mut NCRYPT_SSL_FUNCTION_TABLE, dwflags: u32) -> super::super::Foundation::NTSTATUS>;
pub type GetSecretAgreementInterfaceFn = Option<unsafe extern "system" fn(pszprovidername: windows_core::PCWSTR, pszalgid: windows_core::PCWSTR, ppfunctiontable: *mut *mut BCRYPT_SECRET_AGREEMENT_FUNCTION_TABLE, dwflags: u32) -> super::super::Foundation::NTSTATUS>;
pub type GetSignatureInterfaceFn = Option<unsafe extern "system" fn(pszprovidername: windows_core::PCWSTR, pszalgid: windows_core::PCWSTR, ppfunctiontable: *mut *mut BCRYPT_SIGNATURE_FUNCTION_TABLE, dwflags: u32) -> super::super::Foundation::NTSTATUS>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HASHALGORITHM_ENUM(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HCERTCHAINENGINE(pub *mut core::ffi::c_void);
impl HCERTCHAINENGINE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for HCERTCHAINENGINE {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_core::link!("crypt32.dll" "system" fn CertFreeCertificateChainEngine(hchainengine : *mut core::ffi::c_void));
            unsafe {
                CertFreeCertificateChainEngine(self.0);
            }
        }
    }
}
impl Default for HCERTCHAINENGINE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HCERTSTORE(pub *mut core::ffi::c_void);
impl HCERTSTORE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl Default for HCERTSTORE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HCERTSTOREPROV(pub *mut core::ffi::c_void);
impl HCERTSTOREPROV {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl Default for HCERTSTOREPROV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HCRYPTASYNC(pub isize);
impl HCRYPTASYNC {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl windows_core::Free for HCRYPTASYNC {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_core::link!("crypt32.dll" "system" fn CryptCloseAsyncHandle(hasync : isize) -> i32);
            unsafe {
                CryptCloseAsyncHandle(self.0);
            }
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HCRYPTPROV_LEGACY(pub usize);
impl HCRYPTPROV_LEGACY {
    pub fn is_invalid(&self) -> bool {
        self.0 == 0
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HCRYPTPROV_OR_NCRYPT_KEY_HANDLE(pub usize);
impl HCRYPTPROV_OR_NCRYPT_KEY_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == 0
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HMAC_INFO {
    pub HashAlgid: ALG_ID,
    pub pbInnerString: *mut u8,
    pub cbInnerString: u32,
    pub pbOuterString: *mut u8,
    pub cbOuterString: u32,
}
impl Default for HMAC_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HP_ALGID: u32 = 1u32;
pub const HP_HASHSIZE: u32 = 4u32;
pub const HP_HASHVAL: CRYPT_SET_HASH_PARAM = CRYPT_SET_HASH_PARAM(2u32);
pub const HP_HMAC_INFO: CRYPT_SET_HASH_PARAM = CRYPT_SET_HASH_PARAM(5u32);
pub const HP_TLS1PRF_LABEL: u32 = 6u32;
pub const HP_TLS1PRF_SEED: u32 = 7u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTPSPOLICY_CALLBACK_DATA_AUTH_TYPE(pub u32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HTTPSPolicyCallbackData {
    pub Anonymous: HTTPSPolicyCallbackData_0,
    pub dwAuthType: HTTPSPOLICY_CALLBACK_DATA_AUTH_TYPE,
    pub fdwChecks: u32,
    pub pwszServerName: windows_core::PWSTR,
}
impl Default for HTTPSPolicyCallbackData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union HTTPSPolicyCallbackData_0 {
    pub cbStruct: u32,
    pub cbSize: u32,
}
impl Default for HTTPSPolicyCallbackData_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HandleType(pub i32);
impl HandleType {
    pub const Asymmetric: Self = Self(1i32);
    pub const Symmetric: Self = Self(2i32);
    pub const Transform: Self = Self(3i32);
    pub const Hash: Self = Self(4i32);
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICertSrvSetup, ICertSrvSetup_Vtbl, 0xb760a1bb_4784_44c0_8f12_555f0780ff25);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICertSrvSetup {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICertSrvSetup, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ICertSrvSetup {
    pub unsafe fn CAErrorId(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CAErrorId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CAErrorString(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CAErrorString)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn InitializeDefaults(&self, bserver: super::super::Foundation::VARIANT_BOOL, bclient: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).InitializeDefaults)(windows_core::Interface::as_raw(self), bserver, bclient).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetCASetupProperty(&self, propertyid: CASetupProperty) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCASetupProperty)(windows_core::Interface::as_raw(self), propertyid, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetCASetupProperty(&self, propertyid: CASetupProperty, ppropertyvalue: *const super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCASetupProperty)(windows_core::Interface::as_raw(self), propertyid, core::mem::transmute(ppropertyvalue)).ok() }
    }
    pub unsafe fn IsPropertyEditable(&self, propertyid: CASetupProperty) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsPropertyEditable)(windows_core::Interface::as_raw(self), propertyid, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetSupportedCATypes(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSupportedCATypes)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetProviderNameList(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProviderNameList)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetKeyLengthList(&self, bstrprovidername: &windows_core::BSTR) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetKeyLengthList)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprovidername), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetHashAlgorithmList(&self, bstrprovidername: &windows_core::BSTR) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHashAlgorithmList)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprovidername), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetPrivateKeyContainerList(&self, bstrprovidername: &windows_core::BSTR) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPrivateKeyContainerList)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprovidername), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetExistingCACertificates(&self) -> windows_core::Result<ICertSrvSetupKeyInformationCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetExistingCACertificates)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CAImportPFX(&self, bstrfilename: &windows_core::BSTR, bstrpasswd: &windows_core::BSTR, boverwriteexistingkey: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<ICertSrvSetupKeyInformation> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CAImportPFX)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrfilename), core::mem::transmute_copy(bstrpasswd), boverwriteexistingkey, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetCADistinguishedName(&self, bstrcadn: &windows_core::BSTR, bignoreunicode: super::super::Foundation::VARIANT_BOOL, boverwriteexistingkey: super::super::Foundation::VARIANT_BOOL, boverwriteexistingcainds: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCADistinguishedName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrcadn), bignoreunicode, boverwriteexistingkey, boverwriteexistingcainds).ok() }
    }
    pub unsafe fn SetDatabaseInformation(&self, bstrdbdirectory: &windows_core::BSTR, bstrlogdirectory: &windows_core::BSTR, bstrsharedfolder: &windows_core::BSTR, bforceoverwrite: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDatabaseInformation)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdbdirectory), core::mem::transmute_copy(bstrlogdirectory), core::mem::transmute_copy(bstrsharedfolder), bforceoverwrite).ok() }
    }
    pub unsafe fn SetParentCAInformation(&self, bstrcaconfiguration: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetParentCAInformation)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrcaconfiguration)).ok() }
    }
    pub unsafe fn SetWebCAInformation(&self, bstrcaconfiguration: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetWebCAInformation)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrcaconfiguration)).ok() }
    }
    pub unsafe fn Install(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Install)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn PreUnInstall(&self, bclientonly: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PreUnInstall)(windows_core::Interface::as_raw(self), bclientonly).ok() }
    }
    pub unsafe fn PostUnInstall(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PostUnInstall)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ICertSrvSetup_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub CAErrorId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CAErrorString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InitializeDefaults: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetCASetupProperty: unsafe extern "system" fn(*mut core::ffi::c_void, CASetupProperty, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetCASetupProperty: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetCASetupProperty: unsafe extern "system" fn(*mut core::ffi::c_void, CASetupProperty, *const super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetCASetupProperty: usize,
    pub IsPropertyEditable: unsafe extern "system" fn(*mut core::ffi::c_void, CASetupProperty, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetSupportedCATypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetSupportedCATypes: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetProviderNameList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetProviderNameList: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetKeyLengthList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetKeyLengthList: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetHashAlgorithmList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetHashAlgorithmList: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetPrivateKeyContainerList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetPrivateKeyContainerList: usize,
    pub GetExistingCACertificates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CAImportPFX: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCADistinguishedName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL, super::super::Foundation::VARIANT_BOOL, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetDatabaseInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetParentCAInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetWebCAInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Install: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PreUnInstall: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub PostUnInstall: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICertSrvSetup_Impl: super::super::System::Com::IDispatch_Impl {
    fn CAErrorId(&self) -> windows_core::Result<i32>;
    fn CAErrorString(&self) -> windows_core::Result<windows_core::BSTR>;
    fn InitializeDefaults(&self, bserver: super::super::Foundation::VARIANT_BOOL, bclient: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn GetCASetupProperty(&self, propertyid: CASetupProperty) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetCASetupProperty(&self, propertyid: CASetupProperty, ppropertyvalue: *const super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn IsPropertyEditable(&self, propertyid: CASetupProperty) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetSupportedCATypes(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn GetProviderNameList(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn GetKeyLengthList(&self, bstrprovidername: &windows_core::BSTR) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn GetHashAlgorithmList(&self, bstrprovidername: &windows_core::BSTR) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn GetPrivateKeyContainerList(&self, bstrprovidername: &windows_core::BSTR) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn GetExistingCACertificates(&self) -> windows_core::Result<ICertSrvSetupKeyInformationCollection>;
    fn CAImportPFX(&self, bstrfilename: &windows_core::BSTR, bstrpasswd: &windows_core::BSTR, boverwriteexistingkey: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<ICertSrvSetupKeyInformation>;
    fn SetCADistinguishedName(&self, bstrcadn: &windows_core::BSTR, bignoreunicode: super::super::Foundation::VARIANT_BOOL, boverwriteexistingkey: super::super::Foundation::VARIANT_BOOL, boverwriteexistingcainds: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetDatabaseInformation(&self, bstrdbdirectory: &windows_core::BSTR, bstrlogdirectory: &windows_core::BSTR, bstrsharedfolder: &windows_core::BSTR, bforceoverwrite: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetParentCAInformation(&self, bstrcaconfiguration: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetWebCAInformation(&self, bstrcaconfiguration: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Install(&self) -> windows_core::Result<()>;
    fn PreUnInstall(&self, bclientonly: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn PostUnInstall(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICertSrvSetup_Vtbl {
    pub const fn new<Identity: ICertSrvSetup_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CAErrorId<Identity: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertSrvSetup_Impl::CAErrorId(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CAErrorString<Identity: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertSrvSetup_Impl::CAErrorString(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InitializeDefaults<Identity: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bserver: super::super::Foundation::VARIANT_BOOL, bclient: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertSrvSetup_Impl::InitializeDefaults(this, core::mem::transmute_copy(&bserver), core::mem::transmute_copy(&bclient)).into()
            }
        }
        unsafe extern "system" fn GetCASetupProperty<Identity: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: CASetupProperty, ppropertyvalue: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertSrvSetup_Impl::GetCASetupProperty(this, core::mem::transmute_copy(&propertyid)) {
                    Ok(ok__) => {
                        ppropertyvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCASetupProperty<Identity: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: CASetupProperty, ppropertyvalue: *const super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertSrvSetup_Impl::SetCASetupProperty(this, core::mem::transmute_copy(&propertyid), core::mem::transmute_copy(&ppropertyvalue)).into()
            }
        }
        unsafe extern "system" fn IsPropertyEditable<Identity: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: CASetupProperty, pbeditable: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertSrvSetup_Impl::IsPropertyEditable(this, core::mem::transmute_copy(&propertyid)) {
                    Ok(ok__) => {
                        pbeditable.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSupportedCATypes<Identity: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcatypes: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertSrvSetup_Impl::GetSupportedCATypes(this) {
                    Ok(ok__) => {
                        pcatypes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetProviderNameList<Identity: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertSrvSetup_Impl::GetProviderNameList(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetKeyLengthList<Identity: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprovidername: *mut core::ffi::c_void, pval: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertSrvSetup_Impl::GetKeyLengthList(this, core::mem::transmute(&bstrprovidername)) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetHashAlgorithmList<Identity: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprovidername: *mut core::ffi::c_void, pval: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertSrvSetup_Impl::GetHashAlgorithmList(this, core::mem::transmute(&bstrprovidername)) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPrivateKeyContainerList<Identity: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprovidername: *mut core::ffi::c_void, pval: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertSrvSetup_Impl::GetPrivateKeyContainerList(this, core::mem::transmute(&bstrprovidername)) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetExistingCACertificates<Identity: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertSrvSetup_Impl::GetExistingCACertificates(this) {
                    Ok(ok__) => {
                        ppval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CAImportPFX<Identity: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrfilename: *mut core::ffi::c_void, bstrpasswd: *mut core::ffi::c_void, boverwriteexistingkey: super::super::Foundation::VARIANT_BOOL, ppval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertSrvSetup_Impl::CAImportPFX(this, core::mem::transmute(&bstrfilename), core::mem::transmute(&bstrpasswd), core::mem::transmute_copy(&boverwriteexistingkey)) {
                    Ok(ok__) => {
                        ppval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCADistinguishedName<Identity: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcadn: *mut core::ffi::c_void, bignoreunicode: super::super::Foundation::VARIANT_BOOL, boverwriteexistingkey: super::super::Foundation::VARIANT_BOOL, boverwriteexistingcainds: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertSrvSetup_Impl::SetCADistinguishedName(this, core::mem::transmute(&bstrcadn), core::mem::transmute_copy(&bignoreunicode), core::mem::transmute_copy(&boverwriteexistingkey), core::mem::transmute_copy(&boverwriteexistingcainds)).into()
            }
        }
        unsafe extern "system" fn SetDatabaseInformation<Identity: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdbdirectory: *mut core::ffi::c_void, bstrlogdirectory: *mut core::ffi::c_void, bstrsharedfolder: *mut core::ffi::c_void, bforceoverwrite: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertSrvSetup_Impl::SetDatabaseInformation(this, core::mem::transmute(&bstrdbdirectory), core::mem::transmute(&bstrlogdirectory), core::mem::transmute(&bstrsharedfolder), core::mem::transmute_copy(&bforceoverwrite)).into()
            }
        }
        unsafe extern "system" fn SetParentCAInformation<Identity: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcaconfiguration: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertSrvSetup_Impl::SetParentCAInformation(this, core::mem::transmute(&bstrcaconfiguration)).into()
            }
        }
        unsafe extern "system" fn SetWebCAInformation<Identity: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcaconfiguration: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertSrvSetup_Impl::SetWebCAInformation(this, core::mem::transmute(&bstrcaconfiguration)).into()
            }
        }
        unsafe extern "system" fn Install<Identity: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertSrvSetup_Impl::Install(this).into()
            }
        }
        unsafe extern "system" fn PreUnInstall<Identity: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bclientonly: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertSrvSetup_Impl::PreUnInstall(this, core::mem::transmute_copy(&bclientonly)).into()
            }
        }
        unsafe extern "system" fn PostUnInstall<Identity: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertSrvSetup_Impl::PostUnInstall(this).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            CAErrorId: CAErrorId::<Identity, OFFSET>,
            CAErrorString: CAErrorString::<Identity, OFFSET>,
            InitializeDefaults: InitializeDefaults::<Identity, OFFSET>,
            GetCASetupProperty: GetCASetupProperty::<Identity, OFFSET>,
            SetCASetupProperty: SetCASetupProperty::<Identity, OFFSET>,
            IsPropertyEditable: IsPropertyEditable::<Identity, OFFSET>,
            GetSupportedCATypes: GetSupportedCATypes::<Identity, OFFSET>,
            GetProviderNameList: GetProviderNameList::<Identity, OFFSET>,
            GetKeyLengthList: GetKeyLengthList::<Identity, OFFSET>,
            GetHashAlgorithmList: GetHashAlgorithmList::<Identity, OFFSET>,
            GetPrivateKeyContainerList: GetPrivateKeyContainerList::<Identity, OFFSET>,
            GetExistingCACertificates: GetExistingCACertificates::<Identity, OFFSET>,
            CAImportPFX: CAImportPFX::<Identity, OFFSET>,
            SetCADistinguishedName: SetCADistinguishedName::<Identity, OFFSET>,
            SetDatabaseInformation: SetDatabaseInformation::<Identity, OFFSET>,
            SetParentCAInformation: SetParentCAInformation::<Identity, OFFSET>,
            SetWebCAInformation: SetWebCAInformation::<Identity, OFFSET>,
            Install: Install::<Identity, OFFSET>,
            PreUnInstall: PreUnInstall::<Identity, OFFSET>,
            PostUnInstall: PostUnInstall::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertSrvSetup as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICertSrvSetup {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICertSrvSetupKeyInformation, ICertSrvSetupKeyInformation_Vtbl, 0x6ba73778_36da_4c39_8a85_bcfa7d000793);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICertSrvSetupKeyInformation {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICertSrvSetupKeyInformation, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ICertSrvSetupKeyInformation {
    pub unsafe fn ProviderName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProviderName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetProviderName(&self, bstrval: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetProviderName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrval)).ok() }
    }
    pub unsafe fn Length(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Length)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetLength(&self, lval: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLength)(windows_core::Interface::as_raw(self), lval).ok() }
    }
    pub unsafe fn Existing(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Existing)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetExisting(&self, bval: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetExisting)(windows_core::Interface::as_raw(self), bval).ok() }
    }
    pub unsafe fn ContainerName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ContainerName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetContainerName(&self, bstrval: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetContainerName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrval)).ok() }
    }
    pub unsafe fn HashAlgorithm(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HashAlgorithm)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetHashAlgorithm(&self, bstrval: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetHashAlgorithm)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrval)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ExistingCACertificate(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExistingCACertificate)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetExistingCACertificate(&self, varval: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetExistingCACertificate)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varval)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ICertSrvSetupKeyInformation_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub ProviderName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetProviderName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Length: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetLength: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Existing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetExisting: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ContainerName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetContainerName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HashAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetHashAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ExistingCACertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ExistingCACertificate: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetExistingCACertificate: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetExistingCACertificate: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICertSrvSetupKeyInformation_Impl: super::super::System::Com::IDispatch_Impl {
    fn ProviderName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetProviderName(&self, bstrval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Length(&self) -> windows_core::Result<i32>;
    fn SetLength(&self, lval: i32) -> windows_core::Result<()>;
    fn Existing(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetExisting(&self, bval: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ContainerName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetContainerName(&self, bstrval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn HashAlgorithm(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetHashAlgorithm(&self, bstrval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ExistingCACertificate(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetExistingCACertificate(&self, varval: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICertSrvSetupKeyInformation_Vtbl {
    pub const fn new<Identity: ICertSrvSetupKeyInformation_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ProviderName<Identity: ICertSrvSetupKeyInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertSrvSetupKeyInformation_Impl::ProviderName(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetProviderName<Identity: ICertSrvSetupKeyInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertSrvSetupKeyInformation_Impl::SetProviderName(this, core::mem::transmute(&bstrval)).into()
            }
        }
        unsafe extern "system" fn Length<Identity: ICertSrvSetupKeyInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertSrvSetupKeyInformation_Impl::Length(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLength<Identity: ICertSrvSetupKeyInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lval: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertSrvSetupKeyInformation_Impl::SetLength(this, core::mem::transmute_copy(&lval)).into()
            }
        }
        unsafe extern "system" fn Existing<Identity: ICertSrvSetupKeyInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertSrvSetupKeyInformation_Impl::Existing(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetExisting<Identity: ICertSrvSetupKeyInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bval: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertSrvSetupKeyInformation_Impl::SetExisting(this, core::mem::transmute_copy(&bval)).into()
            }
        }
        unsafe extern "system" fn ContainerName<Identity: ICertSrvSetupKeyInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertSrvSetupKeyInformation_Impl::ContainerName(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetContainerName<Identity: ICertSrvSetupKeyInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertSrvSetupKeyInformation_Impl::SetContainerName(this, core::mem::transmute(&bstrval)).into()
            }
        }
        unsafe extern "system" fn HashAlgorithm<Identity: ICertSrvSetupKeyInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertSrvSetupKeyInformation_Impl::HashAlgorithm(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHashAlgorithm<Identity: ICertSrvSetupKeyInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertSrvSetupKeyInformation_Impl::SetHashAlgorithm(this, core::mem::transmute(&bstrval)).into()
            }
        }
        unsafe extern "system" fn ExistingCACertificate<Identity: ICertSrvSetupKeyInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertSrvSetupKeyInformation_Impl::ExistingCACertificate(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetExistingCACertificate<Identity: ICertSrvSetupKeyInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varval: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertSrvSetupKeyInformation_Impl::SetExistingCACertificate(this, core::mem::transmute(&varval)).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            ProviderName: ProviderName::<Identity, OFFSET>,
            SetProviderName: SetProviderName::<Identity, OFFSET>,
            Length: Length::<Identity, OFFSET>,
            SetLength: SetLength::<Identity, OFFSET>,
            Existing: Existing::<Identity, OFFSET>,
            SetExisting: SetExisting::<Identity, OFFSET>,
            ContainerName: ContainerName::<Identity, OFFSET>,
            SetContainerName: SetContainerName::<Identity, OFFSET>,
            HashAlgorithm: HashAlgorithm::<Identity, OFFSET>,
            SetHashAlgorithm: SetHashAlgorithm::<Identity, OFFSET>,
            ExistingCACertificate: ExistingCACertificate::<Identity, OFFSET>,
            SetExistingCACertificate: SetExistingCACertificate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertSrvSetupKeyInformation as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICertSrvSetupKeyInformation {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICertSrvSetupKeyInformationCollection, ICertSrvSetupKeyInformationCollection_Vtbl, 0xe65c8b00_e58f_41f9_a9ec_a28d7427c844);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICertSrvSetupKeyInformationCollection {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICertSrvSetupKeyInformationCollection, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ICertSrvSetupKeyInformationCollection {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, index: i32) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Add<P0>(&self, pikeyinformation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ICertSrvSetupKeyInformation>,
    {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), pikeyinformation.param().abi()).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ICertSrvSetupKeyInformationCollection_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICertSrvSetupKeyInformationCollection_Impl: super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Item(&self, index: i32) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn Add(&self, pikeyinformation: windows_core::Ref<ICertSrvSetupKeyInformation>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICertSrvSetupKeyInformationCollection_Vtbl {
    pub const fn new<Identity: ICertSrvSetupKeyInformationCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _NewEnum<Identity: ICertSrvSetupKeyInformationCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertSrvSetupKeyInformationCollection_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ppval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: ICertSrvSetupKeyInformationCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pval: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertSrvSetupKeyInformationCollection_Impl::get_Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: ICertSrvSetupKeyInformationCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertSrvSetupKeyInformationCollection_Impl::Count(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Add<Identity: ICertSrvSetupKeyInformationCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pikeyinformation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertSrvSetupKeyInformationCollection_Impl::Add(this, core::mem::transmute_copy(&pikeyinformation)).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertSrvSetupKeyInformationCollection as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICertSrvSetupKeyInformationCollection {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICertificateEnrollmentPolicyServerSetup, ICertificateEnrollmentPolicyServerSetup_Vtbl, 0x859252cc_238c_4a88_b8fd_a37e7d04e68b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICertificateEnrollmentPolicyServerSetup {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICertificateEnrollmentPolicyServerSetup, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ICertificateEnrollmentPolicyServerSetup {
    pub unsafe fn ErrorString(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ErrorString)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn InitializeInstallDefaults(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).InitializeInstallDefaults)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetProperty(&self, propertyid: CEPSetupProperty) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), propertyid, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetProperty(&self, propertyid: CEPSetupProperty, ppropertyvalue: *const super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), propertyid, core::mem::transmute(ppropertyvalue)).ok() }
    }
    pub unsafe fn Install(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Install)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn UnInstall(&self, pauthkeybasedrenewal: *const super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UnInstall)(windows_core::Interface::as_raw(self), core::mem::transmute(pauthkeybasedrenewal)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ICertificateEnrollmentPolicyServerSetup_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub ErrorString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InitializeInstallDefaults: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, CEPSetupProperty, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetProperty: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, CEPSetupProperty, *const super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetProperty: usize,
    pub Install: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub UnInstall: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    UnInstall: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICertificateEnrollmentPolicyServerSetup_Impl: super::super::System::Com::IDispatch_Impl {
    fn ErrorString(&self) -> windows_core::Result<windows_core::BSTR>;
    fn InitializeInstallDefaults(&self) -> windows_core::Result<()>;
    fn GetProperty(&self, propertyid: CEPSetupProperty) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetProperty(&self, propertyid: CEPSetupProperty, ppropertyvalue: *const super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn Install(&self) -> windows_core::Result<()>;
    fn UnInstall(&self, pauthkeybasedrenewal: *const super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICertificateEnrollmentPolicyServerSetup_Vtbl {
    pub const fn new<Identity: ICertificateEnrollmentPolicyServerSetup_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ErrorString<Identity: ICertificateEnrollmentPolicyServerSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertificateEnrollmentPolicyServerSetup_Impl::ErrorString(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InitializeInstallDefaults<Identity: ICertificateEnrollmentPolicyServerSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertificateEnrollmentPolicyServerSetup_Impl::InitializeInstallDefaults(this).into()
            }
        }
        unsafe extern "system" fn GetProperty<Identity: ICertificateEnrollmentPolicyServerSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: CEPSetupProperty, ppropertyvalue: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertificateEnrollmentPolicyServerSetup_Impl::GetProperty(this, core::mem::transmute_copy(&propertyid)) {
                    Ok(ok__) => {
                        ppropertyvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetProperty<Identity: ICertificateEnrollmentPolicyServerSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: CEPSetupProperty, ppropertyvalue: *const super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertificateEnrollmentPolicyServerSetup_Impl::SetProperty(this, core::mem::transmute_copy(&propertyid), core::mem::transmute_copy(&ppropertyvalue)).into()
            }
        }
        unsafe extern "system" fn Install<Identity: ICertificateEnrollmentPolicyServerSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertificateEnrollmentPolicyServerSetup_Impl::Install(this).into()
            }
        }
        unsafe extern "system" fn UnInstall<Identity: ICertificateEnrollmentPolicyServerSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pauthkeybasedrenewal: *const super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertificateEnrollmentPolicyServerSetup_Impl::UnInstall(this, core::mem::transmute_copy(&pauthkeybasedrenewal)).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            ErrorString: ErrorString::<Identity, OFFSET>,
            InitializeInstallDefaults: InitializeInstallDefaults::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            SetProperty: SetProperty::<Identity, OFFSET>,
            Install: Install::<Identity, OFFSET>,
            UnInstall: UnInstall::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertificateEnrollmentPolicyServerSetup as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICertificateEnrollmentPolicyServerSetup {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICertificateEnrollmentServerSetup, ICertificateEnrollmentServerSetup_Vtbl, 0x70027fdb_9dd9_4921_8944_b35cb31bd2ec);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICertificateEnrollmentServerSetup {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICertificateEnrollmentServerSetup, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ICertificateEnrollmentServerSetup {
    pub unsafe fn ErrorString(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ErrorString)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn InitializeInstallDefaults(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).InitializeInstallDefaults)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetProperty(&self, propertyid: CESSetupProperty) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), propertyid, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetProperty(&self, propertyid: CESSetupProperty, ppropertyvalue: *const super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), propertyid, core::mem::transmute(ppropertyvalue)).ok() }
    }
    pub unsafe fn SetApplicationPoolCredentials(&self, bstrusername: &windows_core::BSTR, bstrpassword: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetApplicationPoolCredentials)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrusername), core::mem::transmute_copy(bstrpassword)).ok() }
    }
    pub unsafe fn Install(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Install)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn UnInstall(&self, pcaconfig: *const super::super::System::Variant::VARIANT, pauthentication: *const super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UnInstall)(windows_core::Interface::as_raw(self), core::mem::transmute(pcaconfig), core::mem::transmute(pauthentication)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ICertificateEnrollmentServerSetup_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub ErrorString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InitializeInstallDefaults: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, CESSetupProperty, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetProperty: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, CESSetupProperty, *const super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetProperty: usize,
    pub SetApplicationPoolCredentials: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Install: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub UnInstall: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::System::Variant::VARIANT, *const super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    UnInstall: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICertificateEnrollmentServerSetup_Impl: super::super::System::Com::IDispatch_Impl {
    fn ErrorString(&self) -> windows_core::Result<windows_core::BSTR>;
    fn InitializeInstallDefaults(&self) -> windows_core::Result<()>;
    fn GetProperty(&self, propertyid: CESSetupProperty) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetProperty(&self, propertyid: CESSetupProperty, ppropertyvalue: *const super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn SetApplicationPoolCredentials(&self, bstrusername: &windows_core::BSTR, bstrpassword: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Install(&self) -> windows_core::Result<()>;
    fn UnInstall(&self, pcaconfig: *const super::super::System::Variant::VARIANT, pauthentication: *const super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICertificateEnrollmentServerSetup_Vtbl {
    pub const fn new<Identity: ICertificateEnrollmentServerSetup_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ErrorString<Identity: ICertificateEnrollmentServerSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertificateEnrollmentServerSetup_Impl::ErrorString(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InitializeInstallDefaults<Identity: ICertificateEnrollmentServerSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertificateEnrollmentServerSetup_Impl::InitializeInstallDefaults(this).into()
            }
        }
        unsafe extern "system" fn GetProperty<Identity: ICertificateEnrollmentServerSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: CESSetupProperty, ppropertyvalue: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertificateEnrollmentServerSetup_Impl::GetProperty(this, core::mem::transmute_copy(&propertyid)) {
                    Ok(ok__) => {
                        ppropertyvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetProperty<Identity: ICertificateEnrollmentServerSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: CESSetupProperty, ppropertyvalue: *const super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertificateEnrollmentServerSetup_Impl::SetProperty(this, core::mem::transmute_copy(&propertyid), core::mem::transmute_copy(&ppropertyvalue)).into()
            }
        }
        unsafe extern "system" fn SetApplicationPoolCredentials<Identity: ICertificateEnrollmentServerSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrusername: *mut core::ffi::c_void, bstrpassword: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertificateEnrollmentServerSetup_Impl::SetApplicationPoolCredentials(this, core::mem::transmute(&bstrusername), core::mem::transmute(&bstrpassword)).into()
            }
        }
        unsafe extern "system" fn Install<Identity: ICertificateEnrollmentServerSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertificateEnrollmentServerSetup_Impl::Install(this).into()
            }
        }
        unsafe extern "system" fn UnInstall<Identity: ICertificateEnrollmentServerSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcaconfig: *const super::super::System::Variant::VARIANT, pauthentication: *const super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertificateEnrollmentServerSetup_Impl::UnInstall(this, core::mem::transmute_copy(&pcaconfig), core::mem::transmute_copy(&pauthentication)).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            ErrorString: ErrorString::<Identity, OFFSET>,
            InitializeInstallDefaults: InitializeInstallDefaults::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            SetProperty: SetProperty::<Identity, OFFSET>,
            SetApplicationPoolCredentials: SetApplicationPoolCredentials::<Identity, OFFSET>,
            Install: Install::<Identity, OFFSET>,
            UnInstall: UnInstall::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertificateEnrollmentServerSetup as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICertificateEnrollmentServerSetup {}
pub const IFX_RSA_KEYGEN_VUL_AFFECTED_LEVEL_1: u32 = 1u32;
pub const IFX_RSA_KEYGEN_VUL_AFFECTED_LEVEL_2: u32 = 2u32;
pub const IFX_RSA_KEYGEN_VUL_NOT_AFFECTED: u32 = 0u32;
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMSCEPSetup, IMSCEPSetup_Vtbl, 0x4f7761bb_9f3b_4592_9ee0_9a73259c313e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMSCEPSetup {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMSCEPSetup, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IMSCEPSetup {
    pub unsafe fn MSCEPErrorId(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MSCEPErrorId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MSCEPErrorString(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MSCEPErrorString)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn InitializeDefaults(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).InitializeDefaults)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetMSCEPSetupProperty(&self, propertyid: MSCEPSetupProperty) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMSCEPSetupProperty)(windows_core::Interface::as_raw(self), propertyid, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetMSCEPSetupProperty(&self, propertyid: MSCEPSetupProperty, ppropertyvalue: *const super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMSCEPSetupProperty)(windows_core::Interface::as_raw(self), propertyid, core::mem::transmute(ppropertyvalue)).ok() }
    }
    pub unsafe fn SetAccountInformation(&self, bstrusername: &windows_core::BSTR, bstrpassword: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAccountInformation)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrusername), core::mem::transmute_copy(bstrpassword)).ok() }
    }
    pub unsafe fn IsMSCEPStoreEmpty(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsMSCEPStoreEmpty)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetProviderNameList(&self, bexchange: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProviderNameList)(windows_core::Interface::as_raw(self), bexchange, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetKeyLengthList(&self, bexchange: super::super::Foundation::VARIANT_BOOL, bstrprovidername: &windows_core::BSTR) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetKeyLengthList)(windows_core::Interface::as_raw(self), bexchange, core::mem::transmute_copy(bstrprovidername), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Install(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Install)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn PreUnInstall(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PreUnInstall)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn PostUnInstall(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PostUnInstall)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSCEPSetup_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub MSCEPErrorId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub MSCEPErrorString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InitializeDefaults: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetMSCEPSetupProperty: unsafe extern "system" fn(*mut core::ffi::c_void, MSCEPSetupProperty, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetMSCEPSetupProperty: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetMSCEPSetupProperty: unsafe extern "system" fn(*mut core::ffi::c_void, MSCEPSetupProperty, *const super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetMSCEPSetupProperty: usize,
    pub SetAccountInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsMSCEPStoreEmpty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetProviderNameList: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetProviderNameList: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetKeyLengthList: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL, *mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetKeyLengthList: usize,
    pub Install: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PreUnInstall: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PostUnInstall: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSCEPSetup_Impl: super::super::System::Com::IDispatch_Impl {
    fn MSCEPErrorId(&self) -> windows_core::Result<i32>;
    fn MSCEPErrorString(&self) -> windows_core::Result<windows_core::BSTR>;
    fn InitializeDefaults(&self) -> windows_core::Result<()>;
    fn GetMSCEPSetupProperty(&self, propertyid: MSCEPSetupProperty) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetMSCEPSetupProperty(&self, propertyid: MSCEPSetupProperty, ppropertyvalue: *const super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn SetAccountInformation(&self, bstrusername: &windows_core::BSTR, bstrpassword: &windows_core::BSTR) -> windows_core::Result<()>;
    fn IsMSCEPStoreEmpty(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetProviderNameList(&self, bexchange: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn GetKeyLengthList(&self, bexchange: super::super::Foundation::VARIANT_BOOL, bstrprovidername: &windows_core::BSTR) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Install(&self) -> windows_core::Result<()>;
    fn PreUnInstall(&self) -> windows_core::Result<()>;
    fn PostUnInstall(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMSCEPSetup_Vtbl {
    pub const fn new<Identity: IMSCEPSetup_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn MSCEPErrorId<Identity: IMSCEPSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSCEPSetup_Impl::MSCEPErrorId(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MSCEPErrorString<Identity: IMSCEPSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSCEPSetup_Impl::MSCEPErrorString(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InitializeDefaults<Identity: IMSCEPSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSCEPSetup_Impl::InitializeDefaults(this).into()
            }
        }
        unsafe extern "system" fn GetMSCEPSetupProperty<Identity: IMSCEPSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: MSCEPSetupProperty, pval: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSCEPSetup_Impl::GetMSCEPSetupProperty(this, core::mem::transmute_copy(&propertyid)) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMSCEPSetupProperty<Identity: IMSCEPSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: MSCEPSetupProperty, ppropertyvalue: *const super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSCEPSetup_Impl::SetMSCEPSetupProperty(this, core::mem::transmute_copy(&propertyid), core::mem::transmute_copy(&ppropertyvalue)).into()
            }
        }
        unsafe extern "system" fn SetAccountInformation<Identity: IMSCEPSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrusername: *mut core::ffi::c_void, bstrpassword: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSCEPSetup_Impl::SetAccountInformation(this, core::mem::transmute(&bstrusername), core::mem::transmute(&bstrpassword)).into()
            }
        }
        unsafe extern "system" fn IsMSCEPStoreEmpty<Identity: IMSCEPSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbempty: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSCEPSetup_Impl::IsMSCEPStoreEmpty(this) {
                    Ok(ok__) => {
                        pbempty.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetProviderNameList<Identity: IMSCEPSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bexchange: super::super::Foundation::VARIANT_BOOL, pval: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSCEPSetup_Impl::GetProviderNameList(this, core::mem::transmute_copy(&bexchange)) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetKeyLengthList<Identity: IMSCEPSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bexchange: super::super::Foundation::VARIANT_BOOL, bstrprovidername: *mut core::ffi::c_void, pval: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSCEPSetup_Impl::GetKeyLengthList(this, core::mem::transmute_copy(&bexchange), core::mem::transmute(&bstrprovidername)) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Install<Identity: IMSCEPSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSCEPSetup_Impl::Install(this).into()
            }
        }
        unsafe extern "system" fn PreUnInstall<Identity: IMSCEPSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSCEPSetup_Impl::PreUnInstall(this).into()
            }
        }
        unsafe extern "system" fn PostUnInstall<Identity: IMSCEPSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSCEPSetup_Impl::PostUnInstall(this).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            MSCEPErrorId: MSCEPErrorId::<Identity, OFFSET>,
            MSCEPErrorString: MSCEPErrorString::<Identity, OFFSET>,
            InitializeDefaults: InitializeDefaults::<Identity, OFFSET>,
            GetMSCEPSetupProperty: GetMSCEPSetupProperty::<Identity, OFFSET>,
            SetMSCEPSetupProperty: SetMSCEPSetupProperty::<Identity, OFFSET>,
            SetAccountInformation: SetAccountInformation::<Identity, OFFSET>,
            IsMSCEPStoreEmpty: IsMSCEPStoreEmpty::<Identity, OFFSET>,
            GetProviderNameList: GetProviderNameList::<Identity, OFFSET>,
            GetKeyLengthList: GetKeyLengthList::<Identity, OFFSET>,
            Install: Install::<Identity, OFFSET>,
            PreUnInstall: PreUnInstall::<Identity, OFFSET>,
            PostUnInstall: PostUnInstall::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSCEPSetup as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMSCEPSetup {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct INFORMATIONCARD_ASYMMETRIC_CRYPTO_PARAMETERS {
    pub keySize: i32,
    pub keyExchangeAlgorithm: windows_core::PWSTR,
    pub signatureAlgorithm: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct INFORMATIONCARD_CRYPTO_HANDLE {
    pub r#type: HandleType,
    pub expiration: i64,
    pub cryptoParameters: *mut core::ffi::c_void,
}
impl Default for INFORMATIONCARD_CRYPTO_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct INFORMATIONCARD_HASH_CRYPTO_PARAMETERS {
    pub hashSize: i32,
    pub transform: INFORMATIONCARD_TRANSFORM_CRYPTO_PARAMETERS,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct INFORMATIONCARD_SYMMETRIC_CRYPTO_PARAMETERS {
    pub keySize: i32,
    pub blockSize: i32,
    pub feedbackSize: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct INFORMATIONCARD_TRANSFORM_CRYPTO_PARAMETERS {
    pub inputBlockSize: i32,
    pub outputBlockSize: i32,
    pub canTransformMultipleBlocks: windows_core::BOOL,
    pub canReuseTransform: windows_core::BOOL,
}
pub const INTERNATIONAL_USAGE: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InFileSignatureResource {
    pub dwVersion: u32,
    pub dwCrcOffset: u32,
    pub rgbSignature: [u8; 88],
}
impl Default for InFileSignatureResource {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const InvalidAc: CARD_FILE_ACCESS_CONDITION = CARD_FILE_ACCESS_CONDITION(0i32);
pub const InvalidDirAc: CARD_DIRECTORY_ACCESS_CONDITION = CARD_DIRECTORY_ACCESS_CONDITION(0i32);
pub const KDF_ALGORITHMID: u32 = 8u32;
pub const KDF_CONTEXT: u32 = 14u32;
pub const KDF_GENERIC_PARAMETER: u32 = 17u32;
pub const KDF_HASH_ALGORITHM: u32 = 0u32;
pub const KDF_HKDF_INFO: u32 = 20u32;
pub const KDF_HKDF_SALT: u32 = 19u32;
pub const KDF_HMAC_KEY: u32 = 3u32;
pub const KDF_ITERATION_COUNT: u32 = 16u32;
pub const KDF_KEYBITLENGTH: u32 = 18u32;
pub const KDF_LABEL: u32 = 13u32;
pub const KDF_PARTYUINFO: u32 = 9u32;
pub const KDF_PARTYVINFO: u32 = 10u32;
pub const KDF_SALT: u32 = 15u32;
pub const KDF_SECRET_APPEND: u32 = 2u32;
pub const KDF_SECRET_HANDLE: u32 = 6u32;
pub const KDF_SECRET_PREPEND: u32 = 1u32;
pub const KDF_SUPPPRIVINFO: u32 = 12u32;
pub const KDF_SUPPPUBINFO: u32 = 11u32;
pub const KDF_TLS_PRF_LABEL: u32 = 4u32;
pub const KDF_TLS_PRF_PROTOCOL: u32 = 7u32;
pub const KDF_TLS_PRF_SEED: u32 = 5u32;
pub const KDF_USE_SECRET_AS_HMAC_KEY_FLAG: u32 = 1u32;
pub const KEYSTATEBLOB: u32 = 12u32;
pub const KEY_LENGTH_MASK: u32 = 4294901760u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KEY_TYPE_SUBTYPE {
    pub dwKeySpec: u32,
    pub Type: windows_core::GUID,
    pub Subtype: windows_core::GUID,
}
pub const KP_ADMIN_PIN: u32 = 31u32;
pub const KP_ALGID: CRYPT_KEY_PARAM_ID = CRYPT_KEY_PARAM_ID(7u32);
pub const KP_BLOCKLEN: CRYPT_KEY_PARAM_ID = CRYPT_KEY_PARAM_ID(8u32);
pub const KP_CERTIFICATE: CRYPT_KEY_PARAM_ID = CRYPT_KEY_PARAM_ID(26u32);
pub const KP_CLEAR_KEY: u32 = 27u32;
pub const KP_CLIENT_RANDOM: u32 = 21u32;
pub const KP_CMS_DH_KEY_INFO: u32 = 38u32;
pub const KP_CMS_KEY_INFO: u32 = 37u32;
pub const KP_EFFECTIVE_KEYLEN: u32 = 19u32;
pub const KP_G: u32 = 12u32;
pub const KP_GET_USE_COUNT: CRYPT_KEY_PARAM_ID = CRYPT_KEY_PARAM_ID(42u32);
pub const KP_HIGHEST_VERSION: u32 = 41u32;
pub const KP_INFO: u32 = 18u32;
pub const KP_IV: u32 = 1u32;
pub const KP_KEYEXCHANGE_PIN: u32 = 32u32;
pub const KP_KEYLEN: CRYPT_KEY_PARAM_ID = CRYPT_KEY_PARAM_ID(9u32);
pub const KP_KEYVAL: u32 = 30u32;
pub const KP_MODE: u32 = 4u32;
pub const KP_MODE_BITS: u32 = 5u32;
pub const KP_OAEP_PARAMS: u32 = 36u32;
pub const KP_P: u32 = 11u32;
pub const KP_PADDING: u32 = 3u32;
pub const KP_PERMISSIONS: CRYPT_KEY_PARAM_ID = CRYPT_KEY_PARAM_ID(6u32);
pub const KP_PIN_ID: u32 = 43u32;
pub const KP_PIN_INFO: u32 = 44u32;
pub const KP_PRECOMP_MD5: u32 = 24u32;
pub const KP_PRECOMP_SHA: u32 = 25u32;
pub const KP_PREHASH: u32 = 34u32;
pub const KP_PUB_EX_LEN: u32 = 28u32;
pub const KP_PUB_EX_VAL: u32 = 29u32;
pub const KP_PUB_PARAMS: u32 = 39u32;
pub const KP_Q: u32 = 13u32;
pub const KP_RA: u32 = 16u32;
pub const KP_RB: u32 = 17u32;
pub const KP_ROUNDS: u32 = 35u32;
pub const KP_RP: u32 = 23u32;
pub const KP_SALT: CRYPT_KEY_PARAM_ID = CRYPT_KEY_PARAM_ID(2u32);
pub const KP_SALT_EX: CRYPT_KEY_PARAM_ID = CRYPT_KEY_PARAM_ID(10u32);
pub const KP_SCHANNEL_ALG: u32 = 20u32;
pub const KP_SERVER_RANDOM: u32 = 22u32;
pub const KP_SIGNATURE_PIN: u32 = 33u32;
pub const KP_VERIFY_PARAMS: u32 = 40u32;
pub const KP_X: u32 = 14u32;
pub const KP_Y: u32 = 15u32;
pub const KeyTypeHardware: CertKeyType = CertKeyType(6u32);
pub const KeyTypeOther: CertKeyType = CertKeyType(0u32);
pub const KeyTypePassport: CertKeyType = CertKeyType(3u32);
pub const KeyTypePassportRemote: CertKeyType = CertKeyType(4u32);
pub const KeyTypePassportSmartCard: CertKeyType = CertKeyType(5u32);
pub const KeyTypePhysicalSmartCard: CertKeyType = CertKeyType(2u32);
pub const KeyTypeSelfSigned: CertKeyType = CertKeyType(8u32);
pub const KeyTypeSoftware: CertKeyType = CertKeyType(7u32);
pub const KeyTypeVirtualSmartCard: CertKeyType = CertKeyType(1u32);
pub const LEGACY_DH_PRIVATE_BLOB: windows_core::PCWSTR = windows_core::w!("CAPIDHPRIVATEBLOB");
pub const LEGACY_DH_PUBLIC_BLOB: windows_core::PCWSTR = windows_core::w!("CAPIDHPUBLICBLOB");
pub const LEGACY_DSA_PRIVATE_BLOB: windows_core::PCWSTR = windows_core::w!("CAPIDSAPRIVATEBLOB");
pub const LEGACY_DSA_PUBLIC_BLOB: windows_core::PCWSTR = windows_core::w!("CAPIDSAPUBLICBLOB");
pub const LEGACY_DSA_V2_PRIVATE_BLOB: windows_core::PCWSTR = windows_core::w!("V2CAPIDSAPRIVATEBLOB");
pub const LEGACY_DSA_V2_PUBLIC_BLOB: windows_core::PCWSTR = windows_core::w!("V2CAPIDSAPUBLICBLOB");
pub const LEGACY_RSAPRIVATE_BLOB: windows_core::PCWSTR = windows_core::w!("CAPIPRIVATEBLOB");
pub const LEGACY_RSAPUBLIC_BLOB: windows_core::PCWSTR = windows_core::w!("CAPIPUBLICBLOB");
pub const MAXUIDLEN: u32 = 64u32;
pub const MAX_CONTAINER_NAME_LEN: u32 = 39u32;
pub const MAX_PINS: u32 = 8u32;
pub const MICROSOFT_ROOT_CERT_CHAIN_POLICY_CHECK_APPLICATION_ROOT_FLAG: CERT_CHAIN_POLICY_FLAGS = CERT_CHAIN_POLICY_FLAGS(131072u32);
pub const MICROSOFT_ROOT_CERT_CHAIN_POLICY_DISABLE_FLIGHT_ROOT_FLAG: CERT_CHAIN_POLICY_FLAGS = CERT_CHAIN_POLICY_FLAGS(262144u32);
pub const MICROSOFT_ROOT_CERT_CHAIN_POLICY_ENABLE_TEST_ROOT_FLAG: CERT_CHAIN_POLICY_FLAGS = CERT_CHAIN_POLICY_FLAGS(65536u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MSCEPSetupProperty(pub i32);
pub const MSCRYPT_ECC_MAX_CURVE_NAME_LENGTH: u32 = 255u32;
pub const MSCRYPT_ECC_MAX_OID_LENGTH: u32 = 255u32;
pub const MS_DEF_DH_SCHANNEL_PROV: windows_core::PCWSTR = windows_core::w!("Microsoft DH SChannel Cryptographic Provider");
pub const MS_DEF_DH_SCHANNEL_PROV_A: windows_core::PCSTR = windows_core::s!("Microsoft DH SChannel Cryptographic Provider");
pub const MS_DEF_DH_SCHANNEL_PROV_W: windows_core::PCWSTR = windows_core::w!("Microsoft DH SChannel Cryptographic Provider");
pub const MS_DEF_DSS_DH_PROV: windows_core::PCWSTR = windows_core::w!("Microsoft Base DSS and Diffie-Hellman Cryptographic Provider");
pub const MS_DEF_DSS_DH_PROV_A: windows_core::PCSTR = windows_core::s!("Microsoft Base DSS and Diffie-Hellman Cryptographic Provider");
pub const MS_DEF_DSS_DH_PROV_W: windows_core::PCWSTR = windows_core::w!("Microsoft Base DSS and Diffie-Hellman Cryptographic Provider");
pub const MS_DEF_DSS_PROV: windows_core::PCWSTR = windows_core::w!("Microsoft Base DSS Cryptographic Provider");
pub const MS_DEF_DSS_PROV_A: windows_core::PCSTR = windows_core::s!("Microsoft Base DSS Cryptographic Provider");
pub const MS_DEF_DSS_PROV_W: windows_core::PCWSTR = windows_core::w!("Microsoft Base DSS Cryptographic Provider");
pub const MS_DEF_PROV: windows_core::PCWSTR = windows_core::w!("Microsoft Base Cryptographic Provider v1.0");
pub const MS_DEF_PROV_A: windows_core::PCSTR = windows_core::s!("Microsoft Base Cryptographic Provider v1.0");
pub const MS_DEF_PROV_W: windows_core::PCWSTR = windows_core::w!("Microsoft Base Cryptographic Provider v1.0");
pub const MS_DEF_RSA_SCHANNEL_PROV: windows_core::PCWSTR = windows_core::w!("Microsoft RSA SChannel Cryptographic Provider");
pub const MS_DEF_RSA_SCHANNEL_PROV_A: windows_core::PCSTR = windows_core::s!("Microsoft RSA SChannel Cryptographic Provider");
pub const MS_DEF_RSA_SCHANNEL_PROV_W: windows_core::PCWSTR = windows_core::w!("Microsoft RSA SChannel Cryptographic Provider");
pub const MS_DEF_RSA_SIG_PROV: windows_core::PCWSTR = windows_core::w!("Microsoft RSA Signature Cryptographic Provider");
pub const MS_DEF_RSA_SIG_PROV_A: windows_core::PCSTR = windows_core::s!("Microsoft RSA Signature Cryptographic Provider");
pub const MS_DEF_RSA_SIG_PROV_W: windows_core::PCWSTR = windows_core::w!("Microsoft RSA Signature Cryptographic Provider");
pub const MS_ENHANCED_PROV: windows_core::PCWSTR = windows_core::w!("Microsoft Enhanced Cryptographic Provider v1.0");
pub const MS_ENHANCED_PROV_A: windows_core::PCSTR = windows_core::s!("Microsoft Enhanced Cryptographic Provider v1.0");
pub const MS_ENHANCED_PROV_W: windows_core::PCWSTR = windows_core::w!("Microsoft Enhanced Cryptographic Provider v1.0");
pub const MS_ENH_DSS_DH_PROV: windows_core::PCWSTR = windows_core::w!("Microsoft Enhanced DSS and Diffie-Hellman Cryptographic Provider");
pub const MS_ENH_DSS_DH_PROV_A: windows_core::PCSTR = windows_core::s!("Microsoft Enhanced DSS and Diffie-Hellman Cryptographic Provider");
pub const MS_ENH_DSS_DH_PROV_W: windows_core::PCWSTR = windows_core::w!("Microsoft Enhanced DSS and Diffie-Hellman Cryptographic Provider");
pub const MS_ENH_RSA_AES_PROV: windows_core::PCWSTR = windows_core::w!("Microsoft Enhanced RSA and AES Cryptographic Provider");
pub const MS_ENH_RSA_AES_PROV_A: windows_core::PCSTR = windows_core::s!("Microsoft Enhanced RSA and AES Cryptographic Provider");
pub const MS_ENH_RSA_AES_PROV_W: windows_core::PCWSTR = windows_core::w!("Microsoft Enhanced RSA and AES Cryptographic Provider");
pub const MS_ENH_RSA_AES_PROV_XP: windows_core::PCWSTR = windows_core::w!("Microsoft Enhanced RSA and AES Cryptographic Provider (Prototype)");
pub const MS_ENH_RSA_AES_PROV_XP_A: windows_core::PCSTR = windows_core::s!("Microsoft Enhanced RSA and AES Cryptographic Provider (Prototype)");
pub const MS_ENH_RSA_AES_PROV_XP_W: windows_core::PCWSTR = windows_core::w!("Microsoft Enhanced RSA and AES Cryptographic Provider (Prototype)");
pub const MS_KEY_PROTECTION_PROVIDER: windows_core::PCWSTR = windows_core::w!("Microsoft Key Protection Provider");
pub const MS_KEY_STORAGE_PROVIDER: windows_core::PCWSTR = windows_core::w!("Microsoft Software Key Storage Provider");
pub const MS_NGC_KEY_STORAGE_PROVIDER: windows_core::PCWSTR = windows_core::w!("Microsoft Passport Key Storage Provider");
pub const MS_PLATFORM_CRYPTO_PROVIDER: windows_core::PCWSTR = windows_core::w!("Microsoft Platform Crypto Provider");
pub const MS_PLATFORM_KEY_STORAGE_PROVIDER: windows_core::PCWSTR = windows_core::w!("Microsoft Platform Crypto Provider");
pub const MS_PRIMITIVE_PROVIDER: windows_core::PCWSTR = windows_core::w!("Microsoft Primitive Provider");
pub const MS_SCARD_PROV: windows_core::PCWSTR = windows_core::w!("Microsoft Base Smart Card Crypto Provider");
pub const MS_SCARD_PROV_A: windows_core::PCSTR = windows_core::s!("Microsoft Base Smart Card Crypto Provider");
pub const MS_SCARD_PROV_W: windows_core::PCWSTR = windows_core::w!("Microsoft Base Smart Card Crypto Provider");
pub const MS_SCHANNEL_PROVIDER: windows_core::PCWSTR = windows_core::w!("Microsoft SSL Protocol Provider");
pub const MS_SMART_CARD_KEY_STORAGE_PROVIDER: windows_core::PCWSTR = windows_core::w!("Microsoft Smart Card Key Storage Provider");
pub const MS_STRONG_PROV: windows_core::PCWSTR = windows_core::w!("Microsoft Strong Cryptographic Provider");
pub const MS_STRONG_PROV_A: windows_core::PCSTR = windows_core::s!("Microsoft Strong Cryptographic Provider");
pub const MS_STRONG_PROV_W: windows_core::PCWSTR = windows_core::w!("Microsoft Strong Cryptographic Provider");
pub const NCRYPTBUFFER_ATTESTATIONSTATEMENT_BLOB: u32 = 51u32;
pub const NCRYPTBUFFER_ATTESTATION_CLAIM_CHALLENGE_REQUIRED: u32 = 53u32;
pub const NCRYPTBUFFER_ATTESTATION_CLAIM_TYPE: u32 = 52u32;
pub const NCRYPTBUFFER_CERT_BLOB: u32 = 47u32;
pub const NCRYPTBUFFER_CLAIM_IDBINDING_NONCE: u32 = 48u32;
pub const NCRYPTBUFFER_CLAIM_KEYATTESTATION_NONCE: u32 = 49u32;
pub const NCRYPTBUFFER_DATA: u32 = 1u32;
pub const NCRYPTBUFFER_ECC_CURVE_NAME: u32 = 60u32;
pub const NCRYPTBUFFER_ECC_PARAMETERS: u32 = 61u32;
pub const NCRYPTBUFFER_EMPTY: u32 = 0u32;
pub const NCRYPTBUFFER_KEY_PROPERTY_FLAGS: u32 = 50u32;
pub const NCRYPTBUFFER_PKCS_ALG_ID: u32 = 43u32;
pub const NCRYPTBUFFER_PKCS_ALG_OID: u32 = 41u32;
pub const NCRYPTBUFFER_PKCS_ALG_PARAM: u32 = 42u32;
pub const NCRYPTBUFFER_PKCS_ATTRS: u32 = 44u32;
pub const NCRYPTBUFFER_PKCS_KEY_NAME: u32 = 45u32;
pub const NCRYPTBUFFER_PKCS_OID: u32 = 40u32;
pub const NCRYPTBUFFER_PKCS_SECRET: u32 = 46u32;
pub const NCRYPTBUFFER_PROTECTION_DESCRIPTOR_STRING: u32 = 3u32;
pub const NCRYPTBUFFER_PROTECTION_FLAGS: u32 = 4u32;
pub const NCRYPTBUFFER_SSL_CLEAR_KEY: u32 = 23u32;
pub const NCRYPTBUFFER_SSL_CLIENT_RANDOM: u32 = 20u32;
pub const NCRYPTBUFFER_SSL_HIGHEST_VERSION: u32 = 22u32;
pub const NCRYPTBUFFER_SSL_KEY_ARG_DATA: u32 = 24u32;
pub const NCRYPTBUFFER_SSL_SERVER_RANDOM: u32 = 21u32;
pub const NCRYPTBUFFER_SSL_SESSION_HASH: u32 = 25u32;
pub const NCRYPTBUFFER_TPM_PLATFORM_CLAIM_NONCE: u32 = 81u32;
pub const NCRYPTBUFFER_TPM_PLATFORM_CLAIM_PCR_MASK: u32 = 80u32;
pub const NCRYPTBUFFER_TPM_PLATFORM_CLAIM_STATIC_CREATE: u32 = 82u32;
pub const NCRYPTBUFFER_TPM_SEAL_NO_DA_PROTECTION: u32 = 73u32;
pub const NCRYPTBUFFER_TPM_SEAL_PASSWORD: u32 = 70u32;
pub const NCRYPTBUFFER_TPM_SEAL_POLICYINFO: u32 = 71u32;
pub const NCRYPTBUFFER_TPM_SEAL_TICKET: u32 = 72u32;
pub const NCRYPTBUFFER_VERSION: u32 = 0u32;
pub const NCRYPTBUFFER_VSM_KEY_ATTESTATION_CLAIM_RESTRICTIONS: u32 = 54u32;
pub const NCRYPT_3DES_112_ALGORITHM: windows_core::PCWSTR = windows_core::w!("3DES_112");
pub const NCRYPT_3DES_ALGORITHM: windows_core::PCWSTR = windows_core::w!("3DES");
pub const NCRYPT_AES_ALGORITHM: windows_core::PCWSTR = windows_core::w!("AES");
pub const NCRYPT_AES_ALGORITHM_GROUP: windows_core::PCWSTR = windows_core::w!("AES");
pub const NCRYPT_ALGORITHM_GROUP_PROPERTY: windows_core::PCWSTR = windows_core::w!("Algorithm Group");
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NCRYPT_ALGORITHM_NAME_CLASS(pub u32);
pub const NCRYPT_ALGORITHM_PROPERTY: windows_core::PCWSTR = windows_core::w!("Algorithm Name");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct NCRYPT_ALLOC_PARA {
    pub cbSize: u32,
    pub pfnAlloc: PFN_NCRYPT_ALLOC,
    pub pfnFree: PFN_NCRYPT_FREE,
}
pub const NCRYPT_ALLOW_ALL_USAGES: u32 = 16777215u32;
pub const NCRYPT_ALLOW_ARCHIVING_FLAG: u32 = 4u32;
pub const NCRYPT_ALLOW_DECRYPT_FLAG: u32 = 1u32;
pub const NCRYPT_ALLOW_EXPORT_FLAG: u32 = 1u32;
pub const NCRYPT_ALLOW_KEY_AGREEMENT_FLAG: u32 = 4u32;
pub const NCRYPT_ALLOW_KEY_IMPORT_FLAG: u32 = 8u32;
pub const NCRYPT_ALLOW_PLAINTEXT_ARCHIVING_FLAG: u32 = 8u32;
pub const NCRYPT_ALLOW_PLAINTEXT_EXPORT_FLAG: u32 = 2u32;
pub const NCRYPT_ALLOW_SIGNING_FLAG: u32 = 2u32;
pub const NCRYPT_ALLOW_SILENT_KEY_ACCESS: u32 = 1u32;
pub const NCRYPT_ALTERNATE_KEY_STORAGE_LOCATION_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_ALTERNATE_KEY_STORAGE_LOCATION");
pub const NCRYPT_ASSOCIATED_ECDH_KEY: windows_core::PCWSTR = windows_core::w!("SmartCardAssociatedECDHKey");
pub const NCRYPT_ASYMMETRIC_ENCRYPTION_INTERFACE: NCRYPT_ALGORITHM_NAME_CLASS = NCRYPT_ALGORITHM_NAME_CLASS(3u32);
pub const NCRYPT_ASYMMETRIC_ENCRYPTION_OPERATION: NCRYPT_OPERATION = NCRYPT_OPERATION(4u32);
pub const NCRYPT_ATTESTATION_FLAG: u32 = 32u32;
pub const NCRYPT_AUTHORITY_KEY_FLAG: u32 = 256u32;
pub const NCRYPT_AUTH_TAG_LENGTH: windows_core::PCWSTR = windows_core::w!("AuthTagLength");
pub const NCRYPT_BLOCK_LENGTH_PROPERTY: windows_core::PCWSTR = windows_core::w!("Block Length");
pub const NCRYPT_CAPI_KDF_ALGORITHM: windows_core::PCWSTR = windows_core::w!("CAPI_KDF");
pub const NCRYPT_CERTIFICATE_PROPERTY: windows_core::PCWSTR = windows_core::w!("SmartCardKeyCertificate");
pub const NCRYPT_CHAINING_MODE_PROPERTY: windows_core::PCWSTR = windows_core::w!("Chaining Mode");
pub const NCRYPT_CHANGEPASSWORD_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_CHANGEPASSWORD");
pub const NCRYPT_CIPHER_BLOCK_PADDING_FLAG: u32 = 1u32;
pub const NCRYPT_CIPHER_KEY_BLOB: windows_core::PCWSTR = windows_core::w!("CipherKeyBlob");
pub const NCRYPT_CIPHER_KEY_BLOB_MAGIC: u32 = 1380470851u32;
pub const NCRYPT_CIPHER_NO_PADDING_FLAG: u32 = 0u32;
pub const NCRYPT_CIPHER_OPERATION: NCRYPT_OPERATION = NCRYPT_OPERATION(1u32);
pub const NCRYPT_CIPHER_OTHER_PADDING_FLAG: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NCRYPT_CIPHER_PADDING_INFO {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub pbIV: *mut u8,
    pub cbIV: u32,
    pub pbOtherInfo: *mut u8,
    pub cbOtherInfo: u32,
}
impl Default for NCRYPT_CIPHER_PADDING_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NCRYPT_CLAIM_AUTHORITY_AND_SUBJECT: u32 = 3u32;
pub const NCRYPT_CLAIM_AUTHORITY_ONLY: u32 = 1u32;
pub const NCRYPT_CLAIM_PLATFORM: u32 = 65536u32;
pub const NCRYPT_CLAIM_SUBJECT_ONLY: u32 = 2u32;
pub const NCRYPT_CLAIM_UNKNOWN: u32 = 4096u32;
pub const NCRYPT_CLAIM_VSM_KEY_ATTESTATION_STATEMENT: u32 = 4u32;
pub const NCRYPT_CLAIM_WEB_AUTH_SUBJECT_ONLY: u32 = 258u32;
pub const NCRYPT_DESCR_DELIMITER_AND: windows_core::PCWSTR = windows_core::w!("AND");
pub const NCRYPT_DESCR_DELIMITER_OR: windows_core::PCWSTR = windows_core::w!("OR");
pub const NCRYPT_DESCR_EQUAL: windows_core::PCWSTR = windows_core::w!("=");
pub const NCRYPT_DESX_ALGORITHM: windows_core::PCWSTR = windows_core::w!("DESX");
pub const NCRYPT_DES_ALGORITHM: windows_core::PCWSTR = windows_core::w!("DES");
pub const NCRYPT_DES_ALGORITHM_GROUP: windows_core::PCWSTR = windows_core::w!("DES");
pub const NCRYPT_DH_ALGORITHM: windows_core::PCWSTR = windows_core::w!("DH");
pub const NCRYPT_DH_ALGORITHM_GROUP: windows_core::PCWSTR = windows_core::w!("DH");
pub const NCRYPT_DH_PARAMETERS_PROPERTY: windows_core::PCWSTR = windows_core::w!("DHParameters");
pub const NCRYPT_DISMISS_UI_TIMEOUT_SEC_PROPERTY: windows_core::PCWSTR = windows_core::w!("SmartCardDismissUITimeoutSeconds");
pub const NCRYPT_DO_NOT_FINALIZE_FLAG: u32 = 1024u32;
pub const NCRYPT_DSA_ALGORITHM: windows_core::PCWSTR = windows_core::w!("DSA");
pub const NCRYPT_DSA_ALGORITHM_GROUP: windows_core::PCWSTR = windows_core::w!("DSA");
pub const NCRYPT_ECC_CURVE_NAME_LIST_PROPERTY: windows_core::PCWSTR = windows_core::w!("ECCCurveNameList");
pub const NCRYPT_ECC_CURVE_NAME_PROPERTY: windows_core::PCWSTR = windows_core::w!("ECCCurveName");
pub const NCRYPT_ECC_PARAMETERS_PROPERTY: windows_core::PCWSTR = windows_core::w!("ECCParameters");
pub const NCRYPT_ECDH_ALGORITHM: windows_core::PCWSTR = windows_core::w!("ECDH");
pub const NCRYPT_ECDH_ALGORITHM_GROUP: windows_core::PCWSTR = windows_core::w!("ECDH");
pub const NCRYPT_ECDH_P256_ALGORITHM: windows_core::PCWSTR = windows_core::w!("ECDH_P256");
pub const NCRYPT_ECDH_P384_ALGORITHM: windows_core::PCWSTR = windows_core::w!("ECDH_P384");
pub const NCRYPT_ECDH_P521_ALGORITHM: windows_core::PCWSTR = windows_core::w!("ECDH_P521");
pub const NCRYPT_ECDSA_ALGORITHM: windows_core::PCWSTR = windows_core::w!("ECDSA");
pub const NCRYPT_ECDSA_ALGORITHM_GROUP: windows_core::PCWSTR = windows_core::w!("ECDSA");
pub const NCRYPT_ECDSA_P256_ALGORITHM: windows_core::PCWSTR = windows_core::w!("ECDSA_P256");
pub const NCRYPT_ECDSA_P384_ALGORITHM: windows_core::PCWSTR = windows_core::w!("ECDSA_P384");
pub const NCRYPT_ECDSA_P521_ALGORITHM: windows_core::PCWSTR = windows_core::w!("ECDSA_P521");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NCRYPT_EXPORTED_ISOLATED_KEY_ENVELOPE {
    pub Header: NCRYPT_EXPORTED_ISOLATED_KEY_HEADER,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NCRYPT_EXPORTED_ISOLATED_KEY_HEADER {
    pub Version: u32,
    pub KeyUsage: u32,
    pub _bitfield: u32,
    pub cbAlgName: u32,
    pub cbNonce: u32,
    pub cbAuthTag: u32,
    pub cbWrappingKey: u32,
    pub cbIsolatedKey: u32,
}
pub const NCRYPT_EXPORTED_ISOLATED_KEY_HEADER_CURRENT_VERSION: u32 = 0u32;
pub const NCRYPT_EXPORTED_ISOLATED_KEY_HEADER_V0: u32 = 0u32;
pub const NCRYPT_EXPORT_LEGACY_FLAG: u32 = 2048u32;
pub const NCRYPT_EXPORT_POLICY_PROPERTY: windows_core::PCWSTR = windows_core::w!("Export Policy");
pub const NCRYPT_EXTENDED_ERRORS_FLAG: u32 = 268435456u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NCRYPT_FLAGS(pub u32);
impl NCRYPT_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for NCRYPT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for NCRYPT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for NCRYPT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for NCRYPT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for NCRYPT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct NCRYPT_HANDLE(pub usize);
impl NCRYPT_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == 0
    }
}
impl windows_core::Free for NCRYPT_HANDLE {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_core::link!("ncrypt.dll" "system" fn NCryptFreeObject(hobject : usize) -> i32);
            unsafe {
                NCryptFreeObject(self.0);
            }
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct NCRYPT_HASH_HANDLE(pub usize);
impl NCRYPT_HASH_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == 0
    }
}
pub const NCRYPT_HASH_OPERATION: NCRYPT_OPERATION = NCRYPT_OPERATION(2u32);
pub const NCRYPT_HMAC_SHA256_ALGORITHM: windows_core::PCWSTR = windows_core::w!("HMAC-SHA256");
pub const NCRYPT_IGNORE_DEVICE_STATE_FLAG: u32 = 4096u32;
pub const NCRYPT_IMPL_HARDWARE_FLAG: u32 = 1u32;
pub const NCRYPT_IMPL_HARDWARE_RNG_FLAG: u32 = 16u32;
pub const NCRYPT_IMPL_REMOVABLE_FLAG: u32 = 8u32;
pub const NCRYPT_IMPL_SOFTWARE_FLAG: u32 = 2u32;
pub const NCRYPT_IMPL_TYPE_PROPERTY: windows_core::PCWSTR = windows_core::w!("Impl Type");
pub const NCRYPT_IMPL_VIRTUAL_ISOLATION_FLAG: u32 = 32u32;
pub const NCRYPT_INITIALIZATION_VECTOR: windows_core::PCWSTR = windows_core::w!("IV");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NCRYPT_ISOLATED_KEY_ATTESTED_ATTRIBUTES {
    pub Version: u32,
    pub Flags: u32,
    pub cbPublicKeyBlob: u32,
}
pub const NCRYPT_ISOLATED_KEY_ATTESTED_ATTRIBUTES_CURRENT_VERSION: u32 = 0u32;
pub const NCRYPT_ISOLATED_KEY_ATTESTED_ATTRIBUTES_V0: u32 = 0u32;
pub const NCRYPT_ISOLATED_KEY_ENVELOPE_BLOB: windows_core::PCWSTR = windows_core::w!("ISOLATED_KEY_ENVELOPE");
pub const NCRYPT_ISOLATED_KEY_FLAG_CREATED_IN_ISOLATION: u32 = 1u32;
pub const NCRYPT_ISOLATED_KEY_FLAG_IMPORT_ONLY: u32 = 2u32;
pub const NCRYPT_KDF_KEY_BLOB: windows_core::PCWSTR = windows_core::w!("KDFKeyBlob");
pub const NCRYPT_KDF_KEY_BLOB_MAGIC: u32 = 826688587u32;
pub const NCRYPT_KDF_SECRET_VALUE: windows_core::PCWSTR = windows_core::w!("KDFKeySecret");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NCRYPT_KEY_ACCESS_POLICY_BLOB {
    pub dwVersion: u32,
    pub dwPolicyFlags: u32,
    pub cbUserSid: u32,
    pub cbApplicationSid: u32,
}
pub const NCRYPT_KEY_ACCESS_POLICY_PROPERTY: windows_core::PCWSTR = windows_core::w!("Key Access Policy");
pub const NCRYPT_KEY_ACCESS_POLICY_VERSION: u32 = 1u32;
pub const NCRYPT_KEY_ATTEST_MAGIC: u32 = 1146110283u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NCRYPT_KEY_ATTEST_PADDING_INFO {
    pub magic: u32,
    pub pbKeyBlob: *mut u8,
    pub cbKeyBlob: u32,
    pub pbKeyAuth: *mut u8,
    pub cbKeyAuth: u32,
}
impl Default for NCRYPT_KEY_ATTEST_PADDING_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NCRYPT_KEY_BLOB_HEADER {
    pub cbSize: u32,
    pub dwMagic: u32,
    pub cbAlgName: u32,
    pub cbKeyData: u32,
}
pub const NCRYPT_KEY_DERIVATION_GROUP: windows_core::PCWSTR = windows_core::w!("KEY_DERIVATION");
pub const NCRYPT_KEY_DERIVATION_INTERFACE: u32 = 7u32;
pub const NCRYPT_KEY_DERIVATION_OPERATION: u32 = 64u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct NCRYPT_KEY_HANDLE(pub usize);
impl NCRYPT_KEY_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == 0
    }
}
impl windows_core::Free for NCRYPT_KEY_HANDLE {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_core::link!("ncrypt.dll" "system" fn NCryptFreeObject(hobject : usize) -> i32);
            unsafe {
                NCryptFreeObject(self.0);
            }
        }
    }
}
impl windows_core::imp::CanInto<NCRYPT_HANDLE> for NCRYPT_KEY_HANDLE {}
impl From<NCRYPT_KEY_HANDLE> for NCRYPT_HANDLE {
    fn from(value: NCRYPT_KEY_HANDLE) -> Self {
        Self(value.0)
    }
}
pub const NCRYPT_KEY_PROTECTION_ALGORITHM_CERTIFICATE: windows_core::PCWSTR = windows_core::w!("CERTIFICATE");
pub const NCRYPT_KEY_PROTECTION_ALGORITHM_LOCAL: windows_core::PCWSTR = windows_core::w!("LOCAL");
pub const NCRYPT_KEY_PROTECTION_ALGORITHM_LOCKEDCREDENTIALS: windows_core::PCWSTR = windows_core::w!("LOCKEDCREDENTIALS");
pub const NCRYPT_KEY_PROTECTION_ALGORITHM_SDDL: windows_core::PCWSTR = windows_core::w!("SDDL");
pub const NCRYPT_KEY_PROTECTION_ALGORITHM_SID: windows_core::PCWSTR = windows_core::w!("SID");
pub const NCRYPT_KEY_PROTECTION_ALGORITHM_WEBCREDENTIALS: windows_core::PCWSTR = windows_core::w!("WEBCREDENTIALS");
pub const NCRYPT_KEY_PROTECTION_CERT_CERTBLOB: windows_core::PCWSTR = windows_core::w!("CertBlob");
pub const NCRYPT_KEY_PROTECTION_CERT_HASHID: windows_core::PCWSTR = windows_core::w!("HashId");
pub const NCRYPT_KEY_PROTECTION_INTERFACE: u32 = 65540u32;
pub const NCRYPT_KEY_PROTECTION_LOCAL_LOGON: windows_core::PCWSTR = windows_core::w!("logon");
pub const NCRYPT_KEY_PROTECTION_LOCAL_MACHINE: windows_core::PCWSTR = windows_core::w!("machine");
pub const NCRYPT_KEY_PROTECTION_LOCAL_USER: windows_core::PCWSTR = windows_core::w!("user");
pub const NCRYPT_KEY_STORAGE_ALGORITHM: windows_core::PCWSTR = windows_core::w!("KEY_STORAGE");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct NCRYPT_KEY_STORAGE_FUNCTION_TABLE {
    pub Version: BCRYPT_INTERFACE_VERSION,
    pub OpenProvider: NCryptOpenStorageProviderFn,
    pub OpenKey: NCryptOpenKeyFn,
    pub CreatePersistedKey: NCryptCreatePersistedKeyFn,
    pub GetProviderProperty: NCryptGetProviderPropertyFn,
    pub GetKeyProperty: NCryptGetKeyPropertyFn,
    pub SetProviderProperty: NCryptSetProviderPropertyFn,
    pub SetKeyProperty: NCryptSetKeyPropertyFn,
    pub FinalizeKey: NCryptFinalizeKeyFn,
    pub DeleteKey: NCryptDeleteKeyFn,
    pub FreeProvider: NCryptFreeProviderFn,
    pub FreeKey: NCryptFreeKeyFn,
    pub FreeBuffer: NCryptFreeBufferFn,
    pub Encrypt: NCryptEncryptFn,
    pub Decrypt: NCryptDecryptFn,
    pub IsAlgSupported: NCryptIsAlgSupportedFn,
    pub EnumAlgorithms: NCryptEnumAlgorithmsFn,
    pub EnumKeys: NCryptEnumKeysFn,
    pub ImportKey: NCryptImportKeyFn,
    pub ExportKey: NCryptExportKeyFn,
    pub SignHash: NCryptSignHashFn,
    pub VerifySignature: NCryptVerifySignatureFn,
    pub PromptUser: NCryptPromptUserFn,
    pub NotifyChangeKey: NCryptNotifyChangeKeyFn,
    pub SecretAgreement: NCryptSecretAgreementFn,
    pub DeriveKey: NCryptDeriveKeyFn,
    pub FreeSecret: NCryptFreeSecretFn,
    pub KeyDerivation: NCryptKeyDerivationFn,
    pub CreateClaim: NCryptCreateClaimFn,
    pub VerifyClaim: NCryptVerifyClaimFn,
}
pub const NCRYPT_KEY_STORAGE_INTERFACE: BCRYPT_INTERFACE = BCRYPT_INTERFACE(65537u32);
pub const NCRYPT_KEY_TYPE_PROPERTY: windows_core::PCWSTR = windows_core::w!("Key Type");
pub const NCRYPT_KEY_USAGE_PROPERTY: windows_core::PCWSTR = windows_core::w!("Key Usage");
pub const NCRYPT_LAST_MODIFIED_PROPERTY: windows_core::PCWSTR = windows_core::w!("Modified");
pub const NCRYPT_LENGTHS_PROPERTY: windows_core::PCWSTR = windows_core::w!("Lengths");
pub const NCRYPT_LENGTH_PROPERTY: windows_core::PCWSTR = windows_core::w!("Length");
pub const NCRYPT_MACHINE_KEY_FLAG: NCRYPT_FLAGS = NCRYPT_FLAGS(32u32);
pub const NCRYPT_MAX_ALG_ID_LENGTH: u32 = 512u32;
pub const NCRYPT_MAX_KEY_NAME_LENGTH: u32 = 512u32;
pub const NCRYPT_MAX_NAME_LENGTH_PROPERTY: windows_core::PCWSTR = windows_core::w!("Max Name Length");
pub const NCRYPT_MAX_PROPERTY_DATA: u32 = 1048576u32;
pub const NCRYPT_MAX_PROPERTY_NAME: u32 = 64u32;
pub const NCRYPT_MD2_ALGORITHM: windows_core::PCWSTR = windows_core::w!("MD2");
pub const NCRYPT_MD4_ALGORITHM: windows_core::PCWSTR = windows_core::w!("MD4");
pub const NCRYPT_MD5_ALGORITHM: windows_core::PCWSTR = windows_core::w!("MD5");
pub const NCRYPT_NAMED_DESCRIPTOR_FLAG: u32 = 1u32;
pub const NCRYPT_NAME_PROPERTY: windows_core::PCWSTR = windows_core::w!("Name");
pub const NCRYPT_NO_CACHED_PASSWORD: u32 = 16384u32;
pub const NCRYPT_NO_KEY_VALIDATION: NCRYPT_FLAGS = NCRYPT_FLAGS(8u32);
pub const NCRYPT_NO_PADDING_FLAG: NCRYPT_FLAGS = NCRYPT_FLAGS(1u32);
pub const NCRYPT_OPAQUETRANSPORT_BLOB: windows_core::PCWSTR = windows_core::w!("OpaqueTransport");
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NCRYPT_OPERATION(pub u32);
impl NCRYPT_OPERATION {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for NCRYPT_OPERATION {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for NCRYPT_OPERATION {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for NCRYPT_OPERATION {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for NCRYPT_OPERATION {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for NCRYPT_OPERATION {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const NCRYPT_OVERWRITE_KEY_FLAG: NCRYPT_FLAGS = NCRYPT_FLAGS(128u32);
pub const NCRYPT_PAD_CIPHER_FLAG: u32 = 16u32;
pub const NCRYPT_PAD_OAEP_FLAG: NCRYPT_FLAGS = NCRYPT_FLAGS(4u32);
pub const NCRYPT_PAD_PKCS1_FLAG: NCRYPT_FLAGS = NCRYPT_FLAGS(2u32);
pub const NCRYPT_PAD_PSS_FLAG: NCRYPT_FLAGS = NCRYPT_FLAGS(8u32);
pub const NCRYPT_PBKDF2_ALGORITHM: windows_core::PCWSTR = windows_core::w!("PBKDF2");
pub const NCRYPT_PCP_ALTERNATE_KEY_STORAGE_LOCATION_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_ALTERNATE_KEY_STORAGE_LOCATION");
pub const NCRYPT_PCP_CHANGEPASSWORD_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_CHANGEPASSWORD");
pub const NCRYPT_PCP_ECC_EKCERT_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_ECC_EKCERT");
pub const NCRYPT_PCP_ECC_EKNVCERT_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_ECC_EKNVCERT");
pub const NCRYPT_PCP_ECC_EKPUB_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_ECC_EKPUB");
pub const NCRYPT_PCP_EKCERT_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_EKCERT");
pub const NCRYPT_PCP_EKNVCERT_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_EKNVCERT");
pub const NCRYPT_PCP_EKPUB_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_EKPUB");
pub const NCRYPT_PCP_ENCRYPTION_KEY: u32 = 2u32;
pub const NCRYPT_PCP_EXPORT_ALLOWED_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_EXPORT_ALLOWED");
pub const NCRYPT_PCP_HMACVERIFICATION_KEY: u32 = 16u32;
pub const NCRYPT_PCP_HMAC_AUTH_NONCE: windows_core::PCWSTR = windows_core::w!("PCP_HMAC_AUTH_NONCE");
pub const NCRYPT_PCP_HMAC_AUTH_POLICYINFO: windows_core::PCWSTR = windows_core::w!("PCP_HMAC_AUTH_POLICYINFO");
pub const NCRYPT_PCP_HMAC_AUTH_POLICYREF: windows_core::PCWSTR = windows_core::w!("PCP_HMAC_AUTH_POLICYREF");
pub const NCRYPT_PCP_HMAC_AUTH_SIGNATURE: windows_core::PCWSTR = windows_core::w!("PCP_HMAC_AUTH_SIGNATURE");
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NCRYPT_PCP_HMAC_AUTH_SIGNATURE_INFO {
    pub dwVersion: u32,
    pub iExpiration: i32,
    pub pabNonce: [u8; 32],
    pub pabPolicyRef: [u8; 32],
    pub pabHMAC: [u8; 32],
}
impl Default for NCRYPT_PCP_HMAC_AUTH_SIGNATURE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NCRYPT_PCP_HMAC_AUTH_TICKET: windows_core::PCWSTR = windows_core::w!("PCP_HMAC_AUTH_TICKET");
pub const NCRYPT_PCP_IDENTITY_KEY: u32 = 8u32;
pub const NCRYPT_PCP_INTERMEDIATE_CA_EKCERT_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_INTERMEDIATE_CA_EKCERT");
pub const NCRYPT_PCP_KEYATTESTATION_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_TPM12_KEYATTESTATION");
pub const NCRYPT_PCP_KEY_CREATIONHASH_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_KEY_CREATIONHASH");
pub const NCRYPT_PCP_KEY_CREATIONTICKET_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_KEY_CREATIONTICKET");
pub const NCRYPT_PCP_KEY_USAGE_POLICY_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_KEY_USAGE_POLICY");
pub const NCRYPT_PCP_MIGRATIONPASSWORD_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_MIGRATIONPASSWORD");
pub const NCRYPT_PCP_NO_DA_PROTECTION_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_NO_DA_PROTECTION");
pub const NCRYPT_PCP_PASSWORD_REQUIRED_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_PASSWORD_REQUIRED");
pub const NCRYPT_PCP_PCRTABLE_ALGORITHM_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_PCRTABLE_ALGORITHM");
pub const NCRYPT_PCP_PCRTABLE_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_PCRTABLE");
pub const NCRYPT_PCP_PLATFORMHANDLE_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_PLATFORMHANDLE");
pub const NCRYPT_PCP_PLATFORM_BINDING_PCRALGID_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_PLATFORM_BINDING_PCRALGID");
pub const NCRYPT_PCP_PLATFORM_BINDING_PCRDIGESTLIST_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_PLATFORM_BINDING_PCRDIGESTLIST");
pub const NCRYPT_PCP_PLATFORM_BINDING_PCRDIGEST_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_PLATFORM_BINDING_PCRDIGEST");
pub const NCRYPT_PCP_PLATFORM_BINDING_PCRMASK_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_PLATFORM_BINDING_PCRMASK");
pub const NCRYPT_PCP_PLATFORM_TYPE_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_PLATFORM_TYPE");
pub const NCRYPT_PCP_PROVIDERHANDLE_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_PROVIDERMHANDLE");
pub const NCRYPT_PCP_PROVIDER_VERSION_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_PROVIDER_VERSION");
pub const NCRYPT_PCP_PSS_SALT_SIZE_PROPERTY: windows_core::PCWSTR = windows_core::w!("PSS Salt Size");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NCRYPT_PCP_RAW_POLICYDIGEST_INFO {
    pub dwVersion: u32,
    pub cbDigest: u32,
}
pub const NCRYPT_PCP_RAW_POLICYDIGEST_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_RAW_POLICYDIGEST");
pub const NCRYPT_PCP_RSA_EKCERT_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_RSA_EKCERT");
pub const NCRYPT_PCP_RSA_EKNVCERT_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_RSA_EKNVCERT");
pub const NCRYPT_PCP_RSA_EKPUB_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_RSA_EKPUB");
pub const NCRYPT_PCP_RSA_SCHEME_HASH_ALG_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_RSA_SCHEME_HASH_ALG");
pub const NCRYPT_PCP_RSA_SCHEME_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_RSA_SCHEME");
pub const NCRYPT_PCP_SESSIONID_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_SESSIONID");
pub const NCRYPT_PCP_SIGNATURE_KEY: u32 = 1u32;
pub const NCRYPT_PCP_SRKPUB_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_SRKPUB");
pub const NCRYPT_PCP_STORAGEPARENT_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_STORAGEPARENT");
pub const NCRYPT_PCP_STORAGE_KEY: u32 = 4u32;
pub const NCRYPT_PCP_SYMMETRIC_KEYBITS_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_SYMMETRIC_KEYBITS");
pub const NCRYPT_PCP_TPM12_IDACTIVATION_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_TPM12_IDACTIVATION");
pub const NCRYPT_PCP_TPM12_IDBINDING_DYNAMIC_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_TPM12_IDBINDING_DYNAMIC");
pub const NCRYPT_PCP_TPM12_IDBINDING_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_TPM12_IDBINDING");
pub const NCRYPT_PCP_TPM2BNAME_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_TPM2BNAME");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NCRYPT_PCP_TPM_FW_VERSION_INFO {
    pub major1: u16,
    pub major2: u16,
    pub minor1: u16,
    pub minor2: u16,
}
pub const NCRYPT_PCP_TPM_FW_VERSION_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_TPM_FW_VERSION");
pub const NCRYPT_PCP_TPM_IFX_RSA_KEYGEN_PROHIBITED_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_TPM_IFX_RSA_KEYGEN_PROHIBITED");
pub const NCRYPT_PCP_TPM_IFX_RSA_KEYGEN_VULNERABILITY_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_TPM_IFX_RSA_KEYGEN_VULNERABILITY");
pub const NCRYPT_PCP_TPM_MANUFACTURER_ID_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_TPM_MANUFACTURER_ID");
pub const NCRYPT_PCP_TPM_VERSION_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_TPM_VERSION");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NCRYPT_PCP_TPM_WEB_AUTHN_ATTESTATION_STATEMENT {
    pub Magic: u32,
    pub Version: u32,
    pub HeaderSize: u32,
    pub cbCertifyInfo: u32,
    pub cbSignature: u32,
    pub cbTpmPublic: u32,
}
pub const NCRYPT_PCP_USAGEAUTH_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_USAGEAUTH");
pub const NCRYPT_PERSIST_FLAG: NCRYPT_FLAGS = NCRYPT_FLAGS(2147483648u32);
pub const NCRYPT_PERSIST_ONLY_FLAG: NCRYPT_FLAGS = NCRYPT_FLAGS(1073741824u32);
pub const NCRYPT_PIN_CACHE_APPLICATION_IMAGE_PROPERTY: windows_core::PCWSTR = windows_core::w!("PinCacheApplicationImage");
pub const NCRYPT_PIN_CACHE_APPLICATION_STATUS_PROPERTY: windows_core::PCWSTR = windows_core::w!("PinCacheApplicationStatus");
pub const NCRYPT_PIN_CACHE_APPLICATION_TICKET_BYTE_LENGTH: u32 = 90u32;
pub const NCRYPT_PIN_CACHE_APPLICATION_TICKET_PROPERTY: windows_core::PCWSTR = windows_core::w!("PinCacheApplicationTicket");
pub const NCRYPT_PIN_CACHE_CLEAR_FOR_CALLING_PROCESS_OPTION: u32 = 1u32;
pub const NCRYPT_PIN_CACHE_CLEAR_PROPERTY: windows_core::PCWSTR = windows_core::w!("PinCacheClear");
pub const NCRYPT_PIN_CACHE_DISABLE_DPL_FLAG: u32 = 1u32;
pub const NCRYPT_PIN_CACHE_FLAGS_PROPERTY: windows_core::PCWSTR = windows_core::w!("PinCacheFlags");
pub const NCRYPT_PIN_CACHE_FREE_APPLICATION_TICKET_PROPERTY: windows_core::PCWSTR = windows_core::w!("PinCacheFreeApplicationTicket");
pub const NCRYPT_PIN_CACHE_IS_GESTURE_REQUIRED_PROPERTY: windows_core::PCWSTR = windows_core::w!("PinCacheIsGestureRequired");
pub const NCRYPT_PIN_CACHE_PIN_PROPERTY: windows_core::PCWSTR = windows_core::w!("PinCachePin");
pub const NCRYPT_PIN_CACHE_REQUIRE_GESTURE_FLAG: u32 = 1u32;
pub const NCRYPT_PIN_PROMPT_PROPERTY: windows_core::PCWSTR = windows_core::w!("SmartCardPinPrompt");
pub const NCRYPT_PIN_PROPERTY: windows_core::PCWSTR = windows_core::w!("SmartCardPin");
pub const NCRYPT_PKCS7_ENVELOPE_BLOB: windows_core::PCWSTR = windows_core::w!("PKCS7_ENVELOPE");
pub const NCRYPT_PKCS8_PRIVATE_KEY_BLOB: windows_core::PCWSTR = windows_core::w!("PKCS8_PRIVATEKEY");
pub const NCRYPT_PLATFORM_ATTEST_MAGIC: u32 = 1146110288u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NCRYPT_PLATFORM_ATTEST_PADDING_INFO {
    pub magic: u32,
    pub pcrMask: u32,
}
pub const NCRYPT_PREFER_VIRTUAL_ISOLATION_FLAG: u32 = 65536u32;
pub const NCRYPT_PROTECTED_KEY_BLOB: windows_core::PCWSTR = windows_core::w!("ProtectedKeyBlob");
pub const NCRYPT_PROTECTED_KEY_BLOB_MAGIC: u32 = 1263817296u32;
pub const NCRYPT_PROTECTION_INFO_TYPE_DESCRIPTOR_STRING: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct NCRYPT_PROTECT_STREAM_INFO {
    pub pfnStreamOutput: PFNCryptStreamOutputCallback,
    pub pvCallbackCtxt: *mut core::ffi::c_void,
}
impl Default for NCRYPT_PROTECT_STREAM_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct NCRYPT_PROTECT_STREAM_INFO_EX {
    pub pfnStreamOutput: PFNCryptStreamOutputCallbackEx,
    pub pvCallbackCtxt: *mut core::ffi::c_void,
}
impl Default for NCRYPT_PROTECT_STREAM_INFO_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NCRYPT_PROTECT_TO_LOCAL_SYSTEM: u32 = 32768u32;
pub const NCRYPT_PROVIDER_HANDLE_PROPERTY: windows_core::PCWSTR = windows_core::w!("Provider Handle");
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct NCRYPT_PROV_HANDLE(pub usize);
impl NCRYPT_PROV_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == 0
    }
}
impl windows_core::Free for NCRYPT_PROV_HANDLE {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_core::link!("ncrypt.dll" "system" fn NCryptFreeObject(hobject : usize) -> i32);
            unsafe {
                NCryptFreeObject(self.0);
            }
        }
    }
}
impl windows_core::imp::CanInto<NCRYPT_HANDLE> for NCRYPT_PROV_HANDLE {}
impl From<NCRYPT_PROV_HANDLE> for NCRYPT_HANDLE {
    fn from(value: NCRYPT_PROV_HANDLE) -> Self {
        Self(value.0)
    }
}
pub const NCRYPT_PUBLIC_LENGTH_PROPERTY: windows_core::PCWSTR = windows_core::w!("PublicKeyLength");
pub const NCRYPT_RC2_ALGORITHM: windows_core::PCWSTR = windows_core::w!("RC2");
pub const NCRYPT_RC2_ALGORITHM_GROUP: windows_core::PCWSTR = windows_core::w!("RC2");
pub const NCRYPT_READER_ICON_PROPERTY: windows_core::PCWSTR = windows_core::w!("SmartCardReaderIcon");
pub const NCRYPT_READER_PROPERTY: windows_core::PCWSTR = windows_core::w!("SmartCardReader");
pub const NCRYPT_REGISTER_NOTIFY_FLAG: NCRYPT_FLAGS = NCRYPT_FLAGS(1u32);
pub const NCRYPT_REQUIRE_KDS_LRPC_BIND_FLAG: u32 = 536870912u32;
pub const NCRYPT_ROOT_CERTSTORE_PROPERTY: windows_core::PCWSTR = windows_core::w!("SmartcardRootCertStore");
pub const NCRYPT_RSA_ALGORITHM: windows_core::PCWSTR = windows_core::w!("RSA");
pub const NCRYPT_RSA_ALGORITHM_GROUP: windows_core::PCWSTR = windows_core::w!("RSA");
pub const NCRYPT_RSA_SIGN_ALGORITHM: windows_core::PCWSTR = windows_core::w!("RSA_SIGN");
pub const NCRYPT_SCARD_NGC_KEY_NAME: windows_core::PCWSTR = windows_core::w!("SmartCardNgcKeyName");
pub const NCRYPT_SCARD_PIN_ID: windows_core::PCWSTR = windows_core::w!("SmartCardPinId");
pub const NCRYPT_SCARD_PIN_INFO: windows_core::PCWSTR = windows_core::w!("SmartCardPinInfo");
pub const NCRYPT_SCHANNEL_INTERFACE: BCRYPT_INTERFACE = BCRYPT_INTERFACE(65538u32);
pub const NCRYPT_SCHANNEL_SIGNATURE_INTERFACE: BCRYPT_INTERFACE = BCRYPT_INTERFACE(65539u32);
pub const NCRYPT_SEALING_FLAG: u32 = 256u32;
pub const NCRYPT_SECRET_AGREEMENT_INTERFACE: NCRYPT_ALGORITHM_NAME_CLASS = NCRYPT_ALGORITHM_NAME_CLASS(4u32);
pub const NCRYPT_SECRET_AGREEMENT_OPERATION: NCRYPT_OPERATION = NCRYPT_OPERATION(8u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct NCRYPT_SECRET_HANDLE(pub usize);
impl NCRYPT_SECRET_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == 0
    }
}
pub const NCRYPT_SECURE_PIN_PROPERTY: windows_core::PCWSTR = windows_core::w!("SmartCardSecurePin");
pub const NCRYPT_SECURITY_DESCR_PROPERTY: windows_core::PCWSTR = windows_core::w!("Security Descr");
pub const NCRYPT_SECURITY_DESCR_SUPPORT_PROPERTY: windows_core::PCWSTR = windows_core::w!("Security Descr Support");
pub const NCRYPT_SHA1_ALGORITHM: windows_core::PCWSTR = windows_core::w!("SHA1");
pub const NCRYPT_SHA256_ALGORITHM: windows_core::PCWSTR = windows_core::w!("SHA256");
pub const NCRYPT_SHA384_ALGORITHM: windows_core::PCWSTR = windows_core::w!("SHA384");
pub const NCRYPT_SHA512_ALGORITHM: windows_core::PCWSTR = windows_core::w!("SHA512");
pub const NCRYPT_SIGNATURE_INTERFACE: NCRYPT_ALGORITHM_NAME_CLASS = NCRYPT_ALGORITHM_NAME_CLASS(5u32);
pub const NCRYPT_SIGNATURE_LENGTH_PROPERTY: windows_core::PCWSTR = windows_core::w!("SignatureLength");
pub const NCRYPT_SIGNATURE_OPERATION: NCRYPT_OPERATION = NCRYPT_OPERATION(16u32);
pub const NCRYPT_SILENT_FLAG: NCRYPT_FLAGS = NCRYPT_FLAGS(64u32);
pub const NCRYPT_SMARTCARD_GUID_PROPERTY: windows_core::PCWSTR = windows_core::w!("SmartCardGuid");
pub const NCRYPT_SP800108_CTR_HMAC_ALGORITHM: windows_core::PCWSTR = windows_core::w!("SP800_108_CTR_HMAC");
pub const NCRYPT_SP80056A_CONCAT_ALGORITHM: windows_core::PCWSTR = windows_core::w!("SP800_56A_CONCAT");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NCRYPT_SSL_CIPHER_LENGTHS {
    pub cbLength: u32,
    pub dwHeaderLen: u32,
    pub dwFixedTrailerLen: u32,
    pub dwMaxVariableTrailerLen: u32,
    pub dwFlags: u32,
}
pub const NCRYPT_SSL_CIPHER_LENGTHS_BLOCK_PADDING: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NCRYPT_SSL_CIPHER_SUITE {
    pub dwProtocol: u32,
    pub dwCipherSuite: u32,
    pub dwBaseCipherSuite: u32,
    pub szCipherSuite: [u16; 64],
    pub szCipher: [u16; 64],
    pub dwCipherLen: u32,
    pub dwCipherBlockLen: u32,
    pub szHash: [u16; 64],
    pub dwHashLen: u32,
    pub szExchange: [u16; 64],
    pub dwMinExchangeLen: u32,
    pub dwMaxExchangeLen: u32,
    pub szCertificate: [u16; 64],
    pub dwKeyType: u32,
}
impl Default for NCRYPT_SSL_CIPHER_SUITE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NCRYPT_SSL_CIPHER_SUITE_EX {
    pub dwVersion: u32,
    pub dwProtocol: u32,
    pub dwCipherSuite: u32,
    pub dwBaseCipherSuite: u32,
    pub szCipherSuite: [u16; 64],
    pub szCipher: [u16; 64],
    pub dwCipherLen: u32,
    pub dwCipherBlockLen: u32,
    pub szHash: [u16; 64],
    pub dwHashLen: u32,
    pub szExchange: [u16; 64],
    pub dwMinExchangeLen: u32,
    pub dwMaxExchangeLen: u32,
    pub szCertificate: [u16; 64],
    pub dwKeyType: u32,
    pub szCipherMode: [u16; 64],
}
impl Default for NCRYPT_SSL_CIPHER_SUITE_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NCRYPT_SSL_CIPHER_SUITE_EX_VERSION: u32 = 1u32;
pub const NCRYPT_SSL_CLIENT_FLAG: u32 = 1u32;
pub const NCRYPT_SSL_EAP_FAST_ID: u32 = 3u32;
pub const NCRYPT_SSL_EAP_ID: u32 = 0u32;
pub const NCRYPT_SSL_EAP_PRF_FIELD: u32 = 255u32;
pub const NCRYPT_SSL_EAP_TTLSV0_CHLNG_ID: u32 = 2u32;
pub const NCRYPT_SSL_EAP_TTLSV0_ID: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NCRYPT_SSL_ECC_CURVE {
    pub szCurveName: [u16; 255],
    pub szOID: [i8; 255],
    pub dwPublicKeyLength: u32,
    pub dwCurveType: u32,
    pub dwFlags: u32,
}
impl Default for NCRYPT_SSL_ECC_CURVE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NCRYPT_SSL_EXTERNAL_PSK_FLAG: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct NCRYPT_SSL_FUNCTION_TABLE {
    pub Version: BCRYPT_INTERFACE_VERSION,
    pub ComputeClientAuthHash: SslComputeClientAuthHashFn,
    pub ComputeEapKeyBlock: SslComputeEapKeyBlockFn,
    pub ComputeFinishedHash: SslComputeFinishedHashFn,
    pub CreateEphemeralKey: SslCreateEphemeralKeyFn,
    pub CreateHandshakeHash: SslCreateHandshakeHashFn,
    pub DecryptPacket: SslDecryptPacketFn,
    pub EncryptPacket: SslEncryptPacketFn,
    pub EnumCipherSuites: SslEnumCipherSuitesFn,
    pub ExportKey: SslExportKeyFn,
    pub FreeBuffer: SslFreeBufferFn,
    pub FreeObject: SslFreeObjectFn,
    pub GenerateMasterKey: SslGenerateMasterKeyFn,
    pub GenerateSessionKeys: SslGenerateSessionKeysFn,
    pub GetKeyProperty: SslGetKeyPropertyFn,
    pub GetProviderProperty: SslGetProviderPropertyFn,
    pub HashHandshake: SslHashHandshakeFn,
    pub ImportMasterKey: SslImportMasterKeyFn,
    pub ImportKey: SslImportKeyFn,
    pub LookupCipherSuiteInfo: SslLookupCipherSuiteInfoFn,
    pub OpenPrivateKey: SslOpenPrivateKeyFn,
    pub OpenProvider: SslOpenProviderFn,
    pub SignHash: SslSignHashFn,
    pub VerifySignature: SslVerifySignatureFn,
    pub LookupCipherLengths: SslLookupCipherLengthsFn,
    pub CreateClientAuthHash: SslCreateClientAuthHashFn,
    pub GetCipherSuitePRFHashAlgorithm: SslGetCipherSuitePRFHashAlgorithmFn,
    pub ComputeSessionHash: SslComputeSessionHashFn,
    pub GeneratePreMasterKey: SslGeneratePreMasterKeyFn,
    pub EnumEccCurves: SslEnumEccCurvesFn,
    pub ExportKeyingMaterial: SslExportKeyingMaterialFn,
    pub ExtractEarlyKey: SslExtractEarlyKeyFn,
    pub ExtractHandshakeKey: SslExtractHandshakeKeyFn,
    pub ExtractMasterKey: SslExtractMasterKeyFn,
    pub ExpandTrafficKeys: SslExpandTrafficKeysFn,
    pub ExpandWriteKey: SslExpandWriteKeyFn,
    pub ExpandExporterMasterKey: SslExpandExporterMasterKeyFn,
    pub EnumCipherSuitesEx: SslEnumCipherSuitesExFn,
    pub ExpandResumptionMasterKey: SslExpandResumptionMasterKeyFn,
    pub DuplicateTranscriptHash: SslDuplicateTranscriptHashFn,
    pub ExpandBinderKey: SslExpandBinderKeyFn,
    pub ExpandPreSharedKey: SslExpandPreSharedKeyFn,
}
pub const NCRYPT_SSL_MAX_NAME_SIZE: u32 = 64u32;
pub const NCRYPT_SSL_RESUMPTION_PSK_FLAG: u32 = 2u32;
pub const NCRYPT_SSL_SERVER_FLAG: u32 = 2u32;
pub const NCRYPT_SSL_SIGN_INCLUDE_HASHOID: u32 = 1u32;
pub const NCRYPT_SSL_SIGN_USE_PSS_PADDING: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NCRYPT_SUPPORTED_LENGTHS {
    pub dwMinLength: u32,
    pub dwMaxLength: u32,
    pub dwIncrement: u32,
    pub dwDefaultLength: u32,
}
pub const NCRYPT_TPM12_PROVIDER: u32 = 65536u32;
pub const NCRYPT_TPM_LOADABLE_KEY_BLOB: windows_core::PCWSTR = windows_core::w!("PcpTpmProtectedKeyBlob");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NCRYPT_TPM_LOADABLE_KEY_BLOB_HEADER {
    pub magic: u32,
    pub cbHeader: u32,
    pub cbPublic: u32,
    pub cbPrivate: u32,
    pub cbName: u32,
}
pub const NCRYPT_TPM_LOADABLE_KEY_BLOB_MAGIC: u32 = 1297371211u32;
pub const NCRYPT_TPM_PAD_PSS_IGNORE_SALT: u32 = 32u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NCRYPT_TPM_PLATFORM_ATTESTATION_STATEMENT {
    pub Magic: u32,
    pub Version: u32,
    pub pcrAlg: u32,
    pub cbSignature: u32,
    pub cbQuote: u32,
    pub cbPcrs: u32,
}
pub const NCRYPT_TPM_PLATFORM_ATTESTATION_STATEMENT_CURRENT_VERSION: u32 = 0u32;
pub const NCRYPT_TPM_PLATFORM_ATTESTATION_STATEMENT_V0: u32 = 0u32;
pub const NCRYPT_TPM_PSS_SALT_SIZE_HASHSIZE: u32 = 2u32;
pub const NCRYPT_TPM_PSS_SALT_SIZE_MAXIMUM: u32 = 1u32;
pub const NCRYPT_TPM_PSS_SALT_SIZE_UNKNOWN: u32 = 0u32;
pub const NCRYPT_TREAT_NIST_AS_GENERIC_ECC_FLAG: u32 = 8192u32;
pub const NCRYPT_UI_APPCONTAINER_ACCESS_MEDIUM_FLAG: u32 = 8u32;
pub const NCRYPT_UI_FINGERPRINT_PROTECTION_FLAG: u32 = 4u32;
pub const NCRYPT_UI_FORCE_HIGH_PROTECTION_FLAG: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NCRYPT_UI_POLICY {
    pub dwVersion: u32,
    pub dwFlags: u32,
    pub pszCreationTitle: windows_core::PCWSTR,
    pub pszFriendlyName: windows_core::PCWSTR,
    pub pszDescription: windows_core::PCWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NCRYPT_UI_POLICY_BLOB {
    pub dwVersion: u32,
    pub dwFlags: u32,
    pub cbCreationTitle: u32,
    pub cbFriendlyName: u32,
    pub cbDescription: u32,
}
pub const NCRYPT_UI_POLICY_PROPERTY: windows_core::PCWSTR = windows_core::w!("UI Policy");
pub const NCRYPT_UI_PROTECT_KEY_FLAG: u32 = 1u32;
pub const NCRYPT_UNIQUE_NAME_PROPERTY: windows_core::PCWSTR = windows_core::w!("Unique Name");
pub const NCRYPT_UNPROTECT_NO_DECRYPT: NCRYPT_FLAGS = NCRYPT_FLAGS(1u32);
pub const NCRYPT_UNREGISTER_NOTIFY_FLAG: NCRYPT_FLAGS = NCRYPT_FLAGS(2u32);
pub const NCRYPT_USER_CERTSTORE_PROPERTY: windows_core::PCWSTR = windows_core::w!("SmartCardUserCertStore");
pub const NCRYPT_USE_CONTEXT_PROPERTY: windows_core::PCWSTR = windows_core::w!("Use Context");
pub const NCRYPT_USE_COUNT_ENABLED_PROPERTY: windows_core::PCWSTR = windows_core::w!("Enabled Use Count");
pub const NCRYPT_USE_COUNT_PROPERTY: windows_core::PCWSTR = windows_core::w!("Use Count");
pub const NCRYPT_USE_PER_BOOT_KEY_FLAG: u32 = 262144u32;
pub const NCRYPT_USE_PER_BOOT_KEY_PROPERTY: windows_core::PCWSTR = windows_core::w!("Per Boot Key");
pub const NCRYPT_USE_VIRTUAL_ISOLATION_FLAG: u32 = 131072u32;
pub const NCRYPT_USE_VIRTUAL_ISOLATION_PROPERTY: windows_core::PCWSTR = windows_core::w!("Virtual Iso");
pub const NCRYPT_VERSION_PROPERTY: windows_core::PCWSTR = windows_core::w!("Version");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NCRYPT_VSM_KEY_ATTESTATION_CLAIM_RESTRICTIONS {
    pub Version: u32,
    pub TrustletId: u64,
    pub MinSvn: u32,
    pub FlagsMask: u32,
    pub FlagsExpected: u32,
    pub _bitfield: u32,
}
pub const NCRYPT_VSM_KEY_ATTESTATION_CLAIM_RESTRICTIONS_CURRENT_VERSION: u32 = 0u32;
pub const NCRYPT_VSM_KEY_ATTESTATION_CLAIM_RESTRICTIONS_V0: u32 = 0u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NCRYPT_VSM_KEY_ATTESTATION_STATEMENT {
    pub Magic: u32,
    pub Version: u32,
    pub cbSignature: u32,
    pub cbReport: u32,
    pub cbAttributes: u32,
}
pub const NCRYPT_VSM_KEY_ATTESTATION_STATEMENT_CURRENT_VERSION: u32 = 0u32;
pub const NCRYPT_VSM_KEY_ATTESTATION_STATEMENT_V0: u32 = 0u32;
pub const NCRYPT_WINDOW_HANDLE_PROPERTY: windows_core::PCWSTR = windows_core::w!("HWND Handle");
pub const NCRYPT_WRITE_KEY_TO_LEGACY_STORE_FLAG: NCRYPT_FLAGS = NCRYPT_FLAGS(512u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NCryptAlgorithmName {
    pub pszName: windows_core::PWSTR,
    pub dwClass: NCRYPT_ALGORITHM_NAME_CLASS,
    pub dwAlgOperations: NCRYPT_OPERATION,
    pub dwFlags: u32,
}
pub type NCryptCreateClaimFn = Option<unsafe extern "system" fn(hprov: NCRYPT_PROV_HANDLE, hsubjectkey: NCRYPT_KEY_HANDLE, hauthoritykey: NCRYPT_KEY_HANDLE, dwclaimtype: u32, pparameterlist: *const BCryptBufferDesc, pbclaimblob: *mut u8, cbclaimblob: u32, pcbresult: *mut u32, dwflags: u32) -> windows_core::HRESULT>;
pub type NCryptCreatePersistedKeyFn = Option<unsafe extern "system" fn(hprovider: NCRYPT_PROV_HANDLE, phkey: *mut NCRYPT_KEY_HANDLE, pszalgid: windows_core::PCWSTR, pszkeyname: windows_core::PCWSTR, dwlegacykeyspec: u32, dwflags: u32) -> windows_core::HRESULT>;
pub type NCryptDecryptFn = Option<unsafe extern "system" fn(hprovider: NCRYPT_PROV_HANDLE, hkey: NCRYPT_KEY_HANDLE, pbinput: *const u8, cbinput: u32, ppaddinginfo: *const core::ffi::c_void, pboutput: *mut u8, cboutput: u32, pcbresult: *mut u32, dwflags: u32) -> windows_core::HRESULT>;
pub type NCryptDeleteKeyFn = Option<unsafe extern "system" fn(hprovider: NCRYPT_PROV_HANDLE, hkey: NCRYPT_KEY_HANDLE, dwflags: u32) -> windows_core::HRESULT>;
pub type NCryptDeriveKeyFn = Option<unsafe extern "system" fn(hprovider: NCRYPT_PROV_HANDLE, hsharedsecret: NCRYPT_SECRET_HANDLE, pwszkdf: windows_core::PCWSTR, pparameterlist: *const BCryptBufferDesc, pbderivedkey: *mut u8, cbderivedkey: u32, pcbresult: *mut u32, dwflags: u32) -> windows_core::HRESULT>;
pub type NCryptEncryptFn = Option<unsafe extern "system" fn(hprovider: NCRYPT_PROV_HANDLE, hkey: NCRYPT_KEY_HANDLE, pbinput: *const u8, cbinput: u32, ppaddinginfo: *const core::ffi::c_void, pboutput: *mut u8, cboutput: u32, pcbresult: *mut u32, dwflags: u32) -> windows_core::HRESULT>;
pub type NCryptEnumAlgorithmsFn = Option<unsafe extern "system" fn(hprovider: NCRYPT_PROV_HANDLE, dwalgclass: u32, pdwalgcount: *mut u32, ppalglist: *mut *mut NCryptAlgorithmName, dwflags: u32) -> windows_core::HRESULT>;
pub type NCryptEnumKeysFn = Option<unsafe extern "system" fn(hprovider: NCRYPT_PROV_HANDLE, pszscope: windows_core::PCWSTR, ppkeyname: *mut *mut NCryptKeyName, ppenumstate: *mut *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT>;
pub type NCryptEnumStorageProvidersFn = Option<unsafe extern "system" fn(pdwprovidercount: *mut u32, ppproviderlist: *mut *mut NCryptProviderName, dwflags: u32) -> windows_core::HRESULT>;
pub type NCryptExportKeyFn = Option<unsafe extern "system" fn(hprovider: NCRYPT_PROV_HANDLE, hkey: NCRYPT_KEY_HANDLE, hexportkey: NCRYPT_KEY_HANDLE, pszblobtype: windows_core::PCWSTR, pparameterlist: *const BCryptBufferDesc, pboutput: *mut u8, cboutput: u32, pcbresult: *mut u32, dwflags: u32) -> windows_core::HRESULT>;
pub type NCryptFinalizeKeyFn = Option<unsafe extern "system" fn(hprovider: NCRYPT_PROV_HANDLE, hkey: NCRYPT_KEY_HANDLE, dwflags: u32) -> windows_core::HRESULT>;
pub type NCryptFreeBufferFn = Option<unsafe extern "system" fn(pvinput: *mut core::ffi::c_void) -> windows_core::HRESULT>;
pub type NCryptFreeKeyFn = Option<unsafe extern "system" fn(hprovider: NCRYPT_PROV_HANDLE, hkey: NCRYPT_KEY_HANDLE) -> windows_core::HRESULT>;
pub type NCryptFreeProviderFn = Option<unsafe extern "system" fn(hprovider: NCRYPT_PROV_HANDLE) -> windows_core::HRESULT>;
pub type NCryptFreeSecretFn = Option<unsafe extern "system" fn(hprovider: NCRYPT_PROV_HANDLE, hsharedsecret: NCRYPT_SECRET_HANDLE) -> windows_core::HRESULT>;
pub type NCryptGetKeyPropertyFn = Option<unsafe extern "system" fn(hprovider: NCRYPT_PROV_HANDLE, hkey: NCRYPT_KEY_HANDLE, pszproperty: windows_core::PCWSTR, pboutput: *mut u8, cboutput: u32, pcbresult: *mut u32, dwflags: u32) -> windows_core::HRESULT>;
pub type NCryptGetProviderPropertyFn = Option<unsafe extern "system" fn(hprovider: NCRYPT_PROV_HANDLE, pszproperty: windows_core::PCWSTR, pboutput: *mut u8, cboutput: u32, pcbresult: *mut u32, dwflags: u32) -> windows_core::HRESULT>;
pub type NCryptImportKeyFn = Option<unsafe extern "system" fn(hprovider: NCRYPT_PROV_HANDLE, himportkey: NCRYPT_KEY_HANDLE, pszblobtype: windows_core::PCWSTR, pparameterlist: *const BCryptBufferDesc, phkey: *mut NCRYPT_KEY_HANDLE, pbdata: *const u8, cbdata: u32, dwflags: u32) -> windows_core::HRESULT>;
pub type NCryptIsAlgSupportedFn = Option<unsafe extern "system" fn(hprovider: NCRYPT_PROV_HANDLE, pszalgid: windows_core::PCWSTR, dwflags: u32) -> windows_core::HRESULT>;
pub type NCryptKeyDerivationFn = Option<unsafe extern "system" fn(hprovider: NCRYPT_PROV_HANDLE, hkey: NCRYPT_KEY_HANDLE, pparameterlist: *const BCryptBufferDesc, pbderivedkey: *mut u8, cbderivedkey: u32, pcbresult: *mut u32, dwflags: u32) -> windows_core::HRESULT>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NCryptKeyName {
    pub pszName: windows_core::PWSTR,
    pub pszAlgid: windows_core::PWSTR,
    pub dwLegacyKeySpec: CERT_KEY_SPEC,
    pub dwFlags: u32,
}
pub type NCryptNotifyChangeKeyFn = Option<unsafe extern "system" fn(hprovider: NCRYPT_PROV_HANDLE, phevent: *mut super::super::Foundation::HANDLE, dwflags: u32) -> windows_core::HRESULT>;
pub type NCryptOpenKeyFn = Option<unsafe extern "system" fn(hprovider: NCRYPT_PROV_HANDLE, phkey: *mut NCRYPT_KEY_HANDLE, pszkeyname: windows_core::PCWSTR, dwlegacykeyspec: u32, dwflags: u32) -> windows_core::HRESULT>;
pub type NCryptOpenStorageProviderFn = Option<unsafe extern "system" fn(phprovider: *mut NCRYPT_PROV_HANDLE, pszprovidername: windows_core::PCWSTR, dwflags: u32) -> windows_core::HRESULT>;
pub type NCryptPromptUserFn = Option<unsafe extern "system" fn(hprovider: NCRYPT_PROV_HANDLE, hkey: NCRYPT_KEY_HANDLE, pszoperation: windows_core::PCWSTR, dwflags: u32) -> windows_core::HRESULT>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NCryptProviderName {
    pub pszName: windows_core::PWSTR,
    pub pszComment: windows_core::PWSTR,
}
pub type NCryptSecretAgreementFn = Option<unsafe extern "system" fn(hprovider: NCRYPT_PROV_HANDLE, hprivkey: NCRYPT_KEY_HANDLE, hpubkey: NCRYPT_KEY_HANDLE, phagreedsecret: *mut NCRYPT_SECRET_HANDLE, dwflags: u32) -> windows_core::HRESULT>;
pub type NCryptSetKeyPropertyFn = Option<unsafe extern "system" fn(hprovider: NCRYPT_PROV_HANDLE, hkey: NCRYPT_KEY_HANDLE, pszproperty: windows_core::PCWSTR, pbinput: *const u8, cbinput: u32, dwflags: u32) -> windows_core::HRESULT>;
pub type NCryptSetProviderPropertyFn = Option<unsafe extern "system" fn(hprovider: NCRYPT_PROV_HANDLE, pszproperty: windows_core::PCWSTR, pbinput: *const u8, cbinput: u32, dwflags: u32) -> windows_core::HRESULT>;
pub type NCryptSignHashFn = Option<unsafe extern "system" fn(hprovider: NCRYPT_PROV_HANDLE, hkey: NCRYPT_KEY_HANDLE, ppaddinginfo: *const core::ffi::c_void, pbhashvalue: *const u8, cbhashvalue: u32, pbsignature: *mut u8, cbsignature: u32, pcbresult: *mut u32, dwflags: u32) -> windows_core::HRESULT>;
pub type NCryptVerifyClaimFn = Option<unsafe extern "system" fn(hprov: NCRYPT_PROV_HANDLE, hsubjectkey: NCRYPT_KEY_HANDLE, hauthoritykey: NCRYPT_KEY_HANDLE, dwclaimtype: u32, pparameterlist: *const BCryptBufferDesc, pbclaimblob: *const u8, cbclaimblob: u32, poutput: *mut BCryptBufferDesc, dwflags: u32) -> windows_core::HRESULT>;
pub type NCryptVerifySignatureFn = Option<unsafe extern "system" fn(hprovider: NCRYPT_PROV_HANDLE, hkey: NCRYPT_KEY_HANDLE, ppaddinginfo: *const core::ffi::c_void, pbhashvalue: *const u8, cbhashvalue: u32, pbsignature: *const u8, cbsignature: u32, dwflags: u32) -> windows_core::HRESULT>;
pub const NETSCAPE_SIGN_CA_CERT_TYPE: u32 = 1u32;
pub const NETSCAPE_SIGN_CERT_TYPE: u32 = 16u32;
pub const NETSCAPE_SMIME_CA_CERT_TYPE: u32 = 2u32;
pub const NETSCAPE_SMIME_CERT_TYPE: u32 = 32u32;
pub const NETSCAPE_SSL_CA_CERT_TYPE: u32 = 4u32;
pub const NETSCAPE_SSL_CLIENT_AUTH_CERT_TYPE: u32 = 128u32;
pub const NETSCAPE_SSL_SERVER_AUTH_CERT_TYPE: u32 = 64u32;
pub const NonRepudiationPin: SECRET_PURPOSE = SECRET_PURPOSE(3i32);
pub const OCSP_BASIC_BY_KEY_RESPONDER_ID: u32 = 2u32;
pub const OCSP_BASIC_BY_NAME_RESPONDER_ID: u32 = 1u32;
pub const OCSP_BASIC_GOOD_CERT_STATUS: u32 = 0u32;
pub const OCSP_BASIC_RESPONSE: windows_core::PCSTR = windows_core::PCSTR(69i32 as _);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct OCSP_BASIC_RESPONSE_ENTRY {
    pub CertId: OCSP_CERT_ID,
    pub dwCertStatus: u32,
    pub Anonymous: OCSP_BASIC_RESPONSE_ENTRY_0,
    pub ThisUpdate: super::super::Foundation::FILETIME,
    pub NextUpdate: super::super::Foundation::FILETIME,
    pub cExtension: u32,
    pub rgExtension: *mut CERT_EXTENSION,
}
impl Default for OCSP_BASIC_RESPONSE_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union OCSP_BASIC_RESPONSE_ENTRY_0 {
    pub pRevokedInfo: *mut OCSP_BASIC_REVOKED_INFO,
}
impl Default for OCSP_BASIC_RESPONSE_ENTRY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct OCSP_BASIC_RESPONSE_INFO {
    pub dwVersion: u32,
    pub dwResponderIdChoice: u32,
    pub Anonymous: OCSP_BASIC_RESPONSE_INFO_0,
    pub ProducedAt: super::super::Foundation::FILETIME,
    pub cResponseEntry: u32,
    pub rgResponseEntry: *mut OCSP_BASIC_RESPONSE_ENTRY,
    pub cExtension: u32,
    pub rgExtension: *mut CERT_EXTENSION,
}
impl Default for OCSP_BASIC_RESPONSE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union OCSP_BASIC_RESPONSE_INFO_0 {
    pub ByNameResponderId: CRYPT_INTEGER_BLOB,
    pub ByKeyResponderId: CRYPT_INTEGER_BLOB,
}
impl Default for OCSP_BASIC_RESPONSE_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const OCSP_BASIC_RESPONSE_V1: u32 = 0u32;
pub const OCSP_BASIC_REVOKED_CERT_STATUS: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct OCSP_BASIC_REVOKED_INFO {
    pub RevocationDate: super::super::Foundation::FILETIME,
    pub dwCrlReasonCode: CERT_REVOCATION_STATUS_REASON,
}
pub const OCSP_BASIC_SIGNED_RESPONSE: windows_core::PCSTR = windows_core::PCSTR(68i32 as _);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct OCSP_BASIC_SIGNED_RESPONSE_INFO {
    pub ToBeSigned: CRYPT_INTEGER_BLOB,
    pub SignatureInfo: OCSP_SIGNATURE_INFO,
}
pub const OCSP_BASIC_UNKNOWN_CERT_STATUS: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct OCSP_CERT_ID {
    pub HashAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub IssuerNameHash: CRYPT_INTEGER_BLOB,
    pub IssuerKeyHash: CRYPT_INTEGER_BLOB,
    pub SerialNumber: CRYPT_INTEGER_BLOB,
}
pub const OCSP_INTERNAL_ERROR_RESPONSE: u32 = 2u32;
pub const OCSP_MALFORMED_REQUEST_RESPONSE: u32 = 1u32;
pub const OCSP_REQUEST: windows_core::PCSTR = windows_core::PCSTR(66i32 as _);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OCSP_REQUEST_ENTRY {
    pub CertId: OCSP_CERT_ID,
    pub cExtension: u32,
    pub rgExtension: *mut CERT_EXTENSION,
}
impl Default for OCSP_REQUEST_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OCSP_REQUEST_INFO {
    pub dwVersion: u32,
    pub pRequestorName: *mut CERT_ALT_NAME_ENTRY,
    pub cRequestEntry: u32,
    pub rgRequestEntry: *mut OCSP_REQUEST_ENTRY,
    pub cExtension: u32,
    pub rgExtension: *mut CERT_EXTENSION,
}
impl Default for OCSP_REQUEST_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const OCSP_REQUEST_V1: u32 = 0u32;
pub const OCSP_RESPONSE: windows_core::PCSTR = windows_core::PCSTR(67i32 as _);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct OCSP_RESPONSE_INFO {
    pub dwStatus: u32,
    pub pszObjId: windows_core::PSTR,
    pub Value: CRYPT_INTEGER_BLOB,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OCSP_SIGNATURE_INFO {
    pub SignatureAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub Signature: CRYPT_BIT_BLOB,
    pub cCertEncoded: u32,
    pub rgCertEncoded: *mut CRYPT_INTEGER_BLOB,
}
impl Default for OCSP_SIGNATURE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const OCSP_SIGNED_REQUEST: windows_core::PCSTR = windows_core::PCSTR(65i32 as _);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OCSP_SIGNED_REQUEST_INFO {
    pub ToBeSigned: CRYPT_INTEGER_BLOB,
    pub pOptionalSignatureInfo: *mut OCSP_SIGNATURE_INFO,
}
impl Default for OCSP_SIGNED_REQUEST_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const OCSP_SIG_REQUIRED_RESPONSE: u32 = 5u32;
pub const OCSP_SUCCESSFUL_RESPONSE: u32 = 0u32;
pub const OCSP_TRY_LATER_RESPONSE: u32 = 3u32;
pub const OCSP_UNAUTHORIZED_RESPONSE: u32 = 6u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OFFLOAD_PRIVATE_KEY {
    pub dwVersion: u32,
    pub cbPrime1: u32,
    pub cbPrime2: u32,
    pub pbPrime1: *mut u8,
    pub pbPrime2: *mut u8,
}
impl Default for OFFLOAD_PRIVATE_KEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const OPAQUEKEYBLOB: u32 = 9u32;
pub type PCRYPT_DECRYPT_PRIVATE_KEY_FUNC = Option<unsafe extern "system" fn(algorithm: CRYPT_ALGORITHM_IDENTIFIER, encryptedprivatekey: CRYPT_INTEGER_BLOB, pbcleartextkey: *mut u8, pcbcleartextkey: *mut u32, pvoiddecryptfunc: *const core::ffi::c_void) -> windows_core::BOOL>;
pub type PCRYPT_ENCRYPT_PRIVATE_KEY_FUNC = Option<unsafe extern "system" fn(palgorithm: *mut CRYPT_ALGORITHM_IDENTIFIER, pcleartextprivatekey: *const CRYPT_INTEGER_BLOB, pbencryptedkey: *mut u8, pcbencryptedkey: *mut u32, pvoidencryptfunc: *const core::ffi::c_void) -> windows_core::BOOL>;
pub type PCRYPT_RESOLVE_HCRYPTPROV_FUNC = Option<unsafe extern "system" fn(pprivatekeyinfo: *mut CRYPT_PRIVATE_KEY_INFO, phcryptprov: *mut usize, pvoidresolvefunc: *mut core::ffi::c_void) -> windows_core::BOOL>;
pub type PFNCryptStreamOutputCallback = Option<unsafe extern "system" fn(pvcallbackctxt: *const core::ffi::c_void, pbdata: *const u8, cbdata: usize, ffinal: windows_core::BOOL) -> windows_core::HRESULT>;
pub type PFNCryptStreamOutputCallbackEx = Option<unsafe extern "system" fn(pvcallbackctxt: *const core::ffi::c_void, pbdata: *const u8, cbdata: usize, hdescriptor: super::NCRYPT_DESCRIPTOR_HANDLE, ffinal: windows_core::BOOL) -> windows_core::HRESULT>;
pub type PFN_AUTHENTICODE_DIGEST_SIGN = Option<unsafe extern "system" fn(psigningcert: *const CERT_CONTEXT, pmetadatablob: *const CRYPT_INTEGER_BLOB, digestalgid: ALG_ID, pbtobesigneddigest: *const u8, cbtobesigneddigest: u32, psigneddigest: *mut CRYPT_INTEGER_BLOB) -> windows_core::HRESULT>;
pub type PFN_AUTHENTICODE_DIGEST_SIGN_EX = Option<unsafe extern "system" fn(pmetadatablob: *const CRYPT_INTEGER_BLOB, digestalgid: ALG_ID, pbtobesigneddigest: *const u8, cbtobesigneddigest: u32, psigneddigest: *mut CRYPT_INTEGER_BLOB, ppsignercert: *mut *mut CERT_CONTEXT, hcertchainstore: HCERTSTORE) -> windows_core::HRESULT>;
pub type PFN_AUTHENTICODE_DIGEST_SIGN_EX_WITHFILEHANDLE = Option<unsafe extern "system" fn(pmetadatablob: *const CRYPT_INTEGER_BLOB, digestalgid: ALG_ID, pbtobesigneddigest: *const u8, cbtobesigneddigest: u32, hfile: super::super::Foundation::HANDLE, psigneddigest: *mut CRYPT_INTEGER_BLOB, ppsignercert: *mut *mut CERT_CONTEXT, hcertchainstore: HCERTSTORE) -> windows_core::HRESULT>;
pub type PFN_AUTHENTICODE_DIGEST_SIGN_WITHFILEHANDLE = Option<unsafe extern "system" fn(psigningcert: *const CERT_CONTEXT, pmetadatablob: *const CRYPT_INTEGER_BLOB, digestalgid: ALG_ID, pbtobesigneddigest: *const u8, cbtobesigneddigest: u32, hfile: super::super::Foundation::HANDLE, psigneddigest: *mut CRYPT_INTEGER_BLOB) -> windows_core::HRESULT>;
pub type PFN_CANCEL_ASYNC_RETRIEVAL_FUNC = Option<unsafe extern "system" fn(hasyncretrieve: HCRYPTASYNC) -> windows_core::BOOL>;
pub type PFN_CARD_ACQUIRE_CONTEXT = Option<unsafe extern "system" fn(pcarddata: *mut CARD_DATA, dwflags: u32) -> u32>;
pub type PFN_CARD_AUTHENTICATE_CHALLENGE = Option<unsafe extern "system" fn(pcarddata: *const CARD_DATA, pbresponsedata: *const u8, cbresponsedata: u32, pcattemptsremaining: *mut u32) -> u32>;
pub type PFN_CARD_AUTHENTICATE_EX = Option<unsafe extern "system" fn(pcarddata: *const CARD_DATA, pinid: u32, dwflags: u32, pbpindata: *const u8, cbpindata: u32, ppbsessionpin: *mut *mut u8, pcbsessionpin: *mut u32, pcattemptsremaining: *mut u32) -> u32>;
pub type PFN_CARD_AUTHENTICATE_PIN = Option<unsafe extern "system" fn(pcarddata: *const CARD_DATA, pwszuserid: windows_core::PCWSTR, pbpin: *const u8, cbpin: u32, pcattemptsremaining: *mut u32) -> u32>;
pub type PFN_CARD_CHANGE_AUTHENTICATOR = Option<unsafe extern "system" fn(pcarddata: *const CARD_DATA, pwszuserid: windows_core::PCWSTR, pbcurrentauthenticator: *const u8, cbcurrentauthenticator: u32, pbnewauthenticator: *const u8, cbnewauthenticator: u32, cretrycount: u32, dwflags: u32, pcattemptsremaining: *mut u32) -> u32>;
pub type PFN_CARD_CHANGE_AUTHENTICATOR_EX = Option<unsafe extern "system" fn(pcarddata: *const CARD_DATA, dwflags: u32, dwauthenticatingpinid: u32, pbauthenticatingpindata: *const u8, cbauthenticatingpindata: u32, dwtargetpinid: u32, pbtargetdata: *const u8, cbtargetdata: u32, cretrycount: u32, pcattemptsremaining: *mut u32) -> u32>;
pub type PFN_CARD_CONSTRUCT_DH_AGREEMENT = Option<unsafe extern "system" fn(pcarddata: *const CARD_DATA, pagreementinfo: *mut CARD_DH_AGREEMENT_INFO) -> u32>;
pub type PFN_CARD_CREATE_CONTAINER = Option<unsafe extern "system" fn(pcarddata: *const CARD_DATA, bcontainerindex: u8, dwflags: u32, dwkeyspec: u32, dwkeysize: u32, pbkeydata: *const u8) -> u32>;
pub type PFN_CARD_CREATE_CONTAINER_EX = Option<unsafe extern "system" fn(pcarddata: *const CARD_DATA, bcontainerindex: u8, dwflags: u32, dwkeyspec: u32, dwkeysize: u32, pbkeydata: *const u8, pinid: u32) -> u32>;
pub type PFN_CARD_CREATE_DIRECTORY = Option<unsafe extern "system" fn(pcarddata: *const CARD_DATA, pszdirectoryname: windows_core::PCSTR, accesscondition: CARD_DIRECTORY_ACCESS_CONDITION) -> u32>;
pub type PFN_CARD_CREATE_FILE = Option<unsafe extern "system" fn(pcarddata: *const CARD_DATA, pszdirectoryname: windows_core::PCSTR, pszfilename: windows_core::PCSTR, cbinitialcreationsize: u32, accesscondition: CARD_FILE_ACCESS_CONDITION) -> u32>;
pub type PFN_CARD_DEAUTHENTICATE = Option<unsafe extern "system" fn(pcarddata: *const CARD_DATA, pwszuserid: windows_core::PCWSTR, dwflags: u32) -> u32>;
pub type PFN_CARD_DEAUTHENTICATE_EX = Option<unsafe extern "system" fn(pcarddata: *const CARD_DATA, pinid: u32, dwflags: u32) -> u32>;
pub type PFN_CARD_DELETE_CONTAINER = Option<unsafe extern "system" fn(pcarddata: *const CARD_DATA, bcontainerindex: u8, dwreserved: u32) -> u32>;
pub type PFN_CARD_DELETE_CONTEXT = Option<unsafe extern "system" fn(pcarddata: *mut CARD_DATA) -> u32>;
pub type PFN_CARD_DELETE_DIRECTORY = Option<unsafe extern "system" fn(pcarddata: *const CARD_DATA, pszdirectoryname: windows_core::PCSTR) -> u32>;
pub type PFN_CARD_DELETE_FILE = Option<unsafe extern "system" fn(pcarddata: *const CARD_DATA, pszdirectoryname: windows_core::PCSTR, pszfilename: windows_core::PCSTR, dwflags: u32) -> u32>;
pub type PFN_CARD_DERIVE_KEY = Option<unsafe extern "system" fn(pcarddata: *const CARD_DATA, pagreementinfo: *mut CARD_DERIVE_KEY) -> u32>;
pub type PFN_CARD_DESTROY_DH_AGREEMENT = Option<unsafe extern "system" fn(pcarddata: *const CARD_DATA, bsecretagreementindex: u8, dwflags: u32) -> u32>;
pub type PFN_CARD_DESTROY_KEY = Option<unsafe extern "system" fn(pcarddata: *const CARD_DATA, hkey: usize) -> u32>;
pub type PFN_CARD_ENUM_FILES = Option<unsafe extern "system" fn(pcarddata: *const CARD_DATA, pszdirectoryname: windows_core::PCSTR, pmszfilenames: *mut windows_core::PSTR, pdwcbfilename: *mut u32, dwflags: u32) -> u32>;
pub type PFN_CARD_GET_ALGORITHM_PROPERTY = Option<unsafe extern "system" fn(pcarddata: *const CARD_DATA, pwszalgid: windows_core::PCWSTR, pwszproperty: windows_core::PCWSTR, pbdata: *mut u8, cbdata: u32, pdwdatalen: *mut u32, dwflags: u32) -> u32>;
pub type PFN_CARD_GET_CHALLENGE = Option<unsafe extern "system" fn(pcarddata: *const CARD_DATA, ppbchallengedata: *mut *mut u8, pcbchallengedata: *mut u32) -> u32>;
pub type PFN_CARD_GET_CHALLENGE_EX = Option<unsafe extern "system" fn(pcarddata: *const CARD_DATA, pinid: u32, ppbchallengedata: *mut *mut u8, pcbchallengedata: *mut u32, dwflags: u32) -> u32>;
pub type PFN_CARD_GET_CONTAINER_INFO = Option<unsafe extern "system" fn(pcarddata: *const CARD_DATA, bcontainerindex: u8, dwflags: u32, pcontainerinfo: *mut CONTAINER_INFO) -> u32>;
pub type PFN_CARD_GET_CONTAINER_PROPERTY = Option<unsafe extern "system" fn(pcarddata: *const CARD_DATA, bcontainerindex: u8, wszproperty: windows_core::PCWSTR, pbdata: *mut u8, cbdata: u32, pdwdatalen: *mut u32, dwflags: u32) -> u32>;
pub type PFN_CARD_GET_FILE_INFO = Option<unsafe extern "system" fn(pcarddata: *const CARD_DATA, pszdirectoryname: windows_core::PCSTR, pszfilename: windows_core::PCSTR, pcardfileinfo: *mut CARD_FILE_INFO) -> u32>;
pub type PFN_CARD_GET_KEY_PROPERTY = Option<unsafe extern "system" fn(pcarddata: *const CARD_DATA, hkey: usize, pwszproperty: windows_core::PCWSTR, pbdata: *mut u8, cbdata: u32, pdwdatalen: *mut u32, dwflags: u32) -> u32>;
pub type PFN_CARD_GET_PROPERTY = Option<unsafe extern "system" fn(pcarddata: *const CARD_DATA, wszproperty: windows_core::PCWSTR, pbdata: *mut u8, cbdata: u32, pdwdatalen: *mut u32, dwflags: u32) -> u32>;
pub type PFN_CARD_GET_SHARED_KEY_HANDLE = Option<unsafe extern "system" fn(pcarddata: *const CARD_DATA, pbinput: *const u8, cbinput: u32, ppboutput: *mut *mut u8, pcboutput: *mut u32, phkey: *mut usize) -> u32>;
pub type PFN_CARD_IMPORT_SESSION_KEY = Option<unsafe extern "system" fn(pcarddata: *const CARD_DATA, bcontainerindex: u8, ppaddinginfo: *const core::ffi::c_void, pwszblobtype: windows_core::PCWSTR, pwszalgid: windows_core::PCWSTR, phkey: *mut usize, pbinput: *const u8, cbinput: u32, dwflags: u32) -> u32>;
pub type PFN_CARD_PROCESS_ENCRYPTED_DATA = Option<unsafe extern "system" fn(pcarddata: *const CARD_DATA, hkey: usize, pwszsecurefunction: windows_core::PCWSTR, pencrypteddata: *const CARD_ENCRYPTED_DATA, cencrypteddata: u32, pboutput: *mut u8, cboutput: u32, pdwoutputlen: *mut u32, dwflags: u32) -> u32>;
pub type PFN_CARD_QUERY_CAPABILITIES = Option<unsafe extern "system" fn(pcarddata: *const CARD_DATA, pcardcapabilities: *mut CARD_CAPABILITIES) -> u32>;
pub type PFN_CARD_QUERY_FREE_SPACE = Option<unsafe extern "system" fn(pcarddata: *const CARD_DATA, dwflags: u32, pcardfreespaceinfo: *mut CARD_FREE_SPACE_INFO) -> u32>;
pub type PFN_CARD_QUERY_KEY_SIZES = Option<unsafe extern "system" fn(pcarddata: *const CARD_DATA, dwkeyspec: u32, dwflags: u32, pkeysizes: *mut CARD_KEY_SIZES) -> u32>;
pub type PFN_CARD_READ_FILE = Option<unsafe extern "system" fn(pcarddata: *const CARD_DATA, pszdirectoryname: windows_core::PCSTR, pszfilename: windows_core::PCSTR, dwflags: u32, ppbdata: *mut *mut u8, pcbdata: *mut u32) -> u32>;
pub type PFN_CARD_RSA_DECRYPT = Option<unsafe extern "system" fn(pcarddata: *const CARD_DATA, pinfo: *mut CARD_RSA_DECRYPT_INFO) -> u32>;
pub type PFN_CARD_SET_CONTAINER_PROPERTY = Option<unsafe extern "system" fn(pcarddata: *const CARD_DATA, bcontainerindex: u8, wszproperty: windows_core::PCWSTR, pbdata: *const u8, cbdatalen: u32, dwflags: u32) -> u32>;
pub type PFN_CARD_SET_KEY_PROPERTY = Option<unsafe extern "system" fn(pcarddata: *const CARD_DATA, hkey: usize, pwszproperty: windows_core::PCWSTR, pbinput: *const u8, cbinput: u32, dwflags: u32) -> u32>;
pub type PFN_CARD_SET_PROPERTY = Option<unsafe extern "system" fn(pcarddata: *const CARD_DATA, wszproperty: windows_core::PCWSTR, pbdata: *const u8, cbdatalen: u32, dwflags: u32) -> u32>;
pub type PFN_CARD_SIGN_DATA = Option<unsafe extern "system" fn(pcarddata: *const CARD_DATA, pinfo: *mut CARD_SIGNING_INFO) -> u32>;
pub type PFN_CARD_UNBLOCK_PIN = Option<unsafe extern "system" fn(pcarddata: *const CARD_DATA, pwszuserid: windows_core::PCWSTR, pbauthenticationdata: *const u8, cbauthenticationdata: u32, pbnewpindata: *const u8, cbnewpindata: u32, cretrycount: u32, dwflags: u32) -> u32>;
pub type PFN_CARD_WRITE_FILE = Option<unsafe extern "system" fn(pcarddata: *const CARD_DATA, pszdirectoryname: windows_core::PCSTR, pszfilename: windows_core::PCSTR, dwflags: u32, pbdata: *const u8, cbdata: u32) -> u32>;
pub type PFN_CERT_CHAIN_FIND_BY_ISSUER_CALLBACK = Option<unsafe extern "system" fn(pcert: *const CERT_CONTEXT, pvfindarg: *mut core::ffi::c_void) -> windows_core::BOOL>;
pub type PFN_CERT_CREATE_CONTEXT_SORT_FUNC = Option<unsafe extern "system" fn(cbtotalencoded: u32, cbremainencoded: u32, centry: u32, pvsort: *mut core::ffi::c_void) -> windows_core::BOOL>;
pub type PFN_CERT_DLL_OPEN_STORE_PROV_FUNC = Option<unsafe extern "system" fn(lpszstoreprovider: windows_core::PCSTR, dwencodingtype: CERT_QUERY_ENCODING_TYPE, hcryptprov: HCRYPTPROV_LEGACY, dwflags: CERT_OPEN_STORE_FLAGS, pvpara: *const core::ffi::c_void, hcertstore: HCERTSTORE, pstoreprovinfo: *mut CERT_STORE_PROV_INFO) -> windows_core::BOOL>;
pub type PFN_CERT_ENUM_PHYSICAL_STORE = Option<unsafe extern "system" fn(pvsystemstore: *const core::ffi::c_void, dwflags: u32, pwszstorename: windows_core::PCWSTR, pstoreinfo: *const CERT_PHYSICAL_STORE_INFO, pvreserved: *const core::ffi::c_void, pvarg: *mut core::ffi::c_void) -> windows_core::BOOL>;
pub type PFN_CERT_ENUM_SYSTEM_STORE = Option<unsafe extern "system" fn(pvsystemstore: *const core::ffi::c_void, dwflags: CERT_SYSTEM_STORE_FLAGS, pstoreinfo: *const CERT_SYSTEM_STORE_INFO, pvreserved: *const core::ffi::c_void, pvarg: *mut core::ffi::c_void) -> windows_core::BOOL>;
pub type PFN_CERT_ENUM_SYSTEM_STORE_LOCATION = Option<unsafe extern "system" fn(pwszstorelocation: windows_core::PCWSTR, dwflags: u32, pvreserved: *const core::ffi::c_void, pvarg: *mut core::ffi::c_void) -> windows_core::BOOL>;
pub type PFN_CERT_IS_WEAK_HASH = Option<unsafe extern "system" fn(dwhashusetype: u32, pwszcnghashalgid: windows_core::PCWSTR, dwchainflags: u32, psignerchaincontext: *const CERT_CHAIN_CONTEXT, ptimestamp: *const super::super::Foundation::FILETIME, pwszfilename: windows_core::PCWSTR) -> windows_core::BOOL>;
pub type PFN_CERT_SERVER_OCSP_RESPONSE_UPDATE_CALLBACK = Option<unsafe extern "system" fn(pchaincontext: *const CERT_CHAIN_CONTEXT, pserverocspresponsecontext: *const CERT_SERVER_OCSP_RESPONSE_CONTEXT, pnewcrlcontext: *const CRL_CONTEXT, pprevcrlcontext: *const CRL_CONTEXT, pvarg: *mut core::ffi::c_void, dwwriteocspfileerror: u32)>;
pub type PFN_CERT_STORE_PROV_CLOSE = Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, dwflags: u32)>;
pub type PFN_CERT_STORE_PROV_CONTROL = Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, dwflags: u32, dwctrltype: u32, pvctrlpara: *const core::ffi::c_void) -> windows_core::BOOL>;
pub type PFN_CERT_STORE_PROV_DELETE_CERT = Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, pcertcontext: *const CERT_CONTEXT, dwflags: u32) -> windows_core::BOOL>;
pub type PFN_CERT_STORE_PROV_DELETE_CRL = Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, pcrlcontext: *const CRL_CONTEXT, dwflags: u32) -> windows_core::BOOL>;
pub type PFN_CERT_STORE_PROV_DELETE_CTL = Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, pctlcontext: *const CTL_CONTEXT, dwflags: u32) -> windows_core::BOOL>;
pub type PFN_CERT_STORE_PROV_FIND_CERT = Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, pfindinfo: *const CERT_STORE_PROV_FIND_INFO, pprevcertcontext: *const CERT_CONTEXT, dwflags: u32, ppvstoreprovfindinfo: *mut *mut core::ffi::c_void, ppprovcertcontext: *mut *mut CERT_CONTEXT) -> windows_core::BOOL>;
pub type PFN_CERT_STORE_PROV_FIND_CRL = Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, pfindinfo: *const CERT_STORE_PROV_FIND_INFO, pprevcrlcontext: *const CRL_CONTEXT, dwflags: u32, ppvstoreprovfindinfo: *mut *mut core::ffi::c_void, ppprovcrlcontext: *mut *mut CRL_CONTEXT) -> windows_core::BOOL>;
pub type PFN_CERT_STORE_PROV_FIND_CTL = Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, pfindinfo: *const CERT_STORE_PROV_FIND_INFO, pprevctlcontext: *const CTL_CONTEXT, dwflags: u32, ppvstoreprovfindinfo: *mut *mut core::ffi::c_void, ppprovctlcontext: *mut *mut CTL_CONTEXT) -> windows_core::BOOL>;
pub type PFN_CERT_STORE_PROV_FREE_FIND_CERT = Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, pcertcontext: *const CERT_CONTEXT, pvstoreprovfindinfo: *const core::ffi::c_void, dwflags: u32) -> windows_core::BOOL>;
pub type PFN_CERT_STORE_PROV_FREE_FIND_CRL = Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, pcrlcontext: *const CRL_CONTEXT, pvstoreprovfindinfo: *const core::ffi::c_void, dwflags: u32) -> windows_core::BOOL>;
pub type PFN_CERT_STORE_PROV_FREE_FIND_CTL = Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, pctlcontext: *const CTL_CONTEXT, pvstoreprovfindinfo: *const core::ffi::c_void, dwflags: u32) -> windows_core::BOOL>;
pub type PFN_CERT_STORE_PROV_GET_CERT_PROPERTY = Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, pcertcontext: *const CERT_CONTEXT, dwpropid: u32, dwflags: u32, pvdata: *mut core::ffi::c_void, pcbdata: *mut u32) -> windows_core::BOOL>;
pub type PFN_CERT_STORE_PROV_GET_CRL_PROPERTY = Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, pcrlcontext: *const CRL_CONTEXT, dwpropid: u32, dwflags: u32, pvdata: *mut core::ffi::c_void, pcbdata: *mut u32) -> windows_core::BOOL>;
pub type PFN_CERT_STORE_PROV_GET_CTL_PROPERTY = Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, pctlcontext: *const CTL_CONTEXT, dwpropid: u32, dwflags: u32, pvdata: *mut core::ffi::c_void, pcbdata: *mut u32) -> windows_core::BOOL>;
pub type PFN_CERT_STORE_PROV_READ_CERT = Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, pstorecertcontext: *const CERT_CONTEXT, dwflags: u32, ppprovcertcontext: *mut *mut CERT_CONTEXT) -> windows_core::BOOL>;
pub type PFN_CERT_STORE_PROV_READ_CRL = Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, pstorecrlcontext: *const CRL_CONTEXT, dwflags: u32, ppprovcrlcontext: *mut *mut CRL_CONTEXT) -> windows_core::BOOL>;
pub type PFN_CERT_STORE_PROV_READ_CTL = Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, pstorectlcontext: *const CTL_CONTEXT, dwflags: u32, ppprovctlcontext: *mut *mut CTL_CONTEXT) -> windows_core::BOOL>;
pub type PFN_CERT_STORE_PROV_SET_CERT_PROPERTY = Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, pcertcontext: *const CERT_CONTEXT, dwpropid: u32, dwflags: u32, pvdata: *const core::ffi::c_void) -> windows_core::BOOL>;
pub type PFN_CERT_STORE_PROV_SET_CRL_PROPERTY = Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, pcrlcontext: *const CRL_CONTEXT, dwpropid: u32, dwflags: u32, pvdata: *const core::ffi::c_void) -> windows_core::BOOL>;
pub type PFN_CERT_STORE_PROV_SET_CTL_PROPERTY = Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, pctlcontext: *const CTL_CONTEXT, dwpropid: u32, dwflags: u32, pvdata: *const core::ffi::c_void) -> windows_core::BOOL>;
pub type PFN_CERT_STORE_PROV_WRITE_CERT = Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, pcertcontext: *const CERT_CONTEXT, dwflags: u32) -> windows_core::BOOL>;
pub type PFN_CERT_STORE_PROV_WRITE_CRL = Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, pcrlcontext: *const CRL_CONTEXT, dwflags: u32) -> windows_core::BOOL>;
pub type PFN_CERT_STORE_PROV_WRITE_CTL = Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, pctlcontext: *const CTL_CONTEXT, dwflags: u32) -> windows_core::BOOL>;
pub type PFN_CMSG_ALLOC = Option<unsafe extern "system" fn(cb: usize) -> *mut core::ffi::c_void>;
pub type PFN_CMSG_CNG_IMPORT_CONTENT_ENCRYPT_KEY = Option<unsafe extern "system" fn(pcngcontentdecryptinfo: *mut CMSG_CNG_CONTENT_DECRYPT_INFO, dwflags: u32, pvreserved: *const core::ffi::c_void) -> windows_core::BOOL>;
pub type PFN_CMSG_CNG_IMPORT_KEY_AGREE = Option<unsafe extern "system" fn(pcngcontentdecryptinfo: *mut CMSG_CNG_CONTENT_DECRYPT_INFO, pkeyagreedecryptpara: *const CMSG_CTRL_KEY_AGREE_DECRYPT_PARA, dwflags: u32, pvreserved: *const core::ffi::c_void) -> windows_core::BOOL>;
pub type PFN_CMSG_CNG_IMPORT_KEY_TRANS = Option<unsafe extern "system" fn(pcngcontentdecryptinfo: *mut CMSG_CNG_CONTENT_DECRYPT_INFO, pkeytransdecryptpara: *const CMSG_CTRL_KEY_TRANS_DECRYPT_PARA, dwflags: u32, pvreserved: *const core::ffi::c_void) -> windows_core::BOOL>;
pub type PFN_CMSG_EXPORT_ENCRYPT_KEY = Option<unsafe extern "system" fn(hcryptprov: usize, hencryptkey: usize, ppublickeyinfo: *const CERT_PUBLIC_KEY_INFO, pbdata: *mut u8, pcbdata: *mut u32) -> windows_core::BOOL>;
pub type PFN_CMSG_EXPORT_KEY_AGREE = Option<unsafe extern "system" fn(pcontentencryptinfo: *const CMSG_CONTENT_ENCRYPT_INFO, pkeyagreeencodeinfo: *const CMSG_KEY_AGREE_RECIPIENT_ENCODE_INFO, pkeyagreeencryptinfo: *mut CMSG_KEY_AGREE_ENCRYPT_INFO, dwflags: u32, pvreserved: *const core::ffi::c_void) -> windows_core::BOOL>;
pub type PFN_CMSG_EXPORT_KEY_TRANS = Option<unsafe extern "system" fn(pcontentencryptinfo: *const CMSG_CONTENT_ENCRYPT_INFO, pkeytransencodeinfo: *const CMSG_KEY_TRANS_RECIPIENT_ENCODE_INFO, pkeytransencryptinfo: *mut CMSG_KEY_TRANS_ENCRYPT_INFO, dwflags: u32, pvreserved: *const core::ffi::c_void) -> windows_core::BOOL>;
pub type PFN_CMSG_EXPORT_MAIL_LIST = Option<unsafe extern "system" fn(pcontentencryptinfo: *const CMSG_CONTENT_ENCRYPT_INFO, pmaillistencodeinfo: *const CMSG_MAIL_LIST_RECIPIENT_ENCODE_INFO, pmaillistencryptinfo: *mut CMSG_MAIL_LIST_ENCRYPT_INFO, dwflags: u32, pvreserved: *const core::ffi::c_void) -> windows_core::BOOL>;
pub type PFN_CMSG_FREE = Option<unsafe extern "system" fn(pv: *mut core::ffi::c_void)>;
pub type PFN_CMSG_GEN_CONTENT_ENCRYPT_KEY = Option<unsafe extern "system" fn(pcontentencryptinfo: *mut CMSG_CONTENT_ENCRYPT_INFO, dwflags: u32, pvreserved: *const core::ffi::c_void) -> windows_core::BOOL>;
pub type PFN_CMSG_GEN_ENCRYPT_KEY = Option<unsafe extern "system" fn(phcryptprov: *mut usize, paiencrypt: *const CRYPT_ALGORITHM_IDENTIFIER, pvencryptauxinfo: *const core::ffi::c_void, ppublickeyinfo: *const CERT_PUBLIC_KEY_INFO, pfnalloc: PFN_CMSG_ALLOC, phencryptkey: *mut usize, ppbencryptparameters: *mut *mut u8, pcbencryptparameters: *mut u32) -> windows_core::BOOL>;
pub type PFN_CMSG_IMPORT_ENCRYPT_KEY = Option<unsafe extern "system" fn(hcryptprov: usize, dwkeyspec: u32, paiencrypt: *const CRYPT_ALGORITHM_IDENTIFIER, paipubkey: *const CRYPT_ALGORITHM_IDENTIFIER, pbencodedkey: *const u8, cbencodedkey: u32, phencryptkey: *mut usize) -> windows_core::BOOL>;
pub type PFN_CMSG_IMPORT_KEY_AGREE = Option<unsafe extern "system" fn(pcontentencryptionalgorithm: *const CRYPT_ALGORITHM_IDENTIFIER, pkeyagreedecryptpara: *const CMSG_CTRL_KEY_AGREE_DECRYPT_PARA, dwflags: u32, pvreserved: *const core::ffi::c_void, phcontentencryptkey: *mut usize) -> windows_core::BOOL>;
pub type PFN_CMSG_IMPORT_KEY_TRANS = Option<unsafe extern "system" fn(pcontentencryptionalgorithm: *const CRYPT_ALGORITHM_IDENTIFIER, pkeytransdecryptpara: *const CMSG_CTRL_KEY_TRANS_DECRYPT_PARA, dwflags: u32, pvreserved: *const core::ffi::c_void, phcontentencryptkey: *mut usize) -> windows_core::BOOL>;
pub type PFN_CMSG_IMPORT_MAIL_LIST = Option<unsafe extern "system" fn(pcontentencryptionalgorithm: *const CRYPT_ALGORITHM_IDENTIFIER, pmaillistdecryptpara: *const CMSG_CTRL_MAIL_LIST_DECRYPT_PARA, dwflags: u32, pvreserved: *const core::ffi::c_void, phcontentencryptkey: *mut usize) -> windows_core::BOOL>;
pub type PFN_CMSG_STREAM_OUTPUT = Option<unsafe extern "system" fn(pvarg: *const core::ffi::c_void, pbdata: *const u8, cbdata: u32, ffinal: windows_core::BOOL) -> windows_core::BOOL>;
pub type PFN_CRYPT_ALLOC = Option<unsafe extern "system" fn(cbsize: usize) -> *mut core::ffi::c_void>;
pub type PFN_CRYPT_ASYNC_PARAM_FREE_FUNC = Option<unsafe extern "system" fn(pszparamoid: windows_core::PCSTR, pvparam: *const core::ffi::c_void)>;
pub type PFN_CRYPT_ASYNC_RETRIEVAL_COMPLETION_FUNC = Option<unsafe extern "system" fn(pvcompletion: *mut core::ffi::c_void, dwcompletioncode: u32, pszurl: windows_core::PCSTR, pszobjectoid: windows_core::PCSTR, pvobject: *const core::ffi::c_void)>;
pub type PFN_CRYPT_CANCEL_RETRIEVAL = Option<unsafe extern "system" fn(dwflags: u32, pvarg: *mut core::ffi::c_void) -> windows_core::BOOL>;
pub type PFN_CRYPT_ENUM_KEYID_PROP = Option<unsafe extern "system" fn(pkeyidentifier: *const CRYPT_INTEGER_BLOB, dwflags: u32, pvreserved: *const core::ffi::c_void, pvarg: *mut core::ffi::c_void, cprop: u32, rgdwpropid: *const u32, rgpvdata: *const *const core::ffi::c_void, rgcbdata: *const u32) -> windows_core::BOOL>;
pub type PFN_CRYPT_ENUM_OID_FUNC = Option<unsafe extern "system" fn(dwencodingtype: u32, pszfuncname: windows_core::PCSTR, pszoid: windows_core::PCSTR, cvalue: u32, rgdwvaluetype: *const u32, rgpwszvaluename: *const windows_core::PCWSTR, rgpbvaluedata: *const *const u8, rgcbvaluedata: *const u32, pvarg: *mut core::ffi::c_void) -> windows_core::BOOL>;
pub type PFN_CRYPT_ENUM_OID_INFO = Option<unsafe extern "system" fn(pinfo: *const CRYPT_OID_INFO, pvarg: *mut core::ffi::c_void) -> windows_core::BOOL>;
pub type PFN_CRYPT_EXPORT_PUBLIC_KEY_INFO_EX2_FUNC = Option<unsafe extern "system" fn(hncryptkey: NCRYPT_KEY_HANDLE, dwcertencodingtype: CERT_QUERY_ENCODING_TYPE, pszpublickeyobjid: windows_core::PCSTR, dwflags: u32, pvauxinfo: *const core::ffi::c_void, pinfo: *mut CERT_PUBLIC_KEY_INFO, pcbinfo: *mut u32) -> windows_core::BOOL>;
pub type PFN_CRYPT_EXPORT_PUBLIC_KEY_INFO_FROM_BCRYPT_HANDLE_FUNC = Option<unsafe extern "system" fn(hbcryptkey: BCRYPT_KEY_HANDLE, dwcertencodingtype: CERT_QUERY_ENCODING_TYPE, pszpublickeyobjid: windows_core::PCSTR, dwflags: u32, pvauxinfo: *const core::ffi::c_void, pinfo: *mut CERT_PUBLIC_KEY_INFO, pcbinfo: *mut u32) -> windows_core::BOOL>;
pub type PFN_CRYPT_EXTRACT_ENCODED_SIGNATURE_PARAMETERS_FUNC = Option<unsafe extern "system" fn(dwcertencodingtype: CERT_QUERY_ENCODING_TYPE, psignaturealgorithm: *const CRYPT_ALGORITHM_IDENTIFIER, ppvdecodedsignpara: *mut *mut core::ffi::c_void, ppwszcnghashalgid: *mut windows_core::PWSTR) -> windows_core::BOOL>;
pub type PFN_CRYPT_FREE = Option<unsafe extern "system" fn(pv: *const core::ffi::c_void)>;
pub type PFN_CRYPT_GET_SIGNER_CERTIFICATE = Option<unsafe extern "system" fn(pvgetarg: *mut core::ffi::c_void, dwcertencodingtype: CERT_QUERY_ENCODING_TYPE, psignerid: *const CERT_INFO, hmsgcertstore: HCERTSTORE) -> *mut CERT_CONTEXT>;
pub type PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_FLUSH = Option<unsafe extern "system" fn(pcontext: *const core::ffi::c_void, rgidentifierornamelist: *const *const CRYPT_INTEGER_BLOB, dwidentifierornamelistcount: u32) -> windows_core::BOOL>;
pub type PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_FREE = Option<unsafe extern "system" fn(pplugincontext: *const core::ffi::c_void, pbdata: *const u8)>;
pub type PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_FREE_IDENTIFIER = Option<unsafe extern "system" fn(pplugincontext: *const core::ffi::c_void, pidentifier: *const CRYPT_INTEGER_BLOB)>;
pub type PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_FREE_PASSWORD = Option<unsafe extern "system" fn(pplugincontext: *const core::ffi::c_void, pwszpassword: windows_core::PCWSTR)>;
pub type PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_GET = Option<unsafe extern "system" fn(pplugincontext: *const core::ffi::c_void, pidentifier: *const CRYPT_INTEGER_BLOB, dwnametype: u32, pnameblob: *const CRYPT_INTEGER_BLOB, ppbcontent: *mut *mut u8, pcbcontent: *mut u32, ppwszpassword: *mut windows_core::PCWSTR, ppidentifier: *mut *mut CRYPT_INTEGER_BLOB) -> windows_core::BOOL>;
pub type PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_INITIALIZE = Option<unsafe extern "system" fn(pfnflush: PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_FLUSH, pcontext: *const core::ffi::c_void, pdwexpectedobjectcount: *mut u32, ppfunctable: *mut *mut CRYPT_OBJECT_LOCATOR_PROVIDER_TABLE, ppplugincontext: *mut *mut core::ffi::c_void) -> windows_core::BOOL>;
pub type PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_RELEASE = Option<unsafe extern "system" fn(dwreason: CRYPT_OBJECT_LOCATOR_RELEASE_REASON, pplugincontext: *const core::ffi::c_void)>;
pub type PFN_CRYPT_SIGN_AND_ENCODE_HASH_FUNC = Option<unsafe extern "system" fn(hkey: NCRYPT_KEY_HANDLE, dwcertencodingtype: CERT_QUERY_ENCODING_TYPE, psignaturealgorithm: *const CRYPT_ALGORITHM_IDENTIFIER, pvdecodedsignpara: *const core::ffi::c_void, pwszcngpubkeyalgid: windows_core::PCWSTR, pwszcnghashalgid: windows_core::PCWSTR, pbcomputedhash: *const u8, cbcomputedhash: u32, pbsignature: *mut u8, pcbsignature: *mut u32) -> windows_core::BOOL>;
pub type PFN_CRYPT_VERIFY_ENCODED_SIGNATURE_FUNC = Option<unsafe extern "system" fn(dwcertencodingtype: CERT_QUERY_ENCODING_TYPE, ppubkeyinfo: *const CERT_PUBLIC_KEY_INFO, psignaturealgorithm: *const CRYPT_ALGORITHM_IDENTIFIER, pvdecodedsignpara: *const core::ffi::c_void, pwszcngpubkeyalgid: windows_core::PCWSTR, pwszcnghashalgid: windows_core::PCWSTR, pbcomputedhash: *const u8, cbcomputedhash: u32, pbsignature: *const u8, cbsignature: u32) -> windows_core::BOOL>;
pub type PFN_CRYPT_XML_CREATE_TRANSFORM = Option<unsafe extern "system" fn(ptransform: *const CRYPT_XML_ALGORITHM, pproviderin: *const CRYPT_XML_DATA_PROVIDER, pproviderout: *mut CRYPT_XML_DATA_PROVIDER) -> windows_core::HRESULT>;
pub type PFN_CRYPT_XML_DATA_PROVIDER_CLOSE = Option<unsafe extern "system" fn(pvcallbackstate: *mut core::ffi::c_void) -> windows_core::HRESULT>;
pub type PFN_CRYPT_XML_DATA_PROVIDER_READ = Option<unsafe extern "system" fn(pvcallbackstate: *mut core::ffi::c_void, pbdata: *mut u8, cbdata: u32, pcbread: *mut u32) -> windows_core::HRESULT>;
pub type PFN_CRYPT_XML_ENUM_ALG_INFO = Option<unsafe extern "system" fn(pinfo: *const CRYPT_XML_ALGORITHM_INFO, pvarg: *mut core::ffi::c_void) -> windows_core::BOOL>;
pub type PFN_CRYPT_XML_WRITE_CALLBACK = Option<unsafe extern "system" fn(pvcallbackstate: *mut core::ffi::c_void, pbdata: *const u8, cbdata: u32) -> windows_core::HRESULT>;
pub type PFN_CSP_ALLOC = Option<unsafe extern "system" fn(size: usize) -> *mut core::ffi::c_void>;
pub type PFN_CSP_CACHE_ADD_FILE = Option<unsafe extern "system" fn(pvcachecontext: *const core::ffi::c_void, wsztag: windows_core::PCWSTR, dwflags: u32, pbdata: *const u8, cbdata: u32) -> u32>;
pub type PFN_CSP_CACHE_DELETE_FILE = Option<unsafe extern "system" fn(pvcachecontext: *const core::ffi::c_void, wsztag: windows_core::PCWSTR, dwflags: u32) -> u32>;
pub type PFN_CSP_CACHE_LOOKUP_FILE = Option<unsafe extern "system" fn(pvcachecontext: *const core::ffi::c_void, wsztag: windows_core::PCWSTR, dwflags: u32, ppbdata: *mut *mut u8, pcbdata: *mut u32) -> u32>;
pub type PFN_CSP_FREE = Option<unsafe extern "system" fn(address: *const core::ffi::c_void)>;
pub type PFN_CSP_GET_DH_AGREEMENT = Option<unsafe extern "system" fn(pcarddata: *const CARD_DATA, hsecretagreement: *const core::ffi::c_void, pbsecretagreementindex: *mut u8, dwflags: u32) -> u32>;
pub type PFN_CSP_PAD_DATA = Option<unsafe extern "system" fn(psigninginfo: *const CARD_SIGNING_INFO, cbmaxwidth: u32, pcbpaddedbuffer: *mut u32, ppbpaddedbuffer: *mut *mut u8) -> u32>;
pub type PFN_CSP_REALLOC = Option<unsafe extern "system" fn(address: *const core::ffi::c_void, size: usize) -> *mut core::ffi::c_void>;
pub type PFN_CSP_UNPAD_DATA = Option<unsafe extern "system" fn(prsadecryptinfo: *const CARD_RSA_DECRYPT_INFO, pcbunpaddeddata: *mut u32, ppbunpaddeddata: *mut *mut u8) -> u32>;
pub type PFN_EXPORT_PRIV_KEY_FUNC = Option<unsafe extern "system" fn(hcryptprov: usize, dwkeyspec: u32, pszprivatekeyobjid: windows_core::PCSTR, dwflags: u32, pvauxinfo: *const core::ffi::c_void, pprivatekeyinfo: *mut CRYPT_PRIVATE_KEY_INFO, pcbprivatekeyinfo: *mut u32) -> windows_core::BOOL>;
pub type PFN_FREE_ENCODED_OBJECT_FUNC = Option<unsafe extern "system" fn(pszobjectoid: windows_core::PCSTR, pobject: *mut CRYPT_BLOB_ARRAY, pvfreecontext: *mut core::ffi::c_void)>;
pub type PFN_IMPORT_PRIV_KEY_FUNC = Option<unsafe extern "system" fn(hcryptprov: usize, pprivatekeyinfo: *const CRYPT_PRIVATE_KEY_INFO, dwflags: u32, pvauxinfo: *const core::ffi::c_void) -> windows_core::BOOL>;
pub type PFN_IMPORT_PUBLIC_KEY_INFO_EX2_FUNC = Option<unsafe extern "system" fn(dwcertencodingtype: CERT_QUERY_ENCODING_TYPE, pinfo: *const CERT_PUBLIC_KEY_INFO, dwflags: u32, pvauxinfo: *const core::ffi::c_void, phkey: *mut BCRYPT_KEY_HANDLE) -> windows_core::BOOL>;
pub type PFN_MD_ENCRYPT_DATA = Option<unsafe extern "system" fn(pcarddata: *const CARD_DATA, hkey: usize, pwszsecurefunction: windows_core::PCWSTR, pbinput: *const u8, cbinput: u32, dwflags: u32, ppencrypteddata: *mut *mut CARD_ENCRYPTED_DATA, pcencrypteddata: *mut u32) -> u32>;
pub type PFN_MD_IMPORT_SESSION_KEY = Option<unsafe extern "system" fn(pcarddata: *const CARD_DATA, pwszblobtype: windows_core::PCWSTR, pwszalgid: windows_core::PCWSTR, phkey: *mut usize, pbinput: *const u8, cbinput: u32) -> u32>;
pub type PFN_NCRYPT_ALLOC = Option<unsafe extern "system" fn(cbsize: usize) -> *mut core::ffi::c_void>;
pub type PFN_NCRYPT_FREE = Option<unsafe extern "system" fn(pv: *const core::ffi::c_void)>;
pub type PFN_OFFLOAD_MOD_EXPO = Option<unsafe extern "system" fn(pbbase: *mut u8, pbexponent: *mut u8, cbexponent: u32, pbmodulus: *mut u8, cbmodulus: u32, pbresult: *mut u8, pvoffloadprivatekey: *mut core::ffi::c_void, dwflags: u32) -> windows_core::BOOL>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PIN_CACHE_POLICY {
    pub dwVersion: u32,
    pub PinCachePolicyType: PIN_CACHE_POLICY_TYPE,
    pub dwPinCachePolicyInfo: u32,
}
pub const PIN_CACHE_POLICY_CURRENT_VERSION: u32 = 6u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PIN_CACHE_POLICY_TYPE(pub i32);
pub const PIN_CHANGE_FLAG_CHANGEPIN: u32 = 2u32;
pub const PIN_CHANGE_FLAG_UNBLOCK: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PIN_INFO {
    pub dwVersion: u32,
    pub PinType: SECRET_TYPE,
    pub PinPurpose: SECRET_PURPOSE,
    pub dwChangePermission: u32,
    pub dwUnblockPermission: u32,
    pub PinCachePolicy: PIN_CACHE_POLICY,
    pub dwFlags: u32,
}
pub const PIN_INFO_CURRENT_VERSION: u32 = 6u32;
pub const PIN_INFO_REQUIRE_SECURE_ENTRY: u32 = 1u32;
pub const PIN_SET_ALL_ROLES: u32 = 255u32;
pub const PIN_SET_NONE: u32 = 0u32;
pub const PKCS12_ALLOW_OVERWRITE_KEY: CRYPT_KEY_FLAGS = CRYPT_KEY_FLAGS(16384u32);
pub const PKCS12_ALWAYS_CNG_KSP: CRYPT_KEY_FLAGS = CRYPT_KEY_FLAGS(512u32);
pub const PKCS12_CONFIG_REGPATH: windows_core::PCWSTR = windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\PFX");
pub const PKCS12_DISABLE_ENCRYPT_CERTIFICATES: u32 = 256u32;
pub const PKCS12_ENCRYPT_CERTIFICATES: u32 = 512u32;
pub const PKCS12_ENCRYPT_CERTIFICATES_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("EncryptCertificates");
pub const PKCS12_EXPORT_ECC_CURVE_OID: u32 = 8192u32;
pub const PKCS12_EXPORT_ECC_CURVE_PARAMETERS: u32 = 4096u32;
pub const PKCS12_EXPORT_PBES2_PARAMS: u32 = 128u32;
pub const PKCS12_EXPORT_RESERVED_MASK: u32 = 4294901760u32;
pub const PKCS12_EXPORT_SILENT: u32 = 64u32;
pub const PKCS12_IMPORT_RESERVED_MASK: u32 = 4294901760u32;
pub const PKCS12_IMPORT_SILENT: u32 = 64u32;
pub const PKCS12_INCLUDE_EXTENDED_PROPERTIES: CRYPT_KEY_FLAGS = CRYPT_KEY_FLAGS(16u32);
pub const PKCS12_NO_PERSIST_KEY: CRYPT_KEY_FLAGS = CRYPT_KEY_FLAGS(32768u32);
pub const PKCS12_ONLY_CERTIFICATES: u32 = 1024u32;
pub const PKCS12_ONLY_CERTIFICATES_CONTAINER_NAME: windows_core::PCWSTR = windows_core::w!("PfxContainer");
pub const PKCS12_ONLY_CERTIFICATES_PROVIDER_NAME: windows_core::PCWSTR = windows_core::w!("PfxProvider");
pub const PKCS12_ONLY_CERTIFICATES_PROVIDER_TYPE: u32 = 0u32;
pub const PKCS12_ONLY_NOT_ENCRYPTED_CERTIFICATES: u32 = 2048u32;
pub const PKCS12_PBES2_ALG_AES256_SHA256: windows_core::PCWSTR = windows_core::w!("AES256-SHA256");
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PKCS12_PBES2_EXPORT_PARAMS {
    pub dwSize: u32,
    pub hNcryptDescriptor: *mut core::ffi::c_void,
    pub pwszPbes2Alg: windows_core::PWSTR,
}
impl Default for PKCS12_PBES2_EXPORT_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PKCS12_PBKDF2_ID_HMAC_SHA1: windows_core::PCSTR = windows_core::s!("1.2.840.113549.2.7");
pub const PKCS12_PBKDF2_ID_HMAC_SHA256: windows_core::PCSTR = windows_core::s!("1.2.840.113549.2.9");
pub const PKCS12_PBKDF2_ID_HMAC_SHA384: windows_core::PCSTR = windows_core::s!("1.2.840.113549.2.10");
pub const PKCS12_PBKDF2_ID_HMAC_SHA512: windows_core::PCSTR = windows_core::s!("1.2.840.113549.2.11");
pub const PKCS12_PREFER_CNG_KSP: CRYPT_KEY_FLAGS = CRYPT_KEY_FLAGS(256u32);
pub const PKCS12_PROTECT_TO_DOMAIN_SIDS: u32 = 32u32;
pub const PKCS12_VIRTUAL_ISOLATION_KEY: u32 = 65536u32;
pub const PKCS5_PADDING: u32 = 1u32;
pub const PKCS7_SIGNER_INFO: windows_core::PCSTR = windows_core::PCSTR(500i32 as _);
pub const PKCS_7_ASN_ENCODING: CERT_QUERY_ENCODING_TYPE = CERT_QUERY_ENCODING_TYPE(65536u32);
pub const PKCS_7_NDR_ENCODING: u32 = 131072u32;
pub const PKCS_ATTRIBUTE: windows_core::PCSTR = windows_core::PCSTR(22i32 as _);
pub const PKCS_ATTRIBUTES: windows_core::PCSTR = windows_core::PCSTR(48i32 as _);
pub const PKCS_CONTENT_INFO: windows_core::PCSTR = windows_core::PCSTR(33i32 as _);
pub const PKCS_CONTENT_INFO_SEQUENCE_OF_ANY: windows_core::PCSTR = windows_core::PCSTR(23i32 as _);
pub const PKCS_CTL: windows_core::PCSTR = windows_core::PCSTR(37i32 as _);
pub const PKCS_ENCRYPTED_PRIVATE_KEY_INFO: windows_core::PCSTR = windows_core::PCSTR(45i32 as _);
pub const PKCS_PRIVATE_KEY_INFO: windows_core::PCSTR = windows_core::PCSTR(44i32 as _);
pub const PKCS_RC2_CBC_PARAMETERS: windows_core::PCSTR = windows_core::PCSTR(41i32 as _);
pub const PKCS_RSAES_OAEP_PARAMETERS: windows_core::PCSTR = windows_core::PCSTR(76i32 as _);
pub const PKCS_RSA_PRIVATE_KEY: windows_core::PCSTR = windows_core::PCSTR(43i32 as _);
pub const PKCS_RSA_SSA_PSS_PARAMETERS: windows_core::PCSTR = windows_core::PCSTR(75i32 as _);
pub const PKCS_RSA_SSA_PSS_TRAILER_FIELD_BC: u32 = 1u32;
pub const PKCS_SMIME_CAPABILITIES: windows_core::PCSTR = windows_core::PCSTR(42i32 as _);
pub const PKCS_SORTED_CTL: windows_core::PCSTR = windows_core::PCSTR(49i32 as _);
pub const PKCS_TIME_REQUEST: windows_core::PCSTR = windows_core::PCSTR(18i32 as _);
pub const PKCS_UTC_TIME: windows_core::PCSTR = windows_core::PCSTR(17i32 as _);
pub const PLAINTEXTKEYBLOB: u32 = 8u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct POLICY_ELEMENT {
    pub targetEndpointAddress: windows_core::PCWSTR,
    pub issuerEndpointAddress: windows_core::PCWSTR,
    pub issuedTokenParameters: windows_core::PCWSTR,
    pub privacyNoticeLink: windows_core::PCWSTR,
    pub privacyNoticeVersion: u32,
    pub useManagedPresentation: windows_core::BOOL,
}
pub const PP_ADMIN_PIN: u32 = 31u32;
pub const PP_APPLI_CERT: u32 = 18u32;
pub const PP_CERTCHAIN: u32 = 9u32;
pub const PP_CHANGE_PASSWORD: u32 = 7u32;
pub const PP_CLIENT_HWND: CRYPT_SET_PROV_PARAM_ID = CRYPT_SET_PROV_PARAM_ID(1u32);
pub const PP_CONTAINER: u32 = 6u32;
pub const PP_CONTEXT_INFO: u32 = 11u32;
pub const PP_CRYPT_COUNT_KEY_USE: u32 = 41u32;
pub const PP_DELETEKEY: CRYPT_SET_PROV_PARAM_ID = CRYPT_SET_PROV_PARAM_ID(24u32);
pub const PP_DISMISS_PIN_UI_SEC: u32 = 49u32;
pub const PP_ENUMALGS: u32 = 1u32;
pub const PP_ENUMALGS_EX: u32 = 22u32;
pub const PP_ENUMCONTAINERS: u32 = 2u32;
pub const PP_ENUMELECTROOTS: u32 = 26u32;
pub const PP_ENUMEX_SIGNING_PROT: u32 = 40u32;
pub const PP_ENUMMANDROOTS: u32 = 25u32;
pub const PP_IMPTYPE: u32 = 3u32;
pub const PP_IS_PFX_EPHEMERAL: u32 = 50u32;
pub const PP_KEYEXCHANGE_ALG: CRYPT_SET_PROV_PARAM_ID = CRYPT_SET_PROV_PARAM_ID(14u32);
pub const PP_KEYEXCHANGE_KEYSIZE: CRYPT_SET_PROV_PARAM_ID = CRYPT_SET_PROV_PARAM_ID(12u32);
pub const PP_KEYEXCHANGE_PIN: CRYPT_SET_PROV_PARAM_ID = CRYPT_SET_PROV_PARAM_ID(32u32);
pub const PP_KEYSET_SEC_DESCR: CRYPT_SET_PROV_PARAM_ID = CRYPT_SET_PROV_PARAM_ID(8u32);
pub const PP_KEYSET_TYPE: u32 = 27u32;
pub const PP_KEYSPEC: u32 = 39u32;
pub const PP_KEYSTORAGE: u32 = 17u32;
pub const PP_KEYX_KEYSIZE_INC: u32 = 35u32;
pub const PP_KEY_TYPE_SUBTYPE: u32 = 10u32;
pub const PP_NAME: u32 = 4u32;
pub const PP_PIN_PROMPT_STRING: CRYPT_SET_PROV_PARAM_ID = CRYPT_SET_PROV_PARAM_ID(44u32);
pub const PP_PROVTYPE: u32 = 16u32;
pub const PP_ROOT_CERTSTORE: CRYPT_SET_PROV_PARAM_ID = CRYPT_SET_PROV_PARAM_ID(46u32);
pub const PP_SECURE_KEYEXCHANGE_PIN: CRYPT_SET_PROV_PARAM_ID = CRYPT_SET_PROV_PARAM_ID(47u32);
pub const PP_SECURE_SIGNATURE_PIN: CRYPT_SET_PROV_PARAM_ID = CRYPT_SET_PROV_PARAM_ID(48u32);
pub const PP_SESSION_KEYSIZE: u32 = 20u32;
pub const PP_SGC_INFO: u32 = 37u32;
pub const PP_SIGNATURE_ALG: CRYPT_SET_PROV_PARAM_ID = CRYPT_SET_PROV_PARAM_ID(15u32);
pub const PP_SIGNATURE_KEYSIZE: CRYPT_SET_PROV_PARAM_ID = CRYPT_SET_PROV_PARAM_ID(13u32);
pub const PP_SIGNATURE_PIN: CRYPT_SET_PROV_PARAM_ID = CRYPT_SET_PROV_PARAM_ID(33u32);
pub const PP_SIG_KEYSIZE_INC: u32 = 34u32;
pub const PP_SMARTCARD_GUID: u32 = 45u32;
pub const PP_SMARTCARD_READER: CRYPT_SET_PROV_PARAM_ID = CRYPT_SET_PROV_PARAM_ID(43u32);
pub const PP_SMARTCARD_READER_ICON: u32 = 47u32;
pub const PP_SYM_KEYSIZE: u32 = 19u32;
pub const PP_UI_PROMPT: CRYPT_SET_PROV_PARAM_ID = CRYPT_SET_PROV_PARAM_ID(21u32);
pub const PP_UNIQUE_CONTAINER: u32 = 36u32;
pub const PP_USER_CERTSTORE: CRYPT_SET_PROV_PARAM_ID = CRYPT_SET_PROV_PARAM_ID(42u32);
pub const PP_USE_HARDWARE_RNG: CRYPT_SET_PROV_PARAM_ID = CRYPT_SET_PROV_PARAM_ID(38u32);
pub const PP_VERSION: u32 = 5u32;
pub const PRIVATEKEYBLOB: u32 = 7u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PRIVKEYVER3 {
    pub magic: u32,
    pub bitlenP: u32,
    pub bitlenQ: u32,
    pub bitlenJ: u32,
    pub bitlenX: u32,
    pub DSSSeed: DSSSEED,
}
pub const PROV_DH_SCHANNEL: u32 = 18u32;
pub const PROV_DSS: u32 = 3u32;
pub const PROV_DSS_DH: u32 = 13u32;
pub const PROV_EC_ECDSA_FULL: u32 = 16u32;
pub const PROV_EC_ECDSA_SIG: u32 = 14u32;
pub const PROV_EC_ECNRA_FULL: u32 = 17u32;
pub const PROV_EC_ECNRA_SIG: u32 = 15u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PROV_ENUMALGS {
    pub aiAlgid: ALG_ID,
    pub dwBitLen: u32,
    pub dwNameLen: u32,
    pub szName: [i8; 20],
}
impl Default for PROV_ENUMALGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PROV_ENUMALGS_EX {
    pub aiAlgid: ALG_ID,
    pub dwDefaultLen: u32,
    pub dwMinLen: u32,
    pub dwMaxLen: u32,
    pub dwProtocols: u32,
    pub dwNameLen: u32,
    pub szName: [i8; 20],
    pub dwLongNameLen: u32,
    pub szLongName: [i8; 40],
}
impl Default for PROV_ENUMALGS_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PROV_FORTEZZA: u32 = 4u32;
pub const PROV_INTEL_SEC: u32 = 22u32;
pub const PROV_MS_EXCHANGE: u32 = 5u32;
pub const PROV_REPLACE_OWF: u32 = 23u32;
pub const PROV_RNG: u32 = 21u32;
pub const PROV_RSA_AES: u32 = 24u32;
pub const PROV_RSA_FULL: u32 = 1u32;
pub const PROV_RSA_SCHANNEL: u32 = 12u32;
pub const PROV_RSA_SIG: u32 = 2u32;
pub const PROV_SPYRUS_LYNKS: u32 = 20u32;
pub const PROV_SSL: u32 = 6u32;
pub const PROV_STT_ACQ: u32 = 8u32;
pub const PROV_STT_BRND: u32 = 9u32;
pub const PROV_STT_ISS: u32 = 11u32;
pub const PROV_STT_MER: u32 = 7u32;
pub const PROV_STT_ROOT: u32 = 10u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PUBKEY {
    pub magic: u32,
    pub bitlen: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PUBKEYVER3 {
    pub magic: u32,
    pub bitlenP: u32,
    pub bitlenQ: u32,
    pub bitlenJ: u32,
    pub DSSSeed: DSSSEED,
}
pub const PUBLICKEYBLOB: u32 = 6u32;
pub const PUBLICKEYBLOBEX: u32 = 10u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PUBLICKEYSTRUC {
    pub bType: u8,
    pub bVersion: u8,
    pub reserved: u16,
    pub aiKeyAlg: ALG_ID,
}
pub const PVK_TYPE_FILE_NAME: SIGNER_PRIVATE_KEY_CHOICE = SIGNER_PRIVATE_KEY_CHOICE(1u32);
pub const PVK_TYPE_KEYCONTAINER: SIGNER_PRIVATE_KEY_CHOICE = SIGNER_PRIVATE_KEY_CHOICE(2u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PaddingMode(pub i32);
impl PaddingMode {
    pub const None: Self = Self(1i32);
    pub const PKCS7: Self = Self(2i32);
    pub const Zeros: Self = Self(3i32);
    pub const ANSIX923: Self = Self(4i32);
    pub const ISO10126: Self = Self(5i32);
}
pub const PinCacheAlwaysPrompt: PIN_CACHE_POLICY_TYPE = PIN_CACHE_POLICY_TYPE(3i32);
pub const PinCacheNone: PIN_CACHE_POLICY_TYPE = PIN_CACHE_POLICY_TYPE(2i32);
pub const PinCacheNormal: PIN_CACHE_POLICY_TYPE = PIN_CACHE_POLICY_TYPE(0i32);
pub const PinCacheTimed: PIN_CACHE_POLICY_TYPE = PIN_CACHE_POLICY_TYPE(1i32);
pub const PrimaryCardPin: SECRET_PURPOSE = SECRET_PURPOSE(5i32);
pub const RANDOM_PADDING: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RECIPIENTPOLICY {
    pub recipient: ENDPOINTADDRESS,
    pub issuer: ENDPOINTADDRESS,
    pub tokenType: windows_core::PCWSTR,
    pub requiredClaims: CLAIMLIST,
    pub optionalClaims: CLAIMLIST,
    pub privacyUrl: windows_core::PCWSTR,
    pub privacyVersion: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RECIPIENTPOLICY2 {
    pub recipient: ENDPOINTADDRESS2,
    pub issuer: ENDPOINTADDRESS2,
    pub tokenType: windows_core::PCWSTR,
    pub requiredClaims: CLAIMLIST,
    pub optionalClaims: CLAIMLIST,
    pub privacyUrl: windows_core::PCWSTR,
    pub privacyVersion: u32,
}
pub const RECIPIENTPOLICYV1: u32 = 1u32;
pub const RECIPIENTPOLICYV2: u32 = 2u32;
pub const REPORT_NOT_ABLE_TO_EXPORT_PRIVATE_KEY: u32 = 2u32;
pub const REPORT_NO_PRIVATE_KEY: u32 = 1u32;
pub const REVOCATION_OID_CRL_REVOCATION: windows_core::PCSTR = windows_core::PCSTR(1i32 as _);
pub const ROLE_ADMIN: u32 = 2u32;
pub const ROLE_EVERYONE: u32 = 0u32;
pub const ROLE_PIN_ALWAYS: u32 = 3u32;
pub const ROLE_PUK: u32 = 4u32;
pub const ROLE_USER: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ROOT_INFO_LUID {
    pub LowPart: u32,
    pub HighPart: i32,
}
pub const RSA1024BIT_KEY: u32 = 67108864u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RSAPUBKEY {
    pub magic: u32,
    pub bitlen: u32,
    pub pubexp: u32,
}
pub const RSA_CSP_PUBLICKEYBLOB: windows_core::PCSTR = windows_core::PCSTR(19i32 as _);
pub const SCARD_PROVIDER_CARD_MODULE: u32 = 2147483649u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCHANNEL_ALG {
    pub dwUse: u32,
    pub Algid: ALG_ID,
    pub cBits: u32,
    pub dwFlags: u32,
    pub dwReserved: u32,
}
pub const SCHANNEL_ENC_KEY: u32 = 1u32;
pub const SCHANNEL_MAC_KEY: u32 = 0u32;
pub const SCHEME_OID_RETRIEVE_ENCODED_OBJECTW_FUNC: windows_core::PCSTR = windows_core::s!("SchemeDllRetrieveEncodedObjectW");
pub const SCHEME_OID_RETRIEVE_ENCODED_OBJECT_FUNC: windows_core::PCSTR = windows_core::s!("SchemeDllRetrieveEncodedObject");
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SECRET_PURPOSE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SECRET_TYPE(pub i32);
pub const SIGNATURE_RESOURCE_NUMBER: u32 = 666u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SIGNER_ATTR_AUTHCODE {
    pub cbSize: u32,
    pub fCommercial: windows_core::BOOL,
    pub fIndividual: windows_core::BOOL,
    pub pwszName: windows_core::PCWSTR,
    pub pwszInfo: windows_core::PCWSTR,
}
pub const SIGNER_AUTHCODE_ATTR: SIGNER_SIGNATURE_ATTRIBUTE_CHOICE = SIGNER_SIGNATURE_ATTRIBUTE_CHOICE(1u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SIGNER_BLOB_INFO {
    pub cbSize: u32,
    pub pGuidSubject: *mut windows_core::GUID,
    pub cbBlob: u32,
    pub pbBlob: *mut u8,
    pub pwszDisplayName: windows_core::PCWSTR,
}
impl Default for SIGNER_BLOB_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SIGNER_CERT {
    pub cbSize: u32,
    pub dwCertChoice: SIGNER_CERT_CHOICE,
    pub Anonymous: SIGNER_CERT_0,
    pub hwnd: super::super::Foundation::HWND,
}
impl Default for SIGNER_CERT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union SIGNER_CERT_0 {
    pub pwszSpcFile: windows_core::PCWSTR,
    pub pCertStoreInfo: *mut SIGNER_CERT_STORE_INFO,
    pub pSpcChainInfo: *mut SIGNER_SPC_CHAIN_INFO,
}
impl Default for SIGNER_CERT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SIGNER_CERT_CHOICE(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SIGNER_CERT_POLICY(pub u32);
impl SIGNER_CERT_POLICY {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for SIGNER_CERT_POLICY {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for SIGNER_CERT_POLICY {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for SIGNER_CERT_POLICY {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for SIGNER_CERT_POLICY {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for SIGNER_CERT_POLICY {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const SIGNER_CERT_POLICY_CHAIN: SIGNER_CERT_POLICY = SIGNER_CERT_POLICY(2u32);
pub const SIGNER_CERT_POLICY_CHAIN_NO_ROOT: SIGNER_CERT_POLICY = SIGNER_CERT_POLICY(8u32);
pub const SIGNER_CERT_POLICY_SPC: SIGNER_CERT_POLICY = SIGNER_CERT_POLICY(4u32);
pub const SIGNER_CERT_POLICY_STORE: SIGNER_CERT_POLICY = SIGNER_CERT_POLICY(1u32);
pub const SIGNER_CERT_SPC_CHAIN: SIGNER_CERT_CHOICE = SIGNER_CERT_CHOICE(3u32);
pub const SIGNER_CERT_SPC_FILE: SIGNER_CERT_CHOICE = SIGNER_CERT_CHOICE(1u32);
pub const SIGNER_CERT_STORE: SIGNER_CERT_CHOICE = SIGNER_CERT_CHOICE(2u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SIGNER_CERT_STORE_INFO {
    pub cbSize: u32,
    pub pSigningCert: *const CERT_CONTEXT,
    pub dwCertPolicy: SIGNER_CERT_POLICY,
    pub hCertStore: HCERTSTORE,
}
impl Default for SIGNER_CERT_STORE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SIGNER_CONTEXT {
    pub cbSize: u32,
    pub cbBlob: u32,
    pub pbBlob: *mut u8,
}
impl Default for SIGNER_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SIGNER_DIGEST_SIGN_INFO {
    pub cbSize: u32,
    pub dwDigestSignChoice: u32,
    pub Anonymous: SIGNER_DIGEST_SIGN_INFO_0,
    pub pMetadataBlob: *mut CRYPT_INTEGER_BLOB,
    pub dwReserved: u32,
    pub dwReserved2: u32,
    pub dwReserved3: u32,
}
impl Default for SIGNER_DIGEST_SIGN_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union SIGNER_DIGEST_SIGN_INFO_0 {
    pub pfnAuthenticodeDigestSign: PFN_AUTHENTICODE_DIGEST_SIGN,
    pub pfnAuthenticodeDigestSignWithFileHandle: PFN_AUTHENTICODE_DIGEST_SIGN_WITHFILEHANDLE,
    pub pfnAuthenticodeDigestSignEx: PFN_AUTHENTICODE_DIGEST_SIGN_EX,
    pub pfnAuthenticodeDigestSignExWithFileHandle: PFN_AUTHENTICODE_DIGEST_SIGN_EX_WITHFILEHANDLE,
}
impl Default for SIGNER_DIGEST_SIGN_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SIGNER_DIGEST_SIGN_INFO_V1 {
    pub cbSize: u32,
    pub pfnAuthenticodeDigestSign: PFN_AUTHENTICODE_DIGEST_SIGN,
    pub pMetadataBlob: *mut CRYPT_INTEGER_BLOB,
}
impl Default for SIGNER_DIGEST_SIGN_INFO_V1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SIGNER_DIGEST_SIGN_INFO_V2 {
    pub cbSize: u32,
    pub pfnAuthenticodeDigestSign: PFN_AUTHENTICODE_DIGEST_SIGN,
    pub pfnAuthenticodeDigestSignEx: PFN_AUTHENTICODE_DIGEST_SIGN_EX,
    pub pMetadataBlob: *mut CRYPT_INTEGER_BLOB,
}
impl Default for SIGNER_DIGEST_SIGN_INFO_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SIGNER_FILE_INFO {
    pub cbSize: u32,
    pub pwszFileName: windows_core::PCWSTR,
    pub hFile: super::super::Foundation::HANDLE,
}
pub const SIGNER_NO_ATTR: SIGNER_SIGNATURE_ATTRIBUTE_CHOICE = SIGNER_SIGNATURE_ATTRIBUTE_CHOICE(0u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SIGNER_PRIVATE_KEY_CHOICE(pub u32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SIGNER_PROVIDER_INFO {
    pub cbSize: u32,
    pub pwszProviderName: windows_core::PCWSTR,
    pub dwProviderType: u32,
    pub dwKeySpec: u32,
    pub dwPvkChoice: SIGNER_PRIVATE_KEY_CHOICE,
    pub Anonymous: SIGNER_PROVIDER_INFO_0,
}
impl Default for SIGNER_PROVIDER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union SIGNER_PROVIDER_INFO_0 {
    pub pwszPvkFileName: windows_core::PWSTR,
    pub pwszKeyContainer: windows_core::PWSTR,
}
impl Default for SIGNER_PROVIDER_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SIGNER_SIGNATURE_ATTRIBUTE_CHOICE(pub u32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SIGNER_SIGNATURE_INFO {
    pub cbSize: u32,
    pub algidHash: ALG_ID,
    pub dwAttrChoice: SIGNER_SIGNATURE_ATTRIBUTE_CHOICE,
    pub Anonymous: SIGNER_SIGNATURE_INFO_0,
    pub psAuthenticated: *mut CRYPT_ATTRIBUTES,
    pub psUnauthenticated: *mut CRYPT_ATTRIBUTES,
}
impl Default for SIGNER_SIGNATURE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union SIGNER_SIGNATURE_INFO_0 {
    pub pAttrAuthcode: *mut SIGNER_ATTR_AUTHCODE,
}
impl Default for SIGNER_SIGNATURE_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SIGNER_SIGN_FLAGS(pub u32);
impl SIGNER_SIGN_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for SIGNER_SIGN_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for SIGNER_SIGN_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for SIGNER_SIGN_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for SIGNER_SIGN_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for SIGNER_SIGN_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SIGNER_SPC_CHAIN_INFO {
    pub cbSize: u32,
    pub pwszSpcFile: windows_core::PCWSTR,
    pub dwCertPolicy: u32,
    pub hCertStore: HCERTSTORE,
}
pub const SIGNER_SUBJECT_BLOB: SIGNER_SUBJECT_CHOICE = SIGNER_SUBJECT_CHOICE(2u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SIGNER_SUBJECT_CHOICE(pub u32);
pub const SIGNER_SUBJECT_FILE: SIGNER_SUBJECT_CHOICE = SIGNER_SUBJECT_CHOICE(1u32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SIGNER_SUBJECT_INFO {
    pub cbSize: u32,
    pub pdwIndex: *mut u32,
    pub dwSubjectChoice: SIGNER_SUBJECT_CHOICE,
    pub Anonymous: SIGNER_SUBJECT_INFO_0,
}
impl Default for SIGNER_SUBJECT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union SIGNER_SUBJECT_INFO_0 {
    pub pSignerFileInfo: *mut SIGNER_FILE_INFO,
    pub pSignerBlobInfo: *mut SIGNER_BLOB_INFO,
}
impl Default for SIGNER_SUBJECT_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SIGNER_TIMESTAMP_AUTHENTICODE: SIGNER_TIMESTAMP_FLAGS = SIGNER_TIMESTAMP_FLAGS(1u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SIGNER_TIMESTAMP_FLAGS(pub u32);
pub const SIGNER_TIMESTAMP_RFC3161: SIGNER_TIMESTAMP_FLAGS = SIGNER_TIMESTAMP_FLAGS(2u32);
pub const SIG_APPEND: SIGNER_SIGN_FLAGS = SIGNER_SIGN_FLAGS(4096u32);
pub const SIMPLEBLOB: u32 = 1u32;
pub const SITE_PIN_RULES_ALL_SUBDOMAINS_FLAG: u32 = 1u32;
pub const SORTED_CTL_EXT_HASHED_SUBJECT_IDENTIFIER_FLAG: u32 = 1u32;
pub const SPC_DIGEST_GENERATE_FLAG: SIGNER_SIGN_FLAGS = SIGNER_SIGN_FLAGS(512u32);
pub const SPC_DIGEST_SIGN_EX_FLAG: SIGNER_SIGN_FLAGS = SIGNER_SIGN_FLAGS(16384u32);
pub const SPC_DIGEST_SIGN_FLAG: SIGNER_SIGN_FLAGS = SIGNER_SIGN_FLAGS(1024u32);
pub const SPC_EXC_PE_PAGE_HASHES_FLAG: SIGNER_SIGN_FLAGS = SIGNER_SIGN_FLAGS(16u32);
pub const SPC_INC_PE_DEBUG_INFO_FLAG: SIGNER_SIGN_FLAGS = SIGNER_SIGN_FLAGS(64u32);
pub const SPC_INC_PE_IMPORT_ADDR_TABLE_FLAG: SIGNER_SIGN_FLAGS = SIGNER_SIGN_FLAGS(32u32);
pub const SPC_INC_PE_PAGE_HASHES_FLAG: SIGNER_SIGN_FLAGS = SIGNER_SIGN_FLAGS(256u32);
pub const SPC_INC_PE_RESOURCES_FLAG: SIGNER_SIGN_FLAGS = SIGNER_SIGN_FLAGS(128u32);
pub const SSL2_PROTOCOL_VERSION: u32 = 2u32;
pub const SSL3_PROTOCOL_VERSION: u32 = 768u32;
pub const SSL_CK_DES_192_EDE3_CBC_WITH_MD5: u32 = 458944u32;
pub const SSL_CK_DES_64_CBC_WITH_MD5: u32 = 393280u32;
pub const SSL_CK_IDEA_128_CBC_WITH_MD5: u32 = 327808u32;
pub const SSL_CK_RC2_128_CBC_EXPORT40_WITH_MD5: u32 = 262272u32;
pub const SSL_CK_RC2_128_CBC_WITH_MD5: u32 = 196736u32;
pub const SSL_CK_RC4_128_EXPORT40_WITH_MD5: u32 = 131200u32;
pub const SSL_CK_RC4_128_WITH_MD5: u32 = 65664u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SSL_ECCKEY_BLOB {
    pub dwCurveType: u32,
    pub cbKey: u32,
}
pub const SSL_ECCPUBLIC_BLOB: windows_core::PCWSTR = windows_core::w!("SSLECCPUBLICBLOB");
pub const SSL_ECDSA_ALGORITHM: windows_core::PCWSTR = windows_core::w!("ECDSA");
pub const SSL_F12_ERROR_TEXT_LENGTH: u32 = 256u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SSL_F12_EXTRA_CERT_CHAIN_POLICY_STATUS {
    pub cbSize: u32,
    pub dwErrorLevel: u32,
    pub dwErrorCategory: u32,
    pub dwReserved: u32,
    pub wszErrorText: [u16; 256],
}
impl Default for SSL_F12_EXTRA_CERT_CHAIN_POLICY_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SSL_HPKP_HEADER_COUNT: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SSL_HPKP_HEADER_EXTRA_CERT_CHAIN_POLICY_PARA {
    pub cbSize: u32,
    pub dwReserved: u32,
    pub pwszServerName: windows_core::PWSTR,
    pub rgpszHpkpValue: [windows_core::PSTR; 2],
}
impl Default for SSL_HPKP_HEADER_EXTRA_CERT_CHAIN_POLICY_PARA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SSL_HPKP_PKP_HEADER_INDEX: u32 = 0u32;
pub const SSL_HPKP_PKP_RO_HEADER_INDEX: u32 = 1u32;
pub const SSL_KEY_PIN_ERROR_TEXT_LENGTH: u32 = 512u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SSL_KEY_PIN_EXTRA_CERT_CHAIN_POLICY_PARA {
    pub cbSize: u32,
    pub dwReserved: u32,
    pub pwszServerName: windows_core::PCWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SSL_KEY_PIN_EXTRA_CERT_CHAIN_POLICY_STATUS {
    pub cbSize: u32,
    pub lError: i32,
    pub wszErrorText: [u16; 512],
}
impl Default for SSL_KEY_PIN_EXTRA_CERT_CHAIN_POLICY_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SSL_KEY_TYPE_PROPERTY: windows_core::PCWSTR = windows_core::w!("KEYTYPE");
pub const SSL_OBJECT_LOCATOR_CERT_VALIDATION_CONFIG_FUNC: windows_core::PCSTR = windows_core::s!("SslObjectLocatorInitializeCertValidationConfig");
pub const SSL_OBJECT_LOCATOR_ISSUER_LIST_FUNC: windows_core::PCSTR = windows_core::s!("SslObjectLocatorInitializeIssuerList");
pub const SSL_OBJECT_LOCATOR_PFX_FUNC: windows_core::PCSTR = windows_core::s!("SslObjectLocatorInitializePfx");
pub const SYMMETRICWRAPKEYBLOB: u32 = 11u32;
pub type SslComputeClientAuthHashFn = Option<unsafe extern "system" fn(hsslprovider: NCRYPT_PROV_HANDLE, hmasterkey: NCRYPT_KEY_HANDLE, hhandshakehash: NCRYPT_HASH_HANDLE, pszalgid: windows_core::PCWSTR, pboutput: *mut u8, cboutput: u32, pcbresult: *mut u32, dwflags: u32) -> windows_core::HRESULT>;
pub type SslComputeEapKeyBlockFn = Option<unsafe extern "system" fn(hsslprovider: NCRYPT_PROV_HANDLE, hmasterkey: NCRYPT_KEY_HANDLE, pbrandoms: *const u8, cbrandoms: u32, pboutput: *mut u8, cboutput: u32, pcbresult: *mut u32, dwflags: u32) -> windows_core::HRESULT>;
pub type SslComputeFinishedHashFn = Option<unsafe extern "system" fn(hsslprovider: NCRYPT_PROV_HANDLE, hmasterkey: NCRYPT_KEY_HANDLE, hhandshakehash: NCRYPT_HASH_HANDLE, pboutput: *mut u8, cboutput: u32, dwflags: u32) -> windows_core::HRESULT>;
pub type SslComputeSessionHashFn = Option<unsafe extern "system" fn(hsslprovider: NCRYPT_PROV_HANDLE, hhandshakehash: NCRYPT_HASH_HANDLE, dwprotocol: u32, pboutput: *mut u8, cboutput: u32, pcbresult: *mut u32, dwflags: u32) -> windows_core::HRESULT>;
pub type SslCreateClientAuthHashFn = Option<unsafe extern "system" fn(hsslprovider: NCRYPT_PROV_HANDLE, phhandshakehash: *mut NCRYPT_HASH_HANDLE, dwprotocol: u32, dwciphersuite: u32, pszhashalgid: windows_core::PCWSTR, dwflags: u32) -> windows_core::HRESULT>;
pub type SslCreateEphemeralKeyFn = Option<unsafe extern "system" fn(hsslprovider: NCRYPT_PROV_HANDLE, phephemeralkey: *mut NCRYPT_KEY_HANDLE, dwprotocol: u32, dwciphersuite: u32, dwkeytype: u32, dwkeybitlen: u32, pbparams: *const u8, cbparams: u32, dwflags: u32) -> windows_core::HRESULT>;
pub type SslCreateHandshakeHashFn = Option<unsafe extern "system" fn(hsslprovider: NCRYPT_PROV_HANDLE, phhandshakehash: *mut NCRYPT_HASH_HANDLE, dwprotocol: u32, dwciphersuite: u32, dwflags: u32) -> windows_core::HRESULT>;
pub type SslDecryptPacketFn = Option<unsafe extern "system" fn(hsslprovider: NCRYPT_PROV_HANDLE, hkey: NCRYPT_KEY_HANDLE, pbinput: *const u8, cbinput: u32, pboutput: *mut u8, cboutput: u32, pcbresult: *mut u32, sequencenumber: u64, dwflags: u32) -> windows_core::HRESULT>;
pub type SslDuplicateTranscriptHashFn = Option<unsafe extern "system" fn(hsslprovider: NCRYPT_PROV_HANDLE, htranscripthash: NCRYPT_HASH_HANDLE, phduplicatetranscripthash: *mut NCRYPT_HASH_HANDLE, dwflags: u32) -> windows_core::HRESULT>;
pub type SslEncryptPacketFn = Option<unsafe extern "system" fn(hsslprovider: NCRYPT_PROV_HANDLE, hkey: NCRYPT_KEY_HANDLE, pbinput: *const u8, cbinput: u32, pboutput: *mut u8, cboutput: u32, pcbresult: *mut u32, sequencenumber: u64, dwcontenttype: u32, dwflags: u32) -> windows_core::HRESULT>;
pub type SslEnumCipherSuitesExFn = Option<unsafe extern "system" fn(hsslprovider: NCRYPT_PROV_HANDLE, hprivatekey: NCRYPT_KEY_HANDLE, ppciphersuite: *mut *mut NCRYPT_SSL_CIPHER_SUITE_EX, ppenumstate: *mut *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT>;
pub type SslEnumCipherSuitesFn = Option<unsafe extern "system" fn(hsslprovider: NCRYPT_PROV_HANDLE, hprivatekey: NCRYPT_KEY_HANDLE, ppciphersuite: *mut *mut NCRYPT_SSL_CIPHER_SUITE, ppenumstate: *mut *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT>;
pub type SslEnumEccCurvesFn = Option<unsafe extern "system" fn(hsslprovider: NCRYPT_PROV_HANDLE, pecccurvecount: *mut u32, ppecccurve: *mut *mut NCRYPT_SSL_ECC_CURVE, dwflags: u32) -> windows_core::HRESULT>;
pub type SslExpandBinderKeyFn = Option<unsafe extern "system" fn(hsslprovider: NCRYPT_PROV_HANDLE, hearlykey: NCRYPT_KEY_HANDLE, phbinderkey: *mut NCRYPT_KEY_HANDLE, pparameterlist: *const BCryptBufferDesc, dwflags: u32) -> windows_core::HRESULT>;
pub type SslExpandExporterMasterKeyFn = Option<unsafe extern "system" fn(hsslprovider: NCRYPT_PROV_HANDLE, hbasekey: NCRYPT_KEY_HANDLE, hhashvalue: NCRYPT_HASH_HANDLE, phexportermasterkey: *mut NCRYPT_KEY_HANDLE, pparameterlist: *const BCryptBufferDesc, dwflags: u32) -> windows_core::HRESULT>;
pub type SslExpandPreSharedKeyFn = Option<unsafe extern "system" fn(hsslprovider: NCRYPT_PROV_HANDLE, hresumptionmasterkey: NCRYPT_KEY_HANDLE, pbticketnonce: *const u8, cbticketnonce: u32, phpresharedkey: *mut NCRYPT_KEY_HANDLE, pparameterlist: *const BCryptBufferDesc, dwflags: u32) -> windows_core::HRESULT>;
pub type SslExpandResumptionMasterKeyFn = Option<unsafe extern "system" fn(hsslprovider: NCRYPT_PROV_HANDLE, hmasterkey: NCRYPT_KEY_HANDLE, hhashvalue: NCRYPT_HASH_HANDLE, phresumptionmasterkey: *mut NCRYPT_KEY_HANDLE, pparameterlist: *const BCryptBufferDesc, dwflags: u32) -> windows_core::HRESULT>;
pub type SslExpandTrafficKeysFn = Option<unsafe extern "system" fn(hsslprovider: NCRYPT_PROV_HANDLE, hbasekey: NCRYPT_KEY_HANDLE, hhashvalue: NCRYPT_HASH_HANDLE, phclienttraffickey: *mut NCRYPT_KEY_HANDLE, phservertraffickey: *mut NCRYPT_KEY_HANDLE, pparameterlist: *const BCryptBufferDesc, dwflags: u32) -> windows_core::HRESULT>;
pub type SslExpandWriteKeyFn = Option<unsafe extern "system" fn(hsslprovider: NCRYPT_PROV_HANDLE, hbasetraffickey: NCRYPT_KEY_HANDLE, phwritekey: *mut NCRYPT_KEY_HANDLE, pparameterlist: *const BCryptBufferDesc, dwflags: u32) -> windows_core::HRESULT>;
pub type SslExportKeyFn = Option<unsafe extern "system" fn(hsslprovider: NCRYPT_PROV_HANDLE, hkey: NCRYPT_KEY_HANDLE, pszblobtype: windows_core::PCWSTR, pboutput: *mut u8, cboutput: u32, pcbresult: *mut u32, dwflags: u32) -> windows_core::HRESULT>;
pub type SslExportKeyingMaterialFn = Option<unsafe extern "system" fn(hsslprovider: NCRYPT_PROV_HANDLE, hmasterkey: NCRYPT_KEY_HANDLE, slabel: windows_core::PCSTR, pbrandoms: *const u8, cbrandoms: u32, pbcontextvalue: *const u8, cbcontextvalue: u16, pboutput: *mut u8, cboutput: u32, dwflags: u32) -> windows_core::HRESULT>;
pub type SslExtractEarlyKeyFn = Option<unsafe extern "system" fn(hsslprovider: NCRYPT_PROV_HANDLE, hpresharedkey: NCRYPT_KEY_HANDLE, phearlykey: *mut NCRYPT_KEY_HANDLE, dwprotocol: u32, dwciphersuite: u32, pparameterlist: *const BCryptBufferDesc, dwflags: u32) -> windows_core::HRESULT>;
pub type SslExtractHandshakeKeyFn = Option<unsafe extern "system" fn(hsslprovider: NCRYPT_PROV_HANDLE, hprivatekey: NCRYPT_KEY_HANDLE, hpublickey: NCRYPT_KEY_HANDLE, hearlykey: NCRYPT_KEY_HANDLE, phhandshakekey: *mut NCRYPT_KEY_HANDLE, pparameterlist: *const BCryptBufferDesc, dwflags: u32) -> windows_core::HRESULT>;
pub type SslExtractMasterKeyFn = Option<unsafe extern "system" fn(hsslprovider: NCRYPT_PROV_HANDLE, hhandshakekey: NCRYPT_KEY_HANDLE, phmasterkey: *mut NCRYPT_KEY_HANDLE, pparameterlist: *const BCryptBufferDesc, dwflags: u32) -> windows_core::HRESULT>;
pub type SslFreeBufferFn = Option<unsafe extern "system" fn(pvinput: *mut core::ffi::c_void) -> windows_core::HRESULT>;
pub type SslFreeObjectFn = Option<unsafe extern "system" fn(hobject: NCRYPT_HANDLE, dwflags: u32) -> windows_core::HRESULT>;
pub type SslGenerateMasterKeyFn = Option<unsafe extern "system" fn(hsslprovider: NCRYPT_PROV_HANDLE, hprivatekey: NCRYPT_KEY_HANDLE, hpublickey: NCRYPT_KEY_HANDLE, phmasterkey: *mut NCRYPT_KEY_HANDLE, dwprotocol: u32, dwciphersuite: u32, pparameterlist: *const BCryptBufferDesc, pboutput: *mut u8, cboutput: u32, pcbresult: *mut u32, dwflags: u32) -> windows_core::HRESULT>;
pub type SslGeneratePreMasterKeyFn = Option<unsafe extern "system" fn(hsslprovider: NCRYPT_PROV_HANDLE, hpublickey: NCRYPT_KEY_HANDLE, phpremasterkey: *mut NCRYPT_KEY_HANDLE, dwprotocol: u32, dwciphersuite: u32, pparameterlist: *const BCryptBufferDesc, pboutput: *mut u8, cboutput: u32, pcbresult: *mut u32, dwflags: u32) -> windows_core::HRESULT>;
pub type SslGenerateSessionKeysFn = Option<unsafe extern "system" fn(hsslprovider: NCRYPT_PROV_HANDLE, hmasterkey: NCRYPT_KEY_HANDLE, phreadkey: *mut NCRYPT_KEY_HANDLE, phwritekey: *mut NCRYPT_KEY_HANDLE, pparameterlist: *const BCryptBufferDesc, dwflags: u32) -> windows_core::HRESULT>;
pub type SslGetCipherSuitePRFHashAlgorithmFn = Option<unsafe extern "system" fn(hsslprovider: NCRYPT_PROV_HANDLE, dwprotocol: u32, dwciphersuite: u32, dwkeytype: u32, szprfhash: windows_core::PWSTR, dwflags: u32) -> windows_core::HRESULT>;
pub type SslGetKeyPropertyFn = Option<unsafe extern "system" fn(hkey: NCRYPT_KEY_HANDLE, pszproperty: windows_core::PCWSTR, ppboutput: *mut *mut u8, pcboutput: *mut u32, dwflags: u32) -> windows_core::HRESULT>;
pub type SslGetProviderPropertyFn = Option<unsafe extern "system" fn(hsslprovider: NCRYPT_PROV_HANDLE, pszproperty: windows_core::PCWSTR, ppboutput: *mut *mut u8, pcboutput: *mut u32, ppenumstate: *mut *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT>;
pub type SslHashHandshakeFn = Option<unsafe extern "system" fn(hsslprovider: NCRYPT_PROV_HANDLE, hhandshakehash: NCRYPT_HASH_HANDLE, pbinput: *const u8, cbinput: u32, dwflags: u32) -> windows_core::HRESULT>;
pub type SslImportKeyFn = Option<unsafe extern "system" fn(hsslprovider: NCRYPT_PROV_HANDLE, phkey: *mut NCRYPT_KEY_HANDLE, pszblobtype: windows_core::PCWSTR, pbkeyblob: *const u8, cbkeyblob: u32, dwflags: u32) -> windows_core::HRESULT>;
pub type SslImportMasterKeyFn = Option<unsafe extern "system" fn(hsslprovider: NCRYPT_PROV_HANDLE, hprivatekey: NCRYPT_KEY_HANDLE, phmasterkey: *mut NCRYPT_KEY_HANDLE, dwprotocol: u32, dwciphersuite: u32, pparameterlist: *const BCryptBufferDesc, pbencryptedkey: *const u8, cbencryptedkey: u32, dwflags: u32) -> windows_core::HRESULT>;
pub type SslInitializeInterfaceFn = Option<unsafe extern "system" fn(pszprovidername: windows_core::PCWSTR, pfunctiontable: *mut NCRYPT_SSL_FUNCTION_TABLE, dwflags: u32) -> windows_core::HRESULT>;
pub type SslLookupCipherLengthsFn = Option<unsafe extern "system" fn(hsslprovider: NCRYPT_PROV_HANDLE, dwprotocol: u32, dwciphersuite: u32, dwkeytype: u32, pcipherlengths: *mut NCRYPT_SSL_CIPHER_LENGTHS, cbcipherlengths: u32, dwflags: u32) -> windows_core::HRESULT>;
pub type SslLookupCipherSuiteInfoFn = Option<unsafe extern "system" fn(hsslprovider: NCRYPT_PROV_HANDLE, dwprotocol: u32, dwciphersuite: u32, dwkeytype: u32, pciphersuite: *mut NCRYPT_SSL_CIPHER_SUITE, dwflags: u32) -> windows_core::HRESULT>;
pub type SslOpenPrivateKeyFn = Option<unsafe extern "system" fn(hsslprovider: NCRYPT_PROV_HANDLE, phprivatekey: *mut NCRYPT_KEY_HANDLE, pcertcontext: *const CERT_CONTEXT, dwflags: u32) -> windows_core::HRESULT>;
pub type SslOpenProviderFn = Option<unsafe extern "system" fn(phsslprovider: *mut NCRYPT_PROV_HANDLE, pszprovidername: windows_core::PCWSTR, dwflags: u32) -> windows_core::HRESULT>;
pub type SslSignHashFn = Option<unsafe extern "system" fn(hsslprovider: NCRYPT_PROV_HANDLE, hprivatekey: NCRYPT_KEY_HANDLE, pbhashvalue: *const u8, cbhashvalue: u32, pbsignature: *mut u8, cbsignature: u32, pcbresult: *mut u32, dwflags: u32) -> windows_core::HRESULT>;
pub type SslVerifySignatureFn = Option<unsafe extern "system" fn(hsslprovider: NCRYPT_PROV_HANDLE, hpublickey: NCRYPT_KEY_HANDLE, pbhashvalue: *const u8, cbhashvalue: u32, pbsignature: *const u8, cbsignature: u32, dwflags: u32) -> windows_core::HRESULT>;
pub const TIMESTAMP_DONT_HASH_DATA: u32 = 1u32;
pub const TIMESTAMP_FAILURE_BAD_ALG: u32 = 0u32;
pub const TIMESTAMP_FAILURE_BAD_FORMAT: u32 = 5u32;
pub const TIMESTAMP_FAILURE_BAD_REQUEST: u32 = 2u32;
pub const TIMESTAMP_FAILURE_EXTENSION_NOT_SUPPORTED: u32 = 16u32;
pub const TIMESTAMP_FAILURE_INFO_NOT_AVAILABLE: u32 = 17u32;
pub const TIMESTAMP_FAILURE_POLICY_NOT_SUPPORTED: u32 = 15u32;
pub const TIMESTAMP_FAILURE_SYSTEM_FAILURE: u32 = 25u32;
pub const TIMESTAMP_FAILURE_TIME_NOT_AVAILABLE: u32 = 14u32;
pub const TIMESTAMP_INFO: windows_core::PCSTR = windows_core::PCSTR(80i32 as _);
pub const TIMESTAMP_NO_AUTH_RETRIEVAL: u32 = 131072u32;
pub const TIMESTAMP_REQUEST: windows_core::PCSTR = windows_core::PCSTR(78i32 as _);
pub const TIMESTAMP_RESPONSE: windows_core::PCSTR = windows_core::PCSTR(79i32 as _);
pub const TIMESTAMP_STATUS_GRANTED: CRYPT_TIMESTAMP_RESPONSE_STATUS = CRYPT_TIMESTAMP_RESPONSE_STATUS(0u32);
pub const TIMESTAMP_STATUS_GRANTED_WITH_MODS: CRYPT_TIMESTAMP_RESPONSE_STATUS = CRYPT_TIMESTAMP_RESPONSE_STATUS(1u32);
pub const TIMESTAMP_STATUS_REJECTED: CRYPT_TIMESTAMP_RESPONSE_STATUS = CRYPT_TIMESTAMP_RESPONSE_STATUS(2u32);
pub const TIMESTAMP_STATUS_REVOCATION_WARNING: CRYPT_TIMESTAMP_RESPONSE_STATUS = CRYPT_TIMESTAMP_RESPONSE_STATUS(4u32);
pub const TIMESTAMP_STATUS_REVOKED: CRYPT_TIMESTAMP_RESPONSE_STATUS = CRYPT_TIMESTAMP_RESPONSE_STATUS(5u32);
pub const TIMESTAMP_STATUS_WAITING: CRYPT_TIMESTAMP_RESPONSE_STATUS = CRYPT_TIMESTAMP_RESPONSE_STATUS(3u32);
pub const TIMESTAMP_VERIFY_CONTEXT_SIGNATURE: u32 = 32u32;
pub const TIMESTAMP_VERSION: CRYPT_TIMESTAMP_VERSION = CRYPT_TIMESTAMP_VERSION(1u32);
pub const TIME_VALID_OID_FLUSH_CRL: windows_core::PCSTR = windows_core::PCSTR(2i32 as _);
pub const TIME_VALID_OID_FLUSH_CRL_FROM_CERT: windows_core::PCSTR = windows_core::PCSTR(3i32 as _);
pub const TIME_VALID_OID_FLUSH_CTL: windows_core::PCSTR = windows_core::PCSTR(1i32 as _);
pub const TIME_VALID_OID_FLUSH_FRESHEST_CRL_FROM_CERT: windows_core::PCSTR = windows_core::PCSTR(4i32 as _);
pub const TIME_VALID_OID_FLUSH_FRESHEST_CRL_FROM_CRL: windows_core::PCSTR = windows_core::PCSTR(5i32 as _);
pub const TIME_VALID_OID_FLUSH_OBJECT_FUNC: windows_core::PCSTR = windows_core::s!("TimeValidDllFlushObject");
pub const TIME_VALID_OID_GET_CRL: windows_core::PCSTR = windows_core::PCSTR(2i32 as _);
pub const TIME_VALID_OID_GET_CRL_FROM_CERT: windows_core::PCSTR = windows_core::PCSTR(3i32 as _);
pub const TIME_VALID_OID_GET_CTL: windows_core::PCSTR = windows_core::PCSTR(1i32 as _);
pub const TIME_VALID_OID_GET_FRESHEST_CRL_FROM_CERT: windows_core::PCSTR = windows_core::PCSTR(4i32 as _);
pub const TIME_VALID_OID_GET_FRESHEST_CRL_FROM_CRL: windows_core::PCSTR = windows_core::PCSTR(5i32 as _);
pub const TIME_VALID_OID_GET_OBJECT_FUNC: windows_core::PCSTR = windows_core::s!("TimeValidDllGetObject");
pub const TLS1_0_PROTOCOL_VERSION: u32 = 769u32;
pub const TLS1_1_PROTOCOL_VERSION: u32 = 770u32;
pub const TLS1_2_PROTOCOL_VERSION: u32 = 771u32;
pub const TLS1_3_PROTOCOL_VERSION: u32 = 772u32;
pub const TLS1_PROTOCOL_VERSION: u32 = 769u32;
pub const TLS_AES_128_GCM_SHA256: u32 = 4865u32;
pub const TLS_AES_256_GCM_SHA384: u32 = 4866u32;
pub const TLS_DHE_DSS_EXPORT1024_WITH_DES_CBC_SHA: u32 = 99u32;
pub const TLS_DHE_DSS_WITH_3DES_EDE_CBC_SHA: u32 = 19u32;
pub const TLS_DHE_DSS_WITH_AES_128_CBC_SHA: u32 = 50u32;
pub const TLS_DHE_DSS_WITH_AES_128_CBC_SHA256: u32 = 64u32;
pub const TLS_DHE_DSS_WITH_AES_256_CBC_SHA: u32 = 56u32;
pub const TLS_DHE_DSS_WITH_AES_256_CBC_SHA256: u32 = 106u32;
pub const TLS_DHE_DSS_WITH_DES_CBC_SHA: u32 = 18u32;
pub const TLS_DHE_RSA_WITH_3DES_EDE_CBC_SHA: u32 = 22u32;
pub const TLS_DHE_RSA_WITH_AES_128_CBC_SHA: u32 = 51u32;
pub const TLS_DHE_RSA_WITH_AES_128_GCM_SHA256: u32 = 158u32;
pub const TLS_DHE_RSA_WITH_AES_256_CBC_SHA: u32 = 57u32;
pub const TLS_DHE_RSA_WITH_AES_256_GCM_SHA384: u32 = 159u32;
pub const TLS_ECC_P256_CURVE_KEY_TYPE: u32 = 23u32;
pub const TLS_ECC_P384_CURVE_KEY_TYPE: u32 = 24u32;
pub const TLS_ECC_P521_CURVE_KEY_TYPE: u32 = 25u32;
pub const TLS_ECDHE_ECDSA_WITH_AES_128_CBC_SHA: u32 = 49161u32;
pub const TLS_ECDHE_ECDSA_WITH_AES_128_CBC_SHA256: u32 = 49187u32;
pub const TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256: u32 = 49195u32;
pub const TLS_ECDHE_ECDSA_WITH_AES_256_CBC_SHA: u32 = 49162u32;
pub const TLS_ECDHE_ECDSA_WITH_AES_256_CBC_SHA384: u32 = 49188u32;
pub const TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384: u32 = 49196u32;
pub const TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA: u32 = 49171u32;
pub const TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA256: u32 = 49191u32;
pub const TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256: u32 = 49199u32;
pub const TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA: u32 = 49172u32;
pub const TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA384: u32 = 49192u32;
pub const TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384: u32 = 49200u32;
pub const TLS_PSK_EXCHANGE: windows_core::PCWSTR = windows_core::w!("PSK");
pub const TLS_PSK_WITH_AES_128_CBC_SHA256: u32 = 174u32;
pub const TLS_PSK_WITH_AES_128_GCM_SHA256: u32 = 168u32;
pub const TLS_PSK_WITH_AES_256_CBC_SHA384: u32 = 175u32;
pub const TLS_PSK_WITH_AES_256_GCM_SHA384: u32 = 169u32;
pub const TLS_PSK_WITH_NULL_SHA256: u32 = 176u32;
pub const TLS_PSK_WITH_NULL_SHA384: u32 = 177u32;
pub const TLS_RSA_EXPORT1024_WITH_DES_CBC_SHA: u32 = 98u32;
pub const TLS_RSA_EXPORT1024_WITH_RC4_56_SHA: u32 = 100u32;
pub const TLS_RSA_EXPORT_WITH_RC4_40_MD5: u32 = 3u32;
pub const TLS_RSA_PSK_EXCHANGE: windows_core::PCWSTR = windows_core::w!("RSA_PSK");
pub const TLS_RSA_WITH_3DES_EDE_CBC_SHA: u32 = 10u32;
pub const TLS_RSA_WITH_AES_128_CBC_SHA: u32 = 47u32;
pub const TLS_RSA_WITH_AES_128_CBC_SHA256: u32 = 60u32;
pub const TLS_RSA_WITH_AES_128_GCM_SHA256: u32 = 156u32;
pub const TLS_RSA_WITH_AES_256_CBC_SHA: u32 = 53u32;
pub const TLS_RSA_WITH_AES_256_CBC_SHA256: u32 = 61u32;
pub const TLS_RSA_WITH_AES_256_GCM_SHA384: u32 = 157u32;
pub const TLS_RSA_WITH_DES_CBC_SHA: u32 = 9u32;
pub const TLS_RSA_WITH_NULL_MD5: u32 = 1u32;
pub const TLS_RSA_WITH_NULL_SHA: u32 = 2u32;
pub const TLS_RSA_WITH_NULL_SHA256: u32 = 59u32;
pub const TLS_RSA_WITH_RC4_128_MD5: u32 = 4u32;
pub const TLS_RSA_WITH_RC4_128_SHA: u32 = 5u32;
pub const TPM_RSA_SRK_SEAL_KEY: windows_core::PCWSTR = windows_core::w!("MICROSOFT_PCP_KSP_RSA_SEAL_KEY_3BD1C4BF-004E-4E2F-8A4D-0BF633DCB074");
pub const URL_OID_CERTIFICATE_CRL_DIST_POINT: windows_core::PCSTR = windows_core::PCSTR(2i32 as _);
pub const URL_OID_CERTIFICATE_CRL_DIST_POINT_AND_OCSP: windows_core::PCSTR = windows_core::PCSTR(11i32 as _);
pub const URL_OID_CERTIFICATE_FRESHEST_CRL: windows_core::PCSTR = windows_core::PCSTR(6i32 as _);
pub const URL_OID_CERTIFICATE_ISSUER: windows_core::PCSTR = windows_core::PCSTR(1i32 as _);
pub const URL_OID_CERTIFICATE_OCSP: windows_core::PCSTR = windows_core::PCSTR(9i32 as _);
pub const URL_OID_CERTIFICATE_OCSP_AND_CRL_DIST_POINT: windows_core::PCSTR = windows_core::PCSTR(10i32 as _);
pub const URL_OID_CERTIFICATE_ONLY_OCSP: windows_core::PCSTR = windows_core::PCSTR(13i32 as _);
pub const URL_OID_CRL_FRESHEST_CRL: windows_core::PCSTR = windows_core::PCSTR(7i32 as _);
pub const URL_OID_CRL_ISSUER: windows_core::PCSTR = windows_core::PCSTR(5i32 as _);
pub const URL_OID_CROSS_CERT_DIST_POINT: windows_core::PCSTR = windows_core::PCSTR(8i32 as _);
pub const URL_OID_CROSS_CERT_SUBJECT_INFO_ACCESS: windows_core::PCSTR = windows_core::PCSTR(12i32 as _);
pub const URL_OID_CTL_ISSUER: windows_core::PCSTR = windows_core::PCSTR(3i32 as _);
pub const URL_OID_CTL_NEXT_UPDATE: windows_core::PCSTR = windows_core::PCSTR(4i32 as _);
pub const URL_OID_GET_OBJECT_URL_FUNC: windows_core::PCSTR = windows_core::s!("UrlDllGetObjectUrl");
pub const USAGE_MATCH_TYPE_AND: u32 = 0u32;
pub const USAGE_MATCH_TYPE_OR: u32 = 1u32;
pub const UnblockOnlyPin: SECRET_PURPOSE = SECRET_PURPOSE(6i32);
pub const UnknownAc: CARD_FILE_ACCESS_CONDITION = CARD_FILE_ACCESS_CONDITION(4i32);
pub const UserCreateDeleteDirAc: CARD_DIRECTORY_ACCESS_CONDITION = CARD_DIRECTORY_ACCESS_CONDITION(1i32);
pub const UserReadWriteAc: CARD_FILE_ACCESS_CONDITION = CARD_FILE_ACCESS_CONDITION(5i32);
pub const UserWriteExecuteAc: CARD_FILE_ACCESS_CONDITION = CARD_FILE_ACCESS_CONDITION(2i32);
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct VTableProvStruc {
    pub Version: u32,
    pub FuncVerifyImage: CRYPT_VERIFY_IMAGE_A,
    pub FuncReturnhWnd: CRYPT_RETURN_HWND,
    pub dwProvType: u32,
    pub pbContextInfo: *mut u8,
    pub cbContextInfo: u32,
    pub pszProvName: windows_core::PSTR,
}
impl Default for VTableProvStruc {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct VTableProvStrucW {
    pub Version: u32,
    pub FuncVerifyImage: CRYPT_VERIFY_IMAGE_W,
    pub FuncReturnhWnd: CRYPT_RETURN_HWND,
    pub dwProvType: u32,
    pub pbContextInfo: *mut u8,
    pub cbContextInfo: u32,
    pub pszProvName: windows_core::PWSTR,
}
impl Default for VTableProvStrucW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const X509_ALGORITHM_IDENTIFIER: windows_core::PCSTR = windows_core::PCSTR(74i32 as _);
pub const X509_ALTERNATE_NAME: windows_core::PCSTR = windows_core::PCSTR(12i32 as _);
pub const X509_ANY_STRING: i32 = 6i32;
pub const X509_ASN_ENCODING: CERT_QUERY_ENCODING_TYPE = CERT_QUERY_ENCODING_TYPE(1u32);
pub const X509_AUTHORITY_INFO_ACCESS: windows_core::PCSTR = windows_core::PCSTR(32i32 as _);
pub const X509_AUTHORITY_KEY_ID: windows_core::PCSTR = windows_core::PCSTR(9i32 as _);
pub const X509_AUTHORITY_KEY_ID2: windows_core::PCSTR = windows_core::PCSTR(31i32 as _);
pub const X509_BASIC_CONSTRAINTS: windows_core::PCSTR = windows_core::PCSTR(13i32 as _);
pub const X509_BASIC_CONSTRAINTS2: windows_core::PCSTR = windows_core::PCSTR(15i32 as _);
pub const X509_BIOMETRIC_EXT: windows_core::PCSTR = windows_core::PCSTR(71i32 as _);
pub const X509_BITS: windows_core::PCSTR = windows_core::PCSTR(26i32 as _);
pub const X509_BITS_WITHOUT_TRAILING_ZEROES: windows_core::PCSTR = windows_core::PCSTR(51i32 as _);
pub const X509_CERT: windows_core::PCSTR = windows_core::PCSTR(1i32 as _);
pub const X509_CERTIFICATE_TEMPLATE: windows_core::PCSTR = windows_core::PCSTR(64i32 as _);
pub const X509_CERT_BUNDLE: windows_core::PCSTR = windows_core::PCSTR(81i32 as _);
pub const X509_CERT_CRL_TO_BE_SIGNED: windows_core::PCSTR = windows_core::PCSTR(3i32 as _);
pub const X509_CERT_PAIR: windows_core::PCSTR = windows_core::PCSTR(53i32 as _);
pub const X509_CERT_POLICIES: windows_core::PCSTR = windows_core::PCSTR(16i32 as _);
pub const X509_CERT_REQUEST_TO_BE_SIGNED: windows_core::PCSTR = windows_core::PCSTR(4i32 as _);
pub const X509_CERT_TO_BE_SIGNED: windows_core::PCSTR = windows_core::PCSTR(2i32 as _);
pub const X509_CHOICE_OF_TIME: windows_core::PCSTR = windows_core::PCSTR(30i32 as _);
pub const X509_CRL_DIST_POINTS: windows_core::PCSTR = windows_core::PCSTR(35i32 as _);
pub const X509_CRL_REASON_CODE: i32 = 29i32;
pub const X509_CROSS_CERT_DIST_POINTS: windows_core::PCSTR = windows_core::PCSTR(58i32 as _);
pub const X509_DH_PARAMETERS: windows_core::PCSTR = windows_core::PCSTR(47i32 as _);
pub const X509_DH_PUBLICKEY: i32 = 38i32;
pub const X509_DSS_PARAMETERS: windows_core::PCSTR = windows_core::PCSTR(39i32 as _);
pub const X509_DSS_PUBLICKEY: i32 = 38i32;
pub const X509_DSS_SIGNATURE: windows_core::PCSTR = windows_core::PCSTR(40i32 as _);
pub const X509_ECC_PARAMETERS: windows_core::PCSTR = windows_core::PCSTR(85i32 as _);
pub const X509_ECC_PRIVATE_KEY: windows_core::PCSTR = windows_core::PCSTR(82i32 as _);
pub const X509_ECC_SIGNATURE: windows_core::PCSTR = windows_core::PCSTR(47i32 as _);
pub const X509_ENHANCED_KEY_USAGE: windows_core::PCSTR = windows_core::PCSTR(36i32 as _);
pub const X509_ENUMERATED: windows_core::PCSTR = windows_core::PCSTR(29i32 as _);
pub const X509_EXTENSIONS: windows_core::PCSTR = windows_core::PCSTR(5i32 as _);
pub const X509_INTEGER: windows_core::PCSTR = windows_core::PCSTR(27i32 as _);
pub const X509_ISSUING_DIST_POINT: windows_core::PCSTR = windows_core::PCSTR(54i32 as _);
pub const X509_KEYGEN_REQUEST_TO_BE_SIGNED: windows_core::PCSTR = windows_core::PCSTR(21i32 as _);
pub const X509_KEY_ATTRIBUTES: windows_core::PCSTR = windows_core::PCSTR(10i32 as _);
pub const X509_KEY_USAGE: windows_core::PCSTR = windows_core::PCSTR(14i32 as _);
pub const X509_KEY_USAGE_RESTRICTION: windows_core::PCSTR = windows_core::PCSTR(11i32 as _);
pub const X509_LOGOTYPE_EXT: windows_core::PCSTR = windows_core::PCSTR(70i32 as _);
pub const X509_MULTI_BYTE_INTEGER: windows_core::PCSTR = windows_core::PCSTR(28i32 as _);
pub const X509_MULTI_BYTE_UINT: windows_core::PCSTR = windows_core::PCSTR(38i32 as _);
pub const X509_NAME: windows_core::PCSTR = windows_core::PCSTR(7i32 as _);
pub const X509_NAME_CONSTRAINTS: windows_core::PCSTR = windows_core::PCSTR(55i32 as _);
pub const X509_NAME_VALUE: windows_core::PCSTR = windows_core::PCSTR(6i32 as _);
pub const X509_NDR_ENCODING: u32 = 2u32;
pub const X509_OBJECT_IDENTIFIER: windows_core::PCSTR = windows_core::PCSTR(73i32 as _);
pub const X509_OCTET_STRING: windows_core::PCSTR = windows_core::PCSTR(25i32 as _);
pub const X509_PKIX_POLICY_QUALIFIER_USERNOTICE: windows_core::PCSTR = windows_core::PCSTR(46i32 as _);
pub const X509_POLICY_CONSTRAINTS: windows_core::PCSTR = windows_core::PCSTR(57i32 as _);
pub const X509_POLICY_MAPPINGS: windows_core::PCSTR = windows_core::PCSTR(56i32 as _);
pub const X509_PUBLIC_KEY_INFO: windows_core::PCSTR = windows_core::PCSTR(8i32 as _);
pub const X509_QC_STATEMENTS_EXT: windows_core::PCSTR = windows_core::PCSTR(42i32 as _);
pub const X509_SEQUENCE_OF_ANY: windows_core::PCSTR = windows_core::PCSTR(34i32 as _);
pub const X509_SUBJECT_DIR_ATTRS: windows_core::PCSTR = windows_core::PCSTR(84i32 as _);
pub const X509_SUBJECT_INFO_ACCESS: i32 = 32i32;
pub const X509_UNICODE_ANY_STRING: i32 = 24i32;
pub const X509_UNICODE_NAME: windows_core::PCSTR = windows_core::PCSTR(20i32 as _);
pub const X509_UNICODE_NAME_VALUE: windows_core::PCSTR = windows_core::PCSTR(24i32 as _);
pub const X942_DH_PARAMETERS: windows_core::PCSTR = windows_core::PCSTR(50i32 as _);
pub const X942_OTHER_INFO: windows_core::PCSTR = windows_core::PCSTR(52i32 as _);
pub const ZERO_PADDING: u32 = 3u32;
pub const cPRIV_KEY_CACHE_MAX_ITEMS_DEFAULT: u32 = 20u32;
pub const cPRIV_KEY_CACHE_PURGE_INTERVAL_SECONDS_DEFAULT: u32 = 86400u32;
pub const dwFORCE_KEY_PROTECTION_DISABLED: u32 = 0u32;
pub const dwFORCE_KEY_PROTECTION_HIGH: u32 = 2u32;
pub const dwFORCE_KEY_PROTECTION_USER_SELECT: u32 = 1u32;
pub const szBASE_CSP_DIR: windows_core::PCSTR = windows_core::s!("mscp");
pub const szCACHE_FILE: windows_core::PCSTR = windows_core::s!("cardcf");
pub const szCARD_IDENTIFIER_FILE: windows_core::PCSTR = windows_core::s!("cardid");
pub const szCONTAINER_MAP_FILE: windows_core::PCSTR = windows_core::s!("cmapfile");
pub const szFORCE_KEY_PROTECTION: windows_core::PCSTR = windows_core::s!("ForceKeyProtection");
pub const szINTERMEDIATE_CERTS_DIR: windows_core::PCSTR = windows_core::s!("mscerts");
pub const szKEY_CACHE_ENABLED: windows_core::PCSTR = windows_core::s!("CachePrivateKeys");
pub const szKEY_CACHE_SECONDS: windows_core::PCSTR = windows_core::s!("PrivateKeyLifetimeSeconds");
pub const szKEY_CRYPTOAPI_PRIVATE_KEY_OPTIONS: windows_core::PCSTR = windows_core::s!("Software\\Policies\\Microsoft\\Cryptography");
pub const szOIDVerisign_FailInfo: windows_core::PCSTR = windows_core::s!("2.16.840.1.113733.1.9.4");
pub const szOIDVerisign_MessageType: windows_core::PCSTR = windows_core::s!("2.16.840.1.113733.1.9.2");
pub const szOIDVerisign_PkiStatus: windows_core::PCSTR = windows_core::s!("2.16.840.1.113733.1.9.3");
pub const szOIDVerisign_RecipientNonce: windows_core::PCSTR = windows_core::s!("2.16.840.1.113733.1.9.6");
pub const szOIDVerisign_SenderNonce: windows_core::PCSTR = windows_core::s!("2.16.840.1.113733.1.9.5");
pub const szOIDVerisign_TransactionID: windows_core::PCSTR = windows_core::s!("2.16.840.1.113733.1.9.7");
pub const szOID_ANSI_X942: windows_core::PCSTR = windows_core::s!("1.2.840.10046");
pub const szOID_ANSI_X942_DH: windows_core::PCSTR = windows_core::s!("1.2.840.10046.2.1");
pub const szOID_ANY_APPLICATION_POLICY: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.10.12.1");
pub const szOID_ANY_CERT_POLICY: windows_core::PCSTR = windows_core::s!("2.5.29.32.0");
pub const szOID_ANY_ENHANCED_KEY_USAGE: windows_core::PCSTR = windows_core::s!("2.5.29.37.0");
pub const szOID_APPLICATION_CERT_POLICIES: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.21.10");
pub const szOID_APPLICATION_POLICY_CONSTRAINTS: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.21.12");
pub const szOID_APPLICATION_POLICY_MAPPINGS: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.21.11");
pub const szOID_ARCHIVED_KEY_ATTR: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.21.13");
pub const szOID_ARCHIVED_KEY_CERT_HASH: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.21.16");
pub const szOID_ATTEST_WHQL_CRYPTO: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.10.3.5.1");
pub const szOID_ATTR_PLATFORM_SPECIFICATION: windows_core::PCSTR = windows_core::s!("2.23.133.2.17");
pub const szOID_ATTR_SUPPORTED_ALGORITHMS: windows_core::PCSTR = windows_core::s!("2.5.4.52");
pub const szOID_ATTR_TPM_SECURITY_ASSERTIONS: windows_core::PCSTR = windows_core::s!("2.23.133.2.18");
pub const szOID_ATTR_TPM_SPECIFICATION: windows_core::PCSTR = windows_core::s!("2.23.133.2.16");
pub const szOID_AUTHORITY_INFO_ACCESS: windows_core::PCSTR = windows_core::s!("1.3.6.1.5.5.7.1.1");
pub const szOID_AUTHORITY_KEY_IDENTIFIER: windows_core::PCSTR = windows_core::s!("2.5.29.1");
pub const szOID_AUTHORITY_KEY_IDENTIFIER2: windows_core::PCSTR = windows_core::s!("2.5.29.35");
pub const szOID_AUTHORITY_REVOCATION_LIST: windows_core::PCSTR = windows_core::s!("2.5.4.38");
pub const szOID_AUTO_ENROLL_CTL_USAGE: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.20.1");
pub const szOID_BACKGROUND_OTHER_LOGOTYPE: windows_core::PCSTR = windows_core::s!("1.3.6.1.5.5.7.20.2");
pub const szOID_BASIC_CONSTRAINTS: windows_core::PCSTR = windows_core::s!("2.5.29.10");
pub const szOID_BASIC_CONSTRAINTS2: windows_core::PCSTR = windows_core::s!("2.5.29.19");
pub const szOID_BIOMETRIC_EXT: windows_core::PCSTR = windows_core::s!("1.3.6.1.5.5.7.1.2");
pub const szOID_BIOMETRIC_SIGNING: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.10.3.41");
pub const szOID_BUSINESS_CATEGORY: windows_core::PCSTR = windows_core::s!("2.5.4.15");
pub const szOID_CA_CERTIFICATE: windows_core::PCSTR = windows_core::s!("2.5.4.37");
pub const szOID_CERTIFICATE_REVOCATION_LIST: windows_core::PCSTR = windows_core::s!("2.5.4.39");
pub const szOID_CERTIFICATE_TEMPLATE: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.21.7");
pub const szOID_CERTSRV_CA_VERSION: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.21.1");
pub const szOID_CERTSRV_CROSSCA_VERSION: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.21.22");
pub const szOID_CERTSRV_PREVIOUS_CERT_HASH: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.21.2");
pub const szOID_CERT_DISALLOWED_CA_FILETIME_PROP_ID: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.10.11.128");
pub const szOID_CERT_DISALLOWED_FILETIME_PROP_ID: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.10.11.104");
pub const szOID_CERT_EXTENSIONS: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.2.1.14");
pub const szOID_CERT_ISSUER_SERIAL_NUMBER_MD5_HASH_PROP_ID: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.10.11.28");
pub const szOID_CERT_KEY_IDENTIFIER_PROP_ID: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.10.11.20");
pub const szOID_CERT_MANIFOLD: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.20.3");
pub const szOID_CERT_MD5_HASH_PROP_ID: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.10.11.4");
pub const szOID_CERT_POLICIES: windows_core::PCSTR = windows_core::s!("2.5.29.32");
pub const szOID_CERT_POLICIES_95: windows_core::PCSTR = windows_core::s!("2.5.29.3");
pub const szOID_CERT_POLICIES_95_QUALIFIER1: windows_core::PCSTR = windows_core::s!("2.16.840.1.113733.1.7.1.1");
pub const szOID_CERT_PROP_ID_PREFIX: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.10.11.");
pub const szOID_CERT_SIGNATURE_HASH_PROP_ID: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.10.11.15");
pub const szOID_CERT_STRONG_KEY_OS_1: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.72.2.1");
pub const szOID_CERT_STRONG_KEY_OS_CURRENT: windows_core::PCWSTR = windows_core::w!("1.3.6.1.4.1.311.72.2.1");
pub const szOID_CERT_STRONG_KEY_OS_PREFIX: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.72.2.");
pub const szOID_CERT_STRONG_SIGN_OS_1: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.72.1.1");
pub const szOID_CERT_STRONG_SIGN_OS_CURRENT: windows_core::PCWSTR = windows_core::w!("1.3.6.1.4.1.311.72.1.1");
pub const szOID_CERT_STRONG_SIGN_OS_PREFIX: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.72.1.");
pub const szOID_CERT_SUBJECT_NAME_MD5_HASH_PROP_ID: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.10.11.29");
pub const szOID_CMC: windows_core::PCSTR = windows_core::s!("1.3.6.1.5.5.7.7");
pub const szOID_CMC_ADD_ATTRIBUTES: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.10.10.1");
pub const szOID_CMC_ADD_EXTENSIONS: windows_core::PCSTR = windows_core::s!("1.3.6.1.5.5.7.7.8");
pub const szOID_CMC_DATA_RETURN: windows_core::PCSTR = windows_core::s!("1.3.6.1.5.5.7.7.4");
pub const szOID_CMC_DECRYPTED_POP: windows_core::PCSTR = windows_core::s!("1.3.6.1.5.5.7.7.10");
pub const szOID_CMC_ENCRYPTED_POP: windows_core::PCSTR = windows_core::s!("1.3.6.1.5.5.7.7.9");
pub const szOID_CMC_GET_CERT: windows_core::PCSTR = windows_core::s!("1.3.6.1.5.5.7.7.15");
pub const szOID_CMC_GET_CRL: windows_core::PCSTR = windows_core::s!("1.3.6.1.5.5.7.7.16");
pub const szOID_CMC_IDENTIFICATION: windows_core::PCSTR = windows_core::s!("1.3.6.1.5.5.7.7.2");
pub const szOID_CMC_IDENTITY_PROOF: windows_core::PCSTR = windows_core::s!("1.3.6.1.5.5.7.7.3");
pub const szOID_CMC_ID_CONFIRM_CERT_ACCEPTANCE: windows_core::PCSTR = windows_core::s!("1.3.6.1.5.5.7.7.24");
pub const szOID_CMC_ID_POP_LINK_RANDOM: windows_core::PCSTR = windows_core::s!("1.3.6.1.5.5.7.7.22");
pub const szOID_CMC_ID_POP_LINK_WITNESS: windows_core::PCSTR = windows_core::s!("1.3.6.1.5.5.7.7.23");
pub const szOID_CMC_LRA_POP_WITNESS: windows_core::PCSTR = windows_core::s!("1.3.6.1.5.5.7.7.11");
pub const szOID_CMC_QUERY_PENDING: windows_core::PCSTR = windows_core::s!("1.3.6.1.5.5.7.7.21");
pub const szOID_CMC_RECIPIENT_NONCE: windows_core::PCSTR = windows_core::s!("1.3.6.1.5.5.7.7.7");
pub const szOID_CMC_REG_INFO: windows_core::PCSTR = windows_core::s!("1.3.6.1.5.5.7.7.18");
pub const szOID_CMC_RESPONSE_INFO: windows_core::PCSTR = windows_core::s!("1.3.6.1.5.5.7.7.19");
pub const szOID_CMC_REVOKE_REQUEST: windows_core::PCSTR = windows_core::s!("1.3.6.1.5.5.7.7.17");
pub const szOID_CMC_SENDER_NONCE: windows_core::PCSTR = windows_core::s!("1.3.6.1.5.5.7.7.6");
pub const szOID_CMC_STATUS_INFO: windows_core::PCSTR = windows_core::s!("1.3.6.1.5.5.7.7.1");
pub const szOID_CMC_TRANSACTION_ID: windows_core::PCSTR = windows_core::s!("1.3.6.1.5.5.7.7.5");
pub const szOID_CN_ECDSA_SHA256: windows_core::PCSTR = windows_core::s!("1.2.156.11235.1.1.1");
pub const szOID_COMMON_NAME: windows_core::PCSTR = windows_core::s!("2.5.4.3");
pub const szOID_COUNTRY_NAME: windows_core::PCSTR = windows_core::s!("2.5.4.6");
pub const szOID_CRL_DIST_POINTS: windows_core::PCSTR = windows_core::s!("2.5.29.31");
pub const szOID_CRL_NEXT_PUBLISH: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.21.4");
pub const szOID_CRL_NUMBER: windows_core::PCSTR = windows_core::s!("2.5.29.20");
pub const szOID_CRL_REASON_CODE: windows_core::PCSTR = windows_core::s!("2.5.29.21");
pub const szOID_CRL_SELF_CDP: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.21.14");
pub const szOID_CRL_VIRTUAL_BASE: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.21.3");
pub const szOID_CROSS_CERTIFICATE_PAIR: windows_core::PCSTR = windows_core::s!("2.5.4.40");
pub const szOID_CROSS_CERT_DIST_POINTS: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.10.9.1");
pub const szOID_CTL: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.10.1");
pub const szOID_CT_CERT_SCTLIST: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.11129.2.4.2");
pub const szOID_CT_PKI_DATA: windows_core::PCSTR = windows_core::s!("1.3.6.1.5.5.7.12.2");
pub const szOID_CT_PKI_RESPONSE: windows_core::PCSTR = windows_core::s!("1.3.6.1.5.5.7.12.3");
pub const szOID_DELTA_CRL_INDICATOR: windows_core::PCSTR = windows_core::s!("2.5.29.27");
pub const szOID_DESCRIPTION: windows_core::PCSTR = windows_core::s!("2.5.4.13");
pub const szOID_DESTINATION_INDICATOR: windows_core::PCSTR = windows_core::s!("2.5.4.27");
pub const szOID_DEVICE_SERIAL_NUMBER: windows_core::PCSTR = windows_core::s!("2.5.4.5");
pub const szOID_DH_SINGLE_PASS_STDDH_SHA1_KDF: windows_core::PCSTR = windows_core::s!("1.3.133.16.840.63.0.2");
pub const szOID_DH_SINGLE_PASS_STDDH_SHA256_KDF: windows_core::PCSTR = windows_core::s!("1.3.132.1.11.1");
pub const szOID_DH_SINGLE_PASS_STDDH_SHA384_KDF: windows_core::PCSTR = windows_core::s!("1.3.132.1.11.2");
pub const szOID_DISALLOWED_HASH: windows_core::PCWSTR = windows_core::w!("1.3.6.1.4.1.311.10.11.15");
pub const szOID_DISALLOWED_LIST: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.10.3.30");
pub const szOID_DN_QUALIFIER: windows_core::PCSTR = windows_core::s!("2.5.4.46");
pub const szOID_DOMAIN_COMPONENT: windows_core::PCSTR = windows_core::s!("0.9.2342.19200300.100.1.25");
pub const szOID_DRM: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.10.5.1");
pub const szOID_DRM_INDIVIDUALIZATION: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.10.5.2");
pub const szOID_DS: windows_core::PCSTR = windows_core::s!("2.5");
pub const szOID_DSALG: windows_core::PCSTR = windows_core::s!("2.5.8");
pub const szOID_DSALG_CRPT: windows_core::PCSTR = windows_core::s!("2.5.8.1");
pub const szOID_DSALG_HASH: windows_core::PCSTR = windows_core::s!("2.5.8.2");
pub const szOID_DSALG_RSA: windows_core::PCSTR = windows_core::s!("2.5.8.1.1");
pub const szOID_DSALG_SIGN: windows_core::PCSTR = windows_core::s!("2.5.8.3");
pub const szOID_DS_EMAIL_REPLICATION: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.21.19");
pub const szOID_DYNAMIC_CODE_GEN_SIGNER: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.76.5.1");
pub const szOID_ECC_CURVE_BRAINPOOLP160R1: windows_core::PCSTR = windows_core::s!("1.3.36.3.3.2.8.1.1.1");
pub const szOID_ECC_CURVE_BRAINPOOLP160T1: windows_core::PCSTR = windows_core::s!("1.3.36.3.3.2.8.1.1.2");
pub const szOID_ECC_CURVE_BRAINPOOLP192R1: windows_core::PCSTR = windows_core::s!("1.3.36.3.3.2.8.1.1.3");
pub const szOID_ECC_CURVE_BRAINPOOLP192T1: windows_core::PCSTR = windows_core::s!("1.3.36.3.3.2.8.1.1.4");
pub const szOID_ECC_CURVE_BRAINPOOLP224R1: windows_core::PCSTR = windows_core::s!("1.3.36.3.3.2.8.1.1.5");
pub const szOID_ECC_CURVE_BRAINPOOLP224T1: windows_core::PCSTR = windows_core::s!("1.3.36.3.3.2.8.1.1.6");
pub const szOID_ECC_CURVE_BRAINPOOLP256R1: windows_core::PCSTR = windows_core::s!("1.3.36.3.3.2.8.1.1.7");
pub const szOID_ECC_CURVE_BRAINPOOLP256T1: windows_core::PCSTR = windows_core::s!("1.3.36.3.3.2.8.1.1.8");
pub const szOID_ECC_CURVE_BRAINPOOLP320R1: windows_core::PCSTR = windows_core::s!("1.3.36.3.3.2.8.1.1.9");
pub const szOID_ECC_CURVE_BRAINPOOLP320T1: windows_core::PCSTR = windows_core::s!("1.3.36.3.3.2.8.1.1.10");
pub const szOID_ECC_CURVE_BRAINPOOLP384R1: windows_core::PCSTR = windows_core::s!("1.3.36.3.3.2.8.1.1.11");
pub const szOID_ECC_CURVE_BRAINPOOLP384T1: windows_core::PCSTR = windows_core::s!("1.3.36.3.3.2.8.1.1.12");
pub const szOID_ECC_CURVE_BRAINPOOLP512R1: windows_core::PCSTR = windows_core::s!("1.3.36.3.3.2.8.1.1.13");
pub const szOID_ECC_CURVE_BRAINPOOLP512T1: windows_core::PCSTR = windows_core::s!("1.3.36.3.3.2.8.1.1.14");
pub const szOID_ECC_CURVE_EC192WAPI: windows_core::PCSTR = windows_core::s!("1.2.156.11235.1.1.2.1");
pub const szOID_ECC_CURVE_NISTP192: windows_core::PCSTR = windows_core::s!("1.2.840.10045.3.1.1");
pub const szOID_ECC_CURVE_NISTP224: windows_core::PCSTR = windows_core::s!("1.3.132.0.33");
pub const szOID_ECC_CURVE_NISTP256: windows_core::PCWSTR = windows_core::w!("1.2.840.10045.3.1.7");
pub const szOID_ECC_CURVE_NISTP384: windows_core::PCWSTR = windows_core::w!("1.3.132.0.34");
pub const szOID_ECC_CURVE_NISTP521: windows_core::PCWSTR = windows_core::w!("1.3.132.0.35");
pub const szOID_ECC_CURVE_P256: windows_core::PCSTR = windows_core::s!("1.2.840.10045.3.1.7");
pub const szOID_ECC_CURVE_P384: windows_core::PCSTR = windows_core::s!("1.3.132.0.34");
pub const szOID_ECC_CURVE_P521: windows_core::PCSTR = windows_core::s!("1.3.132.0.35");
pub const szOID_ECC_CURVE_SECP160K1: windows_core::PCSTR = windows_core::s!("1.3.132.0.9");
pub const szOID_ECC_CURVE_SECP160R1: windows_core::PCSTR = windows_core::s!("1.3.132.0.8");
pub const szOID_ECC_CURVE_SECP160R2: windows_core::PCSTR = windows_core::s!("1.3.132.0.30");
pub const szOID_ECC_CURVE_SECP192K1: windows_core::PCSTR = windows_core::s!("1.3.132.0.31");
pub const szOID_ECC_CURVE_SECP192R1: windows_core::PCWSTR = windows_core::w!("1.2.840.10045.3.1.1");
pub const szOID_ECC_CURVE_SECP224K1: windows_core::PCSTR = windows_core::s!("1.3.132.0.32");
pub const szOID_ECC_CURVE_SECP224R1: windows_core::PCWSTR = windows_core::w!("1.3.132.0.33");
pub const szOID_ECC_CURVE_SECP256K1: windows_core::PCSTR = windows_core::s!("1.3.132.0.10");
pub const szOID_ECC_CURVE_SECP256R1: windows_core::PCWSTR = windows_core::w!("1.2.840.10045.3.1.7");
pub const szOID_ECC_CURVE_SECP384R1: windows_core::PCWSTR = windows_core::w!("1.3.132.0.34");
pub const szOID_ECC_CURVE_SECP521R1: windows_core::PCWSTR = windows_core::w!("1.3.132.0.35");
pub const szOID_ECC_CURVE_WTLS12: windows_core::PCWSTR = windows_core::w!("1.3.132.0.33");
pub const szOID_ECC_CURVE_WTLS7: windows_core::PCWSTR = windows_core::w!("1.3.132.0.30");
pub const szOID_ECC_CURVE_WTLS9: windows_core::PCSTR = windows_core::s!("2.23.43.1.4.9");
pub const szOID_ECC_CURVE_X962P192V1: windows_core::PCSTR = windows_core::s!("1.2.840.10045.3.1.1");
pub const szOID_ECC_CURVE_X962P192V2: windows_core::PCSTR = windows_core::s!("1.2.840.10045.3.1.2");
pub const szOID_ECC_CURVE_X962P192V3: windows_core::PCSTR = windows_core::s!("1.2.840.10045.3.1.3");
pub const szOID_ECC_CURVE_X962P239V1: windows_core::PCSTR = windows_core::s!("1.2.840.10045.3.1.4");
pub const szOID_ECC_CURVE_X962P239V2: windows_core::PCSTR = windows_core::s!("1.2.840.10045.3.1.5");
pub const szOID_ECC_CURVE_X962P239V3: windows_core::PCSTR = windows_core::s!("1.2.840.10045.3.1.6");
pub const szOID_ECC_CURVE_X962P256V1: windows_core::PCWSTR = windows_core::w!("1.2.840.10045.3.1.7");
pub const szOID_ECC_PUBLIC_KEY: windows_core::PCSTR = windows_core::s!("1.2.840.10045.2.1");
pub const szOID_ECDSA_SHA1: windows_core::PCSTR = windows_core::s!("1.2.840.10045.4.1");
pub const szOID_ECDSA_SHA256: windows_core::PCSTR = windows_core::s!("1.2.840.10045.4.3.2");
pub const szOID_ECDSA_SHA384: windows_core::PCSTR = windows_core::s!("1.2.840.10045.4.3.3");
pub const szOID_ECDSA_SHA512: windows_core::PCSTR = windows_core::s!("1.2.840.10045.4.3.4");
pub const szOID_ECDSA_SPECIFIED: windows_core::PCSTR = windows_core::s!("1.2.840.10045.4.3");
pub const szOID_EFS_RECOVERY: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.10.3.4.1");
pub const szOID_EMBEDDED_NT_CRYPTO: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.10.3.8");
pub const szOID_ENCLAVE_SIGNING: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.10.3.42");
pub const szOID_ENCRYPTED_KEY_HASH: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.21.21");
pub const szOID_ENHANCED_KEY_USAGE: windows_core::PCSTR = windows_core::s!("2.5.29.37");
pub const szOID_ENROLLMENT_AGENT: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.20.2.1");
pub const szOID_ENROLLMENT_CSP_PROVIDER: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.13.2.2");
pub const szOID_ENROLLMENT_NAME_VALUE_PAIR: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.13.2.1");
pub const szOID_ENROLL_AIK_INFO: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.21.39");
pub const szOID_ENROLL_ATTESTATION_CHALLENGE: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.21.28");
pub const szOID_ENROLL_ATTESTATION_STATEMENT: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.21.24");
pub const szOID_ENROLL_CAXCHGCERT_HASH: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.21.27");
pub const szOID_ENROLL_CERTTYPE_EXTENSION: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.20.2");
pub const szOID_ENROLL_EKPUB_CHALLENGE: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.21.26");
pub const szOID_ENROLL_EKVERIFYCERT: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.21.31");
pub const szOID_ENROLL_EKVERIFYCREDS: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.21.32");
pub const szOID_ENROLL_EKVERIFYKEY: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.21.30");
pub const szOID_ENROLL_EK_CA_KEYID: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.21.43");
pub const szOID_ENROLL_EK_INFO: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.21.23");
pub const szOID_ENROLL_ENCRYPTION_ALGORITHM: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.21.29");
pub const szOID_ENROLL_KEY_AFFINITY: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.21.41");
pub const szOID_ENROLL_KSP_NAME: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.21.25");
pub const szOID_ENROLL_SCEP_CHALLENGE_ANSWER: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.21.35");
pub const szOID_ENROLL_SCEP_CLIENT_REQUEST: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.21.37");
pub const szOID_ENROLL_SCEP_ERROR: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.21.33");
pub const szOID_ENROLL_SCEP_SERVER_MESSAGE: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.21.38");
pub const szOID_ENROLL_SCEP_SERVER_SECRET: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.21.40");
pub const szOID_ENROLL_SCEP_SERVER_STATE: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.21.34");
pub const szOID_ENROLL_SCEP_SIGNER_HASH: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.21.42");
pub const szOID_ENTERPRISE_OID_ROOT: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.21.8");
pub const szOID_EV_RDN_COUNTRY: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.60.2.1.3");
pub const szOID_EV_RDN_LOCALE: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.60.2.1.1");
pub const szOID_EV_RDN_STATE_OR_PROVINCE: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.60.2.1.2");
pub const szOID_EV_WHQL_CRYPTO: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.10.3.39");
pub const szOID_FACSIMILE_TELEPHONE_NUMBER: windows_core::PCSTR = windows_core::s!("2.5.4.23");
pub const szOID_FRESHEST_CRL: windows_core::PCSTR = windows_core::s!("2.5.29.46");
pub const szOID_GIVEN_NAME: windows_core::PCSTR = windows_core::s!("2.5.4.42");
pub const szOID_HPKP_DOMAIN_NAME_CTL: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.10.3.60");
pub const szOID_HPKP_HEADER_VALUE_CTL: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.10.3.61");
pub const szOID_INFOSEC: windows_core::PCSTR = windows_core::s!("2.16.840.1.101.2.1");
pub const szOID_INFOSEC_SuiteAConfidentiality: windows_core::PCSTR = windows_core::s!("2.16.840.1.101.2.1.1.14");
pub const szOID_INFOSEC_SuiteAIntegrity: windows_core::PCSTR = windows_core::s!("2.16.840.1.101.2.1.1.15");
pub const szOID_INFOSEC_SuiteAKMandSig: windows_core::PCSTR = windows_core::s!("2.16.840.1.101.2.1.1.18");
pub const szOID_INFOSEC_SuiteAKeyManagement: windows_core::PCSTR = windows_core::s!("2.16.840.1.101.2.1.1.17");
pub const szOID_INFOSEC_SuiteASignature: windows_core::PCSTR = windows_core::s!("2.16.840.1.101.2.1.1.13");
pub const szOID_INFOSEC_SuiteATokenProtection: windows_core::PCSTR = windows_core::s!("2.16.840.1.101.2.1.1.16");
pub const szOID_INFOSEC_mosaicConfidentiality: windows_core::PCSTR = windows_core::s!("2.16.840.1.101.2.1.1.4");
pub const szOID_INFOSEC_mosaicIntegrity: windows_core::PCSTR = windows_core::s!("2.16.840.1.101.2.1.1.6");
pub const szOID_INFOSEC_mosaicKMandSig: windows_core::PCSTR = windows_core::s!("2.16.840.1.101.2.1.1.12");
pub const szOID_INFOSEC_mosaicKMandUpdSig: windows_core::PCSTR = windows_core::s!("2.16.840.1.101.2.1.1.20");
pub const szOID_INFOSEC_mosaicKeyManagement: windows_core::PCSTR = windows_core::s!("2.16.840.1.101.2.1.1.10");
pub const szOID_INFOSEC_mosaicSignature: windows_core::PCSTR = windows_core::s!("2.16.840.1.101.2.1.1.2");
pub const szOID_INFOSEC_mosaicTokenProtection: windows_core::PCSTR = windows_core::s!("2.16.840.1.101.2.1.1.8");
pub const szOID_INFOSEC_mosaicUpdatedInteg: windows_core::PCSTR = windows_core::s!("2.16.840.1.101.2.1.1.21");
pub const szOID_INFOSEC_mosaicUpdatedSig: windows_core::PCSTR = windows_core::s!("2.16.840.1.101.2.1.1.19");
pub const szOID_INFOSEC_sdnsConfidentiality: windows_core::PCSTR = windows_core::s!("2.16.840.1.101.2.1.1.3");
pub const szOID_INFOSEC_sdnsIntegrity: windows_core::PCSTR = windows_core::s!("2.16.840.1.101.2.1.1.5");
pub const szOID_INFOSEC_sdnsKMandSig: windows_core::PCSTR = windows_core::s!("2.16.840.1.101.2.1.1.11");
pub const szOID_INFOSEC_sdnsKeyManagement: windows_core::PCSTR = windows_core::s!("2.16.840.1.101.2.1.1.9");
pub const szOID_INFOSEC_sdnsSignature: windows_core::PCSTR = windows_core::s!("2.16.840.1.101.2.1.1.1");
pub const szOID_INFOSEC_sdnsTokenProtection: windows_core::PCSTR = windows_core::s!("2.16.840.1.101.2.1.1.7");
pub const szOID_INHIBIT_ANY_POLICY: windows_core::PCSTR = windows_core::s!("2.5.29.54");
pub const szOID_INITIALS: windows_core::PCSTR = windows_core::s!("2.5.4.43");
pub const szOID_INTERNATIONALIZED_EMAIL_ADDRESS: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.20.2.4");
pub const szOID_INTERNATIONAL_ISDN_NUMBER: windows_core::PCSTR = windows_core::s!("2.5.4.25");
pub const szOID_IPSEC_KP_IKE_INTERMEDIATE: windows_core::PCSTR = windows_core::s!("1.3.6.1.5.5.8.2.2");
pub const szOID_ISSUED_CERT_HASH: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.21.17");
pub const szOID_ISSUER_ALT_NAME: windows_core::PCSTR = windows_core::s!("2.5.29.8");
pub const szOID_ISSUER_ALT_NAME2: windows_core::PCSTR = windows_core::s!("2.5.29.18");
pub const szOID_ISSUING_DIST_POINT: windows_core::PCSTR = windows_core::s!("2.5.29.28");
pub const szOID_IUM_SIGNING: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.10.3.37");
pub const szOID_KEYID_RDN: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.10.7.1");
pub const szOID_KEY_ATTRIBUTES: windows_core::PCSTR = windows_core::s!("2.5.29.2");
pub const szOID_KEY_USAGE: windows_core::PCSTR = windows_core::s!("2.5.29.15");
pub const szOID_KEY_USAGE_RESTRICTION: windows_core::PCSTR = windows_core::s!("2.5.29.4");
pub const szOID_KP_CA_EXCHANGE: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.21.5");
pub const szOID_KP_CSP_SIGNATURE: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.10.3.16");
pub const szOID_KP_CTL_USAGE_SIGNING: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.10.3.1");
pub const szOID_KP_DOCUMENT_SIGNING: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.10.3.12");
pub const szOID_KP_EFS: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.10.3.4");
pub const szOID_KP_FLIGHT_SIGNING: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.10.3.27");
pub const szOID_KP_KERNEL_MODE_CODE_SIGNING: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.61.1.1");
pub const szOID_KP_KERNEL_MODE_HAL_EXTENSION_SIGNING: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.61.5.1");
pub const szOID_KP_KERNEL_MODE_TRUSTED_BOOT_SIGNING: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.61.4.1");
pub const szOID_KP_KEY_RECOVERY: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.10.3.11");
pub const szOID_KP_KEY_RECOVERY_AGENT: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.21.6");
pub const szOID_KP_LIFETIME_SIGNING: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.10.3.13");
pub const szOID_KP_MOBILE_DEVICE_SOFTWARE: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.10.3.14");
pub const szOID_KP_PRIVACY_CA: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.21.36");
pub const szOID_KP_QUALIFIED_SUBORDINATION: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.10.3.10");
pub const szOID_KP_SMARTCARD_LOGON: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.20.2.2");
pub const szOID_KP_SMART_DISPLAY: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.10.3.15");
pub const szOID_KP_TIME_STAMP_SIGNING: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.10.3.2");
pub const szOID_KP_TPM_AIK_CERTIFICATE: windows_core::PCSTR = windows_core::s!("2.23.133.8.3");
pub const szOID_KP_TPM_EK_CERTIFICATE: windows_core::PCSTR = windows_core::s!("2.23.133.8.1");
pub const szOID_KP_TPM_PLATFORM_CERTIFICATE: windows_core::PCSTR = windows_core::s!("2.23.133.8.2");
pub const szOID_LEGACY_POLICY_MAPPINGS: windows_core::PCSTR = windows_core::s!("2.5.29.5");
pub const szOID_LICENSES: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.10.6.1");
pub const szOID_LICENSE_SERVER: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.10.6.2");
pub const szOID_LOCALITY_NAME: windows_core::PCSTR = windows_core::s!("2.5.4.7");
pub const szOID_LOCAL_MACHINE_KEYSET: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.17.2");
pub const szOID_LOGOTYPE_EXT: windows_core::PCSTR = windows_core::s!("1.3.6.1.5.5.7.1.12");
pub const szOID_LOYALTY_OTHER_LOGOTYPE: windows_core::PCSTR = windows_core::s!("1.3.6.1.5.5.7.20.1");
pub const szOID_MEMBER: windows_core::PCSTR = windows_core::s!("2.5.4.31");
pub const szOID_MICROSOFT_PUBLISHER_SIGNER: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.76.8.1");
pub const szOID_NAME_CONSTRAINTS: windows_core::PCSTR = windows_core::s!("2.5.29.30");
pub const szOID_NETSCAPE: windows_core::PCSTR = windows_core::s!("2.16.840.1.113730");
pub const szOID_NETSCAPE_BASE_URL: windows_core::PCSTR = windows_core::s!("2.16.840.1.113730.1.2");
pub const szOID_NETSCAPE_CA_POLICY_URL: windows_core::PCSTR = windows_core::s!("2.16.840.1.113730.1.8");
pub const szOID_NETSCAPE_CA_REVOCATION_URL: windows_core::PCSTR = windows_core::s!("2.16.840.1.113730.1.4");
pub const szOID_NETSCAPE_CERT_EXTENSION: windows_core::PCSTR = windows_core::s!("2.16.840.1.113730.1");
pub const szOID_NETSCAPE_CERT_RENEWAL_URL: windows_core::PCSTR = windows_core::s!("2.16.840.1.113730.1.7");
pub const szOID_NETSCAPE_CERT_SEQUENCE: windows_core::PCSTR = windows_core::s!("2.16.840.1.113730.2.5");
pub const szOID_NETSCAPE_CERT_TYPE: windows_core::PCSTR = windows_core::s!("2.16.840.1.113730.1.1");
pub const szOID_NETSCAPE_COMMENT: windows_core::PCSTR = windows_core::s!("2.16.840.1.113730.1.13");
pub const szOID_NETSCAPE_DATA_TYPE: windows_core::PCSTR = windows_core::s!("2.16.840.1.113730.2");
pub const szOID_NETSCAPE_REVOCATION_URL: windows_core::PCSTR = windows_core::s!("2.16.840.1.113730.1.3");
pub const szOID_NETSCAPE_SSL_SERVER_NAME: windows_core::PCSTR = windows_core::s!("2.16.840.1.113730.1.12");
pub const szOID_NEXT_UPDATE_LOCATION: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.10.2");
pub const szOID_NIST_AES128_CBC: windows_core::PCSTR = windows_core::s!("2.16.840.1.101.3.4.1.2");
pub const szOID_NIST_AES128_WRAP: windows_core::PCSTR = windows_core::s!("2.16.840.1.101.3.4.1.5");
pub const szOID_NIST_AES192_CBC: windows_core::PCSTR = windows_core::s!("2.16.840.1.101.3.4.1.22");
pub const szOID_NIST_AES192_WRAP: windows_core::PCSTR = windows_core::s!("2.16.840.1.101.3.4.1.25");
pub const szOID_NIST_AES256_CBC: windows_core::PCSTR = windows_core::s!("2.16.840.1.101.3.4.1.42");
pub const szOID_NIST_AES256_WRAP: windows_core::PCSTR = windows_core::s!("2.16.840.1.101.3.4.1.45");
pub const szOID_NIST_sha256: windows_core::PCSTR = windows_core::s!("2.16.840.1.101.3.4.2.1");
pub const szOID_NIST_sha384: windows_core::PCSTR = windows_core::s!("2.16.840.1.101.3.4.2.2");
pub const szOID_NIST_sha512: windows_core::PCSTR = windows_core::s!("2.16.840.1.101.3.4.2.3");
pub const szOID_NT5_CRYPTO: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.10.3.6");
pub const szOID_NTDS_CA_SECURITY_EXT: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.25.2");
pub const szOID_NTDS_OBJECTSID: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.25.2.1");
pub const szOID_NTDS_REPLICATION: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.25.1");
pub const szOID_NT_PRINCIPAL_NAME: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.20.2.3");
pub const szOID_OEM_WHQL_CRYPTO: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.10.3.7");
pub const szOID_OIW: windows_core::PCSTR = windows_core::s!("1.3.14");
pub const szOID_OIWDIR: windows_core::PCSTR = windows_core::s!("1.3.14.7.2");
pub const szOID_OIWDIR_CRPT: windows_core::PCSTR = windows_core::s!("1.3.14.7.2.1");
pub const szOID_OIWDIR_HASH: windows_core::PCSTR = windows_core::s!("1.3.14.7.2.2");
pub const szOID_OIWDIR_SIGN: windows_core::PCSTR = windows_core::s!("1.3.14.7.2.3");
pub const szOID_OIWDIR_md2: windows_core::PCSTR = windows_core::s!("1.3.14.7.2.2.1");
pub const szOID_OIWDIR_md2RSA: windows_core::PCSTR = windows_core::s!("1.3.14.7.2.3.1");
pub const szOID_OIWSEC: windows_core::PCSTR = windows_core::s!("1.3.14.3.2");
pub const szOID_OIWSEC_desCBC: windows_core::PCSTR = windows_core::s!("1.3.14.3.2.7");
pub const szOID_OIWSEC_desCFB: windows_core::PCSTR = windows_core::s!("1.3.14.3.2.9");
pub const szOID_OIWSEC_desECB: windows_core::PCSTR = windows_core::s!("1.3.14.3.2.6");
pub const szOID_OIWSEC_desEDE: windows_core::PCSTR = windows_core::s!("1.3.14.3.2.17");
pub const szOID_OIWSEC_desMAC: windows_core::PCSTR = windows_core::s!("1.3.14.3.2.10");
pub const szOID_OIWSEC_desOFB: windows_core::PCSTR = windows_core::s!("1.3.14.3.2.8");
pub const szOID_OIWSEC_dhCommMod: windows_core::PCSTR = windows_core::s!("1.3.14.3.2.16");
pub const szOID_OIWSEC_dsa: windows_core::PCSTR = windows_core::s!("1.3.14.3.2.12");
pub const szOID_OIWSEC_dsaComm: windows_core::PCSTR = windows_core::s!("1.3.14.3.2.20");
pub const szOID_OIWSEC_dsaCommSHA: windows_core::PCSTR = windows_core::s!("1.3.14.3.2.21");
pub const szOID_OIWSEC_dsaCommSHA1: windows_core::PCSTR = windows_core::s!("1.3.14.3.2.28");
pub const szOID_OIWSEC_dsaSHA1: windows_core::PCSTR = windows_core::s!("1.3.14.3.2.27");
pub const szOID_OIWSEC_keyHashSeal: windows_core::PCSTR = windows_core::s!("1.3.14.3.2.23");
pub const szOID_OIWSEC_md2RSASign: windows_core::PCSTR = windows_core::s!("1.3.14.3.2.24");
pub const szOID_OIWSEC_md4RSA: windows_core::PCSTR = windows_core::s!("1.3.14.3.2.2");
pub const szOID_OIWSEC_md4RSA2: windows_core::PCSTR = windows_core::s!("1.3.14.3.2.4");
pub const szOID_OIWSEC_md5RSA: windows_core::PCSTR = windows_core::s!("1.3.14.3.2.3");
pub const szOID_OIWSEC_md5RSASign: windows_core::PCSTR = windows_core::s!("1.3.14.3.2.25");
pub const szOID_OIWSEC_mdc2: windows_core::PCSTR = windows_core::s!("1.3.14.3.2.19");
pub const szOID_OIWSEC_mdc2RSA: windows_core::PCSTR = windows_core::s!("1.3.14.3.2.14");
pub const szOID_OIWSEC_rsaSign: windows_core::PCSTR = windows_core::s!("1.3.14.3.2.11");
pub const szOID_OIWSEC_rsaXchg: windows_core::PCSTR = windows_core::s!("1.3.14.3.2.22");
pub const szOID_OIWSEC_sha: windows_core::PCSTR = windows_core::s!("1.3.14.3.2.18");
pub const szOID_OIWSEC_sha1: windows_core::PCSTR = windows_core::s!("1.3.14.3.2.26");
pub const szOID_OIWSEC_sha1RSASign: windows_core::PCSTR = windows_core::s!("1.3.14.3.2.29");
pub const szOID_OIWSEC_shaDSA: windows_core::PCSTR = windows_core::s!("1.3.14.3.2.13");
pub const szOID_OIWSEC_shaRSA: windows_core::PCSTR = windows_core::s!("1.3.14.3.2.15");
pub const szOID_ORGANIZATIONAL_UNIT_NAME: windows_core::PCSTR = windows_core::s!("2.5.4.11");
pub const szOID_ORGANIZATION_NAME: windows_core::PCSTR = windows_core::s!("2.5.4.10");
pub const szOID_OS_VERSION: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.13.2.3");
pub const szOID_OWNER: windows_core::PCSTR = windows_core::s!("2.5.4.32");
pub const szOID_PHYSICAL_DELIVERY_OFFICE_NAME: windows_core::PCSTR = windows_core::s!("2.5.4.19");
pub const szOID_PIN_RULES_CTL: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.10.3.32");
pub const szOID_PIN_RULES_DOMAIN_NAME: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.10.3.34");
pub const szOID_PIN_RULES_EXT: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.10.3.33");
pub const szOID_PIN_RULES_LOG_END_DATE_EXT: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.10.3.35");
pub const szOID_PIN_RULES_SIGNER: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.10.3.31");
pub const szOID_PKCS: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1");
pub const szOID_PKCS_1: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.1");
pub const szOID_PKCS_10: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.10");
pub const szOID_PKCS_12: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.12");
pub const szOID_PKCS_12_EXTENDED_ATTRIBUTES: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.17.3");
pub const szOID_PKCS_12_FRIENDLY_NAME_ATTR: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.9.20");
pub const szOID_PKCS_12_KEY_PROVIDER_NAME_ATTR: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.17.1");
pub const szOID_PKCS_12_LOCAL_KEY_ID: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.9.21");
pub const szOID_PKCS_12_PROTECTED_PASSWORD_SECRET_BAG_TYPE_ID: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.17.4");
pub const szOID_PKCS_12_PbeIds: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.12.1");
pub const szOID_PKCS_12_pbeWithSHA1And128BitRC2: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.12.1.5");
pub const szOID_PKCS_12_pbeWithSHA1And128BitRC4: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.12.1.1");
pub const szOID_PKCS_12_pbeWithSHA1And2KeyTripleDES: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.12.1.4");
pub const szOID_PKCS_12_pbeWithSHA1And3KeyTripleDES: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.12.1.3");
pub const szOID_PKCS_12_pbeWithSHA1And40BitRC2: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.12.1.6");
pub const szOID_PKCS_12_pbeWithSHA1And40BitRC4: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.12.1.2");
pub const szOID_PKCS_2: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.2");
pub const szOID_PKCS_3: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.3");
pub const szOID_PKCS_4: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.4");
pub const szOID_PKCS_5: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.5");
pub const szOID_PKCS_5_PBES2: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.5.13");
pub const szOID_PKCS_5_PBKDF2: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.5.12");
pub const szOID_PKCS_6: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.6");
pub const szOID_PKCS_7: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.7");
pub const szOID_PKCS_7_DATA: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.7.1");
pub const szOID_PKCS_7_DIGESTED: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.7.5");
pub const szOID_PKCS_7_ENCRYPTED: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.7.6");
pub const szOID_PKCS_7_ENVELOPED: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.7.3");
pub const szOID_PKCS_7_SIGNED: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.7.2");
pub const szOID_PKCS_7_SIGNEDANDENVELOPED: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.7.4");
pub const szOID_PKCS_8: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.8");
pub const szOID_PKCS_9: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.9");
pub const szOID_PKCS_9_CONTENT_TYPE: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.9.3");
pub const szOID_PKCS_9_MESSAGE_DIGEST: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.9.4");
pub const szOID_PKINIT_KP_KDC: windows_core::PCSTR = windows_core::s!("1.3.6.1.5.2.3.5");
pub const szOID_PKIX: windows_core::PCSTR = windows_core::s!("1.3.6.1.5.5.7");
pub const szOID_PKIX_ACC_DESCR: windows_core::PCSTR = windows_core::s!("1.3.6.1.5.5.7.48");
pub const szOID_PKIX_CA_ISSUERS: windows_core::PCSTR = windows_core::s!("1.3.6.1.5.5.7.48.2");
pub const szOID_PKIX_CA_REPOSITORY: windows_core::PCSTR = windows_core::s!("1.3.6.1.5.5.7.48.5");
pub const szOID_PKIX_KP: windows_core::PCSTR = windows_core::s!("1.3.6.1.5.5.7.3");
pub const szOID_PKIX_KP_CLIENT_AUTH: windows_core::PCSTR = windows_core::s!("1.3.6.1.5.5.7.3.2");
pub const szOID_PKIX_KP_CODE_SIGNING: windows_core::PCSTR = windows_core::s!("1.3.6.1.5.5.7.3.3");
pub const szOID_PKIX_KP_EMAIL_PROTECTION: windows_core::PCSTR = windows_core::s!("1.3.6.1.5.5.7.3.4");
pub const szOID_PKIX_KP_IPSEC_END_SYSTEM: windows_core::PCSTR = windows_core::s!("1.3.6.1.5.5.7.3.5");
pub const szOID_PKIX_KP_IPSEC_TUNNEL: windows_core::PCSTR = windows_core::s!("1.3.6.1.5.5.7.3.6");
pub const szOID_PKIX_KP_IPSEC_USER: windows_core::PCSTR = windows_core::s!("1.3.6.1.5.5.7.3.7");
pub const szOID_PKIX_KP_OCSP_SIGNING: windows_core::PCSTR = windows_core::s!("1.3.6.1.5.5.7.3.9");
pub const szOID_PKIX_KP_SERVER_AUTH: windows_core::PCSTR = windows_core::s!("1.3.6.1.5.5.7.3.1");
pub const szOID_PKIX_KP_TIMESTAMP_SIGNING: windows_core::PCSTR = windows_core::s!("1.3.6.1.5.5.7.3.8");
pub const szOID_PKIX_NO_SIGNATURE: windows_core::PCSTR = windows_core::s!("1.3.6.1.5.5.7.6.2");
pub const szOID_PKIX_OCSP: windows_core::PCSTR = windows_core::s!("1.3.6.1.5.5.7.48.1");
pub const szOID_PKIX_OCSP_BASIC_SIGNED_RESPONSE: windows_core::PCSTR = windows_core::s!("1.3.6.1.5.5.7.48.1.1");
pub const szOID_PKIX_OCSP_NOCHECK: windows_core::PCSTR = windows_core::s!("1.3.6.1.5.5.7.48.1.5");
pub const szOID_PKIX_OCSP_NONCE: windows_core::PCSTR = windows_core::s!("1.3.6.1.5.5.7.48.1.2");
pub const szOID_PKIX_PE: windows_core::PCSTR = windows_core::s!("1.3.6.1.5.5.7.1");
pub const szOID_PKIX_POLICY_QUALIFIER_CPS: windows_core::PCSTR = windows_core::s!("1.3.6.1.5.5.7.2.1");
pub const szOID_PKIX_POLICY_QUALIFIER_USERNOTICE: windows_core::PCSTR = windows_core::s!("1.3.6.1.5.5.7.2.2");
pub const szOID_PKIX_TIME_STAMPING: windows_core::PCSTR = windows_core::s!("1.3.6.1.5.5.7.48.3");
pub const szOID_PLATFORM_MANIFEST_BINARY_ID: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.10.3.28");
pub const szOID_POLICY_CONSTRAINTS: windows_core::PCSTR = windows_core::s!("2.5.29.36");
pub const szOID_POLICY_MAPPINGS: windows_core::PCSTR = windows_core::s!("2.5.29.33");
pub const szOID_POSTAL_ADDRESS: windows_core::PCSTR = windows_core::s!("2.5.4.16");
pub const szOID_POSTAL_CODE: windows_core::PCSTR = windows_core::s!("2.5.4.17");
pub const szOID_POST_OFFICE_BOX: windows_core::PCSTR = windows_core::s!("2.5.4.18");
pub const szOID_PREFERRED_DELIVERY_METHOD: windows_core::PCSTR = windows_core::s!("2.5.4.28");
pub const szOID_PRESENTATION_ADDRESS: windows_core::PCSTR = windows_core::s!("2.5.4.29");
pub const szOID_PRIVATEKEY_USAGE_PERIOD: windows_core::PCSTR = windows_core::s!("2.5.29.16");
pub const szOID_PRODUCT_UPDATE: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.31.1");
pub const szOID_PROTECTED_PROCESS_LIGHT_SIGNER: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.10.3.22");
pub const szOID_PROTECTED_PROCESS_SIGNER: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.10.3.24");
pub const szOID_QC_EU_COMPLIANCE: windows_core::PCSTR = windows_core::s!("0.4.0.1862.1.1");
pub const szOID_QC_SSCD: windows_core::PCSTR = windows_core::s!("0.4.0.1862.1.4");
pub const szOID_QC_STATEMENTS_EXT: windows_core::PCSTR = windows_core::s!("1.3.6.1.5.5.7.1.3");
pub const szOID_RDN_DUMMY_SIGNER: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.21.9");
pub const szOID_RDN_TCG_PLATFORM_MANUFACTURER: windows_core::PCSTR = windows_core::s!("2.23.133.2.4");
pub const szOID_RDN_TCG_PLATFORM_MODEL: windows_core::PCSTR = windows_core::s!("2.23.133.2.5");
pub const szOID_RDN_TCG_PLATFORM_VERSION: windows_core::PCSTR = windows_core::s!("2.23.133.2.6");
pub const szOID_RDN_TPM_MANUFACTURER: windows_core::PCSTR = windows_core::s!("2.23.133.2.1");
pub const szOID_RDN_TPM_MODEL: windows_core::PCSTR = windows_core::s!("2.23.133.2.2");
pub const szOID_RDN_TPM_VERSION: windows_core::PCSTR = windows_core::s!("2.23.133.2.3");
pub const szOID_REASON_CODE_HOLD: windows_core::PCSTR = windows_core::s!("2.5.29.23");
pub const szOID_REGISTERED_ADDRESS: windows_core::PCSTR = windows_core::s!("2.5.4.26");
pub const szOID_REMOVE_CERTIFICATE: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.10.8.1");
pub const szOID_RENEWAL_CERTIFICATE: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.13.1");
pub const szOID_REQUEST_CLIENT_INFO: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.21.20");
pub const szOID_REQUIRE_CERT_CHAIN_POLICY: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.21.15");
pub const szOID_REVOKED_LIST_SIGNER: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.10.3.19");
pub const szOID_RFC3161_counterSign: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.3.3.1");
pub const szOID_RFC3161v21_counterSign: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.3.3.2");
pub const szOID_RFC3161v21_thumbprints: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.3.3.3");
pub const szOID_ROLE_OCCUPANT: windows_core::PCSTR = windows_core::s!("2.5.4.33");
pub const szOID_ROOT_LIST_SIGNER: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.10.3.9");
pub const szOID_ROOT_PROGRAM_AUTO_UPDATE_CA_REVOCATION: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.60.3.1");
pub const szOID_ROOT_PROGRAM_AUTO_UPDATE_END_REVOCATION: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.60.3.2");
pub const szOID_ROOT_PROGRAM_FLAGS: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.60.1.1");
pub const szOID_ROOT_PROGRAM_NO_OCSP_FAILOVER_TO_CRL: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.60.3.3");
pub const szOID_RSA: windows_core::PCSTR = windows_core::s!("1.2.840.113549");
pub const szOID_RSAES_OAEP: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.1.7");
pub const szOID_RSA_DES_EDE3_CBC: windows_core::PCSTR = windows_core::s!("1.2.840.113549.3.7");
pub const szOID_RSA_DH: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.3.1");
pub const szOID_RSA_ENCRYPT: windows_core::PCSTR = windows_core::s!("1.2.840.113549.3");
pub const szOID_RSA_HASH: windows_core::PCSTR = windows_core::s!("1.2.840.113549.2");
pub const szOID_RSA_MD2: windows_core::PCSTR = windows_core::s!("1.2.840.113549.2.2");
pub const szOID_RSA_MD2RSA: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.1.2");
pub const szOID_RSA_MD4: windows_core::PCSTR = windows_core::s!("1.2.840.113549.2.4");
pub const szOID_RSA_MD4RSA: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.1.3");
pub const szOID_RSA_MD5: windows_core::PCSTR = windows_core::s!("1.2.840.113549.2.5");
pub const szOID_RSA_MD5RSA: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.1.4");
pub const szOID_RSA_MGF1: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.1.8");
pub const szOID_RSA_PSPECIFIED: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.1.9");
pub const szOID_RSA_RC2CBC: windows_core::PCSTR = windows_core::s!("1.2.840.113549.3.2");
pub const szOID_RSA_RC4: windows_core::PCSTR = windows_core::s!("1.2.840.113549.3.4");
pub const szOID_RSA_RC5_CBCPad: windows_core::PCSTR = windows_core::s!("1.2.840.113549.3.9");
pub const szOID_RSA_RSA: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.1.1");
pub const szOID_RSA_SETOAEP_RSA: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.1.6");
pub const szOID_RSA_SHA1RSA: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.1.5");
pub const szOID_RSA_SHA256RSA: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.1.11");
pub const szOID_RSA_SHA384RSA: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.1.12");
pub const szOID_RSA_SHA512RSA: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.1.13");
pub const szOID_RSA_SMIMECapabilities: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.9.15");
pub const szOID_RSA_SMIMEalg: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.9.16.3");
pub const szOID_RSA_SMIMEalgCMS3DESwrap: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.9.16.3.6");
pub const szOID_RSA_SMIMEalgCMSRC2wrap: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.9.16.3.7");
pub const szOID_RSA_SMIMEalgESDH: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.9.16.3.5");
pub const szOID_RSA_SSA_PSS: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.1.10");
pub const szOID_RSA_certExtensions: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.9.14");
pub const szOID_RSA_challengePwd: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.9.7");
pub const szOID_RSA_contentType: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.9.3");
pub const szOID_RSA_counterSign: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.9.6");
pub const szOID_RSA_data: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.7.1");
pub const szOID_RSA_digestedData: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.7.5");
pub const szOID_RSA_emailAddr: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.9.1");
pub const szOID_RSA_encryptedData: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.7.6");
pub const szOID_RSA_envelopedData: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.7.3");
pub const szOID_RSA_extCertAttrs: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.9.9");
pub const szOID_RSA_hashedData: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.7.5");
pub const szOID_RSA_messageDigest: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.9.4");
pub const szOID_RSA_preferSignedData: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.9.15.1");
pub const szOID_RSA_signEnvData: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.7.4");
pub const szOID_RSA_signedData: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.7.2");
pub const szOID_RSA_signingTime: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.9.5");
pub const szOID_RSA_unstructAddr: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.9.8");
pub const szOID_RSA_unstructName: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.9.2");
pub const szOID_SEARCH_GUIDE: windows_core::PCSTR = windows_core::s!("2.5.4.14");
pub const szOID_SEE_ALSO: windows_core::PCSTR = windows_core::s!("2.5.4.34");
pub const szOID_SERIALIZED: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.10.3.3.1");
pub const szOID_SERVER_GATED_CRYPTO: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.10.3.3");
pub const szOID_SGC_NETSCAPE: windows_core::PCSTR = windows_core::s!("2.16.840.1.113730.4.1");
pub const szOID_SITE_PIN_RULES_FLAGS_ATTR: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.10.4.3");
pub const szOID_SITE_PIN_RULES_INDEX_ATTR: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.10.4.2");
pub const szOID_SORTED_CTL: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.10.1.1");
pub const szOID_STATE_OR_PROVINCE_NAME: windows_core::PCSTR = windows_core::s!("2.5.4.8");
pub const szOID_STREET_ADDRESS: windows_core::PCSTR = windows_core::s!("2.5.4.9");
pub const szOID_SUBJECT_ALT_NAME: windows_core::PCSTR = windows_core::s!("2.5.29.7");
pub const szOID_SUBJECT_ALT_NAME2: windows_core::PCSTR = windows_core::s!("2.5.29.17");
pub const szOID_SUBJECT_DIR_ATTRS: windows_core::PCSTR = windows_core::s!("2.5.29.9");
pub const szOID_SUBJECT_INFO_ACCESS: windows_core::PCSTR = windows_core::s!("1.3.6.1.5.5.7.1.11");
pub const szOID_SUBJECT_KEY_IDENTIFIER: windows_core::PCSTR = windows_core::s!("2.5.29.14");
pub const szOID_SUPPORTED_APPLICATION_CONTEXT: windows_core::PCSTR = windows_core::s!("2.5.4.30");
pub const szOID_SUR_NAME: windows_core::PCSTR = windows_core::s!("2.5.4.4");
pub const szOID_SYNC_ROOT_CTL_EXT: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.10.3.50");
pub const szOID_TELEPHONE_NUMBER: windows_core::PCSTR = windows_core::s!("2.5.4.20");
pub const szOID_TELETEXT_TERMINAL_IDENTIFIER: windows_core::PCSTR = windows_core::s!("2.5.4.22");
pub const szOID_TELEX_NUMBER: windows_core::PCSTR = windows_core::s!("2.5.4.21");
pub const szOID_TIMESTAMP_TOKEN: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.9.16.1.4");
pub const szOID_TITLE: windows_core::PCSTR = windows_core::s!("2.5.4.12");
pub const szOID_TLS_FEATURES_EXT: windows_core::PCSTR = windows_core::s!("1.3.6.1.5.5.7.1.24");
pub const szOID_USER_CERTIFICATE: windows_core::PCSTR = windows_core::s!("2.5.4.36");
pub const szOID_USER_PASSWORD: windows_core::PCSTR = windows_core::s!("2.5.4.35");
pub const szOID_VERISIGN_BITSTRING_6_13: windows_core::PCSTR = windows_core::s!("2.16.840.1.113733.1.6.13");
pub const szOID_VERISIGN_ISS_STRONG_CRYPTO: windows_core::PCSTR = windows_core::s!("2.16.840.1.113733.1.8.1");
pub const szOID_VERISIGN_ONSITE_JURISDICTION_HASH: windows_core::PCSTR = windows_core::s!("2.16.840.1.113733.1.6.11");
pub const szOID_VERISIGN_PRIVATE_6_9: windows_core::PCSTR = windows_core::s!("2.16.840.1.113733.1.6.9");
pub const szOID_WHQL_CRYPTO: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.10.3.5");
pub const szOID_WINDOWS_KITS_SIGNER: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.10.3.20");
pub const szOID_WINDOWS_RT_SIGNER: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.10.3.21");
pub const szOID_WINDOWS_SOFTWARE_EXTENSION_SIGNER: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.10.3.26");
pub const szOID_WINDOWS_STORE_SIGNER: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.76.3.1");
pub const szOID_WINDOWS_TCB_SIGNER: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.10.3.23");
pub const szOID_WINDOWS_THIRD_PARTY_COMPONENT_SIGNER: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.10.3.25");
pub const szOID_X21_ADDRESS: windows_core::PCSTR = windows_core::s!("2.5.4.24");
pub const szOID_X957: windows_core::PCSTR = windows_core::s!("1.2.840.10040");
pub const szOID_X957_DSA: windows_core::PCSTR = windows_core::s!("1.2.840.10040.4.1");
pub const szOID_X957_SHA1DSA: windows_core::PCSTR = windows_core::s!("1.2.840.10040.4.3");
pub const szOID_YESNO_TRUST_ATTR: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.10.4.1");
pub const szPRIV_KEY_CACHE_MAX_ITEMS: windows_core::PCSTR = windows_core::s!("PrivKeyCacheMaxItems");
pub const szPRIV_KEY_CACHE_PURGE_INTERVAL_SECONDS: windows_core::PCSTR = windows_core::s!("PrivKeyCachePurgeIntervalSeconds");
pub const szROOT_STORE_FILE: windows_core::PCSTR = windows_core::s!("msroots");
pub const szUSER_KEYEXCHANGE_CERT_PREFIX: windows_core::PCSTR = windows_core::s!("kxc");
pub const szUSER_KEYEXCHANGE_PRIVATE_KEY_PREFIX: windows_core::PCSTR = windows_core::s!("kxs");
pub const szUSER_KEYEXCHANGE_PUBLIC_KEY_PREFIX: windows_core::PCSTR = windows_core::s!("kxp");
pub const szUSER_SIGNATURE_CERT_PREFIX: windows_core::PCSTR = windows_core::s!("ksc");
pub const szUSER_SIGNATURE_PRIVATE_KEY_PREFIX: windows_core::PCSTR = windows_core::s!("kss");
pub const szUSER_SIGNATURE_PUBLIC_KEY_PREFIX: windows_core::PCSTR = windows_core::s!("ksp");
pub const sz_CERT_STORE_PROV_COLLECTION: windows_core::PCSTR = windows_core::s!("Collection");
pub const sz_CERT_STORE_PROV_FILENAME: windows_core::PCWSTR = windows_core::w!("File");
pub const sz_CERT_STORE_PROV_FILENAME_W: windows_core::PCSTR = windows_core::s!("File");
pub const sz_CERT_STORE_PROV_LDAP: windows_core::PCWSTR = windows_core::w!("Ldap");
pub const sz_CERT_STORE_PROV_LDAP_W: windows_core::PCSTR = windows_core::s!("Ldap");
pub const sz_CERT_STORE_PROV_MEMORY: windows_core::PCSTR = windows_core::s!("Memory");
pub const sz_CERT_STORE_PROV_PHYSICAL: windows_core::PCWSTR = windows_core::w!("Physical");
pub const sz_CERT_STORE_PROV_PHYSICAL_W: windows_core::PCSTR = windows_core::s!("Physical");
pub const sz_CERT_STORE_PROV_PKCS12: windows_core::PCSTR = windows_core::s!("PKCS12");
pub const sz_CERT_STORE_PROV_PKCS7: windows_core::PCSTR = windows_core::s!("PKCS7");
pub const sz_CERT_STORE_PROV_SERIALIZED: windows_core::PCSTR = windows_core::s!("Serialized");
pub const sz_CERT_STORE_PROV_SMART_CARD: windows_core::PCWSTR = windows_core::w!("SmartCard");
pub const sz_CERT_STORE_PROV_SMART_CARD_W: windows_core::PCSTR = windows_core::s!("SmartCard");
pub const sz_CERT_STORE_PROV_SYSTEM: windows_core::PCWSTR = windows_core::w!("System");
pub const sz_CERT_STORE_PROV_SYSTEM_REGISTRY: windows_core::PCWSTR = windows_core::w!("SystemRegistry");
pub const sz_CERT_STORE_PROV_SYSTEM_REGISTRY_W: windows_core::PCSTR = windows_core::s!("SystemRegistry");
pub const sz_CERT_STORE_PROV_SYSTEM_W: windows_core::PCSTR = windows_core::s!("System");
pub const wszCARD_USER_ADMIN: windows_core::PCWSTR = windows_core::w!("admin");
pub const wszCARD_USER_EVERYONE: windows_core::PCWSTR = windows_core::w!("anonymous");
pub const wszCARD_USER_USER: windows_core::PCWSTR = windows_core::w!("user");
pub const wszURI_CANONICALIZATION_C14N: windows_core::PCWSTR = windows_core::w!("http://www.w3.org/TR/2001/REC-xml-c14n-20010315");
pub const wszURI_CANONICALIZATION_C14NC: windows_core::PCWSTR = windows_core::w!("http://www.w3.org/TR/2001/REC-xml-c14n-20010315#WithComments");
pub const wszURI_CANONICALIZATION_EXSLUSIVE_C14N: windows_core::PCWSTR = windows_core::w!("http://www.w3.org/2001/10/xml-exc-c14n#");
pub const wszURI_CANONICALIZATION_EXSLUSIVE_C14NC: windows_core::PCWSTR = windows_core::w!("http://www.w3.org/2001/10/xml-exc-c14n#WithComments");
pub const wszURI_NTDS_OBJECTSID_PREFIX: windows_core::PCWSTR = windows_core::w!("tag:microsoft.com,2022-09-14:sid:");
pub const wszURI_TRANSFORM_XPATH: windows_core::PCWSTR = windows_core::w!("http://www.w3.org/TR/1999/REC-xpath-19991116");
pub const wszURI_XMLNS_DIGSIG_BASE64: windows_core::PCWSTR = windows_core::w!("http://www.w3.org/2000/09/xmldsig#base64");
pub const wszURI_XMLNS_DIGSIG_DSA_SHA1: windows_core::PCWSTR = windows_core::w!("http://www.w3.org/2000/09/xmldsig#dsa-sha1");
pub const wszURI_XMLNS_DIGSIG_ECDSA_SHA1: windows_core::PCWSTR = windows_core::w!("http://www.w3.org/2001/04/xmldsig-more#ecdsa-sha1");
pub const wszURI_XMLNS_DIGSIG_ECDSA_SHA256: windows_core::PCWSTR = windows_core::w!("http://www.w3.org/2001/04/xmldsig-more#ecdsa-sha256");
pub const wszURI_XMLNS_DIGSIG_ECDSA_SHA384: windows_core::PCWSTR = windows_core::w!("http://www.w3.org/2001/04/xmldsig-more#ecdsa-sha384");
pub const wszURI_XMLNS_DIGSIG_ECDSA_SHA512: windows_core::PCWSTR = windows_core::w!("http://www.w3.org/2001/04/xmldsig-more#ecdsa-sha512");
pub const wszURI_XMLNS_DIGSIG_HMAC_SHA1: windows_core::PCWSTR = windows_core::w!("http://www.w3.org/2000/09/xmldsig#hmac-sha1");
pub const wszURI_XMLNS_DIGSIG_HMAC_SHA256: windows_core::PCWSTR = windows_core::w!("http://www.w3.org/2001/04/xmldsig-more#hmac-sha256");
pub const wszURI_XMLNS_DIGSIG_HMAC_SHA384: windows_core::PCWSTR = windows_core::w!("http://www.w3.org/2001/04/xmldsig-more#hmac-sha384");
pub const wszURI_XMLNS_DIGSIG_HMAC_SHA512: windows_core::PCWSTR = windows_core::w!("http://www.w3.org/2001/04/xmldsig-more#hmac-sha512");
pub const wszURI_XMLNS_DIGSIG_RSA_SHA1: windows_core::PCWSTR = windows_core::w!("http://www.w3.org/2000/09/xmldsig#rsa-sha1");
pub const wszURI_XMLNS_DIGSIG_RSA_SHA256: windows_core::PCWSTR = windows_core::w!("http://www.w3.org/2001/04/xmldsig-more#rsa-sha256");
pub const wszURI_XMLNS_DIGSIG_RSA_SHA384: windows_core::PCWSTR = windows_core::w!("http://www.w3.org/2001/04/xmldsig-more#rsa-sha384");
pub const wszURI_XMLNS_DIGSIG_RSA_SHA512: windows_core::PCWSTR = windows_core::w!("http://www.w3.org/2001/04/xmldsig-more#rsa-sha512");
pub const wszURI_XMLNS_DIGSIG_SHA1: windows_core::PCWSTR = windows_core::w!("http://www.w3.org/2000/09/xmldsig#sha1");
pub const wszURI_XMLNS_DIGSIG_SHA256: windows_core::PCWSTR = windows_core::w!("http://www.w3.org/2001/04/xmlenc#sha256");
pub const wszURI_XMLNS_DIGSIG_SHA384: windows_core::PCWSTR = windows_core::w!("http://www.w3.org/2001/04/xmldsig-more#sha384");
pub const wszURI_XMLNS_DIGSIG_SHA512: windows_core::PCWSTR = windows_core::w!("http://www.w3.org/2001/04/xmlenc#sha512");
pub const wszURI_XMLNS_TRANSFORM_BASE64: windows_core::PCWSTR = windows_core::w!("http://www.w3.org/2000/09/xmldsig#base64");
pub const wszURI_XMLNS_TRANSFORM_ENVELOPED: windows_core::PCWSTR = windows_core::w!("http://www.w3.org/2000/09/xmldsig#enveloped-signature");
pub const wszXMLNS_DIGSIG: windows_core::PCWSTR = windows_core::w!("http://www.w3.org/2000/09/xmldsig#");
pub const wszXMLNS_DIGSIG_Id: windows_core::PCWSTR = windows_core::w!("Id");
pub const wszXMLNS_DIGSIG_SignatureProperties: windows_core::PCWSTR = windows_core::w!("http://www.w3.org/2000/09/xmldsig#SignatureProperties");
