// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::basetsd::UINT64;
use shared::minwindef::DWORD;
use um::bits::{IBackgroundCopyFile, IBackgroundCopyJob};
use um::bits2_0::BG_FILE_RANGE;
use um::bits3_0::{IBackgroundCopyCallback2, IBackgroundCopyCallback2Vtbl};
use um::bits5_0::{IBackgroundCopyFile5, IBackgroundCopyFile5Vtbl};
use um::winnt::HRESULT;
RIDL!{#[uuid(0x98c97bd2, 0xe32b, 0x4ad8, 0xa5, 0x28, 0x95, 0xfd, 0x8b, 0x16, 0xbd, 0x42)]
interface IBackgroundCopyCallback3(IBackgroundCopyCallback3Vtbl):
    IBackgroundCopyCallback2(IBackgroundCopyCallback2Vtbl) {
    fn FileRangesTransferred(
        job: *mut IBackgroundCopyJob,
        file: *mut IBackgroundCopyFile,
        rangeCount: DWORD,
        ranges: *const BG_FILE_RANGE,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xcf6784f7, 0xd677, 0x49fd, 0x93, 0x68, 0xcb, 0x47, 0xae, 0xe9, 0xd1, 0xad)]
interface IBackgroundCopyFile6(IBackgroundCopyFile6Vtbl):
    IBackgroundCopyFile5(IBackgroundCopyFile5Vtbl) {
    fn UpdateDownloadPosition(
        offset: UINT64,
    ) -> HRESULT,
    fn RequestFileRanges(
        rangeCount: DWORD,
        ranges: *const BG_FILE_RANGE,
    ) -> HRESULT,
    fn GetFilledFileRanges(
        rangeCount: *mut DWORD,
        ranges: *mut *mut BG_FILE_RANGE,
    ) -> HRESULT,
}}
