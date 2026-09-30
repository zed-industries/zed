//! Reads another thread's cumulative resource usage.
//!
//! Every read here is safe to make from a different thread than the one
//! measured, which is what lets the hang watchdog measure the foreground
//! without the foreground doing any work for it.

use std::time::Duration;

/// Cumulative resource usage of one thread, or of its process where noted.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct ThreadUsage {
    /// CPU time the thread spent running, in user and kernel mode.
    pub(crate) cpu: Duration,
    /// The part of `cpu` spent in the kernel, where available.
    pub(crate) system_cpu: Option<Duration>,
    /// Page faults that had to wait for storage: the thread's own on Linux,
    /// the whole process's pageins on macOS, and unavailable on Windows,
    /// which doesn't separate them from faults served from memory.
    pub(crate) major_faults: Option<u64>,
    /// Time the thread was ready to run but waited for a CPU. Linux only.
    pub(crate) run_delay: Option<Duration>,
}

/// Measures one thread's [`ThreadUsage`] from any thread.
pub(crate) struct ThreadUsageReader {
    platform: platform::Reader,
}

impl ThreadUsageReader {
    /// A reader for the calling thread, or `None` where the platform can't
    /// measure a thread from another one.
    pub(crate) fn for_current_thread() -> Option<Self> {
        Some(Self {
            platform: platform::Reader::for_current_thread()?,
        })
    }

    /// The thread's usage so far, or `None` if it can't be read, e.g. because
    /// the thread has exited.
    pub(crate) fn read(&mut self) -> Option<ThreadUsage> {
        self.platform.read()
    }
}

#[cfg(target_os = "linux")]
mod platform {
    use std::fs::File;
    use std::os::unix::fs::FileExt;
    use std::time::Duration;

    use super::ThreadUsage;

    pub(super) struct Reader {
        cpu_clock: libc::clockid_t,
        /// `/proc/self/task/<tid>/stat`, for major faults and system time.
        stat: Option<File>,
        /// `/proc/self/task/<tid>/schedstat`, for run-queue delay. Absent on
        /// kernels built without scheduler statistics.
        schedstat: Option<File>,
        clock_ticks_per_second: u64,
        buffer: Vec<u8>,
    }

    impl Reader {
        pub(super) fn for_current_thread() -> Option<Self> {
            let mut cpu_clock: libc::clockid_t = 0;
            // SAFETY: `pthread_self` is always a valid thread, and
            // `cpu_clock` is a valid, writable clockid_t.
            if unsafe { libc::pthread_getcpuclockid(libc::pthread_self(), &mut cpu_clock) } != 0 {
                return None;
            }
            // `gettid` through `syscall` because glibc only wraps it since 2.30.
            // SAFETY: SYS_gettid takes no arguments and can't fail.
            let thread_id = unsafe { libc::syscall(libc::SYS_gettid) };
            let task = format!("/proc/self/task/{thread_id}");
            // SAFETY: no preconditions.
            let clock_ticks_per_second = unsafe { libc::sysconf(libc::_SC_CLK_TCK) };
            Some(Self {
                cpu_clock,
                stat: File::open(format!("{task}/stat")).ok(),
                schedstat: File::open(format!("{task}/schedstat")).ok(),
                clock_ticks_per_second: u64::try_from(clock_ticks_per_second)
                    .ok()
                    .filter(|ticks| *ticks > 0)
                    .unwrap_or(100),
                buffer: Vec::with_capacity(512),
            })
        }

        pub(super) fn read(&mut self) -> Option<ThreadUsage> {
            let mut time = libc::timespec {
                tv_sec: 0,
                tv_nsec: 0,
            };
            // SAFETY: `time` is a valid, writable timespec.
            if unsafe { libc::clock_gettime(self.cpu_clock, &mut time) } != 0 {
                return None;
            }
            let cpu = Duration::new(
                u64::try_from(time.tv_sec).ok()?,
                u32::try_from(time.tv_nsec).ok()?,
            );
            let (major_faults, system_cpu) = match read_file(self.stat.as_ref(), &mut self.buffer) {
                Some(stat) => parse_stat(stat, self.clock_ticks_per_second),
                None => (None, None),
            };
            let run_delay =
                read_file(self.schedstat.as_ref(), &mut self.buffer).and_then(parse_schedstat);
            Some(ThreadUsage {
                cpu,
                system_cpu,
                major_faults,
                run_delay,
            })
        }
    }

    fn read_file<'a>(file: Option<&File>, buffer: &'a mut Vec<u8>) -> Option<&'a str> {
        let file = file?;
        buffer.resize(buffer.capacity().max(512), 0);
        let length = file.read_at(buffer, 0).ok()?;
        std::str::from_utf8(buffer.get(..length)?).ok()
    }

    /// Major faults and system time from `/proc/<pid>/task/<tid>/stat`; see
    /// proc_pid_stat(5). The command name can contain spaces and
    /// parentheses, so fields are counted from its closing parenthesis.
    pub(super) fn parse_stat(
        stat: &str,
        clock_ticks_per_second: u64,
    ) -> (Option<u64>, Option<Duration>) {
        let Some((_, fields)) = stat.rsplit_once(')') else {
            return (None, None);
        };
        // The first field after the name is field 3, `state`.
        let fields: Vec<&str> = fields.split_whitespace().collect();
        let field = |number: usize| fields.get(number - 3)?.parse::<u64>().ok();
        let major_faults = field(12);
        let system_cpu = field(15).map(|ticks| {
            Duration::from_nanos(ticks.saturating_mul(1_000_000_000) / clock_ticks_per_second)
        });
        (major_faults, system_cpu)
    }

    /// Run-queue delay from `schedstat`: "<cpu ns> <run delay ns> <slices>".
    pub(super) fn parse_schedstat(schedstat: &str) -> Option<Duration> {
        let run_delay = schedstat.split_whitespace().nth(1)?.parse().ok()?;
        Some(Duration::from_nanos(run_delay))
    }
}

#[cfg(target_os = "macos")]
mod platform {
    use std::time::Duration;

    use super::ThreadUsage;

    // Declared here because libc deprecates its mach bindings. Layouts and
    // constants are from `<mach/thread_info.h>`, `<mach/task_info.h>` and
    // `<mach/time_value.h>`:
    // https://github.com/apple-oss-distributions/xnu/tree/main/osfmk/mach
    #[repr(C)]
    #[derive(Default)]
    struct TimeValue {
        seconds: i32,
        microseconds: i32,
    }

    #[repr(C)]
    #[derive(Default)]
    struct ThreadBasicInfo {
        user_time: TimeValue,
        system_time: TimeValue,
        cpu_usage: i32,
        policy: i32,
        run_state: i32,
        flags: i32,
        suspend_count: i32,
        sleep_time: i32,
    }

    #[repr(C)]
    #[derive(Default)]
    struct TaskEventsInfo {
        faults: i32,
        pageins: i32,
        cow_faults: i32,
        messages_sent: i32,
        messages_received: i32,
        syscalls_mach: i32,
        syscalls_unix: i32,
        context_switches: i32,
    }

    const THREAD_BASIC_INFO: u32 = 3;
    const TASK_EVENTS_INFO: u32 = 2;
    const KERN_SUCCESS: i32 = 0;

    unsafe extern "C" {
        static mach_task_self_: u32;
        fn mach_thread_self() -> u32;
        fn mach_port_deallocate(task: u32, name: u32) -> i32;
        fn thread_info(thread: u32, flavor: u32, info: *mut i32, count: *mut u32) -> i32;
        fn task_info(task: u32, flavor: u32, info: *mut i32, count: *mut u32) -> i32;
    }

    /// Counts of `natural_t` in a mach info struct, as the `count` argument
    /// of `thread_info` and `task_info` expects.
    fn natural_count<T>() -> u32 {
        (size_of::<T>() / size_of::<i32>()) as u32
    }

    fn duration(time: &TimeValue) -> Option<Duration> {
        Some(
            Duration::from_secs(u64::try_from(time.seconds).ok()?)
                + Duration::from_micros(u64::try_from(time.microseconds).ok()?),
        )
    }

    pub(super) struct Reader {
        thread: u32,
    }

    // SAFETY: a mach port name is a plain integer that's valid from any
    // thread in the task.
    unsafe impl Send for Reader {}

    impl Reader {
        pub(super) fn for_current_thread() -> Option<Self> {
            // SAFETY: no preconditions. The returned send right is released
            // on drop.
            let thread = unsafe { mach_thread_self() };
            (thread != 0).then_some(Self { thread })
        }

        pub(super) fn read(&mut self) -> Option<ThreadUsage> {
            let mut info = ThreadBasicInfo::default();
            let mut count = natural_count::<ThreadBasicInfo>();
            // SAFETY: `info` is a writable THREAD_BASIC_INFO of `count`
            // naturals.
            let result = unsafe {
                thread_info(
                    self.thread,
                    THREAD_BASIC_INFO,
                    (&raw mut info).cast(),
                    &mut count,
                )
            };
            if result != KERN_SUCCESS {
                return None;
            }
            let user_cpu = duration(&info.user_time)?;
            let system_cpu = duration(&info.system_time)?;

            let mut events = TaskEventsInfo::default();
            let mut count = natural_count::<TaskEventsInfo>();
            // SAFETY: `events` is a writable TASK_EVENTS_INFO of `count`
            // naturals, and `mach_task_self_` is initialized before `main`.
            let result = unsafe {
                task_info(
                    mach_task_self_,
                    TASK_EVENTS_INFO,
                    (&raw mut events).cast(),
                    &mut count,
                )
            };
            let major_faults = (result == KERN_SUCCESS)
                .then(|| u64::try_from(events.pageins).ok())
                .flatten();

            Some(ThreadUsage {
                cpu: user_cpu + system_cpu,
                system_cpu: Some(system_cpu),
                major_faults,
                run_delay: None,
            })
        }
    }

    impl Drop for Reader {
        fn drop(&mut self) {
            // SAFETY: `self.thread` is a send right this reader owns.
            let result = unsafe { mach_port_deallocate(mach_task_self_, self.thread) };
            if result != KERN_SUCCESS {
                log::debug!("failed to release the hang watchdog's thread port: {result}");
            }
        }
    }
}

#[cfg(target_os = "windows")]
mod platform {
    use std::time::Duration;

    use windows::Win32::Foundation::{CloseHandle, FILETIME, HANDLE};
    use windows::Win32::System::Threading::{
        GetCurrentThreadId, GetThreadTimes, OpenThread, THREAD_QUERY_LIMITED_INFORMATION,
    };

    use super::ThreadUsage;

    pub(super) struct Reader {
        thread: HANDLE,
    }

    // SAFETY: a thread handle is valid from any thread in the process.
    unsafe impl Send for Reader {}

    fn duration(time: FILETIME) -> Duration {
        let hundreds_of_nanoseconds =
            (u64::from(time.dwHighDateTime) << 32) | u64::from(time.dwLowDateTime);
        Duration::from_nanos(hundreds_of_nanoseconds.saturating_mul(100))
    }

    impl Reader {
        pub(super) fn for_current_thread() -> Option<Self> {
            // A real handle rather than `GetCurrentThread`, whose pseudo
            // handle would refer to whichever thread uses it.
            // SAFETY: no preconditions; the handle is closed on drop.
            let thread = unsafe {
                OpenThread(
                    THREAD_QUERY_LIMITED_INFORMATION,
                    false,
                    GetCurrentThreadId(),
                )
            }
            .ok()?;
            Some(Self { thread })
        }

        pub(super) fn read(&mut self) -> Option<ThreadUsage> {
            let mut creation = FILETIME::default();
            let mut exit = FILETIME::default();
            let mut kernel = FILETIME::default();
            let mut user = FILETIME::default();
            // SAFETY: `self.thread` is an open thread handle, and all four
            // times are writable. These times advance at the system timer's
            // resolution, about 15.6 ms.
            unsafe {
                GetThreadTimes(
                    self.thread,
                    &mut creation,
                    &mut exit,
                    &mut kernel,
                    &mut user,
                )
            }
            .ok()?;
            let system_cpu = duration(kernel);
            Some(ThreadUsage {
                cpu: system_cpu + duration(user),
                system_cpu: Some(system_cpu),
                major_faults: None,
                run_delay: None,
            })
        }
    }

    impl Drop for Reader {
        fn drop(&mut self) {
            // SAFETY: `self.thread` is a handle this reader owns.
            if let Err(error) = unsafe { CloseHandle(self.thread) } {
                log::debug!("failed to close the hang watchdog's thread handle: {error}");
            }
        }
    }
}

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
mod platform {
    use super::ThreadUsage;

    pub(super) struct Reader;

    impl Reader {
        pub(super) fn for_current_thread() -> Option<Self> {
            None
        }

        pub(super) fn read(&mut self) -> Option<ThreadUsage> {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
    fn reads_cpu_time_of_another_thread() {
        let (sender, receiver) = std::sync::mpsc::channel();
        let (stop, stopped) = std::sync::mpsc::channel::<()>();
        let worker = std::thread::spawn(move || {
            sender.send(ThreadUsageReader::for_current_thread()).ok();
            let started = std::time::Instant::now();
            // Spin until told to stop, so the thread accumulates CPU time.
            while stopped.try_recv().is_err() && started.elapsed() < Duration::from_secs(10) {
                std::hint::spin_loop();
            }
        });
        let mut reader = receiver
            .recv()
            .expect("worker sends its reader")
            .expect("this platform supports reading thread usage");
        let before = reader.read().expect("read usage");
        std::thread::sleep(Duration::from_millis(200));
        let after = reader.read().expect("read usage");
        stop.send(()).ok();
        worker.join().expect("worker exits");

        // A spinning thread gets most of the elapsed time as CPU time; the
        // bound is loose for loaded CI machines and coarse Windows clocks.
        assert!(
            after.cpu.saturating_sub(before.cpu) >= Duration::from_millis(50),
            "{before:?} -> {after:?}"
        );
        assert_eq!(after.system_cpu.is_some(), before.system_cpu.is_some());
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn parses_proc_stat_and_schedstat() {
        let stat = "1234 (a (weird) name) S 1 2 3 4 5 6 7 8 42 9 10 250 0 0 0";
        let (major_faults, system_cpu) = platform::parse_stat(stat, 100);
        assert_eq!(major_faults, Some(42));
        assert_eq!(system_cpu, Some(Duration::from_millis(2500)));
        assert_eq!(
            platform::parse_schedstat("1000 2000000 30\n"),
            Some(Duration::from_millis(2))
        );
    }
}
