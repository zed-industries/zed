// Take a look at the license at the top of the repository in the LICENSE file.

#[allow(deprecated)]
use libc::{mach_timebase_info, mach_timebase_info_data_t};

use libc::{
    host_processor_info, mach_port_t, munmap, natural_t, processor_cpu_load_info,
    processor_cpu_load_info_t, sysconf, vm_page_size, PROCESSOR_CPU_LOAD_INFO, _SC_CLK_TCK,
};
use std::ptr::null_mut;
use std::time::Instant;

struct ProcessorCpuLoadInfo {
    cpu_load: processor_cpu_load_info_t,
    cpu_count: natural_t,
}

impl ProcessorCpuLoadInfo {
    fn new(port: mach_port_t) -> Option<Self> {
        let mut info_size = std::mem::size_of::<processor_cpu_load_info_t>() as _;
        let mut cpu_count = 0;
        let mut cpu_load: processor_cpu_load_info_t = null_mut();

        unsafe {
            if host_processor_info(
                port,
                PROCESSOR_CPU_LOAD_INFO,
                &mut cpu_count,
                &mut cpu_load as *mut _ as *mut _,
                &mut info_size,
            ) != 0
            {
                sysinfo_debug!("host_processor_info failed, not updating CPU ticks usage...");
                None
            } else if cpu_count < 1 || cpu_load.is_null() {
                None
            } else {
                Some(Self {
                    cpu_load,
                    cpu_count,
                })
            }
        }
    }
}

impl Drop for ProcessorCpuLoadInfo {
    fn drop(&mut self) {
        unsafe {
            munmap(self.cpu_load as _, vm_page_size);
        }
    }
}

pub(crate) struct SystemTimeInfo {
    timebase_to_ns: f64,
    clock_per_sec: f64,
    old_cpu_info: ProcessorCpuLoadInfo,
    last_update: Option<Instant>,
    prev_time_interval: f64,
}

unsafe impl Send for SystemTimeInfo {}
unsafe impl Sync for SystemTimeInfo {}

impl SystemTimeInfo {
    #[allow(deprecated)] // Everything related to mach_timebase_info_data_t
    pub fn new(port: mach_port_t) -> Option<Self> {
        unsafe {
            let clock_ticks_per_sec = sysconf(_SC_CLK_TCK);

            // FIXME: Maybe check errno here? Problem is that if errno is not 0 before this call,
            //        we will get an error which isn't related...
            // if let Some(er) = std::io::Error::last_os_error().raw_os_error() {
            //     if err != 0 {
            //         println!("==> {:?}", er);
            //         sysinfo_debug!("Failed to get _SC_CLK_TCK value, using old CPU tick measure system");
            //         return None;
            //     }
            // }

            let mut info = mach_timebase_info_data_t { numer: 0, denom: 0 };
            if mach_timebase_info(&mut info) != libc::KERN_SUCCESS {
                sysinfo_debug!("mach_timebase_info failed, using default value of 1");
                info.numer = 1;
                info.denom = 1;
            }

            let old_cpu_info = match ProcessorCpuLoadInfo::new(port) {
                Some(cpu_info) => cpu_info,
                None => {
                    sysinfo_debug!("host_processor_info failed, using old CPU tick measure system");
                    return None;
                }
            };

            let nano_per_seconds = 1_000_000_000.;
            sysinfo_debug!("");
            Some(Self {
                timebase_to_ns: info.numer as f64 / info.denom as f64,
                clock_per_sec: nano_per_seconds / clock_ticks_per_sec as f64,
                old_cpu_info,
                last_update: None,
                prev_time_interval: 0.,
            })
        }
    }

    pub fn get_time_interval(&mut self, port: mach_port_t) -> f64 {
        let need_cpu_usage_update = self
            .last_update
            .map(|last_update| last_update.elapsed() > crate::MINIMUM_CPU_UPDATE_INTERVAL)
            .unwrap_or(true);
        if need_cpu_usage_update {
            let mut total = 0;
            let new_cpu_info = match ProcessorCpuLoadInfo::new(port) {
                Some(cpu_info) => cpu_info,
                None => return 0.,
            };
            let cpu_count = std::cmp::min(self.old_cpu_info.cpu_count, new_cpu_info.cpu_count);
            unsafe {
                for i in 0..cpu_count {
                    let new_load: &processor_cpu_load_info = &*new_cpu_info.cpu_load.offset(i as _);
                    let old_load: &processor_cpu_load_info =
                        &*self.old_cpu_info.cpu_load.offset(i as _);
                    for (new, old) in new_load.cpu_ticks.iter().zip(old_load.cpu_ticks.iter()) {
                        if new > old {
                            total += new.saturating_sub(*old);
                        }
                    }
                }
            }

            self.old_cpu_info = new_cpu_info;
            self.last_update = Some(Instant::now());

            // Now we convert the ticks to nanoseconds (if the interval is less than
            // `MINIMUM_CPU_UPDATE_INTERVAL`, we replace it with it instead):
            let base_interval = total as f64 / cpu_count as f64 * self.clock_per_sec;
            let smallest = crate::MINIMUM_CPU_UPDATE_INTERVAL.as_secs_f64() * 1_000_000_000.0;
            self.prev_time_interval = if base_interval < smallest {
                smallest
            } else {
                base_interval / self.timebase_to_ns
            };
            self.prev_time_interval
        } else {
            self.prev_time_interval
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    /// Regression test for <https://github.com/GuillaumeGomez/sysinfo/issues/956>.
    #[test]
    fn test_getting_time_interval() {
        if !crate::IS_SUPPORTED_SYSTEM || cfg!(feature = "apple-sandbox") {
            return;
        }

        let port = unsafe { libc::mach_host_self() };
        let mut info = SystemTimeInfo::new(port).unwrap();
        info.get_time_interval(port);

        std::thread::sleep(crate::MINIMUM_CPU_UPDATE_INTERVAL.saturating_mul(5));

        let val = info.get_time_interval(port);
        assert_ne!(
            val,
            crate::MINIMUM_CPU_UPDATE_INTERVAL.as_secs_f64() * 1_000_000_000.0
        );
    }
}
