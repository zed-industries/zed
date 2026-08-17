#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "verifier.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationVerifier\"`, `\"Win32_Foundation\"`*"] fn VerifierEnumerateResource ( process : super::super::Foundation:: HANDLE , flags : VERIFIER_ENUM_RESOURCE_FLAGS , resourcetype : eAvrfResourceTypes , resourcecallback : AVRF_RESOURCE_ENUMERATE_CALLBACK , enumerationcontext : *mut ::core::ffi::c_void ) -> u32 );
#[doc = "*Required features: `\"Win32_System_ApplicationVerifier\"`*"]
pub const AVRF_MAX_TRACES: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_ApplicationVerifier\"`*"]
pub type VERIFIER_ENUM_RESOURCE_FLAGS = u32;
#[doc = "*Required features: `\"Win32_System_ApplicationVerifier\"`*"]
pub const AVRF_ENUM_RESOURCES_FLAGS_DONT_RESOLVE_TRACES: VERIFIER_ENUM_RESOURCE_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_System_ApplicationVerifier\"`*"]
pub const AVRF_ENUM_RESOURCES_FLAGS_SUSPEND: VERIFIER_ENUM_RESOURCE_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_System_ApplicationVerifier\"`*"]
pub type eAvrfResourceTypes = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationVerifier\"`*"]
pub const AvrfResourceHeapAllocation: eAvrfResourceTypes = 0i32;
#[doc = "*Required features: `\"Win32_System_ApplicationVerifier\"`*"]
pub const AvrfResourceHandleTrace: eAvrfResourceTypes = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationVerifier\"`*"]
pub const AvrfResourceMax: eAvrfResourceTypes = 2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationVerifier\"`*"]
pub type eHANDLE_TRACE_OPERATIONS = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationVerifier\"`*"]
pub const OperationDbUnused: eHANDLE_TRACE_OPERATIONS = 0i32;
#[doc = "*Required features: `\"Win32_System_ApplicationVerifier\"`*"]
pub const OperationDbOPEN: eHANDLE_TRACE_OPERATIONS = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationVerifier\"`*"]
pub const OperationDbCLOSE: eHANDLE_TRACE_OPERATIONS = 2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationVerifier\"`*"]
pub const OperationDbBADREF: eHANDLE_TRACE_OPERATIONS = 3i32;
#[doc = "*Required features: `\"Win32_System_ApplicationVerifier\"`*"]
pub type eHeapAllocationState = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationVerifier\"`*"]
pub const HeapFullPageHeap: eHeapAllocationState = 1073741824i32;
#[doc = "*Required features: `\"Win32_System_ApplicationVerifier\"`*"]
pub const HeapMetadata: eHeapAllocationState = -2147483648i32;
#[doc = "*Required features: `\"Win32_System_ApplicationVerifier\"`*"]
pub const HeapStateMask: eHeapAllocationState = -65536i32;
#[doc = "*Required features: `\"Win32_System_ApplicationVerifier\"`*"]
pub type eHeapEnumerationLevel = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationVerifier\"`*"]
pub const HeapEnumerationEverything: eHeapEnumerationLevel = 0i32;
#[doc = "*Required features: `\"Win32_System_ApplicationVerifier\"`*"]
pub const HeapEnumerationStop: eHeapEnumerationLevel = -1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationVerifier\"`*"]
pub type eUserAllocationState = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationVerifier\"`*"]
pub const AllocationStateUnknown: eUserAllocationState = 0i32;
#[doc = "*Required features: `\"Win32_System_ApplicationVerifier\"`*"]
pub const AllocationStateBusy: eUserAllocationState = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationVerifier\"`*"]
pub const AllocationStateFree: eUserAllocationState = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationVerifier\"`*"]
pub struct AVRF_BACKTRACE_INFORMATION {
    pub Depth: u32,
    pub Index: u32,
    pub ReturnAddresses: [u64; 32],
}
impl ::core::marker::Copy for AVRF_BACKTRACE_INFORMATION {}
impl ::core::clone::Clone for AVRF_BACKTRACE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationVerifier\"`*"]
pub struct AVRF_HANDLE_OPERATION {
    pub Handle: u64,
    pub ProcessId: u32,
    pub ThreadId: u32,
    pub OperationType: u32,
    pub Spare0: u32,
    pub BackTraceInformation: AVRF_BACKTRACE_INFORMATION,
}
impl ::core::marker::Copy for AVRF_HANDLE_OPERATION {}
impl ::core::clone::Clone for AVRF_HANDLE_OPERATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationVerifier\"`*"]
pub struct AVRF_HEAP_ALLOCATION {
    pub HeapHandle: u64,
    pub UserAllocation: u64,
    pub UserAllocationSize: u64,
    pub Allocation: u64,
    pub AllocationSize: u64,
    pub UserAllocationState: u32,
    pub HeapState: u32,
    pub HeapContext: u64,
    pub BackTraceInformation: *mut AVRF_BACKTRACE_INFORMATION,
}
impl ::core::marker::Copy for AVRF_HEAP_ALLOCATION {}
impl ::core::clone::Clone for AVRF_HEAP_ALLOCATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationVerifier\"`*"]
pub type AVRF_HANDLEOPERATION_ENUMERATE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(handleoperation: *mut AVRF_HANDLE_OPERATION, enumerationcontext: *mut ::core::ffi::c_void, enumerationlevel: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_System_ApplicationVerifier\"`*"]
pub type AVRF_HEAPALLOCATION_ENUMERATE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(heapallocation: *mut AVRF_HEAP_ALLOCATION, enumerationcontext: *mut ::core::ffi::c_void, enumerationlevel: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_System_ApplicationVerifier\"`*"]
pub type AVRF_RESOURCE_ENUMERATE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(resourcedescription: *mut ::core::ffi::c_void, enumerationcontext: *mut ::core::ffi::c_void, enumerationlevel: *mut u32) -> u32>;
