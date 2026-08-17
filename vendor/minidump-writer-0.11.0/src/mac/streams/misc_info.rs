use {
    super::*,
    format::{MiscInfoFlags, MINIDUMP_MISC_INFO_2 as MDRawMiscInfo},
    std::time::Duration,
};

/// From <usr/include/mach/time_value.h>
#[repr(C)]
#[derive(Copy, Clone)]
struct TimeValue {
    seconds: i32,
    microseconds: i32,
}

impl From<TimeValue> for Duration {
    fn from(tv: TimeValue) -> Self {
        let mut seconds = tv.seconds as u64;
        let mut microseconds = tv.microseconds as u32;
        // This _probably_ will never happen, but this will avoid a panic in
        // Duration::new() if it does
        if tv.microseconds >= 1000000 {
            seconds += 1;
            microseconds -= 1000000;
        }

        Duration::new(seconds, microseconds * 1000)
    }
}

/// From <usr/include/mach/task_info.h>, this includes basic information about
/// a task.
#[repr(C, packed(4))]
struct MachTaskBasicInfo {
    /// Virtual memory size in bytes
    virtual_size: u64,
    /// Resident memory size in bytes
    resident_size: u64,
    /// Maximum resident memory size in bytes
    resident_size_max: u64,
    /// Total user run time for terminated threads
    user_time: TimeValue,
    /// Total system run time for terminated threads
    system_time: TimeValue,
    /// Default policy for new threads
    policy: i32,
    /// Suspend count for task
    suspend_count: i32,
}

impl mach::TaskInfo for MachTaskBasicInfo {
    const FLAVOR: u32 = mach::task_info::MACH_TASK_BASIC_INFO;
}

/// From <usr/include/mach/task_info.h>, this includes times for currently
/// live threads in the task.
#[repr(C, packed(4))]
struct TaskThreadsTimeInfo {
    /// Total user run time for live threads
    user_time: TimeValue,
    /// total system run time for live threads
    system_time: TimeValue,
}

impl mach::TaskInfo for TaskThreadsTimeInfo {
    const FLAVOR: u32 = mach::task_info::TASK_THREAD_TIMES_INFO;
}

impl MinidumpWriter {
    /// Writes the [`MDStreamType::MiscInfoStream`] stream.
    ///
    /// On MacOS, we write a [`minidump_common::format::MINIDUMP_MISC_INFO_2`]
    /// to this stream, which includes the start time of the process at second
    /// granularity, and the (approximate) amount of time spent in user and
    /// system (kernel) time for the lifetime of the task. We attempt to also
    /// retrieve power ie CPU usage statistics, though this information is only
    /// currently available on x86_64, not aarch64 at the moment.
    pub(crate) fn write_misc_info(
        &mut self,
        buffer: &mut DumpBuf,
        dumper: &TaskDumper,
    ) -> Result<MDRawDirectory, WriterError> {
        let mut info_section = MemoryWriter::<MDRawMiscInfo>::alloc(buffer)?;
        let dirent = MDRawDirectory {
            stream_type: MDStreamType::MiscInfoStream as u32,
            location: info_section.location(),
        };

        let pid = dumper.pid_for_task()?;

        let mut misc_info = MDRawMiscInfo {
            size_of_info: std::mem::size_of::<MDRawMiscInfo>() as u32,
            flags1: MiscInfoFlags::MINIDUMP_MISC1_PROCESS_ID.bits()
                | MiscInfoFlags::MINIDUMP_MISC1_PROCESS_TIMES.bits()
                | MiscInfoFlags::MINIDUMP_MISC1_PROCESSOR_POWER_INFO.bits(),
            process_id: pid as u32,
            process_create_time: 0,
            process_user_time: 0,
            process_kernel_time: 0,
            processor_max_mhz: 0,
            processor_current_mhz: 0,
            processor_mhz_limit: 0,
            processor_max_idle_state: 0,
            processor_current_idle_state: 0,
        };

        // Note that both Breakpad and Crashpad use `sysctl CTL_KERN, KERN_PROC, KERN_PROC_PID`
        // to retrieve the process start time, but none of the structures that
        // are filled in by that call are in libc at the moment, and `proc_pidinfo`
        // seems to work just fine, so using that instead.
        //
        // SAFETY: syscall
        misc_info.process_create_time = unsafe {
            // Breakpad was using an old method to retrieve this, let's try the
            // BSD method instead which is already implemented in libc
            let mut proc_info = std::mem::MaybeUninit::<libc::proc_bsdinfo>::uninit();
            let size = std::mem::size_of::<libc::proc_bsdinfo>() as i32;
            if libc::proc_pidinfo(
                pid,
                libc::PROC_PIDTBSDINFO,
                0,
                proc_info.as_mut_ptr().cast(),
                size,
            ) == size
            {
                let proc_info = proc_info.assume_init();

                proc_info.pbi_start_tvsec as u32
            } else {
                0
            }
        };

        // Note that Breakpad is using `getrusage` to retrieve this information,
        // however that is wrong, as it can only retrieve the process usage information
        // for the current or children processes, not an external process, so
        // we use the Crashpad method, which is itself based off of the XNU
        // method of retrieving the process times
        // https://github.com/apple/darwin-xnu/blob/2ff845c2e033bd0ff64b5b6aa6063a1f8f65aa32/bsd/kern/kern_resource.c#L1215

        // The basic task info keeps the timings for all of the terminated threads
        let basic_info = dumper.task_info::<MachTaskBasicInfo>().ok();

        // THe thread times info keeps the timings for all of the living threads
        let thread_times_info = dumper.task_info::<TaskThreadsTimeInfo>().ok();

        let user_time = basic_info
            .as_ref()
            .map(|bi| Duration::from(bi.user_time))
            .unwrap_or_default()
            + thread_times_info
                .as_ref()
                .map(|tt| Duration::from(tt.user_time))
                .unwrap_or_default();
        let system_time = basic_info
            .as_ref()
            .map(|bi| Duration::from(bi.system_time))
            .unwrap_or_default()
            + thread_times_info
                .as_ref()
                .map(|tt| Duration::from(tt.system_time))
                .unwrap_or_default();

        misc_info.process_user_time = user_time.as_secs() as u32;
        misc_info.process_kernel_time = system_time.as_secs() as u32;

        // Note that neither of these two keys are present on aarch64, at least atm
        let max: u64 = mach::sysctl_by_name(b"hw.cpufrequency_max\0");
        let freq: u64 = mach::sysctl_by_name(b"hw.cpufrequency\0");

        let max = (max / 1000 * 1000) as u32;
        let current = (freq / 1000 * 1000) as u32;

        misc_info.processor_max_mhz = max;
        misc_info.processor_mhz_limit = max;
        misc_info.processor_current_mhz = current;

        info_section.set_value(buffer, misc_info)?;

        Ok(dirent)
    }
}
