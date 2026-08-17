use winapi::shared::ntdef::{HANDLE, LONG, NTSTATUS, ULONG, ULONGLONG, USHORT};
use winapi::shared::ntstatus::FACILITY_NTWIN32;
pub type KPRIORITY = LONG;
pub type RTL_ATOM = USHORT;
pub type PRTL_ATOM = *mut RTL_ATOM;
pub const NT_FACILITY_MASK: ULONG = 0xfff;
pub const NT_FACILITY_SHIFT: ULONG = 16;
#[inline]
pub const fn NT_FACILITY(Status: NTSTATUS) -> ULONG {
    (Status as u32) >> NT_FACILITY_SHIFT & NT_FACILITY_MASK
}
#[inline]
pub const fn NT_NTWIN32(Status: NTSTATUS) -> bool {
    NT_FACILITY(Status) == FACILITY_NTWIN32 as u32
}
#[inline]
pub const fn WIN32_FROM_NTSTATUS(Status: NTSTATUS) -> ULONG {
    (Status as u32) & 0xffff
}
STRUCT!{struct CLIENT_ID {
    UniqueProcess: HANDLE,
    UniqueThread: HANDLE,
}}
pub type PCLIENT_ID = *mut CLIENT_ID;
STRUCT!{struct CLIENT_ID32 {
    UniqueProcess: ULONG,
    UniqueThread: ULONG,
}}
pub type PCLIENT_ID32 = *mut CLIENT_ID32;
STRUCT!{struct CLIENT_ID64 {
    UniqueProcess: ULONGLONG,
    UniqueThread: ULONGLONG,
}}
pub type PCLIENT_ID64 = *mut CLIENT_ID64;
STRUCT!{struct KSYSTEM_TIME {
    LowPart: ULONG,
    High1Time: LONG,
    High2Time: LONG,
}}
pub type PKSYSTEM_TIME = *mut KSYSTEM_TIME;
