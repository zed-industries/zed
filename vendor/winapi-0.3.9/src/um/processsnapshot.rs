// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Defines the process snapshot API
use ctypes::c_void;
use shared::basetsd::ULONG_PTR;
use shared::minwindef::DWORD;
use um::winnt::HANDLE;
ENUM!{enum PSS_CAPTURE_FLAGS {
    PSS_CAPTURE_NONE = 0x00000000,
    PSS_CAPTURE_VA_CLONE = 0x00000001,
    PSS_CAPTURE_RESERVED_00000002 = 0x00000002,
    PSS_CAPTURE_HANDLES = 0x00000004,
    PSS_CAPTURE_HANDLE_NAME_INFORMATION = 0x00000008,
    PSS_CAPTURE_HANDLE_BASIC_INFORMATION = 0x00000010,
    PSS_CAPTURE_HANDLE_TYPE_SPECIFIC_INFORMATION = 0x00000020,
    PSS_CAPTURE_HANDLE_TRACE = 0x00000040,
    PSS_CAPTURE_THREADS = 0x00000080,
    PSS_CAPTURE_THREAD_CONTEXT = 0x00000100,
    PSS_CAPTURE_THREAD_CONTEXT_EXTENDED = 0x00000200,
    PSS_CAPTURE_RESERVED_00000400 = 0x00000400,
    PSS_CAPTURE_VA_SPACE = 0x00000800,
    PSS_CAPTURE_VA_SPACE_SECTION_INFORMATION = 0x00001000,
    PSS_CREATE_BREAKAWAY_OPTIONAL = 0x04000000,
    PSS_CREATE_BREAKAWAY = 0x08000000,
    PSS_CREATE_FORCE_BREAKAWAY = 0x10000000,
    PSS_CREATE_USE_VM_ALLOCATIONS = 0x20000000,
    PSS_CREATE_MEASURE_PERFORMANCE = 0x40000000,
    PSS_CREATE_RELEASE_SECTION = 0x80000000,
}}
ENUM!{enum PSS_QUERY_INFORMATION_CLASS {
    PSS_QUERY_PROCESS_INFORMATION = 0,
    PSS_QUERY_VA_CLONE_INFORMATION = 1,
    PSS_QUERY_AUXILIARY_PAGES_INFORMATION = 2,
    PSS_QUERY_VA_SPACE_INFORMATION = 3,
    PSS_QUERY_HANDLE_INFORMATION = 4,
    PSS_QUERY_THREAD_INFORMATION = 5,
    PSS_QUERY_HANDLE_TRACE_INFORMATION = 6,
    PSS_QUERY_PERFORMANCE_COUNTERS = 7,
}}
ENUM!{enum PSS_WALK_INFORMATION_CLASS {
    PSS_WALK_AUXILIARY_PAGES = 0,
    PSS_WALK_VA_SPACE = 1,
    PSS_WALK_HANDLES = 2,
    PSS_WALK_THREADS = 3,
}}
ENUM!{enum PSS_DUPLICATE_FLAGS {
    PSS_DUPLICATE_NONE = 0x00,
    PSS_DUPLICATE_CLOSE_SOURCE = 0x01,
}}
DECLARE_HANDLE!{HPSS, HPSS__}
DECLARE_HANDLE!{HPSSWALK, HPSSWALK__}
FN!{stdcall pAllocRoutine(
    Context: *mut c_void,
    Size: DWORD,
) -> *mut c_void}
FN!{stdcall pFreeRoutine(
    Context: *mut c_void,
    Address: *mut c_void,
) -> ()}
STRUCT!{struct PSS_ALLOCATOR {
    Context: *mut c_void,
    AllocRoutine: pAllocRoutine,
    FreeRoutine: pFreeRoutine,
}}
extern "system" {
    pub fn PssCaptureSnapshot(
        ProcessHandle: HANDLE,
        CaptureFlags: PSS_CAPTURE_FLAGS,
        ThreadContextFlags: DWORD,
        SnapshotHandle: *mut HPSS,
    ) -> DWORD;
    pub fn PssDuplicateSnapshot(
        SourceProcessHandle: HANDLE,
        SnapshotHandle: HPSS,
        TargetProcessHandle: HANDLE,
        TargetSnapshotHandle: *mut HPSS,
        Flags: PSS_DUPLICATE_FLAGS,
    ) -> DWORD;
    pub fn PssFreeSnapshot(
        ProcessHandle: HANDLE,
        SnapshotHandle: HPSS,
    ) -> DWORD;
    pub fn PssQuerySnapshot(
        SnapshotHandle: HPSS,
        InformationClass: PSS_QUERY_INFORMATION_CLASS,
        Buffer: *mut c_void,
        BufferLength: DWORD,
    ) -> DWORD;
    pub fn PssWalkMarkerCreate(
        Allocator: *const PSS_ALLOCATOR,
        WalkMarkerHandle: *mut HPSSWALK,
    ) -> DWORD;
    pub fn PssWalkMarkerFree(
        WalkMarkerHandle: HPSSWALK,
    ) -> DWORD;
    pub fn PssWalkMarkerGetPosition(
        WalkMarkerHandle: HPSSWALK,
        Position: *mut ULONG_PTR,
    ) -> DWORD;
    // pub fn PssWalkMarkerRewind();
    // pub fn PssWalkMarkerSeek();
    pub fn PssWalkMarkerSeekToBeginning(
        WalkMarkerHandle: HPSS,
    ) -> DWORD;
    pub fn PssWalkMarkerSetPosition(
        WalkMarkerHandle: HPSSWALK,
        Position: ULONG_PTR,
    ) -> DWORD;
    // pub fn PssWalkMarkerTell();
    pub fn PssWalkSnapshot(
        SnapshotHandle: HPSS,
        InformationClass: PSS_WALK_INFORMATION_CLASS,
        WalkMarkerHandle: HPSSWALK,
        Buffer: *mut c_void,
        BufferLength: DWORD,
    ) -> DWORD;
}
