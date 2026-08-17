// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// TODO:It is a minimal implementation.
use shared::guiddef::CLSID;
use shared::minwindef::{DWORD, ULONG};
use shared::ntdef::HRESULT;
use shared::wtypes::{CLIPFORMAT, VARTYPE};
use shared::wtypesbase::{LPCOLESTR, LPOLESTR};
use um::oaidl::{IErrorLog, VARIANT};
use um::unknwnbase::{IUnknown, IUnknownVtbl};
ENUM!{enum READYSTATE {
    READYSTATE_UNINITIALIZED = 0,
    READYSTATE_LOADING = 1,
    READYSTATE_LOADED = 2,
    READYSTATE_INTERACTIVE = 3,
    READYSTATE_COMPLETE = 4,
}}
ENUM!{enum PROPBAG2_TYPE {
    PROPBAG2_TYPE_UNDEFINED = 0,
    PROPBAG2_TYPE_DATA = 1,
    PROPBAG2_TYPE_URL = 2,
    PROPBAG2_TYPE_OBJECT = 3,
    PROPBAG2_TYPE_STREAM = 4,
    PROPBAG2_TYPE_STORAGE = 5,
    PROPBAG2_TYPE_MONIKER = 6,
}}
STRUCT!{struct PROPBAG2 {
    dwType: DWORD,
    vt: VARTYPE,
    cfType: CLIPFORMAT,
    dwHint: DWORD,
    pstrName: LPOLESTR,
    clsid: CLSID,
}}
RIDL!{#[uuid(0x22f55882, 0x280b, 0x11d0, 0xa8, 0xa9, 0x00, 0xa0, 0xc9, 0x0c, 0x20, 0x04)]
interface IPropertyBag2(IPropertyBag2Vtbl): IUnknown(IUnknownVtbl) {
    fn Read(
        cProperties: ULONG,
        pPropBag: *const PROPBAG2,
        pErrLog: *const IErrorLog,
        pvarValue: *mut VARIANT,
        phrError: *mut HRESULT,
    ) -> HRESULT,
    fn Write(
        cProperties: ULONG,
        pPropBag: *const PROPBAG2,
        pvarValue: *const VARIANT,
    ) -> HRESULT,
    fn CountProperties(
        pcProperties: *mut ULONG,
    ) -> HRESULT,
    fn GetPropertyInfo(
        iProperty: ULONG,
        cProperties: ULONG,
        pPropBag: *mut PROPBAG2,
        pcProperties: *mut ULONG,
    ) -> HRESULT,
    fn LoadObject(
        pstrName: LPCOLESTR,
        dwHint: DWORD,
        pUnkObject: *const IUnknown,
        pErrLog: *const IErrorLog,
    ) -> HRESULT,
}}
pub type LPPROPERTYBAG2 = *mut IPropertyBag2;
