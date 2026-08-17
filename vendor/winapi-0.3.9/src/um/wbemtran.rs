// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use ctypes::{c_long, c_void};
use shared::guiddef::REFIID;
use shared::minwindef::{BYTE, DWORD};
use shared::winerror::HRESULT;
use shared::wtypes::BSTR;
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::wbemcli::{IWbemCallResult, IWbemContext, IWbemObjectSink, IWbemServices};
use um::winnt::{LPCWSTR, LPWSTR};
// extern RPC_IF_HANDLE __MIDL_itf_wbemtran_0000_0000_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_wbemtran_0000_0000_v0_0_s_ifspec;
ENUM!{enum WBEM_LOGIN_TYPE {
    WBEM_FLAG_INPROC_LOGIN = 0,
    WBEM_FLAG_LOCAL_LOGIN = 1,
    WBEM_FLAG_REMOTE_LOGIN = 2,
    WBEM_AUTHENTICATION_METHOD_MASK = 0xf,
    WBEM_FLAG_USE_MULTIPLE_CHALLENGES = 0x10,
}}
pub type WBEM_128BITS = *mut BYTE;
// EXTERN_C const IID LIBID_WbemTransports_v1;
// EXTERN_C const IID IID_IWbemTransport;
DEFINE_GUID!{IID_IWbemTransport,
    0x553fe584, 0x2156, 0x11d0, 0xb6, 0xae, 0x00, 0xaa, 0x00, 0x32, 0x40, 0xc7}
RIDL!{#[uuid(0x553fe584, 0x2156, 0x11d0, 0xb6, 0xae, 0x00, 0xaa, 0x00, 0x32, 0x40, 0xc7)]
interface IWbemTransport(IWbemTransportVtbl): IUnknown(IUnknownVtbl) {
    fn Initialize() -> HRESULT,
}}
// EXTERN_C const IID IID_IWbemLevel1Login;
DEFINE_GUID!{IID_IWbemLevel1Login,
    0xf309ad18, 0xd86a, 0x11d0, 0xa0, 0x75, 0x00, 0xc0, 0x4f, 0xb6, 0x88, 0x20}
RIDL!{#[uuid(0xf309ad18, 0xd86a, 0x11d0, 0xa0, 0x75, 0x00, 0xc0, 0x4f, 0xb6, 0x88, 0x20)]
interface IWbemLevel1Login(IWbemLevel1LoginVtbl): IUnknown(IUnknownVtbl) {
    fn EstablishPosition(
        wszLocaleList: LPWSTR,
        dwNumLocales: DWORD,
        reserved: *mut DWORD,
    ) -> HRESULT,
    fn RequestChallenge(
        wszNetworkResource: LPWSTR,
        wszUser: LPWSTR,
        Nonce: WBEM_128BITS,
    ) -> HRESULT,
    fn WBEMLogin(
        wszPreferredLocale: LPWSTR,
        AccessToken: WBEM_128BITS,
        lFlags: c_long,
        pCtx: *mut IWbemContext,
        ppNamespace: *mut *mut IWbemServices,
    ) -> HRESULT,
    fn NTLMLogin(
        wszNetworkResource: LPWSTR,
        wszPreferredLocale: LPWSTR,
        lFlags: c_long,
        pCtx: *mut IWbemContext,
        ppNamespace: *mut *mut IWbemServices,
    ) -> HRESULT,
}}
// EXTERN_C const IID IID_IWbemConnectorLogin;
DEFINE_GUID!{IID_IWbemConnectorLogin,
    0xd8ec9cb1, 0xb135, 0x4f10, 0x8b, 0x1b, 0xc7, 0x18, 0x8b, 0xb0, 0xd1, 0x86}
RIDL!{#[uuid(0xd8ec9cb1, 0xb135, 0x4f10, 0x8b, 0x1b, 0xc7, 0x18, 0x8b, 0xb0, 0xd1, 0x86)]
interface IWbemConnectorLogin(IWbemConnectorLoginVtbl): IUnknown(IUnknownVtbl) {
    fn ConnectorLogin(
        wszNetworkResource: LPWSTR,
        wszPreferredLocale: LPWSTR,
        lFlags: c_long,
        pCtx: *mut IWbemContext,
        riid: REFIID,
        pInterface: *mut *mut c_void,
    ) -> HRESULT,
}}
// EXTERN_C const IID IID_IWbemAddressResolution;
DEFINE_GUID!{IID_IWbemAddressResolution,
    0xf7ce2e12, 0x8c90, 0x11d1, 0x9e, 0x7b, 0x00, 0xc0, 0x4f, 0xc3, 0x24, 0xa8}
RIDL!{#[uuid(0xf7ce2e12, 0x8c90, 0x11d1, 0x9e, 0x7b, 0x00, 0xc0, 0x4f, 0xc3, 0x24, 0xa8)]
interface IWbemAddressResolution(IWbemAddressResolutionVtbl): IUnknown(IUnknownVtbl) {
    fn Resolve(
        wszNamespacePath: LPWSTR,
        wszAddressType: LPWSTR,
        pdwAddressLength: *mut DWORD,
        pabBinaryAddress: *mut *mut BYTE,
    ) -> HRESULT,
}}
DEFINE_GUID!{CLSID_WbemLevel1Login,
    0x8BC3F05E, 0xD86B, 0x11d0, 0xA0, 0x75, 0x00, 0xC0, 0x4F, 0xB6, 0x88, 0x20}
// class DECLSPEC_UUID("8BC3F05E-D86B-11d0-A075-00C04FB68820")
// WbemLevel1Login;
DEFINE_GUID!{CLSID_WbemLocalAddrRes,
    0xA1044801, 0x8F7E, 0x11d1, 0x9E, 0x7C, 0x00, 0xC0, 0x4F, 0xC3, 0x24, 0xA8}
// class DECLSPEC_UUID("A1044801-8F7E-11d1-9E7C-00C04FC324A8")
// WbemLocalAddrRes;
DEFINE_GUID!{CLSID_WbemUninitializedClassObject,
    0x7a0227f6, 0x7108, 0x11d1, 0xad, 0x90, 0x00, 0xc0, 0x4f, 0xd8, 0xfd, 0xff}
// class DECLSPEC_UUID("7a0227f6-7108-11d1-ad90-00c04fd8fdff")
// WbemUninitializedClassObject;
// EXTERN_C const IID IID_IWbemClientTransport;
DEFINE_GUID!{IID_IWbemClientTransport,
    0xf7ce2e11, 0x8c90, 0x11d1, 0x9e, 0x7b, 0x00, 0xc0, 0x4f, 0xc3, 0x24, 0xa8}
RIDL!{#[uuid(0xf7ce2e11, 0x8c90, 0x11d1, 0x9e, 0x7b, 0x00, 0xc0, 0x4f, 0xc3, 0x24, 0xa8)]
interface IWbemClientTransport(IWbemClientTransportVtbl): IUnknown(IUnknownVtbl) {
    fn ConnectServer(
        strAddressType: BSTR,
        dwBinaryAddressLength: DWORD,
        abBinaryAddress: *mut BYTE,
        strNetworkResource: BSTR,
        strUser: BSTR,
        strPassword: BSTR,
        strLocale: BSTR,
        lSecurityFlags: c_long,
        strAuthority: BSTR,
        pCtx: *mut IWbemContext,
        ppNamespace: *mut *mut IWbemServices,
    ) -> HRESULT,
}}
// EXTERN_C const IID IID_IWbemClientConnectionTransport;
DEFINE_GUID!{IID_IWbemClientConnectionTransport,
    0xa889c72a, 0xfcc1, 0x4a9e, 0xaf, 0x61, 0xed, 0x07, 0x13, 0x33, 0xfb, 0x5b}
RIDL!{#[uuid(0xa889c72a, 0xfcc1, 0x4a9e, 0xaf, 0x61, 0xed, 0x07, 0x13, 0x33, 0xfb, 0x5b)]
interface IWbemClientConnectionTransport(IWbemClientConnectionTransportVtbl):
    IUnknown(IUnknownVtbl) {
    fn Open(
        strAddressType: BSTR,
        dwBinaryAddressLength: DWORD,
        abBinaryAddress: *mut BYTE,
        strObject: BSTR,
        strUser: BSTR,
        strPassword: BSTR,
        strLocale: BSTR,
        lFlags: c_long,
        pCtx: *mut IWbemContext,
        riid: REFIID,
        pInterface: *mut *mut c_void,
        pCallRes: *mut *mut IWbemCallResult,
    ) -> HRESULT,
    fn OpenAsync(
        strAddressType: BSTR,
        dwBinaryAddressLength: DWORD,
        abBinaryAddress: *mut BYTE,
        strObject: BSTR,
        strUser: BSTR,
        strPassword: BSTR,
        strLocale: BSTR,
        lFlags: c_long,
        pCtx: *mut IWbemContext,
        riid: REFIID,
        pResponseHandler: *mut IWbemObjectSink,
    ) -> HRESULT,
    fn Cancel(
        lFlags: c_long,
        pHandler: *mut IWbemObjectSink,
    ) -> HRESULT,
}}
DEFINE_GUID!{CLSID_WbemDCOMTransport,
    0xF7CE2E13, 0x8C90, 0x11d1, 0x9E, 0x7B, 0x00, 0xC0, 0x4F, 0xC3, 0x24, 0xA8}
// class DECLSPEC_UUID("F7CE2E13-8C90-11d1-9E7B-00C04FC324A8")
// WbemDCOMTransport;
// EXTERN_C const IID IID_IWbemConstructClassObject;
DEFINE_GUID!{IID_IWbemConstructClassObject,
    0x9ef76194, 0x70d5, 0x11d1, 0xad, 0x90, 0x00, 0xc0, 0x4f, 0xd8, 0xfd, 0xff}
RIDL!{#[uuid(0x9ef76194, 0x70d5, 0x11d1, 0xad, 0x90, 0x00, 0xc0, 0x4f, 0xd8, 0xfd, 0xff)]
interface IWbemConstructClassObject(IWbemConstructClassObjectVtbl): IUnknown(IUnknownVtbl) {
    fn SetInheritanceChain(
        lNumAntecedents: c_long,
        awszAntecedents: *mut LPWSTR,
    ) -> HRESULT,
    fn SetPropertyOrigin(
        wszPropertyName: LPCWSTR,
        lOriginIndex: c_long,
    ) -> HRESULT,
    fn SetMethodOrigin(
        wszMethodName: LPCWSTR,
        lOriginIndex: c_long,
    ) -> HRESULT,
    fn SetServerNamespace(
        wszServer: LPCWSTR,
        wszNamespace: LPCWSTR,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_wbemtran_0000_0008_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_wbemtran_0000_0008_v0_0_s_ifspec;
