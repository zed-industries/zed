// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms
use shared::minwindef::{BOOL, LPVOID, ULONG};
use shared::ntdef::{BOOLEAN, HANDLE, LPCWSTR};
use um::minwinbase::LPSECURITY_ATTRIBUTES;
use um::winnt::PSID;
pub const PRIVATE_NAMESPACE_FLAG_DESTROY: ULONG = 0x00000001;
extern "system" {
    pub fn CreatePrivateNamespaceW(
        lpPrivateNamespaceAttributes: LPSECURITY_ATTRIBUTES,
        lpBoundaryDescriptor: LPVOID,
        lpAliasPrefix: LPCWSTR,
    ) -> HANDLE;
    pub fn OpenPrivateNamespaceW(
        lpBoundaryDescriptor: LPVOID,
        lpAliasPrefix: LPCWSTR,
    ) -> HANDLE;
    pub fn ClosePrivateNamespace(
        Handle: HANDLE,
        Flags: ULONG,
    ) -> BOOLEAN;
    pub fn CreateBoundaryDescriptorW(
        Name: LPCWSTR,
        Flags: ULONG,
    ) -> HANDLE;
    pub fn AddSIDToBoundaryDescriptor(
        BoundaryDescriptor: *mut HANDLE,
        RequiredSid: PSID,
    ) -> BOOL;
    pub fn DeleteBoundaryDescriptor(
        BoundaryDescriptor: HANDLE,
    ) -> ();
}
