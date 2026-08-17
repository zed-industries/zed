//! This module corresponds to `mach/task.defs`.

use kern_return::kern_return_t;
use mach_types::{task_name_t, task_t, thread_act_array_t};
use message::mach_msg_type_number_t;
use port::{mach_port_array_t, mach_port_t};
use task_info::{task_flavor_t, task_info_t};

pub type task_special_port_t = ::libc::c_int;

pub const TASK_KERNEL_PORT: task_special_port_t = 1;
pub const TASK_HOST_PORT: task_special_port_t = 2;
pub const TASK_NAME_PORT: task_special_port_t = 3;
pub const TASK_BOOTSTRAP_PORT: task_special_port_t = 4;

extern "C" {
    pub fn task_resume(target_task: task_t) -> kern_return_t;
    pub fn task_suspend(target_task: task_t) -> kern_return_t;
    pub fn task_get_special_port(
        task: task_t,
        which_port: task_special_port_t,
        special_port: *mut mach_port_t,
    ) -> kern_return_t;
    pub fn task_threads(
        target_task: task_t,
        act_list: *mut thread_act_array_t,
        act_list_cnt: *mut mach_msg_type_number_t,
    ) -> kern_return_t;
    pub fn task_info(
        target_task: task_name_t,
        flavor: task_flavor_t,
        task_info_out: task_info_t,
        task_info_outCnt: *mut mach_msg_type_number_t,
    ) -> kern_return_t;
    pub fn task_set_info(
        target_task: task_t,
        flavor: task_flavor_t,
        task_info_in: task_info_t,
        task_info_inCnt: mach_msg_type_number_t,
    ) -> kern_return_t;
    pub fn mach_ports_lookup(
        target_task: task_t,
        init_port_set: *mut mach_port_array_t,
        init_port_setCnt: *mut mach_msg_type_number_t,
    ) -> kern_return_t;
}
