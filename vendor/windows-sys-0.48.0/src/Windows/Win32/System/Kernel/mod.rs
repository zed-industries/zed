::windows_targets::link ! ( "ntdll.dll""system" #[doc = "*Required features: `\"Win32_System_Kernel\"`*"] fn RtlFirstEntrySList ( listhead : *const SLIST_HEADER ) -> *mut SLIST_ENTRY );
::windows_targets::link ! ( "ntdll.dll""system" #[doc = "*Required features: `\"Win32_System_Kernel\"`*"] fn RtlInitializeSListHead ( listhead : *mut SLIST_HEADER ) -> ( ) );
::windows_targets::link ! ( "ntdll.dll""system" #[doc = "*Required features: `\"Win32_System_Kernel\"`*"] fn RtlInterlockedFlushSList ( listhead : *mut SLIST_HEADER ) -> *mut SLIST_ENTRY );
::windows_targets::link ! ( "ntdll.dll""system" #[doc = "*Required features: `\"Win32_System_Kernel\"`*"] fn RtlInterlockedPopEntrySList ( listhead : *mut SLIST_HEADER ) -> *mut SLIST_ENTRY );
::windows_targets::link ! ( "ntdll.dll""system" #[doc = "*Required features: `\"Win32_System_Kernel\"`*"] fn RtlInterlockedPushEntrySList ( listhead : *mut SLIST_HEADER , listentry : *mut SLIST_ENTRY ) -> *mut SLIST_ENTRY );
::windows_targets::link ! ( "ntdll.dll""system" #[doc = "*Required features: `\"Win32_System_Kernel\"`*"] fn RtlInterlockedPushListSListEx ( listhead : *mut SLIST_HEADER , list : *mut SLIST_ENTRY , listend : *mut SLIST_ENTRY , count : u32 ) -> *mut SLIST_ENTRY );
::windows_targets::link ! ( "ntdll.dll""system" #[doc = "*Required features: `\"Win32_System_Kernel\"`*"] fn RtlQueryDepthSList ( listhead : *const SLIST_HEADER ) -> u16 );
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const MAXUCHAR: u32 = 255u32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const MAXULONG: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const MAXUSHORT: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const NULL64: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const OBJ_CASE_INSENSITIVE: i32 = 64i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const OBJ_DONT_REPARSE: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const OBJ_EXCLUSIVE: i32 = 32i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const OBJ_FORCE_ACCESS_CHECK: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const OBJ_HANDLE_TAGBITS: i32 = 3i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const OBJ_IGNORE_IMPERSONATED_DEVICEMAP: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const OBJ_INHERIT: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const OBJ_KERNEL_HANDLE: i32 = 512i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const OBJ_OPENIF: i32 = 128i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const OBJ_OPENLINK: i32 = 256i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const OBJ_PERMANENT: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const OBJ_VALID_ATTRIBUTES: i32 = 8178i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const RTL_BALANCED_NODE_RESERVED_PARENT_MASK: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub type COMPARTMENT_ID = i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const UNSPECIFIED_COMPARTMENT_ID: COMPARTMENT_ID = 0i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const DEFAULT_COMPARTMENT_ID: COMPARTMENT_ID = 1i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub type EVENT_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const NotificationEvent: EVENT_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const SynchronizationEvent: EVENT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub type EXCEPTION_DISPOSITION = i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const ExceptionContinueExecution: EXCEPTION_DISPOSITION = 0i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const ExceptionContinueSearch: EXCEPTION_DISPOSITION = 1i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const ExceptionNestedException: EXCEPTION_DISPOSITION = 2i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const ExceptionCollidedUnwind: EXCEPTION_DISPOSITION = 3i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub type NT_PRODUCT_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const NtProductWinNt: NT_PRODUCT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const NtProductLanManNt: NT_PRODUCT_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const NtProductServer: NT_PRODUCT_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub type SUITE_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const SmallBusiness: SUITE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const Enterprise: SUITE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const BackOffice: SUITE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const CommunicationServer: SUITE_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const TerminalServer: SUITE_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const SmallBusinessRestricted: SUITE_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const EmbeddedNT: SUITE_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const DataCenter: SUITE_TYPE = 7i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const SingleUserTS: SUITE_TYPE = 8i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const Personal: SUITE_TYPE = 9i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const Blade: SUITE_TYPE = 10i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const EmbeddedRestricted: SUITE_TYPE = 11i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const SecurityAppliance: SUITE_TYPE = 12i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const StorageServer: SUITE_TYPE = 13i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const ComputeServer: SUITE_TYPE = 14i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const WHServer: SUITE_TYPE = 15i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const PhoneNT: SUITE_TYPE = 16i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const MultiUserTS: SUITE_TYPE = 17i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const MaxSuiteType: SUITE_TYPE = 18i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub type TIMER_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const NotificationTimer: TIMER_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const SynchronizationTimer: TIMER_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub type WAIT_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const WaitAll: WAIT_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const WaitAny: WAIT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const WaitNotification: WAIT_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const WaitDequeue: WAIT_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const WaitDpc: WAIT_TYPE = 4i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub struct CSTRING {
    pub Length: u16,
    pub MaximumLength: u16,
    pub Buffer: ::windows_sys::core::PCSTR,
}
impl ::core::marker::Copy for CSTRING {}
impl ::core::clone::Clone for CSTRING {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`, `\"Win32_Foundation\"`, `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
pub struct EXCEPTION_REGISTRATION_RECORD {
    pub Next: *mut EXCEPTION_REGISTRATION_RECORD,
    pub Handler: EXCEPTION_ROUTINE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
impl ::core::marker::Copy for EXCEPTION_REGISTRATION_RECORD {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
impl ::core::clone::Clone for EXCEPTION_REGISTRATION_RECORD {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
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
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for FLOATING_SAVE_AREA {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for FLOATING_SAVE_AREA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
#[cfg(target_arch = "x86")]
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
impl ::core::marker::Copy for FLOATING_SAVE_AREA {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for FLOATING_SAVE_AREA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub struct LIST_ENTRY {
    pub Flink: *mut LIST_ENTRY,
    pub Blink: *mut LIST_ENTRY,
}
impl ::core::marker::Copy for LIST_ENTRY {}
impl ::core::clone::Clone for LIST_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub struct LIST_ENTRY32 {
    pub Flink: u32,
    pub Blink: u32,
}
impl ::core::marker::Copy for LIST_ENTRY32 {}
impl ::core::clone::Clone for LIST_ENTRY32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub struct LIST_ENTRY64 {
    pub Flink: u64,
    pub Blink: u64,
}
impl ::core::marker::Copy for LIST_ENTRY64 {}
impl ::core::clone::Clone for LIST_ENTRY64 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`, `\"Win32_Foundation\"`, `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
pub struct NT_TIB {
    pub ExceptionList: *mut EXCEPTION_REGISTRATION_RECORD,
    pub StackBase: *mut ::core::ffi::c_void,
    pub StackLimit: *mut ::core::ffi::c_void,
    pub SubSystemTib: *mut ::core::ffi::c_void,
    pub Anonymous: NT_TIB_0,
    pub ArbitraryUserPointer: *mut ::core::ffi::c_void,
    pub Self_: *mut NT_TIB,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
impl ::core::marker::Copy for NT_TIB {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
impl ::core::clone::Clone for NT_TIB {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`, `\"Win32_Foundation\"`, `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
pub union NT_TIB_0 {
    pub FiberData: *mut ::core::ffi::c_void,
    pub Version: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
impl ::core::marker::Copy for NT_TIB_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
impl ::core::clone::Clone for NT_TIB_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub struct OBJECTID {
    pub Lineage: ::windows_sys::core::GUID,
    pub Uniquifier: u32,
}
impl ::core::marker::Copy for OBJECTID {}
impl ::core::clone::Clone for OBJECTID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub struct OBJECT_ATTRIBUTES32 {
    pub Length: u32,
    pub RootDirectory: u32,
    pub ObjectName: u32,
    pub Attributes: u32,
    pub SecurityDescriptor: u32,
    pub SecurityQualityOfService: u32,
}
impl ::core::marker::Copy for OBJECT_ATTRIBUTES32 {}
impl ::core::clone::Clone for OBJECT_ATTRIBUTES32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub struct OBJECT_ATTRIBUTES64 {
    pub Length: u32,
    pub RootDirectory: u64,
    pub ObjectName: u64,
    pub Attributes: u32,
    pub SecurityDescriptor: u64,
    pub SecurityQualityOfService: u64,
}
impl ::core::marker::Copy for OBJECT_ATTRIBUTES64 {}
impl ::core::clone::Clone for OBJECT_ATTRIBUTES64 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub struct PROCESSOR_NUMBER {
    pub Group: u16,
    pub Number: u8,
    pub Reserved: u8,
}
impl ::core::marker::Copy for PROCESSOR_NUMBER {}
impl ::core::clone::Clone for PROCESSOR_NUMBER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub struct QUAD {
    pub Anonymous: QUAD_0,
}
impl ::core::marker::Copy for QUAD {}
impl ::core::clone::Clone for QUAD {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub union QUAD_0 {
    pub UseThisFieldToCopy: i64,
    pub DoNotUseThisField: f64,
}
impl ::core::marker::Copy for QUAD_0 {}
impl ::core::clone::Clone for QUAD_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub struct RTL_BALANCED_NODE {
    pub Anonymous1: RTL_BALANCED_NODE_0,
    pub Anonymous2: RTL_BALANCED_NODE_1,
}
impl ::core::marker::Copy for RTL_BALANCED_NODE {}
impl ::core::clone::Clone for RTL_BALANCED_NODE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub union RTL_BALANCED_NODE_0 {
    pub Children: [*mut RTL_BALANCED_NODE; 2],
    pub Anonymous: RTL_BALANCED_NODE_0_0,
}
impl ::core::marker::Copy for RTL_BALANCED_NODE_0 {}
impl ::core::clone::Clone for RTL_BALANCED_NODE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub struct RTL_BALANCED_NODE_0_0 {
    pub Left: *mut RTL_BALANCED_NODE,
    pub Right: *mut RTL_BALANCED_NODE,
}
impl ::core::marker::Copy for RTL_BALANCED_NODE_0_0 {}
impl ::core::clone::Clone for RTL_BALANCED_NODE_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub union RTL_BALANCED_NODE_1 {
    pub _bitfield: u8,
    pub ParentValue: usize,
}
impl ::core::marker::Copy for RTL_BALANCED_NODE_1 {}
impl ::core::clone::Clone for RTL_BALANCED_NODE_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub struct SINGLE_LIST_ENTRY {
    pub Next: *mut SINGLE_LIST_ENTRY,
}
impl ::core::marker::Copy for SINGLE_LIST_ENTRY {}
impl ::core::clone::Clone for SINGLE_LIST_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub struct SINGLE_LIST_ENTRY32 {
    pub Next: u32,
}
impl ::core::marker::Copy for SINGLE_LIST_ENTRY32 {}
impl ::core::clone::Clone for SINGLE_LIST_ENTRY32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub struct SLIST_ENTRY {
    pub Next: *mut SLIST_ENTRY,
}
impl ::core::marker::Copy for SLIST_ENTRY {}
impl ::core::clone::Clone for SLIST_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
#[cfg(target_arch = "aarch64")]
pub union SLIST_HEADER {
    pub Anonymous: SLIST_HEADER_0,
    pub HeaderArm64: SLIST_HEADER_1,
}
#[cfg(target_arch = "aarch64")]
impl ::core::marker::Copy for SLIST_HEADER {}
#[cfg(target_arch = "aarch64")]
impl ::core::clone::Clone for SLIST_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
#[cfg(target_arch = "aarch64")]
pub struct SLIST_HEADER_0 {
    pub Alignment: u64,
    pub Region: u64,
}
#[cfg(target_arch = "aarch64")]
impl ::core::marker::Copy for SLIST_HEADER_0 {}
#[cfg(target_arch = "aarch64")]
impl ::core::clone::Clone for SLIST_HEADER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
#[cfg(target_arch = "aarch64")]
pub struct SLIST_HEADER_1 {
    pub _bitfield1: u64,
    pub _bitfield2: u64,
}
#[cfg(target_arch = "aarch64")]
impl ::core::marker::Copy for SLIST_HEADER_1 {}
#[cfg(target_arch = "aarch64")]
impl ::core::clone::Clone for SLIST_HEADER_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
#[cfg(target_arch = "x86_64")]
pub union SLIST_HEADER {
    pub Anonymous: SLIST_HEADER_0,
    pub HeaderX64: SLIST_HEADER_1,
}
#[cfg(target_arch = "x86_64")]
impl ::core::marker::Copy for SLIST_HEADER {}
#[cfg(target_arch = "x86_64")]
impl ::core::clone::Clone for SLIST_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
#[cfg(target_arch = "x86_64")]
pub struct SLIST_HEADER_0 {
    pub Alignment: u64,
    pub Region: u64,
}
#[cfg(target_arch = "x86_64")]
impl ::core::marker::Copy for SLIST_HEADER_0 {}
#[cfg(target_arch = "x86_64")]
impl ::core::clone::Clone for SLIST_HEADER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
#[cfg(target_arch = "x86_64")]
pub struct SLIST_HEADER_1 {
    pub _bitfield1: u64,
    pub _bitfield2: u64,
}
#[cfg(target_arch = "x86_64")]
impl ::core::marker::Copy for SLIST_HEADER_1 {}
#[cfg(target_arch = "x86_64")]
impl ::core::clone::Clone for SLIST_HEADER_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
#[cfg(target_arch = "x86")]
pub union SLIST_HEADER {
    pub Alignment: u64,
    pub Anonymous: SLIST_HEADER_0,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for SLIST_HEADER {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for SLIST_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
#[cfg(target_arch = "x86")]
pub struct SLIST_HEADER_0 {
    pub Next: SINGLE_LIST_ENTRY,
    pub Depth: u16,
    pub CpuId: u16,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for SLIST_HEADER_0 {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for SLIST_HEADER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub struct STRING {
    pub Length: u16,
    pub MaximumLength: u16,
    pub Buffer: ::windows_sys::core::PSTR,
}
impl ::core::marker::Copy for STRING {}
impl ::core::clone::Clone for STRING {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub struct STRING32 {
    pub Length: u16,
    pub MaximumLength: u16,
    pub Buffer: u32,
}
impl ::core::marker::Copy for STRING32 {}
impl ::core::clone::Clone for STRING32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub struct STRING64 {
    pub Length: u16,
    pub MaximumLength: u16,
    pub Buffer: u64,
}
impl ::core::marker::Copy for STRING64 {}
impl ::core::clone::Clone for STRING64 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub struct WNF_STATE_NAME {
    pub Data: [u32; 2],
}
impl ::core::marker::Copy for WNF_STATE_NAME {}
impl ::core::clone::Clone for WNF_STATE_NAME {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Kernel\"`, `\"Win32_Foundation\"`, `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
pub type EXCEPTION_ROUTINE = ::core::option::Option<unsafe extern "system" fn(exceptionrecord: *mut super::Diagnostics::Debug::EXCEPTION_RECORD, establisherframe: *const ::core::ffi::c_void, contextrecord: *mut super::Diagnostics::Debug::CONTEXT, dispatchercontext: *const ::core::ffi::c_void) -> EXCEPTION_DISPOSITION>;
