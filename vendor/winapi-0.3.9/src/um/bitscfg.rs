// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use ctypes::{c_uchar, c_ulong};
use shared::guiddef::REFIID;
use shared::wtypes::BSTR;
use um::oaidl::{IDispatch, IDispatchVtbl};
use um::unknwnbase::IUnknown;
use um::winnt::HRESULT;
RIDL!{#[uuid(0x29cfbbf7, 0x09e4, 0x4b97, 0xb0, 0xbc, 0xf2, 0x28, 0x7e, 0x3d, 0x8e, 0xb3)]
interface IBITSExtensionSetup(IBITSExtensionSetupVtbl): IDispatch(IDispatchVtbl) {
    fn EnableBITSUploads() -> HRESULT,
    fn DisableBITSUploads() -> HRESULT,
    fn GetCleanupTaskName(
        pTaskName: *mut BSTR,
    ) -> HRESULT,
    fn GetCleanupTask(
        riid: REFIID,
        ppUnk: *mut *mut IUnknown,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xd5d2d542, 0x5503, 0x4e64, 0x8b, 0x48, 0x72, 0xef, 0x91, 0xa3, 0x2e, 0xe1)]
interface IBITSExtensionSetupFactory(IBITSExtensionSetupFactoryVtbl): IDispatch(IDispatchVtbl) {
    fn GetObject(
        Path: BSTR,
        ppExtensionSetup: *mut *mut IBITSExtensionSetup,
    ) -> HRESULT,
}}
extern "system" {
    pub fn BSTR_UserSize(
        pFlags: *mut c_ulong,
        Offset: c_ulong,
        pBstr: *mut BSTR,
    ) -> c_ulong;
    pub fn BSTR_UserMarshal(
        pFlags: *mut c_ulong,
        pBuffer: *mut c_uchar,
        pBstr: *mut BSTR,
    ) -> *mut c_uchar;
    pub fn BSTR_UserUnmarshal(
        pFlags: *mut c_ulong,
        pBuffer: *mut c_uchar,
        pBstr: *mut BSTR,
    ) -> *mut c_uchar;
    pub fn BSTR_UserFree(
        pFlags: *mut c_ulong,
        pBstr: *mut BSTR,
    );
    pub fn BSTR_UserSize64(
        pFlags: *mut c_ulong,
        Offset: c_ulong,
        pBstr: *mut BSTR,
    ) -> c_ulong;
    pub fn BSTR_UserMarshal64(
        pFlags: *mut c_ulong,
        pBuffer: *mut c_uchar,
        pBstr: *mut BSTR,
    ) -> *mut c_uchar;
    pub fn BSTR_UserUnmarshal64(
        pFlags: *mut c_ulong,
        pBuffer: *mut c_uchar,
        pBstr: *mut BSTR,
    ) -> *mut c_uchar;
    pub fn BSTR_UserFree64(
        pFlags: *mut c_ulong,
        pBstr: *mut BSTR,
    );
}
