use crate::cvt::cvt;
use std::io;
use windows_sys::Win32::System::Performance::QueryPerformanceFrequency;

pub fn perf_counter_frequency() -> io::Result<u64> {
    unsafe {
        let mut frequency = 0;
        cvt(QueryPerformanceFrequency(&mut frequency))?;
        Ok(frequency as u64)
    }
}
