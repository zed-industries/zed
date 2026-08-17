// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// TODO:It is a minimal implementation.
use ctypes::c_void;
use shared::basetsd::UINT32;
use shared::guiddef::{GUID, REFGUID, REFIID};
use shared::ntdef::HRESULT;
use um::unknwnbase::{IUnknown, IUnknownVtbl};
RIDL!{#[uuid(0x1b8efec4, 0x3019, 0x4c27, 0x96, 0x4e, 0x36, 0x72, 0x02, 0x15, 0x69, 0x06)]
interface IPrintDocumentPackageTarget(IPrintDocumentPackageTargetVtbl): IUnknown(IUnknownVtbl) {
    fn GetPackageTargetTypes(
        targetCount: *mut UINT32,
        targetTypes: *mut *mut GUID,
    ) -> HRESULT,
    fn GetPackageTarget(
        guidTargetType: REFGUID,
        riid: REFIID,
        ppvTarget: *mut *mut c_void,
    ) -> HRESULT,
    fn Cancel() -> HRESULT,
}}
