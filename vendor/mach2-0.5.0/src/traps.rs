//! This module corresponds to `mach/mach_traps.h`.
use kern_return::kern_return_t;
use port::{mach_port_name_t, mach_port_t};

extern "C" {
    static mach_task_self_: mach_port_t;
    pub fn task_for_pid(
        target_tport: mach_port_name_t,
        pid: ::libc::c_int,
        tn: *mut mach_port_name_t,
    ) -> kern_return_t;
}

#[allow(clippy::missing_safety_doc)] // FIXME
pub unsafe fn mach_task_self() -> mach_port_t {
    mach_task_self_
}

#[allow(clippy::missing_safety_doc)] // FIXME
pub unsafe fn current_task() -> mach_port_t {
    mach_task_self()
}

#[cfg(test)]
mod tests {
    use port::*;
    use traps::*;

    #[test]
    fn mach_task_self_sanity() {
        unsafe {
            let this_task = mach_task_self();
            assert!(this_task != MACH_PORT_NULL);
            assert!(this_task != MACH_PORT_DEAD);
        }
    }
}
