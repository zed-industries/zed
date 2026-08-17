//! This module corresponds to `mach/mach_time.h`
use kern_return::kern_return_t;
pub type mach_timebase_info_t = *mut mach_timebase_info;
pub type mach_timebase_info_data_t = mach_timebase_info;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct mach_timebase_info {
    pub numer: u32,
    pub denom: u32,
}

extern "C" {
    pub fn mach_timebase_info(info: mach_timebase_info_t) -> kern_return_t;
    pub fn mach_wait_until(deadline: u64) -> kern_return_t;
    pub fn mach_absolute_time() -> u64;
    pub fn mach_approximate_time() -> u64;
    pub fn mach_continuous_time() -> u64;
    pub fn mach_continuous_approximate_time() -> u64;
}
