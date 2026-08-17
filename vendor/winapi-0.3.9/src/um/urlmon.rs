// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! URL Moniker interfaces
use shared::minwindef::DWORD;
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{HRESULT, LPCWSTR};
RIDL!{#[uuid(0x79eac9ee, 0xbaf9, 0x11ce, 0x8c, 0x82, 0x00, 0xaa, 0x00, 0x4b, 0xa9, 0x0b)]
interface IInternetSecurityManager(IInternetSecurityManagerVtbl): IUnknown(IUnknownVtbl) {
    fn SetSecuritySite() -> HRESULT,
    fn GetSecuritySite() -> HRESULT,
    fn MapUrlToZone(
        pwszUrl: LPCWSTR,
        pdwZone: *mut DWORD,
        dwFlags: DWORD,
    ) -> HRESULT,
    // TODO: the rest
}}
// TODO: the rest
