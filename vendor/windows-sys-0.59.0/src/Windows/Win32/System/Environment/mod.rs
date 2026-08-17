windows_targets::link!("vertdll.dll" "system" fn CallEnclave(lproutine : isize, lpparameter : *const core::ffi::c_void, fwaitforthread : super::super::Foundation:: BOOL, lpreturnvalue : *mut *mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn CreateEnclave(hprocess : super::super::Foundation:: HANDLE, lpaddress : *const core::ffi::c_void, dwsize : usize, dwinitialcommitment : usize, flenclavetype : u32, lpenclaveinformation : *const core::ffi::c_void, dwinfolength : u32, lpenclaveerror : *mut u32) -> *mut core::ffi::c_void);
windows_targets::link!("userenv.dll" "system" fn CreateEnvironmentBlock(lpenvironment : *mut *mut core::ffi::c_void, htoken : super::super::Foundation:: HANDLE, binherit : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
windows_targets::link!("api-ms-win-core-enclave-l1-1-1.dll" "system" fn DeleteEnclave(lpaddress : *const core::ffi::c_void) -> super::super::Foundation:: BOOL);
windows_targets::link!("userenv.dll" "system" fn DestroyEnvironmentBlock(lpenvironment : *const core::ffi::c_void) -> super::super::Foundation:: BOOL);
windows_targets::link!("vertdll.dll" "system" fn EnclaveGetAttestationReport(enclavedata : *const u8, report : *mut core::ffi::c_void, buffersize : u32, outputsize : *mut u32) -> windows_sys::core::HRESULT);
windows_targets::link!("vertdll.dll" "system" fn EnclaveGetEnclaveInformation(informationsize : u32, enclaveinformation : *mut ENCLAVE_INFORMATION) -> windows_sys::core::HRESULT);
windows_targets::link!("vertdll.dll" "system" fn EnclaveSealData(datatoencrypt : *const core::ffi::c_void, datatoencryptsize : u32, identitypolicy : ENCLAVE_SEALING_IDENTITY_POLICY, runtimepolicy : u32, protectedblob : *mut core::ffi::c_void, buffersize : u32, protectedblobsize : *mut u32) -> windows_sys::core::HRESULT);
windows_targets::link!("vertdll.dll" "system" fn EnclaveUnsealData(protectedblob : *const core::ffi::c_void, protectedblobsize : u32, decrypteddata : *mut core::ffi::c_void, buffersize : u32, decrypteddatasize : *mut u32, sealingidentity : *mut ENCLAVE_IDENTITY, unsealingflags : *mut u32) -> windows_sys::core::HRESULT);
windows_targets::link!("vertdll.dll" "system" fn EnclaveVerifyAttestationReport(enclavetype : u32, report : *const core::ffi::c_void, reportsize : u32) -> windows_sys::core::HRESULT);
windows_targets::link!("kernel32.dll" "system" fn ExpandEnvironmentStringsA(lpsrc : windows_sys::core::PCSTR, lpdst : windows_sys::core::PSTR, nsize : u32) -> u32);
windows_targets::link!("userenv.dll" "system" fn ExpandEnvironmentStringsForUserA(htoken : super::super::Foundation:: HANDLE, lpsrc : windows_sys::core::PCSTR, lpdest : windows_sys::core::PSTR, dwsize : u32) -> super::super::Foundation:: BOOL);
windows_targets::link!("userenv.dll" "system" fn ExpandEnvironmentStringsForUserW(htoken : super::super::Foundation:: HANDLE, lpsrc : windows_sys::core::PCWSTR, lpdest : windows_sys::core::PWSTR, dwsize : u32) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn ExpandEnvironmentStringsW(lpsrc : windows_sys::core::PCWSTR, lpdst : windows_sys::core::PWSTR, nsize : u32) -> u32);
windows_targets::link!("kernel32.dll" "system" fn FreeEnvironmentStringsA(penv : windows_sys::core::PCSTR) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn FreeEnvironmentStringsW(penv : windows_sys::core::PCWSTR) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn GetCommandLineA() -> windows_sys::core::PCSTR);
windows_targets::link!("kernel32.dll" "system" fn GetCommandLineW() -> windows_sys::core::PCWSTR);
windows_targets::link!("kernel32.dll" "system" fn GetCurrentDirectoryA(nbufferlength : u32, lpbuffer : windows_sys::core::PSTR) -> u32);
windows_targets::link!("kernel32.dll" "system" fn GetCurrentDirectoryW(nbufferlength : u32, lpbuffer : windows_sys::core::PWSTR) -> u32);
windows_targets::link!("kernel32.dll" "system" fn GetEnvironmentStrings() -> windows_sys::core::PSTR);
windows_targets::link!("kernel32.dll" "system" fn GetEnvironmentStringsW() -> windows_sys::core::PWSTR);
windows_targets::link!("kernel32.dll" "system" fn GetEnvironmentVariableA(lpname : windows_sys::core::PCSTR, lpbuffer : windows_sys::core::PSTR, nsize : u32) -> u32);
windows_targets::link!("kernel32.dll" "system" fn GetEnvironmentVariableW(lpname : windows_sys::core::PCWSTR, lpbuffer : windows_sys::core::PWSTR, nsize : u32) -> u32);
windows_targets::link!("kernel32.dll" "system" fn InitializeEnclave(hprocess : super::super::Foundation:: HANDLE, lpaddress : *const core::ffi::c_void, lpenclaveinformation : *const core::ffi::c_void, dwinfolength : u32, lpenclaveerror : *mut u32) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn IsEnclaveTypeSupported(flenclavetype : u32) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn LoadEnclaveData(hprocess : super::super::Foundation:: HANDLE, lpaddress : *const core::ffi::c_void, lpbuffer : *const core::ffi::c_void, nsize : usize, flprotect : u32, lppageinformation : *const core::ffi::c_void, dwinfolength : u32, lpnumberofbyteswritten : *mut usize, lpenclaveerror : *mut u32) -> super::super::Foundation:: BOOL);
windows_targets::link!("api-ms-win-core-enclave-l1-1-1.dll" "system" fn LoadEnclaveImageA(lpenclaveaddress : *const core::ffi::c_void, lpimagename : windows_sys::core::PCSTR) -> super::super::Foundation:: BOOL);
windows_targets::link!("api-ms-win-core-enclave-l1-1-1.dll" "system" fn LoadEnclaveImageW(lpenclaveaddress : *const core::ffi::c_void, lpimagename : windows_sys::core::PCWSTR) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn NeedCurrentDirectoryForExePathA(exename : windows_sys::core::PCSTR) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn NeedCurrentDirectoryForExePathW(exename : windows_sys::core::PCWSTR) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn SetCurrentDirectoryA(lppathname : windows_sys::core::PCSTR) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn SetCurrentDirectoryW(lppathname : windows_sys::core::PCWSTR) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn SetEnvironmentStringsW(newenvironment : windows_sys::core::PCWSTR) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn SetEnvironmentVariableA(lpname : windows_sys::core::PCSTR, lpvalue : windows_sys::core::PCSTR) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn SetEnvironmentVariableW(lpname : windows_sys::core::PCWSTR, lpvalue : windows_sys::core::PCWSTR) -> super::super::Foundation:: BOOL);
windows_targets::link!("vertdll.dll" "system" fn TerminateEnclave(lpaddress : *const core::ffi::c_void, fwait : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
pub const ENCLAVE_FLAG_DYNAMIC_DEBUG_ACTIVE: u32 = 4u32;
pub const ENCLAVE_FLAG_DYNAMIC_DEBUG_ENABLED: u32 = 2u32;
pub const ENCLAVE_FLAG_FULL_DEBUG_ENABLED: u32 = 1u32;
pub const ENCLAVE_IDENTITY_POLICY_SEAL_EXACT_CODE: ENCLAVE_SEALING_IDENTITY_POLICY = 1i32;
pub const ENCLAVE_IDENTITY_POLICY_SEAL_INVALID: ENCLAVE_SEALING_IDENTITY_POLICY = 0i32;
pub const ENCLAVE_IDENTITY_POLICY_SEAL_SAME_AUTHOR: ENCLAVE_SEALING_IDENTITY_POLICY = 5i32;
pub const ENCLAVE_IDENTITY_POLICY_SEAL_SAME_FAMILY: ENCLAVE_SEALING_IDENTITY_POLICY = 4i32;
pub const ENCLAVE_IDENTITY_POLICY_SEAL_SAME_IMAGE: ENCLAVE_SEALING_IDENTITY_POLICY = 3i32;
pub const ENCLAVE_IDENTITY_POLICY_SEAL_SAME_PRIMARY_CODE: ENCLAVE_SEALING_IDENTITY_POLICY = 2i32;
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
pub type ENCLAVE_SEALING_IDENTITY_POLICY = i32;
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
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ENCLAVE_INFORMATION {
    pub EnclaveType: u32,
    pub Reserved: u32,
    pub BaseAddress: *mut core::ffi::c_void,
    pub Size: usize,
    pub Identity: ENCLAVE_IDENTITY,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ENCLAVE_VBS_BASIC_KEY_REQUEST {
    pub RequestSize: u32,
    pub Flags: u32,
    pub EnclaveSVN: u32,
    pub SystemKeyID: u32,
    pub CurrentSystemKeyID: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
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
#[repr(C)]
#[derive(Clone, Copy)]
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
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR32 {
    pub ThreadContext: [u32; 4],
    pub EntryPoint: u32,
    pub StackPointer: u32,
    pub ExceptionEntryPoint: u32,
    pub ExceptionStack: u32,
    pub ExceptionActive: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR64 {
    pub ThreadContext: [u64; 4],
    pub EntryPoint: u64,
    pub StackPointer: u64,
    pub ExceptionEntryPoint: u64,
    pub ExceptionStack: u64,
    pub ExceptionActive: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct VBS_ENCLAVE_REPORT {
    pub ReportSize: u32,
    pub ReportVersion: u32,
    pub EnclaveData: [u8; 64],
    pub EnclaveIdentity: ENCLAVE_IDENTITY,
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
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct VBS_ENCLAVE_REPORT_VARDATA_HEADER {
    pub DataType: u32,
    pub Size: u32,
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
