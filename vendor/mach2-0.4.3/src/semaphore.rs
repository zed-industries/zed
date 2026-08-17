//! This module corresponds to `mach/semaphore.h`

use clock_types::mach_timespec_t;
use kern_return::kern_return_t;
use mach_types::{semaphore_t, task_t};
use sync_policy::sync_policy_t;

extern "C" {
    pub fn semaphore_create(
        task: task_t,
        semaphore: *mut semaphore_t,
        policy: sync_policy_t,
        value: libc::c_int,
    ) -> kern_return_t;
    pub fn semaphore_signal(semaphore: *mut semaphore_t) -> kern_return_t;
    pub fn semaphore_wait(semaphore: *mut semaphore_t) -> kern_return_t;
    pub fn semaphore_timedwait(
        semaphore: *mut semaphore_t,
        timeout: mach_timespec_t,
    ) -> kern_return_t;
    pub fn semaphore_destroy(task: task_t, semaphore: *mut semaphore_t) -> kern_return_t;
}
