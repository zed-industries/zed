// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::basetsd::ULONG64;
use shared::evntprov::EVENT_DESCRIPTOR;
use shared::evntrace::ETW_BUFFER_CONTEXT;
use shared::guiddef::{GUID, LPGUID};
use shared::minwindef::{PUCHAR, PULONG, PUSHORT, UCHAR, ULONG, USHORT};
use um::winnt::{
    ANYSIZE_ARRAY, BOOLEAN, LARGE_INTEGER, PCSTR, PSECURITY_DESCRIPTOR, PSID, PVOID, ULONGLONG,
};
pub const EVENT_HEADER_EXT_TYPE_RELATED_ACTIVITYID: USHORT = 0x0001;
pub const EVENT_HEADER_EXT_TYPE_SID: USHORT = 0x0002;
pub const EVENT_HEADER_EXT_TYPE_TS_ID: USHORT = 0x0003;
pub const EVENT_HEADER_EXT_TYPE_INSTANCE_INFO: USHORT = 0x0004;
pub const EVENT_HEADER_EXT_TYPE_STACK_TRACE32: USHORT = 0x0005;
pub const EVENT_HEADER_EXT_TYPE_STACK_TRACE64: USHORT = 0x0006;
pub const EVENT_HEADER_EXT_TYPE_PEBS_INDEX: USHORT = 0x0007;
pub const EVENT_HEADER_EXT_TYPE_PMC_COUNTERS: USHORT = 0x0008;
pub const EVENT_HEADER_EXT_TYPE_PSM_KEY: USHORT = 0x0009;
pub const EVENT_HEADER_EXT_TYPE_EVENT_KEY: USHORT = 0x000A;
pub const EVENT_HEADER_EXT_TYPE_EVENT_SCHEMA_TL: USHORT = 0x000B;
pub const EVENT_HEADER_EXT_TYPE_PROV_TRAITS: USHORT = 0x000C;
pub const EVENT_HEADER_EXT_TYPE_PROCESS_START_KEY: USHORT = 0x000D;
pub const EVENT_HEADER_EXT_TYPE_CONTROL_GUID: USHORT = 0x000E;
pub const EVENT_HEADER_EXT_TYPE_MAX: USHORT = 0x000F;
STRUCT!{struct EVENT_HEADER_EXTENDED_DATA_ITEM_s {
    bitfield: USHORT,
}}
BITFIELD!{EVENT_HEADER_EXTENDED_DATA_ITEM_s bitfield: USHORT [
    Linkage set_Linkage[0..1],
    Reserved2 set_Reserved2[1..16],
]}
STRUCT!{struct EVENT_HEADER_EXTENDED_DATA_ITEM {
    Reserved1: USHORT,
    ExtType: USHORT,
    s: EVENT_HEADER_EXTENDED_DATA_ITEM_s,
    DataSize: USHORT,
    DataPtr: ULONGLONG,
}}
pub type PEVENT_HEADER_EXTENDED_DATA_ITEM = *mut EVENT_HEADER_EXTENDED_DATA_ITEM;
STRUCT!{struct EVENT_EXTENDED_ITEM_INSTANCE {
    InstanceId: ULONG,
    ParentInstanceId: ULONG,
    ParentGuid: GUID,
}}
pub type PEVENT_EXTENDED_ITEM_INSTANCE = *mut EVENT_EXTENDED_ITEM_INSTANCE;
STRUCT!{struct EVENT_EXTENDED_ITEM_RELATED_ACTIVITYID {
    RelatedActivityId: GUID,
}}
pub type PEVENT_EXTENDED_ITEM_RELATED_ACTIVITYID = *mut EVENT_EXTENDED_ITEM_RELATED_ACTIVITYID;
STRUCT!{struct EVENT_EXTENDED_ITEM_TS_ID {
    SessionId: ULONG,
}}
pub type PEVENT_EXTENDED_ITEM_TS_ID = *mut EVENT_EXTENDED_ITEM_TS_ID;
STRUCT!{struct EVENT_EXTENDED_ITEM_STACK_TRACE32 {
    MatchId: ULONG64,
    Address: [ULONG; ANYSIZE_ARRAY],
}}
pub type PEVENT_EXTENDED_ITEM_STACK_TRACE32 = *mut EVENT_EXTENDED_ITEM_STACK_TRACE32;
STRUCT!{struct EVENT_EXTENDED_ITEM_STACK_TRACE64 {
    MatchId: ULONG64,
    Address: [ULONG64; ANYSIZE_ARRAY],
}}
pub type PEVENT_EXTENDED_ITEM_STACK_TRACE64 = *mut EVENT_EXTENDED_ITEM_STACK_TRACE64;
STRUCT!{struct EVENT_EXTENDED_ITEM_PEBS_INDEX {
    PebsIndex: ULONG64,
}}
pub type PEVENT_EXTENDED_ITEM_PEBS_INDEX = *mut EVENT_EXTENDED_ITEM_PEBS_INDEX;
STRUCT!{struct EVENT_EXTENDED_ITEM_PMC_COUNTERS {
    Counter: [ULONG64; ANYSIZE_ARRAY],
}}
pub type PEVENT_EXTENDED_ITEM_PMC_COUNTERS = *mut EVENT_EXTENDED_ITEM_PMC_COUNTERS;
STRUCT!{struct EVENT_EXTENDED_ITEM_PROCESS_START_KEY {
    ProcessStartKey: ULONG64,
}}
pub type PEVENT_EXTENDED_ITEM_PROCESS_START_KEY = *mut EVENT_EXTENDED_ITEM_PROCESS_START_KEY;
STRUCT!{struct EVENT_EXTENDED_ITEM_EVENT_KEY {
    Key: ULONG64,
}}
pub type PEVENT_EXTENDED_ITEM_EVENT_KEY = *mut EVENT_EXTENDED_ITEM_EVENT_KEY;
pub const EVENT_HEADER_PROPERTY_XML: USHORT = 0x0001;
pub const EVENT_HEADER_PROPERTY_FORWARDED_XML: USHORT = 0x0002;
pub const EVENT_HEADER_PROPERTY_LEGACY_EVENTLOG: USHORT = 0x0004;
pub const EVENT_HEADER_PROPERTY_RELOGGABLE: USHORT = 0x0008;
pub const EVENT_HEADER_FLAG_EXTENDED_INFO: USHORT = 0x0001;
pub const EVENT_HEADER_FLAG_PRIVATE_SESSION: USHORT = 0x0002;
pub const EVENT_HEADER_FLAG_STRING_ONLY: USHORT = 0x0004;
pub const EVENT_HEADER_FLAG_TRACE_MESSAGE: USHORT = 0x0008;
pub const EVENT_HEADER_FLAG_NO_CPUTIME: USHORT = 0x0010;
pub const EVENT_HEADER_FLAG_32_BIT_HEADER: USHORT = 0x0020;
pub const EVENT_HEADER_FLAG_64_BIT_HEADER: USHORT = 0x0040;
pub const EVENT_HEADER_FLAG_CLASSIC_HEADER: USHORT = 0x0100;
pub const EVENT_HEADER_FLAG_PROCESSOR_INDEX: USHORT = 0x0200;
STRUCT!{struct EVENT_HEADER_u_s {
    KernelTime: ULONG,
    UserTime: ULONG,
}}
UNION!{union EVENT_HEADER_u {
    [u64; 1],
    s s_mut: EVENT_HEADER_u_s,
    ProcessorTime ProcessorTime_mut: ULONG64,
}}
STRUCT!{struct EVENT_HEADER {
    Size: USHORT,
    HeaderType: USHORT,
    Flags: USHORT,
    EventProperty: USHORT,
    ThreadId: ULONG,
    ProcessId: ULONG,
    TimeStamp: LARGE_INTEGER,
    ProviderId: GUID,
    EventDescriptor: EVENT_DESCRIPTOR,
    u: EVENT_HEADER_u,
    ActivityId: GUID,
}}
pub type PEVENT_HEADER = *mut EVENT_HEADER;
STRUCT!{struct EVENT_RECORD {
    EventHeader: EVENT_HEADER,
    BufferContext: ETW_BUFFER_CONTEXT,
    ExtendedDataCount: USHORT,
    UserDataLength: USHORT,
    ExtendedData: PEVENT_HEADER_EXTENDED_DATA_ITEM,
    UserData: PVOID,
    UserContext: PVOID,
}}
pub type PEVENT_RECORD = *mut EVENT_RECORD;
pub type PCEVENT_RECORD = *const EVENT_RECORD;
pub const EVENT_ENABLE_PROPERTY_SID: USHORT = 0x00000001;
pub const EVENT_ENABLE_PROPERTY_TS_ID: USHORT = 0x00000002;
pub const EVENT_ENABLE_PROPERTY_STACK_TRACE: USHORT = 0x00000004;
pub const EVENT_ENABLE_PROPERTY_PSM_KEY: USHORT = 0x00000008;
pub const EVENT_ENABLE_PROPERTY_IGNORE_KEYWORD_0: USHORT = 0x00000010;
pub const EVENT_ENABLE_PROPERTY_PROVIDER_GROUP: USHORT = 0x00000020;
pub const EVENT_ENABLE_PROPERTY_ENABLE_KEYWORD_0: USHORT = 0x00000040;
pub const EVENT_ENABLE_PROPERTY_PROCESS_START_KEY: USHORT = 0x00000080;
pub const EVENT_ENABLE_PROPERTY_EVENT_KEY: USHORT = 0x00000100;
pub const EVENT_ENABLE_PROPERTY_EXCLUDE_INPRIVATE: USHORT = 0x00000200;
pub const PROCESS_TRACE_MODE_REAL_TIME: ULONG = 0x00000100;
pub const PROCESS_TRACE_MODE_RAW_TIMESTAMP: ULONG = 0x00001000;
pub const PROCESS_TRACE_MODE_EVENT_RECORD: ULONG = 0x10000000;
#[inline]
pub unsafe fn GetEventProcessorIndex(EventRecord: PCEVENT_RECORD) -> ULONG {
    if (*EventRecord).EventHeader.Flags & EVENT_HEADER_FLAG_PROCESSOR_INDEX != 0 {
        *(*EventRecord).BufferContext.u.ProcessorIndex() as ULONG
    } else {
        (*EventRecord).BufferContext.u.s().ProcessorNumber as ULONG
    }
}
ENUM!{enum ETW_PROVIDER_TRAIT_TYPE {
    EtwProviderTraitTypeGroup = 1,
    EtwProviderTraitDecodeGuid = 2,
    EtwProviderTraitTypeMax,
}}
#[inline]
unsafe fn strnlen(s: PCSTR, max_len: isize) -> isize {
    let mut len = 0;
    while *s.offset(len) != 0 && len < max_len {
        len += 1
    }
    len
}
// Taken from Rust 1.17.0 sources
#[inline]
unsafe fn read_unaligned<T>(src: *const T) -> T {
    use core::{mem, ptr};
    let mut tmp: T = mem::uninitialized();
    ptr::copy_nonoverlapping(
        src as *const u8,
        &mut tmp as *mut T as *mut u8,
        mem::size_of::<T>(),
    );
    tmp
}
#[inline]
pub unsafe fn EtwGetTraitFromProviderTraits(
    ProviderTraits: PVOID, TraitType: UCHAR, Trait: *mut PVOID, Size: PUSHORT,
) {
    use core::ptr::null_mut;
    let ByteCount = read_unaligned(ProviderTraits as *mut USHORT) as isize;
    let mut Ptr = ProviderTraits as PUCHAR;
    let PtrEnd = Ptr.offset(ByteCount);
    *Trait = null_mut();
    *Size = 0;
    if ByteCount < 3 {
        return;
    }
    Ptr = Ptr.offset(2);
    Ptr = Ptr.offset(strnlen(Ptr as PCSTR, (ByteCount - 3) as isize));
    Ptr = Ptr.offset(1);
    while Ptr < PtrEnd {
        let TraitByteCount = read_unaligned(Ptr as *const USHORT);
        if TraitByteCount < 3 {
            return;
        }
        if *Ptr.offset(2) == TraitType && Ptr.offset(TraitByteCount as isize) <= PtrEnd {
            *Trait = Ptr.offset(3) as PVOID;
            *Size = TraitByteCount - 3;
            return;
        }
        Ptr = Ptr.offset(TraitByteCount as isize);
    }
}
ENUM!{enum EVENTSECURITYOPERATION {
    EventSecuritySetDACL,
    EventSecuritySetSACL,
    EventSecurityAddDACL,
    EventSecurityAddSACL,
    EventSecurityMax,
}}
extern "system" {
    pub fn EventAccessControl(
        Guid: LPGUID,
        Operation: ULONG,
        Sid: PSID,
        Rights: ULONG,
        AllowOrDeny: BOOLEAN,
    ) -> ULONG;
    pub fn EventAccessQuery(
        Guid: LPGUID,
        Buffer: PSECURITY_DESCRIPTOR,
        BufferSize: PULONG,
    ) -> ULONG;
    pub fn EventAccessRemove(
        Guid: LPGUID,
    ) -> ULONG;
}
