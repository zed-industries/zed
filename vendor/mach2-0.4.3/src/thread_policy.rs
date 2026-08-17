//! This module corresponds to `mach/thread_policy.h`.

use boolean::boolean_t;
use kern_return::kern_return_t;
use libc::thread_policy_t;
use mach_types::thread_t;
use message::mach_msg_type_number_t;
use vm_types::{integer_t, natural_t};

pub type thread_policy_flavor_t = natural_t;

pub const THREAD_STANDARD_POLICY: thread_policy_flavor_t = 1;
pub const THREAD_EXTENDED_POLICY: thread_policy_flavor_t = 1;
pub const THREAD_TIME_CONSTRAINT_POLICY: thread_policy_flavor_t = 2;
pub const THREAD_PRECEDENCE_POLICY: thread_policy_flavor_t = 3;
pub const THREAD_AFFINITY_POLICY: thread_policy_flavor_t = 4;
pub const THREAD_BACKGROUND_POLICY: thread_policy_flavor_t = 5;
pub const THREAD_LATENCY_QOS_POLICY: thread_policy_flavor_t = 7;
pub const THREAD_THROUGHPUT_QOS_POLICY: thread_policy_flavor_t = 8;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thread_standard_policy {
    pub no_data: natural_t,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thread_extended_policy {
    pub timeshare: boolean_t,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thread_time_constraint_policy {
    pub period: u32,
    pub computation: u32,
    pub constraint: u32,
    pub preemptible: boolean_t,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thread_precedence_policy {
    pub importance: integer_t,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thread_affinity_policy {
    pub affinity_tag: integer_t,
}

pub const THREAD_AFFINITY_TAG_NULL: integer_t = 0;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thread_background_policy {
    pub priority: integer_t,
}

pub type thread_latency_qos_t = integer_t;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thread_latency_qos_policy {
    thread_latency_qos_tier: thread_latency_qos_t,
}

pub type thread_throughput_qos_t = integer_t;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thread_throughput_qos_policy {
    thread_throughput_qos_tier: thread_throughput_qos_t,
}

pub type thread_standard_policy_data_t = thread_standard_policy;
pub type thread_extended_policy_data_t = thread_extended_policy;
pub type thread_time_constraint_policy_data_t = thread_time_constraint_policy;
pub type thread_precedence_policy_data_t = thread_precedence_policy;
pub type thread_affinity_policy_data_t = thread_affinity_policy;
pub type thread_background_policy_data_t = thread_background_policy;
pub type thread_latency_qos_policy_data_t = thread_latency_qos_policy;
pub type thread_throughput_qos_policy_data_t = thread_throughput_qos_policy;

pub const THREAD_STANDARD_POLICY_COUNT: mach_msg_type_number_t = 0;
pub const THREAD_EXTENDED_POLICY_COUNT: mach_msg_type_number_t =
    (core::mem::size_of::<thread_extended_policy>() / core::mem::size_of::<integer_t>()) as _;
pub const THREAD_TIME_CONSTRAINT_POLICY_COUNT: mach_msg_type_number_t =
    (core::mem::size_of::<thread_time_constraint_policy>() / core::mem::size_of::<integer_t>())
        as _;
pub const THREAD_PRECEDENCE_POLICY_COUNT: mach_msg_type_number_t =
    (core::mem::size_of::<thread_precedence_policy>() / core::mem::size_of::<integer_t>()) as _;
pub const THREAD_AFFINITY_POLICY_COUNT: mach_msg_type_number_t =
    (core::mem::size_of::<thread_affinity_policy>() / core::mem::size_of::<integer_t>()) as _;
pub const THREAD_BACKGROUND_POLICY_COUNT: mach_msg_type_number_t =
    (core::mem::size_of::<thread_background_policy>() / core::mem::size_of::<integer_t>()) as _;
pub const THREAD_LATENCY_QOS_POLICY_COUNT: mach_msg_type_number_t =
    (core::mem::size_of::<thread_latency_qos_policy>() / core::mem::size_of::<integer_t>()) as _;
pub const THREAD_THROUGHPUT_QOS_POLICY_COUNT: mach_msg_type_number_t =
    (core::mem::size_of::<thread_throughput_qos_policy>() / core::mem::size_of::<integer_t>()) as _;

extern "C" {
    pub fn thread_policy_set(
        thread: thread_t,
        flavor: thread_policy_flavor_t,
        policy_info: thread_policy_t,
        count: mach_msg_type_number_t,
    ) -> kern_return_t;
}

extern "C" {
    pub fn thread_policy_get(
        thread: thread_t,
        flavor: thread_policy_flavor_t,
        policy_info: thread_policy_t,
        count: *mut mach_msg_type_number_t,
        get_default: *mut boolean_t,
    ) -> kern_return_t;
}
