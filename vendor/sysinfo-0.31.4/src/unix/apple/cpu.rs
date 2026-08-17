// Take a look at the license at the top of the repository in the LICENSE file.

use crate::sys::utils::{get_sys_value, get_sys_value_by_name};
use crate::{Cpu, CpuRefreshKind};

use libc::{c_char, c_void, host_processor_info, mach_port_t, mach_task_self};
use std::mem;
use std::ops::Deref;
use std::sync::Arc;
use std::time::Instant;

pub(crate) struct CpusWrapper {
    pub(crate) global_cpu: CpuUsage,
    pub(crate) cpus: Vec<Cpu>,
    pub(crate) got_cpu_frequency: bool,
    /// This field is needed to prevent updating when not enough time passed since last update.
    last_update: Option<Instant>,
}

impl CpusWrapper {
    pub(crate) fn new() -> Self {
        Self {
            global_cpu: CpuUsage::new(),
            cpus: Vec::new(),
            got_cpu_frequency: false,
            last_update: None,
        }
    }

    pub(crate) fn refresh(&mut self, refresh_kind: CpuRefreshKind, port: mach_port_t) {
        let need_cpu_usage_update = self
            .last_update
            .is_some_and(|last_update| last_update.elapsed() > crate::MINIMUM_CPU_UPDATE_INTERVAL);

        let cpus = &mut self.cpus;
        if cpus.is_empty() {
            init_cpus(port, cpus, &mut self.global_cpu, refresh_kind);
            self.last_update = Some(Instant::now());
            self.got_cpu_frequency = refresh_kind.frequency();
            return;
        }
        if refresh_kind.frequency() && !self.got_cpu_frequency {
            let frequency = unsafe { get_cpu_frequency() };
            for proc_ in cpus.iter_mut() {
                proc_.inner.set_frequency(frequency);
            }
            self.got_cpu_frequency = true;
        }
        if refresh_kind.cpu_usage() && need_cpu_usage_update {
            self.last_update = Some(Instant::now());
            update_cpu_usage(port, &mut self.global_cpu, |proc_data, cpu_info| {
                let mut percentage = 0f32;
                let mut offset = 0;
                for proc_ in cpus.iter_mut() {
                    let cpu_usage = compute_usage_of_cpu(proc_, cpu_info, offset);
                    proc_.inner.update(cpu_usage, Arc::clone(&proc_data));
                    percentage += proc_.inner.cpu_usage();

                    offset += libc::CPU_STATE_MAX as isize;
                }
                (percentage, cpus.len())
            });
        }
    }
}

pub(crate) struct UnsafePtr<T>(*mut T);

unsafe impl<T> Send for UnsafePtr<T> {}
unsafe impl<T> Sync for UnsafePtr<T> {}

impl<T> Deref for UnsafePtr<T> {
    type Target = *mut T;

    fn deref(&self) -> &*mut T {
        &self.0
    }
}

pub(crate) struct CpuData {
    pub cpu_info: UnsafePtr<i32>,
    pub num_cpu_info: u32,
}

impl CpuData {
    pub fn new(cpu_info: *mut i32, num_cpu_info: u32) -> CpuData {
        CpuData {
            cpu_info: UnsafePtr(cpu_info),
            num_cpu_info,
        }
    }
}

impl Drop for CpuData {
    fn drop(&mut self) {
        if !self.cpu_info.0.is_null() {
            let prev_cpu_info_size = std::mem::size_of::<i32>() as u32 * self.num_cpu_info;
            unsafe {
                libc::vm_deallocate(
                    mach_task_self(),
                    self.cpu_info.0 as _,
                    prev_cpu_info_size as _,
                );
            }
            self.cpu_info.0 = std::ptr::null_mut();
        }
    }
}

pub(crate) struct CpuUsage {
    percent: f32,
    data: Arc<CpuData>,
    // Cannot be frequency for each CPU apparently so we store it in the CPU usage...
    frequency: u64,
}

impl CpuUsage {
    pub(crate) fn new() -> Self {
        Self {
            percent: 0.,
            data: Arc::new(CpuData::new(std::ptr::null_mut(), 0)),
            frequency: 0,
        }
    }

    pub(crate) fn percent(&self) -> f32 {
        self.percent
    }

    pub(crate) fn set_cpu_usage(&mut self, value: f32) {
        self.percent = value;
    }
}

pub(crate) struct CpuInner {
    name: String,
    vendor_id: String,
    brand: String,
    usage: CpuUsage,
}

impl CpuInner {
    pub(crate) fn new(
        name: String,
        cpu_data: Arc<CpuData>,
        frequency: u64,
        vendor_id: String,
        brand: String,
    ) -> Self {
        Self {
            name,
            usage: CpuUsage {
                percent: 0.,
                data: cpu_data,
                frequency,
            },
            vendor_id,
            brand,
        }
    }

    pub(crate) fn set_cpu_usage(&mut self, cpu_usage: f32) {
        self.usage.set_cpu_usage(cpu_usage);
    }

    pub(crate) fn update(&mut self, cpu_usage: f32, cpu_data: Arc<CpuData>) {
        self.usage.percent = cpu_usage;
        self.usage.data = cpu_data;
    }

    pub(crate) fn data(&self) -> Arc<CpuData> {
        Arc::clone(&self.usage.data)
    }

    pub(crate) fn set_frequency(&mut self, frequency: u64) {
        self.usage.frequency = frequency;
    }

    pub(crate) fn cpu_usage(&self) -> f32 {
        self.usage.percent()
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn frequency(&self) -> u64 {
        self.usage.frequency
    }

    pub(crate) fn vendor_id(&self) -> &str {
        &self.vendor_id
    }

    pub(crate) fn brand(&self) -> &str {
        &self.brand
    }
}

pub(crate) unsafe fn get_cpu_frequency() -> u64 {
    let mut speed: u64 = 0;
    let mut len = std::mem::size_of::<u64>();
    if libc::sysctlbyname(
        b"hw.cpufrequency\0".as_ptr() as *const _,
        &mut speed as *mut _ as _,
        &mut len,
        std::ptr::null_mut(),
        0,
    ) == 0
    {
        return speed / 1_000_000;
    }

    #[cfg(any(target_os = "ios", feature = "apple-sandbox"))]
    {
        0
    }
    #[cfg(not(any(target_os = "ios", feature = "apple-sandbox")))]
    {
        crate::sys::inner::cpu::get_cpu_frequency()
    }
}

pub(crate) fn physical_core_count() -> Option<usize> {
    let mut physical_core_count = 0;

    unsafe {
        if get_sys_value_by_name(
            b"hw.physicalcpu\0",
            &mut mem::size_of::<u32>(),
            &mut physical_core_count as *mut usize as *mut c_void,
        ) {
            Some(physical_core_count)
        } else {
            None
        }
    }
}

#[inline]
fn get_in_use(cpu_info: *mut i32, offset: isize) -> i64 {
    unsafe {
        let user = *cpu_info.offset(offset + libc::CPU_STATE_USER as isize) as i64;
        let system = *cpu_info.offset(offset + libc::CPU_STATE_SYSTEM as isize) as i64;
        let nice = *cpu_info.offset(offset + libc::CPU_STATE_NICE as isize) as i64;

        user.saturating_add(system).saturating_add(nice)
    }
}

#[inline]
fn get_idle(cpu_info: *mut i32, offset: isize) -> i32 {
    unsafe { *cpu_info.offset(offset + libc::CPU_STATE_IDLE as isize) }
}

pub(crate) fn compute_usage_of_cpu(proc_: &Cpu, cpu_info: *mut i32, offset: isize) -> f32 {
    let old_cpu_info = proc_.inner.data().cpu_info.0;
    let in_use;
    let idle;

    // In case we are initializing cpus, there is no "old value" yet.
    if old_cpu_info == cpu_info {
        in_use = get_in_use(cpu_info, offset);
        idle = get_idle(cpu_info, offset);
    } else {
        let new_in_use = get_in_use(cpu_info, offset);
        let old_in_use = get_in_use(old_cpu_info, offset);

        let new_idle = get_idle(cpu_info, offset);
        let old_idle = get_idle(old_cpu_info, offset);

        in_use = new_in_use.saturating_sub(old_in_use);
        idle = new_idle.saturating_sub(old_idle) as _;
    }
    let total = in_use.saturating_add(idle as _);
    let usage = (in_use as f32 / total as f32) * 100.;
    if usage.is_nan() {
        // If divided by zero, avoid returning a NaN
        0.
    } else {
        usage
    }
}

pub(crate) fn update_cpu_usage<F: FnOnce(Arc<CpuData>, *mut i32) -> (f32, usize)>(
    port: libc::mach_port_t,
    global_cpu: &mut CpuUsage,
    f: F,
) {
    let mut num_cpu_u = 0u32;
    let mut cpu_info: *mut i32 = std::ptr::null_mut();
    let mut num_cpu_info = 0u32;

    let mut total_cpu_usage = 0f32;

    unsafe {
        if host_processor_info(
            port,
            libc::PROCESSOR_CPU_LOAD_INFO,
            &mut num_cpu_u as *mut u32,
            &mut cpu_info as *mut *mut i32,
            &mut num_cpu_info as *mut u32,
        ) == libc::KERN_SUCCESS
        {
            let (total_percentage, len) =
                f(Arc::new(CpuData::new(cpu_info, num_cpu_info)), cpu_info);
            total_cpu_usage = total_percentage / len as f32;
        }
        global_cpu.set_cpu_usage(total_cpu_usage);
    }
}

pub(crate) fn init_cpus(
    port: libc::mach_port_t,
    cpus: &mut Vec<Cpu>,
    global_cpu: &mut CpuUsage,
    refresh_kind: CpuRefreshKind,
) {
    let mut num_cpu = 0;
    let mut mib = [libc::CTL_HW as _, libc::HW_NCPU as _];

    let (vendor_id, brand) = get_vendor_id_and_brand();
    let frequency = if refresh_kind.frequency() {
        unsafe { get_cpu_frequency() }
    } else {
        global_cpu.frequency
    };

    unsafe {
        if !get_sys_value(
            mem::size_of::<u32>(),
            &mut num_cpu as *mut _ as *mut _,
            &mut mib,
        ) {
            num_cpu = 1;
        }
    }
    update_cpu_usage(port, global_cpu, |proc_data, cpu_info| {
        let mut percentage = 0f32;
        let mut offset = 0;
        for i in 0..num_cpu {
            let mut cpu = Cpu {
                inner: CpuInner::new(
                    format!("{}", i + 1),
                    Arc::clone(&proc_data),
                    frequency,
                    vendor_id.clone(),
                    brand.clone(),
                ),
            };
            if refresh_kind.cpu_usage() {
                let cpu_usage = compute_usage_of_cpu(&cpu, cpu_info, offset);
                cpu.inner.set_cpu_usage(cpu_usage);
                percentage += cpu.cpu_usage();
            }
            cpus.push(cpu);

            offset += libc::CPU_STATE_MAX as isize;
        }
        (percentage, cpus.len())
    });
}

fn get_sysctl_str(s: &[u8]) -> String {
    let mut len = 0;

    unsafe {
        libc::sysctlbyname(
            s.as_ptr() as *const c_char,
            std::ptr::null_mut(),
            &mut len,
            std::ptr::null_mut(),
            0,
        );
        if len < 1 {
            return String::new();
        }

        let mut buf = Vec::with_capacity(len);
        libc::sysctlbyname(
            s.as_ptr() as *const c_char,
            buf.as_mut_ptr() as _,
            &mut len,
            std::ptr::null_mut(),
            0,
        );
        if len > 0 {
            buf.set_len(len);
            while buf.last() == Some(&b'\0') {
                buf.pop();
            }
            String::from_utf8(buf).unwrap_or_else(|_| String::new())
        } else {
            String::new()
        }
    }
}

pub(crate) fn get_vendor_id_and_brand() -> (String, String) {
    // On apple M1, `sysctl machdep.cpu.vendor` returns "", so fallback to "Apple" if the result
    // is empty.
    let mut vendor = get_sysctl_str(b"machdep.cpu.vendor\0");
    if vendor.is_empty() {
        vendor = "Apple".to_string();
    }

    (vendor, get_sysctl_str(b"machdep.cpu.brand_string\0"))
}

#[cfg(test)]
mod test {
    use crate::*;
    use std::process::Command;

    #[test]
    fn check_vendor_and_brand() {
        let child = Command::new("sysctl")
            .arg("-a")
            .output()
            .expect("Failed to start command...");

        assert!(child.status.success());
        let stdout = String::from_utf8(child.stdout).expect("Not valid UTF8");

        let sys = System::new_with_specifics(
            crate::RefreshKind::new().with_cpu(CpuRefreshKind::new().with_cpu_usage()),
        );
        let cpus = sys.cpus();
        assert!(!cpus.is_empty(), "no CPU found");
        if let Some(line) = stdout.lines().find(|l| l.contains("machdep.cpu.vendor")) {
            let sysctl_value = line.split(':').nth(1).unwrap();
            assert_eq!(cpus[0].vendor_id(), sysctl_value.trim());
        }
        if let Some(line) = stdout
            .lines()
            .find(|l| l.contains("machdep.cpu.brand_string"))
        {
            let sysctl_value = line.split(':').nth(1).unwrap();
            assert_eq!(cpus[0].brand(), sysctl_value.trim());
        }
    }
}
