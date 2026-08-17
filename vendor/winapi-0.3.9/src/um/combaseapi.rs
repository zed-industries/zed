// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Base Component Object Model defintions.
use ctypes::{c_int, c_void};
use shared::basetsd::{SIZE_T, UINT64, ULONG_PTR};
use shared::guiddef::{CLSID, GUID, LPCLSID, LPIID, REFCLSID, REFGUID, REFIID};
use shared::minwindef::{BOOL, DWORD, FILETIME, HGLOBAL, LPDWORD, LPHANDLE, LPVOID, ULONG};
use shared::rpcdce::{RPC_AUTHZ_HANDLE, RPC_AUTH_IDENTITY_HANDLE};
use shared::wtypesbase::{
    CLSCTX, CLSCTX_INPROC_HANDLER, CLSCTX_INPROC_SERVER, CLSCTX_LOCAL_SERVER, CLSCTX_REMOTE_SERVER,
    LPCOLESTR, LPOLESTR, OLECHAR,
};
use um::objidl::SOLE_AUTHENTICATION_SERVICE;
use um::objidlbase::{
    APTTYPE, APTTYPEQUALIFIER, COSERVERINFO, IActivationFilter, IAgileReference, LPMALLOC,
    LPMARSHAL, LPSTREAM, LPSURROGATE, MULTI_QI,
};
use um::propidl::PROPVARIANT;
use um::unknwnbase::{IUnknown, LPUNKNOWN};
use um::winnt::{HANDLE, HRESULT, LARGE_INTEGER, LONG, PSECURITY_DESCRIPTOR, PVOID, ULARGE_INTEGER};
#[inline]
pub fn LISet32(li: &mut LARGE_INTEGER, v: DWORD) {
    unsafe {
        li.u_mut().HighPart = if (v as LONG) < 0 {
            -1
        } else {
            0
        };
        li.u_mut().LowPart = v;
    }
}
#[inline]
pub fn ULISet32(li: &mut ULARGE_INTEGER, v: DWORD) {
    unsafe {
        li.u_mut().HighPart = 0;
        li.u_mut().LowPart = v;
    }
}
pub const CLSCTX_INPROC: CLSCTX = CLSCTX_INPROC_SERVER | CLSCTX_INPROC_HANDLER;
pub const CLSCTX_ALL: CLSCTX = CLSCTX_INPROC_SERVER | CLSCTX_INPROC_HANDLER | CLSCTX_LOCAL_SERVER
    | CLSCTX_REMOTE_SERVER;
pub const CLSCTX_SERVER: CLSCTX = CLSCTX_INPROC_SERVER | CLSCTX_LOCAL_SERVER
    | CLSCTX_REMOTE_SERVER;
ENUM!{enum REGCLS {
    REGCLS_SINGLEUSE = 0,
    REGCLS_MULTIPLEUSE = 1,
    REGCLS_MULTI_SEPARATE = 2,
    REGCLS_SUSPENDED = 4,
    REGCLS_SURROGATE = 8,
    REGCLS_AGILE = 0x10,
}}
ENUM!{enum COINITBASE {
    COINITBASE_MULTITHREADED = 0x0,
}}
extern "system" {
    pub fn CoGetMalloc(
        dwMemContext: DWORD,
        ppMalloc: *mut LPMALLOC,
    ) -> HRESULT;
    pub fn CreateStreamOnHGlobal(
        hGlobal: HGLOBAL,
        fDeleteOnRelease: BOOL,
        ppstm: *mut LPSTREAM,
    ) -> HRESULT;
    pub fn GetHGlobalFromStream(
        pstm: LPSTREAM,
        phglobal: *mut HGLOBAL,
    ) -> HRESULT;
    pub fn CoUninitialize() -> ();
    pub fn CoGetCurrentProcess() -> DWORD;
    pub fn CoInitializeEx(
        pvReserved: LPVOID,
        dwCoInit: DWORD,
    ) -> HRESULT;
    pub fn CoGetCallerTID(
        lpdwTID: LPDWORD,
    ) -> HRESULT;
    pub fn CoGetCurrentLogicalThreadId(
        pguid: *mut GUID,
    ) -> HRESULT;
    pub fn CoGetContextToken(
        pToken: *mut ULONG_PTR,
    ) -> HRESULT;
    pub fn CoGetDefaultContext(
        aptType: APTTYPE,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT;
    pub fn CoGetApartmentType(
        pAptType: *mut APTTYPE,
        pAptQualifier: *mut APTTYPEQUALIFIER,
    ) -> HRESULT;
}
STRUCT!{struct ServerInformation {
    dwServerPid: DWORD,
    dwServerTid: DWORD,
    ui64ServerAddress: UINT64,
}}
pub type PServerInformation = *mut ServerInformation;
extern "system" {
    pub fn CoDecodeProxy(
        dwClientPid: DWORD,
        ui64ProxyAddress: UINT64,
        pServerInformation: PServerInformation,
    ) -> HRESULT;
}
DECLARE_HANDLE!{CO_MTA_USAGE_COOKIE, CO_MTA_USAGE_COOKIE__}
extern "system" {
    pub fn CoIncrementMTAUsage(
        pCookie: *mut CO_MTA_USAGE_COOKIE,
    ) -> HRESULT;
    pub fn CoDecrementMTAUsage(
        Cookie: CO_MTA_USAGE_COOKIE,
    ) -> HRESULT;
    pub fn CoAllowUnmarshalerCLSID(
        clsid: REFCLSID,
    ) -> HRESULT;
    pub fn CoGetObjectContext(
        riid: REFIID,
        ppv: *mut LPVOID,
    ) -> HRESULT;
    pub fn CoGetClassObject(
        rclsid: REFCLSID,
        dwClsContext: DWORD,
        pvReserved: LPVOID,
        riid: REFIID,
        ppv: *mut LPVOID,
    ) -> HRESULT;
    pub fn CoRegisterClassObject(
        rclsid: REFCLSID,
        pUnk: LPUNKNOWN,
        dwClsContext: DWORD,
        flags: DWORD,
        lpdwRegister: LPDWORD,
    ) -> HRESULT;
    pub fn CoRevokeClassObject(
        dwRegister: DWORD,
    ) -> HRESULT;
    pub fn CoResumeClassObjects() -> HRESULT;
    pub fn CoSuspendClassObjects() -> HRESULT;
    pub fn CoAddRefServerProcess() -> ULONG;
    pub fn CoReleaseServerProcess() -> ULONG;
    pub fn CoGetPSClsid(
        riid: REFIID,
        pClsid: *mut CLSID,
    ) -> HRESULT;
    pub fn CoRegisterPSClsid(
        riid: REFIID,
        rclsid: REFCLSID,
    ) -> HRESULT;
    pub fn CoRegisterSurrogate(
        pSurrogate: LPSURROGATE,
    ) -> HRESULT;
    pub fn CoGetMarshalSizeMax(
        pulSize: *mut ULONG,
        riid: REFIID,
        pUnk: LPUNKNOWN,
        dwDestContext: DWORD,
        pvDestContext: LPVOID,
        mshlflags: DWORD,
    ) -> HRESULT;
    pub fn CoMarshalInterface(
        pStm: LPSTREAM,
        riid: REFIID,
        pUnk: LPUNKNOWN,
        dwDestContext: DWORD,
        pvDestContext: LPVOID,
        mshlflags: DWORD,
    ) -> HRESULT;
    pub fn CoUnmarshalInterface(
        pStm: LPSTREAM,
        riid: REFIID,
        ppv: *mut LPVOID,
    ) -> HRESULT;
    pub fn CoMarshalHresult(
        pstm: LPSTREAM,
        hresult: HRESULT,
    ) -> HRESULT;
    pub fn CoUnmarshalHresult(
        pstm: LPSTREAM,
        phresult: *mut HRESULT,
    ) -> HRESULT;
    pub fn CoReleaseMarshalData(
        pstm: LPSTREAM,
    ) -> HRESULT;
    pub fn CoDisconnectObject(
        pUnk: LPUNKNOWN,
        dwReserved: DWORD,
    ) -> HRESULT;
    pub fn CoLockObjectExternal(
        pUnk: LPUNKNOWN,
        fLock: BOOL,
        fLastUnlockReleases: BOOL,
    ) -> HRESULT;
    pub fn CoGetStandardMarshal(
        riid: REFIID,
        pUnk: LPUNKNOWN,
        dwDestContext: DWORD,
        pvDestContext: LPVOID,
        mshlflags: DWORD,
        ppMarshal: *mut LPMARSHAL,
    ) -> HRESULT;
    pub fn CoGetStdMarshalEx(
        pUnkOuter: LPUNKNOWN,
        smexflags: DWORD,
        ppUnkInner: *mut LPUNKNOWN,
    ) -> HRESULT;
}
ENUM!{enum STDMSHLFLAGS {
    SMEXF_SERVER = 0x01,
    SMEXF_HANDLER = 0x02,
}}
extern "system" {
    pub fn CoIsHandlerConnected(
        pUnk: LPUNKNOWN,
    ) -> BOOL;
    pub fn CoMarshalInterThreadInterfaceInStream(
        riid: REFIID,
        pUnk: LPUNKNOWN,
        ppStm: *mut LPSTREAM,
    ) -> HRESULT;
    pub fn CoGetInterfaceAndReleaseStream(
        pStm: LPSTREAM,
        iid: REFIID,
        ppv: *mut LPVOID,
    ) -> HRESULT;
    pub fn CoCreateFreeThreadedMarshaler(
        punkOuter: LPUNKNOWN,
        ppunkMarshal: *mut LPUNKNOWN,
    ) -> HRESULT;
    pub fn CoFreeUnusedLibraries();
    pub fn CoFreeUnusedLibrariesEx(
        dwUnloadDelay: DWORD,
        dwReserved: DWORD,
    );
    pub fn CoDisconnectContext(
        dwTimeout: DWORD,
    )-> HRESULT;
    pub fn CoInitializeSecurity(
        pSecDesc: PSECURITY_DESCRIPTOR,
        cAuthSvc: LONG,
        asAuthSvc: *mut SOLE_AUTHENTICATION_SERVICE,
        pReserved1: *mut c_void,
        dwAuthnLevel: DWORD,
        dwImpLevel: DWORD,
        pAuthList: *mut c_void,
        dwCapabilities: DWORD,
        pReserved3: *mut c_void,
    ) -> HRESULT;
    pub fn CoGetCallContext(
        riid: REFIID,
        ppInterface: *mut *mut c_void,
    ) -> HRESULT;
    pub fn CoQueryProxyBlanket(
        pProxy: *mut IUnknown,
        pwAuthnSvc: *mut DWORD,
        pAuthzSvc: *mut DWORD,
        pServerPrincName: *mut LPOLESTR,
        pAuthnLevel: *mut DWORD,
        pImpLevel: *mut DWORD,
        pAuthInfo: *mut RPC_AUTH_IDENTITY_HANDLE,
        pCapabilites: *mut DWORD,
    ) -> HRESULT;
    pub fn CoSetProxyBlanket(
        pProxy: *mut IUnknown,
        dwAuthnSvc: DWORD,
        dwAuthzSvc: DWORD,
        pServerPrincName: *mut OLECHAR,
        dwAuthnLevel: DWORD,
        dwImpLevel: DWORD,
        pAuthInfo: RPC_AUTH_IDENTITY_HANDLE,
        dwCapabilities: DWORD,
    ) -> HRESULT;
    pub fn CoCopyProxy(
        pProxy: *mut IUnknown,
        ppCopy: *mut *mut IUnknown,
    ) -> HRESULT;
    pub fn CoQueryClientBlanket(
        pAuthnSvc: *mut DWORD,
        pAuthzSvc: *mut DWORD,
        pServerPrincName: *mut LPOLESTR,
        pAuthnLevel: *mut DWORD,
        pImpLevel: *mut DWORD,
        pPrivs: *mut RPC_AUTHZ_HANDLE,
        pCapabilities: *mut DWORD,
    ) -> HRESULT;
    pub fn CoImpersonateClient() -> HRESULT;
    pub fn CoRevertToSelf() -> HRESULT;
    pub fn CoQueryAuthenticationServices(
        pcAuthSvc: *mut DWORD,
        asAuthSvc: *mut *mut SOLE_AUTHENTICATION_SERVICE,
    ) -> HRESULT;
    pub fn CoSwitchCallContext(
        pNewObject: *mut IUnknown,
        ppOldObject: *mut *mut IUnknown,
    ) -> HRESULT;
}
pub const COM_RIGHTS_EXECUTE: DWORD = 1;
pub const COM_RIGHTS_EXECUTE_LOCAL: DWORD = 2;
pub const COM_RIGHTS_EXECUTE_REMOTE: DWORD = 4;
pub const COM_RIGHTS_ACTIVATE_LOCAL: DWORD = 8;
pub const COM_RIGHTS_ACTIVATE_REMOTE: DWORD = 16;
extern "system" {
    pub fn CoCreateInstance(
        rclsid: REFCLSID,
        pUnkOuter: LPUNKNOWN,
        dwClsContext: DWORD,
        riid: REFIID,
        ppv: *mut LPVOID,
    ) -> HRESULT;
    pub fn CoCreateInstanceEx(
        Clsid: REFCLSID,
        punkOuter: *mut IUnknown,
        dwClsCtx: DWORD,
        pServerInfo: *mut COSERVERINFO,
        dwCount: DWORD,
        pResults: *mut MULTI_QI,
    ) -> HRESULT;
    pub fn CoRegisterActivationFilter(
        pActivationFilter: *mut IActivationFilter,
    ) -> HRESULT;
    pub fn CoCreateInstanceFromApp(
        Clsid: REFCLSID,
        punkOuter: *mut IUnknown,
        dwClsCtx: DWORD,
        reserved: PVOID,
        dwCount: DWORD,
        pResults: *mut MULTI_QI,
    ) -> HRESULT;
    pub fn CoGetCancelObject(
        dwThreadId: DWORD,
        iid: REFIID,
        ppUnk: *mut *mut c_void,
    ) -> HRESULT;
    pub fn CoSetCancelObject(
        pUnk: *mut *mut IUnknown,
    ) -> HRESULT;
    pub fn CoCancelCall(
        dwThreadId: DWORD,
        ulTimeout: ULONG,
    ) -> HRESULT;
    pub fn CoTestCancel() -> HRESULT;
    pub fn CoEnableCallCancellation(
        pReserved: LPVOID,
    ) -> HRESULT;
    pub fn CoDisableCallCancellation(
        pReserved: LPVOID,
    ) -> HRESULT;
    pub fn StringFromCLSID(
        rclsid: REFCLSID,
        lplpsz: *mut LPOLESTR,
    ) -> HRESULT;
    pub fn CLSIDFromString(
        lpsz: LPCOLESTR,
        pclsid: LPCLSID,
    ) -> HRESULT;
    pub fn StringFromIID(
        rclsid: REFIID,
        lplpsz: *mut LPOLESTR,
    ) -> HRESULT;
    pub fn IIDFromString(
        lpsz: LPCOLESTR,
        lpiid: LPIID,
    ) -> HRESULT;
    pub fn ProgIDFromCLSID(
        clsid: REFCLSID,
        lplpszProgID: *mut LPOLESTR,
    ) -> HRESULT;
    pub fn CLSIDFromProgID(
        lpszProgID: LPCOLESTR,
        lpclsid: LPCLSID,
    ) -> HRESULT;
    pub fn StringFromGUID2(
        rguid: REFGUID,
        lpsz: LPOLESTR,
        cchMax: c_int,
    ) -> c_int;
    pub fn CoCreateGuid(
        pguid: *mut GUID,
    ) -> HRESULT;
    pub fn PropVariantCopy(
        pvarDest: *mut PROPVARIANT,
        pvarSrc: *const PROPVARIANT,
    ) -> HRESULT;
    pub fn PropVariantClear(
        pvar: *mut PROPVARIANT,
    ) -> HRESULT;
    pub fn FreePropVariantArray(
        cVariants: ULONG,
        rgvars: *mut PROPVARIANT,
    ) -> HRESULT;
    pub fn CoWaitForMultipleHandles(
        dwFlags: DWORD,
        dwTimeout: DWORD,
        cHandles: ULONG,
        pHandles: LPHANDLE,
        lpdwindex: LPDWORD,
    ) -> HRESULT;
}
ENUM!{enum COWAIT_FLAGS {
    COWAIT_DEFAULT = 0,
    COWAIT_WAITALL = 1,
    COWAIT_ALERTABLE = 2,
    COWAIT_INPUTAVAILABLE = 4,
    COWAIT_DISPATCH_CALLS = 8,
    COWAIT_DISPATCH_WINDOW_MESSAGES = 0x10,
}}
ENUM!{enum CWMO_FLAGS {
    CWMO_DEFAULT = 0,
    CWMO_DISPATCH_CALLS = 1,
    CWMO_DISPATCH_WINDOW_MESSAGES = 2,
}}
extern "system" {
    pub fn CoWaitForMultipleObjects(
        dwFlags: DWORD,
        dwTimeout: DWORD,
        cHandles: ULONG,
        pHandles: *const HANDLE,
        lpdwindex: LPDWORD,
    ) -> HRESULT;
}
pub const CWMO_MAX_HANDLES: ULONG = 56;
extern "system" {
    pub fn CoGetTreatAsClass(
        clsidOld: REFCLSID,
        pClsidNew: LPCLSID,
    ) -> HRESULT;
    pub fn CoInvalidateRemoteMachineBindings(
        pszMachineName: LPOLESTR,
    ) -> HRESULT;
}
ENUM!{enum AgileReferenceOptions {
    AGILEREFERENCE_DEFAULT = 0,
    AGILEREFERENCE_DELAYEDMARSHAL = 1,
}}
extern "system" {
    pub fn RoGetAgileReference(
        options: AgileReferenceOptions,
        riid: REFIID,
        pUnk: *mut IUnknown,
        ppAgileReference: *mut *mut IAgileReference,
    ) -> HRESULT;
}
FN!{stdcall LPFNGETCLASSOBJECT(
    REFCLSID,
    REFIID,
    *mut LPVOID,
) -> HRESULT}
FN!{stdcall LPFNCANUNLOADNOW() -> HRESULT}
extern "system" {
    pub fn DllGetClassObject(
        rclsid: REFCLSID,
        riid: REFIID,
        ppv: *mut LPVOID,
    ) -> HRESULT;
    pub fn DllCanUnloadNow() -> HRESULT;
    pub fn CoTaskMemAlloc(
        cb: SIZE_T,
    ) -> LPVOID;
    pub fn CoTaskMemRealloc(
        pv: LPVOID,
        cb: SIZE_T,
    ) -> LPVOID;
    pub fn CoTaskMemFree(
        pv: LPVOID,
    );
    pub fn CoFileTimeNow(
        lpFileTime: *mut FILETIME,
    ) -> HRESULT;
    pub fn CLSIDFromProgIDEx(
        lpszProgID: LPCOLESTR,
        lpclsid: LPCLSID,
    ) -> HRESULT;
}
