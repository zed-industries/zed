// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::basetsd::SIZE_T;
use shared::guiddef::{GUID, LPCGUID, LPGUID};
use shared::minwindef::{DWORD, LPBYTE, LPDWORD, LPVOID, ULONG};
use um::minwinbase::SYSTEMTIME;
use um::winnt::{HANDLE, LONG, LONGLONG, LPCWSTR, PCWSTR, PHANDLE, PVOID, ULONGLONG};
pub const PERF_PROVIDER_USER_MODE: ULONG = 0;
pub const PERF_PROVIDER_KERNEL_MODE: ULONG = 1;
pub const PERF_PROVIDER_DRIVER: ULONG = 2;
pub const PERF_COUNTERSET_FLAG_MULTIPLE: ULONG = 2;
pub const PERF_COUNTERSET_FLAG_AGGREGATE: ULONG = 4;
pub const PERF_COUNTERSET_FLAG_HISTORY: ULONG = 8;
pub const PERF_COUNTERSET_FLAG_INSTANCE: ULONG = 16;
pub const PERF_COUNTERSET_SINGLE_INSTANCE: ULONG = 0;
pub const PERF_COUNTERSET_MULTI_INSTANCES: ULONG = PERF_COUNTERSET_FLAG_MULTIPLE;
pub const PERF_COUNTERSET_SINGLE_AGGREGATE: ULONG = PERF_COUNTERSET_FLAG_AGGREGATE;
pub const PERF_COUNTERSET_MULTI_AGGREGATE: ULONG = PERF_COUNTERSET_FLAG_AGGREGATE
    | PERF_COUNTERSET_FLAG_MULTIPLE;
pub const PERF_COUNTERSET_SINGLE_AGGREGATE_HISTORY: ULONG = PERF_COUNTERSET_FLAG_HISTORY
    | PERF_COUNTERSET_SINGLE_AGGREGATE;
pub const PERF_COUNTERSET_INSTANCE_AGGREGATE: ULONG = PERF_COUNTERSET_MULTI_AGGREGATE
    | PERF_COUNTERSET_FLAG_INSTANCE;
pub const PERF_AGGREGATE_UNDEFINED: ULONG = 0;
pub const PERF_AGGREGATE_TOTAL: ULONG = 1;
pub const PERF_AGGREGATE_AVG: ULONG = 2;
pub const PERF_AGGREGATE_MIN: ULONG = 3;
pub const PERF_AGGREGATE_MAX: ULONG = 4;
pub const PERF_ATTRIB_BY_REFERENCE: ULONGLONG = 0x0000000000000001;
pub const PERF_ATTRIB_NO_DISPLAYABLE: ULONGLONG = 0x0000000000000002;
pub const PERF_ATTRIB_NO_GROUP_SEPARATOR: ULONGLONG = 0x0000000000000004;
pub const PERF_ATTRIB_DISPLAY_AS_REAL: ULONGLONG = 0x0000000000000008;
pub const PERF_ATTRIB_DISPLAY_AS_HEX: ULONGLONG = 0x0000000000000010;
STRUCT!{struct PERF_COUNTERSET_INFO {
    CounterSetGuid: GUID,
    ProviderGuid: GUID,
    NumCounters: ULONG,
    InstanceType: ULONG,
}}
pub type PPERF_COUNTERSET_INFO = *mut PERF_COUNTERSET_INFO;
STRUCT!{struct PERF_COUNTER_INFO {
    CounterId: ULONG,
    Type: ULONG,
    Attrib: ULONGLONG,
    Size: ULONG,
    DetailLevel: ULONG,
    Scale: LONG,
    Offset: LONG,
}}
pub type PPERF_COUNTER_INFO = *mut PERF_COUNTER_INFO;
STRUCT!{struct PERF_COUNTERSET_INSTANCE {
    CounterSetGuid: GUID,
    dwSize: ULONG,
    InstanceId: ULONG,
    InstanceNameOffset: ULONG,
    InstanceNameSize: ULONG,
}}
pub type PPERF_COUNTERSET_INSTANCE = *mut PERF_COUNTERSET_INSTANCE;
STRUCT!{struct PERF_COUNTER_IDENTITY {
    CounterSetGuid: GUID,
    BufferSize: ULONG,
    CounterId: ULONG,
    InstanceId: ULONG,
    MachineOffset: ULONG,
    NameOffset: ULONG,
    Reserved: ULONG,
}}
pub type PPERF_COUNTER_IDENTITY = *mut PERF_COUNTER_IDENTITY;
pub const PERF_WILDCARD_COUNTER: ULONG = 0xFFFFFFFF;
pub const PERF_WILDCARD_INSTANCE: &'static str = "*";
pub const PERF_AGGREGATE_INSTANCE: &'static str = "_Total";
pub const PERF_MAX_INSTANCE_NAME: ULONG = 1024;
pub const PERF_ADD_COUNTER: ULONG = 1;
pub const PERF_REMOVE_COUNTER: ULONG = 2;
pub const PERF_ENUM_INSTANCES: ULONG = 3;
pub const PERF_COLLECT_START: ULONG = 5;
pub const PERF_COLLECT_END: ULONG = 6;
pub const PERF_FILTER: ULONG = 9;
FN!{stdcall PERFLIBREQUEST(
    RequestCode: ULONG,
    Buffer: PVOID,
    BufferSize: ULONG,
) -> ULONG}
FN!{stdcall PERF_MEM_ALLOC(
    AllocSize: SIZE_T,
    pContext: LPVOID,
) -> LPVOID}
FN!{stdcall PERF_MEM_FREE(
    pBuffer: LPVOID,
    pContext: LPVOID,
) -> ()}
STRUCT!{struct PERF_PROVIDER_CONTEXT {
    ContextSize: DWORD,
    Reserved: DWORD,
    ControlCallback: PERFLIBREQUEST,
    MemAllocRoutine: PERF_MEM_ALLOC,
    MemFreeRoutine: PERF_MEM_FREE,
    pMemContext: LPVOID,
}}
pub type PPERF_PROVIDER_CONTEXT = *mut PERF_PROVIDER_CONTEXT;
extern "system" {
    pub fn PerfStartProviderEx(
        ProviderGuid: LPGUID,
        ProviderContext: PPERF_PROVIDER_CONTEXT,
        Provider: PHANDLE,
    ) -> ULONG;
    pub fn PerfStartProvider(
        ProviderGuid: LPGUID,
        ControlCallback: PERFLIBREQUEST,
        Provider: PHANDLE,
    ) -> ULONG;
    pub fn PerfStopProvider(
        ProviderHandle: HANDLE,
    ) -> ULONG;
    pub fn PerfSetCounterSetInfo(
        ProviderHandle: HANDLE,
        Template: PPERF_COUNTERSET_INFO,
        TemplateSize: ULONG,
    ) -> ULONG;
    pub fn PerfCreateInstance(
        ProviderHandle: HANDLE,
        CounterSetGuid: LPCGUID,
        Name: PCWSTR,
        Id: ULONG,
    ) -> PPERF_COUNTERSET_INSTANCE;
    pub fn PerfDeleteInstance(
        Provider: HANDLE,
        InstanceBlock: PPERF_COUNTERSET_INSTANCE,
    ) -> ULONG;
    pub fn PerfQueryInstance(
        ProviderHandle: HANDLE,
        CounterSetGuid: LPCGUID,
        Name: LPCWSTR,
        Id: ULONG,
    ) -> PPERF_COUNTERSET_INSTANCE;
    pub fn PerfSetCounterRefValue(
        Provider: HANDLE,
        Instance: PPERF_COUNTERSET_INSTANCE,
        CounterId: ULONG,
        Address: PVOID,
    ) -> ULONG;
    pub fn PerfSetULongCounterValue(
        Provider: HANDLE,
        Instance: PPERF_COUNTERSET_INSTANCE,
        CounterId: ULONG,
        Value: ULONG,
    ) -> ULONG;
    pub fn PerfSetULongLongCounterValue(
        Provider: HANDLE,
        Instance: PPERF_COUNTERSET_INSTANCE,
        CounterId: ULONG,
        Value: ULONGLONG,
    ) -> ULONG;
    pub fn PerfIncrementULongCounterValue(
        Provider: HANDLE,
        Instance: PPERF_COUNTERSET_INSTANCE,
        CounterId: ULONG,
        Value: ULONG,
    ) -> ULONG;
    pub fn PerfIncrementULongLongCounterValue(
        Provider: HANDLE,
        Instance: PPERF_COUNTERSET_INSTANCE,
        CounterId: ULONG,
        Value: ULONGLONG,
    ) -> ULONG;
    pub fn PerfDecrementULongCounterValue(
        Provider: HANDLE,
        Instance: PPERF_COUNTERSET_INSTANCE,
        CounterId: ULONG,
        Value: ULONG,
    ) -> ULONG;
    pub fn PerfDecrementULongLongCounterValue(
        Provider: HANDLE,
        Instance: PPERF_COUNTERSET_INSTANCE,
        CounterId: ULONG,
        Value: ULONGLONG,
    ) -> ULONG;
}
STRUCT!{struct PERF_INSTANCE_HEADER {
    Size: ULONG,
    InstanceId: ULONG,
}}
pub type PPERF_INSTANCE_HEADER = *mut PERF_INSTANCE_HEADER;
ENUM!{enum PerfRegInfoType {
    PERF_REG_COUNTERSET_STRUCT = 1,
    PERF_REG_COUNTER_STRUCT,
    PERF_REG_COUNTERSET_NAME_STRING,
    PERF_REG_COUNTERSET_HELP_STRING,
    PERF_REG_COUNTER_NAME_STRINGS,
    PERF_REG_COUNTER_HELP_STRINGS,
    PERF_REG_PROVIDER_NAME,
    PERF_REG_PROVIDER_GUID,
    PERF_REG_COUNTERSET_ENGLISH_NAME,
    PERF_REG_COUNTER_ENGLISH_NAMES,
}}
STRUCT!{struct PERF_COUNTERSET_REG_INFO {
    CounterSetGuid: GUID,
    CounterSetType: ULONG,
    DetailLevel: ULONG,
    NumCounters: ULONG,
    InstanceType: ULONG,
}}
pub type PPERF_COUNTERSET_REG_INFO = *mut PERF_COUNTERSET_REG_INFO;
STRUCT!{struct PERF_COUNTER_REG_INFO {
    CounterId: ULONG,
    Type: ULONG,
    Attrib: ULONGLONG,
    DetailLevel: ULONG,
    DefaultScale: LONG,
    BaseCounterId: ULONG,
    PerfTimeId: ULONG,
    PerfFreqId: ULONG,
    MultiId: ULONG,
    AggregateFinc: ULONG,
    Reserved: ULONG,
}}
pub type PPERF_COUNTER_REG_INFO = *mut PERF_COUNTER_REG_INFO;
STRUCT!{struct PERF_STRING_BUFFER_HEADER {
    dwSize: DWORD,
    dwCounters: DWORD,
}}
pub type PPERF_STRING_BUFFER_HEADER = *mut PERF_STRING_BUFFER_HEADER;
STRUCT!{struct PERF_STRING_COUNTER_HEADER {
    dwCounterId: DWORD,
    dwOffset: DWORD,
}}
pub type PPERF_STRING_COUNTER_HEADER = *mut PERF_STRING_COUNTER_HEADER;
STRUCT!{struct PERF_COUNTER_IDENTIFIER {
    CounterSetGuid: GUID,
    Status: ULONG,
    Size: ULONG,
    CounterId: ULONG,
    InstanceId: ULONG,
    Index: ULONG,
    Reserved: ULONG,
}}
pub type PPERF_COUNTER_IDENTIFIER = *mut PERF_COUNTER_IDENTIFIER;
STRUCT!{struct PERF_DATA_HEADER {
    dwTotalSize: ULONG,
    dwNumCounters: ULONG,
    PerfTimeStamp: LONGLONG,
    PerfTime100NSec: LONGLONG,
    PrefFreq: LONGLONG,
    SystemTime: SYSTEMTIME,
}}
pub type PPERF_DATA_HEADER = *mut PERF_DATA_HEADER;
ENUM!{enum PerfCounterDataType {
    PERF_ERROR_RETURN = 0,
    PERF_SINGLE_COUNTER = 1,
    PERF_MULTIPLE_COUNTERS = 2,
    PERF_MULTIPLE_INSTANCES = 4,
    PERF_COUNTERSET = 6,
}}
STRUCT!{struct PERF_COUNTER_HEADER {
    dwStatus: ULONG,
    dwType: PerfCounterDataType,
    dwSize: ULONG,
    Reserved: ULONG,
}}
pub type PPERF_COUNTER_HEADER = *mut PERF_COUNTER_HEADER;
STRUCT!{struct PERF_MULTI_INSTANCES {
    dwTotalSize: ULONG,
    dwInstances: ULONG,
}}
pub type PPERF_MULTI_INSTANCES = *mut PERF_MULTI_INSTANCES;
STRUCT!{struct PERF_MULTI_COUNTERS {
    dwSize: ULONG,
    dwCounters: ULONG,
}}
pub type PPERF_MULTI_COUNTERS = *mut PERF_MULTI_COUNTERS;
STRUCT!{struct PERF_COUNTER_DATA {
    dwDataSize: ULONG,
    dwSize: ULONG,
}}
pub type PPERF_COUNTER_DATA = *mut PERF_COUNTER_DATA;
extern "system" {
    pub fn PerfEnumerateCounterSet(
        szMachine: LPCWSTR,
        pCounterSetIds: LPGUID,
        cCounterSetIds: DWORD,
        pcCounterSetIdsActual: LPDWORD,
    ) -> ULONG;
    pub fn PerfEnumerateCounterSetInstances(
        szMachine: LPCWSTR,
        pCounterSetIds: LPCGUID,
        pInstances: PPERF_INSTANCE_HEADER,
        cbInstances: DWORD,
        pcbInstancesActual: LPDWORD,
    ) -> ULONG;
    pub fn PerfQueryCounterSetRegistrationInfo(
        szMachine: LPCWSTR,
        pCounterSetId: LPCGUID,
        requestCode: PerfRegInfoType,
        requestLangId: DWORD,
        pbRegInfo: LPBYTE,
        cbRegInfo: DWORD,
        pcbRegInfoActual: LPDWORD,
    ) -> ULONG;
    pub fn PerfOpenQueryHandle(
        szMachine: LPCWSTR,
        hQuery: *mut HANDLE,
    ) -> ULONG;
    pub fn PerfCloseQueryHandle(
        hQuery: HANDLE,
    ) -> ULONG;
    pub fn PerfQueryCounterInfo(
        hQuery: HANDLE,
        pCounters: PPERF_COUNTER_IDENTIFIER,
        cbCounters: DWORD,
        pcbCountersActual: LPDWORD,
    ) -> ULONG;
    pub fn PerfQueryCounterData(
        hQuery: HANDLE,
        pCounterBlock: PPERF_DATA_HEADER,
        cbCounterBlock: DWORD,
        pcbCounterBlockActual: LPDWORD,
    ) -> ULONG;
    pub fn PerfAddCounters(
        hQuery: HANDLE,
        pCounters: PPERF_COUNTER_IDENTIFIER,
        cbCounters: DWORD,
    ) -> ULONG;
    pub fn PerfDeleteCounters(
        hQuery: HANDLE,
        pCounters: PPERF_COUNTER_IDENTIFIER,
        cbCounters: DWORD,
    ) -> ULONG;
}
