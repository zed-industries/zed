// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::basetsd::DWORD_PTR;
use shared::guiddef::GUID;
use shared::minwindef::{DWORD, WORD};
use shared::winerror::HRESULT;
use shared::wtypes::{BSTR, VARIANT_BOOL, VARTYPE};
use um::oaidl::{DISPID, DISPID_UNKNOWN, DISPPARAMS, EXCEPINFO, IDispatch, IDispatchVtbl, VARIANT};
use um::servprov::IServiceProvider;
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::LCID;
DEFINE_GUID!{IID_IDispatchEx,
    0xa6ef9860, 0xc720, 0x11d0, 0x93, 0x37, 0x0, 0xa0, 0xc9, 0xd, 0xca, 0xa9}
DEFINE_GUID!{IID_IDispError,
    0xa6ef9861, 0xc720, 0x11d0, 0x93, 0x37, 0x0, 0xa0, 0xc9, 0xd, 0xca, 0xa9}
DEFINE_GUID!{IID_IVariantChangeType,
    0xa6ef9862, 0xc720, 0x11d0, 0x93, 0x37, 0x0, 0xa0, 0xc9, 0xd, 0xca, 0xa9}
DEFINE_GUID!{SID_VariantConversion,
    0x1f101481, 0xbccd, 0x11d0, 0x93, 0x36, 0x0, 0xa0, 0xc9, 0xd, 0xca, 0xa9}
DEFINE_GUID!{SID_GetCaller,
    0x4717cc40, 0xbcb9, 0x11d0, 0x93, 0x36, 0x0, 0xa0, 0xc9, 0xd, 0xca, 0xa9}
DEFINE_GUID!{SID_ProvideRuntimeContext,
    0x74a5040c, 0xdd0c, 0x48f0, 0xac, 0x85, 0x19, 0x4c, 0x32, 0x59, 0x18, 0xa}
DEFINE_GUID!{IID_IProvideRuntimeContext,
    0x10e2414a, 0xec59, 0x49d2, 0xbc, 0x51, 0x5a, 0xdd, 0x2c, 0x36, 0xfe, 0xbc}
DEFINE_GUID!{IID_IObjectIdentity,
    0xca04b7e6, 0xd21, 0x11d1, 0x8c, 0xc5, 0x0, 0xc0, 0x4f, 0xc2, 0xb0, 0x85}
DEFINE_GUID!{IID_ICanHandleException,
    0xc5598e60, 0xb307, 0x11d1, 0xb2, 0x7d, 0x0, 0x60, 0x08, 0xc3, 0xfb, 0xfb}
// pub const SID_GetScriptSite = IID_IActiveScriptSite;
pub const fdexNameCaseSensitive: DWORD = 0x00000001;
pub const fdexNameEnsure: DWORD = 0x00000002;
pub const fdexNameImplicit: DWORD = 0x00000004;
pub const fdexNameCaseInsensitive: DWORD = 0x00000008;
pub const fdexNameInternal: DWORD = 0x00000010;
pub const fdexNameNoDynamicProperties: DWORD = 0x00000020;
pub const fdexPropCanGet: DWORD = 0x00000001;
pub const fdexPropCannotGet: DWORD = 0x00000002;
pub const fdexPropCanPut: DWORD = 0x00000004;
pub const fdexPropCannotPut: DWORD = 0x00000008;
pub const fdexPropCanPutRef: DWORD = 0x00000010;
pub const fdexPropCannotPutRef: DWORD = 0x00000020;
pub const fdexPropNoSideEffects: DWORD = 0x00000040;
pub const fdexPropDynamicType: DWORD = 0x00000080;
pub const fdexPropCanCall: DWORD = 0x00000100;
pub const fdexPropCannotCall: DWORD = 0x00000200;
pub const fdexPropCanConstruct: DWORD = 0x00000400;
pub const fdexPropCannotConstruct: DWORD = 0x00000800;
pub const fdexPropCanSourceEvents: DWORD = 0x00001000;
pub const fdexPropCannotSourceEvents: DWORD = 0x00002000;
pub const grfdexPropCanAll: DWORD = fdexPropCanGet | fdexPropCanPut | fdexPropCanPutRef
    | fdexPropCanCall | fdexPropCanConstruct | fdexPropCanSourceEvents;
pub const grfdexPropCannotAll: DWORD = fdexPropCannotGet | fdexPropCannotPut | fdexPropCannotPutRef
    | fdexPropCannotCall | fdexPropCannotConstruct | fdexPropCannotSourceEvents;
pub const grfdexPropExtraAll: DWORD = fdexPropNoSideEffects | fdexPropDynamicType;
pub const grfdexPropAll: DWORD = grfdexPropCanAll | grfdexPropCannotAll | grfdexPropExtraAll;
pub const fdexEnumDefault: DWORD = 0x00000001;
pub const fdexEnumAll: DWORD = 0x00000002;
pub const DISPATCH_CONSTRUCT: DWORD = 0x4000;
pub const DISPID_THIS: DISPID = -613;
pub const DISPID_STARTENUM: DISPID = DISPID_UNKNOWN;
// extern RPC_IF_HANDLE __MIDL_itf_dispex_0000_0000_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_dispex_0000_0000_v0_0_s_ifspec;
// EXTERN_C const IID IID_IDispatchEx;
RIDL!{#[uuid(0xa6ef9860, 0xc720, 0x11d0, 0x93, 0x37, 0x00, 0xa0, 0xc9, 0x0d, 0xca, 0xa9)]
interface IDispatchEx(IDispatchExVtbl): IDispatch(IDispatchVtbl) {
    fn GetDispID(
        bstrName: BSTR,
        grfdex: DWORD,
        pid: *mut DISPID,
    ) -> HRESULT,
    fn InvokeEx(
        id: DISPID,
        lcid: LCID,
        wFlags: WORD,
        pdp: *mut DISPPARAMS,
        pvarRes: *mut VARIANT,
        pei: *mut EXCEPINFO,
        pspCaller: *mut IServiceProvider,
    ) -> HRESULT,
    fn DeleteMemberByName(
        bstrName: BSTR,
        grfdex: DWORD,
    ) -> HRESULT,
    fn DeleteMemberByDispID(
        id: DISPID,
    ) -> HRESULT,
    fn GetMemberProperties(
        id: DISPID,
        grfdexFetch: DWORD,
        pgrfdex: *mut DWORD,
    ) -> HRESULT,
    fn GetMemberName(
        id: DISPID,
        pbstrName: *mut BSTR,
    ) -> HRESULT,
    fn GetNextDispID(
        grfdex: DWORD,
        id: DISPID,
        pid: *mut DISPID,
    ) -> HRESULT,
    fn GetNameSpaceParent(
        ppunk: *mut *mut IUnknown,
    ) -> HRESULT,
}}
// HRESULT STDMETHODCALLTYPE IDispatchEx_RemoteInvokeEx_Proxy(
//     IDispatchEx * This,
//     DISPID id,
//     LCID lcid,
//     DWORD dwFlags,
//     DISPPARAMS *pdp,
//     VARIANT *pvarRes,
//     EXCEPINFO *pei,
//     IServiceProvider *pspCaller,
//     UINT cvarRefArg,
//     UINT *rgiRefArg,
//     VARIANT *rgvarRefArg);
// void __RPC_STUB IDispatchEx_RemoteInvokeEx_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// EXTERN_C const IID IID_IDispError;
RIDL!{#[uuid(0xa6ef9861, 0xc720, 0x11d0, 0x93, 0x37, 0x00, 0xa0, 0xc9, 0x0d, 0xca, 0xa9)]
interface IDispError(IDispErrorVtbl): IUnknown(IUnknownVtbl) {
    fn QueryErrorInfo(
        guidErrorType: GUID,
        ppde: *mut *mut IDispError,
    ) -> HRESULT,
    fn GetNext(
        ppde: *mut *mut IDispError,
    ) -> HRESULT,
    fn GetHresult(
        phr: *mut HRESULT,
    ) -> HRESULT,
    fn GetSource(
        pbstrSource: *mut BSTR,
    ) -> HRESULT,
    fn GetHelpInfo(
        pbstrFileName: *mut BSTR,
        pdwContext: *mut DWORD,
    ) -> HRESULT,
    fn GetDescription(
        pbstrDescription: *mut BSTR,
    ) -> HRESULT,
}}
// EXTERN_C const IID IID_IVariantChangeType;
RIDL!{#[uuid(0xa6ef9862, 0xc720, 0x11d0, 0x93, 0x37, 0x00, 0xa0, 0xc9, 0x0d, 0xca, 0xa9)]
interface IVariantChangeType(IVariantChangeTypeVtbl): IUnknown(IUnknownVtbl) {
    fn ChangeType(
        pvarDst: *mut VARIANT,
        pvarSrc: *mut VARIANT,
        lcid: LCID,
        vtNew: VARTYPE,
    ) -> HRESULT,
}}
// EXTERN_C const IID IID_IObjectIdentity;
RIDL!{#[uuid(0xca04b7e6, 0x0d21, 0x11d1, 0x8c, 0xc5, 0x00, 0xc0, 0x4f, 0xc2, 0xb0, 0x85)]
interface IObjectIdentity(IObjectIdentityVtbl): IUnknown(IUnknownVtbl) {
    fn IsEqualObject(
        punk: *mut IUnknown,
    ) -> HRESULT,
}}
// EXTERN_C const IID IID_ICanHandleException;
RIDL!{#[uuid(0xc5598e60, 0xb307, 0x11d1, 0xb2, 0x7d, 0x00, 0x60, 0x08, 0xc3, 0xfb, 0xfb)]
interface ICanHandleException(ICanHandleExceptionVtbl): IUnknown(IUnknownVtbl) {
    fn CanHandleException(
        pExcepInfo: *mut EXCEPINFO,
        pvar: *mut VARIANT,
    ) -> HRESULT,
}}
// EXTERN_C const IID IID_IProvideRuntimeContext;
RIDL!{#[uuid(0x10e2414a, 0xec59, 0x49d2, 0xbc, 0x51, 0x5a, 0xdd, 0x2c, 0x36, 0xfe, 0xbc)]
interface IProvideRuntimeContext(IProvideRuntimeContextVtbl): IUnknown(IUnknownVtbl) {
    fn GetCurrentSourceContext(
        pdwContext: *mut DWORD_PTR,
        pfExecutingGlobalCode: *mut VARIANT_BOOL,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_dispex_0000_0006_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_dispex_0000_0006_v0_0_s_ifspec;
// unsigned long __RPC_USER BSTR_UserSize( __RPC__in unsigned long *, unsigned long, __RPC__in BSTR * );
// unsigned char * __RPC_USER BSTR_UserMarshal( __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * );
// unsigned char * __RPC_USER BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * );
// void __RPC_USER BSTR_UserFree( __RPC__in unsigned long *, __RPC__in BSTR * );
// unsigned long __RPC_USER VARIANT_UserSize( __RPC__in unsigned long *, unsigned long, __RPC__in VARIANT * );
// unsigned char * __RPC_USER VARIANT_UserMarshal( __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * );
// unsigned char * __RPC_USER VARIANT_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * );
// void __RPC_USER VARIANT_UserFree( __RPC__in unsigned long *, __RPC__in VARIANT * );
// unsigned long __RPC_USER BSTR_UserSize64( __RPC__in unsigned long *, unsigned long, __RPC__in BSTR * );
// unsigned char * __RPC_USER BSTR_UserMarshal64( __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * );
// unsigned char * __RPC_USER BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * );
// void __RPC_USER BSTR_UserFree64( __RPC__in unsigned long *, __RPC__in BSTR * );
// unsigned long __RPC_USER VARIANT_UserSize64( __RPC__in unsigned long *, unsigned long, __RPC__in VARIANT * );
// unsigned char * __RPC_USER VARIANT_UserMarshal64( __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * );
// unsigned char * __RPC_USER VARIANT_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * );
// void __RPC_USER VARIANT_UserFree64( __RPC__in unsigned long *, __RPC__in VARIANT * );
// HRESULT STDMETHODCALLTYPE IDispatchEx_InvokeEx_Proxy(
//     IDispatchEx * This,
//     DISPID id,
//     LCID lcid,
//     WORD wFlags,
//     DISPPARAMS *pdp,
//     VARIANT *pvarRes,
//     EXCEPINFO *pei,
//     IServiceProvider *pspCaller);
// HRESULT STDMETHODCALLTYPE IDispatchEx_InvokeEx_Stub(
//     IDispatchEx * This,
//     DISPID id,
//     LCID lcid,
//     DWORD dwFlags,
//     DISPPARAMS *pdp,
//     VARIANT *pvarRes,
//     EXCEPINFO *pei,
//     IServiceProvider *pspCaller,
//     UINT cvarRefArg,
//     UINT *rgiRefArg,
//     VARIANT *rgvarRefArg);
