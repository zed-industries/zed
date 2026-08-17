//! Multi-Processor Services Protocol
//!
//! This Protocol is defined in the UEFI Platform Integration Specification,
//! Section 13.4.
//!
//! This provides a generalized way of performing the following tasks:
//! - Retrieving information of multi-processor environments.
//! - Dispatching user-provided function to APs.
//! - Maintain MP-related processor status.

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0x3fdda605,
    0xa76e,
    0x4f46,
    0xad,
    0x29,
    &[0x12, 0xf4, 0x53, 0x1b, 0x3d, 0x08],
);

pub const PROCESSOR_AS_BSP_BIT: u32 = 0x00000001;
pub const PROCESSOR_ENABLED_BIT: u32 = 0x00000002;
pub const PROCESSOR_HEALTH_STATUS_BIT: u32 = 0x00000004;

pub const END_OF_CPU_LIST: usize = usize::MAX;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CpuPhysicalLocation {
    pub package: u32,
    pub core: u32,
    pub thread: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CpuPhysicalLocation2 {
    pub package: u32,
    pub module: u32,
    pub tile: u32,
    pub die: u32,
    pub core: u32,
    pub thread: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union ExtendedProcessorInformation {
    pub location2: CpuPhysicalLocation2,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ProcessorInformation {
    pub processor_id: u64,
    pub status_flag: u32,
    pub location: CpuPhysicalLocation,
    pub extended_information: ExtendedProcessorInformation,
}

pub type ApProcedure = eficall! {fn(*mut core::ffi::c_void)};

pub type GetNumberOfProcessors = eficall! {fn(
    *mut Protocol,
    *mut usize,
    *mut usize,
) -> crate::base::Status};

pub type GetProcessorInfo = eficall! {fn(
    *mut Protocol,
    usize,
    *mut ProcessorInformation,
) -> crate::base::Status};

pub type StartupAllAps = eficall! {fn(
    *mut Protocol,
    ApProcedure,
    crate::base::Boolean,
    crate::base::Event,
    usize,
    *mut core::ffi::c_void,
    *mut *mut usize,
) -> crate::base::Status};

pub type StartupThisAp = eficall! {fn(
    *mut Protocol,
    ApProcedure,
    usize,
    crate::base::Event,
    usize,
    *mut core::ffi::c_void,
    *mut crate::base::Boolean,
) -> crate::base::Status};

pub type SwitchBsp = eficall! {fn(
    *mut Protocol,
    usize,
    crate::base::Boolean,
) -> crate::base::Status};

pub type EnableDisableAp = eficall! {fn(
    *mut Protocol,
    usize,
    crate::base::Boolean,
    *mut u32,
) -> crate::base::Status};

pub type WhoAmI = eficall! {fn(
    *mut Protocol,
    *mut usize,
) -> crate::base::Status};

#[repr(C)]
pub struct Protocol {
    pub get_number_of_processors: GetNumberOfProcessors,
    pub get_processor_info: GetProcessorInfo,
    pub startup_all_aps: StartupAllAps,
    pub startup_this_ap: StartupThisAp,
    pub switch_bsp: SwitchBsp,
    pub enable_disable_ap: EnableDisableAp,
    pub who_am_i: WhoAmI,
}
