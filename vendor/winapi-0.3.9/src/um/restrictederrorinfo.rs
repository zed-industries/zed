// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::wtypes::BSTR;
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::HRESULT;
RIDL!{#[uuid(0x82ba7092, 0x4c88, 0x427d, 0xa7, 0xbc, 0x16, 0xdd, 0x93, 0xfe, 0xb6, 0x7e)]
interface IRestrictedErrorInfo(IRestrictedErrorInfoVtbl): IUnknown(IUnknownVtbl) {
    fn GetErrorDetails(
        description: *mut BSTR,
        error: *mut HRESULT,
        restrictedDescription: *mut BSTR,
        capabilitySid: *mut BSTR,
    ) -> HRESULT,
    fn GetReference(
        reference: *mut BSTR,
    ) -> HRESULT,
}}
