//! This module roughly corresponds to `mach/clock_reply.h`.

use clock_types::{alarm_type_t, mach_timespec_t};
use kern_return::kern_return_t;
use mach_types::clock_reply_t;
use message::mach_msg_type_name_t;

extern "C" {
    pub fn clock_alarm_reply(
        alarm_port: clock_reply_t,
        alarm_portPoly: mach_msg_type_name_t,
        alarm_code: kern_return_t,
        alarm_type: alarm_type_t,
        alarm_time: mach_timespec_t,
    ) -> kern_return_t;
}
