// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::winerror::HRESULT;
use shared::wtypes::BSTR;
use um::oaidl::{IDispatch, IDispatchVtbl};
use um::wbemdisp::{ISWbemObject, ISWbemServices};
// extern RPC_IF_HANDLE __MIDL_itf_wbemads_0000_0000_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_wbemads_0000_0000_v0_0_s_ifspec;
// EXTERN_C const IID LIBID_WMIEXTENSIONLib;
// EXTERN_C const IID IID_IWMIExtension;
DEFINE_GUID!{IID_IWMIExtension,
    0xadc1f06e, 0x5c7e, 0x11d2, 0x8b, 0x74, 0x00, 0x10, 0x4b, 0x2a, 0xfb, 0x41}
RIDL!{#[uuid(0xadc1f06e, 0x5c7e, 0x11d2, 0x8b, 0x74, 0x00, 0x10, 0x4b, 0x2a, 0xfb, 0x41)]
interface IWMIExtension(IWMIExtensionVtbl): IDispatch(IDispatchVtbl) {
    fn get_WMIObjectPath(
        strWMIObjectPath: *mut BSTR,
    ) -> HRESULT,
    fn GetWMIObject(
        objWMIObject: *mut *mut ISWbemObject,
    ) -> HRESULT,
    fn GetWMIServices(
        objWMIServices: *mut *mut ISWbemServices,
    ) -> HRESULT,
}}
DEFINE_GUID!{CLSID_WMIExtension,
    0xf0975afe, 0x5c7f, 0x11d2, 0x8b, 0x74, 0x00, 0x10, 0x4b, 0x2a, 0xfb, 0x41}
// class DECLSPEC_UUID("f0975afe-5c7f-11d2-8b74-00104b2afb41")
// WMIExtension;
// extern RPC_IF_HANDLE __MIDL_itf_wbemads_0000_0002_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_wbemads_0000_0002_v0_0_s_ifspec;
// unsigned long             __RPC_USER  BSTR_UserSize( __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * );
// unsigned char * __RPC_USER  BSTR_UserMarshal( __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * );
// unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * );
// void                      __RPC_USER  BSTR_UserFree( __RPC__in unsigned long *, __RPC__in BSTR * );
// unsigned long             __RPC_USER  BSTR_UserSize64( __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * );
// unsigned char * __RPC_USER  BSTR_UserMarshal64( __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * );
// unsigned char * __RPC_USER  BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * );
// void                      __RPC_USER  BSTR_UserFree64( __RPC__in unsigned long *, __RPC__in BSTR * );
