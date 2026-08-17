#[inline]
pub unsafe fn DavAddConnection<P1, P2, P3>(connectionhandle: *mut super::super::Foundation::HANDLE, remotename: P1, username: P2, password: P3, clientcert: &[u8]) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn DavAddConnection(connectionhandle : *mut super::super::Foundation:: HANDLE, remotename : windows_core::PCWSTR, username : windows_core::PCWSTR, password : windows_core::PCWSTR, clientcert : *const u8, certsize : u32) -> u32);
    unsafe { DavAddConnection(connectionhandle as _, remotename.param().abi(), username.param().abi(), password.param().abi(), core::mem::transmute(clientcert.as_ptr()), clientcert.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn DavCancelConnectionsToServer<P0>(lpname: P0, fforce: bool) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("davclnt.dll" "system" fn DavCancelConnectionsToServer(lpname : windows_core::PCWSTR, fforce : windows_core::BOOL) -> u32);
    unsafe { DavCancelConnectionsToServer(lpname.param().abi(), fforce.into()) }
}
#[inline]
pub unsafe fn DavDeleteConnection(connectionhandle: super::super::Foundation::HANDLE) -> u32 {
    windows_core::link!("netapi32.dll" "system" fn DavDeleteConnection(connectionhandle : super::super::Foundation:: HANDLE) -> u32);
    unsafe { DavDeleteConnection(connectionhandle) }
}
#[inline]
pub unsafe fn DavFlushFile(hfile: super::super::Foundation::HANDLE) -> u32 {
    windows_core::link!("netapi32.dll" "system" fn DavFlushFile(hfile : super::super::Foundation:: HANDLE) -> u32);
    unsafe { DavFlushFile(hfile) }
}
#[inline]
pub unsafe fn DavGetExtendedError(hfile: super::super::Foundation::HANDLE, exterror: *mut u32, exterrorstring: windows_core::PWSTR, cchsize: *mut u32) -> u32 {
    windows_core::link!("netapi32.dll" "system" fn DavGetExtendedError(hfile : super::super::Foundation:: HANDLE, exterror : *mut u32, exterrorstring : windows_core::PWSTR, cchsize : *mut u32) -> u32);
    unsafe { DavGetExtendedError(hfile, exterror as _, core::mem::transmute(exterrorstring), cchsize as _) }
}
#[inline]
pub unsafe fn DavGetHTTPFromUNCPath<P0>(uncpath: P0, url: Option<windows_core::PWSTR>, lpsize: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn DavGetHTTPFromUNCPath(uncpath : windows_core::PCWSTR, url : windows_core::PWSTR, lpsize : *mut u32) -> u32);
    unsafe { DavGetHTTPFromUNCPath(uncpath.param().abi(), url.unwrap_or(core::mem::zeroed()) as _, lpsize as _) }
}
#[inline]
pub unsafe fn DavGetTheLockOwnerOfTheFile<P0>(filename: P0, lockownername: Option<windows_core::PWSTR>, lockownernamelengthinbytes: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("davclnt.dll" "system" fn DavGetTheLockOwnerOfTheFile(filename : windows_core::PCWSTR, lockownername : windows_core::PWSTR, lockownernamelengthinbytes : *mut u32) -> u32);
    unsafe { DavGetTheLockOwnerOfTheFile(filename.param().abi(), lockownername.unwrap_or(core::mem::zeroed()) as _, lockownernamelengthinbytes as _) }
}
#[inline]
pub unsafe fn DavGetUNCFromHTTPPath<P0>(url: P0, uncpath: Option<windows_core::PWSTR>, lpsize: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn DavGetUNCFromHTTPPath(url : windows_core::PCWSTR, uncpath : windows_core::PWSTR, lpsize : *mut u32) -> u32);
    unsafe { DavGetUNCFromHTTPPath(url.param().abi(), uncpath.unwrap_or(core::mem::zeroed()) as _, lpsize as _) }
}
#[inline]
pub unsafe fn DavInvalidateCache<P0>(urlname: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("davclnt.dll" "system" fn DavInvalidateCache(urlname : windows_core::PCWSTR) -> u32);
    unsafe { DavInvalidateCache(urlname.param().abi()) }
}
#[inline]
pub unsafe fn DavRegisterAuthCallback(callback: PFNDAVAUTHCALLBACK, version: u32) -> u32 {
    windows_core::link!("davclnt.dll" "system" fn DavRegisterAuthCallback(callback : PFNDAVAUTHCALLBACK, version : u32) -> u32);
    unsafe { DavRegisterAuthCallback(callback, version) }
}
#[inline]
pub unsafe fn DavUnregisterAuthCallback(hcallback: u32) {
    windows_core::link!("davclnt.dll" "system" fn DavUnregisterAuthCallback(hcallback : u32));
    unsafe { DavUnregisterAuthCallback(hcallback) }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AUTHNEXTSTEP(pub i32);
pub const CancelRequest: AUTHNEXTSTEP = AUTHNEXTSTEP(2i32);
pub const DAV_AUTHN_SCHEME_BASIC: u32 = 1u32;
pub const DAV_AUTHN_SCHEME_CERT: u32 = 65536u32;
pub const DAV_AUTHN_SCHEME_DIGEST: u32 = 8u32;
pub const DAV_AUTHN_SCHEME_FBA: u32 = 1048576u32;
pub const DAV_AUTHN_SCHEME_NEGOTIATE: u32 = 16u32;
pub const DAV_AUTHN_SCHEME_NTLM: u32 = 2u32;
pub const DAV_AUTHN_SCHEME_PASSPORT: u32 = 4u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DAV_CALLBACK_AUTH_BLOB {
    pub pBuffer: *mut core::ffi::c_void,
    pub ulSize: u32,
    pub ulType: u32,
}
impl Default for DAV_CALLBACK_AUTH_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DAV_CALLBACK_AUTH_UNP {
    pub pszUserName: windows_core::PWSTR,
    pub ulUserNameLength: u32,
    pub pszPassword: windows_core::PWSTR,
    pub ulPasswordLength: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DAV_CALLBACK_CRED {
    pub AuthBlob: DAV_CALLBACK_AUTH_BLOB,
    pub UNPBlob: DAV_CALLBACK_AUTH_UNP,
    pub bAuthBlobValid: windows_core::BOOL,
    pub bSave: windows_core::BOOL,
}
pub const DefaultBehavior: AUTHNEXTSTEP = AUTHNEXTSTEP(0i32);
pub type PFNDAVAUTHCALLBACK = Option<unsafe extern "system" fn(lpwzservername: windows_core::PCWSTR, lpwzremotename: windows_core::PCWSTR, dwauthscheme: u32, dwflags: u32, pcallbackcred: *mut DAV_CALLBACK_CRED, nextstep: *mut AUTHNEXTSTEP, pfreecred: *mut PFNDAVAUTHCALLBACK_FREECRED) -> u32>;
pub type PFNDAVAUTHCALLBACK_FREECRED = Option<unsafe extern "system" fn(pbuffer: *const core::ffi::c_void) -> u32>;
pub const RetryRequest: AUTHNEXTSTEP = AUTHNEXTSTEP(1i32);
