//! This module roughly corresponds to `mach/clock_priv.h`.

use clock_types::{clock_attr_t, clock_flavor_t, mach_timespec_t};
use kern_return::kern_return_t;
use mach_types::clock_ctrl_t;
use message::mach_msg_type_number_t;

extern "C" {
    pub fn clock_set_time(clock_ctrl: clock_ctrl_t, new_time: mach_timespec_t) -> kern_return_t;
    pub fn clock_set_attributes(
        clock_ctrl: clock_ctrl_t,
        flavor: clock_flavor_t,
        clock_attr: clock_attr_t,
        clock_attrCnt: mach_msg_type_number_t,
    ) -> kern_return_t;
}
