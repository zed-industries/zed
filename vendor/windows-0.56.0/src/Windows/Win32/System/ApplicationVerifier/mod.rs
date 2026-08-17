#[inline]
pub unsafe fn VerifierEnumerateResource<P0>(process: P0, flags: VERIFIER_ENUM_RESOURCE_FLAGS, resourcetype: eAvrfResourceTypes, resourcecallback: AVRF_RESOURCE_ENUMERATE_CALLBACK, enumerationcontext: *mut core::ffi::c_void) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("verifier.dll" "system" fn VerifierEnumerateResource(process : super::super::Foundation:: HANDLE, flags : VERIFIER_ENUM_RESOURCE_FLAGS, resourcetype : u32, resourcecallback : AVRF_RESOURCE_ENUMERATE_CALLBACK, enumerationcontext : *mut core::ffi::c_void) -> u32);
    VerifierEnumerateResource(process.param().abi(), flags, resourcetype.0 as _, resourcecallback, enumerationcontext)
}
pub const AVRF_ENUM_RESOURCES_FLAGS_DONT_RESOLVE_TRACES: VERIFIER_ENUM_RESOURCE_FLAGS = VERIFIER_ENUM_RESOURCE_FLAGS(2u32);
pub const AVRF_ENUM_RESOURCES_FLAGS_SUSPEND: VERIFIER_ENUM_RESOURCE_FLAGS = VERIFIER_ENUM_RESOURCE_FLAGS(1u32);
pub const AVRF_MAX_TRACES: u32 = 32u32;
pub const AllocationStateBusy: eUserAllocationState = eUserAllocationState(1i32);
pub const AllocationStateFree: eUserAllocationState = eUserAllocationState(2i32);
pub const AllocationStateUnknown: eUserAllocationState = eUserAllocationState(0i32);
pub const AvrfResourceHandleTrace: eAvrfResourceTypes = eAvrfResourceTypes(1i32);
pub const AvrfResourceHeapAllocation: eAvrfResourceTypes = eAvrfResourceTypes(0i32);
pub const AvrfResourceMax: eAvrfResourceTypes = eAvrfResourceTypes(2i32);
pub const HeapEnumerationEverything: eHeapEnumerationLevel = eHeapEnumerationLevel(0i32);
pub const HeapEnumerationStop: eHeapEnumerationLevel = eHeapEnumerationLevel(-1i32);
pub const HeapFullPageHeap: eHeapAllocationState = eHeapAllocationState(1073741824i32);
pub const HeapMetadata: eHeapAllocationState = eHeapAllocationState(-2147483648i32);
pub const HeapStateMask: eHeapAllocationState = eHeapAllocationState(-65536i32);
pub const OperationDbBADREF: eHANDLE_TRACE_OPERATIONS = eHANDLE_TRACE_OPERATIONS(3i32);
pub const OperationDbCLOSE: eHANDLE_TRACE_OPERATIONS = eHANDLE_TRACE_OPERATIONS(2i32);
pub const OperationDbOPEN: eHANDLE_TRACE_OPERATIONS = eHANDLE_TRACE_OPERATIONS(1i32);
pub const OperationDbUnused: eHANDLE_TRACE_OPERATIONS = eHANDLE_TRACE_OPERATIONS(0i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VERIFIER_ENUM_RESOURCE_FLAGS(pub u32);
impl windows_core::TypeKind for VERIFIER_ENUM_RESOURCE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VERIFIER_ENUM_RESOURCE_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VERIFIER_ENUM_RESOURCE_FLAGS").field(&self.0).finish()
    }
}
impl VERIFIER_ENUM_RESOURCE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for VERIFIER_ENUM_RESOURCE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for VERIFIER_ENUM_RESOURCE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for VERIFIER_ENUM_RESOURCE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for VERIFIER_ENUM_RESOURCE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for VERIFIER_ENUM_RESOURCE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct eAvrfResourceTypes(pub i32);
impl windows_core::TypeKind for eAvrfResourceTypes {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for eAvrfResourceTypes {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("eAvrfResourceTypes").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct eHANDLE_TRACE_OPERATIONS(pub i32);
impl windows_core::TypeKind for eHANDLE_TRACE_OPERATIONS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for eHANDLE_TRACE_OPERATIONS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("eHANDLE_TRACE_OPERATIONS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct eHeapAllocationState(pub i32);
impl windows_core::TypeKind for eHeapAllocationState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for eHeapAllocationState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("eHeapAllocationState").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct eHeapEnumerationLevel(pub i32);
impl windows_core::TypeKind for eHeapEnumerationLevel {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for eHeapEnumerationLevel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("eHeapEnumerationLevel").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct eUserAllocationState(pub i32);
impl windows_core::TypeKind for eUserAllocationState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for eUserAllocationState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("eUserAllocationState").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct AVRF_BACKTRACE_INFORMATION {
    pub Depth: u32,
    pub Index: u32,
    pub ReturnAddresses: [u64; 32],
}
impl Copy for AVRF_BACKTRACE_INFORMATION {}
impl Clone for AVRF_BACKTRACE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for AVRF_BACKTRACE_INFORMATION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("AVRF_BACKTRACE_INFORMATION").field("Depth", &self.Depth).field("Index", &self.Index).field("ReturnAddresses", &self.ReturnAddresses).finish()
    }
}
impl windows_core::TypeKind for AVRF_BACKTRACE_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for AVRF_BACKTRACE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Depth == other.Depth && self.Index == other.Index && self.ReturnAddresses == other.ReturnAddresses
    }
}
impl Eq for AVRF_BACKTRACE_INFORMATION {}
impl Default for AVRF_BACKTRACE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct AVRF_HANDLE_OPERATION {
    pub Handle: u64,
    pub ProcessId: u32,
    pub ThreadId: u32,
    pub OperationType: u32,
    pub Spare0: u32,
    pub BackTraceInformation: AVRF_BACKTRACE_INFORMATION,
}
impl Copy for AVRF_HANDLE_OPERATION {}
impl Clone for AVRF_HANDLE_OPERATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for AVRF_HANDLE_OPERATION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("AVRF_HANDLE_OPERATION").field("Handle", &self.Handle).field("ProcessId", &self.ProcessId).field("ThreadId", &self.ThreadId).field("OperationType", &self.OperationType).field("Spare0", &self.Spare0).field("BackTraceInformation", &self.BackTraceInformation).finish()
    }
}
impl windows_core::TypeKind for AVRF_HANDLE_OPERATION {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for AVRF_HANDLE_OPERATION {
    fn eq(&self, other: &Self) -> bool {
        self.Handle == other.Handle && self.ProcessId == other.ProcessId && self.ThreadId == other.ThreadId && self.OperationType == other.OperationType && self.Spare0 == other.Spare0 && self.BackTraceInformation == other.BackTraceInformation
    }
}
impl Eq for AVRF_HANDLE_OPERATION {}
impl Default for AVRF_HANDLE_OPERATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
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
impl Copy for AVRF_HEAP_ALLOCATION {}
impl Clone for AVRF_HEAP_ALLOCATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for AVRF_HEAP_ALLOCATION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("AVRF_HEAP_ALLOCATION").field("HeapHandle", &self.HeapHandle).field("UserAllocation", &self.UserAllocation).field("UserAllocationSize", &self.UserAllocationSize).field("Allocation", &self.Allocation).field("AllocationSize", &self.AllocationSize).field("UserAllocationState", &self.UserAllocationState).field("HeapState", &self.HeapState).field("HeapContext", &self.HeapContext).field("BackTraceInformation", &self.BackTraceInformation).finish()
    }
}
impl windows_core::TypeKind for AVRF_HEAP_ALLOCATION {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for AVRF_HEAP_ALLOCATION {
    fn eq(&self, other: &Self) -> bool {
        self.HeapHandle == other.HeapHandle && self.UserAllocation == other.UserAllocation && self.UserAllocationSize == other.UserAllocationSize && self.Allocation == other.Allocation && self.AllocationSize == other.AllocationSize && self.UserAllocationState == other.UserAllocationState && self.HeapState == other.HeapState && self.HeapContext == other.HeapContext && self.BackTraceInformation == other.BackTraceInformation
    }
}
impl Eq for AVRF_HEAP_ALLOCATION {}
impl Default for AVRF_HEAP_ALLOCATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type AVRF_HANDLEOPERATION_ENUMERATE_CALLBACK = Option<unsafe extern "system" fn(handleoperation: *mut AVRF_HANDLE_OPERATION, enumerationcontext: *mut core::ffi::c_void, enumerationlevel: *mut u32) -> u32>;
pub type AVRF_HEAPALLOCATION_ENUMERATE_CALLBACK = Option<unsafe extern "system" fn(heapallocation: *mut AVRF_HEAP_ALLOCATION, enumerationcontext: *mut core::ffi::c_void, enumerationlevel: *mut u32) -> u32>;
pub type AVRF_RESOURCE_ENUMERATE_CALLBACK = Option<unsafe extern "system" fn(resourcedescription: *mut core::ffi::c_void, enumerationcontext: *mut core::ffi::c_void, enumerationlevel: *mut u32) -> u32>;
