// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Common Performance Data Helper definitions
use ctypes::c_double;
use shared::basetsd::DWORD_PTR;
use shared::guiddef::GUID;
use shared::minwindef::{BOOL, DWORD, FILETIME, LPDWORD, UCHAR};
use shared::windef::HWND;
use um::winnt::{BOOLEAN, HANDLE, LONG, LONGLONG, LPCSTR, LPCWSTR, LPSTR, LPWSTR};
pub const PDH_FMT_RAW: DWORD = 0x00000010;
pub const PDH_FMT_ANSI: DWORD = 0x00000020;
pub const PDH_FMT_UNICODE: DWORD = 0x00000040;
pub const PDH_FMT_LONG: DWORD = 0x00000100;
pub const PDH_FMT_DOUBLE: DWORD = 0x00000200;
pub const PDH_FMT_LARGE: DWORD = 0x00000400;
pub const PDH_FMT_NOSCALE: DWORD = 0x00001000;
pub const PDH_FMT_1000: DWORD = 0x00002000;
pub const PDH_FMT_NODATA: DWORD = 0x00004000;
pub const PDH_FMT_NOCAP100: DWORD = 0x00008000;
pub const PERF_DETAIL_COSTLY: DWORD = 0x00010000;
pub const PERF_DETAIL_STANDARD: DWORD = 0x0000FFFF;
pub type PDH_STATUS = LONG;
pub type PDH_HQUERY = HANDLE;
pub type HQUERY = PDH_HQUERY;
pub type PDH_HCOUNTER = HANDLE;
pub type HCOUNTER = PDH_HCOUNTER;
pub type PPDH_BROWSE_DLG_CONFIG_A = *mut PDH_BROWSE_DLG_CONFIG_A;
pub type PPDH_BROWSE_DLG_CONFIG_W = *mut PDH_BROWSE_DLG_CONFIG_W;
pub type PDH_HLOG = HANDLE;
pub type PPDH_RAW_LOG_RECORD = *mut PDH_RAW_LOG_RECORD;
pub type PPDH_TIME_INFO = *mut PDH_TIME_INFO;
pub type PPDH_RAW_COUNTER = *mut PDH_RAW_COUNTER;
pub type PPDH_COUNTER_INFO_A = *mut PDH_COUNTER_INFO_A;
pub type PPDH_COUNTER_INFO_W = *mut PDH_COUNTER_INFO_W;
pub type PPDH_STATISTICS = *mut PDH_STATISTICS;
pub type PPDH_FMT_COUNTERVALUE_ITEM_A = *mut PDH_FMT_COUNTERVALUE_ITEM_A;
pub type PPDH_FMT_COUNTERVALUE_ITEM_W = *mut PDH_FMT_COUNTERVALUE_ITEM_W;
pub type PPDH_DATA_ITEM_PATH_ELEMENTS_A = *mut PDH_DATA_ITEM_PATH_ELEMENTS_A;
pub type PPDH_DATA_ITEM_PATH_ELEMENTS_W = *mut PDH_DATA_ITEM_PATH_ELEMENTS_W;
pub type PPDH_FMT_COUNTERVALUE = *mut PDH_FMT_COUNTERVALUE;
FN!{stdcall CounterPathCallBack(
    DWORD_PTR,
) -> PDH_STATUS}
pub type PPDH_COUNTER_PATH_ELEMENTS_A = *mut PDH_COUNTER_PATH_ELEMENTS_A;
pub type PPDH_COUNTER_PATH_ELEMENTS_W = *mut PDH_COUNTER_PATH_ELEMENTS_W;
pub type PPDH_BROWSE_DLG_CONFIG_HA = *mut PDH_BROWSE_DLG_CONFIG_HA;
pub type PPDH_BROWSE_DLG_CONFIG_HW = *mut PDH_BROWSE_DLG_CONFIG_HW;
UNION!{union PDH_FMT_COUNTERVALUE_u {
    [u64; 1],
    longValue longValue_mut: LONG,
    doubleValue doubleValue_mut: c_double,
    largeValue largeValue_mut: LONGLONG,
    AnsiStringValue AnsiStringValue_mut: LPCSTR,
    WideStringValue WideStringValue_mut: LPCWSTR,
}}
STRUCT!{struct PDH_FMT_COUNTERVALUE {
    CStatus: DWORD,
    u: PDH_FMT_COUNTERVALUE_u,
}}
STRUCT!{struct PDH_RAW_LOG_RECORD {
    dwStructureSize: DWORD,
    dwRecordType: DWORD,
    dwItems: DWORD,
    RawBytes: UCHAR,
}}
STRUCT!{struct PDH_TIME_INFO {
    StartTime: LONGLONG,
    EndTime: LONGLONG,
    SampleCount: DWORD,
}}
STRUCT!{struct PDH_RAW_COUNTER {
    CStatus: DWORD,
    TimeStamp: FILETIME,
    FirstValue: LONGLONG,
    SecondValue: LONGLONG,
    MultiCount: DWORD,
}}
STRUCT!{struct PDH_STATISTICS {
    dwFormat: DWORD,
    count: DWORD,
    min: PDH_FMT_COUNTERVALUE,
    max: PDH_FMT_COUNTERVALUE,
    mean: PDH_FMT_COUNTERVALUE,
}}
STRUCT!{struct PDH_FMT_COUNTERVALUE_ITEM_A {
    szName: LPSTR,
    FmtValue: PDH_FMT_COUNTERVALUE,
}}
STRUCT!{struct PDH_FMT_COUNTERVALUE_ITEM_W {
    szName: LPWSTR,
    FmtValue: PDH_FMT_COUNTERVALUE,
}}
STRUCT!{struct PDH_BROWSE_DLG_CONFIG_A {
    flags: DWORD,
    hWndOwner: HWND,
    szDataSource: LPSTR,
    szReturnPathBuffer: LPSTR,
    cchReturnPathLength: DWORD,
    pCallBack: CounterPathCallBack,
    dwCallBackArg: DWORD_PTR,
    CallBackStatus: PDH_STATUS,
    dwDefaultDetailLevel: DWORD,
    szDialogBoxCaption: LPSTR,
}}
BITFIELD!{PDH_BROWSE_DLG_CONFIG_A flags: DWORD [
    IncludeInstanceIndex set_IncludeInstanceIndex[0..1],
    SingleCounterPerAdd set_SingleCounterPerAdd[1..2],
    SingleCounterPerDialog set_SingleCounterPerDialog[2..3],
    LocalCountersOnly set_LocalCountersOnly[3..4],
    WildCardInstances set_WildCardInstances[4..5],
    HideDetailBox set_HideDetailBox[5..6],
    InitializePath set_InitializePath[6..7],
    DisableMachineSelection set_DisableMachineSelection[7..8],
    IncludeCostlyObjects set_IncludeCostlyObjects[8..9],
    ShowObjectBrowser set_ShowObjectBrowser[9..10],
]}
STRUCT!{struct PDH_BROWSE_DLG_CONFIG_W {
    flags: DWORD,
    hWndOwner: HWND,
    szDataSource: LPWSTR,
    szReturnPathBuffer: LPWSTR,
    cchReturnPathLength: DWORD,
    pCallBack: CounterPathCallBack,
    dwCallBackArg: DWORD_PTR,
    CallBackStatus: PDH_STATUS,
    dwDefaultDetailLevel: DWORD,
    szDialogBoxCaption: LPWSTR,
}}
BITFIELD!{PDH_BROWSE_DLG_CONFIG_W flags: DWORD [
    IncludeInstanceIndex set_IncludeInstanceIndex[0..1],
    SingleCounterPerAdd set_SingleCounterPerAdd[1..2],
    SingleCounterPerDialog set_SingleCounterPerDialog[2..3],
    LocalCountersOnly set_LocalCountersOnly[3..4],
    WildCardInstances set_WildCardInstances[4..5],
    HideDetailBox set_HideDetailBox[5..6],
    InitializePath set_InitializePath[6..7],
    DisableMachineSelection set_DisableMachineSelection[7..8],
    IncludeCostlyObjects set_IncludeCostlyObjects[8..9],
    ShowObjectBrowser set_ShowObjectBrowser[9..10],
]}
STRUCT!{struct PDH_COUNTER_PATH_ELEMENTS_A {
    szMachineName: LPSTR,
    szObjectName: LPSTR,
    szInstanceName: LPSTR,
    szParentInstance: LPSTR,
    dwInstanceIndex: DWORD,
    szCounterName: LPSTR,
}}
STRUCT!{struct PDH_COUNTER_PATH_ELEMENTS_W {
    szMachineName: LPWSTR,
    szObjectName: LPWSTR,
    szInstanceName: LPWSTR,
    szParentInstance: LPWSTR,
    dwInstanceIndex: DWORD,
    szCounterName: LPWSTR,
}}
STRUCT!{struct PDH_DATA_ITEM_PATH_ELEMENTS_A {
    szMachineName: LPSTR,
    ObjectGUID: GUID,
    dwItemId: DWORD,
    szInstanceName: LPSTR,
}}
STRUCT!{struct PDH_DATA_ITEM_PATH_ELEMENTS_W {
    szMachineName: LPWSTR,
    ObjectGUID: GUID,
    dwItemId: DWORD,
    szInstanceName: LPWSTR,
}}
STRUCT!{struct PDH_COUNTER_INFO_A_u_s {
    szMachineName: LPSTR,
    szObjectName: LPSTR,
    szInstanceName: LPSTR,
    szParentInstance: LPSTR,
    dwInstanceIndex: DWORD,
    szCounterName: LPSTR,
}}
UNION!{union PDH_COUNTER_INFO_A_u {
    [u32; 7] [u64; 6],
    DataItemPath DataItemPath_mut: PDH_DATA_ITEM_PATH_ELEMENTS_A,
    CounterPath CounterPath_mut: PDH_COUNTER_PATH_ELEMENTS_A,
    s s_mut: PDH_COUNTER_INFO_A_u_s,
}}
STRUCT!{struct PDH_COUNTER_INFO_A {
    dwLength: DWORD,
    dwType: DWORD,
    CVersion: DWORD,
    CStatus: DWORD,
    lScale: LONG,
    lDefaultScale: LONG,
    dwUserData: DWORD_PTR,
    dwQueryUserData: DWORD_PTR,
    szFullPath: LPSTR,
    u: PDH_COUNTER_INFO_A_u,
    szExplainText: LPSTR,
    DataBuffer: [DWORD; 1],
}}
STRUCT!{struct PDH_COUNTER_INFO_W_u_s {
    szMachineName: LPWSTR,
    szObjectName: LPWSTR,
    szInstanceName: LPWSTR,
    szParentInstance: LPWSTR,
    dwInstanceIndex: DWORD,
    szCounterName: LPWSTR,
}}
UNION!{union PDH_COUNTER_INFO_W_u {
    [u32; 7] [u64; 6],
    DataItemPath DataItemPath_mut: PDH_DATA_ITEM_PATH_ELEMENTS_W,
    CounterPath CounterPath_mut: PDH_COUNTER_PATH_ELEMENTS_W,
    s s_mut: PDH_COUNTER_INFO_W_u_s,
}}
STRUCT!{struct PDH_COUNTER_INFO_W {
    dwLength: DWORD,
    dwType: DWORD,
    CVersion: DWORD,
    CStatus: DWORD,
    lScale: LONG,
    lDefaultScale: LONG,
    dwUserData: DWORD_PTR,
    dwQueryUserData: DWORD_PTR,
    szFullPath: LPWSTR,
    u: PDH_COUNTER_INFO_W_u,
    szExplainText: LPWSTR,
    DataBuffer: [DWORD; 1],
}}
STRUCT!{struct PDH_BROWSE_DLG_CONFIG_HA {
    flags: DWORD,
    hWndOwner: HWND,
    hDataSource: PDH_HLOG,
    szReturnPathBuffer: LPSTR,
    cchReturnPathLength: DWORD,
    pCallBack: CounterPathCallBack,
    dwCallBackArg: DWORD_PTR,
    CallBackStatus: PDH_STATUS,
    dwDefaultDetailLevel: DWORD,
    szDialogBoxCaption: LPSTR,
}}
BITFIELD!{PDH_BROWSE_DLG_CONFIG_HA flags: DWORD [
    IncludeInstanceIndex set_IncludeInstanceIndex[0..1],
    SingleCounterPerAdd set_SingleCounterPerAdd[1..2],
    SingleCounterPerDialog set_SingleCounterPerDialog[2..3],
    LocalCountersOnly set_LocalCountersOnly[3..4],
    WildCardInstances set_WildCardInstances[4..5],
    HideDetailBox set_HideDetailBox[5..6],
    InitializePath set_InitializePath[6..7],
    DisableMachineSelection set_DisableMachineSelection[7..8],
    IncludeCostlyObjects set_IncludeCostlyObjects[8..9],
    ShowObjectBrowser set_ShowObjectBrowser[9..10],
]}
STRUCT!{struct PDH_BROWSE_DLG_CONFIG_HW {
    flags: DWORD,
    hWndOwner: HWND,
    hDataSource: PDH_HLOG,
    szReturnPathBuffer: LPWSTR,
    cchReturnPathLength: DWORD,
    pCallBack: CounterPathCallBack,
    dwCallBackArg: DWORD_PTR,
    CallBackStatus: PDH_STATUS,
    dwDefaultDetailLevel: DWORD,
    szDialogBoxCaption: LPWSTR,
}}
BITFIELD!{PDH_BROWSE_DLG_CONFIG_HW flags: DWORD [
    IncludeInstanceIndex set_IncludeInstanceIndex[0..1],
    SingleCounterPerAdd set_SingleCounterPerAdd[1..2],
    SingleCounterPerDialog set_SingleCounterPerDialog[2..3],
    LocalCountersOnly set_LocalCountersOnly[3..4],
    WildCardInstances set_WildCardInstances[4..5],
    HideDetailBox set_HideDetailBox[5..6],
    InitializePath set_InitializePath[6..7],
    DisableMachineSelection set_DisableMachineSelection[7..8],
    IncludeCostlyObjects set_IncludeCostlyObjects[8..9],
    ShowObjectBrowser set_ShowObjectBrowser[9..10],
]}
extern "system" {
    pub fn PdhGetDllVersion(
        lpdwVersion: LPDWORD,
    ) -> PDH_STATUS;
    pub fn PdhOpenQueryW(
        szDataSource: LPCWSTR,
        dwUserData: DWORD_PTR,
        phQuery: *mut PDH_HQUERY,
    ) -> PDH_STATUS;
    pub fn PdhOpenQueryA(
        szDataSource: LPCSTR,
        dwUserData: DWORD_PTR,
        phQuery: *mut PDH_HQUERY,
    ) -> PDH_STATUS;
    pub fn PdhAddCounterW(
        hQuery: PDH_HQUERY,
        szFullCounterPath: LPCWSTR,
        dwUserData: DWORD_PTR,
        phCounter: *mut PDH_HCOUNTER,
    ) -> PDH_STATUS;
    pub fn PdhAddCounterA(
        hQuery: PDH_HQUERY,
        szFullCounterPath: LPCSTR,
        dwUserData: DWORD_PTR,
        phCounter: *mut PDH_HCOUNTER,
    ) -> PDH_STATUS;
    pub fn PdhAddEnglishCounterW(
        hQuery: PDH_HQUERY,
        szFullCounterPath: LPCWSTR,
        dwUserData: DWORD_PTR,
        phCounter: *mut PDH_HCOUNTER,
    ) -> PDH_STATUS;
    pub fn PdhAddEnglishCounterA(
        hQuery: PDH_HQUERY,
        szFullCounterPath: LPCSTR,
        dwUserData: DWORD_PTR,
        phCounter: *mut PDH_HCOUNTER,
    ) -> PDH_STATUS;
    pub fn PdhCollectQueryDataWithTime(
        hQuery: PDH_HQUERY,
        pllTimeStamp: *mut LONGLONG,
    ) -> PDH_STATUS;
    pub fn PdhValidatePathExW(
        hDataSource: PDH_HLOG,
        szFullPathBuffer: LPCWSTR,
    ) -> PDH_STATUS;
    pub fn PdhValidatePathExA(
        hDataSource: PDH_HLOG,
        szFullPathBuffer: LPCSTR,
    ) -> PDH_STATUS;
    pub fn PdhRemoveCounter(
        hCounter: PDH_HCOUNTER,
    ) -> PDH_STATUS;
    pub fn PdhCollectQueryData(
        hQuery: PDH_HQUERY,
    ) -> PDH_STATUS;
    pub fn PdhCloseQuery(
        hQuery: PDH_HQUERY,
    ) -> PDH_STATUS;
    pub fn PdhGetFormattedCounterValue(
        hCounter: PDH_HCOUNTER,
        dwFormat: DWORD,
        lpdwType: LPDWORD,
        pValue: PPDH_FMT_COUNTERVALUE,
    ) -> PDH_STATUS;
    pub fn PdhGetFormattedCounterArrayA(
        hCounter: PDH_HCOUNTER,
        dwFormat: DWORD,
        lpdwBufferSize: LPDWORD,
        lpdwBufferCount: LPDWORD,
        ItemBuffer: PPDH_FMT_COUNTERVALUE_ITEM_A,
    ) -> PDH_STATUS;
    pub fn PdhGetFormattedCounterArrayW(
        hCounter: PDH_HCOUNTER,
        dwFormat: DWORD,
        lpdwBufferSize: LPDWORD,
        lpdwBufferCount: LPDWORD,
        ItemBuffer: PPDH_FMT_COUNTERVALUE_ITEM_W,
    ) -> PDH_STATUS;
    pub fn PdhGetRawCounterValue(
        hCounter: PDH_HCOUNTER,
        lpdwType: LPDWORD,
        pValue: PPDH_RAW_COUNTER,
    ) -> PDH_STATUS;
    pub fn PdhGetRawCounterArrayA(
        hCounter: PDH_HCOUNTER,
        dwFormat: DWORD,
        lpdwBufferSize: LPDWORD,
        lpdwBufferCount: LPDWORD,
        ItemBuffer: PPDH_FMT_COUNTERVALUE_ITEM_A,
    ) -> PDH_STATUS;
    pub fn PdhGetRawCounterArrayW(
        hCounter: PDH_HCOUNTER,
        dwFormat: DWORD,
        lpdwBufferSize: LPDWORD,
        lpdwBufferCount: LPDWORD,
        ItemBuffer: PPDH_FMT_COUNTERVALUE_ITEM_W,
    ) -> PDH_STATUS;
    pub fn PdhCalculateCounterFromRawValue(
        hCounter: PDH_HCOUNTER,
        dwFormat: DWORD,
        rawValue1: PPDH_RAW_COUNTER,
        rawValue2: PPDH_RAW_COUNTER,
        fmtValue: PPDH_FMT_COUNTERVALUE,
    ) -> PDH_STATUS;
    pub fn PdhComputeCounterStatistics(
        hCounter: PDH_HCOUNTER,
        dwFormat: DWORD,
        dwFirstEntry: DWORD,
        dwNumEntries: DWORD,
        lpRawValueArray: PPDH_RAW_COUNTER,
        data: PPDH_STATISTICS,
    ) -> PDH_STATUS;
    pub fn PdhGetCounterInfoW(
        hCounter: PDH_HCOUNTER,
        bRetrieveExplainText: BOOLEAN,
        pdwBufferSize: LPDWORD,
        lpBuffer: PPDH_COUNTER_INFO_W,
    ) -> PDH_STATUS;
    pub fn PdhGetCounterInfoA(
        hCounter: PDH_HCOUNTER,
        bRetrieveExplainText: BOOLEAN,
        pdwBufferSize: LPDWORD,
        lpBuffer: PPDH_COUNTER_INFO_A,
    ) -> PDH_STATUS;
    pub fn PdhSetCounterScaleFactor(
        hCounter: PDH_HCOUNTER,
        lFactor: LONG,
    ) -> PDH_STATUS;
    pub fn PdhConnectMachineW(
        szMachineName: LPCWSTR,
    ) -> PDH_STATUS;
    pub fn PdhConnectMachineA(
        szMachineName: LPCSTR,
    ) -> PDH_STATUS;
    pub fn PdhEnumMachinesW(
        szDataSource: LPCWSTR,
        mszMachineNameList: LPWSTR,
        pcchBufferLength: LPDWORD,
    ) -> PDH_STATUS;
    pub fn PdhEnumMachinesA(
        szDataSource: LPCSTR,
        mszMachineNameList: LPSTR,
        pcchBufferLength: LPDWORD,
    ) -> PDH_STATUS;
    pub fn PdhEnumObjectsW(
        szDataSource: LPCWSTR,
        szMachineName: LPCWSTR,
        mszObjectList: LPWSTR,
        pcchBufferLength: LPDWORD,
        dwDetailLevel: DWORD,
        bRefresh: BOOL,
    ) -> PDH_STATUS;
    pub fn PdhEnumObjectsA(
        szDataSource: LPCSTR,
        szMachineName: LPCSTR,
        mszObjectList: LPSTR,
        pcchBufferLength: LPDWORD,
        dwDetailLevel: DWORD,
        bRefresh: BOOL,
    ) -> PDH_STATUS;
    pub fn PdhEnumObjectItemsW(
        szDataSource: LPCWSTR,
        szMachineName: LPCWSTR,
        szObjectName: LPCWSTR,
        mszCounterList: LPWSTR,
        pcchCounterListLength: LPDWORD,
        mszInstanceList: LPWSTR,
        pcchInstanceListLength: LPDWORD,
        dwDetailLevel: DWORD,
        dwFlags: DWORD,
    ) -> PDH_STATUS;
    pub fn PdhEnumObjectItemsA(
        szDataSource: LPCSTR,
        szMachineName: LPCSTR,
        szObjectName: LPCSTR,
        mszCounterList: LPSTR,
        pcchCounterListLength: LPDWORD,
        mszInstanceList: LPSTR,
        pcchInstanceListLength: LPDWORD,
        dwDetailLevel: DWORD,
        dwFlags: DWORD,
    ) -> PDH_STATUS;
    pub fn PdhMakeCounterPathW(
        pCounterPathElements: PPDH_COUNTER_PATH_ELEMENTS_W,
        szFullPathBuffer: LPWSTR,
        pcchBufferSize: LPDWORD,
        dwFlags: DWORD,
    ) -> PDH_STATUS;
    pub fn PdhMakeCounterPathA(
        pCounterPathElements: PPDH_COUNTER_PATH_ELEMENTS_A,
        szFullPathBuffer: LPSTR,
        pcchBufferSize: LPDWORD,
        dwFlags: DWORD,
    ) -> PDH_STATUS;
    pub fn PdhParseCounterPathW(
        szFullPathBuffer: LPCWSTR,
        pCounterPathElements: *mut PDH_COUNTER_PATH_ELEMENTS_W,
        pcchBufferSize: LPDWORD,
        dwFlags: DWORD,
    ) -> PDH_STATUS;
    pub fn PdhParseCounterPathA(
        szFullPathBuffer: LPCSTR,
        pCounterPathElements: *mut PDH_COUNTER_PATH_ELEMENTS_A,
        pcchBufferSize: LPDWORD,
        dwFlags: DWORD,
    ) -> PDH_STATUS;
    pub fn PdhParseInstanceNameW(
        szInstanceString: LPCWSTR,
        szInstanceName: LPWSTR,
        pcchInstanceNameLength: LPDWORD,
        szParentName: LPWSTR,
        pcchParentNameLength: LPDWORD,
        lpIndex: LPDWORD,
    ) -> PDH_STATUS;
    pub fn PdhParseInstanceNameA(
        szInstanceString: LPCSTR,
        szInstanceName: LPSTR,
        pcchInstanceNameLength: LPDWORD,
        szParentName: LPSTR,
        pcchParentNameLength: LPDWORD,
        lpIndex: LPDWORD,
    ) -> PDH_STATUS;
    pub fn PdhValidatePathW(
        szFullCounterPath: LPCWSTR,
    ) -> PDH_STATUS;
    pub fn PdhValidatePathA(
        szFullCounterPath: LPCSTR,
    ) -> PDH_STATUS;
    pub fn PdhGetDefaultPerfObjectW(
        szDataSource: LPCWSTR,
        szMachineName: LPCWSTR,
        szDefaultObjectName: LPWSTR,
        pcchBufferSize: LPDWORD,
    ) -> PDH_STATUS;
    pub fn PdhGetDefaultPerfObjectA(
        szDataSource: LPCSTR,
        szMachineName: LPCSTR,
        szDefaultObjectName: LPSTR,
        pcchBufferSize: LPDWORD,
    ) -> PDH_STATUS;
    pub fn PdhGetDefaultPerfCounterW(
        szDataSource: LPCWSTR,
        szMachineName: LPCWSTR,
        szObjectName: LPCWSTR,
        szDefaultCounterName: LPWSTR,
        pcchBufferSize: LPDWORD,
    ) -> PDH_STATUS;
    pub fn PdhGetDefaultPerfCounterA(
        szDataSource: LPCSTR,
        szMachineName: LPCSTR,
        szObjectName: LPCSTR,
        szDefaultCounterName: LPSTR,
        pcchBufferSize: LPDWORD,
    ) -> PDH_STATUS;
    pub fn PdhBrowseCountersW(
        pBrowseDlgData: PPDH_BROWSE_DLG_CONFIG_W,
    ) -> PDH_STATUS;
    pub fn PdhBrowseCountersA(
        pBrowseDlgData: PPDH_BROWSE_DLG_CONFIG_A,
    ) -> PDH_STATUS;
    pub fn PdhExpandCounterPathW(
        szWildCardPath: LPCWSTR,
        mszExpandedPathList: LPWSTR,
        pcchPathListLength: LPDWORD,
    ) -> PDH_STATUS;
    pub fn PdhExpandCounterPathA(
        szWildCardPath: LPCSTR,
        mszExpandedPathList: LPSTR,
        pcchPathListLength: LPDWORD,
    ) -> PDH_STATUS;
    pub fn PdhLookupPerfNameByIndexW(
        szMachineName: LPCWSTR,
        dwNameIndex: DWORD,
        szNameBuffer: LPWSTR,
        pcchNameBufferSize: LPDWORD,
    ) -> PDH_STATUS;
    pub fn PdhLookupPerfNameByIndexA(
        szMachineName: LPCSTR,
        dwNameIndex: DWORD,
        szNameBuffer: LPSTR,
        pcchNameBufferSize: LPDWORD,
    ) -> PDH_STATUS;
    pub fn PdhLookupPerfIndexByNameW(
        szMachineName: LPCWSTR,
        szNameBuffer: LPCWSTR,
        pdwIndex: LPDWORD,
    ) -> PDH_STATUS;
    pub fn PdhLookupPerfIndexByNameA(
        szMachineName: LPCSTR,
        szNameBuffer: LPCSTR,
        pdwIndex: LPDWORD,
    ) -> PDH_STATUS;
    pub fn PdhExpandWildCardPathW(
        szDataSource: LPCWSTR,
        szWildCardPath: LPCWSTR,
        mszExpandedPathList: LPWSTR,
        pcchPathListLength: LPDWORD,
        dwFlags: DWORD,
    ) -> PDH_STATUS;
    pub fn PdhExpandWildCardPathA(
        szDataSource: LPCSTR,
        szWildCardPath: LPCSTR,
        mszExpandedPathList: LPSTR,
        pcchPathListLength: LPDWORD,
        dwFlags: DWORD,
    ) -> PDH_STATUS;
    pub fn PdhOpenLogW(
        szLogFileName: LPCWSTR,
        dwAccessFlags: DWORD,
        lpdwLogType: LPDWORD,
        hQuery: PDH_HQUERY,
        dwMaxSize: DWORD,
        szUserCaption: LPCWSTR,
        phLog: *mut PDH_HLOG,
    ) -> PDH_STATUS;
    pub fn PdhOpenLogA(
        szLogFileName: LPCSTR,
        dwAccessFlags: DWORD,
        lpdwLogType: LPDWORD,
        hQuery: PDH_HQUERY,
        dwMaxSize: DWORD,
        szUserCaption: LPCSTR,
        phLog: *mut PDH_HLOG,
    ) -> PDH_STATUS;
    pub fn PdhUpdateLogW(
        hLog: PDH_HLOG,
        szUserString: LPCWSTR,
    ) -> PDH_STATUS;
    pub fn PdhUpdateLogA(
        hLog: PDH_HLOG,
        szUserString: LPCSTR,
    ) -> PDH_STATUS;
    pub fn PdhUpdateLogFileCatalog(
        hLog: PDH_HLOG,
    ) -> PDH_STATUS;
    pub fn PdhGetLogFileSize(
        hLog: PDH_HLOG,
        llSize: *mut LONGLONG,
    ) -> PDH_STATUS;
    pub fn PdhCloseLog(
        hLog: PDH_HLOG,
        dwFlags: DWORD,
    ) -> PDH_STATUS;
    pub fn PdhSelectDataSourceW(
        hWndOwner: HWND,
        dwFlags: DWORD,
        szDataSource: LPWSTR,
        pcchBufferLength: LPDWORD,
    ) -> PDH_STATUS;
    pub fn PdhSelectDataSourceA(
        hWndOwner: HWND,
        dwFlags: DWORD,
        szDataSource: LPSTR,
        pcchBufferLength: LPDWORD,
    ) -> PDH_STATUS;
    pub fn PdhIsRealTimeQuery(
        hQuery: PDH_HQUERY,
    ) -> PDH_STATUS;
    pub fn PdhSetQueryTimeRange(
        hQuery: PDH_HQUERY,
        pInfo: PPDH_TIME_INFO,
    ) -> PDH_STATUS;
    pub fn PdhGetDataSourceTimeRangeW(
        szDataSource: LPCWSTR,
        pdwNumEntries: LPDWORD,
        pInfo: PPDH_TIME_INFO,
        pdwBufferSize: LPDWORD,
    ) -> PDH_STATUS;
    pub fn PdhGetDataSourceTimeRangeA(
        szDataSource: LPCSTR,
        pdwNumEntries: LPDWORD,
        pInfo: PPDH_TIME_INFO,
        pdwBufferSize: LPDWORD,
    ) -> PDH_STATUS;
    pub fn PdhCollectQueryDataEx(
        hQuery: PDH_HQUERY,
        dwIntervalTime: DWORD,
        hNewDataEvent: HANDLE,
    ) -> PDH_STATUS;
    pub fn PdhFormatFromRawValue(
        dwCounterType: DWORD,
        dwFormat: DWORD,
        pTimeBase: *mut LONGLONG,
        rawValue1: PPDH_RAW_COUNTER,
        rawValue2: PPDH_RAW_COUNTER,
        fmtValue: PPDH_FMT_COUNTERVALUE,
    ) -> PDH_STATUS;
    pub fn PdhGetCounterTimeBase(
        hCounter: PDH_HCOUNTER,
        pTimeBase: *mut LONGLONG,
    ) -> PDH_STATUS;
    pub fn PdhReadRawLogRecord(
        hLog: PDH_HLOG,
        ftRecord: FILETIME,
        pRawLogRecord: PPDH_RAW_LOG_RECORD,
        pdwBufferLength: LPDWORD,
    ) -> PDH_STATUS;
    pub fn PdhSetDefaultRealTimeDataSource(
        dwDataSourceId: DWORD,
    ) -> PDH_STATUS;
    pub fn PdhBindInputDataSourceW(
        phDataSource: *mut PDH_HLOG,
        szLogFileNameList: LPCWSTR,
    ) -> PDH_STATUS;
    pub fn PdhBindInputDataSourceA(
        phDataSource: *mut PDH_HLOG,
        szLogFileNameList: LPCSTR,
    ) -> PDH_STATUS;
    pub fn PdhOpenQueryH(
        hDataSource: PDH_HLOG,
        dwUserData: DWORD_PTR,
        phQuery: *mut PDH_HQUERY,
    ) -> PDH_STATUS;
    pub fn PdhEnumMachinesHW(
        hDataSource: PDH_HLOG,
        mszMachineNameList: LPWSTR,
        pcchBufferLength: LPDWORD,
    ) -> PDH_STATUS;
    pub fn PdhEnumMachinesHA(
        hDataSource: PDH_HLOG,
        mszMachineNameList: LPSTR,
        pcchBufferLength: LPDWORD,
    ) -> PDH_STATUS;
    pub fn PdhEnumObjectsHW(
        hDataSource: PDH_HLOG,
        szMachineName: LPCWSTR,
        mszObjectList: LPWSTR,
        pcchBufferLength: LPDWORD,
        dwDetailLevel: DWORD,
        bRefresh: BOOL,
    ) -> PDH_STATUS;
    pub fn PdhEnumObjectsHA(
        hDataSource: PDH_HLOG,
        szMachineName: LPCSTR,
        mszObjectList: LPSTR,
        pcchBufferLength: LPDWORD,
        dwDetailLevel: DWORD,
        bRefresh: BOOL,
    ) -> PDH_STATUS;
    pub fn PdhEnumObjectItemsHW(
        hDataSource: PDH_HLOG,
        szMachineName: LPCWSTR,
        szObjectName: LPCWSTR,
        mszCounterList: LPWSTR,
        pcchCounterListLength: LPDWORD,
        mszInstanceList: LPWSTR,
        pcchInstanceListLength: LPDWORD,
        dwDetailLevel: DWORD,
        dwFlags: DWORD,
    ) -> PDH_STATUS;
    pub fn PdhEnumObjectItemsHA(
        hDataSource: PDH_HLOG,
        szMachineName: LPCSTR,
        szObjectName: LPCSTR,
        mszCounterList: LPSTR,
        pcchCounterListLength: LPDWORD,
        mszInstanceList: LPSTR,
        pcchInstanceListLength: LPDWORD,
        dwDetailLevel: DWORD,
        dwFlags: DWORD,
    ) -> PDH_STATUS;
    pub fn PdhExpandWildCardPathHW(
        hDataSource: PDH_HLOG,
        szWildCardPath: LPCWSTR,
        mszExpandedPathList: LPWSTR,
        pcchPathListLength: LPDWORD,
        dwFlags: DWORD,
    ) -> PDH_STATUS;
    pub fn PdhExpandWildCardPathHA(
        hDataSource: PDH_HLOG,
        szWildCardPath: LPCSTR,
        mszExpandedPathList: LPSTR,
        pcchPathListLength: LPDWORD,
        dwFlags: DWORD,
    ) -> PDH_STATUS;
    pub fn PdhGetDataSourceTimeRangeH(
        hDataSource: PDH_HLOG,
        pdwNumEntries: LPDWORD,
        pInfo: PPDH_TIME_INFO,
        pdwBufferSize: LPDWORD,
    ) -> PDH_STATUS;
    pub fn PdhGetDefaultPerfObjectHW(
        hDataSource: PDH_HLOG,
        szMachineName: LPCWSTR,
        szDefaultObjectName: LPWSTR,
        pcchBufferSize: LPDWORD,
    ) -> PDH_STATUS;
    pub fn PdhGetDefaultPerfObjectHA(
        hDataSource: PDH_HLOG,
        szMachineName: LPCSTR,
        szDefaultObjectName: LPSTR,
        pcchBufferSize: LPDWORD,
    ) -> PDH_STATUS;
    pub fn PdhGetDefaultPerfCounterHW(
        hDataSource: PDH_HLOG,
        szMachineName: LPCWSTR,
        szObjectName: LPCWSTR,
        szDefaultCounterName: LPWSTR,
        pcchBufferSize: LPDWORD,
    ) -> PDH_STATUS;
    pub fn PdhGetDefaultPerfCounterHA(
        hDataSource: PDH_HLOG,
        szMachineName: LPCSTR,
        szObjectName: LPCSTR,
        szDefaultCounterName: LPSTR,
        pcchBufferSize: LPDWORD,
    ) -> PDH_STATUS;
    pub fn PdhBrowseCountersHW(
        pBrowseDlgData: PPDH_BROWSE_DLG_CONFIG_HW,
    ) -> PDH_STATUS;
    pub fn PdhBrowseCountersHA(
        pBrowseDlgData: PPDH_BROWSE_DLG_CONFIG_HA,
    ) -> PDH_STATUS;
    pub fn PdhEnumLogSetNamesW(
        szDataSource: LPCWSTR,
        mszLogSetNameList: LPWSTR,
        pcchBufferLength: LPDWORD,
    ) -> PDH_STATUS;
    pub fn PdhEnumLogSetNamesA(
        szDataSource: LPCSTR,
        mszLogSetNameList: LPSTR,
        pcchBufferLength: LPDWORD,
    ) -> PDH_STATUS;
}
// pub fn PdhVerifySQLDBW() -> PDH_STATUS;
// pub fn PdhVerifySQLDBA() -> PDH_STATUS;
// pub fn PdhCreateSQLTablesW() -> PDH_STATUS;
// pub fn PdhCreateSQLTablesA() -> PDH_STATUS;
//pub fn PdhGetLogSetGUID() -> PDH_STATUS;
// pub fn PdhSetLogSetRunID() -> PDH_STATUS;
