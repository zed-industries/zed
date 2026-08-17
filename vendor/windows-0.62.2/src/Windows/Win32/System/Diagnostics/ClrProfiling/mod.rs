#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct COR_DEBUG_IL_TO_NATIVE_MAP {
    pub ilOffset: u32,
    pub nativeStartOffset: u32,
    pub nativeEndOffset: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct COR_IL_MAP {
    pub oldOffset: u32,
    pub newOffset: u32,
    pub fAccurate: windows_core::BOOL,
}
pub const COR_PRF_ALL: COR_PRF_MONITOR = COR_PRF_MONITOR(-1879048193i32);
pub const COR_PRF_ALLOWABLE_AFTER_ATTACH: COR_PRF_MONITOR = COR_PRF_MONITOR(268763902i32);
pub const COR_PRF_ALLOWABLE_NOTIFICATION_PROFILER: COR_PRF_MONITOR = COR_PRF_MONITOR(-1310512257i32);
#[repr(C)]
#[cfg(feature = "Win32_System_WinRT_Metadata")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct COR_PRF_ASSEMBLY_REFERENCE_INFO {
    pub pbPublicKeyOrToken: *mut core::ffi::c_void,
    pub cbPublicKeyOrToken: u32,
    pub szName: windows_core::PCWSTR,
    pub pMetaData: *mut super::super::WinRT::Metadata::ASSEMBLYMETADATA,
    pub pbHashValue: *mut core::ffi::c_void,
    pub cbHashValue: u32,
    pub dwAssemblyRefFlags: u32,
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl Default for COR_PRF_ASSEMBLY_REFERENCE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const COR_PRF_CACHED_FUNCTION_FOUND: COR_PRF_JIT_CACHE = COR_PRF_JIT_CACHE(0i32);
pub const COR_PRF_CACHED_FUNCTION_NOT_FOUND: COR_PRF_JIT_CACHE = COR_PRF_JIT_CACHE(1i32);
pub const COR_PRF_CLAUSE_CATCH: COR_PRF_CLAUSE_TYPE = COR_PRF_CLAUSE_TYPE(2i32);
pub const COR_PRF_CLAUSE_FILTER: COR_PRF_CLAUSE_TYPE = COR_PRF_CLAUSE_TYPE(1i32);
pub const COR_PRF_CLAUSE_FINALLY: COR_PRF_CLAUSE_TYPE = COR_PRF_CLAUSE_TYPE(3i32);
pub const COR_PRF_CLAUSE_NONE: COR_PRF_CLAUSE_TYPE = COR_PRF_CLAUSE_TYPE(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COR_PRF_CLAUSE_TYPE(pub i32);
pub const COR_PRF_CODEGEN_DISABLE_ALL_OPTIMIZATIONS: COR_PRF_CODEGEN_FLAGS = COR_PRF_CODEGEN_FLAGS(2i32);
pub const COR_PRF_CODEGEN_DISABLE_INLINING: COR_PRF_CODEGEN_FLAGS = COR_PRF_CODEGEN_FLAGS(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COR_PRF_CODEGEN_FLAGS(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct COR_PRF_CODE_INFO {
    pub startAddress: usize,
    pub size: usize,
}
pub const COR_PRF_CORE_CLR: COR_PRF_RUNTIME_TYPE = COR_PRF_RUNTIME_TYPE(2i32);
pub const COR_PRF_DESKTOP_CLR: COR_PRF_RUNTIME_TYPE = COR_PRF_RUNTIME_TYPE(1i32);
pub const COR_PRF_DISABLE_ALL_NGEN_IMAGES: COR_PRF_MONITOR = COR_PRF_MONITOR(-2147483648i32);
pub const COR_PRF_DISABLE_INLINING: COR_PRF_MONITOR = COR_PRF_MONITOR(2097152i32);
pub const COR_PRF_DISABLE_OPTIMIZATIONS: COR_PRF_MONITOR = COR_PRF_MONITOR(4194304i32);
pub const COR_PRF_DISABLE_TRANSPARENCY_CHECKS_UNDER_FULL_TRUST: COR_PRF_MONITOR = COR_PRF_MONITOR(1073741824i32);
pub const COR_PRF_ENABLE_FRAME_INFO: COR_PRF_MONITOR = COR_PRF_MONITOR(134217728i32);
pub const COR_PRF_ENABLE_FUNCTION_ARGS: COR_PRF_MONITOR = COR_PRF_MONITOR(33554432i32);
pub const COR_PRF_ENABLE_FUNCTION_RETVAL: COR_PRF_MONITOR = COR_PRF_MONITOR(67108864i32);
pub const COR_PRF_ENABLE_INPROC_DEBUGGING: COR_PRF_MONITOR = COR_PRF_MONITOR(524288i32);
pub const COR_PRF_ENABLE_JIT_MAPS: COR_PRF_MONITOR = COR_PRF_MONITOR(1048576i32);
pub const COR_PRF_ENABLE_OBJECT_ALLOCATED: COR_PRF_MONITOR = COR_PRF_MONITOR(8388608i32);
pub const COR_PRF_ENABLE_REJIT: COR_PRF_MONITOR = COR_PRF_MONITOR(262144i32);
pub const COR_PRF_ENABLE_STACK_SNAPSHOT: COR_PRF_MONITOR = COR_PRF_MONITOR(268435456i32);
pub const COR_PRF_EVENTPIPE_ARRAY: COR_PRF_EVENTPIPE_PARAM_TYPE = COR_PRF_EVENTPIPE_PARAM_TYPE(19i32);
pub const COR_PRF_EVENTPIPE_BOOLEAN: COR_PRF_EVENTPIPE_PARAM_TYPE = COR_PRF_EVENTPIPE_PARAM_TYPE(3i32);
pub const COR_PRF_EVENTPIPE_BYTE: COR_PRF_EVENTPIPE_PARAM_TYPE = COR_PRF_EVENTPIPE_PARAM_TYPE(6i32);
pub const COR_PRF_EVENTPIPE_CHAR: COR_PRF_EVENTPIPE_PARAM_TYPE = COR_PRF_EVENTPIPE_PARAM_TYPE(4i32);
pub const COR_PRF_EVENTPIPE_CRITICAL: COR_PRF_EVENTPIPE_LEVEL = COR_PRF_EVENTPIPE_LEVEL(1i32);
pub const COR_PRF_EVENTPIPE_DATETIME: COR_PRF_EVENTPIPE_PARAM_TYPE = COR_PRF_EVENTPIPE_PARAM_TYPE(16i32);
pub const COR_PRF_EVENTPIPE_DECIMAL: COR_PRF_EVENTPIPE_PARAM_TYPE = COR_PRF_EVENTPIPE_PARAM_TYPE(15i32);
pub const COR_PRF_EVENTPIPE_DOUBLE: COR_PRF_EVENTPIPE_PARAM_TYPE = COR_PRF_EVENTPIPE_PARAM_TYPE(14i32);
pub const COR_PRF_EVENTPIPE_ERROR: COR_PRF_EVENTPIPE_LEVEL = COR_PRF_EVENTPIPE_LEVEL(2i32);
pub const COR_PRF_EVENTPIPE_GUID: COR_PRF_EVENTPIPE_PARAM_TYPE = COR_PRF_EVENTPIPE_PARAM_TYPE(17i32);
pub const COR_PRF_EVENTPIPE_INFORMATIONAL: COR_PRF_EVENTPIPE_LEVEL = COR_PRF_EVENTPIPE_LEVEL(4i32);
pub const COR_PRF_EVENTPIPE_INT16: COR_PRF_EVENTPIPE_PARAM_TYPE = COR_PRF_EVENTPIPE_PARAM_TYPE(7i32);
pub const COR_PRF_EVENTPIPE_INT32: COR_PRF_EVENTPIPE_PARAM_TYPE = COR_PRF_EVENTPIPE_PARAM_TYPE(9i32);
pub const COR_PRF_EVENTPIPE_INT64: COR_PRF_EVENTPIPE_PARAM_TYPE = COR_PRF_EVENTPIPE_PARAM_TYPE(11i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COR_PRF_EVENTPIPE_LEVEL(pub i32);
pub const COR_PRF_EVENTPIPE_LOGALWAYS: COR_PRF_EVENTPIPE_LEVEL = COR_PRF_EVENTPIPE_LEVEL(0i32);
pub const COR_PRF_EVENTPIPE_OBJECT: COR_PRF_EVENTPIPE_PARAM_TYPE = COR_PRF_EVENTPIPE_PARAM_TYPE(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct COR_PRF_EVENTPIPE_PARAM_DESC {
    pub r#type: u32,
    pub elementType: u32,
    pub name: windows_core::PCWSTR,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COR_PRF_EVENTPIPE_PARAM_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct COR_PRF_EVENTPIPE_PROVIDER_CONFIG {
    pub providerName: windows_core::PCWSTR,
    pub keywords: u64,
    pub loggingLevel: u32,
    pub filterData: windows_core::PCWSTR,
}
pub const COR_PRF_EVENTPIPE_SBYTE: COR_PRF_EVENTPIPE_PARAM_TYPE = COR_PRF_EVENTPIPE_PARAM_TYPE(5i32);
pub const COR_PRF_EVENTPIPE_SINGLE: COR_PRF_EVENTPIPE_PARAM_TYPE = COR_PRF_EVENTPIPE_PARAM_TYPE(13i32);
pub const COR_PRF_EVENTPIPE_STRING: COR_PRF_EVENTPIPE_PARAM_TYPE = COR_PRF_EVENTPIPE_PARAM_TYPE(18i32);
pub const COR_PRF_EVENTPIPE_UINT16: COR_PRF_EVENTPIPE_PARAM_TYPE = COR_PRF_EVENTPIPE_PARAM_TYPE(8i32);
pub const COR_PRF_EVENTPIPE_UINT32: COR_PRF_EVENTPIPE_PARAM_TYPE = COR_PRF_EVENTPIPE_PARAM_TYPE(10i32);
pub const COR_PRF_EVENTPIPE_UINT64: COR_PRF_EVENTPIPE_PARAM_TYPE = COR_PRF_EVENTPIPE_PARAM_TYPE(12i32);
pub const COR_PRF_EVENTPIPE_VERBOSE: COR_PRF_EVENTPIPE_LEVEL = COR_PRF_EVENTPIPE_LEVEL(5i32);
pub const COR_PRF_EVENTPIPE_WARNING: COR_PRF_EVENTPIPE_LEVEL = COR_PRF_EVENTPIPE_LEVEL(3i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct COR_PRF_EVENT_DATA {
    pub ptr: u64,
    pub size: u32,
    pub reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct COR_PRF_EX_CLAUSE_INFO {
    pub clauseType: COR_PRF_CLAUSE_TYPE,
    pub programCounter: usize,
    pub framePointer: usize,
    pub shadowStackPointer: usize,
}
pub const COR_PRF_FIELD_APP_DOMAIN_STATIC: COR_PRF_STATIC_TYPE = COR_PRF_STATIC_TYPE(1i32);
pub const COR_PRF_FIELD_CONTEXT_STATIC: COR_PRF_STATIC_TYPE = COR_PRF_STATIC_TYPE(4i32);
pub const COR_PRF_FIELD_NOT_A_STATIC: COR_PRF_STATIC_TYPE = COR_PRF_STATIC_TYPE(0i32);
pub const COR_PRF_FIELD_RVA_STATIC: COR_PRF_STATIC_TYPE = COR_PRF_STATIC_TYPE(8i32);
pub const COR_PRF_FIELD_THREAD_STATIC: COR_PRF_STATIC_TYPE = COR_PRF_STATIC_TYPE(2i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct COR_PRF_FILTER_DATA {
    pub Ptr: u64,
    pub Size: u32,
    pub Type: u32,
}
pub const COR_PRF_FINALIZER_CRITICAL: COR_PRF_FINALIZER_FLAGS = COR_PRF_FINALIZER_FLAGS(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COR_PRF_FINALIZER_FLAGS(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct COR_PRF_FUNCTION {
    pub functionId: usize,
    pub reJitId: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct COR_PRF_FUNCTION_ARGUMENT_INFO {
    pub numRanges: u32,
    pub totalArgumentSize: u32,
    pub ranges: [COR_PRF_FUNCTION_ARGUMENT_RANGE; 1],
}
impl Default for COR_PRF_FUNCTION_ARGUMENT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct COR_PRF_FUNCTION_ARGUMENT_RANGE {
    pub startAddress: usize,
    pub length: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COR_PRF_GC_GENERATION(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct COR_PRF_GC_GENERATION_RANGE {
    pub generation: COR_PRF_GC_GENERATION,
    pub rangeStart: usize,
    pub rangeLength: usize,
    pub rangeLengthReserved: usize,
}
pub const COR_PRF_GC_GEN_0: COR_PRF_GC_GENERATION = COR_PRF_GC_GENERATION(0i32);
pub const COR_PRF_GC_GEN_1: COR_PRF_GC_GENERATION = COR_PRF_GC_GENERATION(1i32);
pub const COR_PRF_GC_GEN_2: COR_PRF_GC_GENERATION = COR_PRF_GC_GENERATION(2i32);
pub const COR_PRF_GC_INDUCED: COR_PRF_GC_REASON = COR_PRF_GC_REASON(1i32);
pub const COR_PRF_GC_LARGE_OBJECT_HEAP: COR_PRF_GC_GENERATION = COR_PRF_GC_GENERATION(3i32);
pub const COR_PRF_GC_OTHER: COR_PRF_GC_REASON = COR_PRF_GC_REASON(0i32);
pub const COR_PRF_GC_PINNED_OBJECT_HEAP: COR_PRF_GC_GENERATION = COR_PRF_GC_GENERATION(4i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COR_PRF_GC_REASON(pub i32);
pub const COR_PRF_GC_ROOT_FINALIZER: COR_PRF_GC_ROOT_KIND = COR_PRF_GC_ROOT_KIND(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COR_PRF_GC_ROOT_FLAGS(pub i32);
pub const COR_PRF_GC_ROOT_HANDLE: COR_PRF_GC_ROOT_KIND = COR_PRF_GC_ROOT_KIND(3i32);
pub const COR_PRF_GC_ROOT_INTERIOR: COR_PRF_GC_ROOT_FLAGS = COR_PRF_GC_ROOT_FLAGS(4i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COR_PRF_GC_ROOT_KIND(pub i32);
pub const COR_PRF_GC_ROOT_OTHER: COR_PRF_GC_ROOT_KIND = COR_PRF_GC_ROOT_KIND(0i32);
pub const COR_PRF_GC_ROOT_PINNING: COR_PRF_GC_ROOT_FLAGS = COR_PRF_GC_ROOT_FLAGS(1i32);
pub const COR_PRF_GC_ROOT_REFCOUNTED: COR_PRF_GC_ROOT_FLAGS = COR_PRF_GC_ROOT_FLAGS(8i32);
pub const COR_PRF_GC_ROOT_STACK: COR_PRF_GC_ROOT_KIND = COR_PRF_GC_ROOT_KIND(1i32);
pub const COR_PRF_GC_ROOT_WEAKREF: COR_PRF_GC_ROOT_FLAGS = COR_PRF_GC_ROOT_FLAGS(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COR_PRF_HANDLE_TYPE(pub i32);
pub const COR_PRF_HANDLE_TYPE_PINNED: COR_PRF_HANDLE_TYPE = COR_PRF_HANDLE_TYPE(3i32);
pub const COR_PRF_HANDLE_TYPE_STRONG: COR_PRF_HANDLE_TYPE = COR_PRF_HANDLE_TYPE(2i32);
pub const COR_PRF_HANDLE_TYPE_WEAK: COR_PRF_HANDLE_TYPE = COR_PRF_HANDLE_TYPE(1i32);
pub const COR_PRF_HIGH_ADD_ASSEMBLY_REFERENCES: COR_PRF_HIGH_MONITOR = COR_PRF_HIGH_MONITOR(1i32);
pub const COR_PRF_HIGH_ALLOWABLE_AFTER_ATTACH: COR_PRF_HIGH_MONITOR = COR_PRF_HIGH_MONITOR(246i32);
pub const COR_PRF_HIGH_ALLOWABLE_NOTIFICATION_PROFILER: COR_PRF_HIGH_MONITOR = COR_PRF_HIGH_MONITOR(254i32);
pub const COR_PRF_HIGH_BASIC_GC: COR_PRF_HIGH_MONITOR = COR_PRF_HIGH_MONITOR(16i32);
pub const COR_PRF_HIGH_DISABLE_TIERED_COMPILATION: COR_PRF_HIGH_MONITOR = COR_PRF_HIGH_MONITOR(8i32);
pub const COR_PRF_HIGH_IN_MEMORY_SYMBOLS_UPDATED: COR_PRF_HIGH_MONITOR = COR_PRF_HIGH_MONITOR(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COR_PRF_HIGH_MONITOR(pub i32);
pub const COR_PRF_HIGH_MONITOR_DYNAMIC_FUNCTION_UNLOADS: COR_PRF_HIGH_MONITOR = COR_PRF_HIGH_MONITOR(4i32);
pub const COR_PRF_HIGH_MONITOR_EVENT_PIPE: COR_PRF_HIGH_MONITOR = COR_PRF_HIGH_MONITOR(128i32);
pub const COR_PRF_HIGH_MONITOR_GC_MOVED_OBJECTS: COR_PRF_HIGH_MONITOR = COR_PRF_HIGH_MONITOR(32i32);
pub const COR_PRF_HIGH_MONITOR_IMMUTABLE: COR_PRF_HIGH_MONITOR = COR_PRF_HIGH_MONITOR(8i32);
pub const COR_PRF_HIGH_MONITOR_LARGEOBJECT_ALLOCATED: COR_PRF_HIGH_MONITOR = COR_PRF_HIGH_MONITOR(64i32);
pub const COR_PRF_HIGH_MONITOR_NONE: COR_PRF_HIGH_MONITOR = COR_PRF_HIGH_MONITOR(0i32);
pub const COR_PRF_HIGH_MONITOR_PINNEDOBJECT_ALLOCATED: COR_PRF_HIGH_MONITOR = COR_PRF_HIGH_MONITOR(256i32);
pub const COR_PRF_HIGH_REQUIRE_PROFILE_IMAGE: COR_PRF_HIGH_MONITOR = COR_PRF_HIGH_MONITOR(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COR_PRF_JIT_CACHE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct COR_PRF_METHOD {
    pub moduleId: usize,
    pub methodId: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COR_PRF_MISC(pub i32);
pub const COR_PRF_MODULE_COLLECTIBLE: COR_PRF_MODULE_FLAGS = COR_PRF_MODULE_FLAGS(8i32);
pub const COR_PRF_MODULE_DISK: COR_PRF_MODULE_FLAGS = COR_PRF_MODULE_FLAGS(1i32);
pub const COR_PRF_MODULE_DYNAMIC: COR_PRF_MODULE_FLAGS = COR_PRF_MODULE_FLAGS(4i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COR_PRF_MODULE_FLAGS(pub i32);
pub const COR_PRF_MODULE_FLAT_LAYOUT: COR_PRF_MODULE_FLAGS = COR_PRF_MODULE_FLAGS(32i32);
pub const COR_PRF_MODULE_NGEN: COR_PRF_MODULE_FLAGS = COR_PRF_MODULE_FLAGS(2i32);
pub const COR_PRF_MODULE_RESOURCE: COR_PRF_MODULE_FLAGS = COR_PRF_MODULE_FLAGS(16i32);
pub const COR_PRF_MODULE_WINDOWS_RUNTIME: COR_PRF_MODULE_FLAGS = COR_PRF_MODULE_FLAGS(64i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COR_PRF_MONITOR(pub i32);
pub const COR_PRF_MONITOR_ALL: COR_PRF_MONITOR = COR_PRF_MONITOR(17301503i32);
pub const COR_PRF_MONITOR_APPDOMAIN_LOADS: COR_PRF_MONITOR = COR_PRF_MONITOR(16i32);
pub const COR_PRF_MONITOR_ASSEMBLY_LOADS: COR_PRF_MONITOR = COR_PRF_MONITOR(8i32);
pub const COR_PRF_MONITOR_CACHE_SEARCHES: COR_PRF_MONITOR = COR_PRF_MONITOR(131072i32);
pub const COR_PRF_MONITOR_CCW: COR_PRF_MONITOR = COR_PRF_MONITOR(8192i32);
pub const COR_PRF_MONITOR_CLASS_LOADS: COR_PRF_MONITOR = COR_PRF_MONITOR(2i32);
pub const COR_PRF_MONITOR_CLR_EXCEPTIONS: COR_PRF_MONITOR = COR_PRF_MONITOR(16777216i32);
pub const COR_PRF_MONITOR_CODE_TRANSITIONS: COR_PRF_MONITOR = COR_PRF_MONITOR(2048i32);
pub const COR_PRF_MONITOR_ENTERLEAVE: COR_PRF_MONITOR = COR_PRF_MONITOR(4096i32);
pub const COR_PRF_MONITOR_EXCEPTIONS: COR_PRF_MONITOR = COR_PRF_MONITOR(64i32);
pub const COR_PRF_MONITOR_FUNCTION_UNLOADS: COR_PRF_MONITOR = COR_PRF_MONITOR(1i32);
pub const COR_PRF_MONITOR_GC: COR_PRF_MONITOR = COR_PRF_MONITOR(128i32);
pub const COR_PRF_MONITOR_IMMUTABLE: COR_PRF_MONITOR = COR_PRF_MONITOR(-285684736i32);
pub const COR_PRF_MONITOR_JIT_COMPILATION: COR_PRF_MONITOR = COR_PRF_MONITOR(32i32);
pub const COR_PRF_MONITOR_MODULE_LOADS: COR_PRF_MONITOR = COR_PRF_MONITOR(4i32);
pub const COR_PRF_MONITOR_NONE: COR_PRF_MONITOR = COR_PRF_MONITOR(0i32);
pub const COR_PRF_MONITOR_OBJECT_ALLOCATED: COR_PRF_MONITOR = COR_PRF_MONITOR(256i32);
pub const COR_PRF_MONITOR_REMOTING: COR_PRF_MONITOR = COR_PRF_MONITOR(1024i32);
pub const COR_PRF_MONITOR_REMOTING_ASYNC: COR_PRF_MONITOR = COR_PRF_MONITOR(33792i32);
pub const COR_PRF_MONITOR_REMOTING_COOKIE: COR_PRF_MONITOR = COR_PRF_MONITOR(17408i32);
pub const COR_PRF_MONITOR_SUSPENDS: COR_PRF_MONITOR = COR_PRF_MONITOR(65536i32);
pub const COR_PRF_MONITOR_THREADS: COR_PRF_MONITOR = COR_PRF_MONITOR(512i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct COR_PRF_NONGC_HEAP_RANGE {
    pub rangeStart: usize,
    pub rangeLength: usize,
    pub rangeLengthReserved: usize,
}
pub const COR_PRF_REJIT_BLOCK_INLINING: COR_PRF_REJIT_FLAGS = COR_PRF_REJIT_FLAGS(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COR_PRF_REJIT_FLAGS(pub i32);
pub const COR_PRF_REJIT_INLINING_CALLBACKS: COR_PRF_REJIT_FLAGS = COR_PRF_REJIT_FLAGS(2i32);
pub const COR_PRF_REQUIRE_PROFILE_IMAGE: COR_PRF_MONITOR = COR_PRF_MONITOR(536877056i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COR_PRF_RUNTIME_TYPE(pub i32);
pub const COR_PRF_SNAPSHOT_DEFAULT: COR_PRF_SNAPSHOT_INFO = COR_PRF_SNAPSHOT_INFO(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COR_PRF_SNAPSHOT_INFO(pub i32);
pub const COR_PRF_SNAPSHOT_REGISTER_CONTEXT: COR_PRF_SNAPSHOT_INFO = COR_PRF_SNAPSHOT_INFO(1i32);
pub const COR_PRF_SNAPSHOT_X86_OPTIMIZED: COR_PRF_SNAPSHOT_INFO = COR_PRF_SNAPSHOT_INFO(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COR_PRF_STATIC_TYPE(pub i32);
pub const COR_PRF_SUSPEND_FOR_APPDOMAIN_SHUTDOWN: COR_PRF_SUSPEND_REASON = COR_PRF_SUSPEND_REASON(2i32);
pub const COR_PRF_SUSPEND_FOR_CODE_PITCHING: COR_PRF_SUSPEND_REASON = COR_PRF_SUSPEND_REASON(3i32);
pub const COR_PRF_SUSPEND_FOR_GC: COR_PRF_SUSPEND_REASON = COR_PRF_SUSPEND_REASON(1i32);
pub const COR_PRF_SUSPEND_FOR_GC_PREP: COR_PRF_SUSPEND_REASON = COR_PRF_SUSPEND_REASON(7i32);
pub const COR_PRF_SUSPEND_FOR_INPROC_DEBUGGER: COR_PRF_SUSPEND_REASON = COR_PRF_SUSPEND_REASON(6i32);
pub const COR_PRF_SUSPEND_FOR_PROFILER: COR_PRF_SUSPEND_REASON = COR_PRF_SUSPEND_REASON(9i32);
pub const COR_PRF_SUSPEND_FOR_REJIT: COR_PRF_SUSPEND_REASON = COR_PRF_SUSPEND_REASON(8i32);
pub const COR_PRF_SUSPEND_FOR_SHUTDOWN: COR_PRF_SUSPEND_REASON = COR_PRF_SUSPEND_REASON(4i32);
pub const COR_PRF_SUSPEND_OTHER: COR_PRF_SUSPEND_REASON = COR_PRF_SUSPEND_REASON(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COR_PRF_SUSPEND_REASON(pub i32);
pub const COR_PRF_TRANSITION_CALL: COR_PRF_TRANSITION_REASON = COR_PRF_TRANSITION_REASON(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COR_PRF_TRANSITION_REASON(pub i32);
pub const COR_PRF_TRANSITION_RETURN: COR_PRF_TRANSITION_REASON = COR_PRF_TRANSITION_REASON(1i32);
pub const COR_PRF_USE_PROFILE_IMAGES: COR_PRF_MONITOR = COR_PRF_MONITOR(536870912i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CorDebugIlToNativeMappingTypes(pub i32);
pub const EPILOG: CorDebugIlToNativeMappingTypes = CorDebugIlToNativeMappingTypes(-3i32);
pub type EventPipeProviderCallback = Option<unsafe extern "system" fn(source_id: *const u8, is_enabled: u32, level: u8, match_any_keywords: u64, match_all_keywords: u64, filter_data: *mut COR_PRF_FILTER_DATA, callback_data: *mut core::ffi::c_void)>;
pub type FunctionEnter = Option<unsafe extern "system" fn(funcid: usize)>;
pub type FunctionEnter2 = Option<unsafe extern "system" fn(funcid: usize, clientdata: usize, func: usize, argumentinfo: *mut COR_PRF_FUNCTION_ARGUMENT_INFO)>;
pub type FunctionEnter3 = Option<unsafe extern "system" fn(functionidorclientid: FunctionIDOrClientID)>;
pub type FunctionEnter3WithInfo = Option<unsafe extern "system" fn(functionidorclientid: FunctionIDOrClientID, eltinfo: usize)>;
pub type FunctionIDMapper = Option<unsafe extern "system" fn(funcid: usize, pbhookfunction: *mut windows_core::BOOL) -> usize>;
pub type FunctionIDMapper2 = Option<unsafe extern "system" fn(funcid: usize, clientdata: *mut core::ffi::c_void, pbhookfunction: *mut windows_core::BOOL) -> usize>;
#[repr(C)]
#[derive(Clone, Copy)]
pub union FunctionIDOrClientID {
    pub functionID: usize,
    pub clientID: usize,
}
impl Default for FunctionIDOrClientID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type FunctionLeave = Option<unsafe extern "system" fn(funcid: usize)>;
pub type FunctionLeave2 = Option<unsafe extern "system" fn(funcid: usize, clientdata: usize, func: usize, retvalrange: *mut COR_PRF_FUNCTION_ARGUMENT_RANGE)>;
pub type FunctionLeave3 = Option<unsafe extern "system" fn(functionidorclientid: FunctionIDOrClientID)>;
pub type FunctionLeave3WithInfo = Option<unsafe extern "system" fn(functionidorclientid: FunctionIDOrClientID, eltinfo: usize)>;
pub type FunctionTailcall = Option<unsafe extern "system" fn(funcid: usize)>;
pub type FunctionTailcall2 = Option<unsafe extern "system" fn(funcid: usize, clientdata: usize, func: usize)>;
pub type FunctionTailcall3 = Option<unsafe extern "system" fn(functionidorclientid: FunctionIDOrClientID)>;
pub type FunctionTailcall3WithInfo = Option<unsafe extern "system" fn(functionidorclientid: FunctionIDOrClientID, eltinfo: usize)>;
windows_core::imp::define_interface!(ICorProfilerAssemblyReferenceProvider, ICorProfilerAssemblyReferenceProvider_Vtbl, 0x66a78c24_2eef_4f65_b45f_dd1d8038bf3c);
windows_core::imp::interface_hierarchy!(ICorProfilerAssemblyReferenceProvider, windows_core::IUnknown);
impl ICorProfilerAssemblyReferenceProvider {
    #[cfg(feature = "Win32_System_WinRT_Metadata")]
    pub unsafe fn AddAssemblyReference(&self, passemblyrefinfo: *const COR_PRF_ASSEMBLY_REFERENCE_INFO) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddAssemblyReference)(windows_core::Interface::as_raw(self), passemblyrefinfo).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorProfilerAssemblyReferenceProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_WinRT_Metadata")]
    pub AddAssemblyReference: unsafe extern "system" fn(*mut core::ffi::c_void, *const COR_PRF_ASSEMBLY_REFERENCE_INFO) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_WinRT_Metadata"))]
    AddAssemblyReference: usize,
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
pub trait ICorProfilerAssemblyReferenceProvider_Impl: windows_core::IUnknownImpl {
    fn AddAssemblyReference(&self, passemblyrefinfo: *const COR_PRF_ASSEMBLY_REFERENCE_INFO) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl ICorProfilerAssemblyReferenceProvider_Vtbl {
    pub const fn new<Identity: ICorProfilerAssemblyReferenceProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddAssemblyReference<Identity: ICorProfilerAssemblyReferenceProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, passemblyrefinfo: *const COR_PRF_ASSEMBLY_REFERENCE_INFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerAssemblyReferenceProvider_Impl::AddAssemblyReference(this, core::mem::transmute_copy(&passemblyrefinfo)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AddAssemblyReference: AddAssemblyReference::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorProfilerAssemblyReferenceProvider as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl windows_core::RuntimeName for ICorProfilerAssemblyReferenceProvider {}
windows_core::imp::define_interface!(ICorProfilerCallback, ICorProfilerCallback_Vtbl, 0x176fbed1_a55c_4796_98ca_a9da0ef883e7);
windows_core::imp::interface_hierarchy!(ICorProfilerCallback, windows_core::IUnknown);
impl ICorProfilerCallback {
    pub unsafe fn Initialize<P0>(&self, picorprofilerinfounk: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), picorprofilerinfounk.param().abi()).ok() }
    }
    pub unsafe fn Shutdown(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Shutdown)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn AppDomainCreationStarted(&self, appdomainid: usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AppDomainCreationStarted)(windows_core::Interface::as_raw(self), appdomainid).ok() }
    }
    pub unsafe fn AppDomainCreationFinished(&self, appdomainid: usize, hrstatus: windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AppDomainCreationFinished)(windows_core::Interface::as_raw(self), appdomainid, hrstatus).ok() }
    }
    pub unsafe fn AppDomainShutdownStarted(&self, appdomainid: usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AppDomainShutdownStarted)(windows_core::Interface::as_raw(self), appdomainid).ok() }
    }
    pub unsafe fn AppDomainShutdownFinished(&self, appdomainid: usize, hrstatus: windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AppDomainShutdownFinished)(windows_core::Interface::as_raw(self), appdomainid, hrstatus).ok() }
    }
    pub unsafe fn AssemblyLoadStarted(&self, assemblyid: usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AssemblyLoadStarted)(windows_core::Interface::as_raw(self), assemblyid).ok() }
    }
    pub unsafe fn AssemblyLoadFinished(&self, assemblyid: usize, hrstatus: windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AssemblyLoadFinished)(windows_core::Interface::as_raw(self), assemblyid, hrstatus).ok() }
    }
    pub unsafe fn AssemblyUnloadStarted(&self, assemblyid: usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AssemblyUnloadStarted)(windows_core::Interface::as_raw(self), assemblyid).ok() }
    }
    pub unsafe fn AssemblyUnloadFinished(&self, assemblyid: usize, hrstatus: windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AssemblyUnloadFinished)(windows_core::Interface::as_raw(self), assemblyid, hrstatus).ok() }
    }
    pub unsafe fn ModuleLoadStarted(&self, moduleid: usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ModuleLoadStarted)(windows_core::Interface::as_raw(self), moduleid).ok() }
    }
    pub unsafe fn ModuleLoadFinished(&self, moduleid: usize, hrstatus: windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ModuleLoadFinished)(windows_core::Interface::as_raw(self), moduleid, hrstatus).ok() }
    }
    pub unsafe fn ModuleUnloadStarted(&self, moduleid: usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ModuleUnloadStarted)(windows_core::Interface::as_raw(self), moduleid).ok() }
    }
    pub unsafe fn ModuleUnloadFinished(&self, moduleid: usize, hrstatus: windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ModuleUnloadFinished)(windows_core::Interface::as_raw(self), moduleid, hrstatus).ok() }
    }
    pub unsafe fn ModuleAttachedToAssembly(&self, moduleid: usize, assemblyid: usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ModuleAttachedToAssembly)(windows_core::Interface::as_raw(self), moduleid, assemblyid).ok() }
    }
    pub unsafe fn ClassLoadStarted(&self, classid: usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ClassLoadStarted)(windows_core::Interface::as_raw(self), classid).ok() }
    }
    pub unsafe fn ClassLoadFinished(&self, classid: usize, hrstatus: windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ClassLoadFinished)(windows_core::Interface::as_raw(self), classid, hrstatus).ok() }
    }
    pub unsafe fn ClassUnloadStarted(&self, classid: usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ClassUnloadStarted)(windows_core::Interface::as_raw(self), classid).ok() }
    }
    pub unsafe fn ClassUnloadFinished(&self, classid: usize, hrstatus: windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ClassUnloadFinished)(windows_core::Interface::as_raw(self), classid, hrstatus).ok() }
    }
    pub unsafe fn FunctionUnloadStarted(&self, functionid: usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).FunctionUnloadStarted)(windows_core::Interface::as_raw(self), functionid).ok() }
    }
    pub unsafe fn JITCompilationStarted(&self, functionid: usize, fissafetoblock: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).JITCompilationStarted)(windows_core::Interface::as_raw(self), functionid, fissafetoblock.into()).ok() }
    }
    pub unsafe fn JITCompilationFinished(&self, functionid: usize, hrstatus: windows_core::HRESULT, fissafetoblock: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).JITCompilationFinished)(windows_core::Interface::as_raw(self), functionid, hrstatus, fissafetoblock.into()).ok() }
    }
    pub unsafe fn JITCachedFunctionSearchStarted(&self, functionid: usize) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).JITCachedFunctionSearchStarted)(windows_core::Interface::as_raw(self), functionid, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn JITCachedFunctionSearchFinished(&self, functionid: usize, result: COR_PRF_JIT_CACHE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).JITCachedFunctionSearchFinished)(windows_core::Interface::as_raw(self), functionid, result).ok() }
    }
    pub unsafe fn JITFunctionPitched(&self, functionid: usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).JITFunctionPitched)(windows_core::Interface::as_raw(self), functionid).ok() }
    }
    pub unsafe fn JITInlining(&self, callerid: usize, calleeid: usize) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).JITInlining)(windows_core::Interface::as_raw(self), callerid, calleeid, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ThreadCreated(&self, threadid: usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ThreadCreated)(windows_core::Interface::as_raw(self), threadid).ok() }
    }
    pub unsafe fn ThreadDestroyed(&self, threadid: usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ThreadDestroyed)(windows_core::Interface::as_raw(self), threadid).ok() }
    }
    pub unsafe fn ThreadAssignedToOSThread(&self, managedthreadid: usize, osthreadid: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ThreadAssignedToOSThread)(windows_core::Interface::as_raw(self), managedthreadid, osthreadid).ok() }
    }
    pub unsafe fn RemotingClientInvocationStarted(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemotingClientInvocationStarted)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn RemotingClientSendingMessage(&self, pcookie: *const windows_core::GUID, fisasync: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemotingClientSendingMessage)(windows_core::Interface::as_raw(self), pcookie, fisasync.into()).ok() }
    }
    pub unsafe fn RemotingClientReceivingReply(&self, pcookie: *const windows_core::GUID, fisasync: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemotingClientReceivingReply)(windows_core::Interface::as_raw(self), pcookie, fisasync.into()).ok() }
    }
    pub unsafe fn RemotingClientInvocationFinished(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemotingClientInvocationFinished)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn RemotingServerReceivingMessage(&self, pcookie: *const windows_core::GUID, fisasync: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemotingServerReceivingMessage)(windows_core::Interface::as_raw(self), pcookie, fisasync.into()).ok() }
    }
    pub unsafe fn RemotingServerInvocationStarted(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemotingServerInvocationStarted)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn RemotingServerInvocationReturned(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemotingServerInvocationReturned)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn RemotingServerSendingReply(&self, pcookie: *const windows_core::GUID, fisasync: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemotingServerSendingReply)(windows_core::Interface::as_raw(self), pcookie, fisasync.into()).ok() }
    }
    pub unsafe fn UnmanagedToManagedTransition(&self, functionid: usize, reason: COR_PRF_TRANSITION_REASON) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UnmanagedToManagedTransition)(windows_core::Interface::as_raw(self), functionid, reason).ok() }
    }
    pub unsafe fn ManagedToUnmanagedTransition(&self, functionid: usize, reason: COR_PRF_TRANSITION_REASON) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ManagedToUnmanagedTransition)(windows_core::Interface::as_raw(self), functionid, reason).ok() }
    }
    pub unsafe fn RuntimeSuspendStarted(&self, suspendreason: COR_PRF_SUSPEND_REASON) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RuntimeSuspendStarted)(windows_core::Interface::as_raw(self), suspendreason).ok() }
    }
    pub unsafe fn RuntimeSuspendFinished(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RuntimeSuspendFinished)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn RuntimeSuspendAborted(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RuntimeSuspendAborted)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn RuntimeResumeStarted(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RuntimeResumeStarted)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn RuntimeResumeFinished(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RuntimeResumeFinished)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn RuntimeThreadSuspended(&self, threadid: usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RuntimeThreadSuspended)(windows_core::Interface::as_raw(self), threadid).ok() }
    }
    pub unsafe fn RuntimeThreadResumed(&self, threadid: usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RuntimeThreadResumed)(windows_core::Interface::as_raw(self), threadid).ok() }
    }
    pub unsafe fn MovedReferences(&self, cmovedobjectidranges: u32, oldobjectidrangestart: *const usize, newobjectidrangestart: *const usize, cobjectidrangelength: *const u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).MovedReferences)(windows_core::Interface::as_raw(self), cmovedobjectidranges, oldobjectidrangestart, newobjectidrangestart, cobjectidrangelength).ok() }
    }
    pub unsafe fn ObjectAllocated(&self, objectid: usize, classid: usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ObjectAllocated)(windows_core::Interface::as_raw(self), objectid, classid).ok() }
    }
    pub unsafe fn ObjectsAllocatedByClass(&self, cclasscount: u32, classids: *const usize, cobjects: *const u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ObjectsAllocatedByClass)(windows_core::Interface::as_raw(self), cclasscount, classids, cobjects).ok() }
    }
    pub unsafe fn ObjectReferences(&self, objectid: usize, classid: usize, objectrefids: &[usize]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ObjectReferences)(windows_core::Interface::as_raw(self), objectid, classid, objectrefids.len().try_into().unwrap(), core::mem::transmute(objectrefids.as_ptr())).ok() }
    }
    pub unsafe fn RootReferences(&self, rootrefids: &[usize]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RootReferences)(windows_core::Interface::as_raw(self), rootrefids.len().try_into().unwrap(), core::mem::transmute(rootrefids.as_ptr())).ok() }
    }
    pub unsafe fn ExceptionThrown(&self, thrownobjectid: usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ExceptionThrown)(windows_core::Interface::as_raw(self), thrownobjectid).ok() }
    }
    pub unsafe fn ExceptionSearchFunctionEnter(&self, functionid: usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ExceptionSearchFunctionEnter)(windows_core::Interface::as_raw(self), functionid).ok() }
    }
    pub unsafe fn ExceptionSearchFunctionLeave(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ExceptionSearchFunctionLeave)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn ExceptionSearchFilterEnter(&self, functionid: usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ExceptionSearchFilterEnter)(windows_core::Interface::as_raw(self), functionid).ok() }
    }
    pub unsafe fn ExceptionSearchFilterLeave(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ExceptionSearchFilterLeave)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn ExceptionSearchCatcherFound(&self, functionid: usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ExceptionSearchCatcherFound)(windows_core::Interface::as_raw(self), functionid).ok() }
    }
    pub unsafe fn ExceptionOSHandlerEnter(&self, __unused: usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ExceptionOSHandlerEnter)(windows_core::Interface::as_raw(self), __unused).ok() }
    }
    pub unsafe fn ExceptionOSHandlerLeave(&self, __unused: usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ExceptionOSHandlerLeave)(windows_core::Interface::as_raw(self), __unused).ok() }
    }
    pub unsafe fn ExceptionUnwindFunctionEnter(&self, functionid: usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ExceptionUnwindFunctionEnter)(windows_core::Interface::as_raw(self), functionid).ok() }
    }
    pub unsafe fn ExceptionUnwindFunctionLeave(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ExceptionUnwindFunctionLeave)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn ExceptionUnwindFinallyEnter(&self, functionid: usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ExceptionUnwindFinallyEnter)(windows_core::Interface::as_raw(self), functionid).ok() }
    }
    pub unsafe fn ExceptionUnwindFinallyLeave(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ExceptionUnwindFinallyLeave)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn ExceptionCatcherEnter(&self, functionid: usize, objectid: usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ExceptionCatcherEnter)(windows_core::Interface::as_raw(self), functionid, objectid).ok() }
    }
    pub unsafe fn ExceptionCatcherLeave(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ExceptionCatcherLeave)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn COMClassicVTableCreated(&self, wrappedclassid: usize, implementediid: *const windows_core::GUID, pvtable: *const core::ffi::c_void, cslots: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).COMClassicVTableCreated)(windows_core::Interface::as_raw(self), wrappedclassid, implementediid, pvtable, cslots).ok() }
    }
    pub unsafe fn COMClassicVTableDestroyed(&self, wrappedclassid: usize, implementediid: *const windows_core::GUID, pvtable: *const core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).COMClassicVTableDestroyed)(windows_core::Interface::as_raw(self), wrappedclassid, implementediid, pvtable).ok() }
    }
    pub unsafe fn ExceptionCLRCatcherFound(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ExceptionCLRCatcherFound)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn ExceptionCLRCatcherExecute(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ExceptionCLRCatcherExecute)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorProfilerCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Shutdown: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AppDomainCreationStarted: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    pub AppDomainCreationFinished: unsafe extern "system" fn(*mut core::ffi::c_void, usize, windows_core::HRESULT) -> windows_core::HRESULT,
    pub AppDomainShutdownStarted: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    pub AppDomainShutdownFinished: unsafe extern "system" fn(*mut core::ffi::c_void, usize, windows_core::HRESULT) -> windows_core::HRESULT,
    pub AssemblyLoadStarted: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    pub AssemblyLoadFinished: unsafe extern "system" fn(*mut core::ffi::c_void, usize, windows_core::HRESULT) -> windows_core::HRESULT,
    pub AssemblyUnloadStarted: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    pub AssemblyUnloadFinished: unsafe extern "system" fn(*mut core::ffi::c_void, usize, windows_core::HRESULT) -> windows_core::HRESULT,
    pub ModuleLoadStarted: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    pub ModuleLoadFinished: unsafe extern "system" fn(*mut core::ffi::c_void, usize, windows_core::HRESULT) -> windows_core::HRESULT,
    pub ModuleUnloadStarted: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    pub ModuleUnloadFinished: unsafe extern "system" fn(*mut core::ffi::c_void, usize, windows_core::HRESULT) -> windows_core::HRESULT,
    pub ModuleAttachedToAssembly: unsafe extern "system" fn(*mut core::ffi::c_void, usize, usize) -> windows_core::HRESULT,
    pub ClassLoadStarted: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    pub ClassLoadFinished: unsafe extern "system" fn(*mut core::ffi::c_void, usize, windows_core::HRESULT) -> windows_core::HRESULT,
    pub ClassUnloadStarted: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    pub ClassUnloadFinished: unsafe extern "system" fn(*mut core::ffi::c_void, usize, windows_core::HRESULT) -> windows_core::HRESULT,
    pub FunctionUnloadStarted: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    pub JITCompilationStarted: unsafe extern "system" fn(*mut core::ffi::c_void, usize, windows_core::BOOL) -> windows_core::HRESULT,
    pub JITCompilationFinished: unsafe extern "system" fn(*mut core::ffi::c_void, usize, windows_core::HRESULT, windows_core::BOOL) -> windows_core::HRESULT,
    pub JITCachedFunctionSearchStarted: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub JITCachedFunctionSearchFinished: unsafe extern "system" fn(*mut core::ffi::c_void, usize, COR_PRF_JIT_CACHE) -> windows_core::HRESULT,
    pub JITFunctionPitched: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    pub JITInlining: unsafe extern "system" fn(*mut core::ffi::c_void, usize, usize, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub ThreadCreated: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    pub ThreadDestroyed: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    pub ThreadAssignedToOSThread: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32) -> windows_core::HRESULT,
    pub RemotingClientInvocationStarted: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemotingClientSendingMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, windows_core::BOOL) -> windows_core::HRESULT,
    pub RemotingClientReceivingReply: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, windows_core::BOOL) -> windows_core::HRESULT,
    pub RemotingClientInvocationFinished: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemotingServerReceivingMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, windows_core::BOOL) -> windows_core::HRESULT,
    pub RemotingServerInvocationStarted: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemotingServerInvocationReturned: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemotingServerSendingReply: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, windows_core::BOOL) -> windows_core::HRESULT,
    pub UnmanagedToManagedTransition: unsafe extern "system" fn(*mut core::ffi::c_void, usize, COR_PRF_TRANSITION_REASON) -> windows_core::HRESULT,
    pub ManagedToUnmanagedTransition: unsafe extern "system" fn(*mut core::ffi::c_void, usize, COR_PRF_TRANSITION_REASON) -> windows_core::HRESULT,
    pub RuntimeSuspendStarted: unsafe extern "system" fn(*mut core::ffi::c_void, COR_PRF_SUSPEND_REASON) -> windows_core::HRESULT,
    pub RuntimeSuspendFinished: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RuntimeSuspendAborted: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RuntimeResumeStarted: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RuntimeResumeFinished: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RuntimeThreadSuspended: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    pub RuntimeThreadResumed: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    pub MovedReferences: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const usize, *const usize, *const u32) -> windows_core::HRESULT,
    pub ObjectAllocated: unsafe extern "system" fn(*mut core::ffi::c_void, usize, usize) -> windows_core::HRESULT,
    pub ObjectsAllocatedByClass: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const usize, *const u32) -> windows_core::HRESULT,
    pub ObjectReferences: unsafe extern "system" fn(*mut core::ffi::c_void, usize, usize, u32, *const usize) -> windows_core::HRESULT,
    pub RootReferences: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const usize) -> windows_core::HRESULT,
    pub ExceptionThrown: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    pub ExceptionSearchFunctionEnter: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    pub ExceptionSearchFunctionLeave: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExceptionSearchFilterEnter: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    pub ExceptionSearchFilterLeave: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExceptionSearchCatcherFound: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    pub ExceptionOSHandlerEnter: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    pub ExceptionOSHandlerLeave: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    pub ExceptionUnwindFunctionEnter: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    pub ExceptionUnwindFunctionLeave: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExceptionUnwindFinallyEnter: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    pub ExceptionUnwindFinallyLeave: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExceptionCatcherEnter: unsafe extern "system" fn(*mut core::ffi::c_void, usize, usize) -> windows_core::HRESULT,
    pub ExceptionCatcherLeave: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub COMClassicVTableCreated: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *const windows_core::GUID, *const core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub COMClassicVTableDestroyed: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *const windows_core::GUID, *const core::ffi::c_void) -> windows_core::HRESULT,
    pub ExceptionCLRCatcherFound: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExceptionCLRCatcherExecute: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ICorProfilerCallback_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self, picorprofilerinfounk: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn Shutdown(&self) -> windows_core::Result<()>;
    fn AppDomainCreationStarted(&self, appdomainid: usize) -> windows_core::Result<()>;
    fn AppDomainCreationFinished(&self, appdomainid: usize, hrstatus: windows_core::HRESULT) -> windows_core::Result<()>;
    fn AppDomainShutdownStarted(&self, appdomainid: usize) -> windows_core::Result<()>;
    fn AppDomainShutdownFinished(&self, appdomainid: usize, hrstatus: windows_core::HRESULT) -> windows_core::Result<()>;
    fn AssemblyLoadStarted(&self, assemblyid: usize) -> windows_core::Result<()>;
    fn AssemblyLoadFinished(&self, assemblyid: usize, hrstatus: windows_core::HRESULT) -> windows_core::Result<()>;
    fn AssemblyUnloadStarted(&self, assemblyid: usize) -> windows_core::Result<()>;
    fn AssemblyUnloadFinished(&self, assemblyid: usize, hrstatus: windows_core::HRESULT) -> windows_core::Result<()>;
    fn ModuleLoadStarted(&self, moduleid: usize) -> windows_core::Result<()>;
    fn ModuleLoadFinished(&self, moduleid: usize, hrstatus: windows_core::HRESULT) -> windows_core::Result<()>;
    fn ModuleUnloadStarted(&self, moduleid: usize) -> windows_core::Result<()>;
    fn ModuleUnloadFinished(&self, moduleid: usize, hrstatus: windows_core::HRESULT) -> windows_core::Result<()>;
    fn ModuleAttachedToAssembly(&self, moduleid: usize, assemblyid: usize) -> windows_core::Result<()>;
    fn ClassLoadStarted(&self, classid: usize) -> windows_core::Result<()>;
    fn ClassLoadFinished(&self, classid: usize, hrstatus: windows_core::HRESULT) -> windows_core::Result<()>;
    fn ClassUnloadStarted(&self, classid: usize) -> windows_core::Result<()>;
    fn ClassUnloadFinished(&self, classid: usize, hrstatus: windows_core::HRESULT) -> windows_core::Result<()>;
    fn FunctionUnloadStarted(&self, functionid: usize) -> windows_core::Result<()>;
    fn JITCompilationStarted(&self, functionid: usize, fissafetoblock: windows_core::BOOL) -> windows_core::Result<()>;
    fn JITCompilationFinished(&self, functionid: usize, hrstatus: windows_core::HRESULT, fissafetoblock: windows_core::BOOL) -> windows_core::Result<()>;
    fn JITCachedFunctionSearchStarted(&self, functionid: usize) -> windows_core::Result<windows_core::BOOL>;
    fn JITCachedFunctionSearchFinished(&self, functionid: usize, result: COR_PRF_JIT_CACHE) -> windows_core::Result<()>;
    fn JITFunctionPitched(&self, functionid: usize) -> windows_core::Result<()>;
    fn JITInlining(&self, callerid: usize, calleeid: usize) -> windows_core::Result<windows_core::BOOL>;
    fn ThreadCreated(&self, threadid: usize) -> windows_core::Result<()>;
    fn ThreadDestroyed(&self, threadid: usize) -> windows_core::Result<()>;
    fn ThreadAssignedToOSThread(&self, managedthreadid: usize, osthreadid: u32) -> windows_core::Result<()>;
    fn RemotingClientInvocationStarted(&self) -> windows_core::Result<()>;
    fn RemotingClientSendingMessage(&self, pcookie: *const windows_core::GUID, fisasync: windows_core::BOOL) -> windows_core::Result<()>;
    fn RemotingClientReceivingReply(&self, pcookie: *const windows_core::GUID, fisasync: windows_core::BOOL) -> windows_core::Result<()>;
    fn RemotingClientInvocationFinished(&self) -> windows_core::Result<()>;
    fn RemotingServerReceivingMessage(&self, pcookie: *const windows_core::GUID, fisasync: windows_core::BOOL) -> windows_core::Result<()>;
    fn RemotingServerInvocationStarted(&self) -> windows_core::Result<()>;
    fn RemotingServerInvocationReturned(&self) -> windows_core::Result<()>;
    fn RemotingServerSendingReply(&self, pcookie: *const windows_core::GUID, fisasync: windows_core::BOOL) -> windows_core::Result<()>;
    fn UnmanagedToManagedTransition(&self, functionid: usize, reason: COR_PRF_TRANSITION_REASON) -> windows_core::Result<()>;
    fn ManagedToUnmanagedTransition(&self, functionid: usize, reason: COR_PRF_TRANSITION_REASON) -> windows_core::Result<()>;
    fn RuntimeSuspendStarted(&self, suspendreason: COR_PRF_SUSPEND_REASON) -> windows_core::Result<()>;
    fn RuntimeSuspendFinished(&self) -> windows_core::Result<()>;
    fn RuntimeSuspendAborted(&self) -> windows_core::Result<()>;
    fn RuntimeResumeStarted(&self) -> windows_core::Result<()>;
    fn RuntimeResumeFinished(&self) -> windows_core::Result<()>;
    fn RuntimeThreadSuspended(&self, threadid: usize) -> windows_core::Result<()>;
    fn RuntimeThreadResumed(&self, threadid: usize) -> windows_core::Result<()>;
    fn MovedReferences(&self, cmovedobjectidranges: u32, oldobjectidrangestart: *const usize, newobjectidrangestart: *const usize, cobjectidrangelength: *const u32) -> windows_core::Result<()>;
    fn ObjectAllocated(&self, objectid: usize, classid: usize) -> windows_core::Result<()>;
    fn ObjectsAllocatedByClass(&self, cclasscount: u32, classids: *const usize, cobjects: *const u32) -> windows_core::Result<()>;
    fn ObjectReferences(&self, objectid: usize, classid: usize, cobjectrefs: u32, objectrefids: *const usize) -> windows_core::Result<()>;
    fn RootReferences(&self, crootrefs: u32, rootrefids: *const usize) -> windows_core::Result<()>;
    fn ExceptionThrown(&self, thrownobjectid: usize) -> windows_core::Result<()>;
    fn ExceptionSearchFunctionEnter(&self, functionid: usize) -> windows_core::Result<()>;
    fn ExceptionSearchFunctionLeave(&self) -> windows_core::Result<()>;
    fn ExceptionSearchFilterEnter(&self, functionid: usize) -> windows_core::Result<()>;
    fn ExceptionSearchFilterLeave(&self) -> windows_core::Result<()>;
    fn ExceptionSearchCatcherFound(&self, functionid: usize) -> windows_core::Result<()>;
    fn ExceptionOSHandlerEnter(&self, __unused: usize) -> windows_core::Result<()>;
    fn ExceptionOSHandlerLeave(&self, __unused: usize) -> windows_core::Result<()>;
    fn ExceptionUnwindFunctionEnter(&self, functionid: usize) -> windows_core::Result<()>;
    fn ExceptionUnwindFunctionLeave(&self) -> windows_core::Result<()>;
    fn ExceptionUnwindFinallyEnter(&self, functionid: usize) -> windows_core::Result<()>;
    fn ExceptionUnwindFinallyLeave(&self) -> windows_core::Result<()>;
    fn ExceptionCatcherEnter(&self, functionid: usize, objectid: usize) -> windows_core::Result<()>;
    fn ExceptionCatcherLeave(&self) -> windows_core::Result<()>;
    fn COMClassicVTableCreated(&self, wrappedclassid: usize, implementediid: *const windows_core::GUID, pvtable: *const core::ffi::c_void, cslots: u32) -> windows_core::Result<()>;
    fn COMClassicVTableDestroyed(&self, wrappedclassid: usize, implementediid: *const windows_core::GUID, pvtable: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn ExceptionCLRCatcherFound(&self) -> windows_core::Result<()>;
    fn ExceptionCLRCatcherExecute(&self) -> windows_core::Result<()>;
}
impl ICorProfilerCallback_Vtbl {
    pub const fn new<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, picorprofilerinfounk: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::Initialize(this, core::mem::transmute_copy(&picorprofilerinfounk)).into()
            }
        }
        unsafe extern "system" fn Shutdown<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::Shutdown(this).into()
            }
        }
        unsafe extern "system" fn AppDomainCreationStarted<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appdomainid: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::AppDomainCreationStarted(this, core::mem::transmute_copy(&appdomainid)).into()
            }
        }
        unsafe extern "system" fn AppDomainCreationFinished<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appdomainid: usize, hrstatus: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::AppDomainCreationFinished(this, core::mem::transmute_copy(&appdomainid), core::mem::transmute_copy(&hrstatus)).into()
            }
        }
        unsafe extern "system" fn AppDomainShutdownStarted<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appdomainid: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::AppDomainShutdownStarted(this, core::mem::transmute_copy(&appdomainid)).into()
            }
        }
        unsafe extern "system" fn AppDomainShutdownFinished<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appdomainid: usize, hrstatus: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::AppDomainShutdownFinished(this, core::mem::transmute_copy(&appdomainid), core::mem::transmute_copy(&hrstatus)).into()
            }
        }
        unsafe extern "system" fn AssemblyLoadStarted<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, assemblyid: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::AssemblyLoadStarted(this, core::mem::transmute_copy(&assemblyid)).into()
            }
        }
        unsafe extern "system" fn AssemblyLoadFinished<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, assemblyid: usize, hrstatus: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::AssemblyLoadFinished(this, core::mem::transmute_copy(&assemblyid), core::mem::transmute_copy(&hrstatus)).into()
            }
        }
        unsafe extern "system" fn AssemblyUnloadStarted<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, assemblyid: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::AssemblyUnloadStarted(this, core::mem::transmute_copy(&assemblyid)).into()
            }
        }
        unsafe extern "system" fn AssemblyUnloadFinished<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, assemblyid: usize, hrstatus: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::AssemblyUnloadFinished(this, core::mem::transmute_copy(&assemblyid), core::mem::transmute_copy(&hrstatus)).into()
            }
        }
        unsafe extern "system" fn ModuleLoadStarted<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduleid: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::ModuleLoadStarted(this, core::mem::transmute_copy(&moduleid)).into()
            }
        }
        unsafe extern "system" fn ModuleLoadFinished<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduleid: usize, hrstatus: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::ModuleLoadFinished(this, core::mem::transmute_copy(&moduleid), core::mem::transmute_copy(&hrstatus)).into()
            }
        }
        unsafe extern "system" fn ModuleUnloadStarted<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduleid: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::ModuleUnloadStarted(this, core::mem::transmute_copy(&moduleid)).into()
            }
        }
        unsafe extern "system" fn ModuleUnloadFinished<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduleid: usize, hrstatus: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::ModuleUnloadFinished(this, core::mem::transmute_copy(&moduleid), core::mem::transmute_copy(&hrstatus)).into()
            }
        }
        unsafe extern "system" fn ModuleAttachedToAssembly<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduleid: usize, assemblyid: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::ModuleAttachedToAssembly(this, core::mem::transmute_copy(&moduleid), core::mem::transmute_copy(&assemblyid)).into()
            }
        }
        unsafe extern "system" fn ClassLoadStarted<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, classid: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::ClassLoadStarted(this, core::mem::transmute_copy(&classid)).into()
            }
        }
        unsafe extern "system" fn ClassLoadFinished<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, classid: usize, hrstatus: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::ClassLoadFinished(this, core::mem::transmute_copy(&classid), core::mem::transmute_copy(&hrstatus)).into()
            }
        }
        unsafe extern "system" fn ClassUnloadStarted<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, classid: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::ClassUnloadStarted(this, core::mem::transmute_copy(&classid)).into()
            }
        }
        unsafe extern "system" fn ClassUnloadFinished<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, classid: usize, hrstatus: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::ClassUnloadFinished(this, core::mem::transmute_copy(&classid), core::mem::transmute_copy(&hrstatus)).into()
            }
        }
        unsafe extern "system" fn FunctionUnloadStarted<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::FunctionUnloadStarted(this, core::mem::transmute_copy(&functionid)).into()
            }
        }
        unsafe extern "system" fn JITCompilationStarted<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize, fissafetoblock: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::JITCompilationStarted(this, core::mem::transmute_copy(&functionid), core::mem::transmute_copy(&fissafetoblock)).into()
            }
        }
        unsafe extern "system" fn JITCompilationFinished<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize, hrstatus: windows_core::HRESULT, fissafetoblock: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::JITCompilationFinished(this, core::mem::transmute_copy(&functionid), core::mem::transmute_copy(&hrstatus), core::mem::transmute_copy(&fissafetoblock)).into()
            }
        }
        unsafe extern "system" fn JITCachedFunctionSearchStarted<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize, pbusecachedfunction: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorProfilerCallback_Impl::JITCachedFunctionSearchStarted(this, core::mem::transmute_copy(&functionid)) {
                    Ok(ok__) => {
                        pbusecachedfunction.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn JITCachedFunctionSearchFinished<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize, result: COR_PRF_JIT_CACHE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::JITCachedFunctionSearchFinished(this, core::mem::transmute_copy(&functionid), core::mem::transmute_copy(&result)).into()
            }
        }
        unsafe extern "system" fn JITFunctionPitched<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::JITFunctionPitched(this, core::mem::transmute_copy(&functionid)).into()
            }
        }
        unsafe extern "system" fn JITInlining<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, callerid: usize, calleeid: usize, pfshouldinline: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorProfilerCallback_Impl::JITInlining(this, core::mem::transmute_copy(&callerid), core::mem::transmute_copy(&calleeid)) {
                    Ok(ok__) => {
                        pfshouldinline.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ThreadCreated<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, threadid: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::ThreadCreated(this, core::mem::transmute_copy(&threadid)).into()
            }
        }
        unsafe extern "system" fn ThreadDestroyed<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, threadid: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::ThreadDestroyed(this, core::mem::transmute_copy(&threadid)).into()
            }
        }
        unsafe extern "system" fn ThreadAssignedToOSThread<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, managedthreadid: usize, osthreadid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::ThreadAssignedToOSThread(this, core::mem::transmute_copy(&managedthreadid), core::mem::transmute_copy(&osthreadid)).into()
            }
        }
        unsafe extern "system" fn RemotingClientInvocationStarted<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::RemotingClientInvocationStarted(this).into()
            }
        }
        unsafe extern "system" fn RemotingClientSendingMessage<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcookie: *const windows_core::GUID, fisasync: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::RemotingClientSendingMessage(this, core::mem::transmute_copy(&pcookie), core::mem::transmute_copy(&fisasync)).into()
            }
        }
        unsafe extern "system" fn RemotingClientReceivingReply<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcookie: *const windows_core::GUID, fisasync: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::RemotingClientReceivingReply(this, core::mem::transmute_copy(&pcookie), core::mem::transmute_copy(&fisasync)).into()
            }
        }
        unsafe extern "system" fn RemotingClientInvocationFinished<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::RemotingClientInvocationFinished(this).into()
            }
        }
        unsafe extern "system" fn RemotingServerReceivingMessage<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcookie: *const windows_core::GUID, fisasync: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::RemotingServerReceivingMessage(this, core::mem::transmute_copy(&pcookie), core::mem::transmute_copy(&fisasync)).into()
            }
        }
        unsafe extern "system" fn RemotingServerInvocationStarted<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::RemotingServerInvocationStarted(this).into()
            }
        }
        unsafe extern "system" fn RemotingServerInvocationReturned<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::RemotingServerInvocationReturned(this).into()
            }
        }
        unsafe extern "system" fn RemotingServerSendingReply<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcookie: *const windows_core::GUID, fisasync: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::RemotingServerSendingReply(this, core::mem::transmute_copy(&pcookie), core::mem::transmute_copy(&fisasync)).into()
            }
        }
        unsafe extern "system" fn UnmanagedToManagedTransition<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize, reason: COR_PRF_TRANSITION_REASON) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::UnmanagedToManagedTransition(this, core::mem::transmute_copy(&functionid), core::mem::transmute_copy(&reason)).into()
            }
        }
        unsafe extern "system" fn ManagedToUnmanagedTransition<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize, reason: COR_PRF_TRANSITION_REASON) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::ManagedToUnmanagedTransition(this, core::mem::transmute_copy(&functionid), core::mem::transmute_copy(&reason)).into()
            }
        }
        unsafe extern "system" fn RuntimeSuspendStarted<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, suspendreason: COR_PRF_SUSPEND_REASON) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::RuntimeSuspendStarted(this, core::mem::transmute_copy(&suspendreason)).into()
            }
        }
        unsafe extern "system" fn RuntimeSuspendFinished<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::RuntimeSuspendFinished(this).into()
            }
        }
        unsafe extern "system" fn RuntimeSuspendAborted<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::RuntimeSuspendAborted(this).into()
            }
        }
        unsafe extern "system" fn RuntimeResumeStarted<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::RuntimeResumeStarted(this).into()
            }
        }
        unsafe extern "system" fn RuntimeResumeFinished<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::RuntimeResumeFinished(this).into()
            }
        }
        unsafe extern "system" fn RuntimeThreadSuspended<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, threadid: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::RuntimeThreadSuspended(this, core::mem::transmute_copy(&threadid)).into()
            }
        }
        unsafe extern "system" fn RuntimeThreadResumed<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, threadid: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::RuntimeThreadResumed(this, core::mem::transmute_copy(&threadid)).into()
            }
        }
        unsafe extern "system" fn MovedReferences<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cmovedobjectidranges: u32, oldobjectidrangestart: *const usize, newobjectidrangestart: *const usize, cobjectidrangelength: *const u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::MovedReferences(this, core::mem::transmute_copy(&cmovedobjectidranges), core::mem::transmute_copy(&oldobjectidrangestart), core::mem::transmute_copy(&newobjectidrangestart), core::mem::transmute_copy(&cobjectidrangelength)).into()
            }
        }
        unsafe extern "system" fn ObjectAllocated<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objectid: usize, classid: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::ObjectAllocated(this, core::mem::transmute_copy(&objectid), core::mem::transmute_copy(&classid)).into()
            }
        }
        unsafe extern "system" fn ObjectsAllocatedByClass<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cclasscount: u32, classids: *const usize, cobjects: *const u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::ObjectsAllocatedByClass(this, core::mem::transmute_copy(&cclasscount), core::mem::transmute_copy(&classids), core::mem::transmute_copy(&cobjects)).into()
            }
        }
        unsafe extern "system" fn ObjectReferences<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objectid: usize, classid: usize, cobjectrefs: u32, objectrefids: *const usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::ObjectReferences(this, core::mem::transmute_copy(&objectid), core::mem::transmute_copy(&classid), core::mem::transmute_copy(&cobjectrefs), core::mem::transmute_copy(&objectrefids)).into()
            }
        }
        unsafe extern "system" fn RootReferences<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, crootrefs: u32, rootrefids: *const usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::RootReferences(this, core::mem::transmute_copy(&crootrefs), core::mem::transmute_copy(&rootrefids)).into()
            }
        }
        unsafe extern "system" fn ExceptionThrown<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, thrownobjectid: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::ExceptionThrown(this, core::mem::transmute_copy(&thrownobjectid)).into()
            }
        }
        unsafe extern "system" fn ExceptionSearchFunctionEnter<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::ExceptionSearchFunctionEnter(this, core::mem::transmute_copy(&functionid)).into()
            }
        }
        unsafe extern "system" fn ExceptionSearchFunctionLeave<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::ExceptionSearchFunctionLeave(this).into()
            }
        }
        unsafe extern "system" fn ExceptionSearchFilterEnter<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::ExceptionSearchFilterEnter(this, core::mem::transmute_copy(&functionid)).into()
            }
        }
        unsafe extern "system" fn ExceptionSearchFilterLeave<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::ExceptionSearchFilterLeave(this).into()
            }
        }
        unsafe extern "system" fn ExceptionSearchCatcherFound<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::ExceptionSearchCatcherFound(this, core::mem::transmute_copy(&functionid)).into()
            }
        }
        unsafe extern "system" fn ExceptionOSHandlerEnter<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, __unused: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::ExceptionOSHandlerEnter(this, core::mem::transmute_copy(&__unused)).into()
            }
        }
        unsafe extern "system" fn ExceptionOSHandlerLeave<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, __unused: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::ExceptionOSHandlerLeave(this, core::mem::transmute_copy(&__unused)).into()
            }
        }
        unsafe extern "system" fn ExceptionUnwindFunctionEnter<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::ExceptionUnwindFunctionEnter(this, core::mem::transmute_copy(&functionid)).into()
            }
        }
        unsafe extern "system" fn ExceptionUnwindFunctionLeave<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::ExceptionUnwindFunctionLeave(this).into()
            }
        }
        unsafe extern "system" fn ExceptionUnwindFinallyEnter<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::ExceptionUnwindFinallyEnter(this, core::mem::transmute_copy(&functionid)).into()
            }
        }
        unsafe extern "system" fn ExceptionUnwindFinallyLeave<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::ExceptionUnwindFinallyLeave(this).into()
            }
        }
        unsafe extern "system" fn ExceptionCatcherEnter<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize, objectid: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::ExceptionCatcherEnter(this, core::mem::transmute_copy(&functionid), core::mem::transmute_copy(&objectid)).into()
            }
        }
        unsafe extern "system" fn ExceptionCatcherLeave<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::ExceptionCatcherLeave(this).into()
            }
        }
        unsafe extern "system" fn COMClassicVTableCreated<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wrappedclassid: usize, implementediid: *const windows_core::GUID, pvtable: *const core::ffi::c_void, cslots: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::COMClassicVTableCreated(this, core::mem::transmute_copy(&wrappedclassid), core::mem::transmute_copy(&implementediid), core::mem::transmute_copy(&pvtable), core::mem::transmute_copy(&cslots)).into()
            }
        }
        unsafe extern "system" fn COMClassicVTableDestroyed<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wrappedclassid: usize, implementediid: *const windows_core::GUID, pvtable: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::COMClassicVTableDestroyed(this, core::mem::transmute_copy(&wrappedclassid), core::mem::transmute_copy(&implementediid), core::mem::transmute_copy(&pvtable)).into()
            }
        }
        unsafe extern "system" fn ExceptionCLRCatcherFound<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::ExceptionCLRCatcherFound(this).into()
            }
        }
        unsafe extern "system" fn ExceptionCLRCatcherExecute<Identity: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback_Impl::ExceptionCLRCatcherExecute(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            Shutdown: Shutdown::<Identity, OFFSET>,
            AppDomainCreationStarted: AppDomainCreationStarted::<Identity, OFFSET>,
            AppDomainCreationFinished: AppDomainCreationFinished::<Identity, OFFSET>,
            AppDomainShutdownStarted: AppDomainShutdownStarted::<Identity, OFFSET>,
            AppDomainShutdownFinished: AppDomainShutdownFinished::<Identity, OFFSET>,
            AssemblyLoadStarted: AssemblyLoadStarted::<Identity, OFFSET>,
            AssemblyLoadFinished: AssemblyLoadFinished::<Identity, OFFSET>,
            AssemblyUnloadStarted: AssemblyUnloadStarted::<Identity, OFFSET>,
            AssemblyUnloadFinished: AssemblyUnloadFinished::<Identity, OFFSET>,
            ModuleLoadStarted: ModuleLoadStarted::<Identity, OFFSET>,
            ModuleLoadFinished: ModuleLoadFinished::<Identity, OFFSET>,
            ModuleUnloadStarted: ModuleUnloadStarted::<Identity, OFFSET>,
            ModuleUnloadFinished: ModuleUnloadFinished::<Identity, OFFSET>,
            ModuleAttachedToAssembly: ModuleAttachedToAssembly::<Identity, OFFSET>,
            ClassLoadStarted: ClassLoadStarted::<Identity, OFFSET>,
            ClassLoadFinished: ClassLoadFinished::<Identity, OFFSET>,
            ClassUnloadStarted: ClassUnloadStarted::<Identity, OFFSET>,
            ClassUnloadFinished: ClassUnloadFinished::<Identity, OFFSET>,
            FunctionUnloadStarted: FunctionUnloadStarted::<Identity, OFFSET>,
            JITCompilationStarted: JITCompilationStarted::<Identity, OFFSET>,
            JITCompilationFinished: JITCompilationFinished::<Identity, OFFSET>,
            JITCachedFunctionSearchStarted: JITCachedFunctionSearchStarted::<Identity, OFFSET>,
            JITCachedFunctionSearchFinished: JITCachedFunctionSearchFinished::<Identity, OFFSET>,
            JITFunctionPitched: JITFunctionPitched::<Identity, OFFSET>,
            JITInlining: JITInlining::<Identity, OFFSET>,
            ThreadCreated: ThreadCreated::<Identity, OFFSET>,
            ThreadDestroyed: ThreadDestroyed::<Identity, OFFSET>,
            ThreadAssignedToOSThread: ThreadAssignedToOSThread::<Identity, OFFSET>,
            RemotingClientInvocationStarted: RemotingClientInvocationStarted::<Identity, OFFSET>,
            RemotingClientSendingMessage: RemotingClientSendingMessage::<Identity, OFFSET>,
            RemotingClientReceivingReply: RemotingClientReceivingReply::<Identity, OFFSET>,
            RemotingClientInvocationFinished: RemotingClientInvocationFinished::<Identity, OFFSET>,
            RemotingServerReceivingMessage: RemotingServerReceivingMessage::<Identity, OFFSET>,
            RemotingServerInvocationStarted: RemotingServerInvocationStarted::<Identity, OFFSET>,
            RemotingServerInvocationReturned: RemotingServerInvocationReturned::<Identity, OFFSET>,
            RemotingServerSendingReply: RemotingServerSendingReply::<Identity, OFFSET>,
            UnmanagedToManagedTransition: UnmanagedToManagedTransition::<Identity, OFFSET>,
            ManagedToUnmanagedTransition: ManagedToUnmanagedTransition::<Identity, OFFSET>,
            RuntimeSuspendStarted: RuntimeSuspendStarted::<Identity, OFFSET>,
            RuntimeSuspendFinished: RuntimeSuspendFinished::<Identity, OFFSET>,
            RuntimeSuspendAborted: RuntimeSuspendAborted::<Identity, OFFSET>,
            RuntimeResumeStarted: RuntimeResumeStarted::<Identity, OFFSET>,
            RuntimeResumeFinished: RuntimeResumeFinished::<Identity, OFFSET>,
            RuntimeThreadSuspended: RuntimeThreadSuspended::<Identity, OFFSET>,
            RuntimeThreadResumed: RuntimeThreadResumed::<Identity, OFFSET>,
            MovedReferences: MovedReferences::<Identity, OFFSET>,
            ObjectAllocated: ObjectAllocated::<Identity, OFFSET>,
            ObjectsAllocatedByClass: ObjectsAllocatedByClass::<Identity, OFFSET>,
            ObjectReferences: ObjectReferences::<Identity, OFFSET>,
            RootReferences: RootReferences::<Identity, OFFSET>,
            ExceptionThrown: ExceptionThrown::<Identity, OFFSET>,
            ExceptionSearchFunctionEnter: ExceptionSearchFunctionEnter::<Identity, OFFSET>,
            ExceptionSearchFunctionLeave: ExceptionSearchFunctionLeave::<Identity, OFFSET>,
            ExceptionSearchFilterEnter: ExceptionSearchFilterEnter::<Identity, OFFSET>,
            ExceptionSearchFilterLeave: ExceptionSearchFilterLeave::<Identity, OFFSET>,
            ExceptionSearchCatcherFound: ExceptionSearchCatcherFound::<Identity, OFFSET>,
            ExceptionOSHandlerEnter: ExceptionOSHandlerEnter::<Identity, OFFSET>,
            ExceptionOSHandlerLeave: ExceptionOSHandlerLeave::<Identity, OFFSET>,
            ExceptionUnwindFunctionEnter: ExceptionUnwindFunctionEnter::<Identity, OFFSET>,
            ExceptionUnwindFunctionLeave: ExceptionUnwindFunctionLeave::<Identity, OFFSET>,
            ExceptionUnwindFinallyEnter: ExceptionUnwindFinallyEnter::<Identity, OFFSET>,
            ExceptionUnwindFinallyLeave: ExceptionUnwindFinallyLeave::<Identity, OFFSET>,
            ExceptionCatcherEnter: ExceptionCatcherEnter::<Identity, OFFSET>,
            ExceptionCatcherLeave: ExceptionCatcherLeave::<Identity, OFFSET>,
            COMClassicVTableCreated: COMClassicVTableCreated::<Identity, OFFSET>,
            COMClassicVTableDestroyed: COMClassicVTableDestroyed::<Identity, OFFSET>,
            ExceptionCLRCatcherFound: ExceptionCLRCatcherFound::<Identity, OFFSET>,
            ExceptionCLRCatcherExecute: ExceptionCLRCatcherExecute::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorProfilerCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICorProfilerCallback {}
windows_core::imp::define_interface!(ICorProfilerCallback10, ICorProfilerCallback10_Vtbl, 0xcec5b60e_c69c_495f_87f6_84d28ee16ffb);
impl core::ops::Deref for ICorProfilerCallback10 {
    type Target = ICorProfilerCallback9;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICorProfilerCallback10, windows_core::IUnknown, ICorProfilerCallback, ICorProfilerCallback2, ICorProfilerCallback3, ICorProfilerCallback4, ICorProfilerCallback5, ICorProfilerCallback6, ICorProfilerCallback7, ICorProfilerCallback8, ICorProfilerCallback9);
impl ICorProfilerCallback10 {
    pub unsafe fn EventPipeEventDelivered(&self, provider: usize, eventid: u32, eventversion: u32, metadatablob: &[u8], eventdata: &[u8], pactivityid: *const windows_core::GUID, prelatedactivityid: *const windows_core::GUID, eventthread: usize, numstackframes: u32, stackframes: *const usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EventPipeEventDelivered)(windows_core::Interface::as_raw(self), provider, eventid, eventversion, metadatablob.len().try_into().unwrap(), core::mem::transmute(metadatablob.as_ptr()), eventdata.len().try_into().unwrap(), core::mem::transmute(eventdata.as_ptr()), pactivityid, prelatedactivityid, eventthread, numstackframes, stackframes).ok() }
    }
    pub unsafe fn EventPipeProviderCreated(&self, provider: usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EventPipeProviderCreated)(windows_core::Interface::as_raw(self), provider).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorProfilerCallback10_Vtbl {
    pub base__: ICorProfilerCallback9_Vtbl,
    pub EventPipeEventDelivered: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32, u32, u32, *const u8, u32, *const u8, *const windows_core::GUID, *const windows_core::GUID, usize, u32, *const usize) -> windows_core::HRESULT,
    pub EventPipeProviderCreated: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
}
pub trait ICorProfilerCallback10_Impl: ICorProfilerCallback9_Impl {
    fn EventPipeEventDelivered(&self, provider: usize, eventid: u32, eventversion: u32, cbmetadatablob: u32, metadatablob: *const u8, cbeventdata: u32, eventdata: *const u8, pactivityid: *const windows_core::GUID, prelatedactivityid: *const windows_core::GUID, eventthread: usize, numstackframes: u32, stackframes: *const usize) -> windows_core::Result<()>;
    fn EventPipeProviderCreated(&self, provider: usize) -> windows_core::Result<()>;
}
impl ICorProfilerCallback10_Vtbl {
    pub const fn new<Identity: ICorProfilerCallback10_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EventPipeEventDelivered<Identity: ICorProfilerCallback10_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, provider: usize, eventid: u32, eventversion: u32, cbmetadatablob: u32, metadatablob: *const u8, cbeventdata: u32, eventdata: *const u8, pactivityid: *const windows_core::GUID, prelatedactivityid: *const windows_core::GUID, eventthread: usize, numstackframes: u32, stackframes: *const usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback10_Impl::EventPipeEventDelivered(
                    this,
                    core::mem::transmute_copy(&provider),
                    core::mem::transmute_copy(&eventid),
                    core::mem::transmute_copy(&eventversion),
                    core::mem::transmute_copy(&cbmetadatablob),
                    core::mem::transmute_copy(&metadatablob),
                    core::mem::transmute_copy(&cbeventdata),
                    core::mem::transmute_copy(&eventdata),
                    core::mem::transmute_copy(&pactivityid),
                    core::mem::transmute_copy(&prelatedactivityid),
                    core::mem::transmute_copy(&eventthread),
                    core::mem::transmute_copy(&numstackframes),
                    core::mem::transmute_copy(&stackframes),
                )
                .into()
            }
        }
        unsafe extern "system" fn EventPipeProviderCreated<Identity: ICorProfilerCallback10_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, provider: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback10_Impl::EventPipeProviderCreated(this, core::mem::transmute_copy(&provider)).into()
            }
        }
        Self {
            base__: ICorProfilerCallback9_Vtbl::new::<Identity, OFFSET>(),
            EventPipeEventDelivered: EventPipeEventDelivered::<Identity, OFFSET>,
            EventPipeProviderCreated: EventPipeProviderCreated::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorProfilerCallback10 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback as windows_core::Interface>::IID || iid == &<ICorProfilerCallback2 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback3 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback4 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback5 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback6 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback7 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback8 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback9 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICorProfilerCallback10 {}
windows_core::imp::define_interface!(ICorProfilerCallback11, ICorProfilerCallback11_Vtbl, 0x42350846_aaed_47f7_b128_fd0c98881cde);
impl core::ops::Deref for ICorProfilerCallback11 {
    type Target = ICorProfilerCallback10;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICorProfilerCallback11, windows_core::IUnknown, ICorProfilerCallback, ICorProfilerCallback2, ICorProfilerCallback3, ICorProfilerCallback4, ICorProfilerCallback5, ICorProfilerCallback6, ICorProfilerCallback7, ICorProfilerCallback8, ICorProfilerCallback9, ICorProfilerCallback10);
impl ICorProfilerCallback11 {
    pub unsafe fn LoadAsNotificationOnly(&self, pbnotificationonly: *mut windows_core::BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).LoadAsNotificationOnly)(windows_core::Interface::as_raw(self), pbnotificationonly as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorProfilerCallback11_Vtbl {
    pub base__: ICorProfilerCallback10_Vtbl,
    pub LoadAsNotificationOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait ICorProfilerCallback11_Impl: ICorProfilerCallback10_Impl {
    fn LoadAsNotificationOnly(&self, pbnotificationonly: *mut windows_core::BOOL) -> windows_core::Result<()>;
}
impl ICorProfilerCallback11_Vtbl {
    pub const fn new<Identity: ICorProfilerCallback11_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn LoadAsNotificationOnly<Identity: ICorProfilerCallback11_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbnotificationonly: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback11_Impl::LoadAsNotificationOnly(this, core::mem::transmute_copy(&pbnotificationonly)).into()
            }
        }
        Self { base__: ICorProfilerCallback10_Vtbl::new::<Identity, OFFSET>(), LoadAsNotificationOnly: LoadAsNotificationOnly::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorProfilerCallback11 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback as windows_core::Interface>::IID || iid == &<ICorProfilerCallback2 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback3 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback4 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback5 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback6 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback7 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback8 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback9 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback10 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICorProfilerCallback11 {}
windows_core::imp::define_interface!(ICorProfilerCallback2, ICorProfilerCallback2_Vtbl, 0x8a8cc829_ccf2_49fe_bbae_0f022228071a);
impl core::ops::Deref for ICorProfilerCallback2 {
    type Target = ICorProfilerCallback;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICorProfilerCallback2, windows_core::IUnknown, ICorProfilerCallback);
impl ICorProfilerCallback2 {
    pub unsafe fn ThreadNameChanged(&self, threadid: usize, name: Option<&[u16]>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ThreadNameChanged)(windows_core::Interface::as_raw(self), threadid, name.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(name.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))).ok() }
    }
    pub unsafe fn GarbageCollectionStarted(&self, generationcollected: &[windows_core::BOOL], reason: COR_PRF_GC_REASON) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GarbageCollectionStarted)(windows_core::Interface::as_raw(self), generationcollected.len().try_into().unwrap(), core::mem::transmute(generationcollected.as_ptr()), reason).ok() }
    }
    pub unsafe fn SurvivingReferences(&self, csurvivingobjectidranges: u32, objectidrangestart: *const usize, cobjectidrangelength: *const u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SurvivingReferences)(windows_core::Interface::as_raw(self), csurvivingobjectidranges, objectidrangestart, cobjectidrangelength).ok() }
    }
    pub unsafe fn GarbageCollectionFinished(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GarbageCollectionFinished)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn FinalizeableObjectQueued(&self, finalizerflags: u32, objectid: usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).FinalizeableObjectQueued)(windows_core::Interface::as_raw(self), finalizerflags, objectid).ok() }
    }
    pub unsafe fn RootReferences2(&self, crootrefs: u32, rootrefids: *const usize, rootkinds: *const COR_PRF_GC_ROOT_KIND, rootflags: *const COR_PRF_GC_ROOT_FLAGS, rootids: *const usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RootReferences2)(windows_core::Interface::as_raw(self), crootrefs, rootrefids, rootkinds, rootflags, rootids).ok() }
    }
    pub unsafe fn HandleCreated(&self, handleid: usize, initialobjectid: usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).HandleCreated)(windows_core::Interface::as_raw(self), handleid, initialobjectid).ok() }
    }
    pub unsafe fn HandleDestroyed(&self, handleid: usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).HandleDestroyed)(windows_core::Interface::as_raw(self), handleid).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorProfilerCallback2_Vtbl {
    pub base__: ICorProfilerCallback_Vtbl,
    pub ThreadNameChanged: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GarbageCollectionStarted: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *const windows_core::BOOL, COR_PRF_GC_REASON) -> windows_core::HRESULT,
    pub SurvivingReferences: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const usize, *const u32) -> windows_core::HRESULT,
    pub GarbageCollectionFinished: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FinalizeableObjectQueued: unsafe extern "system" fn(*mut core::ffi::c_void, u32, usize) -> windows_core::HRESULT,
    pub RootReferences2: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const usize, *const COR_PRF_GC_ROOT_KIND, *const COR_PRF_GC_ROOT_FLAGS, *const usize) -> windows_core::HRESULT,
    pub HandleCreated: unsafe extern "system" fn(*mut core::ffi::c_void, usize, usize) -> windows_core::HRESULT,
    pub HandleDestroyed: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
}
pub trait ICorProfilerCallback2_Impl: ICorProfilerCallback_Impl {
    fn ThreadNameChanged(&self, threadid: usize, cchname: u32, name: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GarbageCollectionStarted(&self, cgenerations: i32, generationcollected: *const windows_core::BOOL, reason: COR_PRF_GC_REASON) -> windows_core::Result<()>;
    fn SurvivingReferences(&self, csurvivingobjectidranges: u32, objectidrangestart: *const usize, cobjectidrangelength: *const u32) -> windows_core::Result<()>;
    fn GarbageCollectionFinished(&self) -> windows_core::Result<()>;
    fn FinalizeableObjectQueued(&self, finalizerflags: u32, objectid: usize) -> windows_core::Result<()>;
    fn RootReferences2(&self, crootrefs: u32, rootrefids: *const usize, rootkinds: *const COR_PRF_GC_ROOT_KIND, rootflags: *const COR_PRF_GC_ROOT_FLAGS, rootids: *const usize) -> windows_core::Result<()>;
    fn HandleCreated(&self, handleid: usize, initialobjectid: usize) -> windows_core::Result<()>;
    fn HandleDestroyed(&self, handleid: usize) -> windows_core::Result<()>;
}
impl ICorProfilerCallback2_Vtbl {
    pub const fn new<Identity: ICorProfilerCallback2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ThreadNameChanged<Identity: ICorProfilerCallback2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, threadid: usize, cchname: u32, name: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback2_Impl::ThreadNameChanged(this, core::mem::transmute_copy(&threadid), core::mem::transmute_copy(&cchname), core::mem::transmute(&name)).into()
            }
        }
        unsafe extern "system" fn GarbageCollectionStarted<Identity: ICorProfilerCallback2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cgenerations: i32, generationcollected: *const windows_core::BOOL, reason: COR_PRF_GC_REASON) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback2_Impl::GarbageCollectionStarted(this, core::mem::transmute_copy(&cgenerations), core::mem::transmute_copy(&generationcollected), core::mem::transmute_copy(&reason)).into()
            }
        }
        unsafe extern "system" fn SurvivingReferences<Identity: ICorProfilerCallback2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, csurvivingobjectidranges: u32, objectidrangestart: *const usize, cobjectidrangelength: *const u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback2_Impl::SurvivingReferences(this, core::mem::transmute_copy(&csurvivingobjectidranges), core::mem::transmute_copy(&objectidrangestart), core::mem::transmute_copy(&cobjectidrangelength)).into()
            }
        }
        unsafe extern "system" fn GarbageCollectionFinished<Identity: ICorProfilerCallback2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback2_Impl::GarbageCollectionFinished(this).into()
            }
        }
        unsafe extern "system" fn FinalizeableObjectQueued<Identity: ICorProfilerCallback2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, finalizerflags: u32, objectid: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback2_Impl::FinalizeableObjectQueued(this, core::mem::transmute_copy(&finalizerflags), core::mem::transmute_copy(&objectid)).into()
            }
        }
        unsafe extern "system" fn RootReferences2<Identity: ICorProfilerCallback2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, crootrefs: u32, rootrefids: *const usize, rootkinds: *const COR_PRF_GC_ROOT_KIND, rootflags: *const COR_PRF_GC_ROOT_FLAGS, rootids: *const usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback2_Impl::RootReferences2(this, core::mem::transmute_copy(&crootrefs), core::mem::transmute_copy(&rootrefids), core::mem::transmute_copy(&rootkinds), core::mem::transmute_copy(&rootflags), core::mem::transmute_copy(&rootids)).into()
            }
        }
        unsafe extern "system" fn HandleCreated<Identity: ICorProfilerCallback2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handleid: usize, initialobjectid: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback2_Impl::HandleCreated(this, core::mem::transmute_copy(&handleid), core::mem::transmute_copy(&initialobjectid)).into()
            }
        }
        unsafe extern "system" fn HandleDestroyed<Identity: ICorProfilerCallback2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handleid: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback2_Impl::HandleDestroyed(this, core::mem::transmute_copy(&handleid)).into()
            }
        }
        Self {
            base__: ICorProfilerCallback_Vtbl::new::<Identity, OFFSET>(),
            ThreadNameChanged: ThreadNameChanged::<Identity, OFFSET>,
            GarbageCollectionStarted: GarbageCollectionStarted::<Identity, OFFSET>,
            SurvivingReferences: SurvivingReferences::<Identity, OFFSET>,
            GarbageCollectionFinished: GarbageCollectionFinished::<Identity, OFFSET>,
            FinalizeableObjectQueued: FinalizeableObjectQueued::<Identity, OFFSET>,
            RootReferences2: RootReferences2::<Identity, OFFSET>,
            HandleCreated: HandleCreated::<Identity, OFFSET>,
            HandleDestroyed: HandleDestroyed::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorProfilerCallback2 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICorProfilerCallback2 {}
windows_core::imp::define_interface!(ICorProfilerCallback3, ICorProfilerCallback3_Vtbl, 0x4fd2ed52_7731_4b8d_9469_03d2cc3086c5);
impl core::ops::Deref for ICorProfilerCallback3 {
    type Target = ICorProfilerCallback2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICorProfilerCallback3, windows_core::IUnknown, ICorProfilerCallback, ICorProfilerCallback2);
impl ICorProfilerCallback3 {
    pub unsafe fn InitializeForAttach<P0>(&self, pcorprofilerinfounk: P0, pvclientdata: *const core::ffi::c_void, cbclientdata: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).InitializeForAttach)(windows_core::Interface::as_raw(self), pcorprofilerinfounk.param().abi(), pvclientdata, cbclientdata).ok() }
    }
    pub unsafe fn ProfilerAttachComplete(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ProfilerAttachComplete)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn ProfilerDetachSucceeded(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ProfilerDetachSucceeded)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorProfilerCallback3_Vtbl {
    pub base__: ICorProfilerCallback2_Vtbl,
    pub InitializeForAttach: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ProfilerAttachComplete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProfilerDetachSucceeded: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ICorProfilerCallback3_Impl: ICorProfilerCallback2_Impl {
    fn InitializeForAttach(&self, pcorprofilerinfounk: windows_core::Ref<windows_core::IUnknown>, pvclientdata: *const core::ffi::c_void, cbclientdata: u32) -> windows_core::Result<()>;
    fn ProfilerAttachComplete(&self) -> windows_core::Result<()>;
    fn ProfilerDetachSucceeded(&self) -> windows_core::Result<()>;
}
impl ICorProfilerCallback3_Vtbl {
    pub const fn new<Identity: ICorProfilerCallback3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InitializeForAttach<Identity: ICorProfilerCallback3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcorprofilerinfounk: *mut core::ffi::c_void, pvclientdata: *const core::ffi::c_void, cbclientdata: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback3_Impl::InitializeForAttach(this, core::mem::transmute_copy(&pcorprofilerinfounk), core::mem::transmute_copy(&pvclientdata), core::mem::transmute_copy(&cbclientdata)).into()
            }
        }
        unsafe extern "system" fn ProfilerAttachComplete<Identity: ICorProfilerCallback3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback3_Impl::ProfilerAttachComplete(this).into()
            }
        }
        unsafe extern "system" fn ProfilerDetachSucceeded<Identity: ICorProfilerCallback3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback3_Impl::ProfilerDetachSucceeded(this).into()
            }
        }
        Self {
            base__: ICorProfilerCallback2_Vtbl::new::<Identity, OFFSET>(),
            InitializeForAttach: InitializeForAttach::<Identity, OFFSET>,
            ProfilerAttachComplete: ProfilerAttachComplete::<Identity, OFFSET>,
            ProfilerDetachSucceeded: ProfilerDetachSucceeded::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorProfilerCallback3 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback as windows_core::Interface>::IID || iid == &<ICorProfilerCallback2 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICorProfilerCallback3 {}
windows_core::imp::define_interface!(ICorProfilerCallback4, ICorProfilerCallback4_Vtbl, 0x7b63b2e3_107d_4d48_b2f6_f61e229470d2);
impl core::ops::Deref for ICorProfilerCallback4 {
    type Target = ICorProfilerCallback3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICorProfilerCallback4, windows_core::IUnknown, ICorProfilerCallback, ICorProfilerCallback2, ICorProfilerCallback3);
impl ICorProfilerCallback4 {
    pub unsafe fn ReJITCompilationStarted(&self, functionid: usize, rejitid: usize, fissafetoblock: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReJITCompilationStarted)(windows_core::Interface::as_raw(self), functionid, rejitid, fissafetoblock.into()).ok() }
    }
    pub unsafe fn GetReJITParameters<P2>(&self, moduleid: usize, methodid: u32, pfunctioncontrol: P2) -> windows_core::Result<()>
    where
        P2: windows_core::Param<ICorProfilerFunctionControl>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetReJITParameters)(windows_core::Interface::as_raw(self), moduleid, methodid, pfunctioncontrol.param().abi()).ok() }
    }
    pub unsafe fn ReJITCompilationFinished(&self, functionid: usize, rejitid: usize, hrstatus: windows_core::HRESULT, fissafetoblock: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReJITCompilationFinished)(windows_core::Interface::as_raw(self), functionid, rejitid, hrstatus, fissafetoblock.into()).ok() }
    }
    pub unsafe fn ReJITError(&self, moduleid: usize, methodid: u32, functionid: usize, hrstatus: windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReJITError)(windows_core::Interface::as_raw(self), moduleid, methodid, functionid, hrstatus).ok() }
    }
    pub unsafe fn MovedReferences2(&self, cmovedobjectidranges: u32, oldobjectidrangestart: *const usize, newobjectidrangestart: *const usize, cobjectidrangelength: *const usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).MovedReferences2)(windows_core::Interface::as_raw(self), cmovedobjectidranges, oldobjectidrangestart, newobjectidrangestart, cobjectidrangelength).ok() }
    }
    pub unsafe fn SurvivingReferences2(&self, csurvivingobjectidranges: u32, objectidrangestart: *const usize, cobjectidrangelength: *const usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SurvivingReferences2)(windows_core::Interface::as_raw(self), csurvivingobjectidranges, objectidrangestart, cobjectidrangelength).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorProfilerCallback4_Vtbl {
    pub base__: ICorProfilerCallback3_Vtbl,
    pub ReJITCompilationStarted: unsafe extern "system" fn(*mut core::ffi::c_void, usize, usize, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetReJITParameters: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReJITCompilationFinished: unsafe extern "system" fn(*mut core::ffi::c_void, usize, usize, windows_core::HRESULT, windows_core::BOOL) -> windows_core::HRESULT,
    pub ReJITError: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32, usize, windows_core::HRESULT) -> windows_core::HRESULT,
    pub MovedReferences2: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const usize, *const usize, *const usize) -> windows_core::HRESULT,
    pub SurvivingReferences2: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const usize, *const usize) -> windows_core::HRESULT,
}
pub trait ICorProfilerCallback4_Impl: ICorProfilerCallback3_Impl {
    fn ReJITCompilationStarted(&self, functionid: usize, rejitid: usize, fissafetoblock: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetReJITParameters(&self, moduleid: usize, methodid: u32, pfunctioncontrol: windows_core::Ref<ICorProfilerFunctionControl>) -> windows_core::Result<()>;
    fn ReJITCompilationFinished(&self, functionid: usize, rejitid: usize, hrstatus: windows_core::HRESULT, fissafetoblock: windows_core::BOOL) -> windows_core::Result<()>;
    fn ReJITError(&self, moduleid: usize, methodid: u32, functionid: usize, hrstatus: windows_core::HRESULT) -> windows_core::Result<()>;
    fn MovedReferences2(&self, cmovedobjectidranges: u32, oldobjectidrangestart: *const usize, newobjectidrangestart: *const usize, cobjectidrangelength: *const usize) -> windows_core::Result<()>;
    fn SurvivingReferences2(&self, csurvivingobjectidranges: u32, objectidrangestart: *const usize, cobjectidrangelength: *const usize) -> windows_core::Result<()>;
}
impl ICorProfilerCallback4_Vtbl {
    pub const fn new<Identity: ICorProfilerCallback4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ReJITCompilationStarted<Identity: ICorProfilerCallback4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize, rejitid: usize, fissafetoblock: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback4_Impl::ReJITCompilationStarted(this, core::mem::transmute_copy(&functionid), core::mem::transmute_copy(&rejitid), core::mem::transmute_copy(&fissafetoblock)).into()
            }
        }
        unsafe extern "system" fn GetReJITParameters<Identity: ICorProfilerCallback4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduleid: usize, methodid: u32, pfunctioncontrol: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback4_Impl::GetReJITParameters(this, core::mem::transmute_copy(&moduleid), core::mem::transmute_copy(&methodid), core::mem::transmute_copy(&pfunctioncontrol)).into()
            }
        }
        unsafe extern "system" fn ReJITCompilationFinished<Identity: ICorProfilerCallback4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize, rejitid: usize, hrstatus: windows_core::HRESULT, fissafetoblock: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback4_Impl::ReJITCompilationFinished(this, core::mem::transmute_copy(&functionid), core::mem::transmute_copy(&rejitid), core::mem::transmute_copy(&hrstatus), core::mem::transmute_copy(&fissafetoblock)).into()
            }
        }
        unsafe extern "system" fn ReJITError<Identity: ICorProfilerCallback4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduleid: usize, methodid: u32, functionid: usize, hrstatus: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback4_Impl::ReJITError(this, core::mem::transmute_copy(&moduleid), core::mem::transmute_copy(&methodid), core::mem::transmute_copy(&functionid), core::mem::transmute_copy(&hrstatus)).into()
            }
        }
        unsafe extern "system" fn MovedReferences2<Identity: ICorProfilerCallback4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cmovedobjectidranges: u32, oldobjectidrangestart: *const usize, newobjectidrangestart: *const usize, cobjectidrangelength: *const usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback4_Impl::MovedReferences2(this, core::mem::transmute_copy(&cmovedobjectidranges), core::mem::transmute_copy(&oldobjectidrangestart), core::mem::transmute_copy(&newobjectidrangestart), core::mem::transmute_copy(&cobjectidrangelength)).into()
            }
        }
        unsafe extern "system" fn SurvivingReferences2<Identity: ICorProfilerCallback4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, csurvivingobjectidranges: u32, objectidrangestart: *const usize, cobjectidrangelength: *const usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback4_Impl::SurvivingReferences2(this, core::mem::transmute_copy(&csurvivingobjectidranges), core::mem::transmute_copy(&objectidrangestart), core::mem::transmute_copy(&cobjectidrangelength)).into()
            }
        }
        Self {
            base__: ICorProfilerCallback3_Vtbl::new::<Identity, OFFSET>(),
            ReJITCompilationStarted: ReJITCompilationStarted::<Identity, OFFSET>,
            GetReJITParameters: GetReJITParameters::<Identity, OFFSET>,
            ReJITCompilationFinished: ReJITCompilationFinished::<Identity, OFFSET>,
            ReJITError: ReJITError::<Identity, OFFSET>,
            MovedReferences2: MovedReferences2::<Identity, OFFSET>,
            SurvivingReferences2: SurvivingReferences2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorProfilerCallback4 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback as windows_core::Interface>::IID || iid == &<ICorProfilerCallback2 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback3 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICorProfilerCallback4 {}
windows_core::imp::define_interface!(ICorProfilerCallback5, ICorProfilerCallback5_Vtbl, 0x8dfba405_8c9f_45f8_bffa_83b14cef78b5);
impl core::ops::Deref for ICorProfilerCallback5 {
    type Target = ICorProfilerCallback4;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICorProfilerCallback5, windows_core::IUnknown, ICorProfilerCallback, ICorProfilerCallback2, ICorProfilerCallback3, ICorProfilerCallback4);
impl ICorProfilerCallback5 {
    pub unsafe fn ConditionalWeakTableElementReferences(&self, crootrefs: u32, keyrefids: *const usize, valuerefids: *const usize, rootids: *const usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ConditionalWeakTableElementReferences)(windows_core::Interface::as_raw(self), crootrefs, keyrefids, valuerefids, rootids).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorProfilerCallback5_Vtbl {
    pub base__: ICorProfilerCallback4_Vtbl,
    pub ConditionalWeakTableElementReferences: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const usize, *const usize, *const usize) -> windows_core::HRESULT,
}
pub trait ICorProfilerCallback5_Impl: ICorProfilerCallback4_Impl {
    fn ConditionalWeakTableElementReferences(&self, crootrefs: u32, keyrefids: *const usize, valuerefids: *const usize, rootids: *const usize) -> windows_core::Result<()>;
}
impl ICorProfilerCallback5_Vtbl {
    pub const fn new<Identity: ICorProfilerCallback5_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ConditionalWeakTableElementReferences<Identity: ICorProfilerCallback5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, crootrefs: u32, keyrefids: *const usize, valuerefids: *const usize, rootids: *const usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback5_Impl::ConditionalWeakTableElementReferences(this, core::mem::transmute_copy(&crootrefs), core::mem::transmute_copy(&keyrefids), core::mem::transmute_copy(&valuerefids), core::mem::transmute_copy(&rootids)).into()
            }
        }
        Self {
            base__: ICorProfilerCallback4_Vtbl::new::<Identity, OFFSET>(),
            ConditionalWeakTableElementReferences: ConditionalWeakTableElementReferences::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorProfilerCallback5 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback as windows_core::Interface>::IID || iid == &<ICorProfilerCallback2 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback3 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback4 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICorProfilerCallback5 {}
windows_core::imp::define_interface!(ICorProfilerCallback6, ICorProfilerCallback6_Vtbl, 0xfc13df4b_4448_4f4f_950c_ba8d19d00c36);
impl core::ops::Deref for ICorProfilerCallback6 {
    type Target = ICorProfilerCallback5;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICorProfilerCallback6, windows_core::IUnknown, ICorProfilerCallback, ICorProfilerCallback2, ICorProfilerCallback3, ICorProfilerCallback4, ICorProfilerCallback5);
impl ICorProfilerCallback6 {
    pub unsafe fn GetAssemblyReferences<P0, P1>(&self, wszassemblypath: P0, pasmrefprovider: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<ICorProfilerAssemblyReferenceProvider>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetAssemblyReferences)(windows_core::Interface::as_raw(self), wszassemblypath.param().abi(), pasmrefprovider.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorProfilerCallback6_Vtbl {
    pub base__: ICorProfilerCallback5_Vtbl,
    pub GetAssemblyReferences: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ICorProfilerCallback6_Impl: ICorProfilerCallback5_Impl {
    fn GetAssemblyReferences(&self, wszassemblypath: &windows_core::PCWSTR, pasmrefprovider: windows_core::Ref<ICorProfilerAssemblyReferenceProvider>) -> windows_core::Result<()>;
}
impl ICorProfilerCallback6_Vtbl {
    pub const fn new<Identity: ICorProfilerCallback6_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetAssemblyReferences<Identity: ICorProfilerCallback6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszassemblypath: windows_core::PCWSTR, pasmrefprovider: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback6_Impl::GetAssemblyReferences(this, core::mem::transmute(&wszassemblypath), core::mem::transmute_copy(&pasmrefprovider)).into()
            }
        }
        Self { base__: ICorProfilerCallback5_Vtbl::new::<Identity, OFFSET>(), GetAssemblyReferences: GetAssemblyReferences::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorProfilerCallback6 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback as windows_core::Interface>::IID || iid == &<ICorProfilerCallback2 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback3 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback4 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback5 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICorProfilerCallback6 {}
windows_core::imp::define_interface!(ICorProfilerCallback7, ICorProfilerCallback7_Vtbl, 0xf76a2dba_1d52_4539_866c_2aa518f9efc3);
impl core::ops::Deref for ICorProfilerCallback7 {
    type Target = ICorProfilerCallback6;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICorProfilerCallback7, windows_core::IUnknown, ICorProfilerCallback, ICorProfilerCallback2, ICorProfilerCallback3, ICorProfilerCallback4, ICorProfilerCallback5, ICorProfilerCallback6);
impl ICorProfilerCallback7 {
    pub unsafe fn ModuleInMemorySymbolsUpdated(&self, moduleid: usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ModuleInMemorySymbolsUpdated)(windows_core::Interface::as_raw(self), moduleid).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorProfilerCallback7_Vtbl {
    pub base__: ICorProfilerCallback6_Vtbl,
    pub ModuleInMemorySymbolsUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
}
pub trait ICorProfilerCallback7_Impl: ICorProfilerCallback6_Impl {
    fn ModuleInMemorySymbolsUpdated(&self, moduleid: usize) -> windows_core::Result<()>;
}
impl ICorProfilerCallback7_Vtbl {
    pub const fn new<Identity: ICorProfilerCallback7_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ModuleInMemorySymbolsUpdated<Identity: ICorProfilerCallback7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduleid: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback7_Impl::ModuleInMemorySymbolsUpdated(this, core::mem::transmute_copy(&moduleid)).into()
            }
        }
        Self { base__: ICorProfilerCallback6_Vtbl::new::<Identity, OFFSET>(), ModuleInMemorySymbolsUpdated: ModuleInMemorySymbolsUpdated::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorProfilerCallback7 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback as windows_core::Interface>::IID || iid == &<ICorProfilerCallback2 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback3 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback4 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback5 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback6 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICorProfilerCallback7 {}
windows_core::imp::define_interface!(ICorProfilerCallback8, ICorProfilerCallback8_Vtbl, 0x5bed9b15_c079_4d47_bfe2_215a140c07e0);
impl core::ops::Deref for ICorProfilerCallback8 {
    type Target = ICorProfilerCallback7;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICorProfilerCallback8, windows_core::IUnknown, ICorProfilerCallback, ICorProfilerCallback2, ICorProfilerCallback3, ICorProfilerCallback4, ICorProfilerCallback5, ICorProfilerCallback6, ICorProfilerCallback7);
impl ICorProfilerCallback8 {
    pub unsafe fn DynamicMethodJITCompilationStarted(&self, functionid: usize, fissafetoblock: bool, pilheader: *const u8, cbilheader: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DynamicMethodJITCompilationStarted)(windows_core::Interface::as_raw(self), functionid, fissafetoblock.into(), pilheader, cbilheader).ok() }
    }
    pub unsafe fn DynamicMethodJITCompilationFinished(&self, functionid: usize, hrstatus: windows_core::HRESULT, fissafetoblock: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DynamicMethodJITCompilationFinished)(windows_core::Interface::as_raw(self), functionid, hrstatus, fissafetoblock.into()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorProfilerCallback8_Vtbl {
    pub base__: ICorProfilerCallback7_Vtbl,
    pub DynamicMethodJITCompilationStarted: unsafe extern "system" fn(*mut core::ffi::c_void, usize, windows_core::BOOL, *const u8, u32) -> windows_core::HRESULT,
    pub DynamicMethodJITCompilationFinished: unsafe extern "system" fn(*mut core::ffi::c_void, usize, windows_core::HRESULT, windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait ICorProfilerCallback8_Impl: ICorProfilerCallback7_Impl {
    fn DynamicMethodJITCompilationStarted(&self, functionid: usize, fissafetoblock: windows_core::BOOL, pilheader: *const u8, cbilheader: u32) -> windows_core::Result<()>;
    fn DynamicMethodJITCompilationFinished(&self, functionid: usize, hrstatus: windows_core::HRESULT, fissafetoblock: windows_core::BOOL) -> windows_core::Result<()>;
}
impl ICorProfilerCallback8_Vtbl {
    pub const fn new<Identity: ICorProfilerCallback8_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DynamicMethodJITCompilationStarted<Identity: ICorProfilerCallback8_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize, fissafetoblock: windows_core::BOOL, pilheader: *const u8, cbilheader: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback8_Impl::DynamicMethodJITCompilationStarted(this, core::mem::transmute_copy(&functionid), core::mem::transmute_copy(&fissafetoblock), core::mem::transmute_copy(&pilheader), core::mem::transmute_copy(&cbilheader)).into()
            }
        }
        unsafe extern "system" fn DynamicMethodJITCompilationFinished<Identity: ICorProfilerCallback8_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize, hrstatus: windows_core::HRESULT, fissafetoblock: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback8_Impl::DynamicMethodJITCompilationFinished(this, core::mem::transmute_copy(&functionid), core::mem::transmute_copy(&hrstatus), core::mem::transmute_copy(&fissafetoblock)).into()
            }
        }
        Self {
            base__: ICorProfilerCallback7_Vtbl::new::<Identity, OFFSET>(),
            DynamicMethodJITCompilationStarted: DynamicMethodJITCompilationStarted::<Identity, OFFSET>,
            DynamicMethodJITCompilationFinished: DynamicMethodJITCompilationFinished::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorProfilerCallback8 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback as windows_core::Interface>::IID || iid == &<ICorProfilerCallback2 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback3 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback4 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback5 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback6 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback7 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICorProfilerCallback8 {}
windows_core::imp::define_interface!(ICorProfilerCallback9, ICorProfilerCallback9_Vtbl, 0x27583ec3_c8f5_482f_8052_194b8ce4705a);
impl core::ops::Deref for ICorProfilerCallback9 {
    type Target = ICorProfilerCallback8;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICorProfilerCallback9, windows_core::IUnknown, ICorProfilerCallback, ICorProfilerCallback2, ICorProfilerCallback3, ICorProfilerCallback4, ICorProfilerCallback5, ICorProfilerCallback6, ICorProfilerCallback7, ICorProfilerCallback8);
impl ICorProfilerCallback9 {
    pub unsafe fn DynamicMethodUnloaded(&self, functionid: usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DynamicMethodUnloaded)(windows_core::Interface::as_raw(self), functionid).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorProfilerCallback9_Vtbl {
    pub base__: ICorProfilerCallback8_Vtbl,
    pub DynamicMethodUnloaded: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
}
pub trait ICorProfilerCallback9_Impl: ICorProfilerCallback8_Impl {
    fn DynamicMethodUnloaded(&self, functionid: usize) -> windows_core::Result<()>;
}
impl ICorProfilerCallback9_Vtbl {
    pub const fn new<Identity: ICorProfilerCallback9_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DynamicMethodUnloaded<Identity: ICorProfilerCallback9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerCallback9_Impl::DynamicMethodUnloaded(this, core::mem::transmute_copy(&functionid)).into()
            }
        }
        Self { base__: ICorProfilerCallback8_Vtbl::new::<Identity, OFFSET>(), DynamicMethodUnloaded: DynamicMethodUnloaded::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorProfilerCallback9 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback as windows_core::Interface>::IID || iid == &<ICorProfilerCallback2 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback3 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback4 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback5 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback6 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback7 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback8 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICorProfilerCallback9 {}
windows_core::imp::define_interface!(ICorProfilerFunctionControl, ICorProfilerFunctionControl_Vtbl, 0xf0963021_e1ea_4732_8581_e01b0bd3c0c6);
windows_core::imp::interface_hierarchy!(ICorProfilerFunctionControl, windows_core::IUnknown);
impl ICorProfilerFunctionControl {
    pub unsafe fn SetCodegenFlags(&self, flags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCodegenFlags)(windows_core::Interface::as_raw(self), flags).ok() }
    }
    pub unsafe fn SetILFunctionBody(&self, pbnewilmethodheader: &[u8]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetILFunctionBody)(windows_core::Interface::as_raw(self), pbnewilmethodheader.len().try_into().unwrap(), core::mem::transmute(pbnewilmethodheader.as_ptr())).ok() }
    }
    pub unsafe fn SetILInstrumentedCodeMap(&self, rgilmapentries: &[COR_IL_MAP]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetILInstrumentedCodeMap)(windows_core::Interface::as_raw(self), rgilmapentries.len().try_into().unwrap(), core::mem::transmute(rgilmapentries.as_ptr())).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorProfilerFunctionControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetCodegenFlags: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetILFunctionBody: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8) -> windows_core::HRESULT,
    pub SetILInstrumentedCodeMap: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const COR_IL_MAP) -> windows_core::HRESULT,
}
pub trait ICorProfilerFunctionControl_Impl: windows_core::IUnknownImpl {
    fn SetCodegenFlags(&self, flags: u32) -> windows_core::Result<()>;
    fn SetILFunctionBody(&self, cbnewilmethodheader: u32, pbnewilmethodheader: *const u8) -> windows_core::Result<()>;
    fn SetILInstrumentedCodeMap(&self, cilmapentries: u32, rgilmapentries: *const COR_IL_MAP) -> windows_core::Result<()>;
}
impl ICorProfilerFunctionControl_Vtbl {
    pub const fn new<Identity: ICorProfilerFunctionControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetCodegenFlags<Identity: ICorProfilerFunctionControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerFunctionControl_Impl::SetCodegenFlags(this, core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn SetILFunctionBody<Identity: ICorProfilerFunctionControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbnewilmethodheader: u32, pbnewilmethodheader: *const u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerFunctionControl_Impl::SetILFunctionBody(this, core::mem::transmute_copy(&cbnewilmethodheader), core::mem::transmute_copy(&pbnewilmethodheader)).into()
            }
        }
        unsafe extern "system" fn SetILInstrumentedCodeMap<Identity: ICorProfilerFunctionControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cilmapentries: u32, rgilmapentries: *const COR_IL_MAP) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerFunctionControl_Impl::SetILInstrumentedCodeMap(this, core::mem::transmute_copy(&cilmapentries), core::mem::transmute_copy(&rgilmapentries)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetCodegenFlags: SetCodegenFlags::<Identity, OFFSET>,
            SetILFunctionBody: SetILFunctionBody::<Identity, OFFSET>,
            SetILInstrumentedCodeMap: SetILInstrumentedCodeMap::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorProfilerFunctionControl as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICorProfilerFunctionControl {}
windows_core::imp::define_interface!(ICorProfilerFunctionEnum, ICorProfilerFunctionEnum_Vtbl, 0xff71301a_b994_429d_a10b_b345a65280ef);
windows_core::imp::interface_hierarchy!(ICorProfilerFunctionEnum, windows_core::IUnknown);
impl ICorProfilerFunctionEnum {
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<ICorProfilerFunctionEnum> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Next(&self, ids: &mut [COR_PRF_FUNCTION], pceltfetched: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ids.len().try_into().unwrap(), core::mem::transmute(ids.as_ptr()), pceltfetched as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorProfilerFunctionEnum_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut COR_PRF_FUNCTION, *mut u32) -> windows_core::HRESULT,
}
pub trait ICorProfilerFunctionEnum_Impl: windows_core::IUnknownImpl {
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<ICorProfilerFunctionEnum>;
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn Next(&self, celt: u32, ids: *mut COR_PRF_FUNCTION, pceltfetched: *mut u32) -> windows_core::Result<()>;
}
impl ICorProfilerFunctionEnum_Vtbl {
    pub const fn new<Identity: ICorProfilerFunctionEnum_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Skip<Identity: ICorProfilerFunctionEnum_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerFunctionEnum_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: ICorProfilerFunctionEnum_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerFunctionEnum_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: ICorProfilerFunctionEnum_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorProfilerFunctionEnum_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCount<Identity: ICorProfilerFunctionEnum_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcelt: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorProfilerFunctionEnum_Impl::GetCount(this) {
                    Ok(ok__) => {
                        pcelt.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Next<Identity: ICorProfilerFunctionEnum_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ids: *mut COR_PRF_FUNCTION, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerFunctionEnum_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ids), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorProfilerFunctionEnum as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICorProfilerFunctionEnum {}
windows_core::imp::define_interface!(ICorProfilerInfo, ICorProfilerInfo_Vtbl, 0x28b5557d_3f3f_48b4_90b2_5f9eea2f6c48);
windows_core::imp::interface_hierarchy!(ICorProfilerInfo, windows_core::IUnknown);
impl ICorProfilerInfo {
    pub unsafe fn GetClassFromObject(&self, objectid: usize) -> windows_core::Result<usize> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetClassFromObject)(windows_core::Interface::as_raw(self), objectid, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetClassFromToken(&self, moduleid: usize, typedef: u32) -> windows_core::Result<usize> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetClassFromToken)(windows_core::Interface::as_raw(self), moduleid, typedef, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCodeInfo(&self, functionid: usize, pstart: *mut *mut u8, pcsize: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCodeInfo)(windows_core::Interface::as_raw(self), functionid, pstart as _, pcsize as _).ok() }
    }
    pub unsafe fn GetEventMask(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEventMask)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetFunctionFromIP(&self, ip: *const u8) -> windows_core::Result<usize> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFunctionFromIP)(windows_core::Interface::as_raw(self), ip, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetFunctionFromToken(&self, moduleid: usize, token: u32) -> windows_core::Result<usize> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFunctionFromToken)(windows_core::Interface::as_raw(self), moduleid, token, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetHandleFromThread(&self, threadid: usize) -> windows_core::Result<super::super::super::Foundation::HANDLE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHandleFromThread)(windows_core::Interface::as_raw(self), threadid, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetObjectSize(&self, objectid: usize) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetObjectSize)(windows_core::Interface::as_raw(self), objectid, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_System_WinRT_Metadata")]
    pub unsafe fn IsArrayClass(&self, classid: usize, pbaseelemtype: *mut super::super::WinRT::Metadata::CorElementType, pbaseclassid: *mut usize, pcrank: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).IsArrayClass)(windows_core::Interface::as_raw(self), classid, pbaseelemtype as _, pbaseclassid as _, pcrank as _).ok() }
    }
    pub unsafe fn GetThreadInfo(&self, threadid: usize) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetThreadInfo)(windows_core::Interface::as_raw(self), threadid, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCurrentThreadID(&self) -> windows_core::Result<usize> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentThreadID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetClassIDInfo(&self, classid: usize, pmoduleid: *mut usize, ptypedeftoken: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetClassIDInfo)(windows_core::Interface::as_raw(self), classid, pmoduleid as _, ptypedeftoken as _).ok() }
    }
    pub unsafe fn GetFunctionInfo(&self, functionid: usize, pclassid: *mut usize, pmoduleid: *mut usize, ptoken: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetFunctionInfo)(windows_core::Interface::as_raw(self), functionid, pclassid as _, pmoduleid as _, ptoken as _).ok() }
    }
    pub unsafe fn SetEventMask(&self, dwevents: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEventMask)(windows_core::Interface::as_raw(self), dwevents).ok() }
    }
    pub unsafe fn SetEnterLeaveFunctionHooks(&self, pfuncenter: *const FunctionEnter, pfuncleave: *const FunctionLeave, pfunctailcall: *const FunctionTailcall) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEnterLeaveFunctionHooks)(windows_core::Interface::as_raw(self), pfuncenter, pfuncleave, pfunctailcall).ok() }
    }
    pub unsafe fn SetFunctionIDMapper(&self, pfunc: *const FunctionIDMapper) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFunctionIDMapper)(windows_core::Interface::as_raw(self), pfunc).ok() }
    }
    pub unsafe fn GetTokenAndMetaDataFromFunction(&self, functionid: usize, riid: *const windows_core::GUID, ppimport: *mut Option<windows_core::IUnknown>, ptoken: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetTokenAndMetaDataFromFunction)(windows_core::Interface::as_raw(self), functionid, riid, core::mem::transmute(ppimport), ptoken as _).ok() }
    }
    pub unsafe fn GetModuleInfo(&self, moduleid: usize, ppbaseloadaddress: *mut *mut u8, pcchname: *mut u32, szname: &mut [u16], passemblyid: *mut usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetModuleInfo)(windows_core::Interface::as_raw(self), moduleid, ppbaseloadaddress as _, szname.len().try_into().unwrap(), pcchname as _, core::mem::transmute(szname.as_ptr()), passemblyid as _).ok() }
    }
    pub unsafe fn GetModuleMetaData(&self, moduleid: usize, dwopenflags: u32, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetModuleMetaData)(windows_core::Interface::as_raw(self), moduleid, dwopenflags, riid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetILFunctionBody(&self, moduleid: usize, methodid: u32, ppmethodheader: *mut *mut u8, pcbmethodsize: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetILFunctionBody)(windows_core::Interface::as_raw(self), moduleid, methodid, ppmethodheader as _, pcbmethodsize as _).ok() }
    }
    pub unsafe fn GetILFunctionBodyAllocator(&self, moduleid: usize) -> windows_core::Result<IMethodMalloc> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetILFunctionBodyAllocator)(windows_core::Interface::as_raw(self), moduleid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetILFunctionBody(&self, moduleid: usize, methodid: u32, pbnewilmethodheader: *const u8) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetILFunctionBody)(windows_core::Interface::as_raw(self), moduleid, methodid, pbnewilmethodheader).ok() }
    }
    pub unsafe fn GetAppDomainInfo(&self, appdomainid: usize, pcchname: *mut u32, szname: &mut [u16], pprocessid: *mut usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetAppDomainInfo)(windows_core::Interface::as_raw(self), appdomainid, szname.len().try_into().unwrap(), pcchname as _, core::mem::transmute(szname.as_ptr()), pprocessid as _).ok() }
    }
    pub unsafe fn GetAssemblyInfo(&self, assemblyid: usize, pcchname: *mut u32, szname: &mut [u16], pappdomainid: *mut usize, pmoduleid: *mut usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetAssemblyInfo)(windows_core::Interface::as_raw(self), assemblyid, szname.len().try_into().unwrap(), pcchname as _, core::mem::transmute(szname.as_ptr()), pappdomainid as _, pmoduleid as _).ok() }
    }
    pub unsafe fn SetFunctionReJIT(&self, functionid: usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFunctionReJIT)(windows_core::Interface::as_raw(self), functionid).ok() }
    }
    pub unsafe fn ForceGC(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ForceGC)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SetILInstrumentedCodeMap(&self, functionid: usize, fstartjit: bool, rgilmapentries: &[COR_IL_MAP]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetILInstrumentedCodeMap)(windows_core::Interface::as_raw(self), functionid, fstartjit.into(), rgilmapentries.len().try_into().unwrap(), core::mem::transmute(rgilmapentries.as_ptr())).ok() }
    }
    pub unsafe fn GetInprocInspectionInterface(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInprocInspectionInterface)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetInprocInspectionIThisThread(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInprocInspectionIThisThread)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetThreadContext(&self, threadid: usize) -> windows_core::Result<usize> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetThreadContext)(windows_core::Interface::as_raw(self), threadid, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn BeginInprocDebugging(&self, fthisthreadonly: bool) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BeginInprocDebugging)(windows_core::Interface::as_raw(self), fthisthreadonly.into(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EndInprocDebugging(&self, dwprofilercontext: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EndInprocDebugging)(windows_core::Interface::as_raw(self), dwprofilercontext).ok() }
    }
    pub unsafe fn GetILToNativeMapping(&self, functionid: usize, pcmap: *mut u32, map: &mut [COR_DEBUG_IL_TO_NATIVE_MAP]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetILToNativeMapping)(windows_core::Interface::as_raw(self), functionid, map.len().try_into().unwrap(), pcmap as _, core::mem::transmute(map.as_ptr())).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorProfilerInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetClassFromObject: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut usize) -> windows_core::HRESULT,
    pub GetClassFromToken: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32, *mut usize) -> windows_core::HRESULT,
    pub GetCodeInfo: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    pub GetEventMask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetFunctionFromIP: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, *mut usize) -> windows_core::HRESULT,
    pub GetFunctionFromToken: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32, *mut usize) -> windows_core::HRESULT,
    pub GetHandleFromThread: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut super::super::super::Foundation::HANDLE) -> windows_core::HRESULT,
    pub GetObjectSize: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_WinRT_Metadata")]
    pub IsArrayClass: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut super::super::WinRT::Metadata::CorElementType, *mut usize, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_WinRT_Metadata"))]
    IsArrayClass: usize,
    pub GetThreadInfo: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut u32) -> windows_core::HRESULT,
    pub GetCurrentThreadID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut usize) -> windows_core::HRESULT,
    pub GetClassIDInfo: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut usize, *mut u32) -> windows_core::HRESULT,
    pub GetFunctionInfo: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut usize, *mut usize, *mut u32) -> windows_core::HRESULT,
    pub SetEventMask: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetEnterLeaveFunctionHooks: unsafe extern "system" fn(*mut core::ffi::c_void, *const FunctionEnter, *const FunctionLeave, *const FunctionTailcall) -> windows_core::HRESULT,
    pub SetFunctionIDMapper: unsafe extern "system" fn(*mut core::ffi::c_void, *const FunctionIDMapper) -> windows_core::HRESULT,
    pub GetTokenAndMetaDataFromFunction: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *const windows_core::GUID, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetModuleInfo: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut *mut u8, u32, *mut u32, windows_core::PWSTR, *mut usize) -> windows_core::HRESULT,
    pub GetModuleMetaData: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetILFunctionBody: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    pub GetILFunctionBodyAllocator: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetILFunctionBody: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32, *const u8) -> windows_core::HRESULT,
    pub GetAppDomainInfo: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32, *mut u32, windows_core::PWSTR, *mut usize) -> windows_core::HRESULT,
    pub GetAssemblyInfo: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32, *mut u32, windows_core::PWSTR, *mut usize, *mut usize) -> windows_core::HRESULT,
    pub SetFunctionReJIT: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    pub ForceGC: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetILInstrumentedCodeMap: unsafe extern "system" fn(*mut core::ffi::c_void, usize, windows_core::BOOL, u32, *const COR_IL_MAP) -> windows_core::HRESULT,
    pub GetInprocInspectionInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetInprocInspectionIThisThread: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetThreadContext: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut usize) -> windows_core::HRESULT,
    pub BeginInprocDebugging: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, *mut u32) -> windows_core::HRESULT,
    pub EndInprocDebugging: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetILToNativeMapping: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32, *mut u32, *mut COR_DEBUG_IL_TO_NATIVE_MAP) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
pub trait ICorProfilerInfo_Impl: windows_core::IUnknownImpl {
    fn GetClassFromObject(&self, objectid: usize) -> windows_core::Result<usize>;
    fn GetClassFromToken(&self, moduleid: usize, typedef: u32) -> windows_core::Result<usize>;
    fn GetCodeInfo(&self, functionid: usize, pstart: *mut *mut u8, pcsize: *mut u32) -> windows_core::Result<()>;
    fn GetEventMask(&self) -> windows_core::Result<u32>;
    fn GetFunctionFromIP(&self, ip: *const u8) -> windows_core::Result<usize>;
    fn GetFunctionFromToken(&self, moduleid: usize, token: u32) -> windows_core::Result<usize>;
    fn GetHandleFromThread(&self, threadid: usize) -> windows_core::Result<super::super::super::Foundation::HANDLE>;
    fn GetObjectSize(&self, objectid: usize) -> windows_core::Result<u32>;
    fn IsArrayClass(&self, classid: usize, pbaseelemtype: *mut super::super::WinRT::Metadata::CorElementType, pbaseclassid: *mut usize, pcrank: *mut u32) -> windows_core::Result<()>;
    fn GetThreadInfo(&self, threadid: usize) -> windows_core::Result<u32>;
    fn GetCurrentThreadID(&self) -> windows_core::Result<usize>;
    fn GetClassIDInfo(&self, classid: usize, pmoduleid: *mut usize, ptypedeftoken: *mut u32) -> windows_core::Result<()>;
    fn GetFunctionInfo(&self, functionid: usize, pclassid: *mut usize, pmoduleid: *mut usize, ptoken: *mut u32) -> windows_core::Result<()>;
    fn SetEventMask(&self, dwevents: u32) -> windows_core::Result<()>;
    fn SetEnterLeaveFunctionHooks(&self, pfuncenter: *const FunctionEnter, pfuncleave: *const FunctionLeave, pfunctailcall: *const FunctionTailcall) -> windows_core::Result<()>;
    fn SetFunctionIDMapper(&self, pfunc: *const FunctionIDMapper) -> windows_core::Result<()>;
    fn GetTokenAndMetaDataFromFunction(&self, functionid: usize, riid: *const windows_core::GUID, ppimport: windows_core::OutRef<windows_core::IUnknown>, ptoken: *mut u32) -> windows_core::Result<()>;
    fn GetModuleInfo(&self, moduleid: usize, ppbaseloadaddress: *mut *mut u8, cchname: u32, pcchname: *mut u32, szname: windows_core::PWSTR, passemblyid: *mut usize) -> windows_core::Result<()>;
    fn GetModuleMetaData(&self, moduleid: usize, dwopenflags: u32, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
    fn GetILFunctionBody(&self, moduleid: usize, methodid: u32, ppmethodheader: *mut *mut u8, pcbmethodsize: *mut u32) -> windows_core::Result<()>;
    fn GetILFunctionBodyAllocator(&self, moduleid: usize) -> windows_core::Result<IMethodMalloc>;
    fn SetILFunctionBody(&self, moduleid: usize, methodid: u32, pbnewilmethodheader: *const u8) -> windows_core::Result<()>;
    fn GetAppDomainInfo(&self, appdomainid: usize, cchname: u32, pcchname: *mut u32, szname: windows_core::PWSTR, pprocessid: *mut usize) -> windows_core::Result<()>;
    fn GetAssemblyInfo(&self, assemblyid: usize, cchname: u32, pcchname: *mut u32, szname: windows_core::PWSTR, pappdomainid: *mut usize, pmoduleid: *mut usize) -> windows_core::Result<()>;
    fn SetFunctionReJIT(&self, functionid: usize) -> windows_core::Result<()>;
    fn ForceGC(&self) -> windows_core::Result<()>;
    fn SetILInstrumentedCodeMap(&self, functionid: usize, fstartjit: windows_core::BOOL, cilmapentries: u32, rgilmapentries: *const COR_IL_MAP) -> windows_core::Result<()>;
    fn GetInprocInspectionInterface(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn GetInprocInspectionIThisThread(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn GetThreadContext(&self, threadid: usize) -> windows_core::Result<usize>;
    fn BeginInprocDebugging(&self, fthisthreadonly: windows_core::BOOL) -> windows_core::Result<u32>;
    fn EndInprocDebugging(&self, dwprofilercontext: u32) -> windows_core::Result<()>;
    fn GetILToNativeMapping(&self, functionid: usize, cmap: u32, pcmap: *mut u32, map: *mut COR_DEBUG_IL_TO_NATIVE_MAP) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl ICorProfilerInfo_Vtbl {
    pub const fn new<Identity: ICorProfilerInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetClassFromObject<Identity: ICorProfilerInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objectid: usize, pclassid: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorProfilerInfo_Impl::GetClassFromObject(this, core::mem::transmute_copy(&objectid)) {
                    Ok(ok__) => {
                        pclassid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetClassFromToken<Identity: ICorProfilerInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduleid: usize, typedef: u32, pclassid: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorProfilerInfo_Impl::GetClassFromToken(this, core::mem::transmute_copy(&moduleid), core::mem::transmute_copy(&typedef)) {
                    Ok(ok__) => {
                        pclassid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCodeInfo<Identity: ICorProfilerInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize, pstart: *mut *mut u8, pcsize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo_Impl::GetCodeInfo(this, core::mem::transmute_copy(&functionid), core::mem::transmute_copy(&pstart), core::mem::transmute_copy(&pcsize)).into()
            }
        }
        unsafe extern "system" fn GetEventMask<Identity: ICorProfilerInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwevents: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorProfilerInfo_Impl::GetEventMask(this) {
                    Ok(ok__) => {
                        pdwevents.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFunctionFromIP<Identity: ICorProfilerInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ip: *const u8, pfunctionid: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorProfilerInfo_Impl::GetFunctionFromIP(this, core::mem::transmute_copy(&ip)) {
                    Ok(ok__) => {
                        pfunctionid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFunctionFromToken<Identity: ICorProfilerInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduleid: usize, token: u32, pfunctionid: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorProfilerInfo_Impl::GetFunctionFromToken(this, core::mem::transmute_copy(&moduleid), core::mem::transmute_copy(&token)) {
                    Ok(ok__) => {
                        pfunctionid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetHandleFromThread<Identity: ICorProfilerInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, threadid: usize, phthread: *mut super::super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorProfilerInfo_Impl::GetHandleFromThread(this, core::mem::transmute_copy(&threadid)) {
                    Ok(ok__) => {
                        phthread.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetObjectSize<Identity: ICorProfilerInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objectid: usize, pcsize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorProfilerInfo_Impl::GetObjectSize(this, core::mem::transmute_copy(&objectid)) {
                    Ok(ok__) => {
                        pcsize.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsArrayClass<Identity: ICorProfilerInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, classid: usize, pbaseelemtype: *mut super::super::WinRT::Metadata::CorElementType, pbaseclassid: *mut usize, pcrank: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo_Impl::IsArrayClass(this, core::mem::transmute_copy(&classid), core::mem::transmute_copy(&pbaseelemtype), core::mem::transmute_copy(&pbaseclassid), core::mem::transmute_copy(&pcrank)).into()
            }
        }
        unsafe extern "system" fn GetThreadInfo<Identity: ICorProfilerInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, threadid: usize, pdwwin32threadid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorProfilerInfo_Impl::GetThreadInfo(this, core::mem::transmute_copy(&threadid)) {
                    Ok(ok__) => {
                        pdwwin32threadid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCurrentThreadID<Identity: ICorProfilerInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pthreadid: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorProfilerInfo_Impl::GetCurrentThreadID(this) {
                    Ok(ok__) => {
                        pthreadid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetClassIDInfo<Identity: ICorProfilerInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, classid: usize, pmoduleid: *mut usize, ptypedeftoken: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo_Impl::GetClassIDInfo(this, core::mem::transmute_copy(&classid), core::mem::transmute_copy(&pmoduleid), core::mem::transmute_copy(&ptypedeftoken)).into()
            }
        }
        unsafe extern "system" fn GetFunctionInfo<Identity: ICorProfilerInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize, pclassid: *mut usize, pmoduleid: *mut usize, ptoken: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo_Impl::GetFunctionInfo(this, core::mem::transmute_copy(&functionid), core::mem::transmute_copy(&pclassid), core::mem::transmute_copy(&pmoduleid), core::mem::transmute_copy(&ptoken)).into()
            }
        }
        unsafe extern "system" fn SetEventMask<Identity: ICorProfilerInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwevents: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo_Impl::SetEventMask(this, core::mem::transmute_copy(&dwevents)).into()
            }
        }
        unsafe extern "system" fn SetEnterLeaveFunctionHooks<Identity: ICorProfilerInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfuncenter: *const FunctionEnter, pfuncleave: *const FunctionLeave, pfunctailcall: *const FunctionTailcall) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo_Impl::SetEnterLeaveFunctionHooks(this, core::mem::transmute_copy(&pfuncenter), core::mem::transmute_copy(&pfuncleave), core::mem::transmute_copy(&pfunctailcall)).into()
            }
        }
        unsafe extern "system" fn SetFunctionIDMapper<Identity: ICorProfilerInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfunc: *const FunctionIDMapper) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo_Impl::SetFunctionIDMapper(this, core::mem::transmute_copy(&pfunc)).into()
            }
        }
        unsafe extern "system" fn GetTokenAndMetaDataFromFunction<Identity: ICorProfilerInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize, riid: *const windows_core::GUID, ppimport: *mut *mut core::ffi::c_void, ptoken: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo_Impl::GetTokenAndMetaDataFromFunction(this, core::mem::transmute_copy(&functionid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppimport), core::mem::transmute_copy(&ptoken)).into()
            }
        }
        unsafe extern "system" fn GetModuleInfo<Identity: ICorProfilerInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduleid: usize, ppbaseloadaddress: *mut *mut u8, cchname: u32, pcchname: *mut u32, szname: windows_core::PWSTR, passemblyid: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo_Impl::GetModuleInfo(this, core::mem::transmute_copy(&moduleid), core::mem::transmute_copy(&ppbaseloadaddress), core::mem::transmute_copy(&cchname), core::mem::transmute_copy(&pcchname), core::mem::transmute_copy(&szname), core::mem::transmute_copy(&passemblyid)).into()
            }
        }
        unsafe extern "system" fn GetModuleMetaData<Identity: ICorProfilerInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduleid: usize, dwopenflags: u32, riid: *const windows_core::GUID, ppout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorProfilerInfo_Impl::GetModuleMetaData(this, core::mem::transmute_copy(&moduleid), core::mem::transmute_copy(&dwopenflags), core::mem::transmute_copy(&riid)) {
                    Ok(ok__) => {
                        ppout.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetILFunctionBody<Identity: ICorProfilerInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduleid: usize, methodid: u32, ppmethodheader: *mut *mut u8, pcbmethodsize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo_Impl::GetILFunctionBody(this, core::mem::transmute_copy(&moduleid), core::mem::transmute_copy(&methodid), core::mem::transmute_copy(&ppmethodheader), core::mem::transmute_copy(&pcbmethodsize)).into()
            }
        }
        unsafe extern "system" fn GetILFunctionBodyAllocator<Identity: ICorProfilerInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduleid: usize, ppmalloc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorProfilerInfo_Impl::GetILFunctionBodyAllocator(this, core::mem::transmute_copy(&moduleid)) {
                    Ok(ok__) => {
                        ppmalloc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetILFunctionBody<Identity: ICorProfilerInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduleid: usize, methodid: u32, pbnewilmethodheader: *const u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo_Impl::SetILFunctionBody(this, core::mem::transmute_copy(&moduleid), core::mem::transmute_copy(&methodid), core::mem::transmute_copy(&pbnewilmethodheader)).into()
            }
        }
        unsafe extern "system" fn GetAppDomainInfo<Identity: ICorProfilerInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appdomainid: usize, cchname: u32, pcchname: *mut u32, szname: windows_core::PWSTR, pprocessid: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo_Impl::GetAppDomainInfo(this, core::mem::transmute_copy(&appdomainid), core::mem::transmute_copy(&cchname), core::mem::transmute_copy(&pcchname), core::mem::transmute_copy(&szname), core::mem::transmute_copy(&pprocessid)).into()
            }
        }
        unsafe extern "system" fn GetAssemblyInfo<Identity: ICorProfilerInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, assemblyid: usize, cchname: u32, pcchname: *mut u32, szname: windows_core::PWSTR, pappdomainid: *mut usize, pmoduleid: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo_Impl::GetAssemblyInfo(this, core::mem::transmute_copy(&assemblyid), core::mem::transmute_copy(&cchname), core::mem::transmute_copy(&pcchname), core::mem::transmute_copy(&szname), core::mem::transmute_copy(&pappdomainid), core::mem::transmute_copy(&pmoduleid)).into()
            }
        }
        unsafe extern "system" fn SetFunctionReJIT<Identity: ICorProfilerInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo_Impl::SetFunctionReJIT(this, core::mem::transmute_copy(&functionid)).into()
            }
        }
        unsafe extern "system" fn ForceGC<Identity: ICorProfilerInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo_Impl::ForceGC(this).into()
            }
        }
        unsafe extern "system" fn SetILInstrumentedCodeMap<Identity: ICorProfilerInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize, fstartjit: windows_core::BOOL, cilmapentries: u32, rgilmapentries: *const COR_IL_MAP) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo_Impl::SetILInstrumentedCodeMap(this, core::mem::transmute_copy(&functionid), core::mem::transmute_copy(&fstartjit), core::mem::transmute_copy(&cilmapentries), core::mem::transmute_copy(&rgilmapentries)).into()
            }
        }
        unsafe extern "system" fn GetInprocInspectionInterface<Identity: ICorProfilerInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppicd: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorProfilerInfo_Impl::GetInprocInspectionInterface(this) {
                    Ok(ok__) => {
                        ppicd.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetInprocInspectionIThisThread<Identity: ICorProfilerInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppicd: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorProfilerInfo_Impl::GetInprocInspectionIThisThread(this) {
                    Ok(ok__) => {
                        ppicd.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetThreadContext<Identity: ICorProfilerInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, threadid: usize, pcontextid: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorProfilerInfo_Impl::GetThreadContext(this, core::mem::transmute_copy(&threadid)) {
                    Ok(ok__) => {
                        pcontextid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BeginInprocDebugging<Identity: ICorProfilerInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fthisthreadonly: windows_core::BOOL, pdwprofilercontext: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorProfilerInfo_Impl::BeginInprocDebugging(this, core::mem::transmute_copy(&fthisthreadonly)) {
                    Ok(ok__) => {
                        pdwprofilercontext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EndInprocDebugging<Identity: ICorProfilerInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwprofilercontext: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo_Impl::EndInprocDebugging(this, core::mem::transmute_copy(&dwprofilercontext)).into()
            }
        }
        unsafe extern "system" fn GetILToNativeMapping<Identity: ICorProfilerInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize, cmap: u32, pcmap: *mut u32, map: *mut COR_DEBUG_IL_TO_NATIVE_MAP) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo_Impl::GetILToNativeMapping(this, core::mem::transmute_copy(&functionid), core::mem::transmute_copy(&cmap), core::mem::transmute_copy(&pcmap), core::mem::transmute_copy(&map)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetClassFromObject: GetClassFromObject::<Identity, OFFSET>,
            GetClassFromToken: GetClassFromToken::<Identity, OFFSET>,
            GetCodeInfo: GetCodeInfo::<Identity, OFFSET>,
            GetEventMask: GetEventMask::<Identity, OFFSET>,
            GetFunctionFromIP: GetFunctionFromIP::<Identity, OFFSET>,
            GetFunctionFromToken: GetFunctionFromToken::<Identity, OFFSET>,
            GetHandleFromThread: GetHandleFromThread::<Identity, OFFSET>,
            GetObjectSize: GetObjectSize::<Identity, OFFSET>,
            IsArrayClass: IsArrayClass::<Identity, OFFSET>,
            GetThreadInfo: GetThreadInfo::<Identity, OFFSET>,
            GetCurrentThreadID: GetCurrentThreadID::<Identity, OFFSET>,
            GetClassIDInfo: GetClassIDInfo::<Identity, OFFSET>,
            GetFunctionInfo: GetFunctionInfo::<Identity, OFFSET>,
            SetEventMask: SetEventMask::<Identity, OFFSET>,
            SetEnterLeaveFunctionHooks: SetEnterLeaveFunctionHooks::<Identity, OFFSET>,
            SetFunctionIDMapper: SetFunctionIDMapper::<Identity, OFFSET>,
            GetTokenAndMetaDataFromFunction: GetTokenAndMetaDataFromFunction::<Identity, OFFSET>,
            GetModuleInfo: GetModuleInfo::<Identity, OFFSET>,
            GetModuleMetaData: GetModuleMetaData::<Identity, OFFSET>,
            GetILFunctionBody: GetILFunctionBody::<Identity, OFFSET>,
            GetILFunctionBodyAllocator: GetILFunctionBodyAllocator::<Identity, OFFSET>,
            SetILFunctionBody: SetILFunctionBody::<Identity, OFFSET>,
            GetAppDomainInfo: GetAppDomainInfo::<Identity, OFFSET>,
            GetAssemblyInfo: GetAssemblyInfo::<Identity, OFFSET>,
            SetFunctionReJIT: SetFunctionReJIT::<Identity, OFFSET>,
            ForceGC: ForceGC::<Identity, OFFSET>,
            SetILInstrumentedCodeMap: SetILInstrumentedCodeMap::<Identity, OFFSET>,
            GetInprocInspectionInterface: GetInprocInspectionInterface::<Identity, OFFSET>,
            GetInprocInspectionIThisThread: GetInprocInspectionIThisThread::<Identity, OFFSET>,
            GetThreadContext: GetThreadContext::<Identity, OFFSET>,
            BeginInprocDebugging: BeginInprocDebugging::<Identity, OFFSET>,
            EndInprocDebugging: EndInprocDebugging::<Identity, OFFSET>,
            GetILToNativeMapping: GetILToNativeMapping::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorProfilerInfo as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl windows_core::RuntimeName for ICorProfilerInfo {}
windows_core::imp::define_interface!(ICorProfilerInfo10, ICorProfilerInfo10_Vtbl, 0x2f1b5152_c869_40c9_aa5f_3abe026bd720);
impl core::ops::Deref for ICorProfilerInfo10 {
    type Target = ICorProfilerInfo9;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICorProfilerInfo10, windows_core::IUnknown, ICorProfilerInfo, ICorProfilerInfo2, ICorProfilerInfo3, ICorProfilerInfo4, ICorProfilerInfo5, ICorProfilerInfo6, ICorProfilerInfo7, ICorProfilerInfo8, ICorProfilerInfo9);
impl ICorProfilerInfo10 {
    pub unsafe fn EnumerateObjectReferences(&self, objectid: usize, callback: ObjectReferenceCallback, clientdata: *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnumerateObjectReferences)(windows_core::Interface::as_raw(self), objectid, callback, clientdata as _).ok() }
    }
    pub unsafe fn IsFrozenObject(&self, objectid: usize, pbfrozen: *mut windows_core::BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).IsFrozenObject)(windows_core::Interface::as_raw(self), objectid, pbfrozen as _).ok() }
    }
    pub unsafe fn GetLOHObjectSizeThreshold(&self, pthreshold: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetLOHObjectSizeThreshold)(windows_core::Interface::as_raw(self), pthreshold as _).ok() }
    }
    pub unsafe fn RequestReJITWithInliners(&self, dwrejitflags: u32, cfunctions: u32, moduleids: *const usize, methodids: *const u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RequestReJITWithInliners)(windows_core::Interface::as_raw(self), dwrejitflags, cfunctions, moduleids, methodids).ok() }
    }
    pub unsafe fn SuspendRuntime(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SuspendRuntime)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn ResumeRuntime(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ResumeRuntime)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorProfilerInfo10_Vtbl {
    pub base__: ICorProfilerInfo9_Vtbl,
    pub EnumerateObjectReferences: unsafe extern "system" fn(*mut core::ffi::c_void, usize, ObjectReferenceCallback, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsFrozenObject: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetLOHObjectSizeThreshold: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub RequestReJITWithInliners: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const usize, *const u32) -> windows_core::HRESULT,
    pub SuspendRuntime: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ResumeRuntime: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
pub trait ICorProfilerInfo10_Impl: ICorProfilerInfo9_Impl {
    fn EnumerateObjectReferences(&self, objectid: usize, callback: ObjectReferenceCallback, clientdata: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn IsFrozenObject(&self, objectid: usize, pbfrozen: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn GetLOHObjectSizeThreshold(&self, pthreshold: *mut u32) -> windows_core::Result<()>;
    fn RequestReJITWithInliners(&self, dwrejitflags: u32, cfunctions: u32, moduleids: *const usize, methodids: *const u32) -> windows_core::Result<()>;
    fn SuspendRuntime(&self) -> windows_core::Result<()>;
    fn ResumeRuntime(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl ICorProfilerInfo10_Vtbl {
    pub const fn new<Identity: ICorProfilerInfo10_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumerateObjectReferences<Identity: ICorProfilerInfo10_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objectid: usize, callback: ObjectReferenceCallback, clientdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo10_Impl::EnumerateObjectReferences(this, core::mem::transmute_copy(&objectid), core::mem::transmute_copy(&callback), core::mem::transmute_copy(&clientdata)).into()
            }
        }
        unsafe extern "system" fn IsFrozenObject<Identity: ICorProfilerInfo10_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objectid: usize, pbfrozen: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo10_Impl::IsFrozenObject(this, core::mem::transmute_copy(&objectid), core::mem::transmute_copy(&pbfrozen)).into()
            }
        }
        unsafe extern "system" fn GetLOHObjectSizeThreshold<Identity: ICorProfilerInfo10_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pthreshold: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo10_Impl::GetLOHObjectSizeThreshold(this, core::mem::transmute_copy(&pthreshold)).into()
            }
        }
        unsafe extern "system" fn RequestReJITWithInliners<Identity: ICorProfilerInfo10_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwrejitflags: u32, cfunctions: u32, moduleids: *const usize, methodids: *const u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo10_Impl::RequestReJITWithInliners(this, core::mem::transmute_copy(&dwrejitflags), core::mem::transmute_copy(&cfunctions), core::mem::transmute_copy(&moduleids), core::mem::transmute_copy(&methodids)).into()
            }
        }
        unsafe extern "system" fn SuspendRuntime<Identity: ICorProfilerInfo10_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo10_Impl::SuspendRuntime(this).into()
            }
        }
        unsafe extern "system" fn ResumeRuntime<Identity: ICorProfilerInfo10_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo10_Impl::ResumeRuntime(this).into()
            }
        }
        Self {
            base__: ICorProfilerInfo9_Vtbl::new::<Identity, OFFSET>(),
            EnumerateObjectReferences: EnumerateObjectReferences::<Identity, OFFSET>,
            IsFrozenObject: IsFrozenObject::<Identity, OFFSET>,
            GetLOHObjectSizeThreshold: GetLOHObjectSizeThreshold::<Identity, OFFSET>,
            RequestReJITWithInliners: RequestReJITWithInliners::<Identity, OFFSET>,
            SuspendRuntime: SuspendRuntime::<Identity, OFFSET>,
            ResumeRuntime: ResumeRuntime::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorProfilerInfo10 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo as windows_core::Interface>::IID || iid == &<ICorProfilerInfo2 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo3 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo4 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo5 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo6 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo7 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo8 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo9 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl windows_core::RuntimeName for ICorProfilerInfo10 {}
windows_core::imp::define_interface!(ICorProfilerInfo11, ICorProfilerInfo11_Vtbl, 0x06398876_8987_4154_b621_40a00d6e4d04);
impl core::ops::Deref for ICorProfilerInfo11 {
    type Target = ICorProfilerInfo10;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICorProfilerInfo11, windows_core::IUnknown, ICorProfilerInfo, ICorProfilerInfo2, ICorProfilerInfo3, ICorProfilerInfo4, ICorProfilerInfo5, ICorProfilerInfo6, ICorProfilerInfo7, ICorProfilerInfo8, ICorProfilerInfo9, ICorProfilerInfo10);
impl ICorProfilerInfo11 {
    pub unsafe fn GetEnvironmentVariableA<P0>(&self, szname: P0, pcchvalue: *mut u32, szvalue: &mut [u16]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetEnvironmentVariableA)(windows_core::Interface::as_raw(self), szname.param().abi(), szvalue.len().try_into().unwrap(), pcchvalue as _, core::mem::transmute(szvalue.as_ptr())).ok() }
    }
    pub unsafe fn SetEnvironmentVariable<P0, P1>(&self, szname: P0, szvalue: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetEnvironmentVariable)(windows_core::Interface::as_raw(self), szname.param().abi(), szvalue.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorProfilerInfo11_Vtbl {
    pub base__: ICorProfilerInfo10_Vtbl,
    pub GetEnvironmentVariableA: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut u32, windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetEnvironmentVariable: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
pub trait ICorProfilerInfo11_Impl: ICorProfilerInfo10_Impl {
    fn GetEnvironmentVariableA(&self, szname: &windows_core::PCWSTR, cchvalue: u32, pcchvalue: *mut u32, szvalue: windows_core::PWSTR) -> windows_core::Result<()>;
    fn SetEnvironmentVariable(&self, szname: &windows_core::PCWSTR, szvalue: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl ICorProfilerInfo11_Vtbl {
    pub const fn new<Identity: ICorProfilerInfo11_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetEnvironmentVariableA<Identity: ICorProfilerInfo11_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szname: windows_core::PCWSTR, cchvalue: u32, pcchvalue: *mut u32, szvalue: windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo11_Impl::GetEnvironmentVariableA(this, core::mem::transmute(&szname), core::mem::transmute_copy(&cchvalue), core::mem::transmute_copy(&pcchvalue), core::mem::transmute_copy(&szvalue)).into()
            }
        }
        unsafe extern "system" fn SetEnvironmentVariable<Identity: ICorProfilerInfo11_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szname: windows_core::PCWSTR, szvalue: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo11_Impl::SetEnvironmentVariable(this, core::mem::transmute(&szname), core::mem::transmute(&szvalue)).into()
            }
        }
        Self {
            base__: ICorProfilerInfo10_Vtbl::new::<Identity, OFFSET>(),
            GetEnvironmentVariableA: GetEnvironmentVariableA::<Identity, OFFSET>,
            SetEnvironmentVariable: SetEnvironmentVariable::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorProfilerInfo11 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo as windows_core::Interface>::IID || iid == &<ICorProfilerInfo2 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo3 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo4 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo5 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo6 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo7 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo8 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo9 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo10 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl windows_core::RuntimeName for ICorProfilerInfo11 {}
windows_core::imp::define_interface!(ICorProfilerInfo12, ICorProfilerInfo12_Vtbl, 0x27b24ccd_1cb1_47c5_96ee_98190dc30959);
impl core::ops::Deref for ICorProfilerInfo12 {
    type Target = ICorProfilerInfo11;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICorProfilerInfo12, windows_core::IUnknown, ICorProfilerInfo, ICorProfilerInfo2, ICorProfilerInfo3, ICorProfilerInfo4, ICorProfilerInfo5, ICorProfilerInfo6, ICorProfilerInfo7, ICorProfilerInfo8, ICorProfilerInfo9, ICorProfilerInfo10, ICorProfilerInfo11);
impl ICorProfilerInfo12 {
    pub unsafe fn EventPipeStartSession(&self, pproviderconfigs: &[COR_PRF_EVENTPIPE_PROVIDER_CONFIG], requestrundown: bool) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EventPipeStartSession)(windows_core::Interface::as_raw(self), pproviderconfigs.len().try_into().unwrap(), core::mem::transmute(pproviderconfigs.as_ptr()), requestrundown.into(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EventPipeAddProviderToSession(&self, session: u64, providerconfig: COR_PRF_EVENTPIPE_PROVIDER_CONFIG) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EventPipeAddProviderToSession)(windows_core::Interface::as_raw(self), session, core::mem::transmute(providerconfig)).ok() }
    }
    pub unsafe fn EventPipeStopSession(&self, session: u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EventPipeStopSession)(windows_core::Interface::as_raw(self), session).ok() }
    }
    pub unsafe fn EventPipeCreateProvider<P0>(&self, providername: P0) -> windows_core::Result<usize>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EventPipeCreateProvider)(windows_core::Interface::as_raw(self), providername.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EventPipeGetProviderInfo(&self, provider: usize, pcchname: *mut u32, providername: &mut [u16]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EventPipeGetProviderInfo)(windows_core::Interface::as_raw(self), provider, providername.len().try_into().unwrap(), pcchname as _, core::mem::transmute(providername.as_ptr())).ok() }
    }
    pub unsafe fn EventPipeDefineEvent<P1>(&self, provider: usize, eventname: P1, eventid: u32, keywords: u64, eventversion: u32, level: u32, opcode: u8, needstack: bool, pparamdescs: &[COR_PRF_EVENTPIPE_PARAM_DESC]) -> windows_core::Result<usize>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EventPipeDefineEvent)(windows_core::Interface::as_raw(self), provider, eventname.param().abi(), eventid, keywords, eventversion, level, opcode, needstack.into(), pparamdescs.len().try_into().unwrap(), core::mem::transmute(pparamdescs.as_ptr()), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EventPipeWriteEvent(&self, event: usize, data: &[COR_PRF_EVENT_DATA], pactivityid: *const windows_core::GUID, prelatedactivityid: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EventPipeWriteEvent)(windows_core::Interface::as_raw(self), event, data.len().try_into().unwrap(), core::mem::transmute(data.as_ptr()), pactivityid, prelatedactivityid).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorProfilerInfo12_Vtbl {
    pub base__: ICorProfilerInfo11_Vtbl,
    pub EventPipeStartSession: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const COR_PRF_EVENTPIPE_PROVIDER_CONFIG, windows_core::BOOL, *mut u64) -> windows_core::HRESULT,
    pub EventPipeAddProviderToSession: unsafe extern "system" fn(*mut core::ffi::c_void, u64, COR_PRF_EVENTPIPE_PROVIDER_CONFIG) -> windows_core::HRESULT,
    pub EventPipeStopSession: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    pub EventPipeCreateProvider: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut usize) -> windows_core::HRESULT,
    pub EventPipeGetProviderInfo: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32, *mut u32, windows_core::PWSTR) -> windows_core::HRESULT,
    pub EventPipeDefineEvent: unsafe extern "system" fn(*mut core::ffi::c_void, usize, windows_core::PCWSTR, u32, u64, u32, u32, u8, windows_core::BOOL, u32, *const COR_PRF_EVENTPIPE_PARAM_DESC, *mut usize) -> windows_core::HRESULT,
    pub EventPipeWriteEvent: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32, *const COR_PRF_EVENT_DATA, *const windows_core::GUID, *const windows_core::GUID) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
pub trait ICorProfilerInfo12_Impl: ICorProfilerInfo11_Impl {
    fn EventPipeStartSession(&self, cproviderconfigs: u32, pproviderconfigs: *const COR_PRF_EVENTPIPE_PROVIDER_CONFIG, requestrundown: windows_core::BOOL) -> windows_core::Result<u64>;
    fn EventPipeAddProviderToSession(&self, session: u64, providerconfig: &COR_PRF_EVENTPIPE_PROVIDER_CONFIG) -> windows_core::Result<()>;
    fn EventPipeStopSession(&self, session: u64) -> windows_core::Result<()>;
    fn EventPipeCreateProvider(&self, providername: &windows_core::PCWSTR) -> windows_core::Result<usize>;
    fn EventPipeGetProviderInfo(&self, provider: usize, cchname: u32, pcchname: *mut u32, providername: windows_core::PWSTR) -> windows_core::Result<()>;
    fn EventPipeDefineEvent(&self, provider: usize, eventname: &windows_core::PCWSTR, eventid: u32, keywords: u64, eventversion: u32, level: u32, opcode: u8, needstack: windows_core::BOOL, cparamdescs: u32, pparamdescs: *const COR_PRF_EVENTPIPE_PARAM_DESC) -> windows_core::Result<usize>;
    fn EventPipeWriteEvent(&self, event: usize, cdata: u32, data: *const COR_PRF_EVENT_DATA, pactivityid: *const windows_core::GUID, prelatedactivityid: *const windows_core::GUID) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl ICorProfilerInfo12_Vtbl {
    pub const fn new<Identity: ICorProfilerInfo12_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EventPipeStartSession<Identity: ICorProfilerInfo12_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cproviderconfigs: u32, pproviderconfigs: *const COR_PRF_EVENTPIPE_PROVIDER_CONFIG, requestrundown: windows_core::BOOL, psession: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorProfilerInfo12_Impl::EventPipeStartSession(this, core::mem::transmute_copy(&cproviderconfigs), core::mem::transmute_copy(&pproviderconfigs), core::mem::transmute_copy(&requestrundown)) {
                    Ok(ok__) => {
                        psession.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EventPipeAddProviderToSession<Identity: ICorProfilerInfo12_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, session: u64, providerconfig: COR_PRF_EVENTPIPE_PROVIDER_CONFIG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo12_Impl::EventPipeAddProviderToSession(this, core::mem::transmute_copy(&session), core::mem::transmute(&providerconfig)).into()
            }
        }
        unsafe extern "system" fn EventPipeStopSession<Identity: ICorProfilerInfo12_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, session: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo12_Impl::EventPipeStopSession(this, core::mem::transmute_copy(&session)).into()
            }
        }
        unsafe extern "system" fn EventPipeCreateProvider<Identity: ICorProfilerInfo12_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, providername: windows_core::PCWSTR, pprovider: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorProfilerInfo12_Impl::EventPipeCreateProvider(this, core::mem::transmute(&providername)) {
                    Ok(ok__) => {
                        pprovider.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EventPipeGetProviderInfo<Identity: ICorProfilerInfo12_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, provider: usize, cchname: u32, pcchname: *mut u32, providername: windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo12_Impl::EventPipeGetProviderInfo(this, core::mem::transmute_copy(&provider), core::mem::transmute_copy(&cchname), core::mem::transmute_copy(&pcchname), core::mem::transmute_copy(&providername)).into()
            }
        }
        unsafe extern "system" fn EventPipeDefineEvent<Identity: ICorProfilerInfo12_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, provider: usize, eventname: windows_core::PCWSTR, eventid: u32, keywords: u64, eventversion: u32, level: u32, opcode: u8, needstack: windows_core::BOOL, cparamdescs: u32, pparamdescs: *const COR_PRF_EVENTPIPE_PARAM_DESC, pevent: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorProfilerInfo12_Impl::EventPipeDefineEvent(this, core::mem::transmute_copy(&provider), core::mem::transmute(&eventname), core::mem::transmute_copy(&eventid), core::mem::transmute_copy(&keywords), core::mem::transmute_copy(&eventversion), core::mem::transmute_copy(&level), core::mem::transmute_copy(&opcode), core::mem::transmute_copy(&needstack), core::mem::transmute_copy(&cparamdescs), core::mem::transmute_copy(&pparamdescs)) {
                    Ok(ok__) => {
                        pevent.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EventPipeWriteEvent<Identity: ICorProfilerInfo12_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, event: usize, cdata: u32, data: *const COR_PRF_EVENT_DATA, pactivityid: *const windows_core::GUID, prelatedactivityid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo12_Impl::EventPipeWriteEvent(this, core::mem::transmute_copy(&event), core::mem::transmute_copy(&cdata), core::mem::transmute_copy(&data), core::mem::transmute_copy(&pactivityid), core::mem::transmute_copy(&prelatedactivityid)).into()
            }
        }
        Self {
            base__: ICorProfilerInfo11_Vtbl::new::<Identity, OFFSET>(),
            EventPipeStartSession: EventPipeStartSession::<Identity, OFFSET>,
            EventPipeAddProviderToSession: EventPipeAddProviderToSession::<Identity, OFFSET>,
            EventPipeStopSession: EventPipeStopSession::<Identity, OFFSET>,
            EventPipeCreateProvider: EventPipeCreateProvider::<Identity, OFFSET>,
            EventPipeGetProviderInfo: EventPipeGetProviderInfo::<Identity, OFFSET>,
            EventPipeDefineEvent: EventPipeDefineEvent::<Identity, OFFSET>,
            EventPipeWriteEvent: EventPipeWriteEvent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorProfilerInfo12 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo as windows_core::Interface>::IID || iid == &<ICorProfilerInfo2 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo3 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo4 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo5 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo6 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo7 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo8 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo9 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo10 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo11 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl windows_core::RuntimeName for ICorProfilerInfo12 {}
windows_core::imp::define_interface!(ICorProfilerInfo13, ICorProfilerInfo13_Vtbl, 0x6e6c7ee2_0701_4ec2_9d29_2e8733b66934);
impl core::ops::Deref for ICorProfilerInfo13 {
    type Target = ICorProfilerInfo12;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICorProfilerInfo13, windows_core::IUnknown, ICorProfilerInfo, ICorProfilerInfo2, ICorProfilerInfo3, ICorProfilerInfo4, ICorProfilerInfo5, ICorProfilerInfo6, ICorProfilerInfo7, ICorProfilerInfo8, ICorProfilerInfo9, ICorProfilerInfo10, ICorProfilerInfo11, ICorProfilerInfo12);
impl ICorProfilerInfo13 {
    pub unsafe fn CreateHandle(&self, object: usize, r#type: COR_PRF_HANDLE_TYPE, phandle: *mut *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CreateHandle)(windows_core::Interface::as_raw(self), object, r#type, phandle as _).ok() }
    }
    pub unsafe fn DestroyHandle(&self, handle: *const *const core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DestroyHandle)(windows_core::Interface::as_raw(self), handle).ok() }
    }
    pub unsafe fn GetObjectIDFromHandle(&self, handle: *const *const core::ffi::c_void) -> windows_core::Result<usize> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetObjectIDFromHandle)(windows_core::Interface::as_raw(self), handle, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorProfilerInfo13_Vtbl {
    pub base__: ICorProfilerInfo12_Vtbl,
    pub CreateHandle: unsafe extern "system" fn(*mut core::ffi::c_void, usize, COR_PRF_HANDLE_TYPE, *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DestroyHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *const *const core::ffi::c_void) -> windows_core::HRESULT,
    pub GetObjectIDFromHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *const *const core::ffi::c_void, *mut usize) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
pub trait ICorProfilerInfo13_Impl: ICorProfilerInfo12_Impl {
    fn CreateHandle(&self, object: usize, r#type: COR_PRF_HANDLE_TYPE, phandle: *mut *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn DestroyHandle(&self, handle: *const *const core::ffi::c_void) -> windows_core::Result<()>;
    fn GetObjectIDFromHandle(&self, handle: *const *const core::ffi::c_void) -> windows_core::Result<usize>;
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl ICorProfilerInfo13_Vtbl {
    pub const fn new<Identity: ICorProfilerInfo13_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateHandle<Identity: ICorProfilerInfo13_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: usize, r#type: COR_PRF_HANDLE_TYPE, phandle: *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo13_Impl::CreateHandle(this, core::mem::transmute_copy(&object), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&phandle)).into()
            }
        }
        unsafe extern "system" fn DestroyHandle<Identity: ICorProfilerInfo13_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handle: *const *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo13_Impl::DestroyHandle(this, core::mem::transmute_copy(&handle)).into()
            }
        }
        unsafe extern "system" fn GetObjectIDFromHandle<Identity: ICorProfilerInfo13_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handle: *const *const core::ffi::c_void, pobject: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorProfilerInfo13_Impl::GetObjectIDFromHandle(this, core::mem::transmute_copy(&handle)) {
                    Ok(ok__) => {
                        pobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: ICorProfilerInfo12_Vtbl::new::<Identity, OFFSET>(),
            CreateHandle: CreateHandle::<Identity, OFFSET>,
            DestroyHandle: DestroyHandle::<Identity, OFFSET>,
            GetObjectIDFromHandle: GetObjectIDFromHandle::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorProfilerInfo13 as windows_core::Interface>::IID
            || iid == &<ICorProfilerInfo as windows_core::Interface>::IID
            || iid == &<ICorProfilerInfo2 as windows_core::Interface>::IID
            || iid == &<ICorProfilerInfo3 as windows_core::Interface>::IID
            || iid == &<ICorProfilerInfo4 as windows_core::Interface>::IID
            || iid == &<ICorProfilerInfo5 as windows_core::Interface>::IID
            || iid == &<ICorProfilerInfo6 as windows_core::Interface>::IID
            || iid == &<ICorProfilerInfo7 as windows_core::Interface>::IID
            || iid == &<ICorProfilerInfo8 as windows_core::Interface>::IID
            || iid == &<ICorProfilerInfo9 as windows_core::Interface>::IID
            || iid == &<ICorProfilerInfo10 as windows_core::Interface>::IID
            || iid == &<ICorProfilerInfo11 as windows_core::Interface>::IID
            || iid == &<ICorProfilerInfo12 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl windows_core::RuntimeName for ICorProfilerInfo13 {}
windows_core::imp::define_interface!(ICorProfilerInfo14, ICorProfilerInfo14_Vtbl, 0xf460e352_d76d_4fe9_835f_f6af9d6e862d);
impl core::ops::Deref for ICorProfilerInfo14 {
    type Target = ICorProfilerInfo13;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICorProfilerInfo14, windows_core::IUnknown, ICorProfilerInfo, ICorProfilerInfo2, ICorProfilerInfo3, ICorProfilerInfo4, ICorProfilerInfo5, ICorProfilerInfo6, ICorProfilerInfo7, ICorProfilerInfo8, ICorProfilerInfo9, ICorProfilerInfo10, ICorProfilerInfo11, ICorProfilerInfo12, ICorProfilerInfo13);
impl ICorProfilerInfo14 {
    pub unsafe fn EnumerateNonGCObjects(&self) -> windows_core::Result<ICorProfilerObjectEnum> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumerateNonGCObjects)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetNonGCHeapBounds(&self, pcobjectranges: *mut u32, ranges: &mut [COR_PRF_NONGC_HEAP_RANGE]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetNonGCHeapBounds)(windows_core::Interface::as_raw(self), ranges.len().try_into().unwrap(), pcobjectranges as _, core::mem::transmute(ranges.as_ptr())).ok() }
    }
    pub unsafe fn EventPipeCreateProvider2<P0>(&self, providername: P0, pcallback: *const EventPipeProviderCallback) -> windows_core::Result<usize>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EventPipeCreateProvider2)(windows_core::Interface::as_raw(self), providername.param().abi(), pcallback, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorProfilerInfo14_Vtbl {
    pub base__: ICorProfilerInfo13_Vtbl,
    pub EnumerateNonGCObjects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNonGCHeapBounds: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *mut COR_PRF_NONGC_HEAP_RANGE) -> windows_core::HRESULT,
    pub EventPipeCreateProvider2: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const EventPipeProviderCallback, *mut usize) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
pub trait ICorProfilerInfo14_Impl: ICorProfilerInfo13_Impl {
    fn EnumerateNonGCObjects(&self) -> windows_core::Result<ICorProfilerObjectEnum>;
    fn GetNonGCHeapBounds(&self, cobjectranges: u32, pcobjectranges: *mut u32, ranges: *mut COR_PRF_NONGC_HEAP_RANGE) -> windows_core::Result<()>;
    fn EventPipeCreateProvider2(&self, providername: &windows_core::PCWSTR, pcallback: *const EventPipeProviderCallback) -> windows_core::Result<usize>;
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl ICorProfilerInfo14_Vtbl {
    pub const fn new<Identity: ICorProfilerInfo14_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumerateNonGCObjects<Identity: ICorProfilerInfo14_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorProfilerInfo14_Impl::EnumerateNonGCObjects(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetNonGCHeapBounds<Identity: ICorProfilerInfo14_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cobjectranges: u32, pcobjectranges: *mut u32, ranges: *mut COR_PRF_NONGC_HEAP_RANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo14_Impl::GetNonGCHeapBounds(this, core::mem::transmute_copy(&cobjectranges), core::mem::transmute_copy(&pcobjectranges), core::mem::transmute_copy(&ranges)).into()
            }
        }
        unsafe extern "system" fn EventPipeCreateProvider2<Identity: ICorProfilerInfo14_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, providername: windows_core::PCWSTR, pcallback: *const EventPipeProviderCallback, pprovider: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorProfilerInfo14_Impl::EventPipeCreateProvider2(this, core::mem::transmute(&providername), core::mem::transmute_copy(&pcallback)) {
                    Ok(ok__) => {
                        pprovider.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: ICorProfilerInfo13_Vtbl::new::<Identity, OFFSET>(),
            EnumerateNonGCObjects: EnumerateNonGCObjects::<Identity, OFFSET>,
            GetNonGCHeapBounds: GetNonGCHeapBounds::<Identity, OFFSET>,
            EventPipeCreateProvider2: EventPipeCreateProvider2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorProfilerInfo14 as windows_core::Interface>::IID
            || iid == &<ICorProfilerInfo as windows_core::Interface>::IID
            || iid == &<ICorProfilerInfo2 as windows_core::Interface>::IID
            || iid == &<ICorProfilerInfo3 as windows_core::Interface>::IID
            || iid == &<ICorProfilerInfo4 as windows_core::Interface>::IID
            || iid == &<ICorProfilerInfo5 as windows_core::Interface>::IID
            || iid == &<ICorProfilerInfo6 as windows_core::Interface>::IID
            || iid == &<ICorProfilerInfo7 as windows_core::Interface>::IID
            || iid == &<ICorProfilerInfo8 as windows_core::Interface>::IID
            || iid == &<ICorProfilerInfo9 as windows_core::Interface>::IID
            || iid == &<ICorProfilerInfo10 as windows_core::Interface>::IID
            || iid == &<ICorProfilerInfo11 as windows_core::Interface>::IID
            || iid == &<ICorProfilerInfo12 as windows_core::Interface>::IID
            || iid == &<ICorProfilerInfo13 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl windows_core::RuntimeName for ICorProfilerInfo14 {}
windows_core::imp::define_interface!(ICorProfilerInfo2, ICorProfilerInfo2_Vtbl, 0xcc0935cd_a518_487d_b0bb_a93214e65478);
impl core::ops::Deref for ICorProfilerInfo2 {
    type Target = ICorProfilerInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICorProfilerInfo2, windows_core::IUnknown, ICorProfilerInfo);
impl ICorProfilerInfo2 {
    pub unsafe fn DoStackSnapshot(&self, thread: usize, callback: *const StackSnapshotCallback, infoflags: u32, clientdata: *const core::ffi::c_void, context: &[u8]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DoStackSnapshot)(windows_core::Interface::as_raw(self), thread, callback, infoflags, clientdata, core::mem::transmute(context.as_ptr()), context.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn SetEnterLeaveFunctionHooks2(&self, pfuncenter: *const FunctionEnter2, pfuncleave: *const FunctionLeave2, pfunctailcall: *const FunctionTailcall2) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEnterLeaveFunctionHooks2)(windows_core::Interface::as_raw(self), pfuncenter, pfuncleave, pfunctailcall).ok() }
    }
    pub unsafe fn GetFunctionInfo2(&self, funcid: usize, frameinfo: usize, pclassid: *mut usize, pmoduleid: *mut usize, ptoken: *mut u32, ctypeargs: u32, pctypeargs: *mut u32, typeargs: *mut usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetFunctionInfo2)(windows_core::Interface::as_raw(self), funcid, frameinfo, pclassid as _, pmoduleid as _, ptoken as _, ctypeargs, pctypeargs as _, typeargs as _).ok() }
    }
    pub unsafe fn GetStringLayout(&self, pbufferlengthoffset: *mut u32, pstringlengthoffset: *mut u32, pbufferoffset: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetStringLayout)(windows_core::Interface::as_raw(self), pbufferlengthoffset as _, pstringlengthoffset as _, pbufferoffset as _).ok() }
    }
    #[cfg(feature = "Win32_System_WinRT_Metadata")]
    pub unsafe fn GetClassLayout(&self, classid: usize, rfieldoffset: *mut super::super::WinRT::Metadata::COR_FIELD_OFFSET, cfieldoffset: u32, pcfieldoffset: *mut u32, pulclasssize: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetClassLayout)(windows_core::Interface::as_raw(self), classid, rfieldoffset as _, cfieldoffset, pcfieldoffset as _, pulclasssize as _).ok() }
    }
    pub unsafe fn GetClassIDInfo2(&self, classid: usize, pmoduleid: *mut usize, ptypedeftoken: *mut u32, pparentclassid: *mut usize, cnumtypeargs: u32, pcnumtypeargs: *mut u32, typeargs: *mut usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetClassIDInfo2)(windows_core::Interface::as_raw(self), classid, pmoduleid as _, ptypedeftoken as _, pparentclassid as _, cnumtypeargs, pcnumtypeargs as _, typeargs as _).ok() }
    }
    pub unsafe fn GetCodeInfo2(&self, functionid: usize, pccodeinfos: *mut u32, codeinfos: &mut [COR_PRF_CODE_INFO]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCodeInfo2)(windows_core::Interface::as_raw(self), functionid, codeinfos.len().try_into().unwrap(), pccodeinfos as _, core::mem::transmute(codeinfos.as_ptr())).ok() }
    }
    pub unsafe fn GetClassFromTokenAndTypeArgs(&self, moduleid: usize, typedef: u32, typeargs: &[usize]) -> windows_core::Result<usize> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetClassFromTokenAndTypeArgs)(windows_core::Interface::as_raw(self), moduleid, typedef, typeargs.len().try_into().unwrap(), core::mem::transmute(typeargs.as_ptr()), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetFunctionFromTokenAndTypeArgs(&self, moduleid: usize, funcdef: u32, classid: usize, typeargs: &[usize]) -> windows_core::Result<usize> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFunctionFromTokenAndTypeArgs)(windows_core::Interface::as_raw(self), moduleid, funcdef, classid, typeargs.len().try_into().unwrap(), core::mem::transmute(typeargs.as_ptr()), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EnumModuleFrozenObjects(&self, moduleid: usize) -> windows_core::Result<ICorProfilerObjectEnum> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumModuleFrozenObjects)(windows_core::Interface::as_raw(self), moduleid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetArrayObjectInfo(&self, objectid: usize, cdimensions: u32, pdimensionsizes: *mut u32, pdimensionlowerbounds: *mut i32, ppdata: *mut *mut u8) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetArrayObjectInfo)(windows_core::Interface::as_raw(self), objectid, cdimensions, pdimensionsizes as _, pdimensionlowerbounds as _, ppdata as _).ok() }
    }
    pub unsafe fn GetBoxClassLayout(&self, classid: usize) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBoxClassLayout)(windows_core::Interface::as_raw(self), classid, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetThreadAppDomain(&self, threadid: usize) -> windows_core::Result<usize> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetThreadAppDomain)(windows_core::Interface::as_raw(self), threadid, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetRVAStaticAddress(&self, classid: usize, fieldtoken: u32, ppaddress: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetRVAStaticAddress)(windows_core::Interface::as_raw(self), classid, fieldtoken, ppaddress as _).ok() }
    }
    pub unsafe fn GetAppDomainStaticAddress(&self, classid: usize, fieldtoken: u32, appdomainid: usize, ppaddress: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetAppDomainStaticAddress)(windows_core::Interface::as_raw(self), classid, fieldtoken, appdomainid, ppaddress as _).ok() }
    }
    pub unsafe fn GetThreadStaticAddress(&self, classid: usize, fieldtoken: u32, threadid: usize, ppaddress: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetThreadStaticAddress)(windows_core::Interface::as_raw(self), classid, fieldtoken, threadid, ppaddress as _).ok() }
    }
    pub unsafe fn GetContextStaticAddress(&self, classid: usize, fieldtoken: u32, contextid: usize, ppaddress: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetContextStaticAddress)(windows_core::Interface::as_raw(self), classid, fieldtoken, contextid, ppaddress as _).ok() }
    }
    pub unsafe fn GetStaticFieldInfo(&self, classid: usize, fieldtoken: u32) -> windows_core::Result<COR_PRF_STATIC_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStaticFieldInfo)(windows_core::Interface::as_raw(self), classid, fieldtoken, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetGenerationBounds(&self, pcobjectranges: *mut u32, ranges: &mut [COR_PRF_GC_GENERATION_RANGE]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetGenerationBounds)(windows_core::Interface::as_raw(self), ranges.len().try_into().unwrap(), pcobjectranges as _, core::mem::transmute(ranges.as_ptr())).ok() }
    }
    pub unsafe fn GetObjectGeneration(&self, objectid: usize) -> windows_core::Result<COR_PRF_GC_GENERATION_RANGE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetObjectGeneration)(windows_core::Interface::as_raw(self), objectid, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetNotifiedExceptionClauseInfo(&self) -> windows_core::Result<COR_PRF_EX_CLAUSE_INFO> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNotifiedExceptionClauseInfo)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorProfilerInfo2_Vtbl {
    pub base__: ICorProfilerInfo_Vtbl,
    pub DoStackSnapshot: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *const StackSnapshotCallback, u32, *const core::ffi::c_void, *const u8, u32) -> windows_core::HRESULT,
    pub SetEnterLeaveFunctionHooks2: unsafe extern "system" fn(*mut core::ffi::c_void, *const FunctionEnter2, *const FunctionLeave2, *const FunctionTailcall2) -> windows_core::HRESULT,
    pub GetFunctionInfo2: unsafe extern "system" fn(*mut core::ffi::c_void, usize, usize, *mut usize, *mut usize, *mut u32, u32, *mut u32, *mut usize) -> windows_core::HRESULT,
    pub GetStringLayout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_WinRT_Metadata")]
    pub GetClassLayout: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut super::super::WinRT::Metadata::COR_FIELD_OFFSET, u32, *mut u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_WinRT_Metadata"))]
    GetClassLayout: usize,
    pub GetClassIDInfo2: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut usize, *mut u32, *mut usize, u32, *mut u32, *mut usize) -> windows_core::HRESULT,
    pub GetCodeInfo2: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32, *mut u32, *mut COR_PRF_CODE_INFO) -> windows_core::HRESULT,
    pub GetClassFromTokenAndTypeArgs: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32, u32, *const usize, *mut usize) -> windows_core::HRESULT,
    pub GetFunctionFromTokenAndTypeArgs: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32, usize, u32, *const usize, *mut usize) -> windows_core::HRESULT,
    pub EnumModuleFrozenObjects: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetArrayObjectInfo: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32, *mut u32, *mut i32, *mut *mut u8) -> windows_core::HRESULT,
    pub GetBoxClassLayout: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut u32) -> windows_core::HRESULT,
    pub GetThreadAppDomain: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut usize) -> windows_core::HRESULT,
    pub GetRVAStaticAddress: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAppDomainStaticAddress: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32, usize, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetThreadStaticAddress: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32, usize, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetContextStaticAddress: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32, usize, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetStaticFieldInfo: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32, *mut COR_PRF_STATIC_TYPE) -> windows_core::HRESULT,
    pub GetGenerationBounds: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *mut COR_PRF_GC_GENERATION_RANGE) -> windows_core::HRESULT,
    pub GetObjectGeneration: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut COR_PRF_GC_GENERATION_RANGE) -> windows_core::HRESULT,
    pub GetNotifiedExceptionClauseInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut COR_PRF_EX_CLAUSE_INFO) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
pub trait ICorProfilerInfo2_Impl: ICorProfilerInfo_Impl {
    fn DoStackSnapshot(&self, thread: usize, callback: *const StackSnapshotCallback, infoflags: u32, clientdata: *const core::ffi::c_void, context: *const u8, contextsize: u32) -> windows_core::Result<()>;
    fn SetEnterLeaveFunctionHooks2(&self, pfuncenter: *const FunctionEnter2, pfuncleave: *const FunctionLeave2, pfunctailcall: *const FunctionTailcall2) -> windows_core::Result<()>;
    fn GetFunctionInfo2(&self, funcid: usize, frameinfo: usize, pclassid: *mut usize, pmoduleid: *mut usize, ptoken: *mut u32, ctypeargs: u32, pctypeargs: *mut u32, typeargs: *mut usize) -> windows_core::Result<()>;
    fn GetStringLayout(&self, pbufferlengthoffset: *mut u32, pstringlengthoffset: *mut u32, pbufferoffset: *mut u32) -> windows_core::Result<()>;
    fn GetClassLayout(&self, classid: usize, rfieldoffset: *mut super::super::WinRT::Metadata::COR_FIELD_OFFSET, cfieldoffset: u32, pcfieldoffset: *mut u32, pulclasssize: *mut u32) -> windows_core::Result<()>;
    fn GetClassIDInfo2(&self, classid: usize, pmoduleid: *mut usize, ptypedeftoken: *mut u32, pparentclassid: *mut usize, cnumtypeargs: u32, pcnumtypeargs: *mut u32, typeargs: *mut usize) -> windows_core::Result<()>;
    fn GetCodeInfo2(&self, functionid: usize, ccodeinfos: u32, pccodeinfos: *mut u32, codeinfos: *mut COR_PRF_CODE_INFO) -> windows_core::Result<()>;
    fn GetClassFromTokenAndTypeArgs(&self, moduleid: usize, typedef: u32, ctypeargs: u32, typeargs: *const usize) -> windows_core::Result<usize>;
    fn GetFunctionFromTokenAndTypeArgs(&self, moduleid: usize, funcdef: u32, classid: usize, ctypeargs: u32, typeargs: *const usize) -> windows_core::Result<usize>;
    fn EnumModuleFrozenObjects(&self, moduleid: usize) -> windows_core::Result<ICorProfilerObjectEnum>;
    fn GetArrayObjectInfo(&self, objectid: usize, cdimensions: u32, pdimensionsizes: *mut u32, pdimensionlowerbounds: *mut i32, ppdata: *mut *mut u8) -> windows_core::Result<()>;
    fn GetBoxClassLayout(&self, classid: usize) -> windows_core::Result<u32>;
    fn GetThreadAppDomain(&self, threadid: usize) -> windows_core::Result<usize>;
    fn GetRVAStaticAddress(&self, classid: usize, fieldtoken: u32, ppaddress: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetAppDomainStaticAddress(&self, classid: usize, fieldtoken: u32, appdomainid: usize, ppaddress: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetThreadStaticAddress(&self, classid: usize, fieldtoken: u32, threadid: usize, ppaddress: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetContextStaticAddress(&self, classid: usize, fieldtoken: u32, contextid: usize, ppaddress: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetStaticFieldInfo(&self, classid: usize, fieldtoken: u32) -> windows_core::Result<COR_PRF_STATIC_TYPE>;
    fn GetGenerationBounds(&self, cobjectranges: u32, pcobjectranges: *mut u32, ranges: *mut COR_PRF_GC_GENERATION_RANGE) -> windows_core::Result<()>;
    fn GetObjectGeneration(&self, objectid: usize) -> windows_core::Result<COR_PRF_GC_GENERATION_RANGE>;
    fn GetNotifiedExceptionClauseInfo(&self) -> windows_core::Result<COR_PRF_EX_CLAUSE_INFO>;
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl ICorProfilerInfo2_Vtbl {
    pub const fn new<Identity: ICorProfilerInfo2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DoStackSnapshot<Identity: ICorProfilerInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, thread: usize, callback: *const StackSnapshotCallback, infoflags: u32, clientdata: *const core::ffi::c_void, context: *const u8, contextsize: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo2_Impl::DoStackSnapshot(this, core::mem::transmute_copy(&thread), core::mem::transmute_copy(&callback), core::mem::transmute_copy(&infoflags), core::mem::transmute_copy(&clientdata), core::mem::transmute_copy(&context), core::mem::transmute_copy(&contextsize)).into()
            }
        }
        unsafe extern "system" fn SetEnterLeaveFunctionHooks2<Identity: ICorProfilerInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfuncenter: *const FunctionEnter2, pfuncleave: *const FunctionLeave2, pfunctailcall: *const FunctionTailcall2) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo2_Impl::SetEnterLeaveFunctionHooks2(this, core::mem::transmute_copy(&pfuncenter), core::mem::transmute_copy(&pfuncleave), core::mem::transmute_copy(&pfunctailcall)).into()
            }
        }
        unsafe extern "system" fn GetFunctionInfo2<Identity: ICorProfilerInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, funcid: usize, frameinfo: usize, pclassid: *mut usize, pmoduleid: *mut usize, ptoken: *mut u32, ctypeargs: u32, pctypeargs: *mut u32, typeargs: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo2_Impl::GetFunctionInfo2(this, core::mem::transmute_copy(&funcid), core::mem::transmute_copy(&frameinfo), core::mem::transmute_copy(&pclassid), core::mem::transmute_copy(&pmoduleid), core::mem::transmute_copy(&ptoken), core::mem::transmute_copy(&ctypeargs), core::mem::transmute_copy(&pctypeargs), core::mem::transmute_copy(&typeargs)).into()
            }
        }
        unsafe extern "system" fn GetStringLayout<Identity: ICorProfilerInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbufferlengthoffset: *mut u32, pstringlengthoffset: *mut u32, pbufferoffset: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo2_Impl::GetStringLayout(this, core::mem::transmute_copy(&pbufferlengthoffset), core::mem::transmute_copy(&pstringlengthoffset), core::mem::transmute_copy(&pbufferoffset)).into()
            }
        }
        unsafe extern "system" fn GetClassLayout<Identity: ICorProfilerInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, classid: usize, rfieldoffset: *mut super::super::WinRT::Metadata::COR_FIELD_OFFSET, cfieldoffset: u32, pcfieldoffset: *mut u32, pulclasssize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo2_Impl::GetClassLayout(this, core::mem::transmute_copy(&classid), core::mem::transmute_copy(&rfieldoffset), core::mem::transmute_copy(&cfieldoffset), core::mem::transmute_copy(&pcfieldoffset), core::mem::transmute_copy(&pulclasssize)).into()
            }
        }
        unsafe extern "system" fn GetClassIDInfo2<Identity: ICorProfilerInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, classid: usize, pmoduleid: *mut usize, ptypedeftoken: *mut u32, pparentclassid: *mut usize, cnumtypeargs: u32, pcnumtypeargs: *mut u32, typeargs: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo2_Impl::GetClassIDInfo2(this, core::mem::transmute_copy(&classid), core::mem::transmute_copy(&pmoduleid), core::mem::transmute_copy(&ptypedeftoken), core::mem::transmute_copy(&pparentclassid), core::mem::transmute_copy(&cnumtypeargs), core::mem::transmute_copy(&pcnumtypeargs), core::mem::transmute_copy(&typeargs)).into()
            }
        }
        unsafe extern "system" fn GetCodeInfo2<Identity: ICorProfilerInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize, ccodeinfos: u32, pccodeinfos: *mut u32, codeinfos: *mut COR_PRF_CODE_INFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo2_Impl::GetCodeInfo2(this, core::mem::transmute_copy(&functionid), core::mem::transmute_copy(&ccodeinfos), core::mem::transmute_copy(&pccodeinfos), core::mem::transmute_copy(&codeinfos)).into()
            }
        }
        unsafe extern "system" fn GetClassFromTokenAndTypeArgs<Identity: ICorProfilerInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduleid: usize, typedef: u32, ctypeargs: u32, typeargs: *const usize, pclassid: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorProfilerInfo2_Impl::GetClassFromTokenAndTypeArgs(this, core::mem::transmute_copy(&moduleid), core::mem::transmute_copy(&typedef), core::mem::transmute_copy(&ctypeargs), core::mem::transmute_copy(&typeargs)) {
                    Ok(ok__) => {
                        pclassid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFunctionFromTokenAndTypeArgs<Identity: ICorProfilerInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduleid: usize, funcdef: u32, classid: usize, ctypeargs: u32, typeargs: *const usize, pfunctionid: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorProfilerInfo2_Impl::GetFunctionFromTokenAndTypeArgs(this, core::mem::transmute_copy(&moduleid), core::mem::transmute_copy(&funcdef), core::mem::transmute_copy(&classid), core::mem::transmute_copy(&ctypeargs), core::mem::transmute_copy(&typeargs)) {
                    Ok(ok__) => {
                        pfunctionid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumModuleFrozenObjects<Identity: ICorProfilerInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduleid: usize, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorProfilerInfo2_Impl::EnumModuleFrozenObjects(this, core::mem::transmute_copy(&moduleid)) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetArrayObjectInfo<Identity: ICorProfilerInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objectid: usize, cdimensions: u32, pdimensionsizes: *mut u32, pdimensionlowerbounds: *mut i32, ppdata: *mut *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo2_Impl::GetArrayObjectInfo(this, core::mem::transmute_copy(&objectid), core::mem::transmute_copy(&cdimensions), core::mem::transmute_copy(&pdimensionsizes), core::mem::transmute_copy(&pdimensionlowerbounds), core::mem::transmute_copy(&ppdata)).into()
            }
        }
        unsafe extern "system" fn GetBoxClassLayout<Identity: ICorProfilerInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, classid: usize, pbufferoffset: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorProfilerInfo2_Impl::GetBoxClassLayout(this, core::mem::transmute_copy(&classid)) {
                    Ok(ok__) => {
                        pbufferoffset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetThreadAppDomain<Identity: ICorProfilerInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, threadid: usize, pappdomainid: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorProfilerInfo2_Impl::GetThreadAppDomain(this, core::mem::transmute_copy(&threadid)) {
                    Ok(ok__) => {
                        pappdomainid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRVAStaticAddress<Identity: ICorProfilerInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, classid: usize, fieldtoken: u32, ppaddress: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo2_Impl::GetRVAStaticAddress(this, core::mem::transmute_copy(&classid), core::mem::transmute_copy(&fieldtoken), core::mem::transmute_copy(&ppaddress)).into()
            }
        }
        unsafe extern "system" fn GetAppDomainStaticAddress<Identity: ICorProfilerInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, classid: usize, fieldtoken: u32, appdomainid: usize, ppaddress: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo2_Impl::GetAppDomainStaticAddress(this, core::mem::transmute_copy(&classid), core::mem::transmute_copy(&fieldtoken), core::mem::transmute_copy(&appdomainid), core::mem::transmute_copy(&ppaddress)).into()
            }
        }
        unsafe extern "system" fn GetThreadStaticAddress<Identity: ICorProfilerInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, classid: usize, fieldtoken: u32, threadid: usize, ppaddress: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo2_Impl::GetThreadStaticAddress(this, core::mem::transmute_copy(&classid), core::mem::transmute_copy(&fieldtoken), core::mem::transmute_copy(&threadid), core::mem::transmute_copy(&ppaddress)).into()
            }
        }
        unsafe extern "system" fn GetContextStaticAddress<Identity: ICorProfilerInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, classid: usize, fieldtoken: u32, contextid: usize, ppaddress: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo2_Impl::GetContextStaticAddress(this, core::mem::transmute_copy(&classid), core::mem::transmute_copy(&fieldtoken), core::mem::transmute_copy(&contextid), core::mem::transmute_copy(&ppaddress)).into()
            }
        }
        unsafe extern "system" fn GetStaticFieldInfo<Identity: ICorProfilerInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, classid: usize, fieldtoken: u32, pfieldinfo: *mut COR_PRF_STATIC_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorProfilerInfo2_Impl::GetStaticFieldInfo(this, core::mem::transmute_copy(&classid), core::mem::transmute_copy(&fieldtoken)) {
                    Ok(ok__) => {
                        pfieldinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetGenerationBounds<Identity: ICorProfilerInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cobjectranges: u32, pcobjectranges: *mut u32, ranges: *mut COR_PRF_GC_GENERATION_RANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo2_Impl::GetGenerationBounds(this, core::mem::transmute_copy(&cobjectranges), core::mem::transmute_copy(&pcobjectranges), core::mem::transmute_copy(&ranges)).into()
            }
        }
        unsafe extern "system" fn GetObjectGeneration<Identity: ICorProfilerInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objectid: usize, range: *mut COR_PRF_GC_GENERATION_RANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorProfilerInfo2_Impl::GetObjectGeneration(this, core::mem::transmute_copy(&objectid)) {
                    Ok(ok__) => {
                        range.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetNotifiedExceptionClauseInfo<Identity: ICorProfilerInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *mut COR_PRF_EX_CLAUSE_INFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorProfilerInfo2_Impl::GetNotifiedExceptionClauseInfo(this) {
                    Ok(ok__) => {
                        pinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: ICorProfilerInfo_Vtbl::new::<Identity, OFFSET>(),
            DoStackSnapshot: DoStackSnapshot::<Identity, OFFSET>,
            SetEnterLeaveFunctionHooks2: SetEnterLeaveFunctionHooks2::<Identity, OFFSET>,
            GetFunctionInfo2: GetFunctionInfo2::<Identity, OFFSET>,
            GetStringLayout: GetStringLayout::<Identity, OFFSET>,
            GetClassLayout: GetClassLayout::<Identity, OFFSET>,
            GetClassIDInfo2: GetClassIDInfo2::<Identity, OFFSET>,
            GetCodeInfo2: GetCodeInfo2::<Identity, OFFSET>,
            GetClassFromTokenAndTypeArgs: GetClassFromTokenAndTypeArgs::<Identity, OFFSET>,
            GetFunctionFromTokenAndTypeArgs: GetFunctionFromTokenAndTypeArgs::<Identity, OFFSET>,
            EnumModuleFrozenObjects: EnumModuleFrozenObjects::<Identity, OFFSET>,
            GetArrayObjectInfo: GetArrayObjectInfo::<Identity, OFFSET>,
            GetBoxClassLayout: GetBoxClassLayout::<Identity, OFFSET>,
            GetThreadAppDomain: GetThreadAppDomain::<Identity, OFFSET>,
            GetRVAStaticAddress: GetRVAStaticAddress::<Identity, OFFSET>,
            GetAppDomainStaticAddress: GetAppDomainStaticAddress::<Identity, OFFSET>,
            GetThreadStaticAddress: GetThreadStaticAddress::<Identity, OFFSET>,
            GetContextStaticAddress: GetContextStaticAddress::<Identity, OFFSET>,
            GetStaticFieldInfo: GetStaticFieldInfo::<Identity, OFFSET>,
            GetGenerationBounds: GetGenerationBounds::<Identity, OFFSET>,
            GetObjectGeneration: GetObjectGeneration::<Identity, OFFSET>,
            GetNotifiedExceptionClauseInfo: GetNotifiedExceptionClauseInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorProfilerInfo2 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl windows_core::RuntimeName for ICorProfilerInfo2 {}
windows_core::imp::define_interface!(ICorProfilerInfo3, ICorProfilerInfo3_Vtbl, 0xb555ed4f_452a_4e54_8b39_b5360bad32a0);
impl core::ops::Deref for ICorProfilerInfo3 {
    type Target = ICorProfilerInfo2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICorProfilerInfo3, windows_core::IUnknown, ICorProfilerInfo, ICorProfilerInfo2);
impl ICorProfilerInfo3 {
    pub unsafe fn EnumJITedFunctions(&self) -> windows_core::Result<ICorProfilerFunctionEnum> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumJITedFunctions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn RequestProfilerDetach(&self, dwexpectedcompletionmilliseconds: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RequestProfilerDetach)(windows_core::Interface::as_raw(self), dwexpectedcompletionmilliseconds).ok() }
    }
    pub unsafe fn SetFunctionIDMapper2(&self, pfunc: *const FunctionIDMapper2, clientdata: *const core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFunctionIDMapper2)(windows_core::Interface::as_raw(self), pfunc, clientdata).ok() }
    }
    pub unsafe fn GetStringLayout2(&self, pstringlengthoffset: *mut u32, pbufferoffset: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetStringLayout2)(windows_core::Interface::as_raw(self), pstringlengthoffset as _, pbufferoffset as _).ok() }
    }
    pub unsafe fn SetEnterLeaveFunctionHooks3(&self, pfuncenter3: *const FunctionEnter3, pfuncleave3: *const FunctionLeave3, pfunctailcall3: *const FunctionTailcall3) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEnterLeaveFunctionHooks3)(windows_core::Interface::as_raw(self), pfuncenter3, pfuncleave3, pfunctailcall3).ok() }
    }
    pub unsafe fn SetEnterLeaveFunctionHooks3WithInfo(&self, pfuncenter3withinfo: *const FunctionEnter3WithInfo, pfuncleave3withinfo: *const FunctionLeave3WithInfo, pfunctailcall3withinfo: *const FunctionTailcall3WithInfo) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEnterLeaveFunctionHooks3WithInfo)(windows_core::Interface::as_raw(self), pfuncenter3withinfo, pfuncleave3withinfo, pfunctailcall3withinfo).ok() }
    }
    pub unsafe fn GetFunctionEnter3Info(&self, functionid: usize, eltinfo: usize, pframeinfo: *mut usize, pcbargumentinfo: *mut u32, pargumentinfo: *mut COR_PRF_FUNCTION_ARGUMENT_INFO) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetFunctionEnter3Info)(windows_core::Interface::as_raw(self), functionid, eltinfo, pframeinfo as _, pcbargumentinfo as _, pargumentinfo as _).ok() }
    }
    pub unsafe fn GetFunctionLeave3Info(&self, functionid: usize, eltinfo: usize, pframeinfo: *mut usize, pretvalrange: *mut COR_PRF_FUNCTION_ARGUMENT_RANGE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetFunctionLeave3Info)(windows_core::Interface::as_raw(self), functionid, eltinfo, pframeinfo as _, pretvalrange as _).ok() }
    }
    pub unsafe fn GetFunctionTailcall3Info(&self, functionid: usize, eltinfo: usize) -> windows_core::Result<usize> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFunctionTailcall3Info)(windows_core::Interface::as_raw(self), functionid, eltinfo, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EnumModules(&self) -> windows_core::Result<ICorProfilerModuleEnum> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumModules)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetRuntimeInformation(&self, pclrinstanceid: *mut u16, pruntimetype: *mut COR_PRF_RUNTIME_TYPE, pmajorversion: *mut u16, pminorversion: *mut u16, pbuildnumber: *mut u16, pqfeversion: *mut u16, pcchversionstring: *mut u32, szversionstring: &mut [u16]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetRuntimeInformation)(windows_core::Interface::as_raw(self), pclrinstanceid as _, pruntimetype as _, pmajorversion as _, pminorversion as _, pbuildnumber as _, pqfeversion as _, szversionstring.len().try_into().unwrap(), pcchversionstring as _, core::mem::transmute(szversionstring.as_ptr())).ok() }
    }
    pub unsafe fn GetThreadStaticAddress2(&self, classid: usize, fieldtoken: u32, appdomainid: usize, threadid: usize, ppaddress: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetThreadStaticAddress2)(windows_core::Interface::as_raw(self), classid, fieldtoken, appdomainid, threadid, ppaddress as _).ok() }
    }
    pub unsafe fn GetAppDomainsContainingModule(&self, moduleid: usize, pcappdomainids: *mut u32, appdomainids: &mut [usize]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetAppDomainsContainingModule)(windows_core::Interface::as_raw(self), moduleid, appdomainids.len().try_into().unwrap(), pcappdomainids as _, core::mem::transmute(appdomainids.as_ptr())).ok() }
    }
    pub unsafe fn GetModuleInfo2(&self, moduleid: usize, ppbaseloadaddress: *mut *mut u8, pcchname: *mut u32, szname: &mut [u16], passemblyid: *mut usize, pdwmoduleflags: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetModuleInfo2)(windows_core::Interface::as_raw(self), moduleid, ppbaseloadaddress as _, szname.len().try_into().unwrap(), pcchname as _, core::mem::transmute(szname.as_ptr()), passemblyid as _, pdwmoduleflags as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorProfilerInfo3_Vtbl {
    pub base__: ICorProfilerInfo2_Vtbl,
    pub EnumJITedFunctions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestProfilerDetach: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetFunctionIDMapper2: unsafe extern "system" fn(*mut core::ffi::c_void, *const FunctionIDMapper2, *const core::ffi::c_void) -> windows_core::HRESULT,
    pub GetStringLayout2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub SetEnterLeaveFunctionHooks3: unsafe extern "system" fn(*mut core::ffi::c_void, *const FunctionEnter3, *const FunctionLeave3, *const FunctionTailcall3) -> windows_core::HRESULT,
    pub SetEnterLeaveFunctionHooks3WithInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *const FunctionEnter3WithInfo, *const FunctionLeave3WithInfo, *const FunctionTailcall3WithInfo) -> windows_core::HRESULT,
    pub GetFunctionEnter3Info: unsafe extern "system" fn(*mut core::ffi::c_void, usize, usize, *mut usize, *mut u32, *mut COR_PRF_FUNCTION_ARGUMENT_INFO) -> windows_core::HRESULT,
    pub GetFunctionLeave3Info: unsafe extern "system" fn(*mut core::ffi::c_void, usize, usize, *mut usize, *mut COR_PRF_FUNCTION_ARGUMENT_RANGE) -> windows_core::HRESULT,
    pub GetFunctionTailcall3Info: unsafe extern "system" fn(*mut core::ffi::c_void, usize, usize, *mut usize) -> windows_core::HRESULT,
    pub EnumModules: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRuntimeInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16, *mut COR_PRF_RUNTIME_TYPE, *mut u16, *mut u16, *mut u16, *mut u16, u32, *mut u32, windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetThreadStaticAddress2: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32, usize, usize, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAppDomainsContainingModule: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32, *mut u32, *mut usize) -> windows_core::HRESULT,
    pub GetModuleInfo2: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut *mut u8, u32, *mut u32, windows_core::PWSTR, *mut usize, *mut u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
pub trait ICorProfilerInfo3_Impl: ICorProfilerInfo2_Impl {
    fn EnumJITedFunctions(&self) -> windows_core::Result<ICorProfilerFunctionEnum>;
    fn RequestProfilerDetach(&self, dwexpectedcompletionmilliseconds: u32) -> windows_core::Result<()>;
    fn SetFunctionIDMapper2(&self, pfunc: *const FunctionIDMapper2, clientdata: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn GetStringLayout2(&self, pstringlengthoffset: *mut u32, pbufferoffset: *mut u32) -> windows_core::Result<()>;
    fn SetEnterLeaveFunctionHooks3(&self, pfuncenter3: *const FunctionEnter3, pfuncleave3: *const FunctionLeave3, pfunctailcall3: *const FunctionTailcall3) -> windows_core::Result<()>;
    fn SetEnterLeaveFunctionHooks3WithInfo(&self, pfuncenter3withinfo: *const FunctionEnter3WithInfo, pfuncleave3withinfo: *const FunctionLeave3WithInfo, pfunctailcall3withinfo: *const FunctionTailcall3WithInfo) -> windows_core::Result<()>;
    fn GetFunctionEnter3Info(&self, functionid: usize, eltinfo: usize, pframeinfo: *mut usize, pcbargumentinfo: *mut u32, pargumentinfo: *mut COR_PRF_FUNCTION_ARGUMENT_INFO) -> windows_core::Result<()>;
    fn GetFunctionLeave3Info(&self, functionid: usize, eltinfo: usize, pframeinfo: *mut usize, pretvalrange: *mut COR_PRF_FUNCTION_ARGUMENT_RANGE) -> windows_core::Result<()>;
    fn GetFunctionTailcall3Info(&self, functionid: usize, eltinfo: usize) -> windows_core::Result<usize>;
    fn EnumModules(&self) -> windows_core::Result<ICorProfilerModuleEnum>;
    fn GetRuntimeInformation(&self, pclrinstanceid: *mut u16, pruntimetype: *mut COR_PRF_RUNTIME_TYPE, pmajorversion: *mut u16, pminorversion: *mut u16, pbuildnumber: *mut u16, pqfeversion: *mut u16, cchversionstring: u32, pcchversionstring: *mut u32, szversionstring: windows_core::PWSTR) -> windows_core::Result<()>;
    fn GetThreadStaticAddress2(&self, classid: usize, fieldtoken: u32, appdomainid: usize, threadid: usize, ppaddress: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetAppDomainsContainingModule(&self, moduleid: usize, cappdomainids: u32, pcappdomainids: *mut u32, appdomainids: *mut usize) -> windows_core::Result<()>;
    fn GetModuleInfo2(&self, moduleid: usize, ppbaseloadaddress: *mut *mut u8, cchname: u32, pcchname: *mut u32, szname: windows_core::PWSTR, passemblyid: *mut usize, pdwmoduleflags: *mut u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl ICorProfilerInfo3_Vtbl {
    pub const fn new<Identity: ICorProfilerInfo3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumJITedFunctions<Identity: ICorProfilerInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorProfilerInfo3_Impl::EnumJITedFunctions(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RequestProfilerDetach<Identity: ICorProfilerInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwexpectedcompletionmilliseconds: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo3_Impl::RequestProfilerDetach(this, core::mem::transmute_copy(&dwexpectedcompletionmilliseconds)).into()
            }
        }
        unsafe extern "system" fn SetFunctionIDMapper2<Identity: ICorProfilerInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfunc: *const FunctionIDMapper2, clientdata: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo3_Impl::SetFunctionIDMapper2(this, core::mem::transmute_copy(&pfunc), core::mem::transmute_copy(&clientdata)).into()
            }
        }
        unsafe extern "system" fn GetStringLayout2<Identity: ICorProfilerInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstringlengthoffset: *mut u32, pbufferoffset: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo3_Impl::GetStringLayout2(this, core::mem::transmute_copy(&pstringlengthoffset), core::mem::transmute_copy(&pbufferoffset)).into()
            }
        }
        unsafe extern "system" fn SetEnterLeaveFunctionHooks3<Identity: ICorProfilerInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfuncenter3: *const FunctionEnter3, pfuncleave3: *const FunctionLeave3, pfunctailcall3: *const FunctionTailcall3) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo3_Impl::SetEnterLeaveFunctionHooks3(this, core::mem::transmute_copy(&pfuncenter3), core::mem::transmute_copy(&pfuncleave3), core::mem::transmute_copy(&pfunctailcall3)).into()
            }
        }
        unsafe extern "system" fn SetEnterLeaveFunctionHooks3WithInfo<Identity: ICorProfilerInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfuncenter3withinfo: *const FunctionEnter3WithInfo, pfuncleave3withinfo: *const FunctionLeave3WithInfo, pfunctailcall3withinfo: *const FunctionTailcall3WithInfo) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo3_Impl::SetEnterLeaveFunctionHooks3WithInfo(this, core::mem::transmute_copy(&pfuncenter3withinfo), core::mem::transmute_copy(&pfuncleave3withinfo), core::mem::transmute_copy(&pfunctailcall3withinfo)).into()
            }
        }
        unsafe extern "system" fn GetFunctionEnter3Info<Identity: ICorProfilerInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize, eltinfo: usize, pframeinfo: *mut usize, pcbargumentinfo: *mut u32, pargumentinfo: *mut COR_PRF_FUNCTION_ARGUMENT_INFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo3_Impl::GetFunctionEnter3Info(this, core::mem::transmute_copy(&functionid), core::mem::transmute_copy(&eltinfo), core::mem::transmute_copy(&pframeinfo), core::mem::transmute_copy(&pcbargumentinfo), core::mem::transmute_copy(&pargumentinfo)).into()
            }
        }
        unsafe extern "system" fn GetFunctionLeave3Info<Identity: ICorProfilerInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize, eltinfo: usize, pframeinfo: *mut usize, pretvalrange: *mut COR_PRF_FUNCTION_ARGUMENT_RANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo3_Impl::GetFunctionLeave3Info(this, core::mem::transmute_copy(&functionid), core::mem::transmute_copy(&eltinfo), core::mem::transmute_copy(&pframeinfo), core::mem::transmute_copy(&pretvalrange)).into()
            }
        }
        unsafe extern "system" fn GetFunctionTailcall3Info<Identity: ICorProfilerInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize, eltinfo: usize, pframeinfo: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorProfilerInfo3_Impl::GetFunctionTailcall3Info(this, core::mem::transmute_copy(&functionid), core::mem::transmute_copy(&eltinfo)) {
                    Ok(ok__) => {
                        pframeinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumModules<Identity: ICorProfilerInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorProfilerInfo3_Impl::EnumModules(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRuntimeInformation<Identity: ICorProfilerInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclrinstanceid: *mut u16, pruntimetype: *mut COR_PRF_RUNTIME_TYPE, pmajorversion: *mut u16, pminorversion: *mut u16, pbuildnumber: *mut u16, pqfeversion: *mut u16, cchversionstring: u32, pcchversionstring: *mut u32, szversionstring: windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo3_Impl::GetRuntimeInformation(this, core::mem::transmute_copy(&pclrinstanceid), core::mem::transmute_copy(&pruntimetype), core::mem::transmute_copy(&pmajorversion), core::mem::transmute_copy(&pminorversion), core::mem::transmute_copy(&pbuildnumber), core::mem::transmute_copy(&pqfeversion), core::mem::transmute_copy(&cchversionstring), core::mem::transmute_copy(&pcchversionstring), core::mem::transmute_copy(&szversionstring)).into()
            }
        }
        unsafe extern "system" fn GetThreadStaticAddress2<Identity: ICorProfilerInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, classid: usize, fieldtoken: u32, appdomainid: usize, threadid: usize, ppaddress: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo3_Impl::GetThreadStaticAddress2(this, core::mem::transmute_copy(&classid), core::mem::transmute_copy(&fieldtoken), core::mem::transmute_copy(&appdomainid), core::mem::transmute_copy(&threadid), core::mem::transmute_copy(&ppaddress)).into()
            }
        }
        unsafe extern "system" fn GetAppDomainsContainingModule<Identity: ICorProfilerInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduleid: usize, cappdomainids: u32, pcappdomainids: *mut u32, appdomainids: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo3_Impl::GetAppDomainsContainingModule(this, core::mem::transmute_copy(&moduleid), core::mem::transmute_copy(&cappdomainids), core::mem::transmute_copy(&pcappdomainids), core::mem::transmute_copy(&appdomainids)).into()
            }
        }
        unsafe extern "system" fn GetModuleInfo2<Identity: ICorProfilerInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduleid: usize, ppbaseloadaddress: *mut *mut u8, cchname: u32, pcchname: *mut u32, szname: windows_core::PWSTR, passemblyid: *mut usize, pdwmoduleflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo3_Impl::GetModuleInfo2(this, core::mem::transmute_copy(&moduleid), core::mem::transmute_copy(&ppbaseloadaddress), core::mem::transmute_copy(&cchname), core::mem::transmute_copy(&pcchname), core::mem::transmute_copy(&szname), core::mem::transmute_copy(&passemblyid), core::mem::transmute_copy(&pdwmoduleflags)).into()
            }
        }
        Self {
            base__: ICorProfilerInfo2_Vtbl::new::<Identity, OFFSET>(),
            EnumJITedFunctions: EnumJITedFunctions::<Identity, OFFSET>,
            RequestProfilerDetach: RequestProfilerDetach::<Identity, OFFSET>,
            SetFunctionIDMapper2: SetFunctionIDMapper2::<Identity, OFFSET>,
            GetStringLayout2: GetStringLayout2::<Identity, OFFSET>,
            SetEnterLeaveFunctionHooks3: SetEnterLeaveFunctionHooks3::<Identity, OFFSET>,
            SetEnterLeaveFunctionHooks3WithInfo: SetEnterLeaveFunctionHooks3WithInfo::<Identity, OFFSET>,
            GetFunctionEnter3Info: GetFunctionEnter3Info::<Identity, OFFSET>,
            GetFunctionLeave3Info: GetFunctionLeave3Info::<Identity, OFFSET>,
            GetFunctionTailcall3Info: GetFunctionTailcall3Info::<Identity, OFFSET>,
            EnumModules: EnumModules::<Identity, OFFSET>,
            GetRuntimeInformation: GetRuntimeInformation::<Identity, OFFSET>,
            GetThreadStaticAddress2: GetThreadStaticAddress2::<Identity, OFFSET>,
            GetAppDomainsContainingModule: GetAppDomainsContainingModule::<Identity, OFFSET>,
            GetModuleInfo2: GetModuleInfo2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorProfilerInfo3 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo as windows_core::Interface>::IID || iid == &<ICorProfilerInfo2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl windows_core::RuntimeName for ICorProfilerInfo3 {}
windows_core::imp::define_interface!(ICorProfilerInfo4, ICorProfilerInfo4_Vtbl, 0x0d8fdcaa_6257_47bf_b1bf_94dac88466ee);
impl core::ops::Deref for ICorProfilerInfo4 {
    type Target = ICorProfilerInfo3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICorProfilerInfo4, windows_core::IUnknown, ICorProfilerInfo, ICorProfilerInfo2, ICorProfilerInfo3);
impl ICorProfilerInfo4 {
    pub unsafe fn EnumThreads(&self) -> windows_core::Result<ICorProfilerThreadEnum> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumThreads)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn InitializeCurrentThread(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).InitializeCurrentThread)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn RequestReJIT(&self, cfunctions: u32, moduleids: *const usize, methodids: *const u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RequestReJIT)(windows_core::Interface::as_raw(self), cfunctions, moduleids, methodids).ok() }
    }
    pub unsafe fn RequestRevert(&self, cfunctions: u32, moduleids: *const usize, methodids: *const u32, status: *mut windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RequestRevert)(windows_core::Interface::as_raw(self), cfunctions, moduleids, methodids, status as _).ok() }
    }
    pub unsafe fn GetCodeInfo3(&self, functionid: usize, rejitid: usize, pccodeinfos: *mut u32, codeinfos: &mut [COR_PRF_CODE_INFO]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCodeInfo3)(windows_core::Interface::as_raw(self), functionid, rejitid, codeinfos.len().try_into().unwrap(), pccodeinfos as _, core::mem::transmute(codeinfos.as_ptr())).ok() }
    }
    pub unsafe fn GetFunctionFromIP2(&self, ip: *const u8, pfunctionid: *mut usize, prejitid: *mut usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetFunctionFromIP2)(windows_core::Interface::as_raw(self), ip, pfunctionid as _, prejitid as _).ok() }
    }
    pub unsafe fn GetReJITIDs(&self, functionid: usize, pcrejitids: *mut u32, rejitids: &mut [usize]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetReJITIDs)(windows_core::Interface::as_raw(self), functionid, rejitids.len().try_into().unwrap(), pcrejitids as _, core::mem::transmute(rejitids.as_ptr())).ok() }
    }
    pub unsafe fn GetILToNativeMapping2(&self, functionid: usize, rejitid: usize, pcmap: *mut u32, map: &mut [COR_DEBUG_IL_TO_NATIVE_MAP]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetILToNativeMapping2)(windows_core::Interface::as_raw(self), functionid, rejitid, map.len().try_into().unwrap(), pcmap as _, core::mem::transmute(map.as_ptr())).ok() }
    }
    pub unsafe fn EnumJITedFunctions2(&self) -> windows_core::Result<ICorProfilerFunctionEnum> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumJITedFunctions2)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetObjectSize2(&self, objectid: usize) -> windows_core::Result<usize> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetObjectSize2)(windows_core::Interface::as_raw(self), objectid, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorProfilerInfo4_Vtbl {
    pub base__: ICorProfilerInfo3_Vtbl,
    pub EnumThreads: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InitializeCurrentThread: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestReJIT: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const usize, *const u32) -> windows_core::HRESULT,
    pub RequestRevert: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const usize, *const u32, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    pub GetCodeInfo3: unsafe extern "system" fn(*mut core::ffi::c_void, usize, usize, u32, *mut u32, *mut COR_PRF_CODE_INFO) -> windows_core::HRESULT,
    pub GetFunctionFromIP2: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, *mut usize, *mut usize) -> windows_core::HRESULT,
    pub GetReJITIDs: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32, *mut u32, *mut usize) -> windows_core::HRESULT,
    pub GetILToNativeMapping2: unsafe extern "system" fn(*mut core::ffi::c_void, usize, usize, u32, *mut u32, *mut COR_DEBUG_IL_TO_NATIVE_MAP) -> windows_core::HRESULT,
    pub EnumJITedFunctions2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetObjectSize2: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut usize) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
pub trait ICorProfilerInfo4_Impl: ICorProfilerInfo3_Impl {
    fn EnumThreads(&self) -> windows_core::Result<ICorProfilerThreadEnum>;
    fn InitializeCurrentThread(&self) -> windows_core::Result<()>;
    fn RequestReJIT(&self, cfunctions: u32, moduleids: *const usize, methodids: *const u32) -> windows_core::Result<()>;
    fn RequestRevert(&self, cfunctions: u32, moduleids: *const usize, methodids: *const u32, status: *mut windows_core::HRESULT) -> windows_core::Result<()>;
    fn GetCodeInfo3(&self, functionid: usize, rejitid: usize, ccodeinfos: u32, pccodeinfos: *mut u32, codeinfos: *mut COR_PRF_CODE_INFO) -> windows_core::Result<()>;
    fn GetFunctionFromIP2(&self, ip: *const u8, pfunctionid: *mut usize, prejitid: *mut usize) -> windows_core::Result<()>;
    fn GetReJITIDs(&self, functionid: usize, crejitids: u32, pcrejitids: *mut u32, rejitids: *mut usize) -> windows_core::Result<()>;
    fn GetILToNativeMapping2(&self, functionid: usize, rejitid: usize, cmap: u32, pcmap: *mut u32, map: *mut COR_DEBUG_IL_TO_NATIVE_MAP) -> windows_core::Result<()>;
    fn EnumJITedFunctions2(&self) -> windows_core::Result<ICorProfilerFunctionEnum>;
    fn GetObjectSize2(&self, objectid: usize) -> windows_core::Result<usize>;
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl ICorProfilerInfo4_Vtbl {
    pub const fn new<Identity: ICorProfilerInfo4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumThreads<Identity: ICorProfilerInfo4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorProfilerInfo4_Impl::EnumThreads(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InitializeCurrentThread<Identity: ICorProfilerInfo4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo4_Impl::InitializeCurrentThread(this).into()
            }
        }
        unsafe extern "system" fn RequestReJIT<Identity: ICorProfilerInfo4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cfunctions: u32, moduleids: *const usize, methodids: *const u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo4_Impl::RequestReJIT(this, core::mem::transmute_copy(&cfunctions), core::mem::transmute_copy(&moduleids), core::mem::transmute_copy(&methodids)).into()
            }
        }
        unsafe extern "system" fn RequestRevert<Identity: ICorProfilerInfo4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cfunctions: u32, moduleids: *const usize, methodids: *const u32, status: *mut windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo4_Impl::RequestRevert(this, core::mem::transmute_copy(&cfunctions), core::mem::transmute_copy(&moduleids), core::mem::transmute_copy(&methodids), core::mem::transmute_copy(&status)).into()
            }
        }
        unsafe extern "system" fn GetCodeInfo3<Identity: ICorProfilerInfo4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize, rejitid: usize, ccodeinfos: u32, pccodeinfos: *mut u32, codeinfos: *mut COR_PRF_CODE_INFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo4_Impl::GetCodeInfo3(this, core::mem::transmute_copy(&functionid), core::mem::transmute_copy(&rejitid), core::mem::transmute_copy(&ccodeinfos), core::mem::transmute_copy(&pccodeinfos), core::mem::transmute_copy(&codeinfos)).into()
            }
        }
        unsafe extern "system" fn GetFunctionFromIP2<Identity: ICorProfilerInfo4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ip: *const u8, pfunctionid: *mut usize, prejitid: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo4_Impl::GetFunctionFromIP2(this, core::mem::transmute_copy(&ip), core::mem::transmute_copy(&pfunctionid), core::mem::transmute_copy(&prejitid)).into()
            }
        }
        unsafe extern "system" fn GetReJITIDs<Identity: ICorProfilerInfo4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize, crejitids: u32, pcrejitids: *mut u32, rejitids: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo4_Impl::GetReJITIDs(this, core::mem::transmute_copy(&functionid), core::mem::transmute_copy(&crejitids), core::mem::transmute_copy(&pcrejitids), core::mem::transmute_copy(&rejitids)).into()
            }
        }
        unsafe extern "system" fn GetILToNativeMapping2<Identity: ICorProfilerInfo4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize, rejitid: usize, cmap: u32, pcmap: *mut u32, map: *mut COR_DEBUG_IL_TO_NATIVE_MAP) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo4_Impl::GetILToNativeMapping2(this, core::mem::transmute_copy(&functionid), core::mem::transmute_copy(&rejitid), core::mem::transmute_copy(&cmap), core::mem::transmute_copy(&pcmap), core::mem::transmute_copy(&map)).into()
            }
        }
        unsafe extern "system" fn EnumJITedFunctions2<Identity: ICorProfilerInfo4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorProfilerInfo4_Impl::EnumJITedFunctions2(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetObjectSize2<Identity: ICorProfilerInfo4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objectid: usize, pcsize: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorProfilerInfo4_Impl::GetObjectSize2(this, core::mem::transmute_copy(&objectid)) {
                    Ok(ok__) => {
                        pcsize.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: ICorProfilerInfo3_Vtbl::new::<Identity, OFFSET>(),
            EnumThreads: EnumThreads::<Identity, OFFSET>,
            InitializeCurrentThread: InitializeCurrentThread::<Identity, OFFSET>,
            RequestReJIT: RequestReJIT::<Identity, OFFSET>,
            RequestRevert: RequestRevert::<Identity, OFFSET>,
            GetCodeInfo3: GetCodeInfo3::<Identity, OFFSET>,
            GetFunctionFromIP2: GetFunctionFromIP2::<Identity, OFFSET>,
            GetReJITIDs: GetReJITIDs::<Identity, OFFSET>,
            GetILToNativeMapping2: GetILToNativeMapping2::<Identity, OFFSET>,
            EnumJITedFunctions2: EnumJITedFunctions2::<Identity, OFFSET>,
            GetObjectSize2: GetObjectSize2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorProfilerInfo4 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo as windows_core::Interface>::IID || iid == &<ICorProfilerInfo2 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo3 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl windows_core::RuntimeName for ICorProfilerInfo4 {}
windows_core::imp::define_interface!(ICorProfilerInfo5, ICorProfilerInfo5_Vtbl, 0x07602928_ce38_4b83_81e7_74adaf781214);
impl core::ops::Deref for ICorProfilerInfo5 {
    type Target = ICorProfilerInfo4;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICorProfilerInfo5, windows_core::IUnknown, ICorProfilerInfo, ICorProfilerInfo2, ICorProfilerInfo3, ICorProfilerInfo4);
impl ICorProfilerInfo5 {
    pub unsafe fn GetEventMask2(&self, pdweventslow: *mut u32, pdweventshigh: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetEventMask2)(windows_core::Interface::as_raw(self), pdweventslow as _, pdweventshigh as _).ok() }
    }
    pub unsafe fn SetEventMask2(&self, dweventslow: u32, dweventshigh: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEventMask2)(windows_core::Interface::as_raw(self), dweventslow, dweventshigh).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorProfilerInfo5_Vtbl {
    pub base__: ICorProfilerInfo4_Vtbl,
    pub GetEventMask2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub SetEventMask2: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
pub trait ICorProfilerInfo5_Impl: ICorProfilerInfo4_Impl {
    fn GetEventMask2(&self, pdweventslow: *mut u32, pdweventshigh: *mut u32) -> windows_core::Result<()>;
    fn SetEventMask2(&self, dweventslow: u32, dweventshigh: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl ICorProfilerInfo5_Vtbl {
    pub const fn new<Identity: ICorProfilerInfo5_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetEventMask2<Identity: ICorProfilerInfo5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdweventslow: *mut u32, pdweventshigh: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo5_Impl::GetEventMask2(this, core::mem::transmute_copy(&pdweventslow), core::mem::transmute_copy(&pdweventshigh)).into()
            }
        }
        unsafe extern "system" fn SetEventMask2<Identity: ICorProfilerInfo5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dweventslow: u32, dweventshigh: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo5_Impl::SetEventMask2(this, core::mem::transmute_copy(&dweventslow), core::mem::transmute_copy(&dweventshigh)).into()
            }
        }
        Self {
            base__: ICorProfilerInfo4_Vtbl::new::<Identity, OFFSET>(),
            GetEventMask2: GetEventMask2::<Identity, OFFSET>,
            SetEventMask2: SetEventMask2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorProfilerInfo5 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo as windows_core::Interface>::IID || iid == &<ICorProfilerInfo2 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo3 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo4 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl windows_core::RuntimeName for ICorProfilerInfo5 {}
windows_core::imp::define_interface!(ICorProfilerInfo6, ICorProfilerInfo6_Vtbl, 0xf30a070d_bffb_46a7_b1d8_8781ef7b698a);
impl core::ops::Deref for ICorProfilerInfo6 {
    type Target = ICorProfilerInfo5;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICorProfilerInfo6, windows_core::IUnknown, ICorProfilerInfo, ICorProfilerInfo2, ICorProfilerInfo3, ICorProfilerInfo4, ICorProfilerInfo5);
impl ICorProfilerInfo6 {
    pub unsafe fn EnumNgenModuleMethodsInliningThisMethod(&self, inlinersmoduleid: usize, inlineemoduleid: usize, inlineemethodid: u32, incompletedata: *mut windows_core::BOOL, ppenum: *mut Option<ICorProfilerMethodEnum>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnumNgenModuleMethodsInliningThisMethod)(windows_core::Interface::as_raw(self), inlinersmoduleid, inlineemoduleid, inlineemethodid, incompletedata as _, core::mem::transmute(ppenum)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorProfilerInfo6_Vtbl {
    pub base__: ICorProfilerInfo5_Vtbl,
    pub EnumNgenModuleMethodsInliningThisMethod: unsafe extern "system" fn(*mut core::ffi::c_void, usize, usize, u32, *mut windows_core::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
pub trait ICorProfilerInfo6_Impl: ICorProfilerInfo5_Impl {
    fn EnumNgenModuleMethodsInliningThisMethod(&self, inlinersmoduleid: usize, inlineemoduleid: usize, inlineemethodid: u32, incompletedata: *mut windows_core::BOOL, ppenum: windows_core::OutRef<ICorProfilerMethodEnum>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl ICorProfilerInfo6_Vtbl {
    pub const fn new<Identity: ICorProfilerInfo6_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumNgenModuleMethodsInliningThisMethod<Identity: ICorProfilerInfo6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inlinersmoduleid: usize, inlineemoduleid: usize, inlineemethodid: u32, incompletedata: *mut windows_core::BOOL, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo6_Impl::EnumNgenModuleMethodsInliningThisMethod(this, core::mem::transmute_copy(&inlinersmoduleid), core::mem::transmute_copy(&inlineemoduleid), core::mem::transmute_copy(&inlineemethodid), core::mem::transmute_copy(&incompletedata), core::mem::transmute_copy(&ppenum)).into()
            }
        }
        Self {
            base__: ICorProfilerInfo5_Vtbl::new::<Identity, OFFSET>(),
            EnumNgenModuleMethodsInliningThisMethod: EnumNgenModuleMethodsInliningThisMethod::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorProfilerInfo6 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo as windows_core::Interface>::IID || iid == &<ICorProfilerInfo2 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo3 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo4 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo5 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl windows_core::RuntimeName for ICorProfilerInfo6 {}
windows_core::imp::define_interface!(ICorProfilerInfo7, ICorProfilerInfo7_Vtbl, 0x9aeecc0d_63e0_4187_8c00_e312f503f663);
impl core::ops::Deref for ICorProfilerInfo7 {
    type Target = ICorProfilerInfo6;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICorProfilerInfo7, windows_core::IUnknown, ICorProfilerInfo, ICorProfilerInfo2, ICorProfilerInfo3, ICorProfilerInfo4, ICorProfilerInfo5, ICorProfilerInfo6);
impl ICorProfilerInfo7 {
    pub unsafe fn ApplyMetaData(&self, moduleid: usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ApplyMetaData)(windows_core::Interface::as_raw(self), moduleid).ok() }
    }
    pub unsafe fn GetInMemorySymbolsLength(&self, moduleid: usize) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInMemorySymbolsLength)(windows_core::Interface::as_raw(self), moduleid, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ReadInMemorySymbols(&self, moduleid: usize, symbolsreadoffset: u32, psymbolbytes: *mut u8, countsymbolbytes: u32, pcountsymbolbytesread: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReadInMemorySymbols)(windows_core::Interface::as_raw(self), moduleid, symbolsreadoffset, psymbolbytes as _, countsymbolbytes, pcountsymbolbytesread as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorProfilerInfo7_Vtbl {
    pub base__: ICorProfilerInfo6_Vtbl,
    pub ApplyMetaData: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    pub GetInMemorySymbolsLength: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut u32) -> windows_core::HRESULT,
    pub ReadInMemorySymbols: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32, *mut u8, u32, *mut u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
pub trait ICorProfilerInfo7_Impl: ICorProfilerInfo6_Impl {
    fn ApplyMetaData(&self, moduleid: usize) -> windows_core::Result<()>;
    fn GetInMemorySymbolsLength(&self, moduleid: usize) -> windows_core::Result<u32>;
    fn ReadInMemorySymbols(&self, moduleid: usize, symbolsreadoffset: u32, psymbolbytes: *mut u8, countsymbolbytes: u32, pcountsymbolbytesread: *mut u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl ICorProfilerInfo7_Vtbl {
    pub const fn new<Identity: ICorProfilerInfo7_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ApplyMetaData<Identity: ICorProfilerInfo7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduleid: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo7_Impl::ApplyMetaData(this, core::mem::transmute_copy(&moduleid)).into()
            }
        }
        unsafe extern "system" fn GetInMemorySymbolsLength<Identity: ICorProfilerInfo7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduleid: usize, pcountsymbolbytes: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorProfilerInfo7_Impl::GetInMemorySymbolsLength(this, core::mem::transmute_copy(&moduleid)) {
                    Ok(ok__) => {
                        pcountsymbolbytes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReadInMemorySymbols<Identity: ICorProfilerInfo7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduleid: usize, symbolsreadoffset: u32, psymbolbytes: *mut u8, countsymbolbytes: u32, pcountsymbolbytesread: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo7_Impl::ReadInMemorySymbols(this, core::mem::transmute_copy(&moduleid), core::mem::transmute_copy(&symbolsreadoffset), core::mem::transmute_copy(&psymbolbytes), core::mem::transmute_copy(&countsymbolbytes), core::mem::transmute_copy(&pcountsymbolbytesread)).into()
            }
        }
        Self {
            base__: ICorProfilerInfo6_Vtbl::new::<Identity, OFFSET>(),
            ApplyMetaData: ApplyMetaData::<Identity, OFFSET>,
            GetInMemorySymbolsLength: GetInMemorySymbolsLength::<Identity, OFFSET>,
            ReadInMemorySymbols: ReadInMemorySymbols::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorProfilerInfo7 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo as windows_core::Interface>::IID || iid == &<ICorProfilerInfo2 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo3 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo4 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo5 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo6 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl windows_core::RuntimeName for ICorProfilerInfo7 {}
windows_core::imp::define_interface!(ICorProfilerInfo8, ICorProfilerInfo8_Vtbl, 0xc5ac80a6_782e_4716_8044_39598c60cfbf);
impl core::ops::Deref for ICorProfilerInfo8 {
    type Target = ICorProfilerInfo7;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICorProfilerInfo8, windows_core::IUnknown, ICorProfilerInfo, ICorProfilerInfo2, ICorProfilerInfo3, ICorProfilerInfo4, ICorProfilerInfo5, ICorProfilerInfo6, ICorProfilerInfo7);
impl ICorProfilerInfo8 {
    pub unsafe fn IsFunctionDynamic(&self, functionid: usize) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsFunctionDynamic)(windows_core::Interface::as_raw(self), functionid, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetFunctionFromIP3(&self, ip: *const u8, functionid: *mut usize, prejitid: *mut usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetFunctionFromIP3)(windows_core::Interface::as_raw(self), ip, functionid as _, prejitid as _).ok() }
    }
    pub unsafe fn GetDynamicFunctionInfo(&self, functionid: usize, moduleid: *mut usize, ppvsig: *mut *mut u8, pbsig: *mut u32, cchname: u32, pcchname: *mut u32, wszname: windows_core::PWSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDynamicFunctionInfo)(windows_core::Interface::as_raw(self), functionid, moduleid as _, ppvsig as _, pbsig as _, cchname, pcchname as _, core::mem::transmute(wszname)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorProfilerInfo8_Vtbl {
    pub base__: ICorProfilerInfo7_Vtbl,
    pub IsFunctionDynamic: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetFunctionFromIP3: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, *mut usize, *mut usize) -> windows_core::HRESULT,
    pub GetDynamicFunctionInfo: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut usize, *mut *mut u8, *mut u32, u32, *mut u32, windows_core::PWSTR) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
pub trait ICorProfilerInfo8_Impl: ICorProfilerInfo7_Impl {
    fn IsFunctionDynamic(&self, functionid: usize) -> windows_core::Result<windows_core::BOOL>;
    fn GetFunctionFromIP3(&self, ip: *const u8, functionid: *mut usize, prejitid: *mut usize) -> windows_core::Result<()>;
    fn GetDynamicFunctionInfo(&self, functionid: usize, moduleid: *mut usize, ppvsig: *mut *mut u8, pbsig: *mut u32, cchname: u32, pcchname: *mut u32, wszname: windows_core::PWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl ICorProfilerInfo8_Vtbl {
    pub const fn new<Identity: ICorProfilerInfo8_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsFunctionDynamic<Identity: ICorProfilerInfo8_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize, isdynamic: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorProfilerInfo8_Impl::IsFunctionDynamic(this, core::mem::transmute_copy(&functionid)) {
                    Ok(ok__) => {
                        isdynamic.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFunctionFromIP3<Identity: ICorProfilerInfo8_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ip: *const u8, functionid: *mut usize, prejitid: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo8_Impl::GetFunctionFromIP3(this, core::mem::transmute_copy(&ip), core::mem::transmute_copy(&functionid), core::mem::transmute_copy(&prejitid)).into()
            }
        }
        unsafe extern "system" fn GetDynamicFunctionInfo<Identity: ICorProfilerInfo8_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize, moduleid: *mut usize, ppvsig: *mut *mut u8, pbsig: *mut u32, cchname: u32, pcchname: *mut u32, wszname: windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo8_Impl::GetDynamicFunctionInfo(this, core::mem::transmute_copy(&functionid), core::mem::transmute_copy(&moduleid), core::mem::transmute_copy(&ppvsig), core::mem::transmute_copy(&pbsig), core::mem::transmute_copy(&cchname), core::mem::transmute_copy(&pcchname), core::mem::transmute_copy(&wszname)).into()
            }
        }
        Self {
            base__: ICorProfilerInfo7_Vtbl::new::<Identity, OFFSET>(),
            IsFunctionDynamic: IsFunctionDynamic::<Identity, OFFSET>,
            GetFunctionFromIP3: GetFunctionFromIP3::<Identity, OFFSET>,
            GetDynamicFunctionInfo: GetDynamicFunctionInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorProfilerInfo8 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo as windows_core::Interface>::IID || iid == &<ICorProfilerInfo2 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo3 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo4 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo5 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo6 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo7 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl windows_core::RuntimeName for ICorProfilerInfo8 {}
windows_core::imp::define_interface!(ICorProfilerInfo9, ICorProfilerInfo9_Vtbl, 0x008170db_f8cc_4796_9a51_dc8aa0b47012);
impl core::ops::Deref for ICorProfilerInfo9 {
    type Target = ICorProfilerInfo8;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICorProfilerInfo9, windows_core::IUnknown, ICorProfilerInfo, ICorProfilerInfo2, ICorProfilerInfo3, ICorProfilerInfo4, ICorProfilerInfo5, ICorProfilerInfo6, ICorProfilerInfo7, ICorProfilerInfo8);
impl ICorProfilerInfo9 {
    pub unsafe fn GetNativeCodeStartAddresses(&self, functionid: usize, rejitid: usize, ccodestartaddresses: u32, pccodestartaddresses: *mut u32, codestartaddresses: *mut usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetNativeCodeStartAddresses)(windows_core::Interface::as_raw(self), functionid, rejitid, ccodestartaddresses, pccodestartaddresses as _, codestartaddresses as _).ok() }
    }
    pub unsafe fn GetILToNativeMapping3(&self, pnativecodestartaddress: usize, cmap: u32, pcmap: *mut u32, map: *mut COR_DEBUG_IL_TO_NATIVE_MAP) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetILToNativeMapping3)(windows_core::Interface::as_raw(self), pnativecodestartaddress, cmap, pcmap as _, map as _).ok() }
    }
    pub unsafe fn GetCodeInfo4(&self, pnativecodestartaddress: usize, ccodeinfos: u32, pccodeinfos: *mut u32, codeinfos: *mut COR_PRF_CODE_INFO) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCodeInfo4)(windows_core::Interface::as_raw(self), pnativecodestartaddress, ccodeinfos, pccodeinfos as _, codeinfos as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorProfilerInfo9_Vtbl {
    pub base__: ICorProfilerInfo8_Vtbl,
    pub GetNativeCodeStartAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, usize, usize, u32, *mut u32, *mut usize) -> windows_core::HRESULT,
    pub GetILToNativeMapping3: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32, *mut u32, *mut COR_DEBUG_IL_TO_NATIVE_MAP) -> windows_core::HRESULT,
    pub GetCodeInfo4: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32, *mut u32, *mut COR_PRF_CODE_INFO) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
pub trait ICorProfilerInfo9_Impl: ICorProfilerInfo8_Impl {
    fn GetNativeCodeStartAddresses(&self, functionid: usize, rejitid: usize, ccodestartaddresses: u32, pccodestartaddresses: *mut u32, codestartaddresses: *mut usize) -> windows_core::Result<()>;
    fn GetILToNativeMapping3(&self, pnativecodestartaddress: usize, cmap: u32, pcmap: *mut u32, map: *mut COR_DEBUG_IL_TO_NATIVE_MAP) -> windows_core::Result<()>;
    fn GetCodeInfo4(&self, pnativecodestartaddress: usize, ccodeinfos: u32, pccodeinfos: *mut u32, codeinfos: *mut COR_PRF_CODE_INFO) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl ICorProfilerInfo9_Vtbl {
    pub const fn new<Identity: ICorProfilerInfo9_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetNativeCodeStartAddresses<Identity: ICorProfilerInfo9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize, rejitid: usize, ccodestartaddresses: u32, pccodestartaddresses: *mut u32, codestartaddresses: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo9_Impl::GetNativeCodeStartAddresses(this, core::mem::transmute_copy(&functionid), core::mem::transmute_copy(&rejitid), core::mem::transmute_copy(&ccodestartaddresses), core::mem::transmute_copy(&pccodestartaddresses), core::mem::transmute_copy(&codestartaddresses)).into()
            }
        }
        unsafe extern "system" fn GetILToNativeMapping3<Identity: ICorProfilerInfo9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnativecodestartaddress: usize, cmap: u32, pcmap: *mut u32, map: *mut COR_DEBUG_IL_TO_NATIVE_MAP) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo9_Impl::GetILToNativeMapping3(this, core::mem::transmute_copy(&pnativecodestartaddress), core::mem::transmute_copy(&cmap), core::mem::transmute_copy(&pcmap), core::mem::transmute_copy(&map)).into()
            }
        }
        unsafe extern "system" fn GetCodeInfo4<Identity: ICorProfilerInfo9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnativecodestartaddress: usize, ccodeinfos: u32, pccodeinfos: *mut u32, codeinfos: *mut COR_PRF_CODE_INFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerInfo9_Impl::GetCodeInfo4(this, core::mem::transmute_copy(&pnativecodestartaddress), core::mem::transmute_copy(&ccodeinfos), core::mem::transmute_copy(&pccodeinfos), core::mem::transmute_copy(&codeinfos)).into()
            }
        }
        Self {
            base__: ICorProfilerInfo8_Vtbl::new::<Identity, OFFSET>(),
            GetNativeCodeStartAddresses: GetNativeCodeStartAddresses::<Identity, OFFSET>,
            GetILToNativeMapping3: GetILToNativeMapping3::<Identity, OFFSET>,
            GetCodeInfo4: GetCodeInfo4::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorProfilerInfo9 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo as windows_core::Interface>::IID || iid == &<ICorProfilerInfo2 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo3 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo4 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo5 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo6 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo7 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo8 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl windows_core::RuntimeName for ICorProfilerInfo9 {}
windows_core::imp::define_interface!(ICorProfilerMethodEnum, ICorProfilerMethodEnum_Vtbl, 0xfccee788_0088_454b_a811_c99f298d1942);
windows_core::imp::interface_hierarchy!(ICorProfilerMethodEnum, windows_core::IUnknown);
impl ICorProfilerMethodEnum {
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<ICorProfilerMethodEnum> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Next(&self, elements: &mut [COR_PRF_METHOD], pceltfetched: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), elements.len().try_into().unwrap(), core::mem::transmute(elements.as_ptr()), pceltfetched as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorProfilerMethodEnum_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut COR_PRF_METHOD, *mut u32) -> windows_core::HRESULT,
}
pub trait ICorProfilerMethodEnum_Impl: windows_core::IUnknownImpl {
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<ICorProfilerMethodEnum>;
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn Next(&self, celt: u32, elements: *mut COR_PRF_METHOD, pceltfetched: *mut u32) -> windows_core::Result<()>;
}
impl ICorProfilerMethodEnum_Vtbl {
    pub const fn new<Identity: ICorProfilerMethodEnum_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Skip<Identity: ICorProfilerMethodEnum_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerMethodEnum_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: ICorProfilerMethodEnum_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerMethodEnum_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: ICorProfilerMethodEnum_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorProfilerMethodEnum_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCount<Identity: ICorProfilerMethodEnum_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcelt: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorProfilerMethodEnum_Impl::GetCount(this) {
                    Ok(ok__) => {
                        pcelt.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Next<Identity: ICorProfilerMethodEnum_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, elements: *mut COR_PRF_METHOD, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerMethodEnum_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&elements), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorProfilerMethodEnum as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICorProfilerMethodEnum {}
windows_core::imp::define_interface!(ICorProfilerModuleEnum, ICorProfilerModuleEnum_Vtbl, 0xb0266d75_2081_4493_af7f_028ba34db891);
windows_core::imp::interface_hierarchy!(ICorProfilerModuleEnum, windows_core::IUnknown);
impl ICorProfilerModuleEnum {
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<ICorProfilerModuleEnum> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Next(&self, ids: &mut [usize], pceltfetched: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ids.len().try_into().unwrap(), core::mem::transmute(ids.as_ptr()), pceltfetched as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorProfilerModuleEnum_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut usize, *mut u32) -> windows_core::HRESULT,
}
pub trait ICorProfilerModuleEnum_Impl: windows_core::IUnknownImpl {
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<ICorProfilerModuleEnum>;
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn Next(&self, celt: u32, ids: *mut usize, pceltfetched: *mut u32) -> windows_core::Result<()>;
}
impl ICorProfilerModuleEnum_Vtbl {
    pub const fn new<Identity: ICorProfilerModuleEnum_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Skip<Identity: ICorProfilerModuleEnum_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerModuleEnum_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: ICorProfilerModuleEnum_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerModuleEnum_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: ICorProfilerModuleEnum_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorProfilerModuleEnum_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCount<Identity: ICorProfilerModuleEnum_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcelt: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorProfilerModuleEnum_Impl::GetCount(this) {
                    Ok(ok__) => {
                        pcelt.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Next<Identity: ICorProfilerModuleEnum_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ids: *mut usize, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerModuleEnum_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ids), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorProfilerModuleEnum as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICorProfilerModuleEnum {}
windows_core::imp::define_interface!(ICorProfilerObjectEnum, ICorProfilerObjectEnum_Vtbl, 0x2c6269bd_2d13_4321_ae12_6686365fd6af);
windows_core::imp::interface_hierarchy!(ICorProfilerObjectEnum, windows_core::IUnknown);
impl ICorProfilerObjectEnum {
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<ICorProfilerObjectEnum> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Next(&self, objects: &mut [usize], pceltfetched: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), objects.len().try_into().unwrap(), core::mem::transmute(objects.as_ptr()), pceltfetched as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorProfilerObjectEnum_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut usize, *mut u32) -> windows_core::HRESULT,
}
pub trait ICorProfilerObjectEnum_Impl: windows_core::IUnknownImpl {
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<ICorProfilerObjectEnum>;
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn Next(&self, celt: u32, objects: *mut usize, pceltfetched: *mut u32) -> windows_core::Result<()>;
}
impl ICorProfilerObjectEnum_Vtbl {
    pub const fn new<Identity: ICorProfilerObjectEnum_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Skip<Identity: ICorProfilerObjectEnum_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerObjectEnum_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: ICorProfilerObjectEnum_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerObjectEnum_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: ICorProfilerObjectEnum_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorProfilerObjectEnum_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCount<Identity: ICorProfilerObjectEnum_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcelt: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorProfilerObjectEnum_Impl::GetCount(this) {
                    Ok(ok__) => {
                        pcelt.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Next<Identity: ICorProfilerObjectEnum_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, objects: *mut usize, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerObjectEnum_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&objects), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorProfilerObjectEnum as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICorProfilerObjectEnum {}
windows_core::imp::define_interface!(ICorProfilerThreadEnum, ICorProfilerThreadEnum_Vtbl, 0x571194f7_25ed_419f_aa8b_7016b3159701);
windows_core::imp::interface_hierarchy!(ICorProfilerThreadEnum, windows_core::IUnknown);
impl ICorProfilerThreadEnum {
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<ICorProfilerThreadEnum> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Next(&self, ids: &mut [usize], pceltfetched: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ids.len().try_into().unwrap(), core::mem::transmute(ids.as_ptr()), pceltfetched as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorProfilerThreadEnum_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut usize, *mut u32) -> windows_core::HRESULT,
}
pub trait ICorProfilerThreadEnum_Impl: windows_core::IUnknownImpl {
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<ICorProfilerThreadEnum>;
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn Next(&self, celt: u32, ids: *mut usize, pceltfetched: *mut u32) -> windows_core::Result<()>;
}
impl ICorProfilerThreadEnum_Vtbl {
    pub const fn new<Identity: ICorProfilerThreadEnum_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Skip<Identity: ICorProfilerThreadEnum_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerThreadEnum_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: ICorProfilerThreadEnum_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerThreadEnum_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: ICorProfilerThreadEnum_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorProfilerThreadEnum_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCount<Identity: ICorProfilerThreadEnum_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcelt: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorProfilerThreadEnum_Impl::GetCount(this) {
                    Ok(ok__) => {
                        pcelt.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Next<Identity: ICorProfilerThreadEnum_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ids: *mut usize, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorProfilerThreadEnum_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ids), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorProfilerThreadEnum as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICorProfilerThreadEnum {}
windows_core::imp::define_interface!(IMethodMalloc, IMethodMalloc_Vtbl, 0xa0efb28b_6ee2_4d7b_b983_a75ef7beedb8);
windows_core::imp::interface_hierarchy!(IMethodMalloc, windows_core::IUnknown);
impl IMethodMalloc {
    pub unsafe fn Alloc(&self, cb: u32) -> *mut core::ffi::c_void {
        unsafe { (windows_core::Interface::vtable(self).Alloc)(windows_core::Interface::as_raw(self), cb) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMethodMalloc_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Alloc: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> *mut core::ffi::c_void,
}
pub trait IMethodMalloc_Impl: windows_core::IUnknownImpl {
    fn Alloc(&self, cb: u32) -> *mut core::ffi::c_void;
}
impl IMethodMalloc_Vtbl {
    pub const fn new<Identity: IMethodMalloc_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Alloc<Identity: IMethodMalloc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cb: u32) -> *mut core::ffi::c_void {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMethodMalloc_Impl::Alloc(this, core::mem::transmute_copy(&cb))
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Alloc: Alloc::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMethodMalloc as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMethodMalloc {}
pub const NO_MAPPING: CorDebugIlToNativeMappingTypes = CorDebugIlToNativeMappingTypes(-1i32);
pub type ObjectReferenceCallback = Option<unsafe extern "system" fn(root: usize, reference: *mut usize, clientdata: *mut core::ffi::c_void) -> windows_core::BOOL>;
pub const PROFILER_GLOBAL_CLASS: COR_PRF_MISC = COR_PRF_MISC(-2i32);
pub const PROFILER_GLOBAL_MODULE: COR_PRF_MISC = COR_PRF_MISC(-1i32);
pub const PROFILER_PARENT_UNKNOWN: COR_PRF_MISC = COR_PRF_MISC(-3i32);
pub const PROLOG: CorDebugIlToNativeMappingTypes = CorDebugIlToNativeMappingTypes(-2i32);
pub type StackSnapshotCallback = Option<unsafe extern "system" fn(funcid: usize, ip: usize, frameinfo: usize, contextsize: u32, context: *mut u8, clientdata: *mut core::ffi::c_void) -> windows_core::HRESULT>;
