// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use ctypes::{c_int, c_void};
use shared::basetsd::{SIZE_T, ULONG_PTR};
use shared::guiddef::{CLSID, GUID, IID, REFCLSID, REFGUID, REFIID};
use shared::minwindef::{BOOL, BYTE, DWORD, FILETIME, ULONG};
use shared::wtypesbase::{COAUTHINFO, DOUBLE, LPOLESTR, OLECHAR};
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{HANDLE, HRESULT, LARGE_INTEGER, LONG, LPWSTR, ULARGE_INTEGER};
STRUCT!{struct COSERVERINFO {
    dwReserved1: DWORD,
    pwszName: LPWSTR,
    pAuthInfo: *mut COAUTHINFO,
    dwReserved2: DWORD,
}}
pub type LPMARSHAL = *mut IMarshal;
RIDL!{#[uuid(0x00000003, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IMarshal(IMarshalVtbl): IUnknown(IUnknownVtbl) {
    fn GetUnmarshalClass(
        riid: REFIID,
        pv: *mut c_void,
        dwDestContext: DWORD,
        pvDestContext: *mut c_void,
        mshlflags: DWORD,
        pCid: *mut CLSID,
    ) -> HRESULT,
    fn GetMarshalSizeMax(
        riid: REFIID,
        pv: *mut c_void,
        dwDestContext: DWORD,
        pvDestContext: *mut c_void,
        mshlflags: DWORD,
        pSize: *mut DWORD,
    ) -> HRESULT,
    fn MarshalInterface(
        pStm: *mut IStream,
        riid: REFIID,
        pv: *mut c_void,
        dwDestContext: DWORD,
        pvDestContext: *mut c_void,
        mshlflags: DWORD,
    ) -> HRESULT,
    fn UnmarshalInterface(
        pStm: *mut IStream,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    fn ReleaseMarshalData(
        pStm: *mut IStream,
    ) -> HRESULT,
    fn DisconnectObject(
        dwReserved: DWORD,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xecc8691b, 0xc1db, 0x4dc0, 0x85, 0x5e, 0x65, 0xf6, 0xc5, 0x51, 0xaf, 0x49)]
interface INoMarshal(INoMarshalVtbl): IUnknown(IUnknownVtbl) {}}
RIDL!{#[uuid(0x94ea2b94, 0xe9cc, 0x49e0, 0xc0, 0xff, 0xee, 0x64, 0xca, 0x8f, 0x5b, 0x90)]
interface IAgileObject(IAgileObjectVtbl): IUnknown(IUnknownVtbl) {}}
ENUM!{enum ACTIVATIONTYPE {
    ACTIVATIONTYPE_UNCATEGORIZED = 0,
    ACTIVATIONTYPE_FROM_MONIKER = 0x1,
    ACTIVATIONTYPE_FROM_DATA = 0x2,
    ACTIVATIONTYPE_FROM_STORAGE = 0x4,
    ACTIVATIONTYPE_FROM_STREAM = 0x8,
    ACTIVATIONTYPE_FROM_FILE = 0x10,
}}
RIDL!{#[uuid(0x00000017, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IActivationFilter(IActivationFilterVtbl): IUnknown(IUnknownVtbl) {
    fn HandleActivation(
        dwActivationType: DWORD,
        rclsid: REFCLSID,
        pReplacementClsId: *mut CLSID,
    ) -> HRESULT,
}}
pub type LPMARSHAL2 = *mut IMarshal2;
RIDL!{#[uuid(0x000001cf, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IMarshal2(IMarshal2Vtbl): IMarshal(IMarshalVtbl) {}}
pub type LPMALLOC = *mut IMalloc;
RIDL!{#[uuid(0x00000002, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IMalloc(IMallocVtbl): IUnknown(IUnknownVtbl) {
    fn Alloc(
        cb: SIZE_T,
    ) -> *mut c_void,
    fn Realloc(
        pv: *mut c_void,
        cb: SIZE_T,
    ) -> *mut c_void,
    fn Free(
        pv: *mut c_void,
    ) -> (),
    fn GetSize(
        pv: *mut c_void,
    ) -> SIZE_T,
    fn DidAlloc(
        pv: *mut c_void,
    ) -> c_int,
    fn HeapMinimize() -> (),
}}
pub type LPSTDMARSHALINFO = IStdMarshalInfo;
RIDL!{#[uuid(0x00000018, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IStdMarshalInfo(IStdMarshalInfoVtbl): IUnknown(IUnknownVtbl) {
    fn GetClassForHandler(
        dwDestContext: DWORD,
        pvDestContext: *mut c_void,
        pClsid: *mut CLSID,
    ) -> HRESULT,
}}
ENUM!{enum EXTCONN {
    EXTCONN_STRONG = 0x1,
    EXTCONN_WEAK = 0x2,
    EXTCONN_CALLABLE = 0x4,
}}
RIDL!{#[uuid(0x00000019, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IExternalConnection(IExternalConnectionVtbl): IUnknown(IUnknownVtbl) {
    fn AddConnection(
        extconn: DWORD,
        reserved: DWORD,
    ) -> DWORD,
    fn ReleaseConnection(
        extconn: DWORD,
        reserved: DWORD,
        fLastReleaseCloses: BOOL,
    ) -> DWORD,
}}
pub type LPMULTIQI = *mut IMultiQI;
STRUCT!{struct MULTI_QI {
    pIID: *const IID,
    pItf: *mut IUnknown,
    hr: HRESULT,
}}
RIDL!{#[uuid(0x00000020, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IMultiQI(IMultiQIVtbl): IUnknown(IUnknownVtbl) {
    fn QueryMultipleInterfaces(
        cMQIs: ULONG,
        pMQIs: *mut MULTI_QI,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x000e0020, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface AsyncIMultiQI(AsyncIMultiQIVtbl): IUnknown(IUnknownVtbl) {
    fn Begin_QueryMultipleInterfaces(
        cMQIs: ULONG,
        pMQIs: *mut MULTI_QI,
    ) -> HRESULT,
    fn Finish_QueryMultipleInterfaces(
        pMQIs: *mut MULTI_QI,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x00000021, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IInternalUnknown(IInternalUnknownVtbl): IUnknown(IUnknownVtbl) {
    fn QueryInternalInterface(
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x00000100, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IEnumUnknown(IEnumUnknownVtbl): IUnknown(IUnknownVtbl) {
    fn Next(
        celt: ULONG,
        rgelt: *mut *mut IUnknown,
        pceltFetched: *mut ULONG,
    ) -> HRESULT,
    fn Skip(
        celt: ULONG,
    ) -> HRESULT,
    fn Reset() -> HRESULT,
    fn Clone(
        ppenum: *mut *mut IEnumUnknown,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x00000101, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IEnumString(IEnumStringVtbl): IUnknown(IUnknownVtbl) {
    fn Next(
        celt: ULONG,
        rgelt: *mut LPOLESTR,
        pceltFetched: *mut ULONG,
    ) -> HRESULT,
    fn Skip(
        celt: ULONG,
    ) -> HRESULT,
    fn Reset() -> HRESULT,
    fn Clone(
        ppenum: *mut *mut IEnumString,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x0c733a30, 0x2a1c, 0x11ce, 0xad, 0xe5, 0x00, 0xaa, 0x00, 0x44, 0x77, 0x3d)]
interface ISequentialStream(ISequentialStreamVtbl): IUnknown(IUnknownVtbl) {
    fn Read(
        pv: *mut c_void,
        cb: ULONG,
        pcbRead: *mut ULONG,
    ) -> HRESULT,
    fn Write(
        pv: *const c_void,
        cb: ULONG,
        pcbWritten: *mut ULONG,
    ) -> HRESULT,
}}
STRUCT!{struct STATSTG {
    pwcsName: LPOLESTR,
    type_: DWORD,
    cbSize: ULARGE_INTEGER,
    mtime: FILETIME,
    ctime: FILETIME,
    atime: FILETIME,
    grfMode: DWORD,
    grfLocksSupported: DWORD,
    clsid: CLSID,
    grfStateBits: DWORD,
    reserved: DWORD,
}}
ENUM!{enum STGTY {
    STGTY_STORAGE = 1,
    STGTY_STREAM = 2,
    STGTY_LOCKBYTES = 3,
    STGTY_PROPERTY = 4,
}}
ENUM!{enum STREAM_SEEK {
    STREAM_SEEK_SET = 0,
    STREAM_SEEK_CUR = 1,
    STREAM_SEEK_END = 2,
}}
ENUM!{enum LOCKTYPE {
    LOCK_WRITE = 1,
    LOCK_EXCLUSIVE = 2,
    LOCK_ONLYONCE = 4,
}}
pub type LPSTREAM = *mut IStream;
RIDL!{#[uuid(0x0000000c, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IStream(IStreamVtbl): ISequentialStream(ISequentialStreamVtbl) {
    fn Seek(
        dlibMove: LARGE_INTEGER,
        dwOrigin: DWORD,
        plibNewPosition: *mut ULARGE_INTEGER,
    ) -> HRESULT,
    fn SetSize(
        libNewSize: ULARGE_INTEGER,
    ) -> HRESULT,
    fn CopyTo(
        pstm: *mut IStream,
        cb: ULARGE_INTEGER,
        pcbRead: *mut ULARGE_INTEGER,
        pcbWritten: *mut ULARGE_INTEGER,
    ) -> HRESULT,
    fn Commit(
        grfCommitFlags: DWORD,
    ) -> HRESULT,
    fn Revert() -> HRESULT,
    fn LockRegion(
        libOffset: ULARGE_INTEGER,
        cb: ULARGE_INTEGER,
        dwLockType: DWORD,
    ) -> HRESULT,
    fn UnlockRegion(
        libOffset: ULARGE_INTEGER,
        cb: ULARGE_INTEGER,
        dwLockType: DWORD,
    ) -> HRESULT,
    fn Stat(
        pstatstg: *mut STATSTG,
        grfStatFlag: DWORD,
    ) -> HRESULT,
    fn Clone(
        ppstm: *mut *mut IStream,
    ) -> HRESULT,
}}
pub type RPCOLEDATAREP = ULONG;
STRUCT!{struct RPCOLEMESSAGE {
    reserved1: *mut c_void,
    dataRepresentation: RPCOLEDATAREP,
    Buffer: *mut c_void,
    cbBuffer: ULONG,
    iMethod: ULONG,
    reserved2: [*mut c_void; 5],
    rpcFlags: ULONG,
}}
pub type PRPCOLEMESSAGE = *mut RPCOLEMESSAGE;
RIDL!{#[uuid(0xd5f56b60, 0x593b, 0x101a, 0xb5, 0x69, 0x08, 0x00, 0x2b, 0x2d, 0xbf, 0x7a)]
interface IRpcChannelBuffer(IRpcChannelBufferVtbl): IUnknown(IUnknownVtbl) {
    fn GetBuffer(
        pMessage: *mut RPCOLEMESSAGE,
        riid: REFIID,
    ) -> HRESULT,
    fn SendReceive(
        pMessage: *mut RPCOLEMESSAGE,
        pStatus: *mut ULONG,
    ) -> HRESULT,
    fn FreeBuffer(
        pMessage: *mut RPCOLEMESSAGE,
    ) -> HRESULT,
    fn GetDestCtx(
        pdwDestContext: *mut DWORD,
        ppvDestContext: *mut *mut c_void,
    ) -> HRESULT,
    fn IsConnected() -> HRESULT,
}}
RIDL!{#[uuid(0x594f31d0, 0x7f19, 0x11d0, 0xb1, 0x94, 0x00, 0xa0, 0xc9, 0x0d, 0xc8, 0xbf)]
interface IRpcChannelBuffer2(IRpcChannelBuffer2Vtbl): IRpcChannelBuffer(IRpcChannelBufferVtbl) {
    fn GetProtocolVersion(
        pdwVersion: *mut DWORD,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xa5029fb6, 0x3c34, 0x11d1, 0x9c, 0x99, 0x00, 0xc0, 0x4f, 0xb9, 0x98, 0xaa)]
interface IAsyncRpcChannelBuffer(IAsyncRpcChannelBufferVtbl):
    IRpcChannelBuffer2(IRpcChannelBuffer2Vtbl) {
    fn Send(
        pMsg: *mut RPCOLEMESSAGE,
        pSync: *mut ISynchronize,
        pulStatus: *mut ULONG,
    ) -> HRESULT,
    fn Receive(
        pMsg: *mut RPCOLEMESSAGE,
        pulStatus: *mut ULONG,
    ) -> HRESULT,
    fn GetDestCtxEx(
        pMsg: *mut RPCOLEMESSAGE,
        pdwDestContext: *mut DWORD,
        ppvDestContext: *mut *mut c_void,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x25b15600, 0x0115, 0x11d0, 0xbf, 0x0d, 0x00, 0xaa, 0x00, 0xb8, 0xdf, 0xd2)]
interface IRpcChannelBuffer3(IRpcChannelBuffer3Vtbl): IRpcChannelBuffer2(IRpcChannelBuffer2Vtbl) {
    fn Send(
        pMsg: *mut RPCOLEMESSAGE,
        pulStatus: *mut ULONG,
    ) -> HRESULT,
    fn Receive(
        pMsg: *mut RPCOLEMESSAGE,
        ulSize: ULONG,
        pulStatus: *mut ULONG,
    ) -> HRESULT,
    fn Cancel(
        pMsg: *mut RPCOLEMESSAGE,
    ) -> HRESULT,
    fn GetCallContext(
        pMsg: *mut RPCOLEMESSAGE,
        riid: REFIID,
        pInterface: *mut *mut c_void,
    ) -> HRESULT,
    fn GetDestCtxEx(
        pMsg: *mut RPCOLEMESSAGE,
        pdwDestContext: *mut DWORD,
        ppvDestContext: *mut *mut c_void,
    ) -> HRESULT,
    fn GetState(
        pMsg: *mut RPCOLEMESSAGE,
        pState: *mut DWORD,
    ) -> HRESULT,
    fn RegisterAsync(
        pMsg: *mut RPCOLEMESSAGE,
        pAsyncMgr: *mut IAsyncManager,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x58a08519, 0x24c8, 0x4935, 0xb4, 0x82, 0x3f, 0xd8, 0x23, 0x33, 0x3a, 0x4f)]
interface IRpcSyntaxNegotiate(IRpcSyntaxNegotiateVtbl): IUnknown(IUnknownVtbl) {
    fn NegotiateSyntax(
        pMsg: *mut RPCOLEMESSAGE,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xd5f56a34, 0x593b, 0x101a, 0xb5, 0x69, 0x08, 0x00, 0x2b, 0x2d, 0xbf, 0x7a)]
interface IRpcProxyBuffer(IRpcProxyBufferVtbl): IUnknown(IUnknownVtbl) {
    fn Connect(
        pRpcChannelBuffer: *mut IRpcChannelBuffer,
    ) -> HRESULT,
    fn Disconnect() -> (),
}}
RIDL!{#[uuid(0xd5f56afc, 0x593b, 0x101a, 0xb5, 0x69, 0x08, 0x00, 0x2b, 0x2d, 0xbf, 0x7a)]
interface IRpcStubBuffer(IRpcStubBufferVtbl): IUnknown(IUnknownVtbl) {
    fn Connect(
        pUnkServer: *mut IUnknown,
    ) -> HRESULT,
    fn Disconnect() -> (),
    fn Invoke(
        _prpcmsg: *mut RPCOLEMESSAGE,
        _pRpcChannelBuffer: *mut IRpcChannelBuffer,
    ) -> HRESULT,
    fn IsIIDSupported(
        riid: REFIID,
    ) -> *mut IRpcStubBuffer,
    fn CountRefs() -> ULONG,
    fn DebugServerQueryInterface(
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    fn DebugServerRelease(
        pv: *mut c_void,
    ) -> (),
}}
RIDL!{#[uuid(0xd5f569d0, 0x593b, 0x101a, 0xb5, 0x69, 0x08, 0x00, 0x2b, 0x2d, 0xbf, 0x7a)]
interface IPSFactoryBuffer(IPSFactoryBufferVtbl): IUnknown(IUnknownVtbl) {
    fn CreateProxy(
        pUnkOuter: *mut IUnknown,
        riid: REFIID,
        ppProxy: *mut *mut IRpcProxyBuffer,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    fn CreateStub(
        riid: REFIID,
        pUnkServer: *mut *mut IUnknown,
        ppStub: *mut *mut IRpcStubBuffer,
    ) -> HRESULT,
}}
STRUCT!{struct SChannelHookCallInfo {
    iid: IID,
    cbSize: DWORD,
    uCausality: GUID,
    dwServerPid: DWORD,
    iMethod: DWORD,
    pObject: *mut c_void,
}}
RIDL!{#[uuid(0x1008c4a0, 0x7613, 0x11cf, 0x9a, 0xf1, 0x00, 0x20, 0xaf, 0x6e, 0x72, 0xf4)]
interface IChannelHook(IChannelHookVtbl): IUnknown(IUnknownVtbl) {
    fn ClientGetSize(
        uExtent: REFGUID,
        riid: REFIID,
        pDataSize: *mut ULONG,
    ) -> (),
    fn ClientFillBuffer(
        uExtent: REFGUID,
        riid: REFIID,
        pDataSize: *mut ULONG,
        pDataBuffer: *mut c_void,
    ) -> (),
    fn ClientNotify(
        uExtent: REFGUID,
        riid: REFIID,
        cbDataSize: ULONG,
        pDataBuffer: *mut c_void,
        lDataRep: DWORD,
        hrFault: HRESULT,
    ) -> (),
    fn ServerNotify(
        uExtent: REFGUID,
        riid: REFIID,
        cbDataSize: ULONG,
        pDataBuffer: *mut c_void,
        lDataRep: DWORD,
    ) -> (),
    fn ServerGetSize(
        uExtent: REFGUID,
        riid: REFIID,
        hrFault: HRESULT,
        pDataSize: *mut ULONG,
    ) -> (),
    fn ServerFillBuffer(
        uExtent: REFGUID,
        riid: REFIID,
        pDataSize: *mut ULONG,
        pDataBuffer: *mut c_void,
        hrFault: HRESULT,
    ) -> (),
}}
STRUCT!{struct SOLE_AUTHENTICATION_SERVICE {
    dwAuthnSvc: DWORD,
    dwAuthzSvc: DWORD,
    pPrincipalName: *mut OLECHAR,
    hr: HRESULT,
}}
pub type PSOLE_AUTHENTICATION_SERVICE = *mut SOLE_AUTHENTICATION_SERVICE;
ENUM!{enum EOLE_AUTHENTICATION_CAPABILITIES {
    EOAC_NONE = 0,
    EOAC_MUTUAL_AUTH = 0x1,
    EOAC_STATIC_CLOAKING = 0x20,
    EOAC_DYNAMIC_CLOAKING = 0x40,
    EOAC_ANY_AUTHORITY = 0x80,
    EOAC_MAKE_FULLSIC = 0x100,
    EOAC_DEFAULT = 0x800,
    EOAC_SECURE_REFS = 0x2,
    EOAC_ACCESS_CONTROL = 0x4,
    EOAC_APPID = 0x8,
    EOAC_DYNAMIC = 0x10,
    EOAC_REQUIRE_FULLSIC = 0x200,
    EOAC_AUTO_IMPERSONATE = 0x400,
    EOAC_DISABLE_AAA = 0x1000,
    EOAC_NO_CUSTOM_MARSHAL = 0x2000,
    EOAC_RESERVED1 = 0x4000,
}}
pub const COLE_DEFAULT_PRINCIPAL: *mut OLECHAR = -1isize as *mut OLECHAR;
pub const COLE_DEFAULT_AUTHINFO: *mut c_void = -1isize as *mut c_void;
STRUCT!{struct SOLE_AUTHENTICATION_INFO {
    dwAuthnSvc: DWORD,
    dwAuthzSvc: DWORD,
    pAuthInfo: *mut c_void,
}}
pub type PSOLE_AUTHENTICATION_INFO = *mut SOLE_AUTHENTICATION_INFO;
STRUCT!{struct SOLE_AUTHENTICATION_LIST {
    cAuthInfo: DWORD,
    aAuthInfo: *mut SOLE_AUTHENTICATION_INFO,
}}
pub type PSOLE_AUTHENTICATION_LIST = *mut SOLE_AUTHENTICATION_LIST;
RIDL!{#[uuid(0x0000013d, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IClientSecurity(IClientSecurityVtbl): IUnknown(IUnknownVtbl) {
    fn QueryBlanket(
        pProxy: *mut IUnknown,
        pAuthnSvc: *mut DWORD,
        pAuthzSvc: *mut DWORD,
        pServerPrincName: *mut *mut OLECHAR,
        pAuthnLevel: *mut DWORD,
        pImpLevel: *mut DWORD,
        pAuthInfo: *mut *mut c_void,
        pCapabilities: *mut DWORD,
    ) -> HRESULT,
    fn SetBlanket(
        pProxy: *mut IUnknown,
        dwAuthnSvc: DWORD,
        dwAuthzSvc: DWORD,
        pServerPrincName: *mut OLECHAR,
        dwAuthnLevel: DWORD,
        dwImpLevel: DWORD,
        pAuthInfo: *mut c_void,
        dwCapabilities: DWORD,
    ) -> HRESULT,
    fn CopyProxy(
        pProxy: *mut IUnknown,
        ppCopy: *mut *mut IUnknown,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x0000013e, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IServerSecurity(IServerSecurityVtbl): IUnknown(IUnknownVtbl) {
    fn QueryBlanket(
        pAuthnSvc: *mut DWORD,
        pAuthzSvc: *mut DWORD,
        pServerPrincName: *mut *mut OLECHAR,
        pAuthnLevel: *mut DWORD,
        pImpLevel: *mut DWORD,
        pPrivs: *mut *mut c_void,
        pCapabilities: *mut DWORD,
    ) -> HRESULT,
    fn ImpersonateClient() -> HRESULT,
    fn RevertToSelf() -> HRESULT,
    fn IsImpersonating() -> BOOL,
}}
ENUM!{enum RPCOPT_PROPERTIES {
    COMBND_RPCTIMEOUT = 0x1,
    COMBND_SERVER_LOCALITY = 0x2,
    COMBND_RESERVED1 = 0x4,
    COMBND_RESERVED2 = 0x5,
    COMBND_RESERVED3 = 0x8,
    COMBND_RESERVED4 = 0x10,
}}
ENUM!{enum RPCOPT_SERVER_LOCALITY_VALUES {
    SERVER_LOCALITY_PROCESS_LOCAL = 0,
    SERVER_LOCALITY_MACHINE_LOCAL = 1,
    SERVER_LOCALITY_REMOTE = 2,
}}
RIDL!{#[uuid(0x00000144, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IRpcOptions(IRpcOptionsVtbl): IUnknown(IUnknownVtbl) {
    fn Set(
        pPrx: *mut IUnknown,
        dwProperty: RPCOPT_PROPERTIES,
        dwValue: ULONG_PTR,
    ) -> HRESULT,
    fn Query(
        pPrx: *mut IUnknown,
        dwProperty: RPCOPT_PROPERTIES,
        pdwValue: *mut ULONG_PTR,
    ) -> HRESULT,
}}
ENUM!{enum GLOBALOPT_PROPERTIES {
    COMGLB_EXCEPTION_HANDLING = 1,
    COMGLB_APPID = 2,
    COMGLB_RPC_THREADPOOL_SETTING = 3,
    COMGLB_RO_SETTINGS = 4,
    COMGLB_UNMARSHALING_POLICY = 5,
    COMGLB_PROPERTIES_RESERVED1 = 6,
}}
ENUM!{enum GLOBALOPT_EH_VALUES {
    COMGLB_EXCEPTION_HANDLE = 0,
    COMGLB_EXCEPTION_DONOT_HANDLE_FATAL = 1,
    COMGLB_EXCEPTION_DONOT_HANDLE = COMGLB_EXCEPTION_DONOT_HANDLE_FATAL,
    COMGLB_EXCEPTION_DONOT_HANDLE_ANY = 2,
}}
ENUM!{enum GLOBALOPT_RPCTP_VALUES {
    COMGLB_RPC_THREADPOOL_SETTING_DEFAULT_POOL = 0,
    COMGLB_RPC_THREADPOOL_SETTING_PRIVATE_POOL = 1,
}}
ENUM!{enum GLOBALOPT_RO_FLAGS {
    COMGLB_STA_MODALLOOP_REMOVE_TOUCH_MESSAGES = 0x1,
    COMGLB_STA_MODALLOOP_SHARED_QUEUE_REMOVE_INPUT_MESSAGES = 0x2,
    COMGLB_STA_MODALLOOP_SHARED_QUEUE_DONOT_REMOVE_INPUT_MESSAGES = 0x4,
    COMGLB_FAST_RUNDOWN = 0x8,
    COMGLB_RESERVED1 = 0x10,
    COMGLB_RESERVED2 = 0x20,
    COMGLB_RESERVED3 = 0x40,
    COMGLB_STA_MODALLOOP_SHARED_QUEUE_REORDER_POINTER_MESSAGES = 0x80,
    COMGLB_RESERVED4 = 0x100,
    COMGLB_RESERVED5 = 0x200,
    COMGLB_RESERVED6 = 0x400,
}}
ENUM!{enum GLOBALOPT_UNMARSHALING_POLICY_VALUES {
    COMGLB_UNMARSHALING_POLICY_NORMAL = 0,
    COMGLB_UNMARSHALING_POLICY_STRONG = 1,
    COMGLB_UNMARSHALING_POLICY_HYBRID = 2,
}}
RIDL!{#[uuid(0x0000015b, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IGlobalOptions(IGlobalOptionsVtbl): IUnknown(IUnknownVtbl) {
    fn Set(
        dwProperty: GLOBALOPT_PROPERTIES,
        dwValue: ULONG_PTR,
    ) -> HRESULT,
    fn Query(
        dwProperty: GLOBALOPT_PROPERTIES,
        pdwValue: *mut ULONG_PTR,
    ) -> HRESULT,
}}
pub type LPSURROGATE = *mut ISurrogate;
RIDL!{#[uuid(0x00000022, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface ISurrogate(ISurrogateVtbl): IUnknown(IUnknownVtbl) {
    fn LoadDllServer(
        Clsid: REFCLSID,
    ) -> HRESULT,
    fn FreeSurrogate() -> HRESULT,
}}
pub type LPGLOBALINTERFACETABLE = *mut IGlobalInterfaceTable;
RIDL!{#[uuid(0x00000146, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IGlobalInterfaceTable(IGlobalInterfaceTableVtbl): IUnknown(IUnknownVtbl) {
    fn RegisterInterfaceInGlobal(
        pUnk: *mut IUnknown,
        riid: REFIID,
        pdwCookie: *mut DWORD,
    ) -> HRESULT,
    fn RevokeInterfaceFromGlobal(
        dwCookie: DWORD,
    ) -> HRESULT,
    fn GetInterfaceFromGlobal(
        dwCookie: DWORD,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x00000030, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface ISynchronize(ISynchronizeVtbl): IUnknown(IUnknownVtbl) {
    fn Wait(
        dwFlags: DWORD,
        dwMilliseconds: DWORD,
    ) -> HRESULT,
    fn Signal() -> HRESULT,
    fn Reset() -> HRESULT,
}}
RIDL!{#[uuid(0x00000031, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface ISynchronizeHandle(ISynchronizeHandleVtbl): IUnknown(IUnknownVtbl) {
    fn GetHandle(
        ph: *mut HANDLE,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x00000032, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface ISynchronizeEvent(ISynchronizeEventVtbl): ISynchronizeHandle(ISynchronizeHandleVtbl) {
    fn SetEventHandle(
        ph: *mut HANDLE,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x00000033, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface ISynchronizeContainer(ISynchronizeContainerVtbl): IUnknown(IUnknownVtbl) {
    fn AddSynchronize(
        pSync: *mut ISynchronize,
    ) -> HRESULT,
    fn WaitMultiple(
        dwFlags: DWORD,
        dwTimeOut: DWORD,
        ppSync: *mut *mut ISynchronize,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x00000025, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface ISynchronizeMutex(ISynchronizeMutexVtbl): ISynchronize(ISynchronizeVtbl) {
    fn ReleaseMutex() -> HRESULT,
}}
pub type LPCANCELMETHODCALLS = *mut ICancelMethodCalls;
RIDL!{#[uuid(0x00000029, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface ICancelMethodCalls(ICancelMethodCallsVtbl): IUnknown(IUnknownVtbl) {
    fn Cancel(
        ulSeconds: ULONG,
    ) -> HRESULT,
    fn TestCancel() -> HRESULT,
}}
ENUM!{enum DCOM_CALL_STATE {
    DCOM_NONE = 0,
    DCOM_CALL_COMPLETE = 0x1,
    DCOM_CALL_CANCELED = 0x2,
}}
RIDL!{#[uuid(0x0000002a, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IAsyncManager(IAsyncManagerVtbl): IUnknown(IUnknownVtbl) {
    fn CompleteCall(
        Result: HRESULT,
    ) -> HRESULT,
    fn GetCallContext(
        riid: REFIID,
        pInterface: *mut *mut c_void,
    ) -> HRESULT,
    fn GetState(
        pulStateFlags: *mut ULONG,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x1c733a30, 0x2a1c, 0x11ce, 0xad, 0xe5, 0x00, 0xaa, 0x00, 0x44, 0x77, 0x3d)]
interface ICallFactory(ICallFactoryVtbl): IUnknown(IUnknownVtbl) {
    fn CreateCall(
        riid: REFIID,
        pCtrlUnk: *mut IUnknown,
        riid2: REFIID,
        ppv: *mut *mut IUnknown,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x00000149, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IRpcHelper(IRpcHelperVtbl): IUnknown(IUnknownVtbl) {
    fn GetDCOMProtocolVersion(
        pComVersion: *mut DWORD,
    ) -> HRESULT,
    fn GetIIDFromOBJREF(
        pObjRef: *mut c_void,
        piid: *mut *mut IID,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xeb0cb9e8, 0x7996, 0x11d2, 0x87, 0x2e, 0x00, 0x00, 0xf8, 0x08, 0x08, 0x59)]
interface IReleaseMarshalBuffers(IReleaseMarshalBuffersVtbl): IUnknown(IUnknownVtbl) {
    fn ReleaseMarshalBuffer(
        pMsg: *mut RPCOLEMESSAGE,
        dwFlags: DWORD,
        pChnl: *mut IUnknown,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x0000002b, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IWaitMultiple(IWaitMultipleVtbl): IUnknown(IUnknownVtbl) {
    fn WaitMultiple(
        timeout: DWORD,
        pSync: *mut *mut ISynchronize,
    ) -> HRESULT,
    fn AddSynchronize(
        pSync: *mut ISynchronize,
    ) -> HRESULT,
}}
pub type LPADDRTRACKINGCONTROL = *mut IAddrTrackingControl;
RIDL!{#[uuid(0x00000147, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IAddrTrackingControl(IAddrTrackingControlVtbl): IUnknown(IUnknownVtbl) {
    fn EnableCOMDynamicAddrTracking() -> HRESULT,
    fn DisableCOMDynamicAddrTracking() -> HRESULT,
}}
pub type LPADDREXCLUSIONCONTROL = *mut IAddrExclusionControl;
RIDL!{#[uuid(0x00000148, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IAddrExclusionControl(IAddrExclusionControlVtbl): IUnknown(IUnknownVtbl) {
    fn GetCurrentAddrExclusionList(
        riid: REFIID,
        ppEnumerator: *mut *mut c_void,
    ) -> HRESULT,
    fn UpdateAddrExclusionList(
        pEnumerator: *mut IUnknown,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xdb2f3aca, 0x2f86, 0x11d1, 0x8e, 0x04, 0x00, 0xc0, 0x4f, 0xb9, 0x98, 0x9a)]
interface IPipeByte(IPipeByteVtbl): IUnknown(IUnknownVtbl) {
    fn Pull(
        buf: *mut BYTE,
        cRequest: ULONG,
        pcReturned: *mut ULONG,
    ) -> HRESULT,
    fn Push(
        buf: *mut BYTE,
        cSent: ULONG,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xdb2f3acb, 0x2f86, 0x11d1, 0x8e, 0x04, 0x00, 0xc0, 0x4f, 0xb9, 0x98, 0x9a)]
interface AsyncIPipeByte(AsyncIPipeByteVtbl): IUnknown(IUnknownVtbl) {
    fn Begin_Pull(
        cRequest: ULONG,
    ) -> HRESULT,
    fn Finish_Pull(
        buf: *mut BYTE,
        pcReturned: *mut ULONG,
    ) -> HRESULT,
    fn Begin_Push(
        buf: *mut BYTE,
        cSent: ULONG,
    ) -> HRESULT,
    fn Finish_Push() -> HRESULT,
}}
RIDL!{#[uuid(0xdb2f3acc, 0x2f86, 0x11d1, 0x8e, 0x04, 0x00, 0xc0, 0x4f, 0xb9, 0x98, 0x9a)]
interface IPipeLong(IPipeLongVtbl): IUnknown(IUnknownVtbl) {
    fn Pull(
        buf: *mut LONG,
        cRequest: ULONG,
        pcReturned: *mut ULONG,
    ) -> HRESULT,
    fn Push(
        buf: *mut LONG,
        cSent: ULONG,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xdb2f3acd, 0x2f86, 0x11d1, 0x8e, 0x04, 0x00, 0xc0, 0x4f, 0xb9, 0x98, 0x9a)]
interface AsyncIPipeLong(AsyncIPipeLongVtbl): IUnknown(IUnknownVtbl) {
    fn Begin_Pull(
        cRequest: ULONG,
    ) -> HRESULT,
    fn Finish_Pull(
        buf: *mut LONG,
        pcReturned: *mut ULONG,
    ) -> HRESULT,
    fn Begin_Push(
        buf: *mut LONG,
        cSent: ULONG,
    ) -> HRESULT,
    fn Finish_Push() -> HRESULT,
}}
RIDL!{#[uuid(0xdb2f3ace, 0x2f86, 0x11d1, 0x8e, 0x04, 0x00, 0xc0, 0x4f, 0xb9, 0x98, 0x9a)]
interface IPipeDouble(IPipeDoubleVtbl): IUnknown(IUnknownVtbl) {
    fn Pull(
        buf: *mut DOUBLE,
        cRequest: ULONG,
        pcReturned: *mut ULONG,
    ) -> HRESULT,
    fn Push(
        buf: *mut DOUBLE,
        cSent: ULONG,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xdb2f3acf, 0x2f86, 0x11d1, 0x8e, 0x04, 0x00, 0xc0, 0x4f, 0xb9, 0x98, 0x9a)]
interface AsyncIPipeDouble(AsyncIPipeDoubleVtbl): IUnknown(IUnknownVtbl) {
    fn Begin_Pull(
        cRequest: ULONG,
    ) -> HRESULT,
    fn Finish_Pull(
        buf: *mut DOUBLE,
        pcReturned: *mut ULONG,
    ) -> HRESULT,
    fn Begin_Push(
        buf: *mut DOUBLE,
        cSent: ULONG,
    ) -> HRESULT,
    fn Finish_Push() -> HRESULT,
}}
pub type CPFLAGS = DWORD;
STRUCT!{struct ContextProperty {
    policyId: GUID,
    flags: CPFLAGS,
    pUnk: *mut IUnknown,
}}
pub type LPENUMCONTEXTPROPS = *mut IEnumContextProps;
RIDL!{#[uuid(0x000001c1, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IEnumContextProps(IEnumContextPropsVtbl): IUnknown(IUnknownVtbl) {
    fn Next(
        celt: ULONG,
        pContextProperties: *mut ContextProperty,
        pceltFetched: *mut ULONG,
    ) -> HRESULT,
    fn Skip(
        celt: ULONG,
    ) -> HRESULT,
    fn Reset() -> HRESULT,
    fn Clone(
        ppEnumContextProps: *mut *mut IEnumContextProps,
    ) -> HRESULT,
    fn Count(
        pcelt: *mut ULONG,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x000001c0, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IContext(IContextVtbl): IUnknown(IUnknownVtbl) {
    fn SetProperty(
        rpolicyId: REFGUID,
        flags: CPFLAGS,
        pUnk: *mut IUnknown,
    ) -> HRESULT,
    fn RemoveProperty(
        rPolicyId: REFGUID,
    ) -> HRESULT,
    fn GetProperty(
        policyId: REFGUID,
        pFlags: *mut CPFLAGS,
        ppUnk: *mut *mut IUnknown,
    ) -> HRESULT,
    fn EnumContextProps(
        ppEnumContextProps: *mut *mut IEnumContextProps,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x000001c6, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IObjContext(IObjContextVtbl): IContext(IContextVtbl) {
    fn Reserved1() -> (),
    fn Reserved2() -> (),
    fn Reserved3() -> (),
    fn Reserved4() -> (),
    fn Reserved5() -> (),
    fn Reserved6() -> (),
    fn Reserved7() -> (),
}}
ENUM!{enum APTTYPEQUALIFIER {
    APTTYPEQUALIFIER_NONE = 0,
    APTTYPEQUALIFIER_IMPLICIT_MTA = 1,
    APTTYPEQUALIFIER_NA_ON_MTA = 2,
    APTTYPEQUALIFIER_NA_ON_STA = 3,
    APTTYPEQUALIFIER_NA_ON_IMPLICIT_MTA = 4,
    APTTYPEQUALIFIER_NA_ON_MAINSTA = 5,
    APTTYPEQUALIFIER_APPLICATION_STA= 6,
}}
ENUM!{enum APTTYPE {
    APTTYPE_CURRENT = -1i32 as u32,
    APTTYPE_STA = 0,
    APTTYPE_MTA = 1,
    APTTYPE_NA = 2,
    APTTYPE_MAINSTA = 3,
}}
ENUM!{enum THDTYPE {
    THDTYPE_BLOCKMESSAGES = 0,
    THDTYPE_PROCESSMESSAGES = 1,
}}
pub type APARTMENTID = DWORD;
RIDL!{#[uuid(0x000001ce, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IComThreadingInfo(IComThreadingInfoVtbl): IUnknown(IUnknownVtbl) {
    fn GetCurrentApartmentType(
        pAptType: *mut APTTYPE,
    ) -> HRESULT,
    fn GetCurrentThreadType(
        pThreadType: *mut THDTYPE,
    ) -> HRESULT,
    fn GetCurrentLogicalThreadId(
        pguidLogicalThreadId: *mut GUID,
    ) -> HRESULT,
    fn SetCurrentLogicalThreadId(
        rguid: REFGUID,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x72380d55, 0x8d2b, 0x43a3, 0x85, 0x13, 0x2b, 0x6e, 0xf3, 0x14, 0x34, 0xe9)]
interface IProcessInitControl(IProcessInitControlVtbl): IUnknown(IUnknownVtbl) {
    fn ResetInitializerTimeout(
        dwSecondsRemaining: DWORD,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x00000040, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IFastRundown(IFastRundownVtbl): IUnknown(IUnknownVtbl) {}}
ENUM!{enum CO_MARSHALING_CONTEXT_ATTRIBUTES {
    CO_MARSHALING_SOURCE_IS_APP_CONTAINER = 0,
    CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_1 = 0x80000000,
    CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_2 = 0x80000001,
    CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_3 = 0x80000002,
    CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_4 = 0x80000003,
    CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_5 = 0x80000004,
    CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_6 = 0x80000005,
    CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_7 = 0x80000006,
    CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_8 = 0x80000007,
    CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_9 = 0x80000008,
}}
RIDL!{#[uuid(0xd8f2f5e6, 0x6102, 0x4863, 0x9f, 0x26, 0x38, 0x9a, 0x46, 0x76, 0xef, 0xde)]
interface IMarshalingStream(IMarshalingStreamVtbl): IStream(IStreamVtbl) {
    fn GetMarshalingContextAttribute(
        attribute: CO_MARSHALING_CONTEXT_ATTRIBUTES,
        pAttributeValue: *mut ULONG_PTR,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xc03f6a43, 0x65a4, 0x9818, 0x98, 0x7e, 0xe0, 0xb8, 0x10, 0xd2, 0xa6, 0xf2)]
interface IAgileReference(IAgileReferenceVtbl): IUnknown(IUnknownVtbl) {
    fn Resolve(
        riid: REFIID,
        ppvObjectReference: *mut *mut c_void,
    ) -> HRESULT,
}}
