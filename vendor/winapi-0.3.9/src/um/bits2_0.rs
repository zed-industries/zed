// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::basetsd::UINT64;
use shared::minwindef::DWORD;
use um::bits::{IBackgroundCopyFile, IBackgroundCopyFileVtbl};
use um::bits1_5::{IBackgroundCopyJob2, IBackgroundCopyJob2Vtbl};
use um::winnt::{HRESULT, LPCWSTR};
pub const BG_LENGTH_TO_EOF: UINT64 = -1i64 as u64;
STRUCT!{struct BG_FILE_RANGE {
    InitialOffset: UINT64,
    Length: UINT64,
}}
pub const BG_COPY_FILE_OWNER: DWORD = 1;
pub const BG_COPY_FILE_GROUP: DWORD = 2;
pub const BG_COPY_FILE_DACL: DWORD = 4;
pub const BG_COPY_FILE_SACL: DWORD = 8;
pub const BG_COPY_FILE_ALL: DWORD = 15;
RIDL!{#[uuid(0x443c8934, 0x90ff, 0x48ed, 0xbc, 0xde, 0x26, 0xf5, 0xc7, 0x45, 0x00, 0x42)]
interface IBackgroundCopyJob3(IBackgroundCopyJob3Vtbl):
    IBackgroundCopyJob2(IBackgroundCopyJob2Vtbl) {
    fn ReplaceRemotePrefix(
        OldPrefix: LPCWSTR,
        NewPrefix: LPCWSTR,
    ) -> HRESULT,
    fn AddFileWithRanges(
        RemoteUrl: LPCWSTR,
        LocalName: LPCWSTR,
        RangeCount: DWORD,
        Ranges: *mut BG_FILE_RANGE,
    ) -> HRESULT,
    fn SetFileACLFlags(
        Flags: DWORD,
    ) -> HRESULT,
    fn GetFileACLFlags(
        Flags: *mut DWORD,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x83e81b93, 0x0873, 0x474d, 0x8a, 0x8c, 0xf2, 0x01, 0x8b, 0x1a, 0x93, 0x9c)]
interface IBackgroundCopyFile2(IBackgroundCopyFile2Vtbl):
    IBackgroundCopyFile(IBackgroundCopyFileVtbl) {
    fn GetFileRanges(
        RangeCount: *mut DWORD,
        Ranges: *mut *mut BG_FILE_RANGE,
    ) -> HRESULT,
    fn SetRemoteName(
        Val: LPCWSTR,
    ) -> HRESULT,
}}
