// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::basetsd::PUINT64;
use shared::minwindef::DWORD;
use um::bits3_0::{IBackgroundCopyFile3, IBackgroundCopyFile3Vtbl};
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{HRESULT, LPWSTR};
RIDL!{#[uuid(0x9a2584c3, 0xf7d2, 0x457a, 0x9a, 0x5e, 0x22, 0xb6, 0x7b, 0xff, 0xc7, 0xd2)]
interface IBitsTokenOptions(IBitsTokenOptionsVtbl): IUnknown(IUnknownVtbl) {
    fn SetHelperTokenFlags(
        UsageFlags: DWORD,
    ) -> HRESULT,
    fn GetHelperTokenFlags(
        pFlags: *mut DWORD,
    ) -> HRESULT,
    fn SetHelperToken() -> HRESULT,
    fn ClearHelperToken() -> HRESULT,
    fn GetHelperTokenSid(
        pSid: *mut LPWSTR,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xef7e0655, 0x7888, 0x4960, 0xb0, 0xe5, 0x73, 0x08, 0x46, 0xe0, 0x34, 0x92)]
interface IBackgroundCopyFile4(IBackgroundCopyFile4Vtbl):
    IBackgroundCopyFile3(IBackgroundCopyFile3Vtbl) {
    fn GetPeerDownloadStats(
        pFromOrigin: PUINT64,
        pFromPeers: PUINT64,
    ) -> HRESULT,
}}
