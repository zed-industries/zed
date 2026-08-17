use libc::c_double;

extern "C" {
    pub fn CVGetCurrentHostTime() -> u64;
    pub fn CVGetHostClockFrequency() -> c_double;
    pub fn CVGetHostClockMinimumTimeDelta() -> u32;
}

pub fn get_current_host_time() -> u64 {
    unsafe { CVGetCurrentHostTime() }
}

pub fn get_host_clock_frequency() -> f64 {
    unsafe { CVGetHostClockFrequency() }
}

pub fn get_host_clock_minimum_time_delta() -> u32 {
    unsafe { CVGetHostClockMinimumTimeDelta() }
}
