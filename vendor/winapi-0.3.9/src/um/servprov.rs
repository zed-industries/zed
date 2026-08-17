// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use ctypes::c_void;
use shared::guiddef::{REFGUID, REFIID};
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::HRESULT;
pub type LPSERVICEPROVIDER = *mut IServiceProvider;
RIDL!{#[uuid(0x6d5140c1, 0x7436, 0x11ce, 0x80, 0x34, 0x00, 0xaa, 0x00, 0x60, 0x09, 0xfa)]
interface IServiceProvider(IServiceProviderVtbl): IUnknown(IUnknownVtbl) {
    fn QueryService(
        guidService: REFGUID,
        riid: REFIID,
        ppvObject: *mut *mut c_void,
    ) -> HRESULT,
    fn RemoteQueryService(
        guidService: REFGUID,
        riid: REFIID,
        ppvObject: *mut *mut IUnknown,
    ) -> HRESULT,
}}
