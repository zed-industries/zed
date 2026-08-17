//! Contains bindings for [`MiniDumpWriteDump`](https://learn.microsoft.com/en-us/windows/win32/api/minidumpapiset/nf-minidumpapiset-minidumpwritedump)
//! and related structures, as they are not present in `winapi` and we don't want
//! to depend on `windows-sys` due to version churn.
//!
//! Also has a binding for [`GetThreadContext`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getthreadcontext)
//! as the `CONTEXT` structures in `winapi` are not correctly aligned which can
//! cause crashes or bad data, so the [`crash_context::ffi::CONTEXT`] is used
//! instead. See [#63](https://github.com/rust-minidump/minidump-writer/issues/63)

#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    clippy::upper_case_acronyms
)]

pub use crash_context::{capture_context, CONTEXT, EXCEPTION_POINTERS, EXCEPTION_RECORD};

pub type HANDLE = isize;
pub type BOOL = i32;
pub const FALSE: BOOL = 0;

pub type Hresult = i32;
pub const STATUS_NONCONTINUABLE_EXCEPTION: i32 = -1073741787;

pub type PROCESS_ACCESS_RIGHTS = u32;
pub const PROCESS_ALL_ACCESS: PROCESS_ACCESS_RIGHTS = 2097151;

pub type THREAD_ACCESS_RIGHTS = u32;
pub const THREAD_SUSPEND_RESUME: THREAD_ACCESS_RIGHTS = 2;
pub const THREAD_GET_CONTEXT: THREAD_ACCESS_RIGHTS = 8;
pub const THREAD_QUERY_INFORMATION: THREAD_ACCESS_RIGHTS = 64;

bitflags::bitflags! {
    /// <https://learn.microsoft.com/en-us/windows/win32/api/minidumpapiset/ne-minidumpapiset-minidump_type>
    #[derive(Copy, Clone, Debug)]
    #[repr(transparent)]
    pub struct MinidumpType: u32 {
        /// Include just the information necessary to capture stack traces for all
        /// existing threads in a process.
        const Normal = 0;
        /// Include the data sections from all loaded modules.
        ///
        /// This results in the inclusion of global variables, which can make
        /// the minidump file significantly larger.
        const WithDataSegs = 1 << 0;
        /// Include all accessible memory in the process.
        ///
        /// The raw memory data is included at the end, so that the initial
        /// structures can be mapped directly without the raw memory information.
        /// This option can result in a very large file.
        const WithFullMemory = 1 << 1;
        /// Include high-level information about the operating system handles that
        /// are active when the minidump is made.
        const WithHandleData = 1 << 2;
        /// Stack and backing store memory written to the minidump file should be
        /// filtered to remove all but the pointer values necessary to reconstruct a
        /// stack trace.
        const FilterMemory = 1 << 3;
        /// Stack and backing store memory should be scanned for pointer references
        /// to modules in the module list.
        ///
        /// If a module is referenced by stack or backing store memory, the
        /// [`MINIDUMP_CALLBACK_OUTPUT_0::ModuleWriteFlags`] field is set to
        /// [`ModuleWriteFlags::ModuleReferencedByMemory`].
        const ScanMemory = 1 << 4;
        /// Include information from the list of modules that were recently
        /// unloaded, if this information is maintained by the operating system.
        const WithUnloadedModules = 1 << 5;
        /// Include pages with data referenced by locals or other stack memory.
        /// This option can increase the size of the minidump file significantly.
        const WithIndirectlyReferencedMemory = 1 << 6;
        /// Filter module paths for information such as user names or important
        /// directories.
        ///
        /// This option may prevent the system from locating the image file and
        /// should be used only in special situations.
        const FilterModulePaths = 1 << 7;
        /// Include complete per-process and per-thread information from the
        /// operating system.
        const WithProcessThreadData = 1 << 8;
        /// Scan the virtual address space for [`PAGE_READWRITE`](https://learn.microsoft.com/en-us/windows/win32/memory/memory-protection-constants)
        /// memory to be included.
        const WithPrivateReadWriteMemory = 1 << 9;
        /// Reduce the data that is dumped by eliminating memory regions that
        /// are not essential to meet criteria specified for the dump.
        ///
        /// This can avoid dumping memory that may contain data that is private
        /// to the user. However, it is not a guarantee that no private information
        /// will be present.
        const WithoutOptionalData = 1 << 10;
        /// Include memory region information.
        ///
        /// See [MINIDUMP_MEMORY_INFO_LIST](https://learn.microsoft.com/en-us/windows/win32/api/minidumpapiset/ns-minidumpapiset-minidump_memory_info_list)
        const WithFullMemoryInfo = 1 << 11;
        /// Include thread state information.
        ///
        /// See [MINIDUMP_THREAD_INFO_LIST](https://learn.microsoft.com/en-us/windows/win32/api/minidumpapiset/ns-minidumpapiset-minidump_thread_info_list)
        const WithThreadInfo = 1 << 12;
        /// Include all code and code-related sections from loaded modules to
        /// capture executable content.
        ///
        /// For per-module control, use the [`ModuleWriteFlags::ModuleWriteCodeSegs`]
        const WithCodeSegs = 1 << 13;
        /// Turns off secondary auxiliary-supported memory gathering.
        const WithoutAuxiliaryState = 1 << 14;
        /// Requests that auxiliary data providers include their state in the
        /// dump image; the state data that is included is provider dependent.
        ///
        /// This option can result in a large dump image.
        const WithFullAuxiliaryState = 1 << 15;
        /// Scans the virtual address space for [`PAGE_WRITECOPY`](https://learn.microsoft.com/en-us/windows/win32/memory/memory-protection-constants) memory to be included.
        const WithPrivateWriteCopyMemory = 1 << 16;
        /// If you specify [`MinidumpType::MiniDumpWithFullMemory`], the
        /// `MiniDumpWriteDump` function will fail if the function cannot read
        /// the memory regions; however, if you include
        /// [`IgnoreInaccessibleMemory`], the `MiniDumpWriteDump` function will
        /// ignore the memory read failures and continue to generate the dump.
        ///
        /// Note that the inaccessible memory regions are not included in the dump.
        const IgnoreInaccessibleMemory = 1 << 17;
        /// Adds security token related data.
        ///
        /// This will make the "!token" extension work when processing a user-mode dump.
        const WithTokenInformation = 1 << 18;
        /// Adds module header related data.
        const WithModuleHeaders = 1 << 19;
        /// Adds filter triage related data.
        const FilterTriage = 1 << 20;
        /// Adds AVX crash state context registers.
        const WithAvxXStateContext = 1 << 21;
        /// Adds Intel Processor Trace related data.
        const WithIptTrace = 1 << 22;
        /// Scans inaccessible partial memory pages.
        const ScanInaccessiblePartialPages = 1 << 23;
        /// Exclude all memory with the virtual protection attribute of [`PAGE_WRITECOMBINE`](https://learn.microsoft.com/en-us/windows/win32/memory/memory-protection-constants).
        const FilterWriteCombinedMemory = 1 << 24;
    }
}

pub type VS_FIXEDFILEINFO_FILE_FLAGS = u32;

#[repr(C, packed(4))]
pub struct MINIDUMP_USER_STREAM {
    pub Type: u32,
    pub BufferSize: u32,
    pub Buffer: *mut std::ffi::c_void,
}
#[repr(C, packed(4))]
pub struct MINIDUMP_USER_STREAM_INFORMATION {
    pub UserStreamCount: u32,
    pub UserStreamArray: *mut MINIDUMP_USER_STREAM,
}

#[repr(C, packed(4))]
pub struct MINIDUMP_EXCEPTION_INFORMATION {
    pub ThreadId: u32,
    pub ExceptionPointers: *mut EXCEPTION_POINTERS,
    pub ClientPointers: BOOL,
}

pub type VS_FIXEDFILEINFO_FILE_OS = i32;
pub type VS_FIXEDFILEINFO_FILE_TYPE = i32;
pub type VS_FIXEDFILEINFO_FILE_SUBTYPE = i32;

#[repr(C)]
pub struct VS_FIXEDFILEINFO {
    pub dwSignature: u32,
    pub dwStrucVersion: u32,
    pub dwFileVersionMS: u32,
    pub dwFileVersionLS: u32,
    pub dwProductVersionMS: u32,
    pub dwProductVersionLS: u32,
    pub dwFileFlagsMask: u32,
    pub dwFileFlags: VS_FIXEDFILEINFO_FILE_FLAGS,
    pub dwFileOS: VS_FIXEDFILEINFO_FILE_OS,
    pub dwFileType: VS_FIXEDFILEINFO_FILE_TYPE,
    pub dwFileSubtype: VS_FIXEDFILEINFO_FILE_SUBTYPE,
    pub dwFileDateMS: u32,
    pub dwFileDateLS: u32,
}
#[repr(C, packed(4))]
pub struct MINIDUMP_MODULE_CALLBACK {
    pub FullPath: *mut u16,
    pub BaseOfImage: u64,
    pub SizeOfImage: u32,
    pub CheckSum: u32,
    pub TimeDateStamp: u32,
    pub VersionInfo: VS_FIXEDFILEINFO,
    pub CvRecord: *mut std::ffi::c_void,
    pub SizeOfCvRecord: u32,
    pub MiscRecord: *mut std::ffi::c_void,
    pub SizeOfMiscRecord: u32,
}

#[repr(C, packed(4))]
pub struct MINIDUMP_INCLUDE_THREAD_CALLBACK {
    pub ThreadId: u32,
}

#[repr(C, packed(4))]
pub struct MINIDUMP_INCLUDE_MODULE_CALLBACK {
    pub BaseOfImage: u64,
}

#[repr(C, packed(4))]
pub struct MINIDUMP_IO_CALLBACK {
    pub Handle: HANDLE,
    pub Offset: u64,
    pub Buffer: *mut std::ffi::c_void,
    pub BufferBytes: u32,
}

#[repr(C, packed(4))]
pub struct MINIDUMP_READ_MEMORY_FAILURE_CALLBACK {
    pub Offset: u64,
    pub Bytes: u32,
    pub FailureStatus: Hresult,
}

#[repr(C, packed(4))]
pub struct MINIDUMP_VM_QUERY_CALLBACK {
    pub Offset: u64,
}

#[repr(C, packed(4))]
pub struct MINIDUMP_VM_PRE_READ_CALLBACK {
    pub Offset: u64,
    pub Buffer: *mut std::ffi::c_void,
    pub Size: u32,
}

#[repr(C, packed(4))]
pub struct MINIDUMP_VM_POST_READ_CALLBACK {
    pub Offset: u64,
    pub Buffer: *mut std::ffi::c_void,
    pub Size: u32,
    pub Completed: u32,
    pub Status: Hresult,
}

/// Oof, so we have a problem with these structs, they are all packed(4), but
/// `CONTEXT` is aligned by either 4 (x86) or 16 (x86_64/aarch64)...which Rust
/// doesn't currently allow https://github.com/rust-lang/rust/issues/59154, so
/// we need to basically cheat with a big byte array until that issue is fixed (possibly never)
#[repr(C)]
pub struct CALLBACK_CONTEXT([u8; std::mem::size_of::<CONTEXT>()]);

cfg_if::cfg_if! {
    if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
        #[repr(C, packed(4))]
        pub struct MINIDUMP_THREAD_CALLBACK {
            pub ThreadId: u32,
            pub ThreadHandle: HANDLE,
            pub Context: CALLBACK_CONTEXT,
            pub SizeOfContext: u32,
            pub StackBase: u64,
            pub StackEnd: u64,
        }

        #[repr(C, packed(4))]
        pub struct MINIDUMP_THREAD_EX_CALLBACK {
            pub ThreadId: u32,
            pub ThreadHandle: HANDLE,
            pub Context: CALLBACK_CONTEXT,
            pub SizeOfContext: u32,
            pub StackBase: u64,
            pub StackEnd: u64,
            pub BackingStoreBase: u64,
            pub BackingStoreEnd: u64,
        }
    } else if #[cfg(target_arch = "aarch64")] {
        #[repr(C, packed(4))]
        pub struct MINIDUMP_THREAD_CALLBACK {
            pub ThreadId: u32,
            pub ThreadHandle: HANDLE,
            pub Pad: u32,
            pub Context: CALLBACK_CONTEXT,
            pub SizeOfContext: u32,
            pub StackBase: u64,
            pub StackEnd: u64,
        }

        #[repr(C, packed(4))]
        pub struct MINIDUMP_THREAD_EX_CALLBACK {
            pub ThreadId: u32,
            pub ThreadHandle: HANDLE,
            pub Pad: u32,
            pub Context: CALLBACK_CONTEXT,
            pub SizeOfContext: u32,
            pub StackBase: u64,
            pub StackEnd: u64,
            pub BackingStoreBase: u64,
            pub BackingStoreEnd: u64,
        }
    }
}

#[repr(C)]
pub union MINIDUMP_CALLBACK_INPUT_0 {
    pub Status: Hresult,
    pub Thread: std::mem::ManuallyDrop<MINIDUMP_THREAD_CALLBACK>,
    pub ThreadEx: std::mem::ManuallyDrop<MINIDUMP_THREAD_EX_CALLBACK>,
    pub Module: std::mem::ManuallyDrop<MINIDUMP_MODULE_CALLBACK>,
    pub IncludeThread: std::mem::ManuallyDrop<MINIDUMP_INCLUDE_THREAD_CALLBACK>,
    pub IncludeModule: std::mem::ManuallyDrop<MINIDUMP_INCLUDE_MODULE_CALLBACK>,
    pub Io: std::mem::ManuallyDrop<MINIDUMP_IO_CALLBACK>,
    pub ReadMemoryFailure: std::mem::ManuallyDrop<MINIDUMP_READ_MEMORY_FAILURE_CALLBACK>,
    pub SecondaryFlags: u32,
    pub VmQuery: std::mem::ManuallyDrop<MINIDUMP_VM_QUERY_CALLBACK>,
    pub VmPreRead: std::mem::ManuallyDrop<MINIDUMP_VM_PRE_READ_CALLBACK>,
    pub VmPostRead: std::mem::ManuallyDrop<MINIDUMP_VM_POST_READ_CALLBACK>,
}

#[repr(C, packed(4))]
pub struct MINIDUMP_CALLBACK_INPUT {
    pub ProcessId: u32,
    pub ProcessHandle: HANDLE,
    pub CallbackType: u32,
    pub Anonymous: MINIDUMP_CALLBACK_INPUT_0,
}

pub type VIRTUAL_ALLOCATION_TYPE = u32;

#[repr(C, packed(4))]
pub struct MINIDUMP_MEMORY_INFO {
    pub BaseAddress: u64,
    pub AllocationBase: u64,
    pub AllocationProtect: u32,
    __alignment1: u32,
    pub RegionSize: u64,
    pub State: VIRTUAL_ALLOCATION_TYPE,
    pub Protect: u32,
    pub Type: u32,
    __alignment2: u32,
}

#[repr(C, packed(4))]
pub struct MINIDUMP_CALLBACK_OUTPUT_0_0 {
    pub MemoryBase: u64,
    pub MemorySize: u32,
}

#[repr(C)]
pub struct MINIDUMP_CALLBACK_OUTPUT_0_1 {
    pub CheckCancel: BOOL,
    pub Cancel: BOOL,
}

#[repr(C)]
pub struct MINIDUMP_CALLBACK_OUTPUT_0_2 {
    pub VmRegion: MINIDUMP_MEMORY_INFO,
    pub Continue: BOOL,
}

#[repr(C)]
pub struct MINIDUMP_CALLBACK_OUTPUT_0_3 {
    pub VmQueryStatus: Hresult,
    pub VmQueryResult: MINIDUMP_MEMORY_INFO,
}

#[repr(C)]
pub struct MINIDUMP_CALLBACK_OUTPUT_0_4 {
    pub VmReadStatus: Hresult,
    pub VmReadBytesCompleted: u32,
}

bitflags::bitflags! {
    /// Identifies the type of module information that will be written to the
    /// minidump file by the MiniDumpWriteDump function.
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct ModuleWriteFlags: u32 {
        /// Only module information will be written to the minidump file.
        const ModuleWriteModule = 0x0001;
        const ModuleWriteDataSeg = 0x0002;
        const ModuleWriteMiscRecord = 0x0004;
        const ModuleWriteCvRecord = 0x0008;
        const ModuleReferencedByMemory = 0x0010;
        const ModuleWriteTlsData = 0x0020;
        const ModuleWriteCodeSegs = 0x0040;
    }
}

#[repr(C)]
pub union MINIDUMP_CALLBACK_OUTPUT_0 {
    pub ModuleWriteFlags: ModuleWriteFlags,
    pub ThreadWriteFlags: u32,
    pub SecondaryFlags: u32,
    pub Anonymous1: std::mem::ManuallyDrop<MINIDUMP_CALLBACK_OUTPUT_0_0>,
    pub Anonymous2: std::mem::ManuallyDrop<MINIDUMP_CALLBACK_OUTPUT_0_1>,
    pub Handle: HANDLE,
    pub Anonymous3: std::mem::ManuallyDrop<MINIDUMP_CALLBACK_OUTPUT_0_2>,
    pub Anonymous4: std::mem::ManuallyDrop<MINIDUMP_CALLBACK_OUTPUT_0_3>,
    pub Anonymous5: std::mem::ManuallyDrop<MINIDUMP_CALLBACK_OUTPUT_0_4>,
    pub Status: Hresult,
}

#[repr(C, packed(4))]
pub struct MINIDUMP_CALLBACK_OUTPUT {
    pub Anonymous: MINIDUMP_CALLBACK_OUTPUT_0,
}

pub type MINIDUMP_CALLBACK_ROUTINE = Option<
    unsafe extern "system" fn(
        CallbackParam: *mut std::ffi::c_void,
        CallbackInput: *const MINIDUMP_CALLBACK_INPUT,
        CallbackOutput: *mut MINIDUMP_CALLBACK_OUTPUT,
    ) -> BOOL,
>;

#[repr(C, packed(4))]
pub struct MINIDUMP_CALLBACK_INFORMATION {
    pub CallbackRoutine: MINIDUMP_CALLBACK_ROUTINE,
    pub CallbackParam: *mut std::ffi::c_void,
}

#[link(name = "kernel32")]
extern "system" {
    pub fn CloseHandle(handle: HANDLE) -> BOOL;
    pub fn GetCurrentProcess() -> HANDLE;
    pub fn GetCurrentThreadId() -> u32;
    pub fn OpenProcess(
        desired_access: PROCESS_ACCESS_RIGHTS,
        inherit_handle: BOOL,
        process_id: u32,
    ) -> HANDLE;
    pub fn OpenThread(
        desired_access: THREAD_ACCESS_RIGHTS,
        inherit_handle: BOOL,
        thread_id: u32,
    ) -> HANDLE;
    pub fn ResumeThread(thread: HANDLE) -> u32;
    pub fn SuspendThread(thread: HANDLE) -> u32;
    pub fn GetThreadContext(thread: HANDLE, context: *mut CONTEXT) -> BOOL;
}

#[link(name = "dbghelp")]
extern "system" {
    pub fn MiniDumpWriteDump(
        process: HANDLE,
        process_id: u32,
        file: HANDLE,
        dump_type: MinidumpType,
        exception_param: *const MINIDUMP_EXCEPTION_INFORMATION,
        user_stream_param: *const MINIDUMP_USER_STREAM_INFORMATION,
        callback_param: *const MINIDUMP_CALLBACK_INFORMATION,
    ) -> BOOL;
}
