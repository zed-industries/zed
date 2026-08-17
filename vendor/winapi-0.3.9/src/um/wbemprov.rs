// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use ctypes::{c_long, c_ulong};
use shared::minwindef::{BYTE, DWORD};
use shared::winerror::HRESULT;
use shared::wtypes::BSTR;
use um::oaidl::{VARIANT};
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::wbemcli::{
    IWbemClassObject, IWbemContext, IWbemHiPerfEnum, IWbemObjectAccess, IWbemObjectSink,
    IWbemObjectSinkVtbl, IWbemRefresher, IWbemServices
};
use um::winnt::{LONG, LPCWSTR, LPWSTR, WCHAR};
pub type WBEM_VARIANT = VARIANT;
pub type WBEM_WSTR = LPWSTR;
pub type WBEM_CWSTR = LPCWSTR;
ENUM!{enum WBEM_PROVIDER_REQUIREMENTS_TYPE {
    WBEM_REQUIREMENTS_START_POSTFILTER = 0,
    WBEM_REQUIREMENTS_STOP_POSTFILTER = 1,
    WBEM_REQUIREMENTS_RECHECK_SUBSCRIPTIONS = 2,
}}
// extern RPC_IF_HANDLE __MIDL_itf_wbemprov_0000_0000_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_wbemprov_0000_0000_v0_0_s_ifspec;
// EXTERN_C const IID LIBID_WbemProviders_v1;
// EXTERN_C const IID IID_IWbemPropertyProvider;
DEFINE_GUID!{IID_IWbemPropertyProvider,
    0xce61e841, 0x65bc, 0x11d0, 0xb6, 0xbd, 0x00, 0xaa, 0x00, 0x32, 0x40, 0xc7}
RIDL!{#[uuid(0xce61e841, 0x65bc, 0x11d0, 0xb6, 0xbd, 0x00, 0xaa, 0x00, 0x32, 0x40, 0xc7)]
interface IWbemPropertyProvider(IWbemPropertyProviderVtbl): IUnknown(IUnknownVtbl) {
    fn GetProperty(
        lFlags: c_long,
        strLocale: BSTR,
        strClassMapping: BSTR,
        strInstMapping: BSTR,
        strPropMapping: BSTR,
        pvValue: *mut VARIANT,
    ) -> HRESULT,
    fn PutProperty(
        lFlags: c_long,
        strLocale: BSTR,
        strClassMapping: BSTR,
        strInstMapping: BSTR,
        strPropMapping: BSTR,
        pvValue: *const VARIANT,
    ) -> HRESULT,
}}
// EXTERN_C const IID IID_IWbemUnboundObjectSink;
DEFINE_GUID!{IID_IWbemUnboundObjectSink,
    0xe246107b, 0xb06e, 0x11d0, 0xad, 0x61, 0x00, 0xc0, 0x4f, 0xd8, 0xfd, 0xff}
RIDL!{#[uuid(0xe246107b, 0xb06e, 0x11d0, 0xad, 0x61, 0x00, 0xc0, 0x4f, 0xd8, 0xfd, 0xff)]
interface IWbemUnboundObjectSink(IWbemUnboundObjectSinkVtbl): IUnknown(IUnknownVtbl) {
    fn IndicateToConsumer(
        pLogicalConsumer: *mut IWbemClassObject,
        lNumObjects: c_long,
        apObjects: *mut *mut IWbemClassObject,
    ) -> HRESULT,
}}
// EXTERN_C const IID IID_IWbemEventProvider;
DEFINE_GUID!{IID_IWbemEventProvider,
    0xe245105b, 0xb06e, 0x11d0, 0xad, 0x61, 0x00, 0xc0, 0x4f, 0xd8, 0xfd, 0xff}
RIDL!{#[uuid(0xe245105b, 0xb06e, 0x11d0, 0xad, 0x61, 0x00, 0xc0, 0x4f, 0xd8, 0xfd, 0xff)]
interface IWbemEventProvider(IWbemEventProviderVtbl): IUnknown(IUnknownVtbl) {
    fn ProvideEvents(
        pSink: *mut IWbemObjectSink,
        lFlags: c_long,
    ) -> HRESULT,
}}
// EXTERN_C const IID IID_IWbemEventProviderQuerySink;
DEFINE_GUID!{IID_IWbemEventProviderQuerySink,
    0x580acaf8, 0xfa1c, 0x11d0, 0xad, 0x72, 0x00, 0xc0, 0x4f, 0xd8, 0xfd, 0xff}
RIDL!{#[uuid(0x580acaf8, 0xfa1c, 0x11d0, 0xad, 0x72, 0x00, 0xc0, 0x4f, 0xd8, 0xfd, 0xff)]
interface IWbemEventProviderQuerySink(IWbemEventProviderQuerySinkVtbl): IUnknown(IUnknownVtbl) {
    fn NewQuery(
        dwId: c_ulong,
        wszQueryLanguage: WBEM_WSTR,
        wszQuery: WBEM_WSTR,
    ) -> HRESULT,
    fn CancelQuery(
        dwId: c_ulong,
    ) -> HRESULT,
}}
// EXTERN_C const IID IID_IWbemEventProviderSecurity;
DEFINE_GUID!{IID_IWbemEventProviderSecurity,
    0x631f7d96, 0xd993, 0x11d2, 0xb3, 0x39, 0x00, 0x10, 0x5a, 0x1f, 0x4a, 0xaf}
RIDL!{#[uuid(0x631f7d96, 0xd993, 0x11d2, 0xb3, 0x39, 0x00, 0x10, 0x5a, 0x1f, 0x4a, 0xaf)]
interface IWbemEventProviderSecurity(IWbemEventProviderSecurityVtbl): IUnknown(IUnknownVtbl) {
    fn AccessCheck(
        wszQueryLanguage: WBEM_CWSTR,
        wszQuery: WBEM_CWSTR,
        lSidLength: c_long,
        pSid: *const BYTE,
    ) -> HRESULT,
}}
// EXTERN_C const IID IID_IWbemEventConsumerProvider;
DEFINE_GUID!{IID_IWbemEventConsumerProvider,
    0xe246107a, 0xb06e, 0x11d0, 0xad, 0x61, 0x00, 0xc0, 0x4f, 0xd8, 0xfd, 0xff}
RIDL!{#[uuid(0xe246107a, 0xb06e, 0x11d0, 0xad, 0x61, 0x00, 0xc0, 0x4f, 0xd8, 0xfd, 0xff)]
interface IWbemEventConsumerProvider(IWbemEventConsumerProviderVtbl): IUnknown(IUnknownVtbl) {
    fn FindConsumer(
        pLogicalConsumer: *mut IWbemClassObject,
        ppConsumer: *mut *mut IWbemUnboundObjectSink,
    ) -> HRESULT,
}}
// EXTERN_C const IID IID_IWbemProviderInitSink;
DEFINE_GUID!{IID_IWbemProviderInitSink,
    0x1be41571, 0x91dd, 0x11d1, 0xae, 0xb2, 0x00, 0xc0, 0x4f, 0xb6, 0x88, 0x20}
RIDL!{#[uuid(0x1be41571, 0x91dd, 0x11d1, 0xae, 0xb2, 0x00, 0xc0, 0x4f, 0xb6, 0x88, 0x20)]
interface IWbemProviderInitSink(IWbemProviderInitSinkVtbl): IUnknown(IUnknownVtbl) {
    fn SetStatus(
        lStatus: LONG,
        lFlags: LONG,
    ) -> HRESULT,
}}
// EXTERN_C const IID IID_IWbemProviderInit;
DEFINE_GUID!{IID_IWbemProviderInit,
    0x1be41572, 0x91dd, 0x11d1, 0xae, 0xb2, 0x00, 0xc0, 0x4f, 0xb6, 0x88, 0x20}
RIDL!{#[uuid(0x1be41572, 0x91dd, 0x11d1, 0xae, 0xb2, 0x00, 0xc0, 0x4f, 0xb6, 0x88, 0x20)]
interface IWbemProviderInit(IWbemProviderInitVtbl): IUnknown(IUnknownVtbl) {
    fn Initialize(
        wszUser: LPWSTR,
        lFlags: LONG,
        wszNamespace: LPWSTR,
        wszLocale: LPWSTR,
        pNamespace: *mut IWbemServices,
        pCtx: *mut IWbemContext,
        pInitSink: *mut IWbemProviderInitSink,
    ) -> HRESULT,
}}
// EXTERN_C const IID IID_IWbemHiPerfProvider;
DEFINE_GUID!{IID_IWbemHiPerfProvider,
    0x49353c93, 0x516b, 0x11d1, 0xae, 0xa6, 0x00, 0xc0, 0x4f, 0xb6, 0x88, 0x20}
RIDL!{#[uuid(0x49353c93, 0x516b, 0x11d1, 0xae, 0xa6, 0x00, 0xc0, 0x4f, 0xb6, 0x88, 0x20)]
interface IWbemHiPerfProvider(IWbemHiPerfProviderVtbl): IUnknown(IUnknownVtbl) {
    fn QueryInstances(
        pNamespace: *mut IWbemServices,
        wszClass: *mut WCHAR,
        lFlags: c_long,
        pCtx: *mut IWbemContext,
        pSink: *mut IWbemObjectSink,
    ) -> HRESULT,
    fn CreateRefresher(
        pNamespace: *mut IWbemServices,
        lFlags: c_long,
        ppRefresher: *mut *mut IWbemRefresher,
    ) -> HRESULT,
    fn CreateRefreshableObject(
        pNamespace: *mut IWbemServices,
        pTemplate: *mut IWbemObjectAccess,
        pRefresher: *mut IWbemRefresher,
        lFlags: c_long,
        pContext: *mut IWbemContext,
        ppRefreshable: *mut *mut IWbemObjectAccess,
        plId: *mut c_long,
    ) -> HRESULT,
    fn StopRefreshing(
        pRefresher: *mut IWbemRefresher,
        lId: c_long,
        lFlags: c_long,
    ) -> HRESULT,
    fn CreateRefreshableEnum(
        pNamespace: *mut IWbemServices,
        wszClass: LPCWSTR,
        pRefresher: *mut IWbemRefresher,
        lFlags: c_long,
        pContext: *mut IWbemContext,
        pHiPerfEnum: *mut IWbemHiPerfEnum,
        plId: *mut c_long,
    ) -> HRESULT,
    fn GetObjects(
        pNamespace: *mut IWbemServices,
        lNumObjects: c_long,
        apObj: *mut *mut IWbemObjectAccess,
        lFlags: c_long,
        pContext: *mut IWbemContext,
    ) -> HRESULT,
}}
// EXTERN_C const IID IID_IWbemDecoupledRegistrar;
DEFINE_GUID!{IID_IWbemDecoupledRegistrar,
    0x1005cbcf, 0xe64f, 0x4646, 0xbc, 0xd3, 0x3a, 0x08, 0x9d, 0x8a, 0x84, 0xb4}
RIDL!{#[uuid(0x1005cbcf, 0xe64f, 0x4646, 0xbc, 0xd3, 0x3a, 0x08, 0x9d, 0x8a, 0x84, 0xb4)]
interface IWbemDecoupledRegistrar(IWbemDecoupledRegistrarVtbl): IUnknown(IUnknownVtbl) {
    fn Register(
        a_Flags: c_long,
        a_Context: *mut IWbemContext,
        a_User: LPCWSTR,
        a_Locale: LPCWSTR,
        a_Scope: LPCWSTR,
        a_Registration: LPCWSTR,
        pIUnknown: *mut IUnknown,
    ) -> HRESULT,
    fn UnRegister() -> HRESULT,
}}
DEFINE_GUID!{CLSID_WbemAdministrativeLocator,
    0xcb8555cc, 0x9128, 0x11d1, 0xad, 0x9b, 0x00, 0xc0, 0x4f, 0xd8, 0xfd, 0xff}
// class DECLSPEC_UUID("cb8555cc-9128-11d1-ad9b-00c04fd8fdff")
// WbemAdministrativeLocator;
DEFINE_GUID!{CLSID_WbemAuthenticatedLocator,
    0xcd184336, 0x9128, 0x11d1, 0xad, 0x9b, 0x00, 0xc0, 0x4f, 0xd8, 0xfd, 0xff}
// class DECLSPEC_UUID("cd184336-9128-11d1-ad9b-00c04fd8fdff")
// WbemAuthenticatedLocator;
DEFINE_GUID!{CLSID_WbemUnauthenticatedLocator,
    0x443E7B79, 0xDE31, 0x11d2, 0xB3, 0x40, 0x00, 0x10, 0x4B, 0xCC, 0x4B, 0x4A}
// class DECLSPEC_UUID("443E7B79-DE31-11d2-B340-00104BCC4B4A")
// WbemUnauthenticatedLocator;
DEFINE_GUID!{CLSID_WbemDecoupledRegistrar,
    0x4cfc7932, 0x0f9d, 0x4bef, 0x9c, 0x32, 0x8e, 0xa2, 0xa6, 0xb5, 0x6f, 0xcb}
// class DECLSPEC_UUID("4cfc7932-0f9d-4bef-9c32-8ea2a6b56fcb")
// WbemDecoupledRegistrar;
DEFINE_GUID!{CLSID_WbemDecoupledBasicEventProvider,
    0xf5f75737, 0x2843, 0x4f22, 0x93, 0x3d, 0xc7, 0x6a, 0x97, 0xcd, 0xa6, 0x2f}
// class DECLSPEC_UUID("f5f75737-2843-4f22-933d-c76a97cda62f")
// WbemDecoupledBasicEventProvider;
// EXTERN_C const IID IID_IWbemProviderIdentity;
DEFINE_GUID!{IID_IWbemProviderIdentity,
    0x631f7d97, 0xd993, 0x11d2, 0xb3, 0x39, 0x00, 0x10, 0x5a, 0x1f, 0x4a, 0xaf}
RIDL!{#[uuid(0x631f7d97, 0xd993, 0x11d2, 0xb3, 0x39, 0x00, 0x10, 0x5a, 0x1f, 0x4a, 0xaf)]
interface IWbemProviderIdentity(IWbemProviderIdentityVtbl): IUnknown(IUnknownVtbl) {
    fn SetRegistrationObject(
        lFlags: c_long,
        pProvReg: *mut IWbemClassObject,
    ) -> HRESULT,
}}
ENUM!{enum WBEM_EXTRA_RETURN_CODES {
    WBEM_S_INITIALIZED = 0,
    WBEM_S_LIMITED_SERVICE = 0x43001,
    WBEM_S_INDIRECTLY_UPDATED = WBEM_S_LIMITED_SERVICE + 1,
    WBEM_S_SUBJECT_TO_SDS = WBEM_S_INDIRECTLY_UPDATED + 1,
    WBEM_E_RETRY_LATER = 0x80043001,
    WBEM_E_RESOURCE_CONTENTION = WBEM_E_RETRY_LATER + 1,
}}
ENUM!{enum WBEM_PROVIDER_FLAGS {
    WBEM_FLAG_OWNER_UPDATE = 0x10000,
}}
// extern RPC_IF_HANDLE __MIDL_itf_wbemprov_0000_0008_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_wbemprov_0000_0008_v0_0_s_ifspec;
// EXTERN_C const IID IID_IWbemDecoupledBasicEventProvider;
DEFINE_GUID!{IID_IWbemDecoupledBasicEventProvider,
    0x86336d20, 0xca11, 0x4786, 0x9e, 0xf1, 0xbc, 0x8a, 0x94, 0x6b, 0x42, 0xfc}
RIDL!{#[uuid(0x86336d20, 0xca11, 0x4786, 0x9e, 0xf1, 0xbc, 0x8a, 0x94, 0x6b, 0x42, 0xfc)]
interface IWbemDecoupledBasicEventProvider(IWbemDecoupledBasicEventProviderVtbl):
    IWbemDecoupledRegistrar(IWbemDecoupledRegistrarVtbl) {
    fn GetSink(
        a_Flags: c_long,
        a_Context: *mut IWbemContext,
        a_Sink: *mut *mut IWbemObjectSink,
    ) -> HRESULT,
    fn GetService(
        a_Flags: c_long,
        a_Context: *mut IWbemContext,
        a_Service: *mut *mut IWbemServices,
    ) -> HRESULT,
}}
ENUM!{enum WBEM_BATCH_TYPE {
    WBEM_FLAG_BATCH_IF_NEEDED = 0,
    WBEM_FLAG_MUST_BATCH = 0x1,
    WBEM_FLAG_MUST_NOT_BATCH = 0x2,
}}
// extern RPC_IF_HANDLE __MIDL_itf_wbemprov_0000_0013_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_wbemprov_0000_0013_v0_0_s_ifspec;
// EXTERN_C const IID IID_IWbemEventSink;
DEFINE_GUID!{IID_IWbemEventSink,
    0x3ae0080a, 0x7e3a, 0x4366, 0xbf, 0x89, 0x0f, 0xee, 0xdc, 0x93, 0x16, 0x59}
RIDL!{#[uuid(0x3ae0080a, 0x7e3a, 0x4366, 0xbf, 0x89, 0x0f, 0xee, 0xdc, 0x93, 0x16, 0x59)]
interface IWbemEventSink(IWbemEventSinkVtbl): IWbemObjectSink(IWbemObjectSinkVtbl) {
    fn SetSinkSecurity(
        lSDLength: c_long,
        pSD: *mut BYTE,
    ) -> HRESULT,
    fn IsActive() -> HRESULT,
    fn GetRestrictedSink(
        lNumQueries: c_long,
        awszQueries: *const LPCWSTR,
        pCallback: *mut IUnknown,
        ppSink: *mut *mut IWbemEventSink,
    ) -> HRESULT,
    fn SetBatchingParameters(
        lFlags: LONG,
        dwMaxBufferSize: DWORD,
        dwMaxSendLatency: DWORD,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_wbemprov_0000_0014_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_wbemprov_0000_0014_v0_0_s_ifspec;
// unsigned c_long __RPC_USER BSTR_UserSize( __RPC__in unsigned c_long *, unsigned c_long, __RPC__in BSTR * );
// unsigned char * __RPC_USER BSTR_UserMarshal( __RPC__in unsigned c_long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * );
// unsigned char * __RPC_USER BSTR_UserUnmarshal(__RPC__in unsigned c_long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * );
// void __RPC_USER BSTR_UserFree( __RPC__in unsigned c_long *, __RPC__in BSTR * );
// unsigned c_long __RPC_USER VARIANT_UserSize( __RPC__in unsigned c_long *, unsigned c_long, __RPC__in VARIANT * );
// unsigned char * __RPC_USER VARIANT_UserMarshal( __RPC__in unsigned c_long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * );
// unsigned char * __RPC_USER VARIANT_UserUnmarshal(__RPC__in unsigned c_long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * );
// void __RPC_USER VARIANT_UserFree( __RPC__in unsigned c_long *, __RPC__in VARIANT * );
// unsigned c_long __RPC_USER BSTR_UserSize64( __RPC__in unsigned c_long *, unsigned c_long, __RPC__in BSTR * );
// unsigned char * __RPC_USER BSTR_UserMarshal64( __RPC__in unsigned c_long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * );
// unsigned char * __RPC_USER BSTR_UserUnmarshal64(__RPC__in unsigned c_long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * );
// void __RPC_USER BSTR_UserFree64( __RPC__in unsigned c_long *, __RPC__in BSTR * );
// unsigned c_long __RPC_USER VARIANT_UserSize64( __RPC__in unsigned c_long *, unsigned c_long, __RPC__in VARIANT * );
// unsigned char * __RPC_USER VARIANT_UserMarshal64( __RPC__in unsigned c_long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * );
// unsigned char * __RPC_USER VARIANT_UserUnmarshal64(__RPC__in unsigned c_long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * );
// void __RPC_USER VARIANT_UserFree64( __RPC__in unsigned c_long *, __RPC__in VARIANT * );
