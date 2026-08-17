#[inline]
pub unsafe fn RtlFirstEntrySList(listhead: *const SLIST_HEADER) -> *mut SLIST_ENTRY {
    windows_targets::link!("ntdll.dll" "system" fn RtlFirstEntrySList(listhead : *const SLIST_HEADER) -> *mut SLIST_ENTRY);
    RtlFirstEntrySList(listhead)
}
#[inline]
pub unsafe fn RtlInitializeSListHead() -> SLIST_HEADER {
    windows_targets::link!("ntdll.dll" "system" fn RtlInitializeSListHead(listhead : *mut SLIST_HEADER));
    let mut result__ = core::mem::zeroed();
    RtlInitializeSListHead(&mut result__);
    result__
}
#[inline]
pub unsafe fn RtlInterlockedFlushSList(listhead: *mut SLIST_HEADER) -> *mut SLIST_ENTRY {
    windows_targets::link!("ntdll.dll" "system" fn RtlInterlockedFlushSList(listhead : *mut SLIST_HEADER) -> *mut SLIST_ENTRY);
    RtlInterlockedFlushSList(listhead)
}
#[inline]
pub unsafe fn RtlInterlockedPopEntrySList(listhead: *mut SLIST_HEADER) -> *mut SLIST_ENTRY {
    windows_targets::link!("ntdll.dll" "system" fn RtlInterlockedPopEntrySList(listhead : *mut SLIST_HEADER) -> *mut SLIST_ENTRY);
    RtlInterlockedPopEntrySList(listhead)
}
#[inline]
pub unsafe fn RtlInterlockedPushEntrySList(listhead: *mut SLIST_HEADER, listentry: *mut SLIST_ENTRY) -> *mut SLIST_ENTRY {
    windows_targets::link!("ntdll.dll" "system" fn RtlInterlockedPushEntrySList(listhead : *mut SLIST_HEADER, listentry : *mut SLIST_ENTRY) -> *mut SLIST_ENTRY);
    RtlInterlockedPushEntrySList(listhead, listentry)
}
#[inline]
pub unsafe fn RtlInterlockedPushListSListEx(listhead: *mut SLIST_HEADER, list: *mut SLIST_ENTRY, listend: *mut SLIST_ENTRY, count: u32) -> *mut SLIST_ENTRY {
    windows_targets::link!("ntdll.dll" "system" fn RtlInterlockedPushListSListEx(listhead : *mut SLIST_HEADER, list : *mut SLIST_ENTRY, listend : *mut SLIST_ENTRY, count : u32) -> *mut SLIST_ENTRY);
    RtlInterlockedPushListSListEx(listhead, list, listend, count)
}
#[inline]
pub unsafe fn RtlQueryDepthSList(listhead: *const SLIST_HEADER) -> u16 {
    windows_targets::link!("ntdll.dll" "system" fn RtlQueryDepthSList(listhead : *const SLIST_HEADER) -> u16);
    RtlQueryDepthSList(listhead)
}
pub const BackOffice: SUITE_TYPE = SUITE_TYPE(2i32);
pub const Blade: SUITE_TYPE = SUITE_TYPE(10i32);
pub const CommunicationServer: SUITE_TYPE = SUITE_TYPE(3i32);
pub const ComputeServer: SUITE_TYPE = SUITE_TYPE(14i32);
pub const DEFAULT_COMPARTMENT_ID: COMPARTMENT_ID = COMPARTMENT_ID(1i32);
pub const DataCenter: SUITE_TYPE = SUITE_TYPE(7i32);
pub const EmbeddedNT: SUITE_TYPE = SUITE_TYPE(6i32);
pub const EmbeddedRestricted: SUITE_TYPE = SUITE_TYPE(11i32);
pub const Enterprise: SUITE_TYPE = SUITE_TYPE(1i32);
pub const ExceptionCollidedUnwind: EXCEPTION_DISPOSITION = EXCEPTION_DISPOSITION(3i32);
pub const ExceptionContinueExecution: EXCEPTION_DISPOSITION = EXCEPTION_DISPOSITION(0i32);
pub const ExceptionContinueSearch: EXCEPTION_DISPOSITION = EXCEPTION_DISPOSITION(1i32);
pub const ExceptionNestedException: EXCEPTION_DISPOSITION = EXCEPTION_DISPOSITION(2i32);
pub const MAXUCHAR: u32 = 255u32;
pub const MAXULONG: u32 = 4294967295u32;
pub const MAXUSHORT: u32 = 65535u32;
pub const MaxSuiteType: SUITE_TYPE = SUITE_TYPE(18i32);
pub const MultiUserTS: SUITE_TYPE = SUITE_TYPE(17i32);
pub const NULL64: u32 = 0u32;
pub const NotificationEvent: EVENT_TYPE = EVENT_TYPE(0i32);
pub const NotificationTimer: TIMER_TYPE = TIMER_TYPE(0i32);
pub const NtProductLanManNt: NT_PRODUCT_TYPE = NT_PRODUCT_TYPE(2i32);
pub const NtProductServer: NT_PRODUCT_TYPE = NT_PRODUCT_TYPE(3i32);
pub const NtProductWinNt: NT_PRODUCT_TYPE = NT_PRODUCT_TYPE(1i32);
pub const OBJ_CASE_INSENSITIVE: i32 = 64i32;
pub const OBJ_DONT_REPARSE: i32 = 4096i32;
pub const OBJ_EXCLUSIVE: i32 = 32i32;
pub const OBJ_FORCE_ACCESS_CHECK: i32 = 1024i32;
pub const OBJ_HANDLE_TAGBITS: i32 = 3i32;
pub const OBJ_IGNORE_IMPERSONATED_DEVICEMAP: i32 = 2048i32;
pub const OBJ_INHERIT: i32 = 2i32;
pub const OBJ_KERNEL_HANDLE: i32 = 512i32;
pub const OBJ_OPENIF: i32 = 128i32;
pub const OBJ_OPENLINK: i32 = 256i32;
pub const OBJ_PERMANENT: i32 = 16i32;
pub const OBJ_VALID_ATTRIBUTES: i32 = 8178i32;
pub const Personal: SUITE_TYPE = SUITE_TYPE(9i32);
pub const PhoneNT: SUITE_TYPE = SUITE_TYPE(16i32);
pub const RTL_BALANCED_NODE_RESERVED_PARENT_MASK: u32 = 3u32;
pub const SecurityAppliance: SUITE_TYPE = SUITE_TYPE(12i32);
pub const SingleUserTS: SUITE_TYPE = SUITE_TYPE(8i32);
pub const SmallBusiness: SUITE_TYPE = SUITE_TYPE(0i32);
pub const SmallBusinessRestricted: SUITE_TYPE = SUITE_TYPE(5i32);
pub const StorageServer: SUITE_TYPE = SUITE_TYPE(13i32);
pub const SynchronizationEvent: EVENT_TYPE = EVENT_TYPE(1i32);
pub const SynchronizationTimer: TIMER_TYPE = TIMER_TYPE(1i32);
pub const TerminalServer: SUITE_TYPE = SUITE_TYPE(4i32);
pub const UNSPECIFIED_COMPARTMENT_ID: COMPARTMENT_ID = COMPARTMENT_ID(0i32);
pub const WHServer: SUITE_TYPE = SUITE_TYPE(15i32);
pub const WaitAll: WAIT_TYPE = WAIT_TYPE(0i32);
pub const WaitAny: WAIT_TYPE = WAIT_TYPE(1i32);
pub const WaitDequeue: WAIT_TYPE = WAIT_TYPE(3i32);
pub const WaitDpc: WAIT_TYPE = WAIT_TYPE(4i32);
pub const WaitNotification: WAIT_TYPE = WAIT_TYPE(2i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct COMPARTMENT_ID(pub i32);
impl windows_core::TypeKind for COMPARTMENT_ID {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for COMPARTMENT_ID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("COMPARTMENT_ID").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct EVENT_TYPE(pub i32);
impl windows_core::TypeKind for EVENT_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for EVENT_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("EVENT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct EXCEPTION_DISPOSITION(pub i32);
impl windows_core::TypeKind for EXCEPTION_DISPOSITION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for EXCEPTION_DISPOSITION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("EXCEPTION_DISPOSITION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NT_PRODUCT_TYPE(pub i32);
impl windows_core::TypeKind for NT_PRODUCT_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NT_PRODUCT_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NT_PRODUCT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SUITE_TYPE(pub i32);
impl windows_core::TypeKind for SUITE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SUITE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SUITE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TIMER_TYPE(pub i32);
impl windows_core::TypeKind for TIMER_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TIMER_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TIMER_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WAIT_TYPE(pub i32);
impl windows_core::TypeKind for WAIT_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WAIT_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WAIT_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CSTRING {
    pub Length: u16,
    pub MaximumLength: u16,
    pub Buffer: windows_core::PCSTR,
}
impl windows_core::TypeKind for CSTRING {
    type TypeKind = windows_core::CopyType;
}
impl Default for CSTRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
#[derive(Clone, Copy, Debug)]
pub struct EXCEPTION_REGISTRATION_RECORD {
    pub Next: *mut EXCEPTION_REGISTRATION_RECORD,
    pub Handler: EXCEPTION_ROUTINE,
}
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl windows_core::TypeKind for EXCEPTION_REGISTRATION_RECORD {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl Default for EXCEPTION_REGISTRATION_RECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FLOATING_SAVE_AREA {
    pub ControlWord: u32,
    pub StatusWord: u32,
    pub TagWord: u32,
    pub ErrorOffset: u32,
    pub ErrorSelector: u32,
    pub DataOffset: u32,
    pub DataSelector: u32,
    pub RegisterArea: [u8; 80],
    pub Cr0NpxState: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for FLOATING_SAVE_AREA {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for FLOATING_SAVE_AREA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FLOATING_SAVE_AREA {
    pub ControlWord: u32,
    pub StatusWord: u32,
    pub TagWord: u32,
    pub ErrorOffset: u32,
    pub ErrorSelector: u32,
    pub DataOffset: u32,
    pub DataSelector: u32,
    pub RegisterArea: [u8; 80],
    pub Spare0: u32,
}
#[cfg(target_arch = "x86")]
impl windows_core::TypeKind for FLOATING_SAVE_AREA {
    type TypeKind = windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
impl Default for FLOATING_SAVE_AREA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LIST_ENTRY {
    pub Flink: *mut LIST_ENTRY,
    pub Blink: *mut LIST_ENTRY,
}
impl windows_core::TypeKind for LIST_ENTRY {
    type TypeKind = windows_core::CopyType;
}
impl Default for LIST_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LIST_ENTRY32 {
    pub Flink: u32,
    pub Blink: u32,
}
impl windows_core::TypeKind for LIST_ENTRY32 {
    type TypeKind = windows_core::CopyType;
}
impl Default for LIST_ENTRY32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LIST_ENTRY64 {
    pub Flink: u64,
    pub Blink: u64,
}
impl windows_core::TypeKind for LIST_ENTRY64 {
    type TypeKind = windows_core::CopyType;
}
impl Default for LIST_ENTRY64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
#[derive(Clone, Copy)]
pub struct NT_TIB {
    pub ExceptionList: *mut EXCEPTION_REGISTRATION_RECORD,
    pub StackBase: *mut core::ffi::c_void,
    pub StackLimit: *mut core::ffi::c_void,
    pub SubSystemTib: *mut core::ffi::c_void,
    pub Anonymous: NT_TIB_0,
    pub ArbitraryUserPointer: *mut core::ffi::c_void,
    pub Self_: *mut NT_TIB,
}
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl windows_core::TypeKind for NT_TIB {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl Default for NT_TIB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
#[derive(Clone, Copy)]
pub union NT_TIB_0 {
    pub FiberData: *mut core::ffi::c_void,
    pub Version: u32,
}
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl windows_core::TypeKind for NT_TIB_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl Default for NT_TIB_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OBJECTID {
    pub Lineage: windows_core::GUID,
    pub Uniquifier: u32,
}
impl windows_core::TypeKind for OBJECTID {
    type TypeKind = windows_core::CopyType;
}
impl Default for OBJECTID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PROCESSOR_NUMBER {
    pub Group: u16,
    pub Number: u8,
    pub Reserved: u8,
}
impl windows_core::TypeKind for PROCESSOR_NUMBER {
    type TypeKind = windows_core::CopyType;
}
impl Default for PROCESSOR_NUMBER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct QUAD {
    pub Anonymous: QUAD_0,
}
impl windows_core::TypeKind for QUAD {
    type TypeKind = windows_core::CopyType;
}
impl Default for QUAD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union QUAD_0 {
    pub UseThisFieldToCopy: i64,
    pub DoNotUseThisField: f64,
}
impl windows_core::TypeKind for QUAD_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for QUAD_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RTL_BALANCED_NODE {
    pub Anonymous1: RTL_BALANCED_NODE_0,
    pub Anonymous2: RTL_BALANCED_NODE_1,
}
impl windows_core::TypeKind for RTL_BALANCED_NODE {
    type TypeKind = windows_core::CopyType;
}
impl Default for RTL_BALANCED_NODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union RTL_BALANCED_NODE_0 {
    pub Children: [*mut RTL_BALANCED_NODE; 2],
    pub Anonymous: RTL_BALANCED_NODE_0_0,
}
impl windows_core::TypeKind for RTL_BALANCED_NODE_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for RTL_BALANCED_NODE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RTL_BALANCED_NODE_0_0 {
    pub Left: *mut RTL_BALANCED_NODE,
    pub Right: *mut RTL_BALANCED_NODE,
}
impl windows_core::TypeKind for RTL_BALANCED_NODE_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for RTL_BALANCED_NODE_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union RTL_BALANCED_NODE_1 {
    pub _bitfield: u8,
    pub ParentValue: usize,
}
impl windows_core::TypeKind for RTL_BALANCED_NODE_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for RTL_BALANCED_NODE_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SINGLE_LIST_ENTRY {
    pub Next: *mut SINGLE_LIST_ENTRY,
}
impl windows_core::TypeKind for SINGLE_LIST_ENTRY {
    type TypeKind = windows_core::CopyType;
}
impl Default for SINGLE_LIST_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SINGLE_LIST_ENTRY32 {
    pub Next: u32,
}
impl windows_core::TypeKind for SINGLE_LIST_ENTRY32 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SINGLE_LIST_ENTRY32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SLIST_ENTRY {
    pub Next: *mut SLIST_ENTRY,
}
impl windows_core::TypeKind for SLIST_ENTRY {
    type TypeKind = windows_core::CopyType;
}
impl Default for SLIST_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy)]
pub union SLIST_HEADER {
    pub Anonymous: SLIST_HEADER_0,
    pub HeaderArm64: SLIST_HEADER_1,
}
#[cfg(target_arch = "aarch64")]
impl windows_core::TypeKind for SLIST_HEADER {
    type TypeKind = windows_core::CopyType;
}
#[cfg(target_arch = "aarch64")]
impl Default for SLIST_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SLIST_HEADER_0 {
    pub Alignment: u64,
    pub Region: u64,
}
#[cfg(target_arch = "aarch64")]
impl windows_core::TypeKind for SLIST_HEADER_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(target_arch = "aarch64")]
impl Default for SLIST_HEADER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SLIST_HEADER_1 {
    pub _bitfield1: u64,
    pub _bitfield2: u64,
}
#[cfg(target_arch = "aarch64")]
impl windows_core::TypeKind for SLIST_HEADER_1 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(target_arch = "aarch64")]
impl Default for SLIST_HEADER_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union SLIST_HEADER {
    pub Anonymous: SLIST_HEADER_0,
    pub HeaderX64: SLIST_HEADER_1,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for SLIST_HEADER {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SLIST_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SLIST_HEADER_0 {
    pub Alignment: u64,
    pub Region: u64,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for SLIST_HEADER_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SLIST_HEADER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SLIST_HEADER_1 {
    pub _bitfield1: u64,
    pub _bitfield2: u64,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for SLIST_HEADER_1 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SLIST_HEADER_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub union SLIST_HEADER {
    pub Alignment: u64,
    pub Anonymous: SLIST_HEADER_0,
}
#[cfg(target_arch = "x86")]
impl windows_core::TypeKind for SLIST_HEADER {
    type TypeKind = windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
impl Default for SLIST_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SLIST_HEADER_0 {
    pub Next: SINGLE_LIST_ENTRY,
    pub Depth: u16,
    pub CpuId: u16,
}
#[cfg(target_arch = "x86")]
impl windows_core::TypeKind for SLIST_HEADER_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
impl Default for SLIST_HEADER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct STRING {
    pub Length: u16,
    pub MaximumLength: u16,
    pub Buffer: windows_core::PSTR,
}
impl windows_core::TypeKind for STRING {
    type TypeKind = windows_core::CopyType;
}
impl Default for STRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct STRING32 {
    pub Length: u16,
    pub MaximumLength: u16,
    pub Buffer: u32,
}
impl windows_core::TypeKind for STRING32 {
    type TypeKind = windows_core::CopyType;
}
impl Default for STRING32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct STRING64 {
    pub Length: u16,
    pub MaximumLength: u16,
    pub Buffer: u64,
}
impl windows_core::TypeKind for STRING64 {
    type TypeKind = windows_core::CopyType;
}
impl Default for STRING64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WNF_STATE_NAME {
    pub Data: [u32; 2],
}
impl windows_core::TypeKind for WNF_STATE_NAME {
    type TypeKind = windows_core::CopyType;
}
impl Default for WNF_STATE_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
pub type EXCEPTION_ROUTINE = Option<unsafe extern "system" fn(exceptionrecord: *mut super::Diagnostics::Debug::EXCEPTION_RECORD, establisherframe: *const core::ffi::c_void, contextrecord: *mut super::Diagnostics::Debug::CONTEXT, dispatchercontext: *const core::ffi::c_void) -> EXCEPTION_DISPOSITION>;
