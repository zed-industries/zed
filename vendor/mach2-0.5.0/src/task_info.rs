//! This module roughly corresponds to `mach/task_info.h`.

use mem;
use message::{audit_token_t, security_token_t};
use time_value::time_value_t;
use vm_statistics::{vm_extmod_statistics_data_t, vm_purgeable_info};
use vm_types::{integer_t, mach_vm_address_t, mach_vm_size_t, natural_t, vm_size_t};

pub const TASK_INFO_MAX: ::libc::c_uint = 1024;
pub const TASK_BASIC_INFO_32: ::libc::c_uint = 4;
pub const TASK_BASIC2_INFO_32: ::libc::c_uint = 6;
#[cfg(target_arch = "x86_64")]
pub const TASK_BASIC_INFO_64: ::libc::c_uint = 5;
#[cfg(target_arch = "aarch64")]
pub const TASK_BASIC_INFO_64: ::libc::c_uint = 18;
#[cfg(target_arch = "x86_64")]
pub const TASK_BASIC_INFO: ::libc::c_uint = 5;
#[cfg(target_arch = "aarch64")]
pub const TASK_BASIC_INFO: ::libc::c_uint = 18;
#[cfg(target_arch = "x86")]
pub const TASK_BASIC_INFO: ::libc::c_uint = 4;
pub const TASK_EVENTS_INFO: ::libc::c_uint = 2;
pub const TASK_THREAD_TIMES_INFO: ::libc::c_uint = 3;
pub const TASK_ABSOLUTETIME_INFO: ::libc::c_uint = 1;
pub const TASK_KERNELMEMORY_INFO: ::libc::c_uint = 7;
pub const TASK_SECURITY_TOKEN: ::libc::c_uint = 13;
pub const TASK_AUDIT_TOKEN: ::libc::c_uint = 15;
pub const TASK_AFFINITY_TAG_INFO: ::libc::c_uint = 16;
pub const TASK_DYLD_INFO: ::libc::c_uint = 17;
pub const TASK_DYLD_ALL_IMAGE_INFO_32: ::libc::c_uint = 0;
pub const TASK_DYLD_ALL_IMAGE_INFO_64: ::libc::c_uint = 1;
pub const TASK_EXTMOD_INFO: ::libc::c_uint = 19;
pub const MACH_TASK_BASIC_INFO: ::libc::c_uint = 20;
pub const TASK_POWER_INFO: ::libc::c_uint = 21;
pub const TASK_VM_INFO: ::libc::c_uint = 22;
pub const TASK_VM_INFO_PURGEABLE: ::libc::c_uint = 23;
pub const TASK_TRACE_MEMORY_INFO: ::libc::c_uint = 24;
pub const TASK_WAIT_STATE_INFO: ::libc::c_uint = 25;
pub const TASK_POWER_INFO_V2: ::libc::c_uint = 26;
pub const TASK_VM_INFO_PURGEABLE_ACCOUNT: ::libc::c_uint = 27;
pub const TASK_FLAGS_INFO: ::libc::c_uint = 28;
pub const TASK_DEBUG_INFO_INTERNAL: ::libc::c_uint = 29;

pub type task_flavor_t = natural_t;
pub type task_info_t = *mut integer_t;
pub type policy_t = ::libc::c_int;

#[repr(C, packed(4))]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct task_basic_info_32 {
    pub suspend_count: integer_t,
    pub virtual_size: natural_t,
    pub resident_size: natural_t,
    pub user_time: time_value_t,
    pub system_time: time_value_t,
    pub policy: policy_t,
}

pub const TASK_BASIC_INFO_32_COUNT: u32 =
    (mem::size_of::<task_basic_info_32>() / mem::size_of::<natural_t>()) as u32;

#[repr(C, packed(4))]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct task_basic_info_64 {
    pub suspend_count: integer_t,
    pub virtual_size: mach_vm_size_t,
    pub resident_size: mach_vm_size_t,
    pub user_time: time_value_t,
    pub system_time: time_value_t,
    pub policy: policy_t,
}

pub const TASK_BASIC_INFO_64_COUNT: u32 =
    (mem::size_of::<task_basic_info_64>() / mem::size_of::<natural_t>()) as u32;

#[repr(C, packed(4))]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct task_basic_info {
    pub suspend_count: integer_t,
    pub virtual_size: vm_size_t,
    pub resident_size: vm_size_t,
    pub user_time: time_value_t,
    pub system_time: time_value_t,
    pub policy: policy_t,
}

pub const TASK_BASIC_INFO_COUNT: u32 =
    (mem::size_of::<task_basic_info>() / mem::size_of::<natural_t>()) as u32;

#[repr(C, packed(4))]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct task_events_info {
    pub faults: integer_t,
    pub pageins: integer_t,
    pub cow_faults: integer_t,
    pub messages_sent: integer_t,
    pub messages_received: integer_t,
    pub syscalls_mach: integer_t,
    pub syscalls_unix: integer_t,
    pub csw: integer_t,
}

pub const TASK_EVENTS_INFO_COUNT: u32 =
    (mem::size_of::<task_events_info>() / mem::size_of::<natural_t>()) as u32;

#[repr(C, packed(4))]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct task_thread_times_info {
    pub user_time: time_value_t,
    pub system_time: time_value_t,
}

pub const TASK_THREAD_TIMES_INFO_COUNT: u32 =
    (mem::size_of::<task_thread_times_info>() / mem::size_of::<natural_t>()) as u32;

#[repr(C, packed(4))]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct task_absolutetime_info {
    pub total_user: u64,
    pub total_system: u64,
    pub threads_user: u64,
    pub threads_system: u64,
}

pub const TASK_ABSOLUTETIME_INFO_COUNT: u32 =
    (mem::size_of::<task_absolutetime_info>() / mem::size_of::<natural_t>()) as u32;

#[repr(C, packed(4))]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct task_kernelmemory_info {
    pub total_palloc: u64,
    pub total_pfree: u64,
    pub total_salloc: u64,
    pub total_sfree: u64,
}

pub const TASK_KERNELMEMORY_INFO_COUNT: u32 =
    (mem::size_of::<task_kernelmemory_info>() / mem::size_of::<natural_t>()) as u32;

pub const TASK_SECURITY_TOKEN_COUNT: u32 =
    (mem::size_of::<security_token_t>() / mem::size_of::<natural_t>()) as u32;

pub const TASK_AUDIT_TOKEN_COUNT: u32 =
    (mem::size_of::<audit_token_t>() / mem::size_of::<natural_t>()) as u32;

#[repr(C, packed(4))]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct task_affinity_tag_info {
    pub set_count: integer_t,
    pub min: integer_t,
    pub max: integer_t,
    pub task_count: integer_t,
}

#[repr(C, packed(4))]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct task_dyld_info {
    pub all_image_info_addr: mach_vm_address_t,
    pub all_image_info_size: mach_vm_size_t,
    pub all_image_info_format: integer_t,
}

pub const TASK_DYLD_INFO_COUNT: u32 =
    (mem::size_of::<task_dyld_info>() / mem::size_of::<natural_t>()) as u32;

#[repr(C, packed(4))]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct task_extmod_info {
    pub task_uuid: [::libc::c_uchar; 16usize],
    pub extmod_statistics: vm_extmod_statistics_data_t,
}

pub const TASK_EXTMOD_INFO_COUNT: u32 =
    (mem::size_of::<task_extmod_info>() / mem::size_of::<natural_t>()) as u32;

#[repr(C, packed(4))]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct task_power_info {
    pub total_user: u64,
    pub total_system: u64,
    pub task_interrupt_wakeups: u64,
    pub task_platform_idle_wakeups: u64,
    pub task_timer_wakeups_bin_1: u64,
    pub task_timer_wakeups_bin_2: u64,
}

pub const TASK_POWER_INFO_COUNT: u32 =
    (mem::size_of::<task_power_info>() / mem::size_of::<natural_t>()) as u32;

#[repr(C, packed(4))]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct task_vm_info {
    pub virtual_size: mach_vm_size_t,
    pub region_count: integer_t,
    pub page_size: integer_t,
    pub resident_size: mach_vm_size_t,
    pub resident_size_peak: mach_vm_size_t,
    pub device: mach_vm_size_t,
    pub device_peak: mach_vm_size_t,
    pub internal: mach_vm_size_t,
    pub internal_peak: mach_vm_size_t,
    pub external: mach_vm_size_t,
    pub external_peak: mach_vm_size_t,
    pub reusable: mach_vm_size_t,
    pub reusable_peak: mach_vm_size_t,
    pub purgeable_volatile_pmap: mach_vm_size_t,
    pub purgeable_volatile_resident: mach_vm_size_t,
    pub purgeable_volatile_virtual: mach_vm_size_t,
    pub compressed: mach_vm_size_t,
    pub compressed_peak: mach_vm_size_t,
    pub compressed_lifetime: mach_vm_size_t,
    pub phys_footprint: mach_vm_size_t,
    pub min_address: mach_vm_address_t,
    pub max_address: mach_vm_address_t,
    pub ledger_phys_footprint_peak: i64,
    pub ledger_purgeable_nonvolatile: i64,
    pub ledger_purgeable_novolatile_compressed: i64,
    pub ledger_purgeable_volatile: i64,
    pub ledger_purgeable_volatile_compressed: i64,
    pub ledger_tag_network_nonvolatile: i64,
    pub ledger_tag_network_nonvolatile_compressed: i64,
    pub ledger_tag_network_volatile: i64,
    pub ledger_tag_network_volatile_compressed: i64,
    pub ledger_tag_media_footprint: i64,
    pub ledger_tag_media_footprint_compressed: i64,
    pub ledger_tag_media_nofootprint: i64,
    pub ledger_tag_media_nofootprint_compressed: i64,
    pub ledger_tag_graphics_footprint: i64,
    pub ledger_tag_graphics_footprint_compressed: i64,
    pub ledger_tag_graphics_nofootprint: i64,
    pub ledger_tag_graphics_nofootprint_compressed: i64,
    pub ledger_tag_neural_footprint: i64,
    pub ledger_tag_neural_footprint_compressed: i64,
    pub ledger_tag_neural_nofootprint: i64,
    pub ledger_tag_neural_nofootprint_compressed: i64,
    pub limit_bytes_remaining: u64,
    pub decompressions: integer_t,
    pub ledger_swapins: i64,
    pub ledger_tag_neural_nofootprint_total: i64,
    pub ledger_tag_neural_nofootprint_peak: i64,
}

#[repr(C, packed(4))]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct mach_task_basic_info {
    pub virtual_size: mach_vm_size_t,
    pub resident_size: mach_vm_size_t,
    pub resident_size_max: mach_vm_size_t,
    pub user_time: time_value_t,
    pub system_time: time_value_t,
    pub policy: policy_t,
    pub suspend_count: integer_t,
}

pub const MACH_TASK_BASIC_INFO_COUNT: u32 =
    (mem::size_of::<mach_task_basic_info>() / mem::size_of::<natural_t>()) as u32;

#[repr(C, packed(4))]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct task_trace_memory_info {
    pub user_memory_address: u64,
    pub buffer_size: u64,
    pub mailbox_array_size: u64,
}

#[repr(C, packed(4))]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct task_wait_state_info {
    pub total_wait_state_time: u64,
    pub total_wait_sfi_state_time: u64,
    pub _reserved: [u32; 4],
}

#[repr(C, packed(4))]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct gpu_energy_data {
    pub task_gpu_utilisation: u64,
    pub task_gpu_stat_reserved0: u64,
    pub task_gpu_stat_reserved1: u64,
    pub task_gpu_stat_reserved2: u64,
}

#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
#[repr(C, packed(4))]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct task_power_info_v2 {
    pub cpu_energy: task_power_info,
    pub gpu_energy: gpu_energy_data,
    pub task_energy: u64,
    pub task_ptime: u64,
    pub task_pset_switches: u64,
}

#[cfg(not(any(target_arch = "arm", target_arch = "aarch64")))]
#[repr(C, packed(4))]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct task_power_info_v2 {
    pub cpu_energy: task_power_info,
    pub gpu_energy: gpu_energy_data,
    pub task_ptime: u64,
    pub task_pset_switches: u64,
}

pub const TASK_POWER_INFO_V2_COUNT: u32 =
    (mem::size_of::<task_power_info_v2>() / mem::size_of::<natural_t>()) as u32;

pub type task_purgable_info_t = vm_purgeable_info;

#[repr(C, packed(4))]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct task_flags_info {
    pub flags: u32,
}

pub const TASK_FLAGS_INFO_COUNT: u32 =
    (mem::size_of::<task_flags_info>() / mem::size_of::<natural_t>()) as u32;
