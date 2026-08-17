use libc::c_double;

extern "C" {
    /// @function   CVGetCurrentHostTime
    /// @abstract   Retrieve the current value of the host time base.
    /// @discussion On Mac OS X, the host time base for CoreVideo and CoreAudio are identical, and the values returned from either API
    ///             may be used interchangeably.
    /// @result     The current host time.
    pub fn CVGetCurrentHostTime() -> u64;
    /// @function   CVGetHostClockFrequency
    /// @abstract   Retrieve the frequency of the host time base.
    /// @discussion On Mac OS X, the host time base for CoreVideo and CoreAudio are identical, and the values returned from either API
    ///             may be used interchangeably.
    /// @result     The current host frequency.
    pub fn CVGetHostClockFrequency() -> c_double;
    /// @function   CVGetHostClockMinimumTimeDelta
    /// @abstract   Retrieve the smallest possible increment in the host time base.
    /// @result     The smallest valid increment in the host time base.
    pub fn CVGetHostClockMinimumTimeDelta() -> u32;
}

#[test]
fn test_get_curr_time() {
    unsafe {
        assert!(CVGetCurrentHostTime() > 0);
    }
}
