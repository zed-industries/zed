windows_link::link!("ntdll.dll" "system" fn NtClose(handle : super::super::Win32::Foundation:: HANDLE) -> super::super::Win32::Foundation:: NTSTATUS);
windows_link::link!("ntdll.dll" "system" fn NtQueryObject(handle : super::super::Win32::Foundation:: HANDLE, objectinformationclass : OBJECT_INFORMATION_CLASS, objectinformation : *mut core::ffi::c_void, objectinformationlength : u32, returnlength : *mut u32) -> super::super::Win32::Foundation:: NTSTATUS);
#[repr(C)]
#[cfg(all(feature = "Wdk_System_SystemServices", feature = "Win32_Security"))]
#[derive(Clone, Copy)]
pub struct ACCESS_STATE {
    pub OperationID: super::super::Win32::Foundation::LUID,
    pub SecurityEvaluated: bool,
    pub GenerateAudit: bool,
    pub GenerateOnClose: bool,
    pub PrivilegesAllocated: bool,
    pub Flags: u32,
    pub RemainingDesiredAccess: u32,
    pub PreviouslyGrantedAccess: u32,
    pub OriginalDesiredAccess: u32,
    pub SubjectSecurityContext: SECURITY_SUBJECT_CONTEXT,
    pub SecurityDescriptor: super::super::Win32::Security::PSECURITY_DESCRIPTOR,
    pub AuxData: *mut core::ffi::c_void,
    pub Privileges: ACCESS_STATE_0,
    pub AuditPrivileges: bool,
    pub ObjectName: super::super::Win32::Foundation::UNICODE_STRING,
    pub ObjectTypeName: super::super::Win32::Foundation::UNICODE_STRING,
}
#[cfg(all(feature = "Wdk_System_SystemServices", feature = "Win32_Security"))]
impl Default for ACCESS_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_System_SystemServices", feature = "Win32_Security"))]
#[derive(Clone, Copy)]
pub union ACCESS_STATE_0 {
    pub InitialPrivilegeSet: super::System::SystemServices::INITIAL_PRIVILEGE_SET,
    pub PrivilegeSet: super::super::Win32::Security::PRIVILEGE_SET,
}
#[cfg(all(feature = "Wdk_System_SystemServices", feature = "Win32_Security"))]
impl Default for ACCESS_STATE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
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
impl Default for DEVICE_OBJECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub union DEVICE_OBJECT_0 {
    pub ListEntry: super::super::Win32::System::Kernel::LIST_ENTRY,
    pub Wcb: super::System::SystemServices::WAIT_CONTEXT_BLOCK,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for DEVICE_OBJECT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
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
impl Default for DEVOBJ_EXTENSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy)]
pub struct DISPATCHER_HEADER {
    pub Anonymous: DISPATCHER_HEADER_0,
    pub SignalState: i32,
    pub WaitListHead: super::super::Win32::System::Kernel::LIST_ENTRY,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for DISPATCHER_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy)]
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
impl Default for DISPATCHER_HEADER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy)]
pub union DISPATCHER_HEADER_0_0 {
    pub Lock: i32,
    pub LockNV: i32,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for DISPATCHER_HEADER_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy, Default)]
pub struct DISPATCHER_HEADER_0_1 {
    pub Type: u8,
    pub Signalling: u8,
    pub Size: u8,
    pub Reserved1: u8,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy)]
pub struct DISPATCHER_HEADER_0_2 {
    pub TimerType: u8,
    pub Anonymous1: DISPATCHER_HEADER_0_2_0,
    pub Hand: u8,
    pub Anonymous2: DISPATCHER_HEADER_0_2_1,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for DISPATCHER_HEADER_0_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy)]
pub union DISPATCHER_HEADER_0_2_0 {
    pub TimerControlFlags: u8,
    pub Anonymous: DISPATCHER_HEADER_0_2_0_0,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for DISPATCHER_HEADER_0_2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy, Default)]
pub struct DISPATCHER_HEADER_0_2_0_0 {
    pub _bitfield: u8,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy)]
pub union DISPATCHER_HEADER_0_2_1 {
    pub TimerMiscFlags: u8,
    pub Anonymous: DISPATCHER_HEADER_0_2_1_0,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for DISPATCHER_HEADER_0_2_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy, Default)]
pub struct DISPATCHER_HEADER_0_2_1_0 {
    pub _bitfield: u8,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy)]
pub struct DISPATCHER_HEADER_0_3 {
    pub Timer2Type: u8,
    pub Anonymous: DISPATCHER_HEADER_0_3_0,
    pub Timer2ComponentId: u8,
    pub Timer2RelativeId: u8,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for DISPATCHER_HEADER_0_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy)]
pub union DISPATCHER_HEADER_0_3_0 {
    pub Timer2Flags: u8,
    pub Anonymous: DISPATCHER_HEADER_0_3_0_0,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for DISPATCHER_HEADER_0_3_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy, Default)]
pub struct DISPATCHER_HEADER_0_3_0_0 {
    pub _bitfield: u8,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy)]
pub struct DISPATCHER_HEADER_0_4 {
    pub QueueType: u8,
    pub Anonymous: DISPATCHER_HEADER_0_4_0,
    pub QueueSize: u8,
    pub QueueReserved: u8,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for DISPATCHER_HEADER_0_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy)]
pub union DISPATCHER_HEADER_0_4_0 {
    pub QueueControlFlags: u8,
    pub Anonymous: DISPATCHER_HEADER_0_4_0_0,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for DISPATCHER_HEADER_0_4_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy, Default)]
pub struct DISPATCHER_HEADER_0_4_0_0 {
    pub _bitfield: u8,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy)]
pub struct DISPATCHER_HEADER_0_5 {
    pub ThreadType: u8,
    pub ThreadReserved: u8,
    pub Anonymous1: DISPATCHER_HEADER_0_5_0,
    pub Anonymous2: DISPATCHER_HEADER_0_5_1,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for DISPATCHER_HEADER_0_5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy)]
pub union DISPATCHER_HEADER_0_5_0 {
    pub ThreadControlFlags: u8,
    pub Anonymous: DISPATCHER_HEADER_0_5_0_0,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for DISPATCHER_HEADER_0_5_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy, Default)]
pub struct DISPATCHER_HEADER_0_5_0_0 {
    pub _bitfield: u8,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy)]
pub union DISPATCHER_HEADER_0_5_1 {
    pub DebugActive: u8,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for DISPATCHER_HEADER_0_5_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy, Default)]
pub struct DISPATCHER_HEADER_0_6 {
    pub MutantType: u8,
    pub MutantSize: u8,
    pub DpcActive: bool,
    pub MutantReserved: u8,
}
pub type DMA_COMMON_BUFFER_VECTOR = isize;
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
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct DRIVER_EXTENSION {
    pub DriverObject: *mut DRIVER_OBJECT,
    pub AddDevice: DRIVER_ADD_DEVICE,
    pub Count: u32,
    pub ServiceKeyName: super::super::Win32::Foundation::UNICODE_STRING,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for DRIVER_EXTENSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type DRIVER_FS_NOTIFICATION = Option<unsafe extern "system" fn(deviceobject: *const DEVICE_OBJECT, fsactive: bool)>;
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type DRIVER_INITIALIZE = Option<unsafe extern "system" fn(driverobject: *const DRIVER_OBJECT, registrypath: *const super::super::Win32::Foundation::UNICODE_STRING) -> super::super::Win32::Foundation::NTSTATUS>;
pub type DRIVER_NOTIFICATION_CALLBACK_ROUTINE = Option<unsafe extern "system" fn(notificationstructure: *const core::ffi::c_void, context: *mut core::ffi::c_void) -> super::super::Win32::Foundation::NTSTATUS>;
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
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
    pub DriverInit: DRIVER_INITIALIZE,
    pub DriverStartIo: DRIVER_STARTIO,
    pub DriverUnload: DRIVER_UNLOAD,
    pub MajorFunction: [DRIVER_DISPATCH; 28],
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for DRIVER_OBJECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type DRIVER_REINITIALIZE = Option<unsafe extern "system" fn(driverobject: *const DRIVER_OBJECT, context: *const core::ffi::c_void, count: u32)>;
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type DRIVER_STARTIO = Option<unsafe extern "system" fn(deviceobject: *mut DEVICE_OBJECT, irp: *mut IRP)>;
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type DRIVER_UNLOAD = Option<unsafe extern "system" fn(driverobject: *const DRIVER_OBJECT)>;
pub const DontUseThisType: POOL_TYPE = 3i32;
pub const DontUseThisTypeSession: POOL_TYPE = 35i32;
pub type ECP_HEADER = isize;
pub type ECP_LIST = isize;
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy)]
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
impl Default for ERESOURCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy)]
pub union ERESOURCE_0 {
    pub Flag: u16,
    pub Anonymous: ERESOURCE_0_0,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for ERESOURCE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy, Default)]
pub struct ERESOURCE_0_0 {
    pub ReservedLowFlags: u8,
    pub WaiterPriority: u8,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy)]
pub union ERESOURCE_1 {
    pub Address: *mut core::ffi::c_void,
    pub CreatorBackTraceIndex: usize,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for ERESOURCE_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type FAST_IO_ACQUIRE_FILE = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT)>;
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type FAST_IO_ACQUIRE_FOR_CCFLUSH = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, deviceobject: *const DEVICE_OBJECT) -> super::super::Win32::Foundation::NTSTATUS>;
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type FAST_IO_ACQUIRE_FOR_MOD_WRITE = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, endingoffset: *const i64, resourcetorelease: *mut *mut ERESOURCE, deviceobject: *const DEVICE_OBJECT) -> super::super::Win32::Foundation::NTSTATUS>;
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type FAST_IO_CHECK_IF_POSSIBLE = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, fileoffset: *const i64, length: u32, wait: bool, lockkey: u32, checkforreadoperation: bool, iostatus: *mut super::super::Win32::System::IO::IO_STATUS_BLOCK, deviceobject: *const DEVICE_OBJECT) -> bool>;
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type FAST_IO_DETACH_DEVICE = Option<unsafe extern "system" fn(sourcedevice: *const DEVICE_OBJECT, targetdevice: *const DEVICE_OBJECT)>;
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type FAST_IO_DEVICE_CONTROL = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, wait: bool, inputbuffer: *const core::ffi::c_void, inputbufferlength: u32, outputbuffer: *mut core::ffi::c_void, outputbufferlength: u32, iocontrolcode: u32, iostatus: *mut super::super::Win32::System::IO::IO_STATUS_BLOCK, deviceobject: *const DEVICE_OBJECT) -> bool>;
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Default)]
pub struct FAST_IO_DISPATCH {
    pub SizeOfFastIoDispatch: u32,
    pub FastIoCheckIfPossible: FAST_IO_CHECK_IF_POSSIBLE,
    pub FastIoRead: FAST_IO_READ,
    pub FastIoWrite: FAST_IO_WRITE,
    pub FastIoQueryBasicInfo: FAST_IO_QUERY_BASIC_INFO,
    pub FastIoQueryStandardInfo: FAST_IO_QUERY_STANDARD_INFO,
    pub FastIoLock: FAST_IO_LOCK,
    pub FastIoUnlockSingle: FAST_IO_UNLOCK_SINGLE,
    pub FastIoUnlockAll: FAST_IO_UNLOCK_ALL,
    pub FastIoUnlockAllByKey: FAST_IO_UNLOCK_ALL_BY_KEY,
    pub FastIoDeviceControl: FAST_IO_DEVICE_CONTROL,
    pub AcquireFileForNtCreateSection: FAST_IO_ACQUIRE_FILE,
    pub ReleaseFileForNtCreateSection: FAST_IO_RELEASE_FILE,
    pub FastIoDetachDevice: FAST_IO_DETACH_DEVICE,
    pub FastIoQueryNetworkOpenInfo: FAST_IO_QUERY_NETWORK_OPEN_INFO,
    pub AcquireForModWrite: FAST_IO_ACQUIRE_FOR_MOD_WRITE,
    pub MdlRead: FAST_IO_MDL_READ,
    pub MdlReadComplete: FAST_IO_MDL_READ_COMPLETE,
    pub PrepareMdlWrite: FAST_IO_PREPARE_MDL_WRITE,
    pub MdlWriteComplete: FAST_IO_MDL_WRITE_COMPLETE,
    pub FastIoReadCompressed: FAST_IO_READ_COMPRESSED,
    pub FastIoWriteCompressed: FAST_IO_WRITE_COMPRESSED,
    pub MdlReadCompleteCompressed: FAST_IO_MDL_READ_COMPLETE_COMPRESSED,
    pub MdlWriteCompleteCompressed: FAST_IO_MDL_WRITE_COMPLETE_COMPRESSED,
    pub FastIoQueryOpen: FAST_IO_QUERY_OPEN,
    pub ReleaseForModWrite: FAST_IO_RELEASE_FOR_MOD_WRITE,
    pub AcquireForCcFlush: FAST_IO_ACQUIRE_FOR_CCFLUSH,
    pub ReleaseForCcFlush: FAST_IO_RELEASE_FOR_CCFLUSH,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type FAST_IO_LOCK = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, fileoffset: *const i64, length: *const i64, processid: PEPROCESS, key: u32, failimmediately: bool, exclusivelock: bool, iostatus: *mut super::super::Win32::System::IO::IO_STATUS_BLOCK, deviceobject: *const DEVICE_OBJECT) -> bool>;
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type FAST_IO_MDL_READ = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, fileoffset: *const i64, length: u32, lockkey: u32, mdlchain: *mut *mut MDL, iostatus: *mut super::super::Win32::System::IO::IO_STATUS_BLOCK, deviceobject: *const DEVICE_OBJECT) -> bool>;
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type FAST_IO_MDL_READ_COMPLETE = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, mdlchain: *const MDL, deviceobject: *const DEVICE_OBJECT) -> bool>;
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type FAST_IO_MDL_READ_COMPLETE_COMPRESSED = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, mdlchain: *const MDL, deviceobject: *const DEVICE_OBJECT) -> bool>;
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type FAST_IO_MDL_WRITE_COMPLETE = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, fileoffset: *const i64, mdlchain: *const MDL, deviceobject: *const DEVICE_OBJECT) -> bool>;
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type FAST_IO_MDL_WRITE_COMPLETE_COMPRESSED = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, fileoffset: *const i64, mdlchain: *const MDL, deviceobject: *const DEVICE_OBJECT) -> bool>;
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type FAST_IO_PREPARE_MDL_WRITE = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, fileoffset: *const i64, length: u32, lockkey: u32, mdlchain: *mut *mut MDL, iostatus: *mut super::super::Win32::System::IO::IO_STATUS_BLOCK, deviceobject: *const DEVICE_OBJECT) -> bool>;
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type FAST_IO_QUERY_BASIC_INFO = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, wait: bool, buffer: *mut super::Storage::FileSystem::FILE_BASIC_INFORMATION, iostatus: *mut super::super::Win32::System::IO::IO_STATUS_BLOCK, deviceobject: *const DEVICE_OBJECT) -> bool>;
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type FAST_IO_QUERY_NETWORK_OPEN_INFO = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, wait: bool, buffer: *mut super::Storage::FileSystem::FILE_NETWORK_OPEN_INFORMATION, iostatus: *mut super::super::Win32::System::IO::IO_STATUS_BLOCK, deviceobject: *const DEVICE_OBJECT) -> bool>;
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type FAST_IO_QUERY_OPEN = Option<unsafe extern "system" fn(irp: *mut IRP, networkinformation: *mut super::Storage::FileSystem::FILE_NETWORK_OPEN_INFORMATION, deviceobject: *const DEVICE_OBJECT) -> bool>;
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type FAST_IO_QUERY_STANDARD_INFO = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, wait: bool, buffer: *mut super::Storage::FileSystem::FILE_STANDARD_INFORMATION, iostatus: *mut super::super::Win32::System::IO::IO_STATUS_BLOCK, deviceobject: *const DEVICE_OBJECT) -> bool>;
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type FAST_IO_READ = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, fileoffset: *const i64, length: u32, wait: bool, lockkey: u32, buffer: *mut core::ffi::c_void, iostatus: *mut super::super::Win32::System::IO::IO_STATUS_BLOCK, deviceobject: *const DEVICE_OBJECT) -> bool>;
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type FAST_IO_READ_COMPRESSED = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, fileoffset: *const i64, length: u32, lockkey: u32, buffer: *mut core::ffi::c_void, mdlchain: *mut *mut MDL, iostatus: *mut super::super::Win32::System::IO::IO_STATUS_BLOCK, compresseddatainfo: *mut super::Storage::FileSystem::COMPRESSED_DATA_INFO, compresseddatainfolength: u32, deviceobject: *const DEVICE_OBJECT) -> bool>;
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type FAST_IO_RELEASE_FILE = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT)>;
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type FAST_IO_RELEASE_FOR_CCFLUSH = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, deviceobject: *const DEVICE_OBJECT) -> super::super::Win32::Foundation::NTSTATUS>;
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type FAST_IO_RELEASE_FOR_MOD_WRITE = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, resourcetorelease: *const ERESOURCE, deviceobject: *const DEVICE_OBJECT) -> super::super::Win32::Foundation::NTSTATUS>;
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type FAST_IO_UNLOCK_ALL = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, processid: PEPROCESS, iostatus: *mut super::super::Win32::System::IO::IO_STATUS_BLOCK, deviceobject: *const DEVICE_OBJECT) -> bool>;
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type FAST_IO_UNLOCK_ALL_BY_KEY = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, processid: *const core::ffi::c_void, key: u32, iostatus: *mut super::super::Win32::System::IO::IO_STATUS_BLOCK, deviceobject: *const DEVICE_OBJECT) -> bool>;
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type FAST_IO_UNLOCK_SINGLE = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, fileoffset: *const i64, length: *const i64, processid: PEPROCESS, key: u32, iostatus: *mut super::super::Win32::System::IO::IO_STATUS_BLOCK, deviceobject: *const DEVICE_OBJECT) -> bool>;
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type FAST_IO_WRITE = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, fileoffset: *const i64, length: u32, wait: bool, lockkey: u32, buffer: *const core::ffi::c_void, iostatus: *mut super::super::Win32::System::IO::IO_STATUS_BLOCK, deviceobject: *const DEVICE_OBJECT) -> bool>;
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type FAST_IO_WRITE_COMPRESSED = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, fileoffset: *const i64, length: u32, lockkey: u32, buffer: *const core::ffi::c_void, mdlchain: *mut *mut MDL, iostatus: *mut super::super::Win32::System::IO::IO_STATUS_BLOCK, compresseddatainfo: *const super::Storage::FileSystem::COMPRESSED_DATA_INFO, compresseddatainfolength: u32, deviceobject: *const DEVICE_OBJECT) -> bool>;
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy)]
pub struct FAST_MUTEX {
    pub Count: i32,
    pub Owner: *mut core::ffi::c_void,
    pub Contention: u32,
    pub Event: KEVENT,
    pub OldIrql: u32,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for FAST_MUTEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
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
    pub LockOperation: bool,
    pub DeletePending: bool,
    pub ReadAccess: bool,
    pub WriteAccess: bool,
    pub DeleteAccess: bool,
    pub SharedRead: bool,
    pub SharedWrite: bool,
    pub SharedDelete: bool,
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
impl Default for FILE_OBJECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type IOMMU_DMA_DEVICE = isize;
pub type IOMMU_DMA_DOMAIN = isize;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IO_COMPLETION_CONTEXT {
    pub Port: *mut core::ffi::c_void,
    pub Key: *mut core::ffi::c_void,
    pub UsageCount: isize,
}
impl Default for IO_COMPLETION_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type IO_PRIORITY_HINT = i32;
#[repr(C)]
#[cfg(all(feature = "Wdk_System_SystemServices", feature = "Win32_Security"))]
#[derive(Clone, Copy)]
pub struct IO_SECURITY_CONTEXT {
    pub SecurityQos: *mut super::super::Win32::Security::SECURITY_QUALITY_OF_SERVICE,
    pub AccessState: *mut ACCESS_STATE,
    pub DesiredAccess: u32,
    pub FullCreateOptions: u32,
}
#[cfg(all(feature = "Wdk_System_SystemServices", feature = "Win32_Security"))]
impl Default for IO_SECURITY_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
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
impl Default for IO_STACK_LOCATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub union IO_STACK_LOCATION_0 {
    pub Create: IO_STACK_LOCATION_0_0,
    pub CreatePipe: IO_STACK_LOCATION_0_1,
    pub CreateMailslot: IO_STACK_LOCATION_0_2,
    pub Read: IO_STACK_LOCATION_0_3,
    pub Write: IO_STACK_LOCATION_0_4,
    pub QueryDirectory: IO_STACK_LOCATION_0_5,
    pub NotifyDirectory: IO_STACK_LOCATION_0_6,
    pub NotifyDirectoryEx: IO_STACK_LOCATION_0_7,
    pub QueryFile: IO_STACK_LOCATION_0_8,
    pub SetFile: IO_STACK_LOCATION_0_9,
    pub QueryEa: IO_STACK_LOCATION_0_10,
    pub SetEa: IO_STACK_LOCATION_0_11,
    pub QueryVolume: IO_STACK_LOCATION_0_12,
    pub SetVolume: IO_STACK_LOCATION_0_13,
    pub FileSystemControl: IO_STACK_LOCATION_0_14,
    pub LockControl: IO_STACK_LOCATION_0_15,
    pub DeviceIoControl: IO_STACK_LOCATION_0_16,
    pub QuerySecurity: IO_STACK_LOCATION_0_17,
    pub SetSecurity: IO_STACK_LOCATION_0_18,
    pub MountVolume: IO_STACK_LOCATION_0_19,
    pub VerifyVolume: IO_STACK_LOCATION_0_20,
    pub Scsi: IO_STACK_LOCATION_0_21,
    pub QueryQuota: IO_STACK_LOCATION_0_22,
    pub SetQuota: IO_STACK_LOCATION_0_23,
    pub QueryDeviceRelations: IO_STACK_LOCATION_0_24,
    pub QueryInterface: IO_STACK_LOCATION_0_25,
    pub DeviceCapabilities: IO_STACK_LOCATION_0_26,
    pub FilterResourceRequirements: IO_STACK_LOCATION_0_27,
    pub ReadWriteConfig: IO_STACK_LOCATION_0_28,
    pub SetLock: IO_STACK_LOCATION_0_29,
    pub QueryId: IO_STACK_LOCATION_0_30,
    pub QueryDeviceText: IO_STACK_LOCATION_0_31,
    pub UsageNotification: IO_STACK_LOCATION_0_32,
    pub WaitWake: IO_STACK_LOCATION_0_33,
    pub PowerSequence: IO_STACK_LOCATION_0_34,
    pub Power: IO_STACK_LOCATION_0_35,
    pub StartDevice: IO_STACK_LOCATION_0_36,
    pub WMI: IO_STACK_LOCATION_0_37,
    pub Others: IO_STACK_LOCATION_0_38,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct IO_STACK_LOCATION_0_2 {
    pub SecurityContext: *mut IO_SECURITY_CONTEXT,
    pub Options: u32,
    pub Reserved: u16,
    pub ShareAccess: u16,
    pub Parameters: *mut super::System::SystemServices::MAILSLOT_CREATE_PARAMETERS,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct IO_STACK_LOCATION_0_1 {
    pub SecurityContext: *mut IO_SECURITY_CONTEXT,
    pub Options: u32,
    pub Reserved: u16,
    pub ShareAccess: u16,
    pub Parameters: *mut super::System::SystemServices::NAMED_PIPE_CREATE_PARAMETERS,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct IO_STACK_LOCATION_0_0 {
    pub SecurityContext: *mut IO_SECURITY_CONTEXT,
    pub Options: u32,
    pub FileAttributes: u16,
    pub ShareAccess: u16,
    pub EaLength: u32,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct IO_STACK_LOCATION_0_26 {
    pub Capabilities: *mut super::System::SystemServices::DEVICE_CAPABILITIES,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_26 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct IO_STACK_LOCATION_0_16 {
    pub OutputBufferLength: u32,
    pub InputBufferLength: u32,
    pub IoControlCode: u32,
    pub Type3InputBuffer: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_16 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct IO_STACK_LOCATION_0_14 {
    pub OutputBufferLength: u32,
    pub InputBufferLength: u32,
    pub FsControlCode: u32,
    pub Type3InputBuffer: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_14 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct IO_STACK_LOCATION_0_27 {
    pub IoResourceRequirementList: *mut super::System::SystemServices::IO_RESOURCE_REQUIREMENTS_LIST,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_27 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct IO_STACK_LOCATION_0_15 {
    pub Length: *mut i64,
    pub Key: u32,
    pub ByteOffset: i64,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_15 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct IO_STACK_LOCATION_0_19 {
    pub Vpb: *mut VPB,
    pub DeviceObject: *mut DEVICE_OBJECT,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_19 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Default)]
pub struct IO_STACK_LOCATION_0_7 {
    pub Length: u32,
    pub CompletionFilter: u32,
    pub DirectoryNotifyInformationClass: super::System::SystemServices::DIRECTORY_NOTIFY_INFORMATION_CLASS,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Default)]
pub struct IO_STACK_LOCATION_0_6 {
    pub Length: u32,
    pub CompletionFilter: u32,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct IO_STACK_LOCATION_0_38 {
    pub Argument1: *mut core::ffi::c_void,
    pub Argument2: *mut core::ffi::c_void,
    pub Argument3: *mut core::ffi::c_void,
    pub Argument4: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_38 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct IO_STACK_LOCATION_0_34 {
    pub PowerSequence: *mut super::System::SystemServices::POWER_SEQUENCE,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_34 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct IO_STACK_LOCATION_0_35 {
    pub Anonymous: IO_STACK_LOCATION_0_35_0,
    pub Type: super::System::SystemServices::POWER_STATE_TYPE,
    pub State: super::System::SystemServices::POWER_STATE,
    pub ShutdownType: super::super::Win32::System::Power::POWER_ACTION,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_35 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub union IO_STACK_LOCATION_0_35_0 {
    pub SystemContext: u32,
    pub SystemPowerStateContext: super::System::SystemServices::SYSTEM_POWER_STATE_CONTEXT,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_35_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Default)]
pub struct IO_STACK_LOCATION_0_24 {
    pub Type: super::System::SystemServices::DEVICE_RELATION_TYPE,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Default)]
pub struct IO_STACK_LOCATION_0_31 {
    pub DeviceTextType: super::System::SystemServices::DEVICE_TEXT_TYPE,
    pub LocaleId: u32,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct IO_STACK_LOCATION_0_5 {
    pub Length: u32,
    pub FileName: *mut super::super::Win32::Foundation::UNICODE_STRING,
    pub FileInformationClass: super::Storage::FileSystem::FILE_INFORMATION_CLASS,
    pub FileIndex: u32,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct IO_STACK_LOCATION_0_10 {
    pub Length: u32,
    pub EaList: *mut core::ffi::c_void,
    pub EaListLength: u32,
    pub EaIndex: u32,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_10 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Default)]
pub struct IO_STACK_LOCATION_0_8 {
    pub Length: u32,
    pub FileInformationClass: super::Storage::FileSystem::FILE_INFORMATION_CLASS,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Default)]
pub struct IO_STACK_LOCATION_0_30 {
    pub IdType: super::System::SystemServices::BUS_QUERY_ID_TYPE,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct IO_STACK_LOCATION_0_25 {
    pub InterfaceType: *const windows_sys::core::GUID,
    pub Size: u16,
    pub Version: u16,
    pub Interface: *mut super::System::SystemServices::INTERFACE,
    pub InterfaceSpecificData: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_25 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct IO_STACK_LOCATION_0_22 {
    pub Length: u32,
    pub StartSid: super::super::Win32::Security::PSID,
    pub SidList: *mut super::Storage::FileSystem::FILE_GET_QUOTA_INFORMATION,
    pub SidListLength: u32,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_22 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Default)]
pub struct IO_STACK_LOCATION_0_17 {
    pub SecurityInformation: u32,
    pub Length: u32,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Default)]
pub struct IO_STACK_LOCATION_0_12 {
    pub Length: u32,
    pub FsInformationClass: super::Storage::FileSystem::FS_INFORMATION_CLASS,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct IO_STACK_LOCATION_0_28 {
    pub WhichSpace: u32,
    pub Buffer: *mut core::ffi::c_void,
    pub Offset: u32,
    pub Length: u32,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_28 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Default)]
pub struct IO_STACK_LOCATION_0_3 {
    pub Length: u32,
    pub Key: u32,
    pub ByteOffset: i64,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct IO_STACK_LOCATION_0_21 {
    pub Srb: *mut _SCSI_REQUEST_BLOCK,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_21 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Default)]
pub struct IO_STACK_LOCATION_0_11 {
    pub Length: u32,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct IO_STACK_LOCATION_0_9 {
    pub Length: u32,
    pub FileInformationClass: super::Storage::FileSystem::FILE_INFORMATION_CLASS,
    pub FileObject: *mut FILE_OBJECT,
    pub Anonymous: IO_STACK_LOCATION_0_9_0,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_9 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub union IO_STACK_LOCATION_0_9_0 {
    pub Anonymous: IO_STACK_LOCATION_0_9_0_0,
    pub ClusterCount: u32,
    pub DeleteHandle: super::super::Win32::Foundation::HANDLE,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_9_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Default)]
pub struct IO_STACK_LOCATION_0_9_0_0 {
    pub ReplaceIfExists: bool,
    pub AdvanceOnly: bool,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Default)]
pub struct IO_STACK_LOCATION_0_29 {
    pub Lock: bool,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Default)]
pub struct IO_STACK_LOCATION_0_23 {
    pub Length: u32,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct IO_STACK_LOCATION_0_18 {
    pub SecurityInformation: u32,
    pub SecurityDescriptor: super::super::Win32::Security::PSECURITY_DESCRIPTOR,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_18 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Default)]
pub struct IO_STACK_LOCATION_0_13 {
    pub Length: u32,
    pub FsInformationClass: super::Storage::FileSystem::FS_INFORMATION_CLASS,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct IO_STACK_LOCATION_0_36 {
    pub AllocatedResources: *mut super::System::SystemServices::CM_RESOURCE_LIST,
    pub AllocatedResourcesTranslated: *mut super::System::SystemServices::CM_RESOURCE_LIST,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_36 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct IO_STACK_LOCATION_0_32 {
    pub InPath: bool,
    pub Reserved: [bool; 3],
    pub Type: super::System::SystemServices::DEVICE_USAGE_NOTIFICATION_TYPE,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct IO_STACK_LOCATION_0_20 {
    pub Vpb: *mut VPB,
    pub DeviceObject: *mut DEVICE_OBJECT,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_20 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct IO_STACK_LOCATION_0_37 {
    pub ProviderId: usize,
    pub DataPath: *mut core::ffi::c_void,
    pub BufferSize: u32,
    pub Buffer: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IO_STACK_LOCATION_0_37 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Default)]
pub struct IO_STACK_LOCATION_0_33 {
    pub PowerState: super::super::Win32::System::Power::SYSTEM_POWER_STATE,
}
#[repr(C, packed(4))]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Default)]
pub struct IO_STACK_LOCATION_0_4 {
    pub Length: u32,
    pub Key: u32,
    pub ByteOffset: i64,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct IRP {
    pub Type: i16,
    pub Size: u16,
    pub MdlAddress: *mut MDL,
    pub Flags: u32,
    pub AssociatedIrp: IRP_0,
    pub ThreadListEntry: super::super::Win32::System::Kernel::LIST_ENTRY,
    pub IoStatus: super::super::Win32::System::IO::IO_STATUS_BLOCK,
    pub RequestorMode: i8,
    pub PendingReturned: bool,
    pub StackCount: i8,
    pub CurrentLocation: i8,
    pub Cancel: bool,
    pub CancelIrql: u8,
    pub ApcEnvironment: i8,
    pub AllocationFlags: u8,
    pub Anonymous: IRP_1,
    pub UserEvent: *mut KEVENT,
    pub Overlay: IRP_2,
    pub CancelRoutine: DRIVER_CANCEL,
    pub UserBuffer: *mut core::ffi::c_void,
    pub Tail: IRP_3,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IRP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub union IRP_1 {
    pub UserIosb: *mut super::super::Win32::System::IO::IO_STATUS_BLOCK,
    pub IoRingContext: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IRP_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub union IRP_0 {
    pub MasterIrp: *mut IRP,
    pub IrpCount: i32,
    pub SystemBuffer: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IRP_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub union IRP_2 {
    pub AsynchronousParameters: IRP_2_0,
    pub AllocationSize: i64,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IRP_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct IRP_2_0 {
    pub Anonymous1: IRP_2_0_0,
    pub Anonymous2: IRP_2_0_1,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IRP_2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub union IRP_2_0_0 {
    pub UserApcRoutine: super::super::Win32::System::IO::PIO_APC_ROUTINE,
    pub IssuingProcess: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IRP_2_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub union IRP_2_0_1 {
    pub UserApcContext: *mut core::ffi::c_void,
    pub IoRing: *mut _IORING_OBJECT,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IRP_2_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub union IRP_3 {
    pub Overlay: IRP_3_0,
    pub Apc: super::System::SystemServices::KAPC,
    pub CompletionKey: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IRP_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct IRP_3_0 {
    pub Anonymous1: IRP_3_0_0,
    pub Thread: PETHREAD,
    pub AuxiliaryBuffer: windows_sys::core::PSTR,
    pub Anonymous2: IRP_3_0_1,
    pub OriginalFileObject: *mut FILE_OBJECT,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IRP_3_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub union IRP_3_0_0 {
    pub DeviceQueueEntry: super::System::SystemServices::KDEVICE_QUEUE_ENTRY,
    pub Anonymous: IRP_3_0_0_0,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IRP_3_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct IRP_3_0_0_0 {
    pub DriverContext: [*mut core::ffi::c_void; 4],
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IRP_3_0_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct IRP_3_0_1 {
    pub ListEntry: super::super::Win32::System::Kernel::LIST_ENTRY,
    pub Anonymous: IRP_3_0_1_0,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IRP_3_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub union IRP_3_0_1_0 {
    pub CurrentStackLocation: *mut IO_STACK_LOCATION,
    pub PacketType: u32,
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for IRP_3_0_1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IoPriorityCritical: IO_PRIORITY_HINT = 4i32;
pub const IoPriorityHigh: IO_PRIORITY_HINT = 3i32;
pub const IoPriorityLow: IO_PRIORITY_HINT = 1i32;
pub const IoPriorityNormal: IO_PRIORITY_HINT = 2i32;
pub const IoPriorityVeryLow: IO_PRIORITY_HINT = 0i32;
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy, Default)]
pub struct KDEVICE_QUEUE {
    pub Type: i16,
    pub Size: i16,
    pub DeviceListHead: super::super::Win32::System::Kernel::LIST_ENTRY,
    pub Lock: usize,
    pub Busy: bool,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy)]
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
impl Default for KDPC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy)]
pub union KDPC_0 {
    pub TargetInfoAsUlong: u32,
    pub Anonymous: KDPC_0_0,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for KDPC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy, Default)]
pub struct KDPC_0_0 {
    pub Type: u8,
    pub Importance: u8,
    pub Number: u16,
}
pub type KENLISTMENT = isize;
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy)]
pub struct KEVENT {
    pub Header: DISPATCHER_HEADER,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for KEVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type KGDT = isize;
pub type KIDT = isize;
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy)]
pub struct KMUTANT {
    pub Header: DISPATCHER_HEADER,
    pub MutantListEntry: super::super::Win32::System::Kernel::LIST_ENTRY,
    pub OwnerThread: *mut isize,
    pub Anonymous: KMUTANT_0,
    pub ApcDisable: u8,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for KMUTANT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy)]
pub union KMUTANT_0 {
    pub MutantFlags: u8,
    pub Anonymous: KMUTANT_0_0,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for KMUTANT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy, Default)]
pub struct KMUTANT_0_0 {
    pub _bitfield: u8,
}
pub type KPCR = isize;
pub type KPRCB = isize;
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy)]
pub struct KQUEUE {
    pub Header: DISPATCHER_HEADER,
    pub EntryListHead: super::super::Win32::System::Kernel::LIST_ENTRY,
    pub CurrentCount: u32,
    pub MaximumCount: u32,
    pub ThreadListHead: super::super::Win32::System::Kernel::LIST_ENTRY,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for KQUEUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type KRESOURCEMANAGER = isize;
pub type KSPIN_LOCK_QUEUE_NUMBER = i32;
pub type KTM = isize;
pub type KTRANSACTION = isize;
pub type KTSS = isize;
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy)]
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
impl Default for KWAIT_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy)]
pub union KWAIT_BLOCK_0 {
    pub Thread: *mut isize,
    pub NotificationQueue: *mut KQUEUE,
    pub Dpc: *mut KDPC,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for KWAIT_BLOCK_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type LOADER_PARAMETER_BLOCK = isize;
pub const LockQueueAfdWorkQueueLock: KSPIN_LOCK_QUEUE_NUMBER = 13i32;
pub const LockQueueBcbLock: KSPIN_LOCK_QUEUE_NUMBER = 14i32;
pub const LockQueueIoCancelLock: KSPIN_LOCK_QUEUE_NUMBER = 7i32;
pub const LockQueueIoCompletionLock: KSPIN_LOCK_QUEUE_NUMBER = 11i32;
pub const LockQueueIoDatabaseLock: KSPIN_LOCK_QUEUE_NUMBER = 10i32;
pub const LockQueueIoVpbLock: KSPIN_LOCK_QUEUE_NUMBER = 9i32;
pub const LockQueueMasterLock: KSPIN_LOCK_QUEUE_NUMBER = 5i32;
pub const LockQueueMaximumLock: KSPIN_LOCK_QUEUE_NUMBER = 17i32;
pub const LockQueueNonPagedPoolLock: KSPIN_LOCK_QUEUE_NUMBER = 6i32;
pub const LockQueueNtfsStructLock: KSPIN_LOCK_QUEUE_NUMBER = 12i32;
pub const LockQueueUnusedSpare0: KSPIN_LOCK_QUEUE_NUMBER = 0i32;
pub const LockQueueUnusedSpare1: KSPIN_LOCK_QUEUE_NUMBER = 1i32;
pub const LockQueueUnusedSpare15: KSPIN_LOCK_QUEUE_NUMBER = 15i32;
pub const LockQueueUnusedSpare16: KSPIN_LOCK_QUEUE_NUMBER = 16i32;
pub const LockQueueUnusedSpare2: KSPIN_LOCK_QUEUE_NUMBER = 2i32;
pub const LockQueueUnusedSpare3: KSPIN_LOCK_QUEUE_NUMBER = 3i32;
pub const LockQueueUnusedSpare8: KSPIN_LOCK_QUEUE_NUMBER = 8i32;
pub const LockQueueVacbLock: KSPIN_LOCK_QUEUE_NUMBER = 4i32;
#[repr(C)]
#[derive(Clone, Copy)]
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
impl Default for MDL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MaxIoPriorityTypes: IO_PRIORITY_HINT = 5i32;
pub const MaxPoolType: POOL_TYPE = 7i32;
pub const NTSTRSAFE_MAX_CCH: u32 = 2147483647u32;
pub const NTSTRSAFE_MAX_LENGTH: u32 = 2147483646u32;
pub const NTSTRSAFE_UNICODE_STRING_MAX_CCH: u32 = 32767u32;
pub const NTSTRSAFE_USE_SECURE_CRT: u32 = 0u32;
pub const NonPagedPool: POOL_TYPE = 0i32;
pub const NonPagedPoolBase: POOL_TYPE = 0i32;
pub const NonPagedPoolBaseCacheAligned: POOL_TYPE = 4i32;
pub const NonPagedPoolBaseCacheAlignedMustS: POOL_TYPE = 6i32;
pub const NonPagedPoolBaseMustSucceed: POOL_TYPE = 2i32;
pub const NonPagedPoolCacheAligned: POOL_TYPE = 4i32;
pub const NonPagedPoolCacheAlignedMustS: POOL_TYPE = 6i32;
pub const NonPagedPoolCacheAlignedMustSSession: POOL_TYPE = 38i32;
pub const NonPagedPoolCacheAlignedSession: POOL_TYPE = 36i32;
pub const NonPagedPoolExecute: POOL_TYPE = 0i32;
pub const NonPagedPoolMustSucceed: POOL_TYPE = 2i32;
pub const NonPagedPoolMustSucceedSession: POOL_TYPE = 34i32;
pub const NonPagedPoolNx: POOL_TYPE = 512i32;
pub const NonPagedPoolNxCacheAligned: POOL_TYPE = 516i32;
pub const NonPagedPoolSession: POOL_TYPE = 32i32;
pub const NonPagedPoolSessionNx: POOL_TYPE = 544i32;
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy)]
pub struct OBJECT_ATTRIBUTES {
    pub Length: u32,
    pub RootDirectory: super::super::Win32::Foundation::HANDLE,
    pub ObjectName: *const super::super::Win32::Foundation::UNICODE_STRING,
    pub Attributes: super::super::Win32::Foundation::OBJECT_ATTRIBUTE_FLAGS,
    pub SecurityDescriptor: *const super::super::Win32::Security::SECURITY_DESCRIPTOR,
    pub SecurityQualityOfService: *const super::super::Win32::Security::SECURITY_QUALITY_OF_SERVICE,
}
#[cfg(feature = "Win32_Security")]
impl Default for OBJECT_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy)]
pub struct OBJECT_ATTRIBUTES32 {
    pub Length: u32,
    pub RootDirectory: u32,
    pub ObjectName: u32,
    pub Attributes: super::super::Win32::Foundation::OBJECT_ATTRIBUTE_FLAGS,
    pub SecurityDescriptor: *const super::super::Win32::Security::SECURITY_DESCRIPTOR,
    pub SecurityQualityOfService: *const super::super::Win32::Security::SECURITY_QUALITY_OF_SERVICE,
}
#[cfg(feature = "Win32_Security")]
impl Default for OBJECT_ATTRIBUTES32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy)]
pub struct OBJECT_ATTRIBUTES64 {
    pub Length: u32,
    pub RootDirectory: u64,
    pub ObjectName: u64,
    pub Attributes: super::super::Win32::Foundation::OBJECT_ATTRIBUTE_FLAGS,
    pub SecurityDescriptor: *const super::super::Win32::Security::SECURITY_DESCRIPTOR,
    pub SecurityQualityOfService: *const super::super::Win32::Security::SECURITY_QUALITY_OF_SERVICE,
}
#[cfg(feature = "Win32_Security")]
impl Default for OBJECT_ATTRIBUTES64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type OBJECT_INFORMATION_CLASS = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct OBJECT_NAME_INFORMATION {
    pub Name: super::super::Win32::Foundation::UNICODE_STRING,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct OWNER_ENTRY {
    pub OwnerThread: usize,
    pub Anonymous: OWNER_ENTRY_0,
}
impl Default for OWNER_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union OWNER_ENTRY_0 {
    pub Anonymous: OWNER_ENTRY_0_0,
    pub TableSize: u32,
}
impl Default for OWNER_ENTRY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct OWNER_ENTRY_0_0 {
    pub _bitfield: u32,
}
pub const ObjectBasicInformation: OBJECT_INFORMATION_CLASS = 0i32;
pub const ObjectTypeInformation: OBJECT_INFORMATION_CLASS = 2i32;
pub type PAFFINITY_TOKEN = isize;
pub type PBUS_HANDLER = isize;
pub type PCALLBACK_OBJECT = isize;
pub type PDEVICE_HANDLER_OBJECT = isize;
pub type PEJOB = isize;
pub type PEPROCESS = isize;
pub type PESILO = isize;
pub type PETHREAD = isize;
pub type PEX_RUNDOWN_REF_CACHE_AWARE = isize;
pub type PEX_TIMER = isize;
pub type PFREE_FUNCTION = Option<unsafe extern "system" fn()>;
pub type PIO_COMPLETION_ROUTINE = Option<unsafe extern "system" fn() -> super::super::Win32::Foundation::NTSTATUS>;
pub type PIO_REMOVE_LOCK_TRACKING_BLOCK = isize;
pub type PIO_TIMER = isize;
pub type PIO_WORKITEM = isize;
pub type PKDEFERRED_ROUTINE = Option<unsafe extern "system" fn()>;
pub type PKINTERRUPT = isize;
pub type PKPROCESS = isize;
pub type PKTHREAD = isize;
pub type PNOTIFY_SYNC = isize;
pub type POBJECT_TYPE = isize;
pub type POHANDLE = *mut core::ffi::c_void;
pub type POOL_TYPE = i32;
pub type PPCW_BUFFER = isize;
pub type PPCW_INSTANCE = isize;
pub type PPCW_REGISTRATION = isize;
pub type PRKPROCESS = isize;
pub type PRKTHREAD = isize;
pub type PSILO_MONITOR = isize;
pub type PWORKER_THREAD_ROUTINE = Option<unsafe extern "system" fn()>;
pub const PagedPool: POOL_TYPE = 1i32;
pub const PagedPoolCacheAligned: POOL_TYPE = 5i32;
pub const PagedPoolCacheAlignedSession: POOL_TYPE = 37i32;
pub const PagedPoolSession: POOL_TYPE = 33i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RTL_SPLAY_LINKS {
    pub Parent: *mut RTL_SPLAY_LINKS,
    pub LeftChild: *mut RTL_SPLAY_LINKS,
    pub RightChild: *mut RTL_SPLAY_LINKS,
}
impl Default for RTL_SPLAY_LINKS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SECTION_OBJECT_POINTERS {
    pub DataSectionObject: *mut core::ffi::c_void,
    pub SharedCacheMap: *mut core::ffi::c_void,
    pub ImageSectionObject: *mut core::ffi::c_void,
}
impl Default for SECTION_OBJECT_POINTERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy)]
pub struct SECURITY_SUBJECT_CONTEXT {
    pub ClientToken: *mut core::ffi::c_void,
    pub ImpersonationLevel: super::super::Win32::Security::SECURITY_IMPERSONATION_LEVEL,
    pub PrimaryToken: *mut core::ffi::c_void,
    pub ProcessAuditId: *mut core::ffi::c_void,
}
#[cfg(feature = "Win32_Security")]
impl Default for SECURITY_SUBJECT_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const STRSAFE_FILL_BEHIND: u32 = 512u32;
pub const STRSAFE_FILL_BEHIND_NULL: u32 = 512u32;
pub const STRSAFE_FILL_ON_FAILURE: u32 = 1024u32;
pub const STRSAFE_IGNORE_NULLS: u32 = 256u32;
pub const STRSAFE_NO_TRUNCATION: u32 = 4096u32;
pub const STRSAFE_NULL_ON_FAILURE: u32 = 2048u32;
pub const STRSAFE_ZERO_LENGTH_ON_FAILURE: u32 = 2048u32;
pub type SspiAsyncContext = isize;
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct TARGET_DEVICE_CUSTOM_NOTIFICATION {
    pub Version: u16,
    pub Size: u16,
    pub Event: windows_sys::core::GUID,
    pub FileObject: *mut FILE_OBJECT,
    pub NameBufferOffset: i32,
    pub CustomDataBuffer: [u8; 1],
}
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for TARGET_DEVICE_CUSTOM_NOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
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
impl Default for VPB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy)]
pub struct WORK_QUEUE_ITEM {
    pub List: super::super::Win32::System::Kernel::LIST_ENTRY,
    pub WorkerRoutine: PWORKER_THREAD_ROUTINE,
    pub Parameter: *mut core::ffi::c_void,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for WORK_QUEUE_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type _DEVICE_OBJECT_POWER_EXTENSION = isize;
pub type _IORING_OBJECT = isize;
pub type _SCSI_REQUEST_BLOCK = isize;
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
