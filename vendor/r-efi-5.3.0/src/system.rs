//! UEFI System Integration
//!
//! This header defines the structures and types of the surrounding system of an UEFI application.
//! It contains the definitions of the system table, the runtime and boot services, as well as
//! common types.
//!
//! We do not document the behavior of each of these types and functions. They follow the UEFI
//! specification, which does a well-enough job of documenting each. This file just provides you
//! the rust definitions of each symbol and some limited hints on some pecularities.

//
// Time Management
//
// UEFI time management is modeled around the EFI_TIME structure, which represents any arbitrary
// timestamp. The runtime and boot services provide helper functions to query and set the system
// time.
//

pub const TIME_ADJUST_DAYLIGHT: u8 = 0x01u8;
pub const TIME_IN_DAYLIGHT: u8 = 0x02u8;

pub const UNSPECIFIED_TIMEZONE: i16 = 0x07ffi16;

// Cannot derive `Eq` etc. due to uninitialized `pad2` field.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct Time {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub pad1: u8,
    pub nanosecond: u32,
    pub timezone: i16,
    pub daylight: u8,
    pub pad2: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct TimeCapabilities {
    pub resolution: u32,
    pub accuracy: u32,
    pub sets_to_zero: crate::base::Boolean,
}

//
// UEFI Variables
//
// UEFI systems provide a way to store global variables. These can be persistent or volatile. The
// variable store must be provided by the platform, but persistent storage might not be available.
//

pub const VARIABLE_NON_VOLATILE: u32 = 0x00000001u32;
pub const VARIABLE_BOOTSERVICE_ACCESS: u32 = 0x00000002u32;
pub const VARIABLE_RUNTIME_ACCESS: u32 = 0x00000004u32;
pub const VARIABLE_HARDWARE_ERROR_RECORD: u32 = 0x00000008u32;
pub const VARIABLE_AUTHENTICATED_WRITE_ACCESS: u32 = 0x00000010u32;
pub const VARIABLE_TIME_BASED_AUTHENTICATED_WRITE_ACCESS: u32 = 0x00000020u32;
pub const VARIABLE_APPEND_WRITE: u32 = 0x00000040u32;
pub const VARIABLE_ENHANCED_AUTHENTICATED_ACCESS: u32 = 0x00000080u32;

pub const VARIABLE_AUTHENTICATION_3_CERT_ID_SHA256: u32 = 0x1u32;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct VariableAuthentication3CertId<const N: usize = 0> {
    pub r#type: u8,
    pub id_size: u32,
    pub id: [u8; N],
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct VariableAuthentication<const N: usize = 0> {
    pub monotonic_count: u64,
    pub auth_info: [u8; N], // WIN_CERTIFICATE_UEFI_ID from PE/COFF
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct VariableAuthentication2<const N: usize = 0> {
    pub timestamp: Time,
    pub auth_info: [u8; N], // WIN_CERTIFICATE_UEFI_ID from PE/COFF
}

pub const VARIABLE_AUTHENTICATION_3_TIMESTAMP_TYPE: u32 = 0x1u32;
pub const VARIABLE_AUTHENTICATION_3_NONCE_TYPE: u32 = 0x2u32;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct VariableAuthentication3 {
    pub version: u8,
    pub r#type: u8,
    pub metadata_size: u32,
    pub flags: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct VariableAuthentication3Nonce<const N: usize = 0> {
    pub nonce_size: u32,
    pub nonce: [u8; N],
}

pub const HARDWARE_ERROR_VARIABLE_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0x414E6BDD,
    0xE47B,
    0x47cc,
    0xB2,
    0x44,
    &[0xBB, 0x61, 0x02, 0x0C, 0xF5, 0x16],
);

//
// Virtual Mappings
//
// UEFI runs in an 1-to-1 mapping from virtual to physical addresses. But once you exit boot
// services, you can apply any address mapping you want, as long as you inform UEFI about it (or,
// alternatively, stop using the UEFI runtime services).
//

pub const OPTIONAL_POINTER: u32 = 0x00000001u32;

//
// System Reset
//
// UEFI provides access to firmware functions to reset the system. This includes a wide variety of
// different possible resets.
//

pub type ResetType = u32;

pub const RESET_COLD: ResetType = 0x00000000;
pub const RESET_WARM: ResetType = 0x00000001;
pub const RESET_SHUTDOWN: ResetType = 0x00000002;
pub const RESET_PLATFORM_SPECIFIC: ResetType = 0x00000003;

//
// Update Capsules
//
// The process of firmware updates is generalized in UEFI. There are small blobs called capsules
// that you can push into the firmware to be run either immediately or on next reboot.
//

#[repr(C)]
#[derive(Clone, Copy)]
pub union CapsuleBlockDescriptorUnion {
    pub data_block: crate::base::PhysicalAddress,
    pub continuation_pointer: crate::base::PhysicalAddress,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct CapsuleBlockDescriptor {
    pub length: u64,
    pub data: CapsuleBlockDescriptorUnion,
}

pub const CAPSULE_FLAGS_PERSIST_ACROSS_RESET: u32 = 0x00010000u32;
pub const CAPSULE_FLAGS_POPULATE_SYSTEM_TABLE: u32 = 0x00020000u32;
pub const CAPSULE_FLAGS_INITIATE_RESET: u32 = 0x00040000u32;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CapsuleHeader {
    pub capsule_guid: crate::base::Guid,
    pub header_size: u32,
    pub flags: u32,
    pub capsule_image_size: u32,
}

pub const OS_INDICATIONS_BOOT_TO_FW_UI: u64 = 0x0000000000000001u64;
pub const OS_INDICATIONS_TIMESTAMP_REVOCATION: u64 = 0x0000000000000002u64;
pub const OS_INDICATIONS_FILE_CAPSULE_DELIVERY_SUPPORTED: u64 = 0x0000000000000004u64;
pub const OS_INDICATIONS_FMP_CAPSULE_SUPPORTED: u64 = 0x0000000000000008u64;
pub const OS_INDICATIONS_CAPSULE_RESULT_VAR_SUPPORTED: u64 = 0x0000000000000010u64;
pub const OS_INDICATIONS_START_OS_RECOVERY: u64 = 0x0000000000000020u64;
pub const OS_INDICATIONS_START_PLATFORM_RECOVERY: u64 = 0x0000000000000040u64;

pub const CAPSULE_REPORT_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0x39b68c46,
    0xf7fb,
    0x441b,
    0xb6,
    0xec,
    &[0x16, 0xb0, 0xf6, 0x98, 0x21, 0xf3],
);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CapsuleResultVariableHeader {
    pub variable_total_size: u32,
    pub reserved: u32,
    pub capsule_guid: crate::base::Guid,
    pub capsule_processed: Time,
    pub capsule_status: crate::base::Status,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CapsuleResultVariableFMP<const N: usize = 0> {
    pub version: u16,
    pub payload_index: u8,
    pub update_image_index: u8,
    pub update_image_type_id: crate::base::Guid,
    pub capsule_file_name_and_target: [crate::base::Char16; N],
}

//
// Tasks
//
// UEFI uses a simplified task model, and only ever runs on a single CPU. Usually, there is only
// one single task running on the system, which is the current execution. No interrupts are
// supported, other than timer interrupts. That is, all device management must be reliant on
// polling.
//
// You can, however, register callbacks to be run by the UEFI core. That is, either when execution
// is returned to the UEFI core, or when a timer interrupt fires, the scheduler will run the
// highest priority task next, interrupting the current task. You can use simple
// task-priority-levels (TPL) to adjust the priority of your callbacks and current task.
//

pub const EVT_TIMER: u32 = 0x80000000u32;
pub const EVT_RUNTIME: u32 = 0x40000000u32;
pub const EVT_NOTIFY_WAIT: u32 = 0x00000100u32;
pub const EVT_NOTIFY_SIGNAL: u32 = 0x00000200u32;
pub const EVT_SIGNAL_EXIT_BOOT_SERVICES: u32 = 0x00000201u32;
pub const EVT_SIGNAL_VIRTUAL_ADDRESS_CHANGE: u32 = 0x60000202u32;

pub type EventNotify = eficall! {fn(crate::base::Event, *mut core::ffi::c_void)};

pub const EVENT_GROUP_EXIT_BOOT_SERVICES: crate::base::Guid = crate::base::Guid::from_fields(
    0x27abf055,
    0xb1b8,
    0x4c26,
    0x80,
    0x48,
    &[0x74, 0x8f, 0x37, 0xba, 0xa2, 0xdf],
);
pub const EVENT_GROUP_BEFORE_EXIT_BOOT_SERVICES: crate::base::Guid = crate::base::Guid::from_fields(
    0x8be0e274,
    0x3970,
    0x4b44,
    0x80,
    0xc5,
    &[0x1a, 0xb9, 0x50, 0x2f, 0x3b, 0xfc],
);
pub const EVENT_GROUP_VIRTUAL_ADDRESS_CHANGE: crate::base::Guid = crate::base::Guid::from_fields(
    0x13fa7698,
    0xc831,
    0x49c7,
    0x87,
    0xea,
    &[0x8f, 0x43, 0xfc, 0xc2, 0x51, 0x96],
);
pub const EVENT_GROUP_MEMORY_MAP_CHANGE: crate::base::Guid = crate::base::Guid::from_fields(
    0x78bee926,
    0x692f,
    0x48fd,
    0x9e,
    0xdb,
    &[0x1, 0x42, 0x2e, 0xf0, 0xd7, 0xab],
);
pub const EVENT_GROUP_READY_TO_BOOT: crate::base::Guid = crate::base::Guid::from_fields(
    0x7ce88fb3,
    0x4bd7,
    0x4679,
    0x87,
    0xa8,
    &[0xa8, 0xd8, 0xde, 0xe5, 0x0d, 0x2b],
);
pub const EVENT_GROUP_AFTER_READY_TO_BOOT: crate::base::Guid = crate::base::Guid::from_fields(
    0x3a2a00ad,
    0x98b9,
    0x4cdf,
    0xa4,
    0x78,
    &[0x70, 0x27, 0x77, 0xf1, 0xc1, 0x0b],
);
pub const EVENT_GROUP_RESET_SYSTEM: crate::base::Guid = crate::base::Guid::from_fields(
    0x62da6a56,
    0x13fb,
    0x485a,
    0xa8,
    0xda,
    &[0xa3, 0xdd, 0x79, 0x12, 0xcb, 0x6b],
);

pub type TimerDelay = u32;

pub const TIMER_CANCEL: TimerDelay = 0x00000000;
pub const TIMER_PERIODIC: TimerDelay = 0x00000001;
pub const TIMER_RELATIVE: TimerDelay = 0x00000002;

pub const TPL_APPLICATION: crate::base::Tpl = 4;
pub const TPL_CALLBACK: crate::base::Tpl = 8;
pub const TPL_NOTIFY: crate::base::Tpl = 16;
pub const TPL_HIGH_LEVEL: crate::base::Tpl = 31;

//
// Memory management
//
// The UEFI boot services provide you pool-allocation helpers to reserve memory. The region for
// each allocation can be selected by the caller, allowing to reserve memory that even survives
// beyond boot services. However, dynamic allocations can only performed via boot services, so no
// dynamic modifications can be done once you exit boot services.
//

pub type AllocateType = u32;

pub const ALLOCATE_ANY_PAGES: AllocateType = 0x00000000;
pub const ALLOCATE_MAX_ADDRESS: AllocateType = 0x00000001;
pub const ALLOCATE_ADDRESS: AllocateType = 0x00000002;

pub type MemoryType = u32;

pub const RESERVED_MEMORY_TYPE: MemoryType = 0x00000000;
pub const LOADER_CODE: MemoryType = 0x00000001;
pub const LOADER_DATA: MemoryType = 0x00000002;
pub const BOOT_SERVICES_CODE: MemoryType = 0x00000003;
pub const BOOT_SERVICES_DATA: MemoryType = 0x00000004;
pub const RUNTIME_SERVICES_CODE: MemoryType = 0x00000005;
pub const RUNTIME_SERVICES_DATA: MemoryType = 0x00000006;
pub const CONVENTIONAL_MEMORY: MemoryType = 0x00000007;
pub const UNUSABLE_MEMORY: MemoryType = 0x00000008;
pub const ACPI_RECLAIM_MEMORY: MemoryType = 0x00000009;
pub const ACPI_MEMORY_NVS: MemoryType = 0x0000000a;
pub const MEMORY_MAPPED_IO: MemoryType = 0x0000000b;
pub const MEMORY_MAPPED_IO_PORT_SPACE: MemoryType = 0x0000000c;
pub const PAL_CODE: MemoryType = 0x0000000d;
pub const PERSISTENT_MEMORY: MemoryType = 0x0000000e;
pub const UNACCEPTED_MEMORY_TYPE: MemoryType = 0x0000000f;

pub const MEMORY_UC: u64 = 0x0000000000000001u64;
pub const MEMORY_WC: u64 = 0x0000000000000002u64;
pub const MEMORY_WT: u64 = 0x0000000000000004u64;
pub const MEMORY_WB: u64 = 0x0000000000000008u64;
pub const MEMORY_UCE: u64 = 0x0000000000000010u64;
pub const MEMORY_WP: u64 = 0x0000000000001000u64;
pub const MEMORY_RP: u64 = 0x0000000000002000u64;
pub const MEMORY_XP: u64 = 0x0000000000004000u64;
pub const MEMORY_NV: u64 = 0x0000000000008000u64;
pub const MEMORY_MORE_RELIABLE: u64 = 0x0000000000010000u64;
pub const MEMORY_RO: u64 = 0x0000000000020000u64;
pub const MEMORY_SP: u64 = 0x0000000000040000u64;
pub const MEMORY_CPU_CRYPTO: u64 = 0x0000000000080000u64;
pub const MEMORY_RUNTIME: u64 = 0x8000000000000000u64;
pub const MEMORY_ISA_VALID: u64 = 0x4000000000000000u64;
pub const MEMORY_ISA_MASK: u64 = 0x0FFFF00000000000u64;

/// Mask of memory attributes that specify cacheability attributes. No symbol
/// is defined by the spec, but the attributes are annotated in the spec. Note
/// that `MEMORY_WP`, despite its name, is treated as cacheability attribute.
/// Use `MEMORY_RO` as replacement access attribute (see the spec for details).
pub const CACHE_ATTRIBUTE_MASK: u64 =
    MEMORY_UC | MEMORY_WC | MEMORY_WT | MEMORY_WB | MEMORY_UCE | MEMORY_WP;

/// Mask of memory attributes that specify access protection attributes. No
/// symbol is defined by the spec, but the attributes are annotated in the
/// spec. Note that `MEMORY_WP` is treated as cacheability attribute, and its
/// access protection functionality is replaced by `MEMORY_RO`.
pub const MEMORY_ACCESS_MASK: u64 = MEMORY_RP | MEMORY_XP | MEMORY_RO;

/// Mask of memory attributes that specify properties of a memory region that
/// can be managed via the CPU architecture protocol.
pub const MEMORY_ATTRIBUTE_MASK: u64 = MEMORY_ACCESS_MASK | MEMORY_SP | MEMORY_CPU_CRYPTO;

pub const MEMORY_DESCRIPTOR_VERSION: u32 = 0x00000001u32;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct MemoryDescriptor {
    pub r#type: u32,
    pub physical_start: crate::base::PhysicalAddress,
    pub virtual_start: crate::base::VirtualAddress,
    pub number_of_pages: u64,
    pub attribute: u64,
}

//
// Protocol Management
//
// The UEFI driver model provides ways to have bus-drivers, device-drivers, and applications as
// separate, independent entities. They use protocols to communicate, and handles to refer to
// common state. Drivers and devices can be registered dynamically at runtime, and can support
// hotplugging.
//

pub type InterfaceType = u32;

pub const NATIVE_INTERFACE: InterfaceType = 0x00000000;

pub type LocateSearchType = u32;

pub const ALL_HANDLES: LocateSearchType = 0x00000000;
pub const BY_REGISTER_NOTIFY: LocateSearchType = 0x00000001;
pub const BY_PROTOCOL: LocateSearchType = 0x00000002;

pub const OPEN_PROTOCOL_BY_HANDLE_PROTOCOL: u32 = 0x00000001u32;
pub const OPEN_PROTOCOL_GET_PROTOCOL: u32 = 0x00000002u32;
pub const OPEN_PROTOCOL_TEST_PROTOCOL: u32 = 0x00000004u32;
pub const OPEN_PROTOCOL_BY_CHILD_CONTROLLER: u32 = 0x00000008u32;
pub const OPEN_PROTOCOL_BY_DRIVER: u32 = 0x00000010u32;
pub const OPEN_PROTOCOL_EXCLUSIVE: u32 = 0x00000020u32;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct OpenProtocolInformationEntry {
    pub agent_handle: crate::base::Handle,
    pub controller_handle: crate::base::Handle,
    pub attributes: u32,
    pub open_count: u32,
}

//
// Configuration Tables
//
// The system table contains an array of auxiliary tables, indexed by their GUID, called
// configuration tables. Each table uses the generic ConfigurationTable structure as header.
//

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ConfigurationTable {
    pub vendor_guid: crate::base::Guid,
    pub vendor_table: *mut core::ffi::c_void,
}

pub const RT_PROPERTIES_TABLE_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0xeb66918a,
    0x7eef,
    0x402a,
    0x84,
    0x2e,
    &[0x93, 0x1d, 0x21, 0xc3, 0x8a, 0xe9],
);

pub const RT_PROPERTIES_TABLE_VERSION: u16 = 0x0001;

pub const RT_SUPPORTED_GET_TIME: u32 = 0x0000001;
pub const RT_SUPPORTED_SET_TIME: u32 = 0x0000002;
pub const RT_SUPPORTED_GET_WAKEUP_TIME: u32 = 0x0000004;
pub const RT_SUPPORTED_SET_WAKEUP_TIME: u32 = 0x0000008;
pub const RT_SUPPORTED_GET_VARIABLE: u32 = 0x00000010;
pub const RT_SUPPORTED_GET_NEXT_VARIABLE_NAME: u32 = 0x00000020;
pub const RT_SUPPORTED_SET_VARIABLE: u32 = 0x00000040;
pub const RT_SUPPORTED_SET_VIRTUAL_ADDRESS_MAP: u32 = 0x00000080;
pub const RT_SUPPORTED_CONVERT_POINTER: u32 = 0x00000100;
pub const RT_SUPPORTED_GET_NEXT_HIGH_MONOTONIC_COUNT: u32 = 0x00000200;
pub const RT_SUPPORTED_RESET_SYSTEM: u32 = 0x00000400;
pub const RT_SUPPORTED_UPDATE_CAPSULE: u32 = 0x00000800;
pub const RT_SUPPORTED_QUERY_CAPSULE_CAPABILITIES: u32 = 0x00001000;
pub const RT_SUPPORTED_QUERY_VARIABLE_INFO: u32 = 0x00002000;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RtPropertiesTable {
    pub version: u16,
    pub length: u16,
    pub runtime_services_supported: u32,
}

pub const PROPERTIES_TABLE_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0x880aaca3,
    0x4adc,
    0x4a04,
    0x90,
    0x79,
    &[0xb7, 0x47, 0x34, 0x8, 0x25, 0xe5],
);

pub const PROPERTIES_TABLE_VERSION: u32 = 0x00010000u32;

pub const PROPERTIES_RUNTIME_MEMORY_PROTECTION_NON_EXECUTABLE_PE_DATA: u64 = 0x1u64;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct PropertiesTable {
    pub version: u32,
    pub length: u32,
    pub memory_protection_attribute: u64,
}

pub const MEMORY_ATTRIBUTES_TABLE_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0xdcfa911d,
    0x26eb,
    0x469f,
    0xa2,
    0x20,
    &[0x38, 0xb7, 0xdc, 0x46, 0x12, 0x20],
);

pub const MEMORY_ATTRIBUTES_TABLE_VERSION: u32 = 0x00000001u32;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct MemoryAttributesTable<const N: usize = 0> {
    pub version: u32,
    pub number_of_entries: u32,
    pub descriptor_size: u32,
    pub reserved: u32,
    pub entry: [MemoryDescriptor; N],
}

pub const CONFORMANCE_PROFILES_TABLE_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0x36122546,
    0xf7e7,
    0x4c8f,
    0xbd,
    0x9b,
    &[0xeb, 0x85, 0x25, 0xb5, 0x0c, 0x0b],
);

pub const CONFORMANCE_PROFILES_TABLE_VERSION: u16 = 0x0001;

pub const CONFORMANCE_PROFILES_UEFI_SPEC_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0x523c91af,
    0xa195,
    0x4382,
    0x81,
    0x8d,
    &[0x29, 0x5f, 0xe4, 0x00, 0x64, 0x65],
);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ConformanceProfilesTable<const N: usize = 0> {
    pub version: u16,
    pub number_of_profiles: u16,
    pub conformance_profiles: [crate::base::Guid; N],
}

//
// External Configuration Tables
//
// This lists the Guids of configuration tables of other industry standards, as listed in
// the UEFI specification. See each standard for details on the data included in each table.
//

pub const ACPI_10_TABLE_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0xeb9d2d30,
    0x2d88,
    0x11d3,
    0x9a,
    0x16,
    &[0x00, 0x90, 0x27, 0x3f, 0xc1, 0x4d],
);

pub const ACPI_20_TABLE_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0x8868e871,
    0xe4f1,
    0x11d3,
    0xbc,
    0x22,
    &[0x00, 0x80, 0xc7, 0x3c, 0x88, 0x81],
);

pub const DTB_TABLE_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0xb1b621d5,
    0xf19c,
    0x41a5,
    0x83,
    0x0b,
    &[0xd9, 0x15, 0x2c, 0x69, 0xaa, 0xe0],
);

pub const JSON_CAPSULE_DATA_TABLE_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0x35e7a725,
    0x8dd2,
    0x4cac,
    0x80,
    0x11,
    &[0x33, 0xcd, 0xa8, 0x10, 0x90, 0x56],
);

pub const JSON_CAPSULE_RESULT_TABLE_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0xdbc461c3,
    0xb3de,
    0x422a,
    0xb9,
    0xb4,
    &[0x98, 0x86, 0xfd, 0x49, 0xa1, 0xe5],
);

pub const JSON_CONFIG_DATA_TABLE_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0x87367f87,
    0x1119,
    0x41ce,
    0xaa,
    0xec,
    &[0x8b, 0xe0, 0x11, 0x1f, 0x55, 0x8a],
);

pub const MPS_TABLE_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0xeb9d2d2f,
    0x2d88,
    0x11d3,
    0x9a,
    0x16,
    &[0x00, 0x90, 0x27, 0x3f, 0xc1, 0x4d],
);

pub const SAL_SYSTEM_TABLE_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0xeb9d2d32,
    0x2d88,
    0x11d3,
    0x9a,
    0x16,
    &[0x00, 0x90, 0x27, 0x3f, 0xc1, 0x4d],
);

pub const SMBIOS_TABLE_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0xeb9d2d31,
    0x2d88,
    0x11d3,
    0x9a,
    0x16,
    &[0x00, 0x90, 0x27, 0x3f, 0xc1, 0x4d],
);

pub const SMBIOS3_TABLE_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0xf2fd1544,
    0x9794,
    0x4a2c,
    0x99,
    0x2e,
    &[0xe5, 0xbb, 0xcf, 0x20, 0xe3, 0x94],
);

//
// Global Tables
//
// UEFI uses no global state, so all access to UEFI internal state is done through vtables you get
// passed to your entry-point. The global entry is the system-table, which encorporates several
// sub-tables, including the runtime and boot service tables, and configuration tables (including
// vendor extensions).
//

pub const SPECIFICATION_REVISION: u32 = SYSTEM_TABLE_REVISION;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct TableHeader {
    pub signature: u64,
    pub revision: u32,
    pub header_size: u32,
    pub crc32: u32,
    pub reserved: u32,
}

pub const RUNTIME_SERVICES_SIGNATURE: u64 = 0x56524553544e5552u64; // "RUNTSERV"
pub const RUNTIME_SERVICES_REVISION: u32 = SPECIFICATION_REVISION;

pub type RuntimeGetTime = eficall! {fn(
    *mut Time,
    *mut TimeCapabilities,
) -> crate::base::Status};

pub type RuntimeSetTime = eficall! {fn(
    *mut Time,
) -> crate::base::Status};

pub type RuntimeGetWakeupTime = eficall! {fn(
    *mut crate::base::Boolean,
    *mut crate::base::Boolean,
    *mut Time,
) -> crate::base::Status};

pub type RuntimeSetWakeupTime = eficall! {fn(
    crate::base::Boolean,
    *mut Time,
) -> crate::base::Status};

pub type RuntimeSetVirtualAddressMap = eficall! {fn(
    usize,
    usize,
    u32,
    *mut MemoryDescriptor,
) -> crate::base::Status};

pub type RuntimeConvertPointer = eficall! {fn(
    usize,
    *mut *mut core::ffi::c_void,
) -> crate::base::Status};

pub type RuntimeGetVariable = eficall! {fn(
    *mut crate::base::Char16,
    *mut crate::base::Guid,
    *mut u32,
    *mut usize,
    *mut core::ffi::c_void,
) -> crate::base::Status};

pub type RuntimeGetNextVariableName = eficall! {fn(
    *mut usize,
    *mut crate::base::Char16,
    *mut crate::base::Guid,
) -> crate::base::Status};

pub type RuntimeSetVariable = eficall! {fn(
    *mut crate::base::Char16,
    *mut crate::base::Guid,
    u32,
    usize,
    *mut core::ffi::c_void,
) -> crate::base::Status};

pub type RuntimeGetNextHighMonoCount = eficall! {fn(
    *mut u32,
) -> crate::base::Status};

pub type RuntimeResetSystem = eficall! {fn(
    ResetType,
    crate::base::Status,
    usize,
    *mut core::ffi::c_void,
)};

pub type RuntimeUpdateCapsule = eficall! {fn(
    *mut *mut CapsuleHeader,
    usize,
    crate::base::PhysicalAddress,
) -> crate::base::Status};

pub type RuntimeQueryCapsuleCapabilities = eficall! {fn(
    *mut *mut CapsuleHeader,
    usize,
    *mut u64,
    *mut ResetType,
) -> crate::base::Status};

pub type RuntimeQueryVariableInfo = eficall! {fn(
    u32,
    *mut u64,
    *mut u64,
    *mut u64,
) -> crate::base::Status};

#[repr(C)]
pub struct RuntimeServices {
    pub hdr: TableHeader,

    pub get_time: RuntimeGetTime,
    pub set_time: RuntimeSetTime,
    pub get_wakeup_time: RuntimeGetWakeupTime,
    pub set_wakeup_time: RuntimeSetWakeupTime,

    pub set_virtual_address_map: RuntimeSetVirtualAddressMap,
    pub convert_pointer: RuntimeConvertPointer,

    pub get_variable: RuntimeGetVariable,
    pub get_next_variable_name: RuntimeGetNextVariableName,
    pub set_variable: RuntimeSetVariable,

    pub get_next_high_mono_count: RuntimeGetNextHighMonoCount,
    pub reset_system: RuntimeResetSystem,

    pub update_capsule: RuntimeUpdateCapsule,
    pub query_capsule_capabilities: RuntimeQueryCapsuleCapabilities,
    pub query_variable_info: RuntimeQueryVariableInfo,
}

pub const BOOT_SERVICES_SIGNATURE: u64 = 0x56524553544f4f42u64; // "BOOTSERV"
pub const BOOT_SERVICES_REVISION: u32 = SPECIFICATION_REVISION;

pub type BootRaiseTpl = eficall! {fn(
    crate::base::Tpl,
) -> crate::base::Tpl};

pub type BootRestoreTpl = eficall! {fn(
    crate::base::Tpl,
)};

pub type BootAllocatePages = eficall! {fn(
    AllocateType,
    MemoryType,
    usize,
    *mut crate::base::PhysicalAddress,
) -> crate::base::Status};

pub type BootFreePages = eficall! {fn(
    crate::base::PhysicalAddress,
    usize,
) -> crate::base::Status};

pub type BootGetMemoryMap = eficall! {fn(
    *mut usize,
    *mut MemoryDescriptor,
    *mut usize,
    *mut usize,
    *mut u32,
) -> crate::base::Status};

pub type BootAllocatePool = eficall! {fn(
    MemoryType,
    usize,
    *mut *mut core::ffi::c_void,
) -> crate::base::Status};

pub type BootFreePool = eficall! {fn(
    *mut core::ffi::c_void,
) -> crate::base::Status};

pub type BootCreateEvent = eficall! {fn(
    u32,
    crate::base::Tpl,
    Option<EventNotify>,
    *mut core::ffi::c_void,
    *mut crate::base::Event,
) -> crate::base::Status};

pub type BootSetTimer = eficall! {fn(
    crate::base::Event,
    TimerDelay,
    u64,
) -> crate::base::Status};

pub type BootWaitForEvent = eficall! {fn(
    usize,
    *mut crate::base::Event,
    *mut usize,
) -> crate::base::Status};

pub type BootSignalEvent = eficall! {fn(
    crate::base::Event,
) -> crate::base::Status};

pub type BootCloseEvent = eficall! {fn(
    crate::base::Event,
) -> crate::base::Status};

pub type BootCheckEvent = eficall! {fn(
    crate::base::Event,
) -> crate::base::Status};

pub type BootInstallProtocolInterface = eficall! {fn(
    *mut crate::base::Handle,
    *mut crate::base::Guid,
    InterfaceType,
    *mut core::ffi::c_void,
) -> crate::base::Status};

pub type BootReinstallProtocolInterface = eficall! {fn(
    crate::base::Handle,
    *mut crate::base::Guid,
    *mut core::ffi::c_void,
    *mut core::ffi::c_void,
) -> crate::base::Status};

pub type BootUninstallProtocolInterface = eficall! {fn(
    crate::base::Handle,
    *mut crate::base::Guid,
    *mut core::ffi::c_void,
) -> crate::base::Status};

pub type BootHandleProtocol = eficall! {fn(
    crate::base::Handle,
    *mut crate::base::Guid,
    *mut *mut core::ffi::c_void,
) -> crate::base::Status};

pub type BootRegisterProtocolNotify = eficall! {fn(
    *mut crate::base::Guid,
    crate::base::Event,
    *mut *mut core::ffi::c_void,
) -> crate::base::Status};

pub type BootLocateHandle = eficall! {fn(
    LocateSearchType,
    *mut crate::base::Guid,
    *mut core::ffi::c_void,
    *mut usize,
    *mut crate::base::Handle,
) -> crate::base::Status};

pub type BootLocateDevicePath = eficall! {fn(
    *mut crate::base::Guid,
    *mut *mut crate::protocols::device_path::Protocol,
    *mut crate::base::Handle,
) -> crate::base::Status};

pub type BootInstallConfigurationTable = eficall! {fn(
    *mut crate::base::Guid,
    *mut core::ffi::c_void,
) -> crate::base::Status};

pub type BootLoadImage = eficall! {fn(
    crate::base::Boolean,
    crate::base::Handle,
    *mut crate::protocols::device_path::Protocol,
    *mut core::ffi::c_void,
    usize,
    *mut crate::base::Handle,
) -> crate::base::Status};

pub type BootStartImage = eficall! {fn(
    crate::base::Handle,
    *mut usize,
    *mut *mut crate::base::Char16,
) -> crate::base::Status};

pub type BootExit = eficall! {fn(
    crate::base::Handle,
    crate::base::Status,
    usize,
    *mut crate::base::Char16,
) -> crate::base::Status};

pub type BootUnloadImage = eficall! {fn(
    crate::base::Handle,
) -> crate::base::Status};

pub type BootExitBootServices = eficall! {fn(
    crate::base::Handle,
    usize,
) -> crate::base::Status};

pub type BootGetNextMonotonicCount = eficall! {fn(
    *mut u64,
) -> crate::base::Status};

pub type BootStall = eficall! {fn(
    usize,
) -> crate::base::Status};

pub type BootSetWatchdogTimer = eficall! {fn(
    usize,
    u64,
    usize,
    *mut crate::base::Char16,
) -> crate::base::Status};

pub type BootConnectController = eficall! {fn(
    crate::base::Handle,
    *mut crate::base::Handle,
    *mut crate::protocols::device_path::Protocol,
    crate::base::Boolean,
) -> crate::base::Status};

pub type BootDisconnectController = eficall! {fn(
    crate::base::Handle,
    crate::base::Handle,
    crate::base::Handle,
) -> crate::base::Status};

pub type BootOpenProtocol = eficall! {fn(
    crate::base::Handle,
    *mut crate::base::Guid,
    *mut *mut core::ffi::c_void,
    crate::base::Handle,
    crate::base::Handle,
    u32,
) -> crate::base::Status};

pub type BootCloseProtocol = eficall! {fn(
    crate::base::Handle,
    *mut crate::base::Guid,
    crate::base::Handle,
    crate::base::Handle,
) -> crate::base::Status};

pub type BootOpenProtocolInformation = eficall! {fn(
    crate::base::Handle,
    *mut crate::base::Guid,
    *mut *mut OpenProtocolInformationEntry,
    *mut usize,
) -> crate::base::Status};

pub type BootProtocolsPerHandle = eficall! {fn(
    crate::base::Handle,
    *mut *mut *mut crate::base::Guid,
    *mut usize,
) -> crate::base::Status};

pub type BootLocateHandleBuffer = eficall! {fn(
    LocateSearchType,
    *mut crate::base::Guid,
    *mut core::ffi::c_void,
    *mut usize,
    *mut *mut crate::base::Handle,
) -> crate::base::Status};

pub type BootLocateProtocol = eficall! {fn(
    *mut crate::base::Guid,
    *mut core::ffi::c_void,
    *mut *mut core::ffi::c_void,
) -> crate::base::Status};

pub type BootInstallMultipleProtocolInterfaces = eficall! {fn(
    *mut crate::base::Handle,
    // XXX: Actual definition is variadic. See eficall!{} for details.
    *mut core::ffi::c_void,
    *mut core::ffi::c_void,
) -> crate::base::Status};

pub type BootUninstallMultipleProtocolInterfaces = eficall! {fn(
    crate::base::Handle,
    // XXX: Actual definition is variadic. See eficall!{} for details.
    *mut core::ffi::c_void,
    *mut core::ffi::c_void,
) -> crate::base::Status};

pub type BootCalculateCrc32 = eficall! {fn(
    *mut core::ffi::c_void,
    usize,
    *mut u32,
) -> crate::base::Status};

pub type BootCopyMem = eficall! {fn(
    *mut core::ffi::c_void,
    *mut core::ffi::c_void,
    usize,
)};

pub type BootSetMem = eficall! {fn(
    *mut core::ffi::c_void,
    usize,
    u8,
)};

pub type BootCreateEventEx = eficall! {fn(
    u32,
    crate::base::Tpl,
    Option<EventNotify>,
    *const core::ffi::c_void,
    *const crate::base::Guid,
    *mut crate::base::Event,
) -> crate::base::Status};

#[repr(C)]
pub struct BootServices {
    pub hdr: TableHeader,

    pub raise_tpl: BootRaiseTpl,
    pub restore_tpl: BootRestoreTpl,

    pub allocate_pages: BootAllocatePages,
    pub free_pages: BootFreePages,
    pub get_memory_map: BootGetMemoryMap,
    pub allocate_pool: BootAllocatePool,
    pub free_pool: BootFreePool,

    pub create_event: BootCreateEvent,
    pub set_timer: BootSetTimer,
    pub wait_for_event: BootWaitForEvent,
    pub signal_event: BootSignalEvent,
    pub close_event: BootCloseEvent,
    pub check_event: BootCheckEvent,

    pub install_protocol_interface: BootInstallProtocolInterface,
    pub reinstall_protocol_interface: BootReinstallProtocolInterface,
    pub uninstall_protocol_interface: BootUninstallProtocolInterface,
    pub handle_protocol: BootHandleProtocol,
    pub reserved: *mut core::ffi::c_void,
    pub register_protocol_notify: BootRegisterProtocolNotify,
    pub locate_handle: BootLocateHandle,
    pub locate_device_path: BootLocateDevicePath,

    pub install_configuration_table: BootInstallConfigurationTable,

    pub load_image: BootLoadImage,
    pub start_image: BootStartImage,
    pub exit: BootExit,
    pub unload_image: BootUnloadImage,
    pub exit_boot_services: BootExitBootServices,

    pub get_next_monotonic_count: BootGetNextMonotonicCount,
    pub stall: BootStall,
    pub set_watchdog_timer: BootSetWatchdogTimer,

    // 1.1+
    pub connect_controller: BootConnectController,
    pub disconnect_controller: BootDisconnectController,

    pub open_protocol: BootOpenProtocol,
    pub close_protocol: BootCloseProtocol,
    pub open_protocol_information: BootOpenProtocolInformation,

    pub protocols_per_handle: BootProtocolsPerHandle,
    pub locate_handle_buffer: BootLocateHandleBuffer,
    pub locate_protocol: BootLocateProtocol,
    pub install_multiple_protocol_interfaces: BootInstallMultipleProtocolInterfaces,
    pub uninstall_multiple_protocol_interfaces: BootUninstallMultipleProtocolInterfaces,

    pub calculate_crc32: BootCalculateCrc32,

    pub copy_mem: BootCopyMem,
    pub set_mem: BootSetMem,

    // 2.0+
    pub create_event_ex: BootCreateEventEx,
}

pub const SYSTEM_TABLE_REVISION_2_70: u32 = (2 << 16) | (70);
pub const SYSTEM_TABLE_REVISION_2_60: u32 = (2 << 16) | (60);
pub const SYSTEM_TABLE_REVISION_2_50: u32 = (2 << 16) | (50);
pub const SYSTEM_TABLE_REVISION_2_40: u32 = (2 << 16) | (40);
pub const SYSTEM_TABLE_REVISION_2_31: u32 = (2 << 16) | (31);
pub const SYSTEM_TABLE_REVISION_2_30: u32 = (2 << 16) | (30);
pub const SYSTEM_TABLE_REVISION_2_20: u32 = (2 << 16) | (20);
pub const SYSTEM_TABLE_REVISION_2_10: u32 = (2 << 16) | (10);
pub const SYSTEM_TABLE_REVISION_2_00: u32 = (2 << 16) | (0);
pub const SYSTEM_TABLE_REVISION_1_10: u32 = (1 << 16) | (10);
pub const SYSTEM_TABLE_REVISION_1_02: u32 = (1 << 16) | (2);

pub const SYSTEM_TABLE_SIGNATURE: u64 = 0x5453595320494249u64; // "IBI SYST"
pub const SYSTEM_TABLE_REVISION: u32 = SYSTEM_TABLE_REVISION_2_70;

#[repr(C)]
pub struct SystemTable {
    pub hdr: TableHeader,
    pub firmware_vendor: *mut crate::base::Char16,
    pub firmware_revision: u32,

    pub console_in_handle: crate::base::Handle,
    pub con_in: *mut crate::protocols::simple_text_input::Protocol,
    pub console_out_handle: crate::base::Handle,
    pub con_out: *mut crate::protocols::simple_text_output::Protocol,
    pub standard_error_handle: crate::base::Handle,
    pub std_err: *mut crate::protocols::simple_text_output::Protocol,

    pub runtime_services: *mut RuntimeServices,
    pub boot_services: *mut BootServices,

    pub number_of_table_entries: usize,
    pub configuration_table: *mut ConfigurationTable,
}
