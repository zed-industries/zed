// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::basetsd::{SIZE_T, ULONG32, ULONG64};
use shared::evntprov::PEVENT_FILTER_DESCRIPTOR;
use shared::guiddef::{GUID, LPCGUID, LPGUID};
use shared::minwindef::{DWORD, LPFILETIME, PULONG, UCHAR, UINT, ULONG, USHORT};
use shared::wmistr::{WMIDPREQUESTCODE, WNODE_HEADER};
use um::evntcons::PEVENT_RECORD;
use um::handleapi::INVALID_HANDLE_VALUE;
use um::timezoneapi::TIME_ZONE_INFORMATION;
use um::winnt::{
    ANYSIZE_ARRAY, BOOLEAN, HANDLE, LARGE_INTEGER, LONG, LONGLONG, LPCSTR, LPCWSTR, LPSTR, LPWSTR,
    PVOID, ULONGLONG, WCHAR
};
use vc::vadefs::va_list;
DEFINE_GUID!{EventTraceGuid,
    0x68fdd900, 0x4a3e, 0x11d1, 0x84, 0xf4, 0x00, 0x00, 0xf8, 0x04, 0x64, 0xe3}
DEFINE_GUID!{SystemTraceControlGuid,
    0x9e814aad, 0x3204, 0x11d2, 0x9a, 0x82, 0x00, 0x60, 0x08, 0xa8, 0x69, 0x39}
DEFINE_GUID!{EventTraceConfigGuid,
    0x01853a65, 0x418f, 0x4f36, 0xae, 0xfc, 0xdc, 0x0f, 0x1d, 0x2f, 0xd2, 0x35}
DEFINE_GUID!{DefaultTraceSecurityGuid,
    0x0811c1af, 0x7a07, 0x4a06, 0x82, 0xed, 0x86, 0x94, 0x55, 0xcd, 0xf7, 0x13}
DEFINE_GUID!{PrivateLoggerNotificationGuid,
    0x3595ab5c, 0x042a, 0x4c8e, 0xb9, 0x42, 0x2d, 0x05, 0x9b, 0xfe, 0xb1, 0xb1}
pub const KERNEL_LOGGER_NAME: &'static str = "NT Kernel Logger";
pub const GLOBAL_LOGGER_NAME: &'static str = "GlobalLogger";
pub const EVENT_LOGGER_NAME: &'static str = "EventLog";
pub const DIAG_LOGGER_NAME: &'static str = "DiagLog";
pub const MAX_MOF_FIELDS: SIZE_T = 16;
DECLARE_HANDLE!{TRACEHANDLE, __TRACEHANDLE}
pub type PTRACEHANDLE = *mut TRACEHANDLE;
pub const EVENT_TRACE_TYPE_INFO: DWORD = 0x00;
pub const EVENT_TRACE_TYPE_START: DWORD = 0x01;
pub const EVENT_TRACE_TYPE_END: DWORD = 0x02;
pub const EVENT_TRACE_TYPE_STOP: DWORD = 0x02;
pub const EVENT_TRACE_TYPE_DC_START: DWORD = 0x03;
pub const EVENT_TRACE_TYPE_DC_END: DWORD = 0x04;
pub const EVENT_TRACE_TYPE_EXTENSION: DWORD = 0x05;
pub const EVENT_TRACE_TYPE_REPLY: DWORD = 0x06;
pub const EVENT_TRACE_TYPE_DEQUEUE: DWORD = 0x07;
pub const EVENT_TRACE_TYPE_RESUME: DWORD = 0x07;
pub const EVENT_TRACE_TYPE_CHECKPOINT: DWORD = 0x08;
pub const EVENT_TRACE_TYPE_SUSPEND: DWORD = 0x08;
pub const EVENT_TRACE_TYPE_WINEVT_SEND: DWORD = 0x09;
pub const EVENT_TRACE_TYPE_WINEVT_RECEIVE: DWORD = 0xF0;
pub const TRACE_LEVEL_CRITICAL: UCHAR = 1;
pub const TRACE_LEVEL_ERROR: UCHAR = 2;
pub const TRACE_LEVEL_WARNING: UCHAR = 3;
pub const TRACE_LEVEL_INFORMATION: UCHAR = 4;
pub const TRACE_LEVEL_VERBOSE: UCHAR = 5;
pub const TRACE_LEVEL_RESERVED6: UCHAR = 6;
pub const TRACE_LEVEL_RESERVED7: UCHAR = 7;
pub const TRACE_LEVEL_RESERVED8: UCHAR = 8;
pub const TRACE_LEVEL_RESERVED9: UCHAR = 9;
pub const EVENT_TRACE_TYPE_LOAD: DWORD = 0x0A;
pub const EVENT_TRACE_TYPE_TERMINATE: DWORD = 0x0B;
pub const EVENT_TRACE_TYPE_IO_READ: DWORD = 0x0A;
pub const EVENT_TRACE_TYPE_IO_WRITE: DWORD = 0x0B;
pub const EVENT_TRACE_TYPE_IO_READ_INIT: DWORD = 0x0C;
pub const EVENT_TRACE_TYPE_IO_WRITE_INIT: DWORD = 0x0D;
pub const EVENT_TRACE_TYPE_IO_FLUSH: DWORD = 0x0E;
pub const EVENT_TRACE_TYPE_IO_FLUSH_INIT: DWORD = 0x0F;
pub const EVENT_TRACE_TYPE_IO_REDIRECTED_INIT: DWORD = 0x10;
pub const EVENT_TRACE_TYPE_MM_TF: DWORD = 0x0A;
pub const EVENT_TRACE_TYPE_MM_DZF: DWORD = 0x0B;
pub const EVENT_TRACE_TYPE_MM_COW: DWORD = 0x0C;
pub const EVENT_TRACE_TYPE_MM_GPF: DWORD = 0x0D;
pub const EVENT_TRACE_TYPE_MM_HPF: DWORD = 0x0E;
pub const EVENT_TRACE_TYPE_MM_AV: DWORD = 0x0F;
pub const EVENT_TRACE_TYPE_SEND: DWORD = 0x0A;
pub const EVENT_TRACE_TYPE_RECEIVE: DWORD = 0x0B;
pub const EVENT_TRACE_TYPE_CONNECT: DWORD = 0x0C;
pub const EVENT_TRACE_TYPE_DISCONNECT: DWORD = 0x0D;
pub const EVENT_TRACE_TYPE_RETRANSMIT: DWORD = 0x0E;
pub const EVENT_TRACE_TYPE_ACCEPT: DWORD = 0x0F;
pub const EVENT_TRACE_TYPE_RECONNECT: DWORD = 0x10;
pub const EVENT_TRACE_TYPE_CONNFAIL: DWORD = 0x11;
pub const EVENT_TRACE_TYPE_COPY_TCP: DWORD = 0x12;
pub const EVENT_TRACE_TYPE_COPY_ARP: DWORD = 0x13;
pub const EVENT_TRACE_TYPE_ACKFULL: DWORD = 0x14;
pub const EVENT_TRACE_TYPE_ACKPART: DWORD = 0x15;
pub const EVENT_TRACE_TYPE_ACKDUP: DWORD = 0x16;
pub const EVENT_TRACE_TYPE_GUIDMAP: DWORD = 0x0A;
pub const EVENT_TRACE_TYPE_CONFIG: DWORD = 0x0B;
pub const EVENT_TRACE_TYPE_SIDINFO: DWORD = 0x0C;
pub const EVENT_TRACE_TYPE_SECURITY: DWORD = 0x0D;
pub const EVENT_TRACE_TYPE_DBGID_RSDS: DWORD = 0x40;
pub const EVENT_TRACE_TYPE_REGCREATE: DWORD = 0x0A;
pub const EVENT_TRACE_TYPE_REGOPEN: DWORD = 0x0B;
pub const EVENT_TRACE_TYPE_REGDELETE: DWORD = 0x0C;
pub const EVENT_TRACE_TYPE_REGQUERY: DWORD = 0x0D;
pub const EVENT_TRACE_TYPE_REGSETVALUE: DWORD = 0x0E;
pub const EVENT_TRACE_TYPE_REGDELETEVALUE: DWORD = 0x0F;
pub const EVENT_TRACE_TYPE_REGQUERYVALUE: DWORD = 0x10;
pub const EVENT_TRACE_TYPE_REGENUMERATEKEY: DWORD = 0x11;
pub const EVENT_TRACE_TYPE_REGENUMERATEVALUEKEY: DWORD = 0x12;
pub const EVENT_TRACE_TYPE_REGQUERYMULTIPLEVALUE: DWORD = 0x13;
pub const EVENT_TRACE_TYPE_REGSETINFORMATION: DWORD = 0x14;
pub const EVENT_TRACE_TYPE_REGFLUSH: DWORD = 0x15;
pub const EVENT_TRACE_TYPE_REGKCBCREATE: DWORD = 0x16;
pub const EVENT_TRACE_TYPE_REGKCBDELETE: DWORD = 0x17;
pub const EVENT_TRACE_TYPE_REGKCBRUNDOWNBEGIN: DWORD = 0x18;
pub const EVENT_TRACE_TYPE_REGKCBRUNDOWNEND: DWORD = 0x19;
pub const EVENT_TRACE_TYPE_REGVIRTUALIZE: DWORD = 0x1A;
pub const EVENT_TRACE_TYPE_REGCLOSE: DWORD = 0x1B;
pub const EVENT_TRACE_TYPE_REGSETSECURITY: DWORD = 0x1C;
pub const EVENT_TRACE_TYPE_REGQUERYSECURITY: DWORD = 0x1D;
pub const EVENT_TRACE_TYPE_REGCOMMIT: DWORD = 0x1E;
pub const EVENT_TRACE_TYPE_REGPREPARE: DWORD = 0x1F;
pub const EVENT_TRACE_TYPE_REGROLLBACK: DWORD = 0x20;
pub const EVENT_TRACE_TYPE_REGMOUNTHIVE: DWORD = 0x21;
pub const EVENT_TRACE_TYPE_CONFIG_CPU: DWORD = 0x0A;
pub const EVENT_TRACE_TYPE_CONFIG_PHYSICALDISK: DWORD = 0x0B;
pub const EVENT_TRACE_TYPE_CONFIG_LOGICALDISK: DWORD = 0x0C;
pub const EVENT_TRACE_TYPE_CONFIG_NIC: DWORD = 0x0D;
pub const EVENT_TRACE_TYPE_CONFIG_VIDEO: DWORD = 0x0E;
pub const EVENT_TRACE_TYPE_CONFIG_SERVICES: DWORD = 0x0F;
pub const EVENT_TRACE_TYPE_CONFIG_POWER: DWORD = 0x10;
pub const EVENT_TRACE_TYPE_CONFIG_NETINFO: DWORD = 0x11;
pub const EVENT_TRACE_TYPE_CONFIG_OPTICALMEDIA: DWORD = 0x12;
pub const EVENT_TRACE_TYPE_CONFIG_IRQ: DWORD = 0x15;
pub const EVENT_TRACE_TYPE_CONFIG_PNP: DWORD = 0x16;
pub const EVENT_TRACE_TYPE_CONFIG_IDECHANNEL: DWORD = 0x17;
pub const EVENT_TRACE_TYPE_CONFIG_NUMANODE: DWORD = 0x18;
pub const EVENT_TRACE_TYPE_CONFIG_PLATFORM: DWORD = 0x19;
pub const EVENT_TRACE_TYPE_CONFIG_PROCESSORGROUP: DWORD = 0x1A;
pub const EVENT_TRACE_TYPE_CONFIG_PROCESSORNUMBER: DWORD = 0x1B;
pub const EVENT_TRACE_TYPE_CONFIG_DPI: DWORD = 0x1C;
pub const EVENT_TRACE_TYPE_CONFIG_CI_INFO: DWORD = 0x1D;
pub const EVENT_TRACE_TYPE_CONFIG_MACHINEID: DWORD = 0x1E;
pub const EVENT_TRACE_TYPE_CONFIG_DEFRAG: DWORD = 0x1F;
pub const EVENT_TRACE_TYPE_CONFIG_MOBILEPLATFORM: DWORD = 0x20;
pub const EVENT_TRACE_TYPE_CONFIG_DEVICEFAMILY: DWORD = 0x21;
pub const EVENT_TRACE_TYPE_CONFIG_FLIGHTID: DWORD = 0x22;
pub const EVENT_TRACE_TYPE_CONFIG_PROCESSOR: DWORD = 0x23;
pub const EVENT_TRACE_TYPE_OPTICAL_IO_READ: DWORD = 0x37;
pub const EVENT_TRACE_TYPE_OPTICAL_IO_WRITE: DWORD = 0x38;
pub const EVENT_TRACE_TYPE_OPTICAL_IO_FLUSH: DWORD = 0x39;
pub const EVENT_TRACE_TYPE_OPTICAL_IO_READ_INIT: DWORD = 0x3a;
pub const EVENT_TRACE_TYPE_OPTICAL_IO_WRITE_INIT: DWORD = 0x3b;
pub const EVENT_TRACE_TYPE_OPTICAL_IO_FLUSH_INIT: DWORD = 0x3c;
pub const EVENT_TRACE_TYPE_FLT_PREOP_INIT: DWORD = 0x60;
pub const EVENT_TRACE_TYPE_FLT_POSTOP_INIT: DWORD = 0x61;
pub const EVENT_TRACE_TYPE_FLT_PREOP_COMPLETION: DWORD = 0x62;
pub const EVENT_TRACE_TYPE_FLT_POSTOP_COMPLETION: DWORD = 0x63;
pub const EVENT_TRACE_TYPE_FLT_PREOP_FAILURE: DWORD = 0x64;
pub const EVENT_TRACE_TYPE_FLT_POSTOP_FAILURE: DWORD = 0x65;
pub const EVENT_TRACE_FLAG_PROCESS: DWORD = 0x00000001;
pub const EVENT_TRACE_FLAG_THREAD: DWORD = 0x00000002;
pub const EVENT_TRACE_FLAG_IMAGE_LOAD: DWORD = 0x00000004;
pub const EVENT_TRACE_FLAG_DISK_IO: DWORD = 0x00000100;
pub const EVENT_TRACE_FLAG_DISK_FILE_IO: DWORD = 0x00000200;
pub const EVENT_TRACE_FLAG_MEMORY_PAGE_FAULTS: DWORD = 0x00001000;
pub const EVENT_TRACE_FLAG_MEMORY_HARD_FAULTS: DWORD = 0x00002000;
pub const EVENT_TRACE_FLAG_NETWORK_TCPIP: DWORD = 0x00010000;
pub const EVENT_TRACE_FLAG_REGISTRY: DWORD = 0x00020000;
pub const EVENT_TRACE_FLAG_DBGPRINT: DWORD = 0x00040000;
pub const EVENT_TRACE_FLAG_PROCESS_COUNTERS: DWORD = 0x00000008;
pub const EVENT_TRACE_FLAG_CSWITCH: DWORD = 0x00000010;
pub const EVENT_TRACE_FLAG_DPC: DWORD = 0x00000020;
pub const EVENT_TRACE_FLAG_INTERRUPT: DWORD = 0x00000040;
pub const EVENT_TRACE_FLAG_SYSTEMCALL: DWORD = 0x00000080;
pub const EVENT_TRACE_FLAG_DISK_IO_INIT: DWORD = 0x00000400;
pub const EVENT_TRACE_FLAG_ALPC: DWORD = 0x00100000;
pub const EVENT_TRACE_FLAG_SPLIT_IO: DWORD = 0x00200000;
pub const EVENT_TRACE_FLAG_DRIVER: DWORD = 0x00800000;
pub const EVENT_TRACE_FLAG_PROFILE: DWORD = 0x01000000;
pub const EVENT_TRACE_FLAG_FILE_IO: DWORD = 0x02000000;
pub const EVENT_TRACE_FLAG_FILE_IO_INIT: DWORD = 0x04000000;
pub const EVENT_TRACE_FLAG_DISPATCHER: DWORD = 0x00000800;
pub const EVENT_TRACE_FLAG_VIRTUAL_ALLOC: DWORD = 0x00004000;
pub const EVENT_TRACE_FLAG_VAMAP: DWORD = 0x00008000;
pub const EVENT_TRACE_FLAG_NO_SYSCONFIG: DWORD = 0x10000000;
pub const EVENT_TRACE_FLAG_JOB: DWORD = 0x00080000;
pub const EVENT_TRACE_FLAG_DEBUG_EVENTS: DWORD = 0x00400000;
pub const EVENT_TRACE_FLAG_EXTENSION: DWORD = 0x80000000;
pub const EVENT_TRACE_FLAG_FORWARD_WMI: DWORD = 0x40000000;
pub const EVENT_TRACE_FLAG_ENABLE_RESERVE: DWORD = 0x20000000;
pub const EVENT_TRACE_FILE_MODE_NONE: DWORD = 0x00000000;
pub const EVENT_TRACE_FILE_MODE_SEQUENTIAL: DWORD = 0x00000001;
pub const EVENT_TRACE_FILE_MODE_CIRCULAR: DWORD = 0x00000002;
pub const EVENT_TRACE_FILE_MODE_APPEND: DWORD = 0x00000004;
pub const EVENT_TRACE_REAL_TIME_MODE: DWORD = 0x00000100;
pub const EVENT_TRACE_DELAY_OPEN_FILE_MODE: DWORD = 0x00000200;
pub const EVENT_TRACE_BUFFERING_MODE: DWORD = 0x00000400;
pub const EVENT_TRACE_PRIVATE_LOGGER_MODE: DWORD = 0x00000800;
pub const EVENT_TRACE_ADD_HEADER_MODE: DWORD = 0x00001000;
pub const EVENT_TRACE_USE_GLOBAL_SEQUENCE: DWORD = 0x00004000;
pub const EVENT_TRACE_USE_LOCAL_SEQUENCE: DWORD = 0x00008000;
pub const EVENT_TRACE_RELOG_MODE: DWORD = 0x00010000;
pub const EVENT_TRACE_USE_PAGED_MEMORY: DWORD = 0x01000000;
pub const EVENT_TRACE_FILE_MODE_NEWFILE: DWORD = 0x00000008;
pub const EVENT_TRACE_FILE_MODE_PREALLOCATE: DWORD = 0x00000020;
pub const EVENT_TRACE_NONSTOPPABLE_MODE: DWORD = 0x00000040;
pub const EVENT_TRACE_SECURE_MODE: DWORD = 0x00000080;
pub const EVENT_TRACE_USE_KBYTES_FOR_SIZE: DWORD = 0x00002000;
pub const EVENT_TRACE_PRIVATE_IN_PROC: DWORD = 0x00020000;
pub const EVENT_TRACE_MODE_RESERVED: DWORD = 0x00100000;
pub const EVENT_TRACE_NO_PER_PROCESSOR_BUFFERING: DWORD = 0x10000000;
pub const EVENT_TRACE_SYSTEM_LOGGER_MODE: DWORD = 0x02000000;
pub const EVENT_TRACE_ADDTO_TRIAGE_DUMP: DWORD = 0x80000000;
pub const EVENT_TRACE_STOP_ON_HYBRID_SHUTDOWN: DWORD = 0x00400000;
pub const EVENT_TRACE_PERSIST_ON_HYBRID_SHUTDOWN: DWORD = 0x00800000;
pub const EVENT_TRACE_INDEPENDENT_SESSION_MODE: DWORD = 0x08000000;
pub const EVENT_TRACE_COMPRESSED_MODE: DWORD = 0x04000000;
pub const EVENT_TRACE_CONTROL_QUERY: DWORD = 0;
pub const EVENT_TRACE_CONTROL_STOP: DWORD = 1;
pub const EVENT_TRACE_CONTROL_UPDATE: DWORD = 2;
pub const EVENT_TRACE_CONTROL_FLUSH: DWORD = 3;
pub const TRACE_MESSAGE_SEQUENCE: DWORD = 1;
pub const TRACE_MESSAGE_GUID: DWORD = 2;
pub const TRACE_MESSAGE_COMPONENTID: DWORD = 4;
pub const TRACE_MESSAGE_TIMESTAMP: DWORD = 8;
pub const TRACE_MESSAGE_PERFORMANCE_TIMESTAMP: DWORD = 16;
pub const TRACE_MESSAGE_SYSTEMINFO: DWORD = 32;
pub const TRACE_MESSAGE_POINTER32: DWORD = 0x0040;
pub const TRACE_MESSAGE_POINTER64: DWORD = 0x0080;
pub const TRACE_MESSAGE_FLAG_MASK: DWORD = 0xFFFF;
pub const TRACE_MESSAGE_MAXIMUM_SIZE: SIZE_T = 64 * 1024;
pub const EVENT_TRACE_USE_PROCTIME: DWORD = 0x0001;
pub const EVENT_TRACE_USE_NOCPUTIME: DWORD = 0x0002;
pub const TRACE_HEADER_FLAG_USE_TIMESTAMP: DWORD = 0x00000200;
pub const TRACE_HEADER_FLAG_TRACED_GUID: DWORD = 0x00020000;
pub const TRACE_HEADER_FLAG_LOG_WNODE: DWORD = 0x00040000;
pub const TRACE_HEADER_FLAG_USE_GUID_PTR: DWORD = 0x00080000;
pub const TRACE_HEADER_FLAG_USE_MOF_PTR: DWORD = 0x00100000;
ENUM!{enum ETW_COMPRESSION_RESUMPTION_MODE {
    EtwCompressionModeRestart = 0,
    EtwCompressionModeNoDisable = 1,
    EtwCompressionModeNoRestart = 2,
}}
STRUCT!{struct EVENT_TRACE_HEADER_u1_s {
    HeaderType: UCHAR,
    MarkerFlags: UCHAR,
}}
UNION!{union EVENT_TRACE_HEADER_u1 {
    [u16; 1],
    FieldTypeFlags FieldTypeFlags_mut: USHORT,
    s s_mut: EVENT_TRACE_HEADER_u1_s,
}}
STRUCT!{struct EVENT_TRACE_HEADER_u2_CLASS {
    Type: UCHAR,
    Level: UCHAR,
    Version: USHORT,
}}
UNION!{union EVENT_TRACE_HEADER_u2 {
    [u32; 1],
    Version Version_mut: ULONG,
    Class Class_mut: EVENT_TRACE_HEADER_u2_CLASS,
}}
UNION!{union EVENT_TRACE_HEADER_u3 {
    [u64; 2],
    Guid Guid_mut: GUID,
    GuidPtr GuidPtr_mut: ULONGLONG,
}}
STRUCT!{struct EVENT_TRACE_HEADER_u4_s1 {
    ClientContext: ULONG,
    Flags: ULONG,
}}
STRUCT!{struct EVENT_TRACE_HEADER_u4_s2 {
    KernelTime: ULONG,
    UserTime: ULONG,
}}
UNION!{union EVENT_TRACE_HEADER_u4 {
    [u64; 1],
    s1 s1_mut: EVENT_TRACE_HEADER_u4_s1,
    s2 s2_mut: EVENT_TRACE_HEADER_u4_s2,
    ProcessorTime ProcessorTime_mut: ULONG64,
}}
STRUCT!{struct EVENT_TRACE_HEADER {
    Size: USHORT,
    u1: EVENT_TRACE_HEADER_u1,
    u2: EVENT_TRACE_HEADER_u2,
    ThreadId: ULONG,
    ProcessId: ULONG,
    TimeStamp: LARGE_INTEGER,
    u3: EVENT_TRACE_HEADER_u3,
    u4: EVENT_TRACE_HEADER_u4,
}}
pub type PEVENT_TRACE_HEADER = *mut EVENT_TRACE_HEADER;
STRUCT!{struct EVENT_INSTANCE_HEADER_u1_s {
    HeaderType: UCHAR,
    MarkerFlags: UCHAR,
}}
UNION!{union EVENT_INSTANCE_HEADER_u1 {
    [u16; 1],
    FieldTypeFlags FieldTypeFlags_mut: USHORT,
    s s_mut: EVENT_INSTANCE_HEADER_u1_s,
}}
STRUCT!{struct EVENT_INSTANCE_HEADER_u2_CLASS {
    Type: UCHAR,
    Level: UCHAR,
    Version: USHORT,
}}
UNION!{union EVENT_INSTANCE_HEADER_u2 {
    [u32; 1],
    Version Version_mut: ULONG,
    Class Class_mut: EVENT_INSTANCE_HEADER_u2_CLASS,
}}
STRUCT!{struct EVENT_INSTANCE_HEADER_u3_s1 {
    KernelTime: ULONG,
    UserTime: ULONG,
}}
STRUCT!{struct EVENT_INSTANCE_HEADER_u3_s2 {
    EventId: ULONG,
    Flags: ULONG,
}}
UNION!{union EVENT_INSTANCE_HEADER_u3 {
    [u64; 1],
    s1 s1_mut: EVENT_INSTANCE_HEADER_u3_s1,
    ProcessorTime ProcessorTime_mut: ULONG64,
    s2 s2_mut: EVENT_INSTANCE_HEADER_u3_s2,
}}
STRUCT!{struct EVENT_INSTANCE_HEADER {
    Size: USHORT,
    u1: EVENT_INSTANCE_HEADER_u1,
    u2: EVENT_INSTANCE_HEADER_u2,
    ThreadId: ULONG,
    ProcessId: ULONG,
    TimeStamp: LARGE_INTEGER,
    RegHandle: ULONGLONG,
    InstanceId: ULONG,
    ParentInstanceId: ULONG,
    u3: EVENT_INSTANCE_HEADER_u3,
    ParentRegHandle: ULONGLONG,
}}
pub type PEVENT_INSTANCE_HEADER = *mut EVENT_INSTANCE_HEADER;
pub const ETW_NULL_TYPE_VALUE: ULONG = 0;
pub const ETW_OBJECT_TYPE_VALUE: ULONG = 1;
pub const ETW_STRING_TYPE_VALUE: ULONG = 2;
pub const ETW_SBYTE_TYPE_VALUE: ULONG = 3;
pub const ETW_BYTE_TYPE_VALUE: ULONG = 4;
pub const ETW_INT16_TYPE_VALUE: ULONG = 5;
pub const ETW_UINT16_TYPE_VALUE: ULONG = 6;
pub const ETW_INT32_TYPE_VALUE: ULONG = 7;
pub const ETW_UINT32_TYPE_VALUE: ULONG = 8;
pub const ETW_INT64_TYPE_VALUE: ULONG = 9;
pub const ETW_UINT64_TYPE_VALUE: ULONG = 10;
pub const ETW_CHAR_TYPE_VALUE: ULONG = 11;
pub const ETW_SINGLE_TYPE_VALUE: ULONG = 12;
pub const ETW_DOUBLE_TYPE_VALUE: ULONG = 13;
pub const ETW_BOOLEAN_TYPE_VALUE: ULONG = 14;
pub const ETW_DECIMAL_TYPE_VALUE: ULONG = 15;
pub const ETW_GUID_TYPE_VALUE: ULONG = 101;
pub const ETW_ASCIICHAR_TYPE_VALUE: ULONG = 102;
pub const ETW_ASCIISTRING_TYPE_VALUE: ULONG = 103;
pub const ETW_COUNTED_STRING_TYPE_VALUE: ULONG = 104;
pub const ETW_POINTER_TYPE_VALUE: ULONG = 105;
pub const ETW_SIZET_TYPE_VALUE: ULONG = 106;
pub const ETW_HIDDEN_TYPE_VALUE: ULONG = 107;
pub const ETW_BOOL_TYPE_VALUE: ULONG = 108;
pub const ETW_COUNTED_ANSISTRING_TYPE_VALUE: ULONG = 109;
pub const ETW_REVERSED_COUNTED_STRING_TYPE_VALUE: ULONG = 110;
pub const ETW_REVERSED_COUNTED_ANSISTRING_TYPE_VALUE: ULONG = 111;
pub const ETW_NON_NULL_TERMINATED_STRING_TYPE_VALUE: ULONG = 112;
pub const ETW_REDUCED_ANSISTRING_TYPE_VALUE: ULONG = 113;
pub const ETW_REDUCED_STRING_TYPE_VALUE: ULONG = 114;
pub const ETW_SID_TYPE_VALUE: ULONG = 115;
pub const ETW_VARIANT_TYPE_VALUE: ULONG = 116;
pub const ETW_PTVECTOR_TYPE_VALUE: ULONG = 117;
pub const ETW_WMITIME_TYPE_VALUE: ULONG = 118;
pub const ETW_DATETIME_TYPE_VALUE: ULONG = 119;
pub const ETW_REFRENCE_TYPE_VALUE: ULONG = 120;
// TODO: DEFINE_TRACE_MOF_FIELD
STRUCT!{struct MOF_FIELD {
    DataPtr: ULONG64,
    Length: ULONG,
    DataType: ULONG,
}}
pub type PMOF_FIELD = *mut MOF_FIELD;
STRUCT!{struct TRACE_LOGFILE_HEADER_u1_VERSIONDETAIL {
    MajorVersion: UCHAR,
    MinorVersion: UCHAR,
    SubVersion: UCHAR,
    SubMinorVersion: UCHAR,
}}
UNION!{union TRACE_LOGFILE_HEADER_u1 {
    [u32; 1],
    Version Version_mut: ULONG,
    VersionDetail VersionDetail_mut: TRACE_LOGFILE_HEADER_u1_VERSIONDETAIL,
}}
STRUCT!{struct TRACE_LOGFILE_HEADER_u2_s {
    StartBuffers: ULONG,
    PointerSize: ULONG,
    EventsLost: ULONG,
    CpuSpeedInMHz: ULONG,
}}
UNION!{union TRACE_LOGFILE_HEADER_u2 {
    [u32; 4],
    LogInstanceGuid LogInstanceGuid_mut: GUID,
    s s_mut: TRACE_LOGFILE_HEADER_u2_s,
}}
STRUCT!{struct TRACE_LOGFILE_HEADER {
    BufferSize: ULONG,
    u1: TRACE_LOGFILE_HEADER_u1,
    ProviderVersion: ULONG,
    NumberOfProcessors: ULONG,
    EndTime: LARGE_INTEGER,
    TimerResolution: ULONG,
    MaximumFileSize: ULONG,
    LogFileMode: ULONG,
    BuffersWritten: ULONG,
    u2: TRACE_LOGFILE_HEADER_u2,
    LoggerName: LPWSTR,
    LogFileName: LPWSTR,
    TimeZone: TIME_ZONE_INFORMATION,
    BootTime: LARGE_INTEGER,
    PrefFreq: LARGE_INTEGER,
    StartTime: LARGE_INTEGER,
    ReservedFlags: ULONG,
    BuffersLost: ULONG,
}}
pub type PTRACE_LOGFILE_HEADER = *mut TRACE_LOGFILE_HEADER;
STRUCT!{struct TRACE_LOGFILE_HEADER32 {
    BufferSize: ULONG,
    u1: TRACE_LOGFILE_HEADER_u1,
    ProviderVersion: ULONG,
    NumberOfProcessors: ULONG,
    EndTime: LARGE_INTEGER,
    TimerResolution: ULONG,
    MaximumFileSize: ULONG,
    LogFileMode: ULONG,
    BuffersWritten: ULONG,
    u2: TRACE_LOGFILE_HEADER_u2,
    LoggerName: ULONG32,
    LogFileName: ULONG32,
    TimeZone: TIME_ZONE_INFORMATION,
    BootTime: LARGE_INTEGER,
    PrefFreq: LARGE_INTEGER,
    StartTime: LARGE_INTEGER,
    ReservedFlags: ULONG,
    BuffersLost: ULONG,
}}
pub type PTRACE_LOGFILE_HEADER32 = *mut TRACE_LOGFILE_HEADER32;
STRUCT!{struct TRACE_LOGFILE_HEADER64 {
    BufferSize: ULONG,
    u1: TRACE_LOGFILE_HEADER_u1,
    ProviderVersion: ULONG,
    NumberOfProcessors: ULONG,
    EndTime: LARGE_INTEGER,
    TimerResolution: ULONG,
    MaximumFileSize: ULONG,
    LogFileMode: ULONG,
    BuffersWritten: ULONG,
    u2: TRACE_LOGFILE_HEADER_u2,
    LoggerName: ULONG64,
    LogFileName: ULONG64,
    TimeZone: TIME_ZONE_INFORMATION,
    BootTime: LARGE_INTEGER,
    PrefFreq: LARGE_INTEGER,
    StartTime: LARGE_INTEGER,
    ReservedFlags: ULONG,
    BuffersLost: ULONG,
}}
pub type PTRACE_LOGFILE_HEADER64 = *mut TRACE_LOGFILE_HEADER64;
STRUCT!{struct EVENT_INSTANCE_INFO {
    RegHandle: HANDLE,
    InstanceId: ULONG,
}}
pub type PEVENT_INSTANCE_INFO = *mut EVENT_INSTANCE_INFO;
UNION!{union EVENT_TRACE_PROPERTIES_u {
    [u32; 1],
    AgeLimit AgeLimit_mut: LONG,
    FlushThreshold FlushThreshold_mut: LONG,
}}
STRUCT!{struct EVENT_TRACE_PROPERTIES {
    Wnode: WNODE_HEADER,
    BufferSize: ULONG,
    MinimumBuffers: ULONG,
    MaximumBuffers: ULONG,
    MaximumFileSize: ULONG,
    LogFileMode: ULONG,
    FlushTimer: ULONG,
    EnableFlags: ULONG,
    u: EVENT_TRACE_PROPERTIES_u,
    NumberOfBuffers: ULONG,
    FreeBuffers: ULONG,
    EventsLost: ULONG,
    BuffersWritten: ULONG,
    LogBuffersLost: ULONG,
    RealTimeBuffersLost: ULONG,
    LoggerThreadId: HANDLE,
    LogFileNameOffset: ULONG,
    LoggerNameOffset: ULONG,
}}
pub type PEVENT_TRACE_PROPERTIES = *mut EVENT_TRACE_PROPERTIES;
UNION!{union EVENT_TRACE_PROPERTIES_V2_u1 {
    [u32; 1],
    AgeLimit AgeLimit_mut: LONG,
    FlushThreshold FlushThreshold_mut: LONG,
}}
STRUCT!{struct EVENT_TRACE_PROPERTIES_V2_u2_s {
    bitfield: ULONG,
}}
BITFIELD!{EVENT_TRACE_PROPERTIES_V2_u2_s bitfield: ULONG [
    VersionNumber set_VersionNumber[0..8],
]}
UNION!{union EVENT_TRACE_PROPERTIES_V2_u2 {
    [u32; 1],
    s s_mut: EVENT_TRACE_PROPERTIES_V2_u2_s,
    V2Control V2Control_mut: ULONG,
}}
STRUCT!{struct EVENT_TRACE_PROPERTIES_V2_u3_s {
    bitfield: ULONG,
}}
BITFIELD!{EVENT_TRACE_PROPERTIES_V2_u3_s bitfield: ULONG [
    Wow set_Wow[0..1],
]}
UNION!{union EVENT_TRACE_PROPERTIES_V2_u3 {
    [u64; 1],
    s s_mut: EVENT_TRACE_PROPERTIES_V2_u3_s,
    V2Options V2Options_mut: ULONG64,
}}
STRUCT!{struct EVENT_TRACE_PROPERTIES_V2 {
    Wnode: WNODE_HEADER,
    BufferSize: ULONG,
    MinimumBuffers: ULONG,
    MaximumBuffers: ULONG,
    MaximumFileSize: ULONG,
    LogFileMode: ULONG,
    FlushTimer: ULONG,
    EnableFlags: ULONG,
    u1: EVENT_TRACE_PROPERTIES_u,
    NumberOfBuffers: ULONG,
    FreeBuffers: ULONG,
    EventsLost: ULONG,
    BuffersWritten: ULONG,
    LogBuffersLost: ULONG,
    RealTimeBuffersLost: ULONG,
    LoggerThreadId: HANDLE,
    LogFileNameOffset: ULONG,
    LoggerNameOffset: ULONG,
    u2: EVENT_TRACE_PROPERTIES_V2_u2,
    FilterDescCount: ULONG,
    FilterDesc: PEVENT_FILTER_DESCRIPTOR,
    u3: EVENT_TRACE_PROPERTIES_V2_u3,
}}
pub type PEVENT_TRACE_PROPERTIES_V2 = *mut EVENT_TRACE_PROPERTIES_V2;
STRUCT!{struct TRACE_GUID_REGISTRATION {
    Guid: LPCGUID,
    RegHandle: HANDLE,
}}
pub type PTRACE_GUID_REGISTRATION = *mut TRACE_GUID_REGISTRATION;
STRUCT!{struct TRACE_GUID_PROPERTIES {
    Guid: GUID,
    GuidType: ULONG,
    LoggerId: ULONG,
    EnableLevel: ULONG,
    EnableFlags: ULONG,
    IsEnable: BOOLEAN,
}}
pub type PTRACE_GUID_PROPERTIES = *mut TRACE_GUID_PROPERTIES;
STRUCT!{struct ETW_BUFFER_CONTEXT_u_s {
    ProcessorNumber: UCHAR,
    Alignment: UCHAR,
}}
UNION!{union ETW_BUFFER_CONTEXT_u {
    [u16; 1],
    s s_mut: ETW_BUFFER_CONTEXT_u_s,
    ProcessorIndex ProcessorIndex_mut: USHORT,
}}
STRUCT!{struct ETW_BUFFER_CONTEXT {
    u: ETW_BUFFER_CONTEXT_u,
    LoggerId: USHORT,
}}
pub type PETW_BUFFER_CONTEXT = *mut ETW_BUFFER_CONTEXT;
pub const TRACE_PROVIDER_FLAG_LEGACY: ULONG = 0x00000001;
pub const TRACE_PROVIDER_FLAG_PRE_ENABLE: ULONG = 0x00000002;
STRUCT!{struct TRACE_ENABLE_INFO {
    IsEnabled: ULONG,
    Level: UCHAR,
    Reserved1: UCHAR,
    LoggerId: USHORT,
    EnabledProperty: ULONG,
    Reserved2: ULONG,
    MatchAnyKeyword: ULONGLONG,
    MatchAllKeyword: ULONGLONG,
}}
pub type PTRACE_ENABLE_INFO = *mut TRACE_ENABLE_INFO;
STRUCT!{struct TRACE_PROVIDER_INSTANCE_INFO {
    NameOffset: ULONG,
    EnableCount: ULONG,
    Pid: ULONG,
    Flags: ULONG,
}}
pub type PTRACE_PROVIDER_INSTANCE_INFO = *mut TRACE_PROVIDER_INSTANCE_INFO;
STRUCT!{struct TRACE_GUID_INFO {
    InstanceCount: ULONG,
    Reserved: ULONG,
}}
pub type PTRACE_GUID_INFO = *mut TRACE_GUID_INFO;
STRUCT!{struct PROFILE_SOURCE_INFO {
    NextEntryOffset: ULONG,
    Source: ULONG,
    MinInterval: ULONG,
    MaxInterval: ULONG,
    Reserved: ULONG64,
    Description: [WCHAR; ANYSIZE_ARRAY],
}}
pub type PPROFILE_SOURCE_INFO = *mut PROFILE_SOURCE_INFO;
UNION!{union EVENT_TRACE_u {
    [u32; 1],
    ClientContext ClientContext_mut: ULONG,
    BufferContext BufferContext_mut: ETW_BUFFER_CONTEXT,
}}
STRUCT!{struct EVENT_TRACE {
    Header: EVENT_TRACE_HEADER,
    InstanceId: ULONG,
    ParentInstanceId: ULONG,
    ParentGuid: GUID,
    MofData: PVOID,
    MofLength: ULONG,
    u: EVENT_TRACE_u,
}}
pub type PEVENT_TRACE = *mut EVENT_TRACE;
pub const EVENT_CONTROL_CODE_DISABLE_PROVIDER: ULONG = 0;
pub const EVENT_CONTROL_CODE_ENABLE_PROVIDER: ULONG = 1;
pub const EVENT_CONTROL_CODE_CAPTURE_STATE: ULONG = 2;
FN!{stdcall PEVENT_TRACE_BUFFER_CALLBACKW(
    PEVENT_TRACE_LOGFILEW,
) -> ULONG}
FN!{stdcall PEVENT_TRACE_BUFFER_CALLBACKA(
    PEVENT_TRACE_LOGFILEA,
) -> ULONG}
FN!{stdcall PEVENT_CALLBACK(
    pEvent: PEVENT_TRACE,
) -> ()}
FN!{stdcall PEVENT_RECORD_CALLBACK(
    EventRecord: PEVENT_RECORD,
) -> ()}
FN!{stdcall WMIDPREQUEST(
    RequestCode: WMIDPREQUESTCODE,
    RequestContext: PVOID,
    BufferSize: *mut ULONG,
    Buffer: PVOID,
) -> ULONG}
UNION!{union EVENT_TRACE_LOGFILE_u1 {
    [u32; 1],
    LogFileMode LogFileMode_mut: ULONG,
    ProcessTraceMode ProcessTraceMode_mut: ULONG,
}}
UNION!{union EVENT_TRACE_LOGFILE_u2 {
    [u32; 1] [u64; 1],
    EventCallback EventCallback_mut: PEVENT_CALLBACK,
    EventRecordCallback EventRecordCallback_mut: PEVENT_RECORD_CALLBACK,
}}
STRUCT!{struct EVENT_TRACE_LOGFILEW {
    LogFileName: LPWSTR,
    LoggerName: LPWSTR,
    CurrentTime: LONGLONG,
    BuffersRead: ULONG,
    u1: EVENT_TRACE_LOGFILE_u1,
    CurrentEvent: EVENT_TRACE,
    LogfileHeader: TRACE_LOGFILE_HEADER,
    BufferCallback: PEVENT_TRACE_BUFFER_CALLBACKW,
    BufferSize: ULONG,
    Filled: ULONG,
    EventsLost: ULONG,
    u2: EVENT_TRACE_LOGFILE_u2,
    IsKernelTrace: ULONG,
    Context: PVOID,
}}
pub type PEVENT_TRACE_LOGFILEW = *mut EVENT_TRACE_LOGFILEW;
STRUCT!{struct EVENT_TRACE_LOGFILEA {
    LogFileName: LPSTR,
    LoggerName: LPSTR,
    CurrentTime: LONGLONG,
    BuffersRead: ULONG,
    u1: EVENT_TRACE_LOGFILE_u1,
    CurrentEvent: EVENT_TRACE,
    LogfileHeader: TRACE_LOGFILE_HEADER,
    BufferCallback: PEVENT_TRACE_BUFFER_CALLBACKA,
    BufferSize: ULONG,
    Filled: ULONG,
    EventsLost: ULONG,
    u2: EVENT_TRACE_LOGFILE_u2,
    IsKernelTrace: ULONG,
    Context: PVOID,
}}
pub type PEVENT_TRACE_LOGFILEA = *mut EVENT_TRACE_LOGFILEA;
extern "system" {
    pub fn StartTraceW(
        SessionHandle: PTRACEHANDLE,
        SessionName: LPCWSTR,
        Properties: PEVENT_TRACE_PROPERTIES,
    ) -> ULONG;
    pub fn StartTraceA(
        SessionHandle: PTRACEHANDLE,
        SessionName: LPCSTR,
        Properties: PEVENT_TRACE_PROPERTIES,
    ) -> ULONG;
    pub fn StopTraceW(
        SessionHandle: TRACEHANDLE,
        SessionName: LPCWSTR,
        Properties: PEVENT_TRACE_PROPERTIES,
    ) -> ULONG;
    pub fn StopTraceA(
        SessionHandle: TRACEHANDLE,
        SessionName: LPCSTR,
        Properties: PEVENT_TRACE_PROPERTIES,
    ) -> ULONG;
    pub fn QueryTraceW(
        SessionHandle: TRACEHANDLE,
        SessionName: LPCWSTR,
        Properties: PEVENT_TRACE_PROPERTIES,
    ) -> ULONG;
    pub fn QueryTraceA(
        SessionHandle: TRACEHANDLE,
        SessionName: LPCSTR,
        Properties: PEVENT_TRACE_PROPERTIES,
    ) -> ULONG;
    pub fn UpdateTraceW(
        SessionHandle: TRACEHANDLE,
        SessionName: LPCWSTR,
        Properties: PEVENT_TRACE_PROPERTIES,
    ) -> ULONG;
    pub fn UpdateTraceA(
        SessionHandle: TRACEHANDLE,
        SessionName: LPCSTR,
        Properties: PEVENT_TRACE_PROPERTIES,
    ) -> ULONG;
    pub fn FlushTraceW(
        SessionHandle: TRACEHANDLE,
        SessionName: LPCWSTR,
        Properties: PEVENT_TRACE_PROPERTIES,
    ) -> ULONG;
    pub fn FlushTraceA(
        SessionHandle: TRACEHANDLE,
        SessionName: LPCSTR,
        Properties: PEVENT_TRACE_PROPERTIES,
    ) -> ULONG;
    pub fn ControlTraceW(
        SessionHandle: TRACEHANDLE,
        SessionName: LPCWSTR,
        Properties: PEVENT_TRACE_PROPERTIES,
        ControlCode: ULONG,
    ) -> ULONG;
    pub fn ControlTraceA(
        SessionHandle: TRACEHANDLE,
        SessionName: LPCSTR,
        Properties: PEVENT_TRACE_PROPERTIES,
        ControlCode: ULONG,
    ) -> ULONG;
    pub fn QueryAllTracesW(
        PropertyArray: *mut PEVENT_TRACE_PROPERTIES,
        PropertyArrayCount: ULONG,
        SessionCount: PULONG,
    ) -> ULONG;
    pub fn QueryAllTracesA(
        PropertyArray: *mut PEVENT_TRACE_PROPERTIES,
        PropertyArrayCount: ULONG,
        SessionCount: PULONG,
    ) -> ULONG;
    pub fn EnableTrace(
        Enable: ULONG,
        EnableFlag: ULONG,
        EnableLevel: ULONG,
        ControlGuid: LPCGUID,
        SessionHandle: TRACEHANDLE,
    ) -> ULONG;
    pub fn EnableTraceEx(
        ProviderId: LPCGUID,
        SourceId: LPCGUID,
        TraceHandle: TRACEHANDLE,
        IsEnabled: ULONG,
        Level: UCHAR,
        MatchAnyKeyword: ULONGLONG,
        MatchAllKeyword: ULONGLONG,
        EnableProperty: ULONG,
        EnableFilterDesc: PEVENT_FILTER_DESCRIPTOR,
    ) -> ULONG;
}
pub const ENABLE_TRACE_PARAMETERS_VERSION: ULONG = 1;
pub const ENABLE_TRACE_PARAMETERS_VERSION_2: ULONG = 2;
STRUCT!{struct ENABLE_TRACE_PARAMETERS_V1 {
    Version: ULONG,
    EnableProperty: ULONG,
    ControlFlags: ULONG,
    SourceId: GUID,
    EnableFilterDesc: PEVENT_FILTER_DESCRIPTOR,
}}
pub type PENABLE_TRACE_PARAMETERS_V1 = *mut ENABLE_TRACE_PARAMETERS_V1;
STRUCT!{struct ENABLE_TRACE_PARAMETERS {
    Version: ULONG,
    EnableProperty: ULONG,
    ControlFlags: ULONG,
    SourceId: GUID,
    EnableFilterDesc: PEVENT_FILTER_DESCRIPTOR,
    FilterDescCount: ULONG,
}}
pub type PENABLE_TRACE_PARAMETERS = *mut ENABLE_TRACE_PARAMETERS;
extern "system" {
    pub fn EnableTraceEx2(
        TraceHandle: TRACEHANDLE,
        ProviderId: LPCGUID,
        ControlCode: ULONG,
        Level: UCHAR,
        MatchAnyKeyword: ULONGLONG,
        MatchAllKeyword: ULONGLONG,
        Timeout: ULONG,
        EnableParameters: PENABLE_TRACE_PARAMETERS,
    ) -> ULONG;
}
ENUM!{enum TRACE_QUERY_INFO_CLASS {
    TraceGuidQueryList,
    TraceGuidQueryInfo,
    TraceGuidQueryProcess,
    TraceStackTracingInfo,
    TraceSystemTraceEnableFlagsInfo,
    TraceSampledProfileIntervalInfo,
    TraceProfileSourceConfigInfo,
    TraceProfileSourceListInfo,
    TracePmcEventListInfo,
    TracePmcCounterListInfo,
    TraceSetDisallowList,
    TraceVersionInfo,
    TraceGroupQueryList,
    TraceGroupQueryInfo,
    TraceDisallowListQuery,
    TraceCompressionInfo,
    TracePeriodicCaptureStateListInfo,
    TracePeriodicCaptureStateInfo,
    TraceProviderBinaryTracking,
    TraceMaxLoggersQuery,
    MaxTraceSetInfoClass,
}}
pub type TRACE_INFO_CLASS = TRACE_QUERY_INFO_CLASS;
extern "system" {
    pub fn EnumerateTraceGuidsEx(
        TraceQueryInfoClass: TRACE_QUERY_INFO_CLASS,
        InBuffer: PVOID,
        InBufferSize: ULONG,
        OutBuffer: PVOID,
        OutBufferSize: ULONG,
        ReturnLength: PULONG,
    ) -> ULONG;
}
STRUCT!{struct CLASSIC_EVENT_ID {
    EventGuid: GUID,
    Type: UCHAR,
    Reserved: [UCHAR; 7],
}}
pub type PCLASSIC_EVENT_ID = *mut CLASSIC_EVENT_ID;
STRUCT!{struct TRACE_PROFILE_INTERVAL {
    Source: ULONG,
    Interval: ULONG,
}}
pub type PTRACE_PROFILE_INTERVAL = *mut TRACE_PROFILE_INTERVAL;
STRUCT!{struct TRACE_VERSION_INFO {
    EtwTraceProcessingVersion: UINT,
    Reserved: UINT,
}}
pub type PTRACE_VERSION_INFO = *mut TRACE_VERSION_INFO;
STRUCT!{struct TRACE_PERIODIC_CAPTURE_STATE_INFO {
    CaptureStateFrequencyInSeconds: ULONG,
    ProviderCount: USHORT,
    Reserved: USHORT,
}}
pub type PTRACE_PERIODIC_CAPTURE_STATE_INFO = *mut TRACE_PERIODIC_CAPTURE_STATE_INFO;
extern "system" {
    pub fn TraceSetInformation(
        SessionHandle: TRACEHANDLE,
        InformationClass: TRACE_INFO_CLASS,
        TraceInformation: PVOID,
        InformationLength: ULONG,
    ) -> ULONG;
    pub fn TraceQueryInformation(
        SessionHandle: TRACEHANDLE,
        InformationClass: TRACE_QUERY_INFO_CLASS,
        TraceInformation: PVOID,
        InformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> ULONG;
    pub fn CreateTraceInstanceId(
        RegHandle: HANDLE,
        pInstInfo: PEVENT_INSTANCE_INFO,
    ) -> ULONG;
    pub fn TraceEvent(
        SessionHandle: TRACEHANDLE,
        EventTrace: PEVENT_TRACE_HEADER,
    ) -> ULONG;
    pub fn TraceEventInstance(
        SessionHandle: TRACEHANDLE,
        EventTrace: PEVENT_TRACE_HEADER,
        pInstInfo: PEVENT_INSTANCE_INFO,
        pParentInstInfo: PEVENT_INSTANCE_INFO,
    ) -> ULONG;
    pub fn RegisterTraceGuidsW(
        RequestAddress: WMIDPREQUEST,
        RequestContext: PVOID,
        ControlGuid: LPCGUID,
        GuidCount: ULONG,
        TraceGuidReg: PTRACE_GUID_REGISTRATION,
        MofImagePath: LPCWSTR,
        MofResourceName: LPCWSTR,
        RegistrationHandle: PTRACEHANDLE,
    ) -> ULONG;
    pub fn RegisterTraceGuidsA(
        RequestAddress: WMIDPREQUEST,
        RequestContext: PVOID,
        ControlGuid: LPCGUID,
        GuidCount: ULONG,
        TraceGuidReg: PTRACE_GUID_REGISTRATION,
        MofImagePath: LPCSTR,
        MofResourceName: LPCSTR,
        RegistrationHandle: PTRACEHANDLE,
    ) -> ULONG;
    pub fn EnumerateTraceGuids(
        GuidPropertiesArray: *mut PTRACE_GUID_PROPERTIES,
        PropertyArrayCount: ULONG,
        GuidCount: PULONG,
    ) -> ULONG;
    pub fn UnregisterTraceGuids(
        RegistrationHandle: TRACEHANDLE,
    ) -> ULONG;
    pub fn GetTraceLoggerHandle(
        Buffer: PVOID,
    ) -> TRACEHANDLE;
    pub fn GetTraceEnableLevel(
        SessionHandle: TRACEHANDLE,
    ) -> UCHAR;
    pub fn GetTraceEnableFlags(
        SessionHandle: TRACEHANDLE,
    ) -> ULONG;
    pub fn OpenTraceW(
        Logfile: PEVENT_TRACE_LOGFILEW,
    ) -> TRACEHANDLE;
    pub fn ProcessTrace(
        HandleArray: PTRACEHANDLE,
        HandleCount: ULONG,
        StartTime: LPFILETIME,
        EndTime: LPFILETIME,
    ) -> ULONG;
    pub fn CloseTrace(
        TraceHandle: TRACEHANDLE,
    ) -> ULONG;
}
ENUM!{enum ETW_PROCESS_HANDLE_INFO_TYPE {
    EtwQueryPartitionInformation = 1,
    EtwQueryProcessHandleInfoMax,
}}
STRUCT!{struct ETW_TRACE_PARTITION_INFORMATION {
    PartitionId: GUID,
    ParentId: GUID,
    Reserved: ULONG64,
    PartitionType: ULONG,
}}
pub type PETW_TRACE_PARTITION_INFORMATION = *mut ETW_TRACE_PARTITION_INFORMATION;
extern "system" {
    pub fn QueryTraceProcessingHandle(
        ProcessingHandle: TRACEHANDLE,
        InformationClass: ETW_PROCESS_HANDLE_INFO_TYPE,
        InBuffer: PVOID,
        InBufferSize: ULONG,
        OutBuffer: PVOID,
        OutBufferSize: ULONG,
        ReturnLength: PULONG,
    ) -> ULONG;
    pub fn OpenTraceA(
        Logfile: PEVENT_TRACE_LOGFILEA,
    ) -> TRACEHANDLE;
    pub fn SetTraceCallback(
        pGuid: LPCGUID,
        EventCallback: PEVENT_CALLBACK,
    ) -> ULONG;
    pub fn RemoveTraceCallback(
        pGuid: LPCGUID,
    ) -> ULONG;
}
extern "C" {
    pub fn TraceMessage(
        SessionHandle: TRACEHANDLE,
        MessageFlags: ULONG,
        MessageGuid: LPGUID,
        MessageNumber: USHORT,
        ...
    ) -> ULONG;
    pub fn TraceMessageVa(
        SessionHandle: TRACEHANDLE,
        MessageFlags: ULONG,
        MessageGuid: LPGUID,
        MessageNumber: USHORT,
        MessageArgList: va_list,
    );
}
pub const INVALID_PROCESSTRACE_HANDLE: TRACEHANDLE = INVALID_HANDLE_VALUE as TRACEHANDLE;
