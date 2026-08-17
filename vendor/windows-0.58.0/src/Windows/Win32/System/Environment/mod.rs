#[inline]
pub unsafe fn CallEnclave<P0>(lproutine: isize, lpparameter: *const core::ffi::c_void, fwaitforthread: P0, lpreturnvalue: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("vertdll.dll" "system" fn CallEnclave(lproutine : isize, lpparameter : *const core::ffi::c_void, fwaitforthread : super::super::Foundation:: BOOL, lpreturnvalue : *mut *mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
    CallEnclave(lproutine, lpparameter, fwaitforthread.param().abi(), lpreturnvalue).ok()
}
#[inline]
pub unsafe fn CreateEnclave<P0>(hprocess: P0, lpaddress: Option<*const core::ffi::c_void>, dwsize: usize, dwinitialcommitment: usize, flenclavetype: u32, lpenclaveinformation: *const core::ffi::c_void, dwinfolength: u32, lpenclaveerror: Option<*mut u32>) -> *mut core::ffi::c_void
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn CreateEnclave(hprocess : super::super::Foundation:: HANDLE, lpaddress : *const core::ffi::c_void, dwsize : usize, dwinitialcommitment : usize, flenclavetype : u32, lpenclaveinformation : *const core::ffi::c_void, dwinfolength : u32, lpenclaveerror : *mut u32) -> *mut core::ffi::c_void);
    CreateEnclave(hprocess.param().abi(), core::mem::transmute(lpaddress.unwrap_or(std::ptr::null())), dwsize, dwinitialcommitment, flenclavetype, lpenclaveinformation, dwinfolength, core::mem::transmute(lpenclaveerror.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn CreateEnvironmentBlock<P0, P1>(lpenvironment: *mut *mut core::ffi::c_void, htoken: P0, binherit: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("userenv.dll" "system" fn CreateEnvironmentBlock(lpenvironment : *mut *mut core::ffi::c_void, htoken : super::super::Foundation:: HANDLE, binherit : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    CreateEnvironmentBlock(lpenvironment, htoken.param().abi(), binherit.param().abi()).ok()
}
#[inline]
pub unsafe fn DeleteEnclave(lpaddress: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("api-ms-win-core-enclave-l1-1-1.dll" "system" fn DeleteEnclave(lpaddress : *const core::ffi::c_void) -> super::super::Foundation:: BOOL);
    DeleteEnclave(lpaddress).ok()
}
#[inline]
pub unsafe fn DestroyEnvironmentBlock(lpenvironment: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("userenv.dll" "system" fn DestroyEnvironmentBlock(lpenvironment : *const core::ffi::c_void) -> super::super::Foundation:: BOOL);
    DestroyEnvironmentBlock(lpenvironment).ok()
}
#[inline]
pub unsafe fn EnclaveGetAttestationReport(enclavedata: Option<&[u8; 64]>, report: Option<*mut core::ffi::c_void>, buffersize: u32, outputsize: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("vertdll.dll" "system" fn EnclaveGetAttestationReport(enclavedata : *const u8, report : *mut core::ffi::c_void, buffersize : u32, outputsize : *mut u32) -> windows_core::HRESULT);
    EnclaveGetAttestationReport(core::mem::transmute(enclavedata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), core::mem::transmute(report.unwrap_or(std::ptr::null_mut())), buffersize, outputsize).ok()
}
#[inline]
pub unsafe fn EnclaveGetEnclaveInformation(informationsize: u32, enclaveinformation: *mut ENCLAVE_INFORMATION) -> windows_core::Result<()> {
    windows_targets::link!("vertdll.dll" "system" fn EnclaveGetEnclaveInformation(informationsize : u32, enclaveinformation : *mut ENCLAVE_INFORMATION) -> windows_core::HRESULT);
    EnclaveGetEnclaveInformation(informationsize, enclaveinformation).ok()
}
#[inline]
pub unsafe fn EnclaveSealData(datatoencrypt: *const core::ffi::c_void, datatoencryptsize: u32, identitypolicy: ENCLAVE_SEALING_IDENTITY_POLICY, runtimepolicy: u32, protectedblob: Option<*mut core::ffi::c_void>, buffersize: u32, protectedblobsize: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("vertdll.dll" "system" fn EnclaveSealData(datatoencrypt : *const core::ffi::c_void, datatoencryptsize : u32, identitypolicy : ENCLAVE_SEALING_IDENTITY_POLICY, runtimepolicy : u32, protectedblob : *mut core::ffi::c_void, buffersize : u32, protectedblobsize : *mut u32) -> windows_core::HRESULT);
    EnclaveSealData(datatoencrypt, datatoencryptsize, identitypolicy, runtimepolicy, core::mem::transmute(protectedblob.unwrap_or(std::ptr::null_mut())), buffersize, protectedblobsize).ok()
}
#[inline]
pub unsafe fn EnclaveUnsealData(protectedblob: *const core::ffi::c_void, protectedblobsize: u32, decrypteddata: Option<*mut core::ffi::c_void>, buffersize: u32, decrypteddatasize: *mut u32, sealingidentity: Option<*mut ENCLAVE_IDENTITY>, unsealingflags: Option<*mut u32>) -> windows_core::Result<()> {
    windows_targets::link!("vertdll.dll" "system" fn EnclaveUnsealData(protectedblob : *const core::ffi::c_void, protectedblobsize : u32, decrypteddata : *mut core::ffi::c_void, buffersize : u32, decrypteddatasize : *mut u32, sealingidentity : *mut ENCLAVE_IDENTITY, unsealingflags : *mut u32) -> windows_core::HRESULT);
    EnclaveUnsealData(protectedblob, protectedblobsize, core::mem::transmute(decrypteddata.unwrap_or(std::ptr::null_mut())), buffersize, decrypteddatasize, core::mem::transmute(sealingidentity.unwrap_or(std::ptr::null_mut())), core::mem::transmute(unsealingflags.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn EnclaveVerifyAttestationReport(enclavetype: u32, report: *const core::ffi::c_void, reportsize: u32) -> windows_core::Result<()> {
    windows_targets::link!("vertdll.dll" "system" fn EnclaveVerifyAttestationReport(enclavetype : u32, report : *const core::ffi::c_void, reportsize : u32) -> windows_core::HRESULT);
    EnclaveVerifyAttestationReport(enclavetype, report, reportsize).ok()
}
#[inline]
pub unsafe fn ExpandEnvironmentStringsA<P0>(lpsrc: P0, lpdst: Option<&mut [u8]>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn ExpandEnvironmentStringsA(lpsrc : windows_core::PCSTR, lpdst : windows_core::PSTR, nsize : u32) -> u32);
    ExpandEnvironmentStringsA(lpsrc.param().abi(), core::mem::transmute(lpdst.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpdst.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn ExpandEnvironmentStringsForUserA<P0, P1>(htoken: P0, lpsrc: P1, lpdest: &mut [u8]) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("userenv.dll" "system" fn ExpandEnvironmentStringsForUserA(htoken : super::super::Foundation:: HANDLE, lpsrc : windows_core::PCSTR, lpdest : windows_core::PSTR, dwsize : u32) -> super::super::Foundation:: BOOL);
    ExpandEnvironmentStringsForUserA(htoken.param().abi(), lpsrc.param().abi(), core::mem::transmute(lpdest.as_ptr()), lpdest.len().try_into().unwrap()).ok()
}
#[inline]
pub unsafe fn ExpandEnvironmentStringsForUserW<P0, P1>(htoken: P0, lpsrc: P1, lpdest: &mut [u16]) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("userenv.dll" "system" fn ExpandEnvironmentStringsForUserW(htoken : super::super::Foundation:: HANDLE, lpsrc : windows_core::PCWSTR, lpdest : windows_core::PWSTR, dwsize : u32) -> super::super::Foundation:: BOOL);
    ExpandEnvironmentStringsForUserW(htoken.param().abi(), lpsrc.param().abi(), core::mem::transmute(lpdest.as_ptr()), lpdest.len().try_into().unwrap()).ok()
}
#[inline]
pub unsafe fn ExpandEnvironmentStringsW<P0>(lpsrc: P0, lpdst: Option<&mut [u16]>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn ExpandEnvironmentStringsW(lpsrc : windows_core::PCWSTR, lpdst : windows_core::PWSTR, nsize : u32) -> u32);
    ExpandEnvironmentStringsW(lpsrc.param().abi(), core::mem::transmute(lpdst.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpdst.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn FreeEnvironmentStringsA<P0>(penv: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn FreeEnvironmentStringsA(penv : windows_core::PCSTR) -> super::super::Foundation:: BOOL);
    FreeEnvironmentStringsA(penv.param().abi()).ok()
}
#[inline]
pub unsafe fn FreeEnvironmentStringsW<P0>(penv: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn FreeEnvironmentStringsW(penv : windows_core::PCWSTR) -> super::super::Foundation:: BOOL);
    FreeEnvironmentStringsW(penv.param().abi()).ok()
}
#[inline]
pub unsafe fn GetCommandLineA() -> windows_core::PCSTR {
    windows_targets::link!("kernel32.dll" "system" fn GetCommandLineA() -> windows_core::PCSTR);
    GetCommandLineA()
}
#[inline]
pub unsafe fn GetCommandLineW() -> windows_core::PCWSTR {
    windows_targets::link!("kernel32.dll" "system" fn GetCommandLineW() -> windows_core::PCWSTR);
    GetCommandLineW()
}
#[inline]
pub unsafe fn GetCurrentDirectoryA(lpbuffer: Option<&mut [u8]>) -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn GetCurrentDirectoryA(nbufferlength : u32, lpbuffer : windows_core::PSTR) -> u32);
    GetCurrentDirectoryA(lpbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(lpbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
}
#[inline]
pub unsafe fn GetCurrentDirectoryW(lpbuffer: Option<&mut [u16]>) -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn GetCurrentDirectoryW(nbufferlength : u32, lpbuffer : windows_core::PWSTR) -> u32);
    GetCurrentDirectoryW(lpbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(lpbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
}
#[inline]
pub unsafe fn GetEnvironmentStrings() -> windows_core::PSTR {
    windows_targets::link!("kernel32.dll" "system" fn GetEnvironmentStrings() -> windows_core::PSTR);
    GetEnvironmentStrings()
}
#[inline]
pub unsafe fn GetEnvironmentStringsW() -> windows_core::PWSTR {
    windows_targets::link!("kernel32.dll" "system" fn GetEnvironmentStringsW() -> windows_core::PWSTR);
    GetEnvironmentStringsW()
}
#[inline]
pub unsafe fn GetEnvironmentVariableA<P0>(lpname: P0, lpbuffer: Option<&mut [u8]>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetEnvironmentVariableA(lpname : windows_core::PCSTR, lpbuffer : windows_core::PSTR, nsize : u32) -> u32);
    GetEnvironmentVariableA(lpname.param().abi(), core::mem::transmute(lpbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn GetEnvironmentVariableW<P0>(lpname: P0, lpbuffer: Option<&mut [u16]>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetEnvironmentVariableW(lpname : windows_core::PCWSTR, lpbuffer : windows_core::PWSTR, nsize : u32) -> u32);
    GetEnvironmentVariableW(lpname.param().abi(), core::mem::transmute(lpbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn InitializeEnclave<P0>(hprocess: P0, lpaddress: *const core::ffi::c_void, lpenclaveinformation: *const core::ffi::c_void, dwinfolength: u32, lpenclaveerror: Option<*mut u32>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn InitializeEnclave(hprocess : super::super::Foundation:: HANDLE, lpaddress : *const core::ffi::c_void, lpenclaveinformation : *const core::ffi::c_void, dwinfolength : u32, lpenclaveerror : *mut u32) -> super::super::Foundation:: BOOL);
    InitializeEnclave(hprocess.param().abi(), lpaddress, lpenclaveinformation, dwinfolength, core::mem::transmute(lpenclaveerror.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn IsEnclaveTypeSupported(flenclavetype: u32) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn IsEnclaveTypeSupported(flenclavetype : u32) -> super::super::Foundation:: BOOL);
    IsEnclaveTypeSupported(flenclavetype).ok()
}
#[inline]
pub unsafe fn LoadEnclaveData<P0>(hprocess: P0, lpaddress: *const core::ffi::c_void, lpbuffer: *const core::ffi::c_void, nsize: usize, flprotect: u32, lppageinformation: *const core::ffi::c_void, dwinfolength: u32, lpnumberofbyteswritten: *mut usize, lpenclaveerror: Option<*mut u32>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn LoadEnclaveData(hprocess : super::super::Foundation:: HANDLE, lpaddress : *const core::ffi::c_void, lpbuffer : *const core::ffi::c_void, nsize : usize, flprotect : u32, lppageinformation : *const core::ffi::c_void, dwinfolength : u32, lpnumberofbyteswritten : *mut usize, lpenclaveerror : *mut u32) -> super::super::Foundation:: BOOL);
    LoadEnclaveData(hprocess.param().abi(), lpaddress, lpbuffer, nsize, flprotect, lppageinformation, dwinfolength, lpnumberofbyteswritten, core::mem::transmute(lpenclaveerror.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn LoadEnclaveImageA<P0>(lpenclaveaddress: *const core::ffi::c_void, lpimagename: P0) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("api-ms-win-core-enclave-l1-1-1.dll" "system" fn LoadEnclaveImageA(lpenclaveaddress : *const core::ffi::c_void, lpimagename : windows_core::PCSTR) -> super::super::Foundation:: BOOL);
    LoadEnclaveImageA(lpenclaveaddress, lpimagename.param().abi())
}
#[inline]
pub unsafe fn LoadEnclaveImageW<P0>(lpenclaveaddress: *const core::ffi::c_void, lpimagename: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("api-ms-win-core-enclave-l1-1-1.dll" "system" fn LoadEnclaveImageW(lpenclaveaddress : *const core::ffi::c_void, lpimagename : windows_core::PCWSTR) -> super::super::Foundation:: BOOL);
    LoadEnclaveImageW(lpenclaveaddress, lpimagename.param().abi()).ok()
}
#[inline]
pub unsafe fn NeedCurrentDirectoryForExePathA<P0>(exename: P0) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn NeedCurrentDirectoryForExePathA(exename : windows_core::PCSTR) -> super::super::Foundation:: BOOL);
    NeedCurrentDirectoryForExePathA(exename.param().abi())
}
#[inline]
pub unsafe fn NeedCurrentDirectoryForExePathW<P0>(exename: P0) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn NeedCurrentDirectoryForExePathW(exename : windows_core::PCWSTR) -> super::super::Foundation:: BOOL);
    NeedCurrentDirectoryForExePathW(exename.param().abi())
}
#[inline]
pub unsafe fn SetCurrentDirectoryA<P0>(lppathname: P0) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetCurrentDirectoryA(lppathname : windows_core::PCSTR) -> super::super::Foundation:: BOOL);
    SetCurrentDirectoryA(lppathname.param().abi())
}
#[inline]
pub unsafe fn SetCurrentDirectoryW<P0>(lppathname: P0) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetCurrentDirectoryW(lppathname : windows_core::PCWSTR) -> super::super::Foundation:: BOOL);
    SetCurrentDirectoryW(lppathname.param().abi())
}
#[inline]
pub unsafe fn SetEnvironmentStringsW<P0>(newenvironment: P0) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetEnvironmentStringsW(newenvironment : windows_core::PCWSTR) -> super::super::Foundation:: BOOL);
    SetEnvironmentStringsW(newenvironment.param().abi())
}
#[inline]
pub unsafe fn SetEnvironmentVariableA<P0, P1>(lpname: P0, lpvalue: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetEnvironmentVariableA(lpname : windows_core::PCSTR, lpvalue : windows_core::PCSTR) -> super::super::Foundation:: BOOL);
    SetEnvironmentVariableA(lpname.param().abi(), lpvalue.param().abi()).ok()
}
#[inline]
pub unsafe fn SetEnvironmentVariableW<P0, P1>(lpname: P0, lpvalue: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetEnvironmentVariableW(lpname : windows_core::PCWSTR, lpvalue : windows_core::PCWSTR) -> super::super::Foundation:: BOOL);
    SetEnvironmentVariableW(lpname.param().abi(), lpvalue.param().abi()).ok()
}
#[inline]
pub unsafe fn TerminateEnclave<P0>(lpaddress: *const core::ffi::c_void, fwait: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("vertdll.dll" "system" fn TerminateEnclave(lpaddress : *const core::ffi::c_void, fwait : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    TerminateEnclave(lpaddress, fwait.param().abi()).ok()
}
pub const ENCLAVE_FLAG_DYNAMIC_DEBUG_ACTIVE: u32 = 4u32;
pub const ENCLAVE_FLAG_DYNAMIC_DEBUG_ENABLED: u32 = 2u32;
pub const ENCLAVE_FLAG_FULL_DEBUG_ENABLED: u32 = 1u32;
pub const ENCLAVE_IDENTITY_POLICY_SEAL_EXACT_CODE: ENCLAVE_SEALING_IDENTITY_POLICY = ENCLAVE_SEALING_IDENTITY_POLICY(1i32);
pub const ENCLAVE_IDENTITY_POLICY_SEAL_INVALID: ENCLAVE_SEALING_IDENTITY_POLICY = ENCLAVE_SEALING_IDENTITY_POLICY(0i32);
pub const ENCLAVE_IDENTITY_POLICY_SEAL_SAME_AUTHOR: ENCLAVE_SEALING_IDENTITY_POLICY = ENCLAVE_SEALING_IDENTITY_POLICY(5i32);
pub const ENCLAVE_IDENTITY_POLICY_SEAL_SAME_FAMILY: ENCLAVE_SEALING_IDENTITY_POLICY = ENCLAVE_SEALING_IDENTITY_POLICY(4i32);
pub const ENCLAVE_IDENTITY_POLICY_SEAL_SAME_IMAGE: ENCLAVE_SEALING_IDENTITY_POLICY = ENCLAVE_SEALING_IDENTITY_POLICY(3i32);
pub const ENCLAVE_IDENTITY_POLICY_SEAL_SAME_PRIMARY_CODE: ENCLAVE_SEALING_IDENTITY_POLICY = ENCLAVE_SEALING_IDENTITY_POLICY(2i32);
pub const ENCLAVE_REPORT_DATA_LENGTH: u32 = 64u32;
pub const ENCLAVE_RUNTIME_POLICY_ALLOW_DYNAMIC_DEBUG: u32 = 2u32;
pub const ENCLAVE_RUNTIME_POLICY_ALLOW_FULL_DEBUG: u32 = 1u32;
pub const ENCLAVE_UNSEAL_FLAG_STALE_KEY: u32 = 1u32;
pub const ENCLAVE_VBS_BASIC_KEY_FLAG_DEBUG_KEY: u32 = 8u32;
pub const ENCLAVE_VBS_BASIC_KEY_FLAG_FAMILY_ID: u32 = 2u32;
pub const ENCLAVE_VBS_BASIC_KEY_FLAG_IMAGE_ID: u32 = 4u32;
pub const ENCLAVE_VBS_BASIC_KEY_FLAG_MEASUREMENT: u32 = 1u32;
pub const VBS_ENCLAVE_REPORT_PKG_HEADER_VERSION_CURRENT: u32 = 1u32;
pub const VBS_ENCLAVE_REPORT_SIGNATURE_SCHEME_SHA256_RSA_PSS_SHA256: u32 = 1u32;
pub const VBS_ENCLAVE_REPORT_VERSION_CURRENT: u32 = 1u32;
pub const VBS_ENCLAVE_VARDATA_INVALID: u32 = 0u32;
pub const VBS_ENCLAVE_VARDATA_MODULE: u32 = 1u32;
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ENCLAVE_SEALING_IDENTITY_POLICY(pub i32);
impl windows_core::TypeKind for ENCLAVE_SEALING_IDENTITY_POLICY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ENCLAVE_SEALING_IDENTITY_POLICY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ENCLAVE_SEALING_IDENTITY_POLICY").field(&self.0).finish()
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct ENCLAVE_IDENTITY {
    pub OwnerId: [u8; 32],
    pub UniqueId: [u8; 32],
    pub AuthorId: [u8; 32],
    pub FamilyId: [u8; 16],
    pub ImageId: [u8; 16],
    pub EnclaveSvn: u32,
    pub SecureKernelSvn: u32,
    pub PlatformSvn: u32,
    pub Flags: u32,
    pub SigningLevel: u32,
    pub EnclaveType: u32,
}
impl windows_core::TypeKind for ENCLAVE_IDENTITY {
    type TypeKind = windows_core::CopyType;
}
impl Default for ENCLAVE_IDENTITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ENCLAVE_INFORMATION {
    pub EnclaveType: u32,
    pub Reserved: u32,
    pub BaseAddress: *mut core::ffi::c_void,
    pub Size: usize,
    pub Identity: ENCLAVE_IDENTITY,
}
impl windows_core::TypeKind for ENCLAVE_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for ENCLAVE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ENCLAVE_VBS_BASIC_KEY_REQUEST {
    pub RequestSize: u32,
    pub Flags: u32,
    pub EnclaveSVN: u32,
    pub SystemKeyID: u32,
    pub CurrentSystemKeyID: u32,
}
impl windows_core::TypeKind for ENCLAVE_VBS_BASIC_KEY_REQUEST {
    type TypeKind = windows_core::CopyType;
}
impl Default for ENCLAVE_VBS_BASIC_KEY_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VBS_BASIC_ENCLAVE_EXCEPTION_AMD64 {
    pub ExceptionCode: u32,
    pub NumberParameters: u32,
    pub ExceptionInformation: [usize; 3],
    pub ExceptionRAX: usize,
    pub ExceptionRCX: usize,
    pub ExceptionRIP: usize,
    pub ExceptionRFLAGS: usize,
    pub ExceptionRSP: usize,
}
impl windows_core::TypeKind for VBS_BASIC_ENCLAVE_EXCEPTION_AMD64 {
    type TypeKind = windows_core::CopyType;
}
impl Default for VBS_BASIC_ENCLAVE_EXCEPTION_AMD64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct VBS_BASIC_ENCLAVE_SYSCALL_PAGE {
    pub ReturnFromEnclave: VBS_BASIC_ENCLAVE_BASIC_CALL_RETURN_FROM_ENCLAVE,
    pub ReturnFromException: VBS_BASIC_ENCLAVE_BASIC_CALL_RETURN_FROM_EXCEPTION,
    pub TerminateThread: VBS_BASIC_ENCLAVE_BASIC_CALL_TERMINATE_THREAD,
    pub InterruptThread: VBS_BASIC_ENCLAVE_BASIC_CALL_INTERRUPT_THREAD,
    pub CommitPages: VBS_BASIC_ENCLAVE_BASIC_CALL_COMMIT_PAGES,
    pub DecommitPages: VBS_BASIC_ENCLAVE_BASIC_CALL_DECOMMIT_PAGES,
    pub ProtectPages: VBS_BASIC_ENCLAVE_BASIC_CALL_PROTECT_PAGES,
    pub CreateThread: VBS_BASIC_ENCLAVE_BASIC_CALL_CREATE_THREAD,
    pub GetEnclaveInformation: VBS_BASIC_ENCLAVE_BASIC_CALL_GET_ENCLAVE_INFORMATION,
    pub GenerateKey: VBS_BASIC_ENCLAVE_BASIC_CALL_GENERATE_KEY,
    pub GenerateReport: VBS_BASIC_ENCLAVE_BASIC_CALL_GENERATE_REPORT,
    pub VerifyReport: VBS_BASIC_ENCLAVE_BASIC_CALL_VERIFY_REPORT,
    pub GenerateRandomData: VBS_BASIC_ENCLAVE_BASIC_CALL_GENERATE_RANDOM_DATA,
}
impl windows_core::TypeKind for VBS_BASIC_ENCLAVE_SYSCALL_PAGE {
    type TypeKind = windows_core::CopyType;
}
impl Default for VBS_BASIC_ENCLAVE_SYSCALL_PAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR32 {
    pub ThreadContext: [u32; 4],
    pub EntryPoint: u32,
    pub StackPointer: u32,
    pub ExceptionEntryPoint: u32,
    pub ExceptionStack: u32,
    pub ExceptionActive: u32,
}
impl windows_core::TypeKind for VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR32 {
    type TypeKind = windows_core::CopyType;
}
impl Default for VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR64 {
    pub ThreadContext: [u64; 4],
    pub EntryPoint: u64,
    pub StackPointer: u64,
    pub ExceptionEntryPoint: u64,
    pub ExceptionStack: u64,
    pub ExceptionActive: u32,
}
impl windows_core::TypeKind for VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR64 {
    type TypeKind = windows_core::CopyType;
}
impl Default for VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct VBS_ENCLAVE_REPORT {
    pub ReportSize: u32,
    pub ReportVersion: u32,
    pub EnclaveData: [u8; 64],
    pub EnclaveIdentity: ENCLAVE_IDENTITY,
}
impl windows_core::TypeKind for VBS_ENCLAVE_REPORT {
    type TypeKind = windows_core::CopyType;
}
impl Default for VBS_ENCLAVE_REPORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct VBS_ENCLAVE_REPORT_MODULE {
    pub Header: VBS_ENCLAVE_REPORT_VARDATA_HEADER,
    pub UniqueId: [u8; 32],
    pub AuthorId: [u8; 32],
    pub FamilyId: [u8; 16],
    pub ImageId: [u8; 16],
    pub Svn: u32,
    pub ModuleName: [u16; 1],
}
impl windows_core::TypeKind for VBS_ENCLAVE_REPORT_MODULE {
    type TypeKind = windows_core::CopyType;
}
impl Default for VBS_ENCLAVE_REPORT_MODULE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct VBS_ENCLAVE_REPORT_PKG_HEADER {
    pub PackageSize: u32,
    pub Version: u32,
    pub SignatureScheme: u32,
    pub SignedStatementSize: u32,
    pub SignatureSize: u32,
    pub Reserved: u32,
}
impl windows_core::TypeKind for VBS_ENCLAVE_REPORT_PKG_HEADER {
    type TypeKind = windows_core::CopyType;
}
impl Default for VBS_ENCLAVE_REPORT_PKG_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct VBS_ENCLAVE_REPORT_VARDATA_HEADER {
    pub DataType: u32,
    pub Size: u32,
}
impl windows_core::TypeKind for VBS_ENCLAVE_REPORT_VARDATA_HEADER {
    type TypeKind = windows_core::CopyType;
}
impl Default for VBS_ENCLAVE_REPORT_VARDATA_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_COMMIT_PAGES = Option<unsafe extern "system" fn(enclaveaddress: *const core::ffi::c_void, numberofbytes: usize, sourceaddress: *const core::ffi::c_void, pageprotection: u32) -> i32>;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_CREATE_THREAD = Option<unsafe extern "system" fn(threaddescriptor: *const VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR64) -> i32>;
#[cfg(target_arch = "x86")]
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_CREATE_THREAD = Option<unsafe extern "system" fn(threaddescriptor: *const VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR32) -> i32>;
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_DECOMMIT_PAGES = Option<unsafe extern "system" fn(enclaveaddress: *const core::ffi::c_void, numberofbytes: usize) -> i32>;
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_GENERATE_KEY = Option<unsafe extern "system" fn(keyrequest: *mut ENCLAVE_VBS_BASIC_KEY_REQUEST, requestedkeysize: u32, returnedkey: *mut u8) -> i32>;
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_GENERATE_RANDOM_DATA = Option<unsafe extern "system" fn(buffer: *mut u8, numberofbytes: u32, generation: *mut u64) -> i32>;
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_GENERATE_REPORT = Option<unsafe extern "system" fn(enclavedata: *const u8, report: *mut core::ffi::c_void, buffersize: u32, outputsize: *mut u32) -> i32>;
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_GET_ENCLAVE_INFORMATION = Option<unsafe extern "system" fn(enclaveinfo: *mut ENCLAVE_INFORMATION) -> i32>;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_INTERRUPT_THREAD = Option<unsafe extern "system" fn(threaddescriptor: *const VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR64) -> i32>;
#[cfg(target_arch = "x86")]
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_INTERRUPT_THREAD = Option<unsafe extern "system" fn(threaddescriptor: *const VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR32) -> i32>;
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_PROTECT_PAGES = Option<unsafe extern "system" fn(enclaveaddress: *const core::ffi::c_void, numberofytes: usize, pageprotection: u32) -> i32>;
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_RETURN_FROM_ENCLAVE = Option<unsafe extern "system" fn(returnvalue: usize)>;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_RETURN_FROM_EXCEPTION = Option<unsafe extern "system" fn(exceptionrecord: *const VBS_BASIC_ENCLAVE_EXCEPTION_AMD64) -> i32>;
#[cfg(any(target_arch = "aarch64", target_arch = "x86"))]
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_RETURN_FROM_EXCEPTION = Option<unsafe extern "system" fn(exceptionrecord: *const core::ffi::c_void) -> i32>;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_TERMINATE_THREAD = Option<unsafe extern "system" fn(threaddescriptor: *const VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR64) -> i32>;
#[cfg(target_arch = "x86")]
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_TERMINATE_THREAD = Option<unsafe extern "system" fn(threaddescriptor: *const VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR32) -> i32>;
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_VERIFY_REPORT = Option<unsafe extern "system" fn(report: *const core::ffi::c_void, reportsize: u32) -> i32>;
