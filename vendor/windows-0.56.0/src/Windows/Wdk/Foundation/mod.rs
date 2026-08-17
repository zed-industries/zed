#[inline]
pub unsafe fn NtClose<P0>(handle: P0) -> super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtClose(handle : super::super::Win32::Foundation:: HANDLE) -> super::super::Win32::Foundation:: NTSTATUS);
    NtClose(handle.param().abi())
}
#[inline]
pub unsafe fn NtQueryObject<P0>(handle: P0, objectinformationclass: OBJECT_INFORMATION_CLASS, objectinformation: Option<*mut core::ffi::c_void>, objectinformationlength: u32, returnlength: Option<*mut u32>) -> super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtQueryObject(handle : super::super::Win32::Foundation:: HANDLE, objectinformationclass : OBJECT_INFORMATION_CLASS, objectinformation : *mut core::ffi::c_void, objectinformationlength : u32, returnlength : *mut u32) -> super::super::Win32::Foundation:: NTSTATUS);
    NtQueryObject(handle.param().abi(), objectinformationclass, core::mem::transmute(objectinformation.unwrap_or(std::ptr::null_mut())), objectinformationlength, core::mem::transmute(returnlength.unwrap_or(std::ptr::null_mut())))
}
pub const DontUseThisType: POOL_TYPE = POOL_TYPE(3i32);
pub const DontUseThisTypeSession: POOL_TYPE = POOL_TYPE(35i32);
pub const IoPriorityCritical: IO_PRIORITY_HINT = IO_PRIORITY_HINT(4i32);
pub const IoPriorityHigh: IO_PRIORITY_HINT = IO_PRIORITY_HINT(3i32);
pub const IoPriorityLow: IO_PRIORITY_HINT = IO_PRIORITY_HINT(1i32);
pub const IoPriorityNormal: IO_PRIORITY_HINT = IO_PRIORITY_HINT(2i32);
pub const IoPriorityVeryLow: IO_PRIORITY_HINT = IO_PRIORITY_HINT(0i32);
pub const LockQueueAfdWorkQueueLock: KSPIN_LOCK_QUEUE_NUMBER = KSPIN_LOCK_QUEUE_NUMBER(13i32);
pub const LockQueueBcbLock: KSPIN_LOCK_QUEUE_NUMBER = KSPIN_LOCK_QUEUE_NUMBER(14i32);
pub const LockQueueIoCancelLock: KSPIN_LOCK_QUEUE_NUMBER = KSPIN_LOCK_QUEUE_NUMBER(7i32);
pub const LockQueueIoCompletionLock: KSPIN_LOCK_QUEUE_NUMBER = KSPIN_LOCK_QUEUE_NUMBER(11i32);
pub const LockQueueIoDatabaseLock: KSPIN_LOCK_QUEUE_NUMBER = KSPIN_LOCK_QUEUE_NUMBER(10i32);
pub const LockQueueIoVpbLock: KSPIN_LOCK_QUEUE_NUMBER = KSPIN_LOCK_QUEUE_NUMBER(9i32);
pub const LockQueueMasterLock: KSPIN_LOCK_QUEUE_NUMBER = KSPIN_LOCK_QUEUE_NUMBER(5i32);
pub const LockQueueMaximumLock: KSPIN_LOCK_QUEUE_NUMBER = KSPIN_LOCK_QUEUE_NUMBER(17i32);
pub const LockQueueNonPagedPoolLock: KSPIN_LOCK_QUEUE_NUMBER = KSPIN_LOCK_QUEUE_NUMBER(6i32);
pub const LockQueueNtfsStructLock: KSPIN_LOCK_QUEUE_NUMBER = KSPIN_LOCK_QUEUE_NUMBER(12i32);
pub const LockQueueUnusedSpare0: KSPIN_LOCK_QUEUE_NUMBER = KSPIN_LOCK_QUEUE_NUMBER(0i32);
pub const LockQueueUnusedSpare1: KSPIN_LOCK_QUEUE_NUMBER = KSPIN_LOCK_QUEUE_NUMBER(1i32);
pub const LockQueueUnusedSpare15: KSPIN_LOCK_QUEUE_NUMBER = KSPIN_LOCK_QUEUE_NUMBER(15i32);
pub const LockQueueUnusedSpare16: KSPIN_LOCK_QUEUE_NUMBER = KSPIN_LOCK_QUEUE_NUMBER(16i32);
pub const LockQueueUnusedSpare2: KSPIN_LOCK_QUEUE_NUMBER = KSPIN_LOCK_QUEUE_NUMBER(2i32);
pub const LockQueueUnusedSpare3: KSPIN_LOCK_QUEUE_NUMBER = KSPIN_LOCK_QUEUE_NUMBER(3i32);
pub const LockQueueUnusedSpare8: KSPIN_LOCK_QUEUE_NUMBER = KSPIN_LOCK_QUEUE_NUMBER(8i32);
pub const LockQueueVacbLock: KSPIN_LOCK_QUEUE_NUMBER = KSPIN_LOCK_QUEUE_NUMBER(4i32);
pub const MaxIoPriorityTypes: IO_PRIORITY_HINT = IO_PRIORITY_HINT(5i32);
pub const MaxPoolType: POOL_TYPE = POOL_TYPE(7i32);
pub const NTSTRSAFE_MAX_CCH: u32 = 2147483647u32;
pub const NTSTRSAFE_MAX_LENGTH: u32 = 2147483646u32;
pub const NTSTRSAFE_UNICODE_STRING_MAX_CCH: u32 = 32767u32;
pub const NTSTRSAFE_USE_SECURE_CRT: u32 = 0u32;
pub const NonPagedPool: POOL_TYPE = POOL_TYPE(0i32);
pub const NonPagedPoolBase: POOL_TYPE = POOL_TYPE(0i32);
pub const NonPagedPoolBaseCacheAligned: POOL_TYPE = POOL_TYPE(4i32);
pub const NonPagedPoolBaseCacheAlignedMustS: POOL_TYPE = POOL_TYPE(6i32);
pub const NonPagedPoolBaseMustSucceed: POOL_TYPE = POOL_TYPE(2i32);
pub const NonPagedPoolCacheAligned: POOL_TYPE = POOL_TYPE(4i32);
pub const NonPagedPoolCacheAlignedMustS: POOL_TYPE = POOL_TYPE(6i32);
pub const NonPagedPoolCacheAlignedMustSSession: POOL_TYPE = POOL_TYPE(38i32);
pub const NonPagedPoolCacheAlignedSession: POOL_TYPE = POOL_TYPE(36i32);
pub const NonPagedPoolExecute: POOL_TYPE = POOL_TYPE(0i32);
pub const NonPagedPoolMustSucceed: POOL_TYPE = POOL_TYPE(2i32);
pub const NonPagedPoolMustSucceedSession: POOL_TYPE = POOL_TYPE(34i32);
pub const NonPagedPoolNx: POOL_TYPE = POOL_TYPE(512i32);
pub const NonPagedPoolNxCacheAligned: POOL_TYPE = POOL_TYPE(516i32);
pub const NonPagedPoolSession: POOL_TYPE = POOL_TYPE(32i32);
pub const NonPagedPoolSessionNx: POOL_TYPE = POOL_TYPE(544i32);
pub const ObjectBasicInformation: OBJECT_INFORMATION_CLASS = OBJECT_INFORMATION_CLASS(0i32);
pub const ObjectTypeInformation: OBJECT_INFORMATION_CLASS = OBJECT_INFORMATION_CLASS(2i32);
pub const PagedPool: POOL_TYPE = POOL_TYPE(1i32);
pub const PagedPoolCacheAligned: POOL_TYPE = POOL_TYPE(5i32);
pub const PagedPoolCacheAlignedSession: POOL_TYPE = POOL_TYPE(37i32);
pub const PagedPoolSession: POOL_TYPE = POOL_TYPE(33i32);
pub const STRSAFE_FILL_BEHIND: u32 = 512u32;
pub const STRSAFE_FILL_BEHIND_NULL: u32 = 512u32;
pub const STRSAFE_FILL_ON_FAILURE: u32 = 1024u32;
pub const STRSAFE_IGNORE_NULLS: u32 = 256u32;
pub const STRSAFE_NO_TRUNCATION: u32 = 4096u32;
pub const STRSAFE_NULL_ON_FAILURE: u32 = 2048u32;
pub const STRSAFE_ZERO_LENGTH_ON_FAILURE: u32 = 2048u32;
pub const __WARNING_BANNED_API_USAGE: u32 = 28719u32;
pub const __WARNING_CYCLOMATIC_COMPLEXITY: u32 = 28734u32;
pub const __WARNING_DEREF_NULL_PTR: u32 = 6011u32;
pub const __WARNING_HIGH_PRIORITY_OVERFLOW_POSTCONDITION: u32 = 26045u32;
pub const __WARNING_INCORRECT_ANNOTATION: u32 = 26007u32;
pub const __WARNING_INVALID_PARAM_VALUE_1: u32 = 6387u32;
pub const __WARNING_INVALID_PARAM_VALUE_3: u32 = 28183u32;
pub const __WARNING_MISSING_ZERO_TERMINATION2: u32 = 6054u32;
pub const __WARNING_POSTCONDITION_NULLTERMINATION_VIOLATION: u32 = 26036u32;
pub const __WARNING_POST_EXPECTED: u32 = 28210u32;
pub const __WARNING_POTENTIAL_BUFFER_OVERFLOW_HIGH_PRIORITY: u32 = 26015u32;
pub const __WARNING_POTENTIAL_RANGE_POSTCONDITION_VIOLATION: u32 = 26071u32;
pub const __WARNING_PRECONDITION_NULLTERMINATION_VIOLATION: u32 = 26035u32;
pub const __WARNING_RANGE_POSTCONDITION_VIOLATION: u32 = 26061u32;
pub const __WARNING_RETURNING_BAD_RESULT: u32 = 28196u32;
pub const __WARNING_RETURN_UNINIT_VAR: u32 = 6101u32;
pub const __WARNING_USING_UNINIT_VAR: u32 = 6001u32;
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IO_PRIORITY_HINT(pub i32);
impl windows_core::TypeKind for IO_PRIORITY_HINT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for IO_PRIORITY_HINT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IO_PRIORITY_HINT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct KSPIN_LOCK_QUEUE_NUMBER(pub i32);
impl windows_core::TypeKind for KSPIN_LOCK_QUEUE_NUMBER {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for KSPIN_LOCK_QUEUE_NUMBER {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("KSPIN_LOCK_QUEUE_NUMBER").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct OBJECT_INFORMATION_CLASS(pub i32);
impl windows_core::TypeKind for OBJECT_INFORMATION_CLASS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for OBJECT_INFORMATION_CLASS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OBJECT_INFORMATION_CLASS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct POOL_TYPE(pub i32);
impl windows_core::TypeKind for POOL_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for POOL_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("POOL_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_System_SystemServices", feature = "Win32_Security"))]
pub struct ACCESS_STATE {
    pub OperationID: super::super::Win32::Foundation::LUID,
    pub SecurityEvaluated: super::super::Win32::Foundation::BOOLEAN,
    pub GenerateAudit: super::super::Win32::Foundation::BOOLEAN,
    pub GenerateOnClose: super::super::Win32::Foundation::BOOLEAN,
    pub PrivilegesAllocated: super::super::Win32::Foundation::BOOLEAN,
    pub Flags: u32,
    pub RemainingDesiredAccess: u32,
    pub PreviouslyGrantedAccess: u32,
    pub OriginalDesiredAccess: u32,
    pub SubjectSecurityContext: SECURITY_SUBJECT_CONTEXT,
    pub SecurityDescriptor: super::super::Win32::Security::PSECURITY_DESCRIPTOR,
    pub AuxData: *mut core::ffi::c_void,
    pub Privileges: ACCESS_STATE_0,
    pub AuditPrivileges: super::super::Win32::Foundation::BOOLEAN,
    pub ObjectName: super::super::Win32::Foundation::UNICODE_STRING,
    pub ObjectTypeName: super::super::Win32::Foundation::UNICODE_STRING,
}
#[cfg(all(feature = "Wdk_System_SystemServices", feature = "Win32_Security"))]
impl Copy for ACCESS_STATE {}
#[cfg(all(feature = "Wdk_System_SystemServices", feature = "Win32_Security"))]
impl Clone for ACCESS_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_System_SystemServices", feature = "Win32_Security"))]
impl windows_core::TypeKind for ACCESS_STATE {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_System_SystemServices", feature = "Win32_Security"))]
impl Default for ACCESS_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_System_SystemServices", feature = "Win32_Security"))]
pub union ACCESS_STATE_0 {
    pub InitialPrivilegeSet: super::System::SystemServices::INITIAL_PRIVILEGE_SET,
    pub PrivilegeSet: super::super::Win32::Security::PRIVILEGE_SET,
}
#[cfg(all(feature = "Wdk_System_SystemServices", feature = "Win32_Security"))]
impl Copy for ACCESS_STATE_0 {}
#[cfg(all(feature = "Wdk_System_SystemServices", feature = "Win32_Security"))]
impl Clone for ACCESS_STATE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_System_SystemServices", feature = "Win32_Security"))]
impl windows_core::TypeKind for ACCESS_STATE_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_System_SystemServices", feature = "Win32_Security"))]
impl Default for ACCESS_STATE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub struct DEVICE_OBJECT {
    pub Type: i16,
    pub Size: u16,
    pub ReferenceCount: i32,
    pub DriverObject: *mut DRIVER_OBJECT,
    pub NextDevice: *mut DEVICE_OBJECT,
    pub AttachedDevice: *mut DEVICE_OBJECT,
    pub CurrentIrp: *mut IRP,
    pub Timer: PIO_TIMER,
    pub Flags: u32,
    pub Characteristics: u32,
    pub Vpb: *mut VPB,
    pub DeviceExtension: *mut core::ffi::c_void,
    pub DeviceType: u32,
    pub StackSize: i8,
    pub Queue: DEVICE_OBJECT_0,
    pub AlignmentRequirement: u32,
    pub DeviceQueue: KDEVICE_QUEUE,
    pub Dpc: KDPC,
    pub ActiveThreadCount: u32,
    pub SecurityDescriptor: super::super::Win32::Security::PSECURITY_DESCRIPTOR,
    pub DeviceLock: KEVENT,
    pub SectorSize: u16,
    pub Spare1: u16,
    pub DeviceObjectExtension: *mut DEVOBJ_EXTENSION,
    pub Reserved: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Copy for DEVICE_OBJECT {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Clone for DEVICE_OBJECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for DEVICE_OBJECT {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for DEVICE_OBJECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub union DEVICE_OBJECT_0 {
    pub ListEntry: super::super::Win32::System::Kernel::LIST_ENTRY,
    pub Wcb: super::System::SystemServices::WAIT_CONTEXT_BLOCK,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Copy for DEVICE_OBJECT_0 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Clone for DEVICE_OBJECT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for DEVICE_OBJECT_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for DEVICE_OBJECT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub struct DEVOBJ_EXTENSION {
    pub Type: i16,
    pub Size: u16,
    pub DeviceObject: *mut DEVICE_OBJECT,
    pub PowerFlags: u32,
    pub Dope: *mut _DEVICE_OBJECT_POWER_EXTENSION,
    pub ExtensionFlags: u32,
    pub DeviceNode: *mut core::ffi::c_void,
    pub AttachedTo: *mut DEVICE_OBJECT,
    pub StartIoCount: i32,
    pub StartIoKey: i32,
    pub StartIoFlags: u32,
    pub Vpb: *mut VPB,
    pub DependencyNode: *mut core::ffi::c_void,
    pub InterruptContext: *mut core::ffi::c_void,
    pub InterruptCount: i32,
    pub VerifierContext: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Copy for DEVOBJ_EXTENSION {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Clone for DEVOBJ_EXTENSION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl core::fmt::Debug for DEVOBJ_EXTENSION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DEVOBJ_EXTENSION")
            .field("Type", &self.Type)
            .field("Size", &self.Size)
            .field("DeviceObject", &self.DeviceObject)
            .field("PowerFlags", &self.PowerFlags)
            .field("Dope", &self.Dope)
            .field("ExtensionFlags", &self.ExtensionFlags)
            .field("DeviceNode", &self.DeviceNode)
            .field("AttachedTo", &self.AttachedTo)
            .field("StartIoCount", &self.StartIoCount)
            .field("StartIoKey", &self.StartIoKey)
            .field("StartIoFlags", &self.StartIoFlags)
            .field("Vpb", &self.Vpb)
            .field("DependencyNode", &self.DependencyNode)
            .field("InterruptContext", &self.InterruptContext)
            .field("InterruptCount", &self.InterruptCount)
            .field("VerifierContext", &self.VerifierContext)
            .finish()
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for DEVOBJ_EXTENSION {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl PartialEq for DEVOBJ_EXTENSION {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Size == other.Size && self.DeviceObject == other.DeviceObject && self.PowerFlags == other.PowerFlags && self.Dope == other.Dope && self.ExtensionFlags == other.ExtensionFlags && self.DeviceNode == other.DeviceNode && self.AttachedTo == other.AttachedTo && self.StartIoCount == other.StartIoCount && self.StartIoKey == other.StartIoKey && self.StartIoFlags == other.StartIoFlags && self.Vpb == other.Vpb && self.DependencyNode == other.DependencyNode && self.InterruptContext == other.InterruptContext && self.InterruptCount == other.InterruptCount && self.VerifierContext == other.VerifierContext
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Eq for DEVOBJ_EXTENSION {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for DEVOBJ_EXTENSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
pub struct DISPATCHER_HEADER {
    pub Anonymous: DISPATCHER_HEADER_0,
    pub SignalState: i32,
    pub WaitListHead: super::super::Win32::System::Kernel::LIST_ENTRY,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Copy for DISPATCHER_HEADER {}
#[cfg(feature = "Win32_System_Kernel")]
impl Clone for DISPATCHER_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl windows_core::TypeKind for DISPATCHER_HEADER {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for DISPATCHER_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
pub union DISPATCHER_HEADER_0 {
    pub Anonymous1: DISPATCHER_HEADER_0_0,
    pub Anonymous2: DISPATCHER_HEADER_0_1,
    pub Anonymous3: DISPATCHER_HEADER_0_2,
    pub Anonymous4: DISPATCHER_HEADER_0_3,
    pub Anonymous5: DISPATCHER_HEADER_0_4,
    pub Anonymous6: DISPATCHER_HEADER_0_5,
    pub Anonymous7: DISPATCHER_HEADER_0_6,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Copy for DISPATCHER_HEADER_0 {}
#[cfg(feature = "Win32_System_Kernel")]
impl Clone for DISPATCHER_HEADER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl windows_core::TypeKind for DISPATCHER_HEADER_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for DISPATCHER_HEADER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
pub union DISPATCHER_HEADER_0_0 {
    pub Lock: i32,
    pub LockNV: i32,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Copy for DISPATCHER_HEADER_0_0 {}
#[cfg(feature = "Win32_System_Kernel")]
impl Clone for DISPATCHER_HEADER_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl windows_core::TypeKind for DISPATCHER_HEADER_0_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for DISPATCHER_HEADER_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
pub struct DISPATCHER_HEADER_0_1 {
    pub Type: u8,
    pub Signalling: u8,
    pub Size: u8,
    pub Reserved1: u8,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Copy for DISPATCHER_HEADER_0_1 {}
#[cfg(feature = "Win32_System_Kernel")]
impl Clone for DISPATCHER_HEADER_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl core::fmt::Debug for DISPATCHER_HEADER_0_1 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DISPATCHER_HEADER_0_1").field("Type", &self.Type).field("Signalling", &self.Signalling).field("Size", &self.Size).field("Reserved1", &self.Reserved1).finish()
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl windows_core::TypeKind for DISPATCHER_HEADER_0_1 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Kernel")]
impl PartialEq for DISPATCHER_HEADER_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Signalling == other.Signalling && self.Size == other.Size && self.Reserved1 == other.Reserved1
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl Eq for DISPATCHER_HEADER_0_1 {}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for DISPATCHER_HEADER_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
pub struct DISPATCHER_HEADER_0_2 {
    pub TimerType: u8,
    pub Anonymous1: DISPATCHER_HEADER_0_2_0,
    pub Hand: u8,
    pub Anonymous2: DISPATCHER_HEADER_0_2_1,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Copy for DISPATCHER_HEADER_0_2 {}
#[cfg(feature = "Win32_System_Kernel")]
impl Clone for DISPATCHER_HEADER_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl windows_core::TypeKind for DISPATCHER_HEADER_0_2 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for DISPATCHER_HEADER_0_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
pub union DISPATCHER_HEADER_0_2_0 {
    pub TimerControlFlags: u8,
    pub Anonymous: DISPATCHER_HEADER_0_2_0_0,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Copy for DISPATCHER_HEADER_0_2_0 {}
#[cfg(feature = "Win32_System_Kernel")]
impl Clone for DISPATCHER_HEADER_0_2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl windows_core::TypeKind for DISPATCHER_HEADER_0_2_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for DISPATCHER_HEADER_0_2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
pub struct DISPATCHER_HEADER_0_2_0_0 {
    pub _bitfield: u8,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Copy for DISPATCHER_HEADER_0_2_0_0 {}
#[cfg(feature = "Win32_System_Kernel")]
impl Clone for DISPATCHER_HEADER_0_2_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl core::fmt::Debug for DISPATCHER_HEADER_0_2_0_0 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DISPATCHER_HEADER_0_2_0_0").field("_bitfield", &self._bitfield).finish()
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl windows_core::TypeKind for DISPATCHER_HEADER_0_2_0_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Kernel")]
impl PartialEq for DISPATCHER_HEADER_0_2_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl Eq for DISPATCHER_HEADER_0_2_0_0 {}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for DISPATCHER_HEADER_0_2_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
pub union DISPATCHER_HEADER_0_2_1 {
    pub TimerMiscFlags: u8,
    pub Anonymous: DISPATCHER_HEADER_0_2_1_0,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Copy for DISPATCHER_HEADER_0_2_1 {}
#[cfg(feature = "Win32_System_Kernel")]
impl Clone for DISPATCHER_HEADER_0_2_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl windows_core::TypeKind for DISPATCHER_HEADER_0_2_1 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for DISPATCHER_HEADER_0_2_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
pub struct DISPATCHER_HEADER_0_2_1_0 {
    pub _bitfield: u8,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Copy for DISPATCHER_HEADER_0_2_1_0 {}
#[cfg(feature = "Win32_System_Kernel")]
impl Clone for DISPATCHER_HEADER_0_2_1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl core::fmt::Debug for DISPATCHER_HEADER_0_2_1_0 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DISPATCHER_HEADER_0_2_1_0").field("_bitfield", &self._bitfield).finish()
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl windows_core::TypeKind for DISPATCHER_HEADER_0_2_1_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Kernel")]
impl PartialEq for DISPATCHER_HEADER_0_2_1_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl Eq for DISPATCHER_HEADER_0_2_1_0 {}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for DISPATCHER_HEADER_0_2_1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
pub struct DISPATCHER_HEADER_0_3 {
    pub Timer2Type: u8,
    pub Anonymous: DISPATCHER_HEADER_0_3_0,
    pub Timer2ComponentId: u8,
    pub Timer2RelativeId: u8,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Copy for DISPATCHER_HEADER_0_3 {}
#[cfg(feature = "Win32_System_Kernel")]
impl Clone for DISPATCHER_HEADER_0_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl windows_core::TypeKind for DISPATCHER_HEADER_0_3 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for DISPATCHER_HEADER_0_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
pub union DISPATCHER_HEADER_0_3_0 {
    pub Timer2Flags: u8,
    pub Anonymous: DISPATCHER_HEADER_0_3_0_0,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Copy for DISPATCHER_HEADER_0_3_0 {}
#[cfg(feature = "Win32_System_Kernel")]
impl Clone for DISPATCHER_HEADER_0_3_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl windows_core::TypeKind for DISPATCHER_HEADER_0_3_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for DISPATCHER_HEADER_0_3_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
pub struct DISPATCHER_HEADER_0_3_0_0 {
    pub _bitfield: u8,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Copy for DISPATCHER_HEADER_0_3_0_0 {}
#[cfg(feature = "Win32_System_Kernel")]
impl Clone for DISPATCHER_HEADER_0_3_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl core::fmt::Debug for DISPATCHER_HEADER_0_3_0_0 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DISPATCHER_HEADER_0_3_0_0").field("_bitfield", &self._bitfield).finish()
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl windows_core::TypeKind for DISPATCHER_HEADER_0_3_0_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Kernel")]
impl PartialEq for DISPATCHER_HEADER_0_3_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl Eq for DISPATCHER_HEADER_0_3_0_0 {}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for DISPATCHER_HEADER_0_3_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
pub struct DISPATCHER_HEADER_0_4 {
    pub QueueType: u8,
    pub Anonymous: DISPATCHER_HEADER_0_4_0,
    pub QueueSize: u8,
    pub QueueReserved: u8,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Copy for DISPATCHER_HEADER_0_4 {}
#[cfg(feature = "Win32_System_Kernel")]
impl Clone for DISPATCHER_HEADER_0_4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl windows_core::TypeKind for DISPATCHER_HEADER_0_4 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for DISPATCHER_HEADER_0_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
pub union DISPATCHER_HEADER_0_4_0 {
    pub QueueControlFlags: u8,
    pub Anonymous: DISPATCHER_HEADER_0_4_0_0,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Copy for DISPATCHER_HEADER_0_4_0 {}
#[cfg(feature = "Win32_System_Kernel")]
impl Clone for DISPATCHER_HEADER_0_4_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl windows_core::TypeKind for DISPATCHER_HEADER_0_4_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for DISPATCHER_HEADER_0_4_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
pub struct DISPATCHER_HEADER_0_4_0_0 {
    pub _bitfield: u8,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Copy for DISPATCHER_HEADER_0_4_0_0 {}
#[cfg(feature = "Win32_System_Kernel")]
impl Clone for DISPATCHER_HEADER_0_4_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl core::fmt::Debug for DISPATCHER_HEADER_0_4_0_0 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DISPATCHER_HEADER_0_4_0_0").field("_bitfield", &self._bitfield).finish()
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl windows_core::TypeKind for DISPATCHER_HEADER_0_4_0_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Kernel")]
impl PartialEq for DISPATCHER_HEADER_0_4_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl Eq for DISPATCHER_HEADER_0_4_0_0 {}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for DISPATCHER_HEADER_0_4_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
pub struct DISPATCHER_HEADER_0_5 {
    pub ThreadType: u8,
    pub ThreadReserved: u8,
    pub Anonymous1: DISPATCHER_HEADER_0_5_0,
    pub Anonymous2: DISPATCHER_HEADER_0_5_1,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Copy for DISPATCHER_HEADER_0_5 {}
#[cfg(feature = "Win32_System_Kernel")]
impl Clone for DISPATCHER_HEADER_0_5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl windows_core::TypeKind for DISPATCHER_HEADER_0_5 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for DISPATCHER_HEADER_0_5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
pub union DISPATCHER_HEADER_0_5_0 {
    pub ThreadControlFlags: u8,
    pub Anonymous: DISPATCHER_HEADER_0_5_0_0,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Copy for DISPATCHER_HEADER_0_5_0 {}
#[cfg(feature = "Win32_System_Kernel")]
impl Clone for DISPATCHER_HEADER_0_5_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl windows_core::TypeKind for DISPATCHER_HEADER_0_5_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for DISPATCHER_HEADER_0_5_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
pub struct DISPATCHER_HEADER_0_5_0_0 {
    pub _bitfield: u8,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Copy for DISPATCHER_HEADER_0_5_0_0 {}
#[cfg(feature = "Win32_System_Kernel")]
impl Clone for DISPATCHER_HEADER_0_5_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl core::fmt::Debug for DISPATCHER_HEADER_0_5_0_0 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DISPATCHER_HEADER_0_5_0_0").field("_bitfield", &self._bitfield).finish()
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl windows_core::TypeKind for DISPATCHER_HEADER_0_5_0_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Kernel")]
impl PartialEq for DISPATCHER_HEADER_0_5_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl Eq for DISPATCHER_HEADER_0_5_0_0 {}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for DISPATCHER_HEADER_0_5_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
pub union DISPATCHER_HEADER_0_5_1 {
    pub DebugActive: u8,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Copy for DISPATCHER_HEADER_0_5_1 {}
#[cfg(feature = "Win32_System_Kernel")]
impl Clone for DISPATCHER_HEADER_0_5_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl windows_core::TypeKind for DISPATCHER_HEADER_0_5_1 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for DISPATCHER_HEADER_0_5_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
pub struct DISPATCHER_HEADER_0_6 {
    pub MutantType: u8,
    pub MutantSize: u8,
    pub DpcActive: super::super::Win32::Foundation::BOOLEAN,
    pub MutantReserved: u8,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Copy for DISPATCHER_HEADER_0_6 {}
#[cfg(feature = "Win32_System_Kernel")]
impl Clone for DISPATCHER_HEADER_0_6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl core::fmt::Debug for DISPATCHER_HEADER_0_6 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DISPATCHER_HEADER_0_6").field("MutantType", &self.MutantType).field("MutantSize", &self.MutantSize).field("DpcActive", &self.DpcActive).field("MutantReserved", &self.MutantReserved).finish()
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl windows_core::TypeKind for DISPATCHER_HEADER_0_6 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Kernel")]
impl PartialEq for DISPATCHER_HEADER_0_6 {
    fn eq(&self, other: &Self) -> bool {
        self.MutantType == other.MutantType && self.MutantSize == other.MutantSize && self.DpcActive == other.DpcActive && self.MutantReserved == other.MutantReserved
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl Eq for DISPATCHER_HEADER_0_6 {}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for DISPATCHER_HEADER_0_6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct DMA_COMMON_BUFFER_VECTOR(pub isize);
impl Default for DMA_COMMON_BUFFER_VECTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for DMA_COMMON_BUFFER_VECTOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for DMA_COMMON_BUFFER_VECTOR {}
impl core::fmt::Debug for DMA_COMMON_BUFFER_VECTOR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DMA_COMMON_BUFFER_VECTOR").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for DMA_COMMON_BUFFER_VECTOR {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub struct DRIVER_EXTENSION {
    pub DriverObject: *mut DRIVER_OBJECT,
    pub AddDevice: *mut DRIVER_ADD_DEVICE,
    pub Count: u32,
    pub ServiceKeyName: super::super::Win32::Foundation::UNICODE_STRING,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Copy for DRIVER_EXTENSION {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Clone for DRIVER_EXTENSION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl core::fmt::Debug for DRIVER_EXTENSION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DRIVER_EXTENSION").field("DriverObject", &self.DriverObject).field("AddDevice", &self.AddDevice).field("Count", &self.Count).field("ServiceKeyName", &self.ServiceKeyName).finish()
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for DRIVER_EXTENSION {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl PartialEq for DRIVER_EXTENSION {
    fn eq(&self, other: &Self) -> bool {
        self.DriverObject == other.DriverObject && self.AddDevice == other.AddDevice && self.Count == other.Count && self.ServiceKeyName == other.ServiceKeyName
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Eq for DRIVER_EXTENSION {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for DRIVER_EXTENSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub struct DRIVER_OBJECT {
    pub Type: i16,
    pub Size: i16,
    pub DeviceObject: *mut DEVICE_OBJECT,
    pub Flags: u32,
    pub DriverStart: *mut core::ffi::c_void,
    pub DriverSize: u32,
    pub DriverSection: *mut core::ffi::c_void,
    pub DriverExtension: *mut DRIVER_EXTENSION,
    pub DriverName: super::super::Win32::Foundation::UNICODE_STRING,
    pub HardwareDatabase: *mut super::super::Win32::Foundation::UNICODE_STRING,
    pub FastIoDispatch: *mut FAST_IO_DISPATCH,
    pub DriverInit: *mut DRIVER_INITIALIZE,
    pub DriverStartIo: *mut DRIVER_STARTIO,
    pub DriverUnload: *mut DRIVER_UNLOAD,
    pub MajorFunction: [*mut DRIVER_DISPATCH; 28],
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Copy for DRIVER_OBJECT {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Clone for DRIVER_OBJECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl core::fmt::Debug for DRIVER_OBJECT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DRIVER_OBJECT")
            .field("Type", &self.Type)
            .field("Size", &self.Size)
            .field("DeviceObject", &self.DeviceObject)
            .field("Flags", &self.Flags)
            .field("DriverStart", &self.DriverStart)
            .field("DriverSize", &self.DriverSize)
            .field("DriverSection", &self.DriverSection)
            .field("DriverExtension", &self.DriverExtension)
            .field("DriverName", &self.DriverName)
            .field("HardwareDatabase", &self.HardwareDatabase)
            .field("FastIoDispatch", &self.FastIoDispatch)
            .field("DriverInit", &self.DriverInit)
            .field("DriverStartIo", &self.DriverStartIo)
            .field("DriverUnload", &self.DriverUnload)
            .field("MajorFunction", &self.MajorFunction)
            .finish()
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for DRIVER_OBJECT {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl PartialEq for DRIVER_OBJECT {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Size == other.Size && self.DeviceObject == other.DeviceObject && self.Flags == other.Flags && self.DriverStart == other.DriverStart && self.DriverSize == other.DriverSize && self.DriverSection == other.DriverSection && self.DriverExtension == other.DriverExtension && self.DriverName == other.DriverName && self.HardwareDatabase == other.HardwareDatabase && self.FastIoDispatch == other.FastIoDispatch && self.DriverInit == other.DriverInit && self.DriverStartIo == other.DriverStartIo && self.DriverUnload == other.DriverUnload && self.MajorFunction == other.MajorFunction
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Eq for DRIVER_OBJECT {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for DRIVER_OBJECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct ECP_HEADER(pub isize);
impl Default for ECP_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for ECP_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for ECP_HEADER {}
impl core::fmt::Debug for ECP_HEADER {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ECP_HEADER").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for ECP_HEADER {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct ECP_LIST(pub isize);
impl Default for ECP_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for ECP_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for ECP_LIST {}
impl core::fmt::Debug for ECP_LIST {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ECP_LIST").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for ECP_LIST {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
pub struct ERESOURCE {
    pub SystemResourcesList: super::super::Win32::System::Kernel::LIST_ENTRY,
    pub OwnerTable: *mut OWNER_ENTRY,
    pub ActiveCount: i16,
    pub Anonymous1: ERESOURCE_0,
    pub SharedWaiters: *mut core::ffi::c_void,
    pub ExclusiveWaiters: *mut core::ffi::c_void,
    pub OwnerEntry: OWNER_ENTRY,
    pub ActiveEntries: u32,
    pub ContentionCount: u32,
    pub NumberOfSharedWaiters: u32,
    pub NumberOfExclusiveWaiters: u32,
    pub Anonymous2: ERESOURCE_1,
    pub SpinLock: usize,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Copy for ERESOURCE {}
#[cfg(feature = "Win32_System_Kernel")]
impl Clone for ERESOURCE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl windows_core::TypeKind for ERESOURCE {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for ERESOURCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
pub union ERESOURCE_0 {
    pub Flag: u16,
    pub Anonymous: ERESOURCE_0_0,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Copy for ERESOURCE_0 {}
#[cfg(feature = "Win32_System_Kernel")]
impl Clone for ERESOURCE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl windows_core::TypeKind for ERESOURCE_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for ERESOURCE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
pub struct ERESOURCE_0_0 {
    pub ReservedLowFlags: u8,
    pub WaiterPriority: u8,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Copy for ERESOURCE_0_0 {}
#[cfg(feature = "Win32_System_Kernel")]
impl Clone for ERESOURCE_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl core::fmt::Debug for ERESOURCE_0_0 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ERESOURCE_0_0").field("ReservedLowFlags", &self.ReservedLowFlags).field("WaiterPriority", &self.WaiterPriority).finish()
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl windows_core::TypeKind for ERESOURCE_0_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Kernel")]
impl PartialEq for ERESOURCE_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.ReservedLowFlags == other.ReservedLowFlags && self.WaiterPriority == other.WaiterPriority
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl Eq for ERESOURCE_0_0 {}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for ERESOURCE_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
pub union ERESOURCE_1 {
    pub Address: *mut core::ffi::c_void,
    pub CreatorBackTraceIndex: usize,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Copy for ERESOURCE_1 {}
#[cfg(feature = "Win32_System_Kernel")]
impl Clone for ERESOURCE_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl windows_core::TypeKind for ERESOURCE_1 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for ERESOURCE_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub struct FAST_IO_DISPATCH {
    pub SizeOfFastIoDispatch: u32,
    pub FastIoCheckIfPossible: *mut FAST_IO_CHECK_IF_POSSIBLE,
    pub FastIoRead: *mut FAST_IO_READ,
    pub FastIoWrite: *mut FAST_IO_WRITE,
    pub FastIoQueryBasicInfo: *mut FAST_IO_QUERY_BASIC_INFO,
    pub FastIoQueryStandardInfo: *mut FAST_IO_QUERY_STANDARD_INFO,
    pub FastIoLock: *mut FAST_IO_LOCK,
    pub FastIoUnlockSingle: *mut FAST_IO_UNLOCK_SINGLE,
    pub FastIoUnlockAll: *mut FAST_IO_UNLOCK_ALL,
    pub FastIoUnlockAllByKey: *mut FAST_IO_UNLOCK_ALL_BY_KEY,
    pub FastIoDeviceControl: *mut FAST_IO_DEVICE_CONTROL,
    pub AcquireFileForNtCreateSection: *mut FAST_IO_ACQUIRE_FILE,
    pub ReleaseFileForNtCreateSection: *mut FAST_IO_RELEASE_FILE,
    pub FastIoDetachDevice: *mut FAST_IO_DETACH_DEVICE,
    pub FastIoQueryNetworkOpenInfo: *mut FAST_IO_QUERY_NETWORK_OPEN_INFO,
    pub AcquireForModWrite: *mut FAST_IO_ACQUIRE_FOR_MOD_WRITE,
    pub MdlRead: *mut FAST_IO_MDL_READ,
    pub MdlReadComplete: *mut FAST_IO_MDL_READ_COMPLETE,
    pub PrepareMdlWrite: *mut FAST_IO_PREPARE_MDL_WRITE,
    pub MdlWriteComplete: *mut FAST_IO_MDL_WRITE_COMPLETE,
    pub FastIoReadCompressed: *mut FAST_IO_READ_COMPRESSED,
    pub FastIoWriteCompressed: *mut FAST_IO_WRITE_COMPRESSED,
    pub MdlReadCompleteCompressed: *mut FAST_IO_MDL_READ_COMPLETE_COMPRESSED,
    pub MdlWriteCompleteCompressed: *mut FAST_IO_MDL_WRITE_COMPLETE_COMPRESSED,
    pub FastIoQueryOpen: *mut FAST_IO_QUERY_OPEN,
    pub ReleaseForModWrite: *mut FAST_IO_RELEASE_FOR_MOD_WRITE,
    pub AcquireForCcFlush: *mut FAST_IO_ACQUIRE_FOR_CCFLUSH,
    pub ReleaseForCcFlush: *mut FAST_IO_RELEASE_FOR_CCFLUSH,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Copy for FAST_IO_DISPATCH {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Clone for FAST_IO_DISPATCH {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl core::fmt::Debug for FAST_IO_DISPATCH {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("FAST_IO_DISPATCH")
            .field("SizeOfFastIoDispatch", &self.SizeOfFastIoDispatch)
            .field("FastIoCheckIfPossible", &self.FastIoCheckIfPossible)
            .field("FastIoRead", &self.FastIoRead)
            .field("FastIoWrite", &self.FastIoWrite)
            .field("FastIoQueryBasicInfo", &self.FastIoQueryBasicInfo)
            .field("FastIoQueryStandardInfo", &self.FastIoQueryStandardInfo)
            .field("FastIoLock", &self.FastIoLock)
            .field("FastIoUnlockSingle", &self.FastIoUnlockSingle)
            .field("FastIoUnlockAll", &self.FastIoUnlockAll)
            .field("FastIoUnlockAllByKey", &self.FastIoUnlockAllByKey)
            .field("FastIoDeviceControl", &self.FastIoDeviceControl)
            .field("AcquireFileForNtCreateSection", &self.AcquireFileForNtCreateSection)
            .field("ReleaseFileForNtCreateSection", &self.ReleaseFileForNtCreateSection)
            .field("FastIoDetachDevice", &self.FastIoDetachDevice)
            .field("FastIoQueryNetworkOpenInfo", &self.FastIoQueryNetworkOpenInfo)
            .field("AcquireForModWrite", &self.AcquireForModWrite)
            .field("MdlRead", &self.MdlRead)
            .field("MdlReadComplete", &self.MdlReadComplete)
            .field("PrepareMdlWrite", &self.PrepareMdlWrite)
            .field("MdlWriteComplete", &self.MdlWriteComplete)
            .field("FastIoReadCompressed", &self.FastIoReadCompressed)
            .field("FastIoWriteCompressed", &self.FastIoWriteCompressed)
            .field("MdlReadCompleteCompressed", &self.MdlReadCompleteCompressed)
            .field("MdlWriteCompleteCompressed", &self.MdlWriteCompleteCompressed)
            .field("FastIoQueryOpen", &self.FastIoQueryOpen)
            .field("ReleaseForModWrite", &self.ReleaseForModWrite)
            .field("AcquireForCcFlush", &self.AcquireForCcFlush)
            .field("ReleaseForCcFlush", &self.ReleaseForCcFlush)
            .finish()
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FAST_IO_DISPATCH {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl PartialEq for FAST_IO_DISPATCH {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfFastIoDispatch == other.SizeOfFastIoDispatch
            && self.FastIoCheckIfPossible == other.FastIoCheckIfPossible
            && self.FastIoRead == other.FastIoRead
            && self.FastIoWrite == other.FastIoWrite
            && self.FastIoQueryBasicInfo == other.FastIoQueryBasicInfo
            && self.FastIoQueryStandardInfo == other.FastIoQueryStandardInfo
            && self.FastIoLock == other.FastIoLock
            && self.FastIoUnlockSingle == other.FastIoUnlockSingle
            && self.FastIoUnlockAll == other.FastIoUnlockAll
            && self.FastIoUnlockAllByKey == other.FastIoUnlockAllByKey
            && self.FastIoDeviceControl == other.FastIoDeviceControl
            && self.AcquireFileForNtCreateSection == other.AcquireFileForNtCreateSection
            && self.ReleaseFileForNtCreateSection == other.ReleaseFileForNtCreateSection
            && self.FastIoDetachDevice == other.FastIoDetachDevice
            && self.FastIoQueryNetworkOpenInfo == other.FastIoQueryNetworkOpenInfo
            && self.AcquireForModWrite == other.AcquireForModWrite
            && self.MdlRead == other.MdlRead
            && self.MdlReadComplete == other.MdlReadComplete
            && self.PrepareMdlWrite == other.PrepareMdlWrite
            && self.MdlWriteComplete == other.MdlWriteComplete
            && self.FastIoReadCompressed == other.FastIoReadCompressed
            && self.FastIoWriteCompressed == other.FastIoWriteCompressed
            && self.MdlReadCompleteCompressed == other.MdlReadCompleteCompressed
            && self.MdlWriteCompleteCompressed == other.MdlWriteCompleteCompressed
            && self.FastIoQueryOpen == other.FastIoQueryOpen
            && self.ReleaseForModWrite == other.ReleaseForModWrite
            && self.AcquireForCcFlush == other.AcquireForCcFlush
            && self.ReleaseForCcFlush == other.ReleaseForCcFlush
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Eq for FAST_IO_DISPATCH {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FAST_IO_DISPATCH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
pub struct FAST_MUTEX {
    pub Count: i32,
    pub Owner: *mut core::ffi::c_void,
    pub Contention: u32,
    pub Event: KEVENT,
    pub OldIrql: u32,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Copy for FAST_MUTEX {}
#[cfg(feature = "Win32_System_Kernel")]
impl Clone for FAST_MUTEX {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl windows_core::TypeKind for FAST_MUTEX {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for FAST_MUTEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub struct FILE_OBJECT {
    pub Type: i16,
    pub Size: i16,
    pub DeviceObject: *mut DEVICE_OBJECT,
    pub Vpb: *mut VPB,
    pub FsContext: *mut core::ffi::c_void,
    pub FsContext2: *mut core::ffi::c_void,
    pub SectionObjectPointer: *mut SECTION_OBJECT_POINTERS,
    pub PrivateCacheMap: *mut core::ffi::c_void,
    pub FinalStatus: super::super::Win32::Foundation::NTSTATUS,
    pub RelatedFileObject: *mut FILE_OBJECT,
    pub LockOperation: super::super::Win32::Foundation::BOOLEAN,
    pub DeletePending: super::super::Win32::Foundation::BOOLEAN,
    pub ReadAccess: super::super::Win32::Foundation::BOOLEAN,
    pub WriteAccess: super::super::Win32::Foundation::BOOLEAN,
    pub DeleteAccess: super::super::Win32::Foundation::BOOLEAN,
    pub SharedRead: super::super::Win32::Foundation::BOOLEAN,
    pub SharedWrite: super::super::Win32::Foundation::BOOLEAN,
    pub SharedDelete: super::super::Win32::Foundation::BOOLEAN,
    pub Flags: u32,
    pub FileName: super::super::Win32::Foundation::UNICODE_STRING,
    pub CurrentByteOffset: i64,
    pub Waiters: u32,
    pub Busy: u32,
    pub LastLock: *mut core::ffi::c_void,
    pub Lock: KEVENT,
    pub Event: KEVENT,
    pub CompletionContext: *mut IO_COMPLETION_CONTEXT,
    pub IrpListLock: usize,
    pub IrpList: super::super::Win32::System::Kernel::LIST_ENTRY,
    pub FileObjectExtension: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Copy for FILE_OBJECT {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Clone for FILE_OBJECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FILE_OBJECT {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FILE_OBJECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct IOMMU_DMA_DEVICE(pub isize);
impl Default for IOMMU_DMA_DEVICE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for IOMMU_DMA_DEVICE {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for IOMMU_DMA_DEVICE {}
impl core::fmt::Debug for IOMMU_DMA_DEVICE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IOMMU_DMA_DEVICE").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for IOMMU_DMA_DEVICE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct IOMMU_DMA_DOMAIN(pub isize);
impl Default for IOMMU_DMA_DOMAIN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for IOMMU_DMA_DOMAIN {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for IOMMU_DMA_DOMAIN {}
impl core::fmt::Debug for IOMMU_DMA_DOMAIN {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IOMMU_DMA_DOMAIN").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for IOMMU_DMA_DOMAIN {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
pub struct IO_COMPLETION_CONTEXT {
    pub Port: *mut core::ffi::c_void,
    pub Key: *mut core::ffi::c_void,
    pub UsageCount: isize,
}
impl Copy for IO_COMPLETION_CONTEXT {}
impl Clone for IO_COMPLETION_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for IO_COMPLETION_CONTEXT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("IO_COMPLETION_CONTEXT").field("Port", &self.Port).field("Key", &self.Key).field("UsageCount", &self.UsageCount).finish()
    }
}
impl windows_core::TypeKind for IO_COMPLETION_CONTEXT {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for IO_COMPLETION_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.Port == other.Port && self.Key == other.Key && self.UsageCount == other.UsageCount
    }
}
impl Eq for IO_COMPLETION_CONTEXT {}
impl Default for IO_COMPLETION_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_System_SystemServices", feature = "Win32_Security"))]
pub struct IO_SECURITY_CONTEXT {
    pub SecurityQos: *mut super::super::Win32::Security::SECURITY_QUALITY_OF_SERVICE,
    pub AccessState: *mut ACCESS_STATE,
    pub DesiredAccess: u32,
    pub FullCreateOptions: u32,
}
#[cfg(all(feature = "Wdk_System_SystemServices", feature = "Win32_Security"))]
impl Copy for IO_SECURITY_CONTEXT {}
#[cfg(all(feature = "Wdk_System_SystemServices", feature = "Win32_Security"))]
impl Clone for IO_SECURITY_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_System_SystemServices", feature = "Win32_Security"))]
impl core::fmt::Debug for IO_SECURITY_CONTEXT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("IO_SECURITY_CONTEXT").field("SecurityQos", &self.SecurityQos).field("AccessState", &self.AccessState).field("DesiredAccess", &self.DesiredAccess).field("FullCreateOptions", &self.FullCreateOptions).finish()
    }
}
#[cfg(all(feature = "Wdk_System_SystemServices", feature = "Win32_Security"))]
impl windows_core::TypeKind for IO_SECURITY_CONTEXT {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_System_SystemServices", feature = "Win32_Security"))]
impl PartialEq for IO_SECURITY_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.SecurityQos == other.SecurityQos && self.AccessState == other.AccessState && self.DesiredAccess == other.DesiredAccess && self.FullCreateOptions == other.FullCreateOptions
    }
}
#[cfg(all(feature = "Wdk_System_SystemServices", feature = "Win32_Security"))]
impl Eq for IO_SECURITY_CONTEXT {}
#[cfg(all(feature = "Wdk_System_SystemServices", feature = "Win32_Security"))]
impl Default for IO_SECURITY_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub struct IO_STACK_LOCATION {
    pub MajorFunction: u8,
    pub MinorFunction: u8,
    pub Flags: u8,
    pub Control: u8,
    pub Parameters: IO_STACK_LOCATION_0,
    pub DeviceObject: *mut DEVICE_OBJECT,
    pub FileObject: *mut FILE_OBJECT,
    pub CompletionRoutine: PIO_COMPLETION_ROUTINE,
    pub Context: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Copy for IO_STACK_LOCATION {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Clone for IO_STACK_LOCATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for IO_STACK_LOCATION {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub union IO_STACK_LOCATION_0 {
    pub Create: IO_STACK_LOCATION_0_2,
    pub CreatePipe: IO_STACK_LOCATION_0_1,
    pub CreateMailslot: IO_STACK_LOCATION_0_0,
    pub Read: IO_STACK_LOCATION_0_25,
    pub Write: IO_STACK_LOCATION_0_38,
    pub QueryDirectory: IO_STACK_LOCATION_0_16,
    pub NotifyDirectory: IO_STACK_LOCATION_0_10,
    pub NotifyDirectoryEx: IO_STACK_LOCATION_0_9,
    pub QueryFile: IO_STACK_LOCATION_0_18,
    pub SetFile: IO_STACK_LOCATION_0_28,
    pub QueryEa: IO_STACK_LOCATION_0_17,
    pub SetEa: IO_STACK_LOCATION_0_27,
    pub QueryVolume: IO_STACK_LOCATION_0_23,
    pub SetVolume: IO_STACK_LOCATION_0_32,
    pub FileSystemControl: IO_STACK_LOCATION_0_5,
    pub LockControl: IO_STACK_LOCATION_0_7,
    pub DeviceIoControl: IO_STACK_LOCATION_0_4,
    pub QuerySecurity: IO_STACK_LOCATION_0_22,
    pub SetSecurity: IO_STACK_LOCATION_0_31,
    pub MountVolume: IO_STACK_LOCATION_0_8,
    pub VerifyVolume: IO_STACK_LOCATION_0_35,
    pub Scsi: IO_STACK_LOCATION_0_26,
    pub QueryQuota: IO_STACK_LOCATION_0_21,
    pub SetQuota: IO_STACK_LOCATION_0_30,
    pub QueryDeviceRelations: IO_STACK_LOCATION_0_14,
    pub QueryInterface: IO_STACK_LOCATION_0_20,
    pub DeviceCapabilities: IO_STACK_LOCATION_0_3,
    pub FilterResourceRequirements: IO_STACK_LOCATION_0_6,
    pub ReadWriteConfig: IO_STACK_LOCATION_0_24,
    pub SetLock: IO_STACK_LOCATION_0_29,
    pub QueryId: IO_STACK_LOCATION_0_19,
    pub QueryDeviceText: IO_STACK_LOCATION_0_15,
    pub UsageNotification: IO_STACK_LOCATION_0_34,
    pub WaitWake: IO_STACK_LOCATION_0_37,
    pub PowerSequence: IO_STACK_LOCATION_0_12,
    pub Power: IO_STACK_LOCATION_0_13,
    pub StartDevice: IO_STACK_LOCATION_0_33,
    pub WMI: IO_STACK_LOCATION_0_36,
    pub Others: IO_STACK_LOCATION_0_11,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Copy for IO_STACK_LOCATION_0 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Clone for IO_STACK_LOCATION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for IO_STACK_LOCATION_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub struct IO_STACK_LOCATION_0_0 {
    pub SecurityContext: *mut IO_SECURITY_CONTEXT,
    pub Options: u32,
    pub Reserved: u16,
    pub ShareAccess: u16,
    pub Parameters: *mut super::System::SystemServices::MAILSLOT_CREATE_PARAMETERS,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Copy for IO_STACK_LOCATION_0_0 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Clone for IO_STACK_LOCATION_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl core::fmt::Debug for IO_STACK_LOCATION_0_0 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("IO_STACK_LOCATION_0_0").field("SecurityContext", &self.SecurityContext).field("Options", &self.Options).field("Reserved", &self.Reserved).field("ShareAccess", &self.ShareAccess).field("Parameters", &self.Parameters).finish()
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for IO_STACK_LOCATION_0_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl PartialEq for IO_STACK_LOCATION_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.SecurityContext == other.SecurityContext && self.Options == other.Options && self.Reserved == other.Reserved && self.ShareAccess == other.ShareAccess && self.Parameters == other.Parameters
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Eq for IO_STACK_LOCATION_0_0 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub struct IO_STACK_LOCATION_0_1 {
    pub SecurityContext: *mut IO_SECURITY_CONTEXT,
    pub Options: u32,
    pub Reserved: u16,
    pub ShareAccess: u16,
    pub Parameters: *mut super::System::SystemServices::NAMED_PIPE_CREATE_PARAMETERS,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Copy for IO_STACK_LOCATION_0_1 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Clone for IO_STACK_LOCATION_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl core::fmt::Debug for IO_STACK_LOCATION_0_1 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("IO_STACK_LOCATION_0_1").field("SecurityContext", &self.SecurityContext).field("Options", &self.Options).field("Reserved", &self.Reserved).field("ShareAccess", &self.ShareAccess).field("Parameters", &self.Parameters).finish()
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for IO_STACK_LOCATION_0_1 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl PartialEq for IO_STACK_LOCATION_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.SecurityContext == other.SecurityContext && self.Options == other.Options && self.Reserved == other.Reserved && self.ShareAccess == other.ShareAccess && self.Parameters == other.Parameters
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Eq for IO_STACK_LOCATION_0_1 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub struct IO_STACK_LOCATION_0_2 {
    pub SecurityContext: *mut IO_SECURITY_CONTEXT,
    pub Options: u32,
    pub FileAttributes: u16,
    pub ShareAccess: u16,
    pub EaLength: u32,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Copy for IO_STACK_LOCATION_0_2 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Clone for IO_STACK_LOCATION_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl core::fmt::Debug for IO_STACK_LOCATION_0_2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("IO_STACK_LOCATION_0_2").field("SecurityContext", &self.SecurityContext).field("Options", &self.Options).field("FileAttributes", &self.FileAttributes).field("ShareAccess", &self.ShareAccess).field("EaLength", &self.EaLength).finish()
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for IO_STACK_LOCATION_0_2 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl PartialEq for IO_STACK_LOCATION_0_2 {
    fn eq(&self, other: &Self) -> bool {
        self.SecurityContext == other.SecurityContext && self.Options == other.Options && self.FileAttributes == other.FileAttributes && self.ShareAccess == other.ShareAccess && self.EaLength == other.EaLength
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Eq for IO_STACK_LOCATION_0_2 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub struct IO_STACK_LOCATION_0_3 {
    pub Capabilities: *mut super::System::SystemServices::DEVICE_CAPABILITIES,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Copy for IO_STACK_LOCATION_0_3 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Clone for IO_STACK_LOCATION_0_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl core::fmt::Debug for IO_STACK_LOCATION_0_3 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("IO_STACK_LOCATION_0_3").field("Capabilities", &self.Capabilities).finish()
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for IO_STACK_LOCATION_0_3 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl PartialEq for IO_STACK_LOCATION_0_3 {
    fn eq(&self, other: &Self) -> bool {
        self.Capabilities == other.Capabilities
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Eq for IO_STACK_LOCATION_0_3 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub struct IO_STACK_LOCATION_0_4 {
    pub OutputBufferLength: u32,
    pub InputBufferLength: u32,
    pub IoControlCode: u32,
    pub Type3InputBuffer: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Copy for IO_STACK_LOCATION_0_4 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Clone for IO_STACK_LOCATION_0_4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl core::fmt::Debug for IO_STACK_LOCATION_0_4 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("IO_STACK_LOCATION_0_4").field("OutputBufferLength", &self.OutputBufferLength).field("InputBufferLength", &self.InputBufferLength).field("IoControlCode", &self.IoControlCode).field("Type3InputBuffer", &self.Type3InputBuffer).finish()
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for IO_STACK_LOCATION_0_4 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl PartialEq for IO_STACK_LOCATION_0_4 {
    fn eq(&self, other: &Self) -> bool {
        self.OutputBufferLength == other.OutputBufferLength && self.InputBufferLength == other.InputBufferLength && self.IoControlCode == other.IoControlCode && self.Type3InputBuffer == other.Type3InputBuffer
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Eq for IO_STACK_LOCATION_0_4 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub struct IO_STACK_LOCATION_0_5 {
    pub OutputBufferLength: u32,
    pub InputBufferLength: u32,
    pub FsControlCode: u32,
    pub Type3InputBuffer: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Copy for IO_STACK_LOCATION_0_5 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Clone for IO_STACK_LOCATION_0_5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl core::fmt::Debug for IO_STACK_LOCATION_0_5 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("IO_STACK_LOCATION_0_5").field("OutputBufferLength", &self.OutputBufferLength).field("InputBufferLength", &self.InputBufferLength).field("FsControlCode", &self.FsControlCode).field("Type3InputBuffer", &self.Type3InputBuffer).finish()
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for IO_STACK_LOCATION_0_5 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl PartialEq for IO_STACK_LOCATION_0_5 {
    fn eq(&self, other: &Self) -> bool {
        self.OutputBufferLength == other.OutputBufferLength && self.InputBufferLength == other.InputBufferLength && self.FsControlCode == other.FsControlCode && self.Type3InputBuffer == other.Type3InputBuffer
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Eq for IO_STACK_LOCATION_0_5 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub struct IO_STACK_LOCATION_0_6 {
    pub IoResourceRequirementList: *mut super::System::SystemServices::IO_RESOURCE_REQUIREMENTS_LIST,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Copy for IO_STACK_LOCATION_0_6 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Clone for IO_STACK_LOCATION_0_6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl core::fmt::Debug for IO_STACK_LOCATION_0_6 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("IO_STACK_LOCATION_0_6").field("IoResourceRequirementList", &self.IoResourceRequirementList).finish()
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for IO_STACK_LOCATION_0_6 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl PartialEq for IO_STACK_LOCATION_0_6 {
    fn eq(&self, other: &Self) -> bool {
        self.IoResourceRequirementList == other.IoResourceRequirementList
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Eq for IO_STACK_LOCATION_0_6 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub struct IO_STACK_LOCATION_0_7 {
    pub Length: *mut i64,
    pub Key: u32,
    pub ByteOffset: i64,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Copy for IO_STACK_LOCATION_0_7 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Clone for IO_STACK_LOCATION_0_7 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for IO_STACK_LOCATION_0_7 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_7 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub struct IO_STACK_LOCATION_0_8 {
    pub Vpb: *mut VPB,
    pub DeviceObject: *mut DEVICE_OBJECT,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Copy for IO_STACK_LOCATION_0_8 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Clone for IO_STACK_LOCATION_0_8 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl core::fmt::Debug for IO_STACK_LOCATION_0_8 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("IO_STACK_LOCATION_0_8").field("Vpb", &self.Vpb).field("DeviceObject", &self.DeviceObject).finish()
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for IO_STACK_LOCATION_0_8 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl PartialEq for IO_STACK_LOCATION_0_8 {
    fn eq(&self, other: &Self) -> bool {
        self.Vpb == other.Vpb && self.DeviceObject == other.DeviceObject
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Eq for IO_STACK_LOCATION_0_8 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_8 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub struct IO_STACK_LOCATION_0_9 {
    pub Length: u32,
    pub CompletionFilter: u32,
    pub DirectoryNotifyInformationClass: super::System::SystemServices::DIRECTORY_NOTIFY_INFORMATION_CLASS,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Copy for IO_STACK_LOCATION_0_9 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Clone for IO_STACK_LOCATION_0_9 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl core::fmt::Debug for IO_STACK_LOCATION_0_9 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("IO_STACK_LOCATION_0_9").field("Length", &self.Length).field("CompletionFilter", &self.CompletionFilter).field("DirectoryNotifyInformationClass", &self.DirectoryNotifyInformationClass).finish()
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for IO_STACK_LOCATION_0_9 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl PartialEq for IO_STACK_LOCATION_0_9 {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.CompletionFilter == other.CompletionFilter && self.DirectoryNotifyInformationClass == other.DirectoryNotifyInformationClass
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Eq for IO_STACK_LOCATION_0_9 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_9 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub struct IO_STACK_LOCATION_0_10 {
    pub Length: u32,
    pub CompletionFilter: u32,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Copy for IO_STACK_LOCATION_0_10 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Clone for IO_STACK_LOCATION_0_10 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl core::fmt::Debug for IO_STACK_LOCATION_0_10 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("IO_STACK_LOCATION_0_10").field("Length", &self.Length).field("CompletionFilter", &self.CompletionFilter).finish()
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for IO_STACK_LOCATION_0_10 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl PartialEq for IO_STACK_LOCATION_0_10 {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.CompletionFilter == other.CompletionFilter
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Eq for IO_STACK_LOCATION_0_10 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_10 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub struct IO_STACK_LOCATION_0_11 {
    pub Argument1: *mut core::ffi::c_void,
    pub Argument2: *mut core::ffi::c_void,
    pub Argument3: *mut core::ffi::c_void,
    pub Argument4: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Copy for IO_STACK_LOCATION_0_11 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Clone for IO_STACK_LOCATION_0_11 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl core::fmt::Debug for IO_STACK_LOCATION_0_11 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("IO_STACK_LOCATION_0_11").field("Argument1", &self.Argument1).field("Argument2", &self.Argument2).field("Argument3", &self.Argument3).field("Argument4", &self.Argument4).finish()
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for IO_STACK_LOCATION_0_11 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl PartialEq for IO_STACK_LOCATION_0_11 {
    fn eq(&self, other: &Self) -> bool {
        self.Argument1 == other.Argument1 && self.Argument2 == other.Argument2 && self.Argument3 == other.Argument3 && self.Argument4 == other.Argument4
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Eq for IO_STACK_LOCATION_0_11 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_11 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub struct IO_STACK_LOCATION_0_12 {
    pub PowerSequence: *mut super::System::SystemServices::POWER_SEQUENCE,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Copy for IO_STACK_LOCATION_0_12 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Clone for IO_STACK_LOCATION_0_12 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl core::fmt::Debug for IO_STACK_LOCATION_0_12 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("IO_STACK_LOCATION_0_12").field("PowerSequence", &self.PowerSequence).finish()
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for IO_STACK_LOCATION_0_12 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl PartialEq for IO_STACK_LOCATION_0_12 {
    fn eq(&self, other: &Self) -> bool {
        self.PowerSequence == other.PowerSequence
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Eq for IO_STACK_LOCATION_0_12 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_12 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub struct IO_STACK_LOCATION_0_13 {
    pub Anonymous: IO_STACK_LOCATION_0_13_0,
    pub Type: super::System::SystemServices::POWER_STATE_TYPE,
    pub State: super::System::SystemServices::POWER_STATE,
    pub ShutdownType: super::super::Win32::System::Power::POWER_ACTION,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Copy for IO_STACK_LOCATION_0_13 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Clone for IO_STACK_LOCATION_0_13 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for IO_STACK_LOCATION_0_13 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_13 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub union IO_STACK_LOCATION_0_13_0 {
    pub SystemContext: u32,
    pub SystemPowerStateContext: super::System::SystemServices::SYSTEM_POWER_STATE_CONTEXT,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Copy for IO_STACK_LOCATION_0_13_0 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Clone for IO_STACK_LOCATION_0_13_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for IO_STACK_LOCATION_0_13_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_13_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub struct IO_STACK_LOCATION_0_14 {
    pub Type: super::System::SystemServices::DEVICE_RELATION_TYPE,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Copy for IO_STACK_LOCATION_0_14 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Clone for IO_STACK_LOCATION_0_14 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl core::fmt::Debug for IO_STACK_LOCATION_0_14 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("IO_STACK_LOCATION_0_14").field("Type", &self.Type).finish()
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for IO_STACK_LOCATION_0_14 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl PartialEq for IO_STACK_LOCATION_0_14 {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Eq for IO_STACK_LOCATION_0_14 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_14 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub struct IO_STACK_LOCATION_0_15 {
    pub DeviceTextType: super::System::SystemServices::DEVICE_TEXT_TYPE,
    pub LocaleId: u32,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Copy for IO_STACK_LOCATION_0_15 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Clone for IO_STACK_LOCATION_0_15 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl core::fmt::Debug for IO_STACK_LOCATION_0_15 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("IO_STACK_LOCATION_0_15").field("DeviceTextType", &self.DeviceTextType).field("LocaleId", &self.LocaleId).finish()
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for IO_STACK_LOCATION_0_15 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl PartialEq for IO_STACK_LOCATION_0_15 {
    fn eq(&self, other: &Self) -> bool {
        self.DeviceTextType == other.DeviceTextType && self.LocaleId == other.LocaleId
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Eq for IO_STACK_LOCATION_0_15 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_15 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub struct IO_STACK_LOCATION_0_16 {
    pub Length: u32,
    pub FileName: *mut super::super::Win32::Foundation::UNICODE_STRING,
    pub FileInformationClass: super::Storage::FileSystem::FILE_INFORMATION_CLASS,
    pub FileIndex: u32,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Copy for IO_STACK_LOCATION_0_16 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Clone for IO_STACK_LOCATION_0_16 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl core::fmt::Debug for IO_STACK_LOCATION_0_16 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("IO_STACK_LOCATION_0_16").field("Length", &self.Length).field("FileName", &self.FileName).field("FileInformationClass", &self.FileInformationClass).field("FileIndex", &self.FileIndex).finish()
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for IO_STACK_LOCATION_0_16 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl PartialEq for IO_STACK_LOCATION_0_16 {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.FileName == other.FileName && self.FileInformationClass == other.FileInformationClass && self.FileIndex == other.FileIndex
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Eq for IO_STACK_LOCATION_0_16 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_16 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub struct IO_STACK_LOCATION_0_17 {
    pub Length: u32,
    pub EaList: *mut core::ffi::c_void,
    pub EaListLength: u32,
    pub EaIndex: u32,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Copy for IO_STACK_LOCATION_0_17 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Clone for IO_STACK_LOCATION_0_17 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl core::fmt::Debug for IO_STACK_LOCATION_0_17 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("IO_STACK_LOCATION_0_17").field("Length", &self.Length).field("EaList", &self.EaList).field("EaListLength", &self.EaListLength).field("EaIndex", &self.EaIndex).finish()
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for IO_STACK_LOCATION_0_17 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl PartialEq for IO_STACK_LOCATION_0_17 {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.EaList == other.EaList && self.EaListLength == other.EaListLength && self.EaIndex == other.EaIndex
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Eq for IO_STACK_LOCATION_0_17 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_17 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub struct IO_STACK_LOCATION_0_18 {
    pub Length: u32,
    pub FileInformationClass: super::Storage::FileSystem::FILE_INFORMATION_CLASS,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Copy for IO_STACK_LOCATION_0_18 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Clone for IO_STACK_LOCATION_0_18 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl core::fmt::Debug for IO_STACK_LOCATION_0_18 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("IO_STACK_LOCATION_0_18").field("Length", &self.Length).field("FileInformationClass", &self.FileInformationClass).finish()
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for IO_STACK_LOCATION_0_18 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl PartialEq for IO_STACK_LOCATION_0_18 {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.FileInformationClass == other.FileInformationClass
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Eq for IO_STACK_LOCATION_0_18 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_18 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub struct IO_STACK_LOCATION_0_19 {
    pub IdType: super::System::SystemServices::BUS_QUERY_ID_TYPE,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Copy for IO_STACK_LOCATION_0_19 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Clone for IO_STACK_LOCATION_0_19 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl core::fmt::Debug for IO_STACK_LOCATION_0_19 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("IO_STACK_LOCATION_0_19").field("IdType", &self.IdType).finish()
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for IO_STACK_LOCATION_0_19 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl PartialEq for IO_STACK_LOCATION_0_19 {
    fn eq(&self, other: &Self) -> bool {
        self.IdType == other.IdType
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Eq for IO_STACK_LOCATION_0_19 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_19 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub struct IO_STACK_LOCATION_0_20 {
    pub InterfaceType: *const windows_core::GUID,
    pub Size: u16,
    pub Version: u16,
    pub Interface: *mut super::System::SystemServices::INTERFACE,
    pub InterfaceSpecificData: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Copy for IO_STACK_LOCATION_0_20 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Clone for IO_STACK_LOCATION_0_20 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl core::fmt::Debug for IO_STACK_LOCATION_0_20 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("IO_STACK_LOCATION_0_20").field("InterfaceType", &self.InterfaceType).field("Size", &self.Size).field("Version", &self.Version).field("Interface", &self.Interface).field("InterfaceSpecificData", &self.InterfaceSpecificData).finish()
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for IO_STACK_LOCATION_0_20 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl PartialEq for IO_STACK_LOCATION_0_20 {
    fn eq(&self, other: &Self) -> bool {
        self.InterfaceType == other.InterfaceType && self.Size == other.Size && self.Version == other.Version && self.Interface == other.Interface && self.InterfaceSpecificData == other.InterfaceSpecificData
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Eq for IO_STACK_LOCATION_0_20 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_20 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub struct IO_STACK_LOCATION_0_21 {
    pub Length: u32,
    pub StartSid: super::super::Win32::Foundation::PSID,
    pub SidList: *mut super::Storage::FileSystem::FILE_GET_QUOTA_INFORMATION,
    pub SidListLength: u32,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Copy for IO_STACK_LOCATION_0_21 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Clone for IO_STACK_LOCATION_0_21 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl core::fmt::Debug for IO_STACK_LOCATION_0_21 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("IO_STACK_LOCATION_0_21").field("Length", &self.Length).field("StartSid", &self.StartSid).field("SidList", &self.SidList).field("SidListLength", &self.SidListLength).finish()
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for IO_STACK_LOCATION_0_21 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl PartialEq for IO_STACK_LOCATION_0_21 {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.StartSid == other.StartSid && self.SidList == other.SidList && self.SidListLength == other.SidListLength
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Eq for IO_STACK_LOCATION_0_21 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_21 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub struct IO_STACK_LOCATION_0_22 {
    pub SecurityInformation: u32,
    pub Length: u32,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Copy for IO_STACK_LOCATION_0_22 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Clone for IO_STACK_LOCATION_0_22 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl core::fmt::Debug for IO_STACK_LOCATION_0_22 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("IO_STACK_LOCATION_0_22").field("SecurityInformation", &self.SecurityInformation).field("Length", &self.Length).finish()
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for IO_STACK_LOCATION_0_22 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl PartialEq for IO_STACK_LOCATION_0_22 {
    fn eq(&self, other: &Self) -> bool {
        self.SecurityInformation == other.SecurityInformation && self.Length == other.Length
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Eq for IO_STACK_LOCATION_0_22 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_22 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub struct IO_STACK_LOCATION_0_23 {
    pub Length: u32,
    pub FsInformationClass: super::Storage::FileSystem::FS_INFORMATION_CLASS,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Copy for IO_STACK_LOCATION_0_23 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Clone for IO_STACK_LOCATION_0_23 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl core::fmt::Debug for IO_STACK_LOCATION_0_23 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("IO_STACK_LOCATION_0_23").field("Length", &self.Length).field("FsInformationClass", &self.FsInformationClass).finish()
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for IO_STACK_LOCATION_0_23 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl PartialEq for IO_STACK_LOCATION_0_23 {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.FsInformationClass == other.FsInformationClass
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Eq for IO_STACK_LOCATION_0_23 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_23 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub struct IO_STACK_LOCATION_0_24 {
    pub WhichSpace: u32,
    pub Buffer: *mut core::ffi::c_void,
    pub Offset: u32,
    pub Length: u32,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Copy for IO_STACK_LOCATION_0_24 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Clone for IO_STACK_LOCATION_0_24 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl core::fmt::Debug for IO_STACK_LOCATION_0_24 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("IO_STACK_LOCATION_0_24").field("WhichSpace", &self.WhichSpace).field("Buffer", &self.Buffer).field("Offset", &self.Offset).field("Length", &self.Length).finish()
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for IO_STACK_LOCATION_0_24 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl PartialEq for IO_STACK_LOCATION_0_24 {
    fn eq(&self, other: &Self) -> bool {
        self.WhichSpace == other.WhichSpace && self.Buffer == other.Buffer && self.Offset == other.Offset && self.Length == other.Length
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Eq for IO_STACK_LOCATION_0_24 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_24 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub struct IO_STACK_LOCATION_0_25 {
    pub Length: u32,
    pub Key: u32,
    pub ByteOffset: i64,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Copy for IO_STACK_LOCATION_0_25 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Clone for IO_STACK_LOCATION_0_25 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for IO_STACK_LOCATION_0_25 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_25 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub struct IO_STACK_LOCATION_0_26 {
    pub Srb: *mut _SCSI_REQUEST_BLOCK,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Copy for IO_STACK_LOCATION_0_26 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Clone for IO_STACK_LOCATION_0_26 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl core::fmt::Debug for IO_STACK_LOCATION_0_26 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("IO_STACK_LOCATION_0_26").field("Srb", &self.Srb).finish()
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for IO_STACK_LOCATION_0_26 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl PartialEq for IO_STACK_LOCATION_0_26 {
    fn eq(&self, other: &Self) -> bool {
        self.Srb == other.Srb
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Eq for IO_STACK_LOCATION_0_26 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_26 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub struct IO_STACK_LOCATION_0_27 {
    pub Length: u32,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Copy for IO_STACK_LOCATION_0_27 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Clone for IO_STACK_LOCATION_0_27 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl core::fmt::Debug for IO_STACK_LOCATION_0_27 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("IO_STACK_LOCATION_0_27").field("Length", &self.Length).finish()
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for IO_STACK_LOCATION_0_27 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl PartialEq for IO_STACK_LOCATION_0_27 {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Eq for IO_STACK_LOCATION_0_27 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_27 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub struct IO_STACK_LOCATION_0_28 {
    pub Length: u32,
    pub FileInformationClass: super::Storage::FileSystem::FILE_INFORMATION_CLASS,
    pub FileObject: *mut FILE_OBJECT,
    pub Anonymous: IO_STACK_LOCATION_0_28_0,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Copy for IO_STACK_LOCATION_0_28 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Clone for IO_STACK_LOCATION_0_28 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for IO_STACK_LOCATION_0_28 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_28 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub union IO_STACK_LOCATION_0_28_0 {
    pub Anonymous: IO_STACK_LOCATION_0_28_0_0,
    pub ClusterCount: u32,
    pub DeleteHandle: super::super::Win32::Foundation::HANDLE,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Copy for IO_STACK_LOCATION_0_28_0 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Clone for IO_STACK_LOCATION_0_28_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for IO_STACK_LOCATION_0_28_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_28_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub struct IO_STACK_LOCATION_0_28_0_0 {
    pub ReplaceIfExists: super::super::Win32::Foundation::BOOLEAN,
    pub AdvanceOnly: super::super::Win32::Foundation::BOOLEAN,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Copy for IO_STACK_LOCATION_0_28_0_0 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Clone for IO_STACK_LOCATION_0_28_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl core::fmt::Debug for IO_STACK_LOCATION_0_28_0_0 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("IO_STACK_LOCATION_0_28_0_0").field("ReplaceIfExists", &self.ReplaceIfExists).field("AdvanceOnly", &self.AdvanceOnly).finish()
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for IO_STACK_LOCATION_0_28_0_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl PartialEq for IO_STACK_LOCATION_0_28_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.ReplaceIfExists == other.ReplaceIfExists && self.AdvanceOnly == other.AdvanceOnly
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Eq for IO_STACK_LOCATION_0_28_0_0 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_28_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub struct IO_STACK_LOCATION_0_29 {
    pub Lock: super::super::Win32::Foundation::BOOLEAN,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Copy for IO_STACK_LOCATION_0_29 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Clone for IO_STACK_LOCATION_0_29 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl core::fmt::Debug for IO_STACK_LOCATION_0_29 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("IO_STACK_LOCATION_0_29").field("Lock", &self.Lock).finish()
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for IO_STACK_LOCATION_0_29 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl PartialEq for IO_STACK_LOCATION_0_29 {
    fn eq(&self, other: &Self) -> bool {
        self.Lock == other.Lock
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Eq for IO_STACK_LOCATION_0_29 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_29 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub struct IO_STACK_LOCATION_0_30 {
    pub Length: u32,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Copy for IO_STACK_LOCATION_0_30 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Clone for IO_STACK_LOCATION_0_30 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl core::fmt::Debug for IO_STACK_LOCATION_0_30 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("IO_STACK_LOCATION_0_30").field("Length", &self.Length).finish()
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for IO_STACK_LOCATION_0_30 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl PartialEq for IO_STACK_LOCATION_0_30 {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Eq for IO_STACK_LOCATION_0_30 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_30 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub struct IO_STACK_LOCATION_0_31 {
    pub SecurityInformation: u32,
    pub SecurityDescriptor: super::super::Win32::Security::PSECURITY_DESCRIPTOR,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Copy for IO_STACK_LOCATION_0_31 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Clone for IO_STACK_LOCATION_0_31 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl core::fmt::Debug for IO_STACK_LOCATION_0_31 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("IO_STACK_LOCATION_0_31").field("SecurityInformation", &self.SecurityInformation).field("SecurityDescriptor", &self.SecurityDescriptor).finish()
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for IO_STACK_LOCATION_0_31 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl PartialEq for IO_STACK_LOCATION_0_31 {
    fn eq(&self, other: &Self) -> bool {
        self.SecurityInformation == other.SecurityInformation && self.SecurityDescriptor == other.SecurityDescriptor
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Eq for IO_STACK_LOCATION_0_31 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_31 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub struct IO_STACK_LOCATION_0_32 {
    pub Length: u32,
    pub FsInformationClass: super::Storage::FileSystem::FS_INFORMATION_CLASS,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Copy for IO_STACK_LOCATION_0_32 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Clone for IO_STACK_LOCATION_0_32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl core::fmt::Debug for IO_STACK_LOCATION_0_32 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("IO_STACK_LOCATION_0_32").field("Length", &self.Length).field("FsInformationClass", &self.FsInformationClass).finish()
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for IO_STACK_LOCATION_0_32 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl PartialEq for IO_STACK_LOCATION_0_32 {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.FsInformationClass == other.FsInformationClass
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Eq for IO_STACK_LOCATION_0_32 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub struct IO_STACK_LOCATION_0_33 {
    pub AllocatedResources: *mut super::System::SystemServices::CM_RESOURCE_LIST,
    pub AllocatedResourcesTranslated: *mut super::System::SystemServices::CM_RESOURCE_LIST,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Copy for IO_STACK_LOCATION_0_33 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Clone for IO_STACK_LOCATION_0_33 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl core::fmt::Debug for IO_STACK_LOCATION_0_33 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("IO_STACK_LOCATION_0_33").field("AllocatedResources", &self.AllocatedResources).field("AllocatedResourcesTranslated", &self.AllocatedResourcesTranslated).finish()
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for IO_STACK_LOCATION_0_33 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl PartialEq for IO_STACK_LOCATION_0_33 {
    fn eq(&self, other: &Self) -> bool {
        self.AllocatedResources == other.AllocatedResources && self.AllocatedResourcesTranslated == other.AllocatedResourcesTranslated
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Eq for IO_STACK_LOCATION_0_33 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_33 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub struct IO_STACK_LOCATION_0_34 {
    pub InPath: super::super::Win32::Foundation::BOOLEAN,
    pub Reserved: [super::super::Win32::Foundation::BOOLEAN; 3],
    pub Type: super::System::SystemServices::DEVICE_USAGE_NOTIFICATION_TYPE,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Copy for IO_STACK_LOCATION_0_34 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Clone for IO_STACK_LOCATION_0_34 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl core::fmt::Debug for IO_STACK_LOCATION_0_34 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("IO_STACK_LOCATION_0_34").field("InPath", &self.InPath).field("Reserved", &self.Reserved).field("Type", &self.Type).finish()
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for IO_STACK_LOCATION_0_34 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl PartialEq for IO_STACK_LOCATION_0_34 {
    fn eq(&self, other: &Self) -> bool {
        self.InPath == other.InPath && self.Reserved == other.Reserved && self.Type == other.Type
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Eq for IO_STACK_LOCATION_0_34 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_34 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub struct IO_STACK_LOCATION_0_35 {
    pub Vpb: *mut VPB,
    pub DeviceObject: *mut DEVICE_OBJECT,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Copy for IO_STACK_LOCATION_0_35 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Clone for IO_STACK_LOCATION_0_35 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl core::fmt::Debug for IO_STACK_LOCATION_0_35 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("IO_STACK_LOCATION_0_35").field("Vpb", &self.Vpb).field("DeviceObject", &self.DeviceObject).finish()
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for IO_STACK_LOCATION_0_35 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl PartialEq for IO_STACK_LOCATION_0_35 {
    fn eq(&self, other: &Self) -> bool {
        self.Vpb == other.Vpb && self.DeviceObject == other.DeviceObject
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Eq for IO_STACK_LOCATION_0_35 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_35 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub struct IO_STACK_LOCATION_0_36 {
    pub ProviderId: usize,
    pub DataPath: *mut core::ffi::c_void,
    pub BufferSize: u32,
    pub Buffer: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Copy for IO_STACK_LOCATION_0_36 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Clone for IO_STACK_LOCATION_0_36 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl core::fmt::Debug for IO_STACK_LOCATION_0_36 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("IO_STACK_LOCATION_0_36").field("ProviderId", &self.ProviderId).field("DataPath", &self.DataPath).field("BufferSize", &self.BufferSize).field("Buffer", &self.Buffer).finish()
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for IO_STACK_LOCATION_0_36 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl PartialEq for IO_STACK_LOCATION_0_36 {
    fn eq(&self, other: &Self) -> bool {
        self.ProviderId == other.ProviderId && self.DataPath == other.DataPath && self.BufferSize == other.BufferSize && self.Buffer == other.Buffer
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Eq for IO_STACK_LOCATION_0_36 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_36 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub struct IO_STACK_LOCATION_0_37 {
    pub PowerState: super::super::Win32::System::Power::SYSTEM_POWER_STATE,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Copy for IO_STACK_LOCATION_0_37 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Clone for IO_STACK_LOCATION_0_37 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl core::fmt::Debug for IO_STACK_LOCATION_0_37 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("IO_STACK_LOCATION_0_37").field("PowerState", &self.PowerState).finish()
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for IO_STACK_LOCATION_0_37 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl PartialEq for IO_STACK_LOCATION_0_37 {
    fn eq(&self, other: &Self) -> bool {
        self.PowerState == other.PowerState
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Eq for IO_STACK_LOCATION_0_37 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_37 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub struct IO_STACK_LOCATION_0_38 {
    pub Length: u32,
    pub Key: u32,
    pub ByteOffset: i64,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Copy for IO_STACK_LOCATION_0_38 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Clone for IO_STACK_LOCATION_0_38 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for IO_STACK_LOCATION_0_38 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_38 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub struct IRP {
    pub Type: i16,
    pub Size: u16,
    pub MdlAddress: *mut MDL,
    pub Flags: u32,
    pub AssociatedIrp: IRP_1,
    pub ThreadListEntry: super::super::Win32::System::Kernel::LIST_ENTRY,
    pub IoStatus: super::super::Win32::System::IO::IO_STATUS_BLOCK,
    pub RequestorMode: i8,
    pub PendingReturned: super::super::Win32::Foundation::BOOLEAN,
    pub StackCount: i8,
    pub CurrentLocation: i8,
    pub Cancel: super::super::Win32::Foundation::BOOLEAN,
    pub CancelIrql: u8,
    pub ApcEnvironment: i8,
    pub AllocationFlags: u8,
    pub Anonymous: IRP_0,
    pub UserEvent: *mut KEVENT,
    pub Overlay: IRP_2,
    pub CancelRoutine: *mut DRIVER_CANCEL,
    pub UserBuffer: *mut core::ffi::c_void,
    pub Tail: IRP_3,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Copy for IRP {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Clone for IRP {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for IRP {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IRP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub union IRP_0 {
    pub UserIosb: *mut super::super::Win32::System::IO::IO_STATUS_BLOCK,
    pub IoRingContext: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Copy for IRP_0 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Clone for IRP_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for IRP_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IRP_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub union IRP_1 {
    pub MasterIrp: *mut IRP,
    pub IrpCount: i32,
    pub SystemBuffer: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Copy for IRP_1 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Clone for IRP_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for IRP_1 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IRP_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub union IRP_2 {
    pub AsynchronousParameters: IRP_2_0,
    pub AllocationSize: i64,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Copy for IRP_2 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Clone for IRP_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for IRP_2 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IRP_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub struct IRP_2_0 {
    pub Anonymous1: IRP_2_0_0,
    pub Anonymous2: IRP_2_0_1,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Copy for IRP_2_0 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Clone for IRP_2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for IRP_2_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IRP_2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub union IRP_2_0_0 {
    pub UserApcRoutine: super::super::Win32::System::IO::PIO_APC_ROUTINE,
    pub IssuingProcess: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Copy for IRP_2_0_0 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Clone for IRP_2_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for IRP_2_0_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IRP_2_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub union IRP_2_0_1 {
    pub UserApcContext: *mut core::ffi::c_void,
    pub IoRing: *mut _IORING_OBJECT,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Copy for IRP_2_0_1 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Clone for IRP_2_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for IRP_2_0_1 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IRP_2_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub union IRP_3 {
    pub Overlay: IRP_3_0,
    pub Apc: super::System::SystemServices::KAPC,
    pub CompletionKey: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Copy for IRP_3 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Clone for IRP_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for IRP_3 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IRP_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub struct IRP_3_0 {
    pub Anonymous1: IRP_3_0_0,
    pub Thread: PETHREAD,
    pub AuxiliaryBuffer: windows_core::PSTR,
    pub Anonymous2: IRP_3_0_1,
    pub OriginalFileObject: *mut FILE_OBJECT,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Copy for IRP_3_0 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Clone for IRP_3_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for IRP_3_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IRP_3_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub union IRP_3_0_0 {
    pub DeviceQueueEntry: super::System::SystemServices::KDEVICE_QUEUE_ENTRY,
    pub Anonymous: IRP_3_0_0_0,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Copy for IRP_3_0_0 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Clone for IRP_3_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for IRP_3_0_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IRP_3_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub struct IRP_3_0_0_0 {
    pub DriverContext: [*mut core::ffi::c_void; 4],
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Copy for IRP_3_0_0_0 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Clone for IRP_3_0_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl core::fmt::Debug for IRP_3_0_0_0 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("IRP_3_0_0_0").field("DriverContext", &self.DriverContext).finish()
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for IRP_3_0_0_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl PartialEq for IRP_3_0_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.DriverContext == other.DriverContext
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Eq for IRP_3_0_0_0 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IRP_3_0_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub struct IRP_3_0_1 {
    pub ListEntry: super::super::Win32::System::Kernel::LIST_ENTRY,
    pub Anonymous: IRP_3_0_1_0,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Copy for IRP_3_0_1 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Clone for IRP_3_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for IRP_3_0_1 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IRP_3_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub union IRP_3_0_1_0 {
    pub CurrentStackLocation: *mut IO_STACK_LOCATION,
    pub PacketType: u32,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Copy for IRP_3_0_1_0 {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Clone for IRP_3_0_1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for IRP_3_0_1_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IRP_3_0_1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
pub struct KDEVICE_QUEUE {
    pub Type: i16,
    pub Size: i16,
    pub DeviceListHead: super::super::Win32::System::Kernel::LIST_ENTRY,
    pub Lock: usize,
    pub Busy: super::super::Win32::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Copy for KDEVICE_QUEUE {}
#[cfg(feature = "Win32_System_Kernel")]
impl Clone for KDEVICE_QUEUE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl core::fmt::Debug for KDEVICE_QUEUE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("KDEVICE_QUEUE").field("Type", &self.Type).field("Size", &self.Size).field("DeviceListHead", &self.DeviceListHead).field("Lock", &self.Lock).field("Busy", &self.Busy).finish()
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl windows_core::TypeKind for KDEVICE_QUEUE {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Kernel")]
impl PartialEq for KDEVICE_QUEUE {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Size == other.Size && self.DeviceListHead == other.DeviceListHead && self.Lock == other.Lock && self.Busy == other.Busy
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl Eq for KDEVICE_QUEUE {}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for KDEVICE_QUEUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
pub struct KDPC {
    pub Anonymous: KDPC_0,
    pub DpcListEntry: super::super::Win32::System::Kernel::SINGLE_LIST_ENTRY,
    pub ProcessorHistory: usize,
    pub DeferredRoutine: PKDEFERRED_ROUTINE,
    pub DeferredContext: *mut core::ffi::c_void,
    pub SystemArgument1: *mut core::ffi::c_void,
    pub SystemArgument2: *mut core::ffi::c_void,
    pub DpcData: *mut core::ffi::c_void,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Copy for KDPC {}
#[cfg(feature = "Win32_System_Kernel")]
impl Clone for KDPC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl windows_core::TypeKind for KDPC {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for KDPC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
pub union KDPC_0 {
    pub TargetInfoAsUlong: u32,
    pub Anonymous: KDPC_0_0,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Copy for KDPC_0 {}
#[cfg(feature = "Win32_System_Kernel")]
impl Clone for KDPC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl windows_core::TypeKind for KDPC_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for KDPC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
pub struct KDPC_0_0 {
    pub Type: u8,
    pub Importance: u8,
    pub Number: u16,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Copy for KDPC_0_0 {}
#[cfg(feature = "Win32_System_Kernel")]
impl Clone for KDPC_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl core::fmt::Debug for KDPC_0_0 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("KDPC_0_0").field("Type", &self.Type).field("Importance", &self.Importance).field("Number", &self.Number).finish()
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl windows_core::TypeKind for KDPC_0_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Kernel")]
impl PartialEq for KDPC_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Importance == other.Importance && self.Number == other.Number
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl Eq for KDPC_0_0 {}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for KDPC_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct KENLISTMENT(pub isize);
impl Default for KENLISTMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for KENLISTMENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for KENLISTMENT {}
impl core::fmt::Debug for KENLISTMENT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("KENLISTMENT").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for KENLISTMENT {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
pub struct KEVENT {
    pub Header: DISPATCHER_HEADER,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Copy for KEVENT {}
#[cfg(feature = "Win32_System_Kernel")]
impl Clone for KEVENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl windows_core::TypeKind for KEVENT {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for KEVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct KGDT(pub isize);
impl Default for KGDT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for KGDT {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for KGDT {}
impl core::fmt::Debug for KGDT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("KGDT").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for KGDT {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct KIDT(pub isize);
impl Default for KIDT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for KIDT {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for KIDT {}
impl core::fmt::Debug for KIDT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("KIDT").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for KIDT {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
pub struct KMUTANT {
    pub Header: DISPATCHER_HEADER,
    pub MutantListEntry: super::super::Win32::System::Kernel::LIST_ENTRY,
    pub OwnerThread: *mut isize,
    pub Anonymous: KMUTANT_0,
    pub ApcDisable: u8,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Copy for KMUTANT {}
#[cfg(feature = "Win32_System_Kernel")]
impl Clone for KMUTANT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl windows_core::TypeKind for KMUTANT {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for KMUTANT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
pub union KMUTANT_0 {
    pub MutantFlags: u8,
    pub Anonymous: KMUTANT_0_0,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Copy for KMUTANT_0 {}
#[cfg(feature = "Win32_System_Kernel")]
impl Clone for KMUTANT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl windows_core::TypeKind for KMUTANT_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for KMUTANT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
pub struct KMUTANT_0_0 {
    pub _bitfield: u8,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Copy for KMUTANT_0_0 {}
#[cfg(feature = "Win32_System_Kernel")]
impl Clone for KMUTANT_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl core::fmt::Debug for KMUTANT_0_0 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("KMUTANT_0_0").field("_bitfield", &self._bitfield).finish()
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl windows_core::TypeKind for KMUTANT_0_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Kernel")]
impl PartialEq for KMUTANT_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl Eq for KMUTANT_0_0 {}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for KMUTANT_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct KPCR(pub isize);
impl Default for KPCR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for KPCR {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for KPCR {}
impl core::fmt::Debug for KPCR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("KPCR").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for KPCR {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct KPRCB(pub isize);
impl Default for KPRCB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for KPRCB {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for KPRCB {}
impl core::fmt::Debug for KPRCB {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("KPRCB").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for KPRCB {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
pub struct KQUEUE {
    pub Header: DISPATCHER_HEADER,
    pub EntryListHead: super::super::Win32::System::Kernel::LIST_ENTRY,
    pub CurrentCount: u32,
    pub MaximumCount: u32,
    pub ThreadListHead: super::super::Win32::System::Kernel::LIST_ENTRY,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Copy for KQUEUE {}
#[cfg(feature = "Win32_System_Kernel")]
impl Clone for KQUEUE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl windows_core::TypeKind for KQUEUE {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for KQUEUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct KRESOURCEMANAGER(pub isize);
impl Default for KRESOURCEMANAGER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for KRESOURCEMANAGER {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for KRESOURCEMANAGER {}
impl core::fmt::Debug for KRESOURCEMANAGER {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("KRESOURCEMANAGER").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for KRESOURCEMANAGER {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct KTM(pub isize);
impl Default for KTM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for KTM {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for KTM {}
impl core::fmt::Debug for KTM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("KTM").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for KTM {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct KTRANSACTION(pub isize);
impl Default for KTRANSACTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for KTRANSACTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for KTRANSACTION {}
impl core::fmt::Debug for KTRANSACTION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("KTRANSACTION").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for KTRANSACTION {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct KTSS(pub isize);
impl Default for KTSS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for KTSS {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for KTSS {}
impl core::fmt::Debug for KTSS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("KTSS").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for KTSS {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
pub struct KWAIT_BLOCK {
    pub WaitListEntry: super::super::Win32::System::Kernel::LIST_ENTRY,
    pub WaitType: u8,
    pub BlockState: u8,
    pub WaitKey: u16,
    pub Anonymous: KWAIT_BLOCK_0,
    pub Object: *mut core::ffi::c_void,
    pub SparePtr: *mut core::ffi::c_void,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Copy for KWAIT_BLOCK {}
#[cfg(feature = "Win32_System_Kernel")]
impl Clone for KWAIT_BLOCK {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl windows_core::TypeKind for KWAIT_BLOCK {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for KWAIT_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
pub union KWAIT_BLOCK_0 {
    pub Thread: *mut isize,
    pub NotificationQueue: *mut KQUEUE,
    pub Dpc: *mut KDPC,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Copy for KWAIT_BLOCK_0 {}
#[cfg(feature = "Win32_System_Kernel")]
impl Clone for KWAIT_BLOCK_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl windows_core::TypeKind for KWAIT_BLOCK_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for KWAIT_BLOCK_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct LOADER_PARAMETER_BLOCK(pub isize);
impl Default for LOADER_PARAMETER_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for LOADER_PARAMETER_BLOCK {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for LOADER_PARAMETER_BLOCK {}
impl core::fmt::Debug for LOADER_PARAMETER_BLOCK {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("LOADER_PARAMETER_BLOCK").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for LOADER_PARAMETER_BLOCK {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
pub struct MDL {
    pub Next: *mut MDL,
    pub Size: i16,
    pub MdlFlags: i16,
    pub Process: *mut isize,
    pub MappedSystemVa: *mut core::ffi::c_void,
    pub StartVa: *mut core::ffi::c_void,
    pub ByteCount: u32,
    pub ByteOffset: u32,
}
impl Copy for MDL {}
impl Clone for MDL {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for MDL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("MDL").field("Next", &self.Next).field("Size", &self.Size).field("MdlFlags", &self.MdlFlags).field("Process", &self.Process).field("MappedSystemVa", &self.MappedSystemVa).field("StartVa", &self.StartVa).field("ByteCount", &self.ByteCount).field("ByteOffset", &self.ByteOffset).finish()
    }
}
impl windows_core::TypeKind for MDL {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for MDL {
    fn eq(&self, other: &Self) -> bool {
        self.Next == other.Next && self.Size == other.Size && self.MdlFlags == other.MdlFlags && self.Process == other.Process && self.MappedSystemVa == other.MappedSystemVa && self.StartVa == other.StartVa && self.ByteCount == other.ByteCount && self.ByteOffset == other.ByteOffset
    }
}
impl Eq for MDL {}
impl Default for MDL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct OBJECT_ATTRIBUTES {
    pub Length: u32,
    pub RootDirectory: super::super::Win32::Foundation::HANDLE,
    pub ObjectName: *const super::super::Win32::Foundation::UNICODE_STRING,
    pub Attributes: u32,
    pub SecurityDescriptor: *const core::ffi::c_void,
    pub SecurityQualityOfService: *const core::ffi::c_void,
}
impl Copy for OBJECT_ATTRIBUTES {}
impl Clone for OBJECT_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for OBJECT_ATTRIBUTES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("OBJECT_ATTRIBUTES").field("Length", &self.Length).field("RootDirectory", &self.RootDirectory).field("ObjectName", &self.ObjectName).field("Attributes", &self.Attributes).field("SecurityDescriptor", &self.SecurityDescriptor).field("SecurityQualityOfService", &self.SecurityQualityOfService).finish()
    }
}
impl windows_core::TypeKind for OBJECT_ATTRIBUTES {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for OBJECT_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.RootDirectory == other.RootDirectory && self.ObjectName == other.ObjectName && self.Attributes == other.Attributes && self.SecurityDescriptor == other.SecurityDescriptor && self.SecurityQualityOfService == other.SecurityQualityOfService
    }
}
impl Eq for OBJECT_ATTRIBUTES {}
impl Default for OBJECT_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct OBJECT_ATTRIBUTES32 {
    pub Length: u32,
    pub RootDirectory: u32,
    pub ObjectName: u32,
    pub Attributes: u32,
    pub SecurityDescriptor: u32,
    pub SecurityQualityOfService: u32,
}
impl Copy for OBJECT_ATTRIBUTES32 {}
impl Clone for OBJECT_ATTRIBUTES32 {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for OBJECT_ATTRIBUTES32 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("OBJECT_ATTRIBUTES32").field("Length", &self.Length).field("RootDirectory", &self.RootDirectory).field("ObjectName", &self.ObjectName).field("Attributes", &self.Attributes).field("SecurityDescriptor", &self.SecurityDescriptor).field("SecurityQualityOfService", &self.SecurityQualityOfService).finish()
    }
}
impl windows_core::TypeKind for OBJECT_ATTRIBUTES32 {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for OBJECT_ATTRIBUTES32 {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.RootDirectory == other.RootDirectory && self.ObjectName == other.ObjectName && self.Attributes == other.Attributes && self.SecurityDescriptor == other.SecurityDescriptor && self.SecurityQualityOfService == other.SecurityQualityOfService
    }
}
impl Eq for OBJECT_ATTRIBUTES32 {}
impl Default for OBJECT_ATTRIBUTES32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct OBJECT_ATTRIBUTES64 {
    pub Length: u32,
    pub RootDirectory: u64,
    pub ObjectName: u64,
    pub Attributes: u32,
    pub SecurityDescriptor: u64,
    pub SecurityQualityOfService: u64,
}
impl Copy for OBJECT_ATTRIBUTES64 {}
impl Clone for OBJECT_ATTRIBUTES64 {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for OBJECT_ATTRIBUTES64 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("OBJECT_ATTRIBUTES64").field("Length", &self.Length).field("RootDirectory", &self.RootDirectory).field("ObjectName", &self.ObjectName).field("Attributes", &self.Attributes).field("SecurityDescriptor", &self.SecurityDescriptor).field("SecurityQualityOfService", &self.SecurityQualityOfService).finish()
    }
}
impl windows_core::TypeKind for OBJECT_ATTRIBUTES64 {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for OBJECT_ATTRIBUTES64 {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.RootDirectory == other.RootDirectory && self.ObjectName == other.ObjectName && self.Attributes == other.Attributes && self.SecurityDescriptor == other.SecurityDescriptor && self.SecurityQualityOfService == other.SecurityQualityOfService
    }
}
impl Eq for OBJECT_ATTRIBUTES64 {}
impl Default for OBJECT_ATTRIBUTES64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct OBJECT_NAME_INFORMATION {
    pub Name: super::super::Win32::Foundation::UNICODE_STRING,
}
impl Copy for OBJECT_NAME_INFORMATION {}
impl Clone for OBJECT_NAME_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for OBJECT_NAME_INFORMATION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("OBJECT_NAME_INFORMATION").field("Name", &self.Name).finish()
    }
}
impl windows_core::TypeKind for OBJECT_NAME_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for OBJECT_NAME_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Name == other.Name
    }
}
impl Eq for OBJECT_NAME_INFORMATION {}
impl Default for OBJECT_NAME_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct OWNER_ENTRY {
    pub OwnerThread: usize,
    pub Anonymous: OWNER_ENTRY_0,
}
impl Copy for OWNER_ENTRY {}
impl Clone for OWNER_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
impl windows_core::TypeKind for OWNER_ENTRY {
    type TypeKind = windows_core::CopyType;
}
impl Default for OWNER_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub union OWNER_ENTRY_0 {
    pub Anonymous: OWNER_ENTRY_0_0,
    pub TableSize: u32,
}
impl Copy for OWNER_ENTRY_0 {}
impl Clone for OWNER_ENTRY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl windows_core::TypeKind for OWNER_ENTRY_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for OWNER_ENTRY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct OWNER_ENTRY_0_0 {
    pub _bitfield: u32,
}
impl Copy for OWNER_ENTRY_0_0 {}
impl Clone for OWNER_ENTRY_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for OWNER_ENTRY_0_0 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("OWNER_ENTRY_0_0").field("_bitfield", &self._bitfield).finish()
    }
}
impl windows_core::TypeKind for OWNER_ENTRY_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for OWNER_ENTRY_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl Eq for OWNER_ENTRY_0_0 {}
impl Default for OWNER_ENTRY_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct PAFFINITY_TOKEN(pub isize);
impl Default for PAFFINITY_TOKEN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for PAFFINITY_TOKEN {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for PAFFINITY_TOKEN {}
impl core::fmt::Debug for PAFFINITY_TOKEN {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PAFFINITY_TOKEN").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for PAFFINITY_TOKEN {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct PBUS_HANDLER(pub isize);
impl Default for PBUS_HANDLER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for PBUS_HANDLER {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for PBUS_HANDLER {}
impl core::fmt::Debug for PBUS_HANDLER {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PBUS_HANDLER").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for PBUS_HANDLER {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct PCALLBACK_OBJECT(pub isize);
impl Default for PCALLBACK_OBJECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for PCALLBACK_OBJECT {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for PCALLBACK_OBJECT {}
impl core::fmt::Debug for PCALLBACK_OBJECT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PCALLBACK_OBJECT").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for PCALLBACK_OBJECT {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct PDEVICE_HANDLER_OBJECT(pub isize);
impl Default for PDEVICE_HANDLER_OBJECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for PDEVICE_HANDLER_OBJECT {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for PDEVICE_HANDLER_OBJECT {}
impl core::fmt::Debug for PDEVICE_HANDLER_OBJECT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PDEVICE_HANDLER_OBJECT").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for PDEVICE_HANDLER_OBJECT {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct PEJOB(pub isize);
impl Default for PEJOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for PEJOB {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for PEJOB {}
impl core::fmt::Debug for PEJOB {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PEJOB").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for PEJOB {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct PEPROCESS(pub isize);
impl Default for PEPROCESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for PEPROCESS {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for PEPROCESS {}
impl core::fmt::Debug for PEPROCESS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PEPROCESS").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for PEPROCESS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct PESILO(pub isize);
impl Default for PESILO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for PESILO {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for PESILO {}
impl core::fmt::Debug for PESILO {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PESILO").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for PESILO {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct PETHREAD(pub isize);
impl Default for PETHREAD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for PETHREAD {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for PETHREAD {}
impl core::fmt::Debug for PETHREAD {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PETHREAD").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for PETHREAD {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct PEX_RUNDOWN_REF_CACHE_AWARE(pub isize);
impl Default for PEX_RUNDOWN_REF_CACHE_AWARE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for PEX_RUNDOWN_REF_CACHE_AWARE {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for PEX_RUNDOWN_REF_CACHE_AWARE {}
impl core::fmt::Debug for PEX_RUNDOWN_REF_CACHE_AWARE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PEX_RUNDOWN_REF_CACHE_AWARE").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for PEX_RUNDOWN_REF_CACHE_AWARE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct PEX_TIMER(pub isize);
impl Default for PEX_TIMER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for PEX_TIMER {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for PEX_TIMER {}
impl core::fmt::Debug for PEX_TIMER {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PEX_TIMER").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for PEX_TIMER {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct PIO_REMOVE_LOCK_TRACKING_BLOCK(pub isize);
impl Default for PIO_REMOVE_LOCK_TRACKING_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for PIO_REMOVE_LOCK_TRACKING_BLOCK {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for PIO_REMOVE_LOCK_TRACKING_BLOCK {}
impl core::fmt::Debug for PIO_REMOVE_LOCK_TRACKING_BLOCK {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PIO_REMOVE_LOCK_TRACKING_BLOCK").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for PIO_REMOVE_LOCK_TRACKING_BLOCK {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct PIO_TIMER(pub isize);
impl Default for PIO_TIMER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for PIO_TIMER {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for PIO_TIMER {}
impl core::fmt::Debug for PIO_TIMER {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PIO_TIMER").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for PIO_TIMER {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct PIO_WORKITEM(pub isize);
impl Default for PIO_WORKITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for PIO_WORKITEM {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for PIO_WORKITEM {}
impl core::fmt::Debug for PIO_WORKITEM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PIO_WORKITEM").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for PIO_WORKITEM {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct PKINTERRUPT(pub isize);
impl Default for PKINTERRUPT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for PKINTERRUPT {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for PKINTERRUPT {}
impl core::fmt::Debug for PKINTERRUPT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PKINTERRUPT").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for PKINTERRUPT {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct PKPROCESS(pub isize);
impl Default for PKPROCESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for PKPROCESS {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for PKPROCESS {}
impl core::fmt::Debug for PKPROCESS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PKPROCESS").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for PKPROCESS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct PKTHREAD(pub isize);
impl Default for PKTHREAD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for PKTHREAD {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for PKTHREAD {}
impl core::fmt::Debug for PKTHREAD {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PKTHREAD").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for PKTHREAD {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct PNOTIFY_SYNC(pub isize);
impl Default for PNOTIFY_SYNC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for PNOTIFY_SYNC {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for PNOTIFY_SYNC {}
impl core::fmt::Debug for PNOTIFY_SYNC {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PNOTIFY_SYNC").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for PNOTIFY_SYNC {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct POBJECT_TYPE(pub isize);
impl Default for POBJECT_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for POBJECT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for POBJECT_TYPE {}
impl core::fmt::Debug for POBJECT_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("POBJECT_TYPE").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for POBJECT_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct POHANDLE(pub isize);
impl Default for POHANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for POHANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for POHANDLE {}
impl core::fmt::Debug for POHANDLE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("POHANDLE").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for POHANDLE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct PPCW_BUFFER(pub isize);
impl Default for PPCW_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for PPCW_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for PPCW_BUFFER {}
impl core::fmt::Debug for PPCW_BUFFER {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PPCW_BUFFER").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for PPCW_BUFFER {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct PPCW_INSTANCE(pub isize);
impl Default for PPCW_INSTANCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for PPCW_INSTANCE {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for PPCW_INSTANCE {}
impl core::fmt::Debug for PPCW_INSTANCE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PPCW_INSTANCE").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for PPCW_INSTANCE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct PPCW_REGISTRATION(pub isize);
impl Default for PPCW_REGISTRATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for PPCW_REGISTRATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for PPCW_REGISTRATION {}
impl core::fmt::Debug for PPCW_REGISTRATION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PPCW_REGISTRATION").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for PPCW_REGISTRATION {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct PRKPROCESS(pub isize);
impl Default for PRKPROCESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for PRKPROCESS {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for PRKPROCESS {}
impl core::fmt::Debug for PRKPROCESS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PRKPROCESS").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for PRKPROCESS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct PRKTHREAD(pub isize);
impl Default for PRKTHREAD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for PRKTHREAD {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for PRKTHREAD {}
impl core::fmt::Debug for PRKTHREAD {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PRKTHREAD").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for PRKTHREAD {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct PSILO_MONITOR(pub isize);
impl Default for PSILO_MONITOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for PSILO_MONITOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for PSILO_MONITOR {}
impl core::fmt::Debug for PSILO_MONITOR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PSILO_MONITOR").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for PSILO_MONITOR {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
pub struct RTL_SPLAY_LINKS {
    pub Parent: *mut RTL_SPLAY_LINKS,
    pub LeftChild: *mut RTL_SPLAY_LINKS,
    pub RightChild: *mut RTL_SPLAY_LINKS,
}
impl Copy for RTL_SPLAY_LINKS {}
impl Clone for RTL_SPLAY_LINKS {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for RTL_SPLAY_LINKS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("RTL_SPLAY_LINKS").field("Parent", &self.Parent).field("LeftChild", &self.LeftChild).field("RightChild", &self.RightChild).finish()
    }
}
impl windows_core::TypeKind for RTL_SPLAY_LINKS {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for RTL_SPLAY_LINKS {
    fn eq(&self, other: &Self) -> bool {
        self.Parent == other.Parent && self.LeftChild == other.LeftChild && self.RightChild == other.RightChild
    }
}
impl Eq for RTL_SPLAY_LINKS {}
impl Default for RTL_SPLAY_LINKS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct SECTION_OBJECT_POINTERS {
    pub DataSectionObject: *mut core::ffi::c_void,
    pub SharedCacheMap: *mut core::ffi::c_void,
    pub ImageSectionObject: *mut core::ffi::c_void,
}
impl Copy for SECTION_OBJECT_POINTERS {}
impl Clone for SECTION_OBJECT_POINTERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for SECTION_OBJECT_POINTERS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SECTION_OBJECT_POINTERS").field("DataSectionObject", &self.DataSectionObject).field("SharedCacheMap", &self.SharedCacheMap).field("ImageSectionObject", &self.ImageSectionObject).finish()
    }
}
impl windows_core::TypeKind for SECTION_OBJECT_POINTERS {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for SECTION_OBJECT_POINTERS {
    fn eq(&self, other: &Self) -> bool {
        self.DataSectionObject == other.DataSectionObject && self.SharedCacheMap == other.SharedCacheMap && self.ImageSectionObject == other.ImageSectionObject
    }
}
impl Eq for SECTION_OBJECT_POINTERS {}
impl Default for SECTION_OBJECT_POINTERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
pub struct SECURITY_SUBJECT_CONTEXT {
    pub ClientToken: *mut core::ffi::c_void,
    pub ImpersonationLevel: super::super::Win32::Security::SECURITY_IMPERSONATION_LEVEL,
    pub PrimaryToken: *mut core::ffi::c_void,
    pub ProcessAuditId: *mut core::ffi::c_void,
}
#[cfg(feature = "Win32_Security")]
impl Copy for SECURITY_SUBJECT_CONTEXT {}
#[cfg(feature = "Win32_Security")]
impl Clone for SECURITY_SUBJECT_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Security")]
impl core::fmt::Debug for SECURITY_SUBJECT_CONTEXT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SECURITY_SUBJECT_CONTEXT").field("ClientToken", &self.ClientToken).field("ImpersonationLevel", &self.ImpersonationLevel).field("PrimaryToken", &self.PrimaryToken).field("ProcessAuditId", &self.ProcessAuditId).finish()
    }
}
#[cfg(feature = "Win32_Security")]
impl windows_core::TypeKind for SECURITY_SUBJECT_CONTEXT {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security")]
impl PartialEq for SECURITY_SUBJECT_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.ClientToken == other.ClientToken && self.ImpersonationLevel == other.ImpersonationLevel && self.PrimaryToken == other.PrimaryToken && self.ProcessAuditId == other.ProcessAuditId
    }
}
#[cfg(feature = "Win32_Security")]
impl Eq for SECURITY_SUBJECT_CONTEXT {}
#[cfg(feature = "Win32_Security")]
impl Default for SECURITY_SUBJECT_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct SspiAsyncContext(pub isize);
impl Default for SspiAsyncContext {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for SspiAsyncContext {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for SspiAsyncContext {}
impl core::fmt::Debug for SspiAsyncContext {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SspiAsyncContext").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for SspiAsyncContext {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub struct TARGET_DEVICE_CUSTOM_NOTIFICATION {
    pub Version: u16,
    pub Size: u16,
    pub Event: windows_core::GUID,
    pub FileObject: *mut FILE_OBJECT,
    pub NameBufferOffset: i32,
    pub CustomDataBuffer: [u8; 1],
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Copy for TARGET_DEVICE_CUSTOM_NOTIFICATION {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Clone for TARGET_DEVICE_CUSTOM_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl core::fmt::Debug for TARGET_DEVICE_CUSTOM_NOTIFICATION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("TARGET_DEVICE_CUSTOM_NOTIFICATION").field("Version", &self.Version).field("Size", &self.Size).field("Event", &self.Event).field("FileObject", &self.FileObject).field("NameBufferOffset", &self.NameBufferOffset).field("CustomDataBuffer", &self.CustomDataBuffer).finish()
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for TARGET_DEVICE_CUSTOM_NOTIFICATION {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl PartialEq for TARGET_DEVICE_CUSTOM_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.Event == other.Event && self.FileObject == other.FileObject && self.NameBufferOffset == other.NameBufferOffset && self.CustomDataBuffer == other.CustomDataBuffer
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Eq for TARGET_DEVICE_CUSTOM_NOTIFICATION {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for TARGET_DEVICE_CUSTOM_NOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub struct VPB {
    pub Type: i16,
    pub Size: i16,
    pub Flags: u16,
    pub VolumeLabelLength: u16,
    pub DeviceObject: *mut DEVICE_OBJECT,
    pub RealDevice: *mut DEVICE_OBJECT,
    pub SerialNumber: u32,
    pub ReferenceCount: u32,
    pub VolumeLabel: [u16; 32],
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Copy for VPB {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Clone for VPB {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl core::fmt::Debug for VPB {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("VPB").field("Type", &self.Type).field("Size", &self.Size).field("Flags", &self.Flags).field("VolumeLabelLength", &self.VolumeLabelLength).field("DeviceObject", &self.DeviceObject).field("RealDevice", &self.RealDevice).field("SerialNumber", &self.SerialNumber).field("ReferenceCount", &self.ReferenceCount).field("VolumeLabel", &self.VolumeLabel).finish()
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for VPB {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl PartialEq for VPB {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Size == other.Size && self.Flags == other.Flags && self.VolumeLabelLength == other.VolumeLabelLength && self.DeviceObject == other.DeviceObject && self.RealDevice == other.RealDevice && self.SerialNumber == other.SerialNumber && self.ReferenceCount == other.ReferenceCount && self.VolumeLabel == other.VolumeLabel
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Eq for VPB {}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for VPB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
pub struct WORK_QUEUE_ITEM {
    pub List: super::super::Win32::System::Kernel::LIST_ENTRY,
    pub WorkerRoutine: PWORKER_THREAD_ROUTINE,
    pub Parameter: *mut core::ffi::c_void,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Copy for WORK_QUEUE_ITEM {}
#[cfg(feature = "Win32_System_Kernel")]
impl Clone for WORK_QUEUE_ITEM {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl core::fmt::Debug for WORK_QUEUE_ITEM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("WORK_QUEUE_ITEM").field("List", &self.List).field("Parameter", &self.Parameter).finish()
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl windows_core::TypeKind for WORK_QUEUE_ITEM {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for WORK_QUEUE_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct _DEVICE_OBJECT_POWER_EXTENSION(pub isize);
impl Default for _DEVICE_OBJECT_POWER_EXTENSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for _DEVICE_OBJECT_POWER_EXTENSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for _DEVICE_OBJECT_POWER_EXTENSION {}
impl core::fmt::Debug for _DEVICE_OBJECT_POWER_EXTENSION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("_DEVICE_OBJECT_POWER_EXTENSION").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for _DEVICE_OBJECT_POWER_EXTENSION {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct _IORING_OBJECT(pub isize);
impl Default for _IORING_OBJECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for _IORING_OBJECT {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for _IORING_OBJECT {}
impl core::fmt::Debug for _IORING_OBJECT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("_IORING_OBJECT").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for _IORING_OBJECT {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct _SCSI_REQUEST_BLOCK(pub isize);
impl Default for _SCSI_REQUEST_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for _SCSI_REQUEST_BLOCK {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for _SCSI_REQUEST_BLOCK {}
impl core::fmt::Debug for _SCSI_REQUEST_BLOCK {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("_SCSI_REQUEST_BLOCK").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for _SCSI_REQUEST_BLOCK {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type DRIVER_ADD_DEVICE = Option<unsafe extern "system" fn(driverobject: *const DRIVER_OBJECT, physicaldeviceobject: *const DEVICE_OBJECT) -> super::super::Win32::Foundation::NTSTATUS>;
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type DRIVER_CANCEL = Option<unsafe extern "system" fn(deviceobject: *mut DEVICE_OBJECT, irp: *mut IRP)>;
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type DRIVER_CONTROL = Option<unsafe extern "system" fn(deviceobject: *const DEVICE_OBJECT, irp: *mut IRP, mapregisterbase: *const core::ffi::c_void, context: *const core::ffi::c_void) -> super::System::SystemServices::IO_ALLOCATION_ACTION>;
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type DRIVER_DISPATCH = Option<unsafe extern "system" fn(deviceobject: *const DEVICE_OBJECT, irp: *mut IRP) -> super::super::Win32::Foundation::NTSTATUS>;
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type DRIVER_DISPATCH_PAGED = Option<unsafe extern "system" fn(deviceobject: *const DEVICE_OBJECT, irp: *mut IRP) -> super::super::Win32::Foundation::NTSTATUS>;
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type DRIVER_FS_NOTIFICATION = Option<unsafe extern "system" fn(deviceobject: *const DEVICE_OBJECT, fsactive: super::super::Win32::Foundation::BOOLEAN)>;
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type DRIVER_INITIALIZE = Option<unsafe extern "system" fn(driverobject: *const DRIVER_OBJECT, registrypath: *const super::super::Win32::Foundation::UNICODE_STRING) -> super::super::Win32::Foundation::NTSTATUS>;
pub type DRIVER_NOTIFICATION_CALLBACK_ROUTINE = Option<unsafe extern "system" fn(notificationstructure: *const core::ffi::c_void, context: *mut core::ffi::c_void) -> super::super::Win32::Foundation::NTSTATUS>;
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type DRIVER_REINITIALIZE = Option<unsafe extern "system" fn(driverobject: *const DRIVER_OBJECT, context: *const core::ffi::c_void, count: u32)>;
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type DRIVER_STARTIO = Option<unsafe extern "system" fn(deviceobject: *mut DEVICE_OBJECT, irp: *mut IRP)>;
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type DRIVER_UNLOAD = Option<unsafe extern "system" fn(driverobject: *const DRIVER_OBJECT)>;
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type FAST_IO_ACQUIRE_FILE = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT)>;
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type FAST_IO_ACQUIRE_FOR_CCFLUSH = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, deviceobject: *const DEVICE_OBJECT) -> super::super::Win32::Foundation::NTSTATUS>;
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type FAST_IO_ACQUIRE_FOR_MOD_WRITE = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, endingoffset: *const i64, resourcetorelease: *mut *mut ERESOURCE, deviceobject: *const DEVICE_OBJECT) -> super::super::Win32::Foundation::NTSTATUS>;
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type FAST_IO_CHECK_IF_POSSIBLE = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, fileoffset: *const i64, length: u32, wait: super::super::Win32::Foundation::BOOLEAN, lockkey: u32, checkforreadoperation: super::super::Win32::Foundation::BOOLEAN, iostatus: *mut super::super::Win32::System::IO::IO_STATUS_BLOCK, deviceobject: *const DEVICE_OBJECT) -> super::super::Win32::Foundation::BOOLEAN>;
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type FAST_IO_DETACH_DEVICE = Option<unsafe extern "system" fn(sourcedevice: *const DEVICE_OBJECT, targetdevice: *const DEVICE_OBJECT)>;
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type FAST_IO_DEVICE_CONTROL = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, wait: super::super::Win32::Foundation::BOOLEAN, inputbuffer: *const core::ffi::c_void, inputbufferlength: u32, outputbuffer: *mut core::ffi::c_void, outputbufferlength: u32, iocontrolcode: u32, iostatus: *mut super::super::Win32::System::IO::IO_STATUS_BLOCK, deviceobject: *const DEVICE_OBJECT) -> super::super::Win32::Foundation::BOOLEAN>;
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type FAST_IO_LOCK = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, fileoffset: *const i64, length: *const i64, processid: PEPROCESS, key: u32, failimmediately: super::super::Win32::Foundation::BOOLEAN, exclusivelock: super::super::Win32::Foundation::BOOLEAN, iostatus: *mut super::super::Win32::System::IO::IO_STATUS_BLOCK, deviceobject: *const DEVICE_OBJECT) -> super::super::Win32::Foundation::BOOLEAN>;
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type FAST_IO_MDL_READ = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, fileoffset: *const i64, length: u32, lockkey: u32, mdlchain: *mut *mut MDL, iostatus: *mut super::super::Win32::System::IO::IO_STATUS_BLOCK, deviceobject: *const DEVICE_OBJECT) -> super::super::Win32::Foundation::BOOLEAN>;
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type FAST_IO_MDL_READ_COMPLETE = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, mdlchain: *const MDL, deviceobject: *const DEVICE_OBJECT) -> super::super::Win32::Foundation::BOOLEAN>;
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type FAST_IO_MDL_READ_COMPLETE_COMPRESSED = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, mdlchain: *const MDL, deviceobject: *const DEVICE_OBJECT) -> super::super::Win32::Foundation::BOOLEAN>;
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type FAST_IO_MDL_WRITE_COMPLETE = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, fileoffset: *const i64, mdlchain: *const MDL, deviceobject: *const DEVICE_OBJECT) -> super::super::Win32::Foundation::BOOLEAN>;
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type FAST_IO_MDL_WRITE_COMPLETE_COMPRESSED = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, fileoffset: *const i64, mdlchain: *const MDL, deviceobject: *const DEVICE_OBJECT) -> super::super::Win32::Foundation::BOOLEAN>;
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type FAST_IO_PREPARE_MDL_WRITE = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, fileoffset: *const i64, length: u32, lockkey: u32, mdlchain: *mut *mut MDL, iostatus: *mut super::super::Win32::System::IO::IO_STATUS_BLOCK, deviceobject: *const DEVICE_OBJECT) -> super::super::Win32::Foundation::BOOLEAN>;
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type FAST_IO_QUERY_BASIC_INFO = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, wait: super::super::Win32::Foundation::BOOLEAN, buffer: *mut super::Storage::FileSystem::FILE_BASIC_INFORMATION, iostatus: *mut super::super::Win32::System::IO::IO_STATUS_BLOCK, deviceobject: *const DEVICE_OBJECT) -> super::super::Win32::Foundation::BOOLEAN>;
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type FAST_IO_QUERY_NETWORK_OPEN_INFO = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, wait: super::super::Win32::Foundation::BOOLEAN, buffer: *mut super::Storage::FileSystem::FILE_NETWORK_OPEN_INFORMATION, iostatus: *mut super::super::Win32::System::IO::IO_STATUS_BLOCK, deviceobject: *const DEVICE_OBJECT) -> super::super::Win32::Foundation::BOOLEAN>;
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type FAST_IO_QUERY_OPEN = Option<unsafe extern "system" fn(irp: *mut IRP, networkinformation: *mut super::Storage::FileSystem::FILE_NETWORK_OPEN_INFORMATION, deviceobject: *const DEVICE_OBJECT) -> super::super::Win32::Foundation::BOOLEAN>;
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type FAST_IO_QUERY_STANDARD_INFO = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, wait: super::super::Win32::Foundation::BOOLEAN, buffer: *mut super::Storage::FileSystem::FILE_STANDARD_INFORMATION, iostatus: *mut super::super::Win32::System::IO::IO_STATUS_BLOCK, deviceobject: *const DEVICE_OBJECT) -> super::super::Win32::Foundation::BOOLEAN>;
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type FAST_IO_READ = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, fileoffset: *const i64, length: u32, wait: super::super::Win32::Foundation::BOOLEAN, lockkey: u32, buffer: *mut core::ffi::c_void, iostatus: *mut super::super::Win32::System::IO::IO_STATUS_BLOCK, deviceobject: *const DEVICE_OBJECT) -> super::super::Win32::Foundation::BOOLEAN>;
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type FAST_IO_READ_COMPRESSED = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, fileoffset: *const i64, length: u32, lockkey: u32, buffer: *mut core::ffi::c_void, mdlchain: *mut *mut MDL, iostatus: *mut super::super::Win32::System::IO::IO_STATUS_BLOCK, compresseddatainfo: *mut super::Storage::FileSystem::COMPRESSED_DATA_INFO, compresseddatainfolength: u32, deviceobject: *const DEVICE_OBJECT) -> super::super::Win32::Foundation::BOOLEAN>;
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type FAST_IO_RELEASE_FILE = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT)>;
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type FAST_IO_RELEASE_FOR_CCFLUSH = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, deviceobject: *const DEVICE_OBJECT) -> super::super::Win32::Foundation::NTSTATUS>;
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type FAST_IO_RELEASE_FOR_MOD_WRITE = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, resourcetorelease: *const ERESOURCE, deviceobject: *const DEVICE_OBJECT) -> super::super::Win32::Foundation::NTSTATUS>;
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type FAST_IO_UNLOCK_ALL = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, processid: PEPROCESS, iostatus: *mut super::super::Win32::System::IO::IO_STATUS_BLOCK, deviceobject: *const DEVICE_OBJECT) -> super::super::Win32::Foundation::BOOLEAN>;
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type FAST_IO_UNLOCK_ALL_BY_KEY = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, processid: *const core::ffi::c_void, key: u32, iostatus: *mut super::super::Win32::System::IO::IO_STATUS_BLOCK, deviceobject: *const DEVICE_OBJECT) -> super::super::Win32::Foundation::BOOLEAN>;
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type FAST_IO_UNLOCK_SINGLE = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, fileoffset: *const i64, length: *const i64, processid: PEPROCESS, key: u32, iostatus: *mut super::super::Win32::System::IO::IO_STATUS_BLOCK, deviceobject: *const DEVICE_OBJECT) -> super::super::Win32::Foundation::BOOLEAN>;
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type FAST_IO_WRITE = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, fileoffset: *const i64, length: u32, wait: super::super::Win32::Foundation::BOOLEAN, lockkey: u32, buffer: *const core::ffi::c_void, iostatus: *mut super::super::Win32::System::IO::IO_STATUS_BLOCK, deviceobject: *const DEVICE_OBJECT) -> super::super::Win32::Foundation::BOOLEAN>;
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type FAST_IO_WRITE_COMPRESSED = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, fileoffset: *const i64, length: u32, lockkey: u32, buffer: *const core::ffi::c_void, mdlchain: *mut *mut MDL, iostatus: *mut super::super::Win32::System::IO::IO_STATUS_BLOCK, compresseddatainfo: *const super::Storage::FileSystem::COMPRESSED_DATA_INFO, compresseddatainfolength: u32, deviceobject: *const DEVICE_OBJECT) -> super::super::Win32::Foundation::BOOLEAN>;
pub type PFREE_FUNCTION = Option<unsafe extern "system" fn()>;
pub type PIO_COMPLETION_ROUTINE = Option<unsafe extern "system" fn() -> super::super::Win32::Foundation::NTSTATUS>;
pub type PKDEFERRED_ROUTINE = Option<unsafe extern "system" fn()>;
pub type PWORKER_THREAD_ROUTINE = Option<unsafe extern "system" fn()>;
