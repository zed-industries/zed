pub type ICorProfilerAssemblyReferenceProvider = *mut ::core::ffi::c_void;
pub type ICorProfilerCallback = *mut ::core::ffi::c_void;
pub type ICorProfilerCallback2 = *mut ::core::ffi::c_void;
pub type ICorProfilerCallback3 = *mut ::core::ffi::c_void;
pub type ICorProfilerCallback4 = *mut ::core::ffi::c_void;
pub type ICorProfilerCallback5 = *mut ::core::ffi::c_void;
pub type ICorProfilerCallback6 = *mut ::core::ffi::c_void;
pub type ICorProfilerCallback7 = *mut ::core::ffi::c_void;
pub type ICorProfilerCallback8 = *mut ::core::ffi::c_void;
pub type ICorProfilerCallback9 = *mut ::core::ffi::c_void;
pub type ICorProfilerFunctionControl = *mut ::core::ffi::c_void;
pub type ICorProfilerFunctionEnum = *mut ::core::ffi::c_void;
pub type ICorProfilerInfo = *mut ::core::ffi::c_void;
pub type ICorProfilerInfo2 = *mut ::core::ffi::c_void;
pub type ICorProfilerInfo3 = *mut ::core::ffi::c_void;
pub type ICorProfilerInfo4 = *mut ::core::ffi::c_void;
pub type ICorProfilerInfo5 = *mut ::core::ffi::c_void;
pub type ICorProfilerInfo6 = *mut ::core::ffi::c_void;
pub type ICorProfilerInfo7 = *mut ::core::ffi::c_void;
pub type ICorProfilerInfo8 = *mut ::core::ffi::c_void;
pub type ICorProfilerMethodEnum = *mut ::core::ffi::c_void;
pub type ICorProfilerModuleEnum = *mut ::core::ffi::c_void;
pub type ICorProfilerObjectEnum = *mut ::core::ffi::c_void;
pub type ICorProfilerThreadEnum = *mut ::core::ffi::c_void;
pub type IMethodMalloc = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const CorDB_CONTROL_Profiling: ::windows_sys::core::PCSTR = ::windows_sys::core::s!("Cor_Enable_Profiling");
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const CorDB_CONTROL_ProfilingL: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Cor_Enable_Profiling");
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub type COR_PRF_CLAUSE_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_CLAUSE_NONE: COR_PRF_CLAUSE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_CLAUSE_FILTER: COR_PRF_CLAUSE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_CLAUSE_CATCH: COR_PRF_CLAUSE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_CLAUSE_FINALLY: COR_PRF_CLAUSE_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub type COR_PRF_CODEGEN_FLAGS = i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_CODEGEN_DISABLE_INLINING: COR_PRF_CODEGEN_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_CODEGEN_DISABLE_ALL_OPTIMIZATIONS: COR_PRF_CODEGEN_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub type COR_PRF_FINALIZER_FLAGS = i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_FINALIZER_CRITICAL: COR_PRF_FINALIZER_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub type COR_PRF_GC_GENERATION = i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_GC_GEN_0: COR_PRF_GC_GENERATION = 0i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_GC_GEN_1: COR_PRF_GC_GENERATION = 1i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_GC_GEN_2: COR_PRF_GC_GENERATION = 2i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_GC_LARGE_OBJECT_HEAP: COR_PRF_GC_GENERATION = 3i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub type COR_PRF_GC_REASON = i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_GC_INDUCED: COR_PRF_GC_REASON = 1i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_GC_OTHER: COR_PRF_GC_REASON = 0i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub type COR_PRF_GC_ROOT_FLAGS = i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_GC_ROOT_PINNING: COR_PRF_GC_ROOT_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_GC_ROOT_WEAKREF: COR_PRF_GC_ROOT_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_GC_ROOT_INTERIOR: COR_PRF_GC_ROOT_FLAGS = 4i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_GC_ROOT_REFCOUNTED: COR_PRF_GC_ROOT_FLAGS = 8i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub type COR_PRF_GC_ROOT_KIND = i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_GC_ROOT_STACK: COR_PRF_GC_ROOT_KIND = 1i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_GC_ROOT_FINALIZER: COR_PRF_GC_ROOT_KIND = 2i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_GC_ROOT_HANDLE: COR_PRF_GC_ROOT_KIND = 3i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_GC_ROOT_OTHER: COR_PRF_GC_ROOT_KIND = 0i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub type COR_PRF_HIGH_MONITOR = i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_HIGH_MONITOR_NONE: COR_PRF_HIGH_MONITOR = 0i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_HIGH_ADD_ASSEMBLY_REFERENCES: COR_PRF_HIGH_MONITOR = 1i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_HIGH_IN_MEMORY_SYMBOLS_UPDATED: COR_PRF_HIGH_MONITOR = 2i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_HIGH_MONITOR_DYNAMIC_FUNCTION_UNLOADS: COR_PRF_HIGH_MONITOR = 4i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_HIGH_REQUIRE_PROFILE_IMAGE: COR_PRF_HIGH_MONITOR = 0i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_HIGH_ALLOWABLE_AFTER_ATTACH: COR_PRF_HIGH_MONITOR = 6i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_HIGH_MONITOR_IMMUTABLE: COR_PRF_HIGH_MONITOR = 0i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub type COR_PRF_JIT_CACHE = i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_CACHED_FUNCTION_FOUND: COR_PRF_JIT_CACHE = 0i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_CACHED_FUNCTION_NOT_FOUND: COR_PRF_JIT_CACHE = 1i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub type COR_PRF_MISC = i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const PROFILER_PARENT_UNKNOWN: COR_PRF_MISC = -3i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const PROFILER_GLOBAL_CLASS: COR_PRF_MISC = -2i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const PROFILER_GLOBAL_MODULE: COR_PRF_MISC = -1i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub type COR_PRF_MODULE_FLAGS = i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_MODULE_DISK: COR_PRF_MODULE_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_MODULE_NGEN: COR_PRF_MODULE_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_MODULE_DYNAMIC: COR_PRF_MODULE_FLAGS = 4i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_MODULE_COLLECTIBLE: COR_PRF_MODULE_FLAGS = 8i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_MODULE_RESOURCE: COR_PRF_MODULE_FLAGS = 16i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_MODULE_FLAT_LAYOUT: COR_PRF_MODULE_FLAGS = 32i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_MODULE_WINDOWS_RUNTIME: COR_PRF_MODULE_FLAGS = 64i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub type COR_PRF_MONITOR = i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_MONITOR_NONE: COR_PRF_MONITOR = 0i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_MONITOR_FUNCTION_UNLOADS: COR_PRF_MONITOR = 1i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_MONITOR_CLASS_LOADS: COR_PRF_MONITOR = 2i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_MONITOR_MODULE_LOADS: COR_PRF_MONITOR = 4i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_MONITOR_ASSEMBLY_LOADS: COR_PRF_MONITOR = 8i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_MONITOR_APPDOMAIN_LOADS: COR_PRF_MONITOR = 16i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_MONITOR_JIT_COMPILATION: COR_PRF_MONITOR = 32i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_MONITOR_EXCEPTIONS: COR_PRF_MONITOR = 64i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_MONITOR_GC: COR_PRF_MONITOR = 128i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_MONITOR_OBJECT_ALLOCATED: COR_PRF_MONITOR = 256i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_MONITOR_THREADS: COR_PRF_MONITOR = 512i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_MONITOR_REMOTING: COR_PRF_MONITOR = 1024i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_MONITOR_CODE_TRANSITIONS: COR_PRF_MONITOR = 2048i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_MONITOR_ENTERLEAVE: COR_PRF_MONITOR = 4096i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_MONITOR_CCW: COR_PRF_MONITOR = 8192i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_MONITOR_REMOTING_COOKIE: COR_PRF_MONITOR = 17408i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_MONITOR_REMOTING_ASYNC: COR_PRF_MONITOR = 33792i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_MONITOR_SUSPENDS: COR_PRF_MONITOR = 65536i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_MONITOR_CACHE_SEARCHES: COR_PRF_MONITOR = 131072i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_ENABLE_REJIT: COR_PRF_MONITOR = 262144i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_ENABLE_INPROC_DEBUGGING: COR_PRF_MONITOR = 524288i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_ENABLE_JIT_MAPS: COR_PRF_MONITOR = 1048576i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_DISABLE_INLINING: COR_PRF_MONITOR = 2097152i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_DISABLE_OPTIMIZATIONS: COR_PRF_MONITOR = 4194304i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_ENABLE_OBJECT_ALLOCATED: COR_PRF_MONITOR = 8388608i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_MONITOR_CLR_EXCEPTIONS: COR_PRF_MONITOR = 16777216i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_MONITOR_ALL: COR_PRF_MONITOR = 17301503i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_ENABLE_FUNCTION_ARGS: COR_PRF_MONITOR = 33554432i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_ENABLE_FUNCTION_RETVAL: COR_PRF_MONITOR = 67108864i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_ENABLE_FRAME_INFO: COR_PRF_MONITOR = 134217728i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_ENABLE_STACK_SNAPSHOT: COR_PRF_MONITOR = 268435456i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_USE_PROFILE_IMAGES: COR_PRF_MONITOR = 536870912i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_DISABLE_TRANSPARENCY_CHECKS_UNDER_FULL_TRUST: COR_PRF_MONITOR = 1073741824i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_DISABLE_ALL_NGEN_IMAGES: COR_PRF_MONITOR = -2147483648i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_ALL: COR_PRF_MONITOR = -1879048193i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_REQUIRE_PROFILE_IMAGE: COR_PRF_MONITOR = 536877056i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_ALLOWABLE_AFTER_ATTACH: COR_PRF_MONITOR = 268501758i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_MONITOR_IMMUTABLE: COR_PRF_MONITOR = -285422592i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub type COR_PRF_RUNTIME_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_DESKTOP_CLR: COR_PRF_RUNTIME_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_CORE_CLR: COR_PRF_RUNTIME_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub type COR_PRF_SNAPSHOT_INFO = i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_SNAPSHOT_DEFAULT: COR_PRF_SNAPSHOT_INFO = 0i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_SNAPSHOT_REGISTER_CONTEXT: COR_PRF_SNAPSHOT_INFO = 1i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_SNAPSHOT_X86_OPTIMIZED: COR_PRF_SNAPSHOT_INFO = 2i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub type COR_PRF_STATIC_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_FIELD_NOT_A_STATIC: COR_PRF_STATIC_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_FIELD_APP_DOMAIN_STATIC: COR_PRF_STATIC_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_FIELD_THREAD_STATIC: COR_PRF_STATIC_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_FIELD_CONTEXT_STATIC: COR_PRF_STATIC_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_FIELD_RVA_STATIC: COR_PRF_STATIC_TYPE = 8i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub type COR_PRF_SUSPEND_REASON = i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_SUSPEND_OTHER: COR_PRF_SUSPEND_REASON = 0i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_SUSPEND_FOR_GC: COR_PRF_SUSPEND_REASON = 1i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_SUSPEND_FOR_APPDOMAIN_SHUTDOWN: COR_PRF_SUSPEND_REASON = 2i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_SUSPEND_FOR_CODE_PITCHING: COR_PRF_SUSPEND_REASON = 3i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_SUSPEND_FOR_SHUTDOWN: COR_PRF_SUSPEND_REASON = 4i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_SUSPEND_FOR_INPROC_DEBUGGER: COR_PRF_SUSPEND_REASON = 6i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_SUSPEND_FOR_GC_PREP: COR_PRF_SUSPEND_REASON = 7i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_SUSPEND_FOR_REJIT: COR_PRF_SUSPEND_REASON = 8i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub type COR_PRF_TRANSITION_REASON = i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_TRANSITION_CALL: COR_PRF_TRANSITION_REASON = 0i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_TRANSITION_RETURN: COR_PRF_TRANSITION_REASON = 1i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub type CorDebugIlToNativeMappingTypes = i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const NO_MAPPING: CorDebugIlToNativeMappingTypes = -1i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const PROLOG: CorDebugIlToNativeMappingTypes = -2i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const EPILOG: CorDebugIlToNativeMappingTypes = -3i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub struct COR_DEBUG_IL_TO_NATIVE_MAP {
    pub ilOffset: u32,
    pub nativeStartOffset: u32,
    pub nativeEndOffset: u32,
}
impl ::core::marker::Copy for COR_DEBUG_IL_TO_NATIVE_MAP {}
impl ::core::clone::Clone for COR_DEBUG_IL_TO_NATIVE_MAP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct COR_IL_MAP {
    pub oldOffset: u32,
    pub newOffset: u32,
    pub fAccurate: super::super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for COR_IL_MAP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for COR_IL_MAP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`, `\"Win32_System_WinRT_Metadata\"`*"]
#[cfg(feature = "Win32_System_WinRT_Metadata")]
pub struct COR_PRF_ASSEMBLY_REFERENCE_INFO {
    pub pbPublicKeyOrToken: *mut ::core::ffi::c_void,
    pub cbPublicKeyOrToken: u32,
    pub szName: ::windows_sys::core::PCWSTR,
    pub pMetaData: *mut super::super::WinRT::Metadata::ASSEMBLYMETADATA,
    pub pbHashValue: *mut ::core::ffi::c_void,
    pub cbHashValue: u32,
    pub dwAssemblyRefFlags: u32,
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl ::core::marker::Copy for COR_PRF_ASSEMBLY_REFERENCE_INFO {}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl ::core::clone::Clone for COR_PRF_ASSEMBLY_REFERENCE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub struct COR_PRF_CODE_INFO {
    pub startAddress: usize,
    pub size: usize,
}
impl ::core::marker::Copy for COR_PRF_CODE_INFO {}
impl ::core::clone::Clone for COR_PRF_CODE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub struct COR_PRF_EX_CLAUSE_INFO {
    pub clauseType: COR_PRF_CLAUSE_TYPE,
    pub programCounter: usize,
    pub framePointer: usize,
    pub shadowStackPointer: usize,
}
impl ::core::marker::Copy for COR_PRF_EX_CLAUSE_INFO {}
impl ::core::clone::Clone for COR_PRF_EX_CLAUSE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub struct COR_PRF_FUNCTION {
    pub functionId: usize,
    pub reJitId: usize,
}
impl ::core::marker::Copy for COR_PRF_FUNCTION {}
impl ::core::clone::Clone for COR_PRF_FUNCTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub struct COR_PRF_FUNCTION_ARGUMENT_INFO {
    pub numRanges: u32,
    pub totalArgumentSize: u32,
    pub ranges: [COR_PRF_FUNCTION_ARGUMENT_RANGE; 1],
}
impl ::core::marker::Copy for COR_PRF_FUNCTION_ARGUMENT_INFO {}
impl ::core::clone::Clone for COR_PRF_FUNCTION_ARGUMENT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub struct COR_PRF_FUNCTION_ARGUMENT_RANGE {
    pub startAddress: usize,
    pub length: u32,
}
impl ::core::marker::Copy for COR_PRF_FUNCTION_ARGUMENT_RANGE {}
impl ::core::clone::Clone for COR_PRF_FUNCTION_ARGUMENT_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub struct COR_PRF_GC_GENERATION_RANGE {
    pub generation: COR_PRF_GC_GENERATION,
    pub rangeStart: usize,
    pub rangeLength: usize,
    pub rangeLengthReserved: usize,
}
impl ::core::marker::Copy for COR_PRF_GC_GENERATION_RANGE {}
impl ::core::clone::Clone for COR_PRF_GC_GENERATION_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub struct COR_PRF_METHOD {
    pub moduleId: usize,
    pub methodId: u32,
}
impl ::core::marker::Copy for COR_PRF_METHOD {}
impl ::core::clone::Clone for COR_PRF_METHOD {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub union FunctionIDOrClientID {
    pub functionID: usize,
    pub clientID: usize,
}
impl ::core::marker::Copy for FunctionIDOrClientID {}
impl ::core::clone::Clone for FunctionIDOrClientID {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub type FunctionEnter = ::core::option::Option<unsafe extern "system" fn(funcid: usize) -> ()>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub type FunctionEnter2 = ::core::option::Option<unsafe extern "system" fn(funcid: usize, clientdata: usize, func: usize, argumentinfo: *mut COR_PRF_FUNCTION_ARGUMENT_INFO) -> ()>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub type FunctionEnter3 = ::core::option::Option<unsafe extern "system" fn(functionidorclientid: FunctionIDOrClientID) -> ()>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub type FunctionEnter3WithInfo = ::core::option::Option<unsafe extern "system" fn(functionidorclientid: FunctionIDOrClientID, eltinfo: usize) -> ()>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type FunctionIDMapper = ::core::option::Option<unsafe extern "system" fn(funcid: usize, pbhookfunction: *mut super::super::super::Foundation::BOOL) -> usize>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type FunctionIDMapper2 = ::core::option::Option<unsafe extern "system" fn(funcid: usize, clientdata: *mut ::core::ffi::c_void, pbhookfunction: *mut super::super::super::Foundation::BOOL) -> usize>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub type FunctionLeave = ::core::option::Option<unsafe extern "system" fn(funcid: usize) -> ()>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub type FunctionLeave2 = ::core::option::Option<unsafe extern "system" fn(funcid: usize, clientdata: usize, func: usize, retvalrange: *mut COR_PRF_FUNCTION_ARGUMENT_RANGE) -> ()>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub type FunctionLeave3 = ::core::option::Option<unsafe extern "system" fn(functionidorclientid: FunctionIDOrClientID) -> ()>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub type FunctionLeave3WithInfo = ::core::option::Option<unsafe extern "system" fn(functionidorclientid: FunctionIDOrClientID, eltinfo: usize) -> ()>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub type FunctionTailcall = ::core::option::Option<unsafe extern "system" fn(funcid: usize) -> ()>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub type FunctionTailcall2 = ::core::option::Option<unsafe extern "system" fn(funcid: usize, clientdata: usize, func: usize) -> ()>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub type FunctionTailcall3 = ::core::option::Option<unsafe extern "system" fn(functionidorclientid: FunctionIDOrClientID) -> ()>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub type FunctionTailcall3WithInfo = ::core::option::Option<unsafe extern "system" fn(functionidorclientid: FunctionIDOrClientID, eltinfo: usize) -> ()>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub type StackSnapshotCallback = ::core::option::Option<unsafe extern "system" fn(funcid: usize, ip: usize, frameinfo: usize, contextsize: u32, context: *mut u8, clientdata: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT>;
