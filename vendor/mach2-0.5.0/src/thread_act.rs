//! This module corresponds to `mach/thread_act.defs`.

use exception_types::{exception_behavior_t, exception_mask_t};
use kern_return::kern_return_t;
use mach_types::{thread_act_t, thread_port_t};
use message::mach_msg_type_number_t;
use port::mach_port_t;
use thread_status::{thread_state_flavor_t, thread_state_t};

extern "C" {
    pub fn thread_get_state(
        target_act: thread_act_t,
        flavor: thread_state_flavor_t,
        new_state: thread_state_t,
        new_state_count: *mut mach_msg_type_number_t,
    ) -> kern_return_t;

    pub fn thread_set_state(
        target_act: thread_port_t,
        flavor: thread_state_flavor_t,
        new_state: thread_state_t,
        new_stateCnt: mach_msg_type_number_t,
    ) -> kern_return_t;

    pub fn thread_set_exception_ports(
        thread: thread_port_t,
        exception_mask: exception_mask_t,
        new_port: mach_port_t,
        behavior: exception_behavior_t,
        new_flavor: thread_state_flavor_t,
    ) -> kern_return_t;

    pub fn thread_suspend(target_act: thread_act_t) -> kern_return_t;

    pub fn thread_resume(target_act: thread_act_t) -> kern_return_t;
}
