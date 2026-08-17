// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::basetsd::UINT64;
use shared::guiddef::{GUID, REFGUID};
use shared::minwindef::{BOOL, DWORD, FILETIME, ULONG};
use um::bits::{
    IBackgroundCopyCallback, IBackgroundCopyCallbackVtbl, IBackgroundCopyFile, IBackgroundCopyJob,
};
use um::bits2_0::{
    BG_FILE_RANGE, IBackgroundCopyFile2, IBackgroundCopyFile2Vtbl, IBackgroundCopyJob3,
    IBackgroundCopyJob3Vtbl,
};
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{HRESULT, LPCWSTR, LPWSTR};
RIDL!{#[uuid(0x659cdeaf, 0x489e, 0x11d9, 0xa9, 0xcd, 0x00, 0x0d, 0x56, 0x96, 0x52, 0x51)]
interface IBitsPeerCacheRecord(IBitsPeerCacheRecordVtbl): IUnknown(IUnknownVtbl) {
    fn GetId(
        pVal: *mut GUID,
    ) -> HRESULT,
    fn GetOriginUrl(
        pVal: *mut LPWSTR,
    ) -> HRESULT,
    fn GetFileSize(
        pVal: *mut UINT64,
    ) -> HRESULT,
    fn GetFileModificationTime(
        pVal: *mut FILETIME,
    ) -> HRESULT,
    fn GetLastAccessTime(
        pVal: *mut FILETIME,
    ) -> HRESULT,
    fn IsFileValidated() -> HRESULT,
    fn GetFileRanges(
        pRangeCount: *mut DWORD,
        ppRanges: *mut *mut BG_FILE_RANGE,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x659cdea4, 0x489e, 0x11d9, 0xa9, 0xcd, 0x00, 0x0d, 0x56, 0x96, 0x52, 0x51)]
interface IEnumBitsPeerCacheRecords(IEnumBitsPeerCacheRecordsVtbl): IUnknown(IUnknownVtbl) {
    fn Next(
        celt: ULONG,
        rgelt: *mut *mut IBitsPeerCacheRecord,
        pceltFetched: *mut ULONG,
    ) -> HRESULT,
    fn Skip(
        celt: ULONG,
    ) -> HRESULT,
    fn Reset() -> HRESULT,
    fn Clone(
        ppenum: *mut *mut IEnumBitsPeerCacheRecords,
    ) -> HRESULT,
    fn GetCount(
        puCount: *mut ULONG,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x659cdea2, 0x489e, 0x11d9, 0xa9, 0xcd, 0x00, 0x0d, 0x56, 0x96, 0x52, 0x51)]
interface IBitsPeer(IBitsPeerVtbl): IUnknown(IUnknownVtbl) {
    fn GetPeerName(
        pName: *mut LPWSTR,
    ) -> HRESULT,
    fn IsAuthenticated(
        pAuth: *mut BOOL,
    ) -> HRESULT,
    fn IsAvailable(
        pOnline: *mut BOOL,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x659cdea5, 0x489e, 0x11d9, 0xa9, 0xcd, 0x00, 0x0d, 0x56, 0x96, 0x52, 0x51)]
interface IEnumBitsPeers(IEnumBitsPeersVtbl): IUnknown(IUnknownVtbl) {
    fn Next(
        celt: ULONG,
        rgelt: *mut *mut IBitsPeer,
        pceltFetched: *mut ULONG,
    ) -> HRESULT,
    fn Skip(
        celt: ULONG,
    ) -> HRESULT,
    fn Reset() -> HRESULT,
    fn Clone(
        ppenum: *mut *mut IEnumBitsPeers,
    ) -> HRESULT,
    fn GetCount(
        puCount: *mut ULONG,
    ) -> HRESULT,
}}
pub const BG_ENABLE_PEERCACHING_CLIENT: DWORD = 0x0001;
pub const BG_ENABLE_PEERCACHING_SERVER: DWORD = 0x0002;
pub const BG_DISABLE_BRANCH_CACHE: DWORD = 0x0004;
RIDL!{#[uuid(0x659cdead, 0x489e, 0x11d9, 0xa9, 0xcd, 0x00, 0x0d, 0x56, 0x96, 0x52, 0x51)]
interface IBitsPeerCacheAdministration(IBitsPeerCacheAdministrationVtbl): IUnknown(IUnknownVtbl) {
    fn GetMaximumCacheSize(
        pBytes: *mut DWORD,
    ) -> HRESULT,
    fn SetMaximumCacheSize(
        Bytes: DWORD,
    ) -> HRESULT,
    fn GetMaximumContentAge(
        pSeconds: *mut ULONG,
    ) -> HRESULT,
    fn SetMaximumContentAge(
        Seconds: ULONG,
    ) -> HRESULT,
    fn GetConfigurationFlags(
        pFlags: *mut DWORD,
    ) -> HRESULT,
    fn SetConfigurationFlags(
        Flags: DWORD,
    ) -> HRESULT,
    fn EnumRecords(
        ppEnum: *mut *mut IEnumBitsPeerCacheRecords,
    ) -> HRESULT,
    fn GetRecord(
        ppRecord: *mut *mut IBitsPeerCacheRecord,
    ) -> HRESULT,
    fn ClearRecords() -> HRESULT,
    fn DeleteRecord(
        id: REFGUID,
    ) -> HRESULT,
    fn DeleteUrl(
        url: LPCWSTR,
    ) -> HRESULT,
    fn EnumPeers(
        ppEnum: *mut *mut IEnumBitsPeers,
    ) -> HRESULT,
    fn ClearPeers() -> HRESULT,
    fn DiscoverPeers() -> HRESULT,
}}
pub const BG_JOB_ENABLE_PEERCACHING_CLIENT: DWORD = 0x0001;
pub const BG_JOB_ENABLE_PEERCACHING_SERVER: DWORD = 0x0002;
pub const BG_JOB_DISABLE_BRANCH_CACHE: DWORD = 0x0004;
RIDL!{#[uuid(0x659cdeae, 0x489e, 0x11d9, 0xa9, 0xcd, 0x00, 0x0d, 0x56, 0x96, 0x52, 0x51)]
interface IBackgroundCopyJob4(IBackgroundCopyJob4Vtbl):
    IBackgroundCopyJob3(IBackgroundCopyJob3Vtbl) {
    fn SetPeerCachingFlags(
        Flags: DWORD,
    ) -> HRESULT,
    fn GetPeerCachingFlags(
        pFlags: *mut DWORD,
    ) -> HRESULT,
    fn GetOwnerIntegrityLevel(
        pLevel: *mut ULONG,
    ) -> HRESULT,
    fn GetOwnerElevationState(
        pElevated: *mut BOOL,
    ) -> HRESULT,
    fn SetMaximumDownloadTime(
        Timeout: ULONG,
    ) -> HRESULT,
    fn GetMaximumDownloadTime(
        pTimeout: *mut ULONG,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x659cdeaa, 0x489e, 0x11d9, 0xa9, 0xcd, 0x00, 0x0d, 0x56, 0x96, 0x52, 0x51)]
interface IBackgroundCopyFile3(IBackgroundCopyFile3Vtbl):
    IBackgroundCopyFile2(IBackgroundCopyFile2Vtbl) {
    fn GetTemporaryName(
        pFilename: *mut LPWSTR,
    ) -> HRESULT,
    fn SetValidationState(
        state: BOOL,
    ) -> HRESULT,
    fn GetValidationState(
        pState: *mut BOOL,
    ) -> HRESULT,
    fn IsDownloadedFromPeer(
        pVal: *mut BOOL,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x659cdeac, 0x489e, 0x11d9, 0xa9, 0xcd, 0x00, 0x0d, 0x56, 0x96, 0x52, 0x51)]
interface IBackgroundCopyCallback2(IBackgroundCopyCallback2Vtbl):
    IBackgroundCopyCallback(IBackgroundCopyCallbackVtbl) {
    fn FileTransferred(
        pJob: *mut IBackgroundCopyJob,
        pFile: *mut IBackgroundCopyFile,
    ) -> HRESULT,
}}
