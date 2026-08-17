// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms
use shared::lmcons::NET_API_STATUS;
use shared::minwindef::{DWORD, LPBYTE};
use um::winnt::{LARGE_INTEGER, LPCWSTR};
extern "system" {
    pub fn NetStatisticsGet(
        ServerName: LPCWSTR,
        Service: LPCWSTR,
        Level: DWORD,
        Options: DWORD,
        Buffer: *mut LPBYTE,
    ) -> NET_API_STATUS;
}
STRUCT!{struct STAT_WORKSTATION_0 {
    StatisticsStartTime: LARGE_INTEGER,
    BytesReceived: LARGE_INTEGER,
    SmbsReceived: LARGE_INTEGER,
    PagingReadBytesRequested: LARGE_INTEGER,
    NonPagingReadBytesRequested: LARGE_INTEGER,
    CacheReadBytesRequested: LARGE_INTEGER,
    NetworkReadBytesRequested: LARGE_INTEGER,
    BytesTransmitted: LARGE_INTEGER,
    SmbsTransmitted: LARGE_INTEGER,
    PagingWriteBytesRequested: LARGE_INTEGER,
    NonPagingWriteBytesRequested: LARGE_INTEGER,
    CacheWriteBytesRequested: LARGE_INTEGER,
    NetworkWriteBytesRequested: LARGE_INTEGER,
    InitiallyFailedOperations: DWORD,
    FailedCompletionOperations: DWORD,
    ReadOperations: DWORD,
    RandomReadOperations: DWORD,
    ReadSmbs: DWORD,
    LargeReadSmbs: DWORD,
    SmallReadSmbs: DWORD,
    WriteOperations: DWORD,
    RandomWriteOperations: DWORD,
    WriteSmbs: DWORD,
    LargeWriteSmbs: DWORD,
    SmallWriteSmbs: DWORD,
    RawReadsDenied: DWORD,
    RawWritesDenied: DWORD,
    NetworkErrors: DWORD,
    Sessions: DWORD,
    FailedSessions: DWORD,
    Reconnects: DWORD,
    CoreConnects: DWORD,
    Lanman20Connects: DWORD,
    Lanman21Connects: DWORD,
    LanmanNtConnects: DWORD,
    ServerDisconnects: DWORD,
    HungSessions: DWORD,
    UseCount: DWORD,
    FailedUseCount: DWORD,
    CurrentCommands: DWORD,
}}
pub type PSTAT_WORKSTATION_0 = *mut STAT_WORKSTATION_0;
pub type LPSTAT_WORKSTATION_0 = *mut STAT_WORKSTATION_0;
STRUCT!{struct STAT_SERVER_0 {
    sts0_start: DWORD,
    sts0_fopens: DWORD,
    sts0_devopens: DWORD,
    sts0_jobsqueued: DWORD,
    sts0_sopens: DWORD,
    sts0_stimedout: DWORD,
    sts0_serrorout: DWORD,
    sts0_pwerrors: DWORD,
    sts0_permerrors: DWORD,
    sts0_syserrors: DWORD,
    sts0_bytessent_low: DWORD,
    sts0_bytessent_high: DWORD,
    sts0_bytesrcvd_low: DWORD,
    sts0_bytesrcvd_high: DWORD,
    sts0_avresponse: DWORD,
    sts0_reqbufneed: DWORD,
    sts0_bigbufneed: DWORD,
}}
pub type PSTAT_SERVER_0 = *mut STAT_SERVER_0;
pub type LPSTAT_SERVER_0 = *mut STAT_SERVER_0;
pub const STATSOPT_CLR: DWORD = 1;
pub const STATS_NO_VALUE: DWORD = -1i32 as u32;
pub const STATS_OVERFLOW: DWORD = -2i32 as u32;
