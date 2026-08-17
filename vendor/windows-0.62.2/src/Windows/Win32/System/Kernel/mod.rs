#[inline]
pub unsafe fn RtlFirstEntrySList(listhead: *const SLIST_HEADER) -> *mut SLIST_ENTRY {
    windows_core::link!("ntdll.dll" "system" fn RtlFirstEntrySList(listhead : *const SLIST_HEADER) -> *mut SLIST_ENTRY);
    unsafe { RtlFirstEntrySList(listhead) }
}
#[inline]
pub unsafe fn RtlInitializeSListHead() -> SLIST_HEADER {
    windows_core::link!("ntdll.dll" "system" fn RtlInitializeSListHead(listhead : *mut SLIST_HEADER));
    unsafe {
        let mut result__ = core::mem::zeroed();
        RtlInitializeSListHead(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn RtlInterlockedFlushSList(listhead: *mut SLIST_HEADER) -> *mut SLIST_ENTRY {
    windows_core::link!("ntdll.dll" "system" fn RtlInterlockedFlushSList(listhead : *mut SLIST_HEADER) -> *mut SLIST_ENTRY);
    unsafe { RtlInterlockedFlushSList(listhead as _) }
}
#[inline]
pub unsafe fn RtlInterlockedPopEntrySList(listhead: *mut SLIST_HEADER) -> *mut SLIST_ENTRY {
    windows_core::link!("ntdll.dll" "system" fn RtlInterlockedPopEntrySList(listhead : *mut SLIST_HEADER) -> *mut SLIST_ENTRY);
    unsafe { RtlInterlockedPopEntrySList(listhead as _) }
}
#[inline]
pub unsafe fn RtlInterlockedPushEntrySList(listhead: *mut SLIST_HEADER, listentry: *mut SLIST_ENTRY) -> *mut SLIST_ENTRY {
    windows_core::link!("ntdll.dll" "system" fn RtlInterlockedPushEntrySList(listhead : *mut SLIST_HEADER, listentry : *mut SLIST_ENTRY) -> *mut SLIST_ENTRY);
    unsafe { RtlInterlockedPushEntrySList(listhead as _, listentry as _) }
}
#[inline]
pub unsafe fn RtlInterlockedPushListSListEx(listhead: *mut SLIST_HEADER, list: *mut SLIST_ENTRY, listend: *mut SLIST_ENTRY, count: u32) -> *mut SLIST_ENTRY {
    windows_core::link!("ntdll.dll" "system" fn RtlInterlockedPushListSListEx(listhead : *mut SLIST_HEADER, list : *mut SLIST_ENTRY, listend : *mut SLIST_ENTRY, count : u32) -> *mut SLIST_ENTRY);
    unsafe { RtlInterlockedPushListSListEx(listhead as _, list as _, listend as _, count) }
}
#[inline]
pub unsafe fn RtlQueryDepthSList(listhead: *const SLIST_HEADER) -> u16 {
    windows_core::link!("ntdll.dll" "system" fn RtlQueryDepthSList(listhead : *const SLIST_HEADER) -> u16);
    unsafe { RtlQueryDepthSList(listhead) }
}
pub const BackOffice: SUITE_TYPE = SUITE_TYPE(2i32);
pub const Blade: SUITE_TYPE = SUITE_TYPE(10i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COMPARTMENT_ID(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CSTRING {
    pub Length: u16,
    pub MaximumLength: u16,
    pub Buffer: windows_core::PCSTR,
}
pub const CommunicationServer: SUITE_TYPE = SUITE_TYPE(3i32);
pub const ComputeServer: SUITE_TYPE = SUITE_TYPE(14i32);
pub const DEFAULT_COMPARTMENT_ID: COMPARTMENT_ID = COMPARTMENT_ID(1i32);
pub const DataCenter: SUITE_TYPE = SUITE_TYPE(7i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVENT_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EXCEPTION_DISPOSITION(pub i32);
#[repr(C)]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
#[derive(Clone, Copy, Debug)]
pub struct EXCEPTION_REGISTRATION_RECORD {
    pub Next: *mut EXCEPTION_REGISTRATION_RECORD,
    pub Handler: EXCEPTION_ROUTINE,
}
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl Default for EXCEPTION_REGISTRATION_RECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
pub type EXCEPTION_ROUTINE = Option<unsafe extern "system" fn(exceptionrecord: *mut super::Diagnostics::Debug::EXCEPTION_RECORD, establisherframe: *const core::ffi::c_void, contextrecord: *mut super::Diagnostics::Debug::CONTEXT, dispatchercontext: *const core::ffi::c_void) -> EXCEPTION_DISPOSITION>;
pub const EmbeddedNT: SUITE_TYPE = SUITE_TYPE(6i32);
pub const EmbeddedRestricted: SUITE_TYPE = SUITE_TYPE(11i32);
pub const Enterprise: SUITE_TYPE = SUITE_TYPE(1i32);
pub const ExceptionCollidedUnwind: EXCEPTION_DISPOSITION = EXCEPTION_DISPOSITION(3i32);
pub const ExceptionContinueExecution: EXCEPTION_DISPOSITION = EXCEPTION_DISPOSITION(0i32);
pub const ExceptionContinueSearch: EXCEPTION_DISPOSITION = EXCEPTION_DISPOSITION(1i32);
pub const ExceptionNestedException: EXCEPTION_DISPOSITION = EXCEPTION_DISPOSITION(2i32);
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, PartialEq)]
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
impl Default for FLOATING_SAVE_AREA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
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
impl Default for FLOATING_SAVE_AREA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LIST_ENTRY {
    pub Flink: *mut LIST_ENTRY,
    pub Blink: *mut LIST_ENTRY,
}
impl Default for LIST_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LIST_ENTRY32 {
    pub Flink: u32,
    pub Blink: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LIST_ENTRY64 {
    pub Flink: u64,
    pub Blink: u64,
}
pub const MAXUCHAR: u32 = 255u32;
pub const MAXULONG: u32 = 4294967295u32;
pub const MAXUSHORT: u32 = 65535u32;
pub const MaxSuiteType: SUITE_TYPE = SUITE_TYPE(18i32);
pub const MultiUserTS: SUITE_TYPE = SUITE_TYPE(17i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NT_PRODUCT_TYPE(pub i32);
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
impl Default for NT_TIB_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NULL64: u32 = 0u32;
pub const NotificationEvent: EVENT_TYPE = EVENT_TYPE(0i32);
pub const NotificationTimer: TIMER_TYPE = TIMER_TYPE(0i32);
pub const NtProductLanManNt: NT_PRODUCT_TYPE = NT_PRODUCT_TYPE(2i32);
pub const NtProductServer: NT_PRODUCT_TYPE = NT_PRODUCT_TYPE(3i32);
pub const NtProductWinNt: NT_PRODUCT_TYPE = NT_PRODUCT_TYPE(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct OBJECTID {
    pub Lineage: windows_core::GUID,
    pub Uniquifier: u32,
}
pub const OBJ_HANDLE_TAGBITS: i32 = 3i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROCESSOR_NUMBER {
    pub Group: u16,
    pub Number: u8,
    pub Reserved: u8,
}
pub const Personal: SUITE_TYPE = SUITE_TYPE(9i32);
pub const PhoneNT: SUITE_TYPE = SUITE_TYPE(16i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct QUAD {
    pub Anonymous: QUAD_0,
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
impl Default for RTL_BALANCED_NODE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RTL_BALANCED_NODE_0_0 {
    pub Left: *mut RTL_BALANCED_NODE,
    pub Right: *mut RTL_BALANCED_NODE,
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
impl Default for RTL_BALANCED_NODE_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RTL_BALANCED_NODE_RESERVED_PARENT_MASK: u32 = 3u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SINGLE_LIST_ENTRY {
    pub Next: *mut SINGLE_LIST_ENTRY,
}
impl Default for SINGLE_LIST_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SINGLE_LIST_ENTRY32 {
    pub Next: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SLIST_ENTRY {
    pub Next: *mut SLIST_ENTRY,
}
impl Default for SLIST_ENTRY {
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
impl Default for SLIST_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SLIST_HEADER_0 {
    pub Next: SINGLE_LIST_ENTRY,
    pub Depth: u16,
    pub CpuId: u16,
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union SLIST_HEADER {
    pub Anonymous: SLIST_HEADER_0,
    pub HeaderX64: SLIST_HEADER_1,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SLIST_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SLIST_HEADER_0 {
    pub Alignment: u64,
    pub Region: u64,
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SLIST_HEADER_1 {
    pub _bitfield1: u64,
    pub _bitfield2: u64,
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy)]
pub union SLIST_HEADER {
    pub Anonymous: SLIST_HEADER_0,
    pub HeaderArm64: SLIST_HEADER_1,
}
#[cfg(target_arch = "aarch64")]
impl Default for SLIST_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SLIST_HEADER_0 {
    pub Alignment: u64,
    pub Region: u64,
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SLIST_HEADER_1 {
    pub _bitfield1: u64,
    pub _bitfield2: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STRING {
    pub Length: u16,
    pub MaximumLength: u16,
    pub Buffer: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STRING32 {
    pub Length: u16,
    pub MaximumLength: u16,
    pub Buffer: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STRING64 {
    pub Length: u16,
    pub MaximumLength: u16,
    pub Buffer: u64,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SUITE_TYPE(pub i32);
pub const SecurityAppliance: SUITE_TYPE = SUITE_TYPE(12i32);
pub const SingleUserTS: SUITE_TYPE = SUITE_TYPE(8i32);
pub const SmallBusiness: SUITE_TYPE = SUITE_TYPE(0i32);
pub const SmallBusinessRestricted: SUITE_TYPE = SUITE_TYPE(5i32);
pub const StorageServer: SUITE_TYPE = SUITE_TYPE(13i32);
pub const SynchronizationEvent: EVENT_TYPE = EVENT_TYPE(1i32);
pub const SynchronizationTimer: TIMER_TYPE = TIMER_TYPE(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TIMER_TYPE(pub i32);
pub const TerminalServer: SUITE_TYPE = SUITE_TYPE(4i32);
pub const UNSPECIFIED_COMPARTMENT_ID: COMPARTMENT_ID = COMPARTMENT_ID(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WAIT_TYPE(pub i32);
pub const WHServer: SUITE_TYPE = SUITE_TYPE(15i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WNF_STATE_NAME {
    pub Data: [u32; 2],
}
impl Default for WNF_STATE_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WaitAll: WAIT_TYPE = WAIT_TYPE(0i32);
pub const WaitAny: WAIT_TYPE = WAIT_TYPE(1i32);
pub const WaitDequeue: WAIT_TYPE = WAIT_TYPE(3i32);
pub const WaitDpc: WAIT_TYPE = WAIT_TYPE(4i32);
pub const WaitNotification: WAIT_TYPE = WAIT_TYPE(2i32);
