// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms
//! Service Provider Order
use ctypes::c_int;
use shared::guiddef::LPGUID;
use shared::minwindef::{DWORD, LPDWORD};
extern "system" {
    pub fn WSCWriteProviderOrder(
        lpwdCatalogEntryId: LPDWORD,
        dwNumberOfEntries: DWORD,
    ) -> c_int;
}
FN!{stdcall LPWSCWRITEPROVIDERORDER(
    lpwdCatalogEntryId: LPDWORD,
    dwNumberOfEntries: DWORD,
) -> c_int}
#[cfg(target_pointer_width = "64")]
extern "system" {
    pub fn WSCWriteProviderOrder32(
        lpwdCatalogEntryId: LPDWORD,
        dwNumberOfEntries: DWORD,
    ) -> c_int;
    pub fn WSCWriteNameSpaceOrder(
        lpProviderId: LPGUID,
        dwNumberOfEntries: DWORD,
    ) -> c_int;
}
FN!{stdcall LPWSCWRITENAMESPACEORDER(
    lpProviderId: LPGUID,
    dwNumberOfEntries: DWORD,
) -> c_int}
#[cfg(target_pointer_width = "64")]
extern "system" {
    pub fn WSCWriteNameSpaceOrder32(
        lpProviderId: LPGUID,
        dwNumberOfEntries: DWORD,
    ) -> c_int;
}
