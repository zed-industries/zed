// Take a look at the license at the top of the repository in the LICENSE file.

use crate::sys::utils::{
    get_sys_value_array, get_sys_value_by_name, get_sys_value_str_by_name, init_mib,
};
use crate::{Cpu, CpuRefreshKind};

use libc::{c_int, c_ulong};

pub(crate) struct CpusWrapper {
    pub(crate) global_cpu_usage: f32,
    pub(crate) cpus: Vec<Cpu>,
    got_cpu_frequency: bool,
    mib_cp_time: [c_int; 2],
    mib_cp_times: [c_int; 2],
    // For the global CPU usage.
    cp_time: VecSwitcher<c_ulong>,
    // For each CPU usage.
    cp_times: VecSwitcher<c_ulong>,
    nb_cpus: usize,
}

impl CpusWrapper {
    pub(crate) fn new() -> Self {
        let mut mib_cp_time = [0; 2];
        let mut mib_cp_times = [0; 2];

        unsafe {
            let nb_cpus = super::utils::get_nb_cpus();
            init_mib(b"kern.cp_time\0", &mut mib_cp_time);
            init_mib(b"kern.cp_times\0", &mut mib_cp_times);
            Self {
                global_cpu_usage: 0.,
                cpus: Vec::with_capacity(nb_cpus),
                got_cpu_frequency: false,
                mib_cp_time,
                mib_cp_times,
                cp_time: VecSwitcher::new(vec![0; libc::CPUSTATES as usize]),
                cp_times: VecSwitcher::new(vec![0; nb_cpus * libc::CPUSTATES as usize]),
                nb_cpus,
            }
        }
    }

    pub(crate) fn refresh(&mut self, refresh_kind: CpuRefreshKind) {
        if self.cpus.is_empty() {
            let mut frequency = 0;

            // We get the CPU vendor ID in here.
            let vendor_id =
                get_sys_value_str_by_name(b"hw.model\0").unwrap_or_else(|| "<unknown>".to_owned());

            for pos in 0..self.nb_cpus {
                if refresh_kind.frequency() {
                    unsafe {
                        frequency = get_frequency_for_cpu(pos);
                    }
                }
                self.cpus.push(Cpu {
                    inner: CpuInner::new(format!("cpu {pos}"), vendor_id.clone(), frequency),
                });
            }
            self.got_cpu_frequency = refresh_kind.frequency();
        } else if refresh_kind.frequency() && !self.got_cpu_frequency {
            for (pos, proc_) in self.cpus.iter_mut().enumerate() {
                unsafe {
                    proc_.inner.frequency = get_frequency_for_cpu(pos);
                }
            }
            self.got_cpu_frequency = true;
        }
        if refresh_kind.cpu_usage() {
            self.get_cpu_usage();
        }
    }

    fn get_cpu_usage(&mut self) {
        unsafe {
            get_sys_value_array(&self.mib_cp_time, self.cp_time.get_mut());
            get_sys_value_array(&self.mib_cp_times, self.cp_times.get_mut());
        }

        fn compute_cpu_usage(new_cp_time: &[c_ulong], old_cp_time: &[c_ulong]) -> f32 {
            let mut total_new: u64 = 0;
            let mut total_old: u64 = 0;
            let mut cp_diff: c_ulong = 0;

            for i in 0..(libc::CPUSTATES as usize) {
                // We obviously don't want to get the idle part of the CPU usage, otherwise
                // we would always be at 100%...
                if i != libc::CP_IDLE as usize {
                    cp_diff = cp_diff.saturating_add(new_cp_time[i].saturating_sub(old_cp_time[i]));
                }
                total_new = total_new.saturating_add(new_cp_time[i] as _);
                total_old = total_old.saturating_add(old_cp_time[i] as _);
            }

            let total_diff = total_new.saturating_sub(total_old);
            if total_diff < 1 {
                0.
            } else {
                cp_diff as f32 / total_diff as f32 * 100.
            }
        }

        self.global_cpu_usage = compute_cpu_usage(self.cp_time.get_new(), self.cp_time.get_old());
        let old_cp_times = self.cp_times.get_old();
        let new_cp_times = self.cp_times.get_new();
        for (pos, cpu) in self.cpus.iter_mut().enumerate() {
            let index = pos * libc::CPUSTATES as usize;

            cpu.inner.cpu_usage = compute_cpu_usage(&new_cp_times[index..], &old_cp_times[index..]);
        }
    }
}

pub(crate) struct CpuInner {
    pub(crate) cpu_usage: f32,
    name: String,
    pub(crate) vendor_id: String,
    pub(crate) frequency: u64,
}

impl CpuInner {
    pub(crate) fn new(name: String, vendor_id: String, frequency: u64) -> Self {
        Self {
            cpu_usage: 0.,
            name,
            vendor_id,
            frequency,
        }
    }

    pub(crate) fn cpu_usage(&self) -> f32 {
        self.cpu_usage
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn frequency(&self) -> u64 {
        self.frequency
    }

    pub(crate) fn vendor_id(&self) -> &str {
        &self.vendor_id
    }

    pub(crate) fn brand(&self) -> &str {
        ""
    }
}

pub(crate) fn physical_core_count() -> Option<usize> {
    let mut physical_core_count: u32 = 0;

    unsafe {
        if get_sys_value_by_name(b"hw.ncpu\0", &mut physical_core_count) {
            Some(physical_core_count as _)
        } else {
            None
        }
    }
}

unsafe fn get_frequency_for_cpu(cpu_nb: usize) -> u64 {
    let mut frequency: c_int = 0;

    // The information can be missing if it's running inside a VM.
    if !get_sys_value_by_name(
        format!("dev.cpu.{cpu_nb}.freq\0").as_bytes(),
        &mut frequency,
    ) {
        frequency = 0;
    }
    frequency as _
}

/// This struct is used to switch between the "old" and "new" every time you use "get_mut".
#[derive(Debug)]
pub(crate) struct VecSwitcher<T> {
    v1: Vec<T>,
    v2: Vec<T>,
    first: bool,
}

impl<T: Clone> VecSwitcher<T> {
    pub fn new(v1: Vec<T>) -> Self {
        let v2 = v1.clone();

        Self {
            v1,
            v2,
            first: true,
        }
    }

    pub fn get_mut(&mut self) -> &mut [T] {
        self.first = !self.first;
        if self.first {
            // It means that `v2` will be the "new".
            &mut self.v2
        } else {
            // It means that `v1` will be the "new".
            &mut self.v1
        }
    }

    pub fn get_old(&self) -> &[T] {
        if self.first {
            &self.v1
        } else {
            &self.v2
        }
    }

    pub fn get_new(&self) -> &[T] {
        if self.first {
            &self.v2
        } else {
            &self.v1
        }
    }
}
