use super::ProcState;
use super::StatFlags;
use crate::{from_iter, from_iter_optional, ProcResult};
#[cfg(feature = "serde1")]
use serde::{Deserialize, Serialize};

use std::io::Read;
use std::str::FromStr;

/// Status information about the process, based on the `/proc/<pid>/stat` file.
///
/// Not all fields are available in every kernel.  These fields have `Option<T>` types.
///
/// New fields to this struct may be added at any time (even without a major or minor semver bump).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct Stat {
    /// The process ID.
    pub pid: i32,
    /// The filename of the executable, without the parentheses.
    ///
    /// This is visible whether or not the executable is swapped out.
    ///
    /// Note that if the actual comm field contains invalid UTF-8 characters, they will be replaced
    /// here by the U+FFFD replacement character.
    pub comm: String,
    /// Process State.
    ///
    /// See [state()](#method.state) to get the process state as an enum.
    pub state: char,
    /// The PID of the parent of this process.
    pub ppid: i32,
    /// The process group ID of the process.
    pub pgrp: i32,
    /// The session ID of the process.
    pub session: i32,
    /// The controlling terminal of the process.
    ///
    /// The minor device number is contained in the combination of bits 31 to 20 and  7  to  0;
    /// the major device number is in bits 15 to 8.
    ///
    /// See [tty_nr()](#method.tty_nr) to get this value decoded into a (major, minor) tuple
    pub tty_nr: i32,
    /// The ID of the foreground process group of the controlling terminal of the process.
    pub tpgid: i32,
    /// The kernel flags  word of the process.
    ///
    /// For bit meanings, see the PF_* defines in  the  Linux  kernel  source  file
    /// [`include/linux/sched.h`](https://github.com/torvalds/linux/blob/master/include/linux/sched.h).
    ///
    /// See [flags()](#method.flags) to get a [`StatFlags`](struct.StatFlags.html) bitfield object.
    pub flags: u32,
    /// The number of minor faults the process has made which have not required loading a memory
    /// page from disk.
    pub minflt: u64,
    /// The number of minor faults that the process's waited-for children have made.
    pub cminflt: u64,
    /// The number of major faults the process has made which have required loading a memory page
    /// from disk.
    pub majflt: u64,
    /// The number of major faults that the process's waited-for children have made.
    pub cmajflt: u64,
    /// Amount of time that this process has been scheduled in user mode, measured in clock ticks
    /// (divide by `ticks_per_second()`).
    ///
    /// This includes guest time, guest_time (time spent running a virtual CPU, see below), so that
    /// applications that are not aware of the guest time field  do not lose that time from their
    /// calculations.
    pub utime: u64,
    /// Amount of time that this process has been scheduled in kernel mode, measured in clock ticks
    /// (divide by `ticks_per_second()`).
    pub stime: u64,

    /// Amount  of  time  that  this  process's  waited-for  children  have  been  scheduled  in
    /// user  mode,  measured  in clock ticks (divide by `ticks_per_second()`).
    ///
    /// This includes guest time, cguest_time (time spent running a virtual CPU, see below).
    pub cutime: i64,

    /// Amount of time that this process's waited-for  children  have  been  scheduled  in  kernel
    /// mode,  measured  in  clock  ticks  (divide  by `ticks_per_second()`).
    pub cstime: i64,
    /// For processes running a real-time scheduling policy (policy below; see sched_setscheduler(2)),
    /// this is the negated scheduling priority, minus one;
    ///
    /// That is, a number in the range -2 to -100,
    /// corresponding to real-time priority 1 to 99.  For processes running under a non-real-time
    /// scheduling policy, this is the raw nice value (setpriority(2)) as represented in the kernel.
    /// The kernel stores nice values as numbers in the range 0 (high) to 39  (low),  corresponding
    /// to the user-visible nice range of -20 to 19.
    /// (This explanation is for Linux 2.6)
    ///
    /// Before Linux 2.6, this was a scaled value based on the scheduler weighting given to this process.
    pub priority: i64,
    /// The nice value (see `setpriority(2)`), a value in the range 19 (low priority) to -20 (high priority).
    pub nice: i64,
    /// Number  of  threads in this process (since Linux 2.6).  Before kernel 2.6, this field was
    /// hard coded to 0 as a placeholder for an earlier removed field.
    pub num_threads: i64,
    /// The time in jiffies before the next SIGALRM is sent to the process due to an interval
    /// timer.
    ///
    /// Since kernel 2.6.17, this  field is no longer maintained, and is hard coded as 0.
    pub itrealvalue: i64,
    /// The time the process started after system boot.
    ///
    /// In kernels before Linux 2.6, this value was expressed in  jiffies.  Since  Linux 2.6, the
    /// value is expressed in clock ticks (divide by `sysconf(_SC_CLK_TCK)`).
    ///
    #[cfg_attr(
        feature = "chrono",
        doc = "See also the [Stat::starttime()] method to get the starttime as a `DateTime` object"
    )]
    #[cfg_attr(
        not(feature = "chrono"),
        doc = "If you compile with the optional `chrono` feature, you can use the `starttime()` method to get the starttime as a `DateTime` object"
    )]
    pub starttime: u64,
    /// Virtual memory size in bytes.
    pub vsize: u64,
    /// Resident Set Size: number of pages the process has in real memory.
    ///
    /// This is just the pages which count toward text,  data,  or stack space.
    /// This does not include pages which have not been demand-loaded in, or which are swapped out.
    pub rss: u64,
    /// Current soft limit in bytes on the rss of the process; see the description of RLIMIT_RSS in
    /// getrlimit(2).
    pub rsslim: u64,
    /// The address above which program text can run.
    pub startcode: u64,
    /// The address below which program text can run.
    pub endcode: u64,
    /// The address of the start (i.e., bottom) of the stack.
    pub startstack: u64,
    /// The current value of ESP (stack pointer), as found in the kernel stack page for the
    /// process.
    pub kstkesp: u64,
    /// The current EIP (instruction pointer).
    pub kstkeip: u64,
    /// The  bitmap of pending signals, displayed as a decimal number.  Obsolete, because it does
    /// not provide information on real-time signals; use `/proc/<pid>/status` instead.
    pub signal: u64,
    /// The bitmap of blocked signals, displayed as a decimal number.  Obsolete, because it does
    /// not provide information on  real-time signals; use `/proc/<pid>/status` instead.
    pub blocked: u64,
    /// The  bitmap of ignored signals, displayed as a decimal number.  Obsolete, because it does
    /// not provide information on real-time signals; use `/proc/<pid>/status` instead.
    pub sigignore: u64,
    /// The bitmap of caught signals, displayed as a decimal number.  Obsolete, because it does not
    /// provide information  on  real-time signals; use `/proc/<pid>/status` instead.
    pub sigcatch: u64,
    /// This  is  the  "channel"  in which the process is waiting.  It is the address of a location
    /// in the kernel where the process is sleeping.  The corresponding symbolic name can be found in
    /// `/proc/<pid>/wchan`.
    pub wchan: u64,
    /// Number of pages swapped **(not maintained)**.
    pub nswap: u64,
    /// Cumulative nswap for child processes **(not maintained)**.
    pub cnswap: u64,
    /// Signal to be sent to parent when we die.
    ///
    /// (since Linux 2.1.22)
    pub exit_signal: Option<i32>,
    /// CPU number last executed on.
    ///
    /// (since Linux 2.2.8)
    pub processor: Option<i32>,
    /// Real-time scheduling priority
    ///
    ///  Real-time scheduling priority, a number in the range 1 to 99 for processes scheduled under a real-time policy, or 0, for non-real-time processes
    ///
    /// (since Linux 2.5.19)
    pub rt_priority: Option<u32>,
    /// Scheduling policy (see sched_setscheduler(2)).
    ///
    /// Decode using the `SCHED_*` constants in `linux/sched.h`.
    ///
    /// (since Linux 2.5.19)
    pub policy: Option<u32>,
    /// Aggregated block I/O delays, measured in clock ticks (centiseconds).
    ///
    /// (since Linux 2.6.18)
    pub delayacct_blkio_ticks: Option<u64>,
    /// Guest time of the process (time spent running a virtual CPU for a guest operating system),
    /// measured in clock ticks (divide by `ticks_per_second()`)
    ///
    /// (since Linux 2.6.24)
    pub guest_time: Option<u64>,
    /// Guest time of the process's children, measured in clock ticks (divide by
    /// `ticks_per_second()`).
    ///
    /// (since Linux 2.6.24)
    pub cguest_time: Option<i64>,
    /// Address above which program initialized and uninitialized (BSS) data are placed.
    ///
    /// (since Linux 3.3)
    pub start_data: Option<u64>,
    /// Address below which program initialized and uninitialized (BSS) data are placed.
    ///
    /// (since Linux 3.3)
    pub end_data: Option<u64>,
    /// Address above which program heap can be expanded with brk(2).
    ///
    /// (since Linux 3.3)
    pub start_brk: Option<u64>,
    /// Address above which program command-line arguments (argv) are placed.
    ///
    /// (since Linux 3.5)
    pub arg_start: Option<u64>,
    /// Address below program command-line arguments (argv) are placed.
    ///
    /// (since Linux 3.5)
    pub arg_end: Option<u64>,
    /// Address above which program environment is placed.
    ///
    /// (since Linux 3.5)
    pub env_start: Option<u64>,
    /// Address below which program environment is placed.
    ///
    /// (since Linux 3.5)
    pub env_end: Option<u64>,
    /// The thread's exit status in the form reported by waitpid(2).
    ///
    /// (since Linux 3.5)
    pub exit_code: Option<i32>,
}

impl crate::FromRead for Stat {
    #[allow(clippy::cognitive_complexity)]
    fn from_read<R: Read>(mut r: R) -> ProcResult<Self> {
        // read in entire thing, this is only going to be 1 line
        let mut buf = Vec::with_capacity(512);
        r.read_to_end(&mut buf)?;

        let line = String::from_utf8_lossy(&buf);
        let buf = line.trim();

        // find the first opening paren, and split off the first part (pid)
        let start_paren = expect!(buf.find('('));
        let end_paren = expect!(buf.rfind(')'));
        let pid_s = &buf[..start_paren - 1];
        let comm = buf[start_paren + 1..end_paren].to_string();
        let rest = &buf[end_paren + 2..];

        let pid = expect!(FromStr::from_str(pid_s));

        let mut rest = rest.split(' ');
        let state = expect!(expect!(rest.next()).chars().next());

        let ppid = expect!(from_iter(&mut rest));
        let pgrp = expect!(from_iter(&mut rest));
        let session = expect!(from_iter(&mut rest));
        let tty_nr = expect!(from_iter(&mut rest));
        let tpgid = expect!(from_iter(&mut rest));
        let flags = expect!(from_iter(&mut rest));
        let minflt = expect!(from_iter(&mut rest));
        let cminflt = expect!(from_iter(&mut rest));
        let majflt = expect!(from_iter(&mut rest));
        let cmajflt = expect!(from_iter(&mut rest));
        let utime = expect!(from_iter(&mut rest));
        let stime = expect!(from_iter(&mut rest));
        let cutime = expect!(from_iter(&mut rest));
        let cstime = expect!(from_iter(&mut rest));
        let priority = expect!(from_iter(&mut rest));
        let nice = expect!(from_iter(&mut rest));
        let num_threads = expect!(from_iter(&mut rest));
        let itrealvalue = expect!(from_iter(&mut rest));
        let starttime = expect!(from_iter(&mut rest));
        let vsize = expect!(from_iter(&mut rest));
        let rss = expect!(from_iter(&mut rest));
        let rsslim = expect!(from_iter(&mut rest));
        let startcode = expect!(from_iter(&mut rest));
        let endcode = expect!(from_iter(&mut rest));
        let startstack = expect!(from_iter(&mut rest));
        let kstkesp = expect!(from_iter(&mut rest));
        let kstkeip = expect!(from_iter(&mut rest));
        let signal = expect!(from_iter(&mut rest));
        let blocked = expect!(from_iter(&mut rest));
        let sigignore = expect!(from_iter(&mut rest));
        let sigcatch = expect!(from_iter(&mut rest));
        let wchan = expect!(from_iter(&mut rest));
        let nswap = expect!(from_iter(&mut rest));
        let cnswap = expect!(from_iter(&mut rest));

        // Since 2.1.22
        let exit_signal = expect!(from_iter_optional(&mut rest));
        // Since 2.2.8
        let processor = expect!(from_iter_optional(&mut rest));
        // Since 2.5.19
        let rt_priority = expect!(from_iter_optional(&mut rest));
        let policy = expect!(from_iter_optional(&mut rest));
        // Since 2.6.18
        let delayacct_blkio_ticks = expect!(from_iter_optional(&mut rest));
        // Since 2.6.24
        let guest_time = expect!(from_iter_optional(&mut rest));
        let cguest_time = expect!(from_iter_optional(&mut rest));
        // Since 3.3.0
        let start_data = expect!(from_iter_optional(&mut rest));
        let end_data = expect!(from_iter_optional(&mut rest));
        let start_brk = expect!(from_iter_optional(&mut rest));
        // Since 3.5.0
        let arg_start = expect!(from_iter_optional(&mut rest));
        let arg_end = expect!(from_iter_optional(&mut rest));
        let env_start = expect!(from_iter_optional(&mut rest));
        let env_end = expect!(from_iter_optional(&mut rest));
        let exit_code = expect!(from_iter_optional(&mut rest));

        Ok(Stat {
            pid,
            comm,
            state,
            ppid,
            pgrp,
            session,
            tty_nr,
            tpgid,
            flags,
            minflt,
            cminflt,
            majflt,
            cmajflt,
            utime,
            stime,
            cutime,
            cstime,
            priority,
            nice,
            num_threads,
            itrealvalue,
            starttime,
            vsize,
            rss,
            rsslim,
            startcode,
            endcode,
            startstack,
            kstkesp,
            kstkeip,
            signal,
            blocked,
            sigignore,
            sigcatch,
            wchan,
            nswap,
            cnswap,
            exit_signal,
            processor,
            rt_priority,
            policy,
            delayacct_blkio_ticks,
            guest_time,
            cguest_time,
            start_data,
            end_data,
            start_brk,
            arg_start,
            arg_end,
            env_start,
            env_end,
            exit_code,
        })
    }
}

impl Stat {
    pub fn state(&self) -> ProcResult<ProcState> {
        ProcState::from_char(self.state)
            .ok_or_else(|| build_internal_error!(format!("{:?} is not a recognized process state", self.state)))
    }

    pub fn tty_nr(&self) -> (i32, i32) {
        // minor is bits 31-20 and 7-0
        // major is 15-8

        // mmmmmmmmmmmm____MMMMMMMMmmmmmmmm
        // 11111111111100000000000000000000
        let major = (self.tty_nr & 0xfff00) >> 8;
        let minor = (self.tty_nr & 0x000ff) | ((self.tty_nr >> 12) & 0xfff00);
        (major, minor)
    }

    /// The kernel flags word of the process, as a bitfield
    ///
    /// See also the [Stat::flags](struct.Stat.html#structfield.flags) field.
    pub fn flags(&self) -> ProcResult<StatFlags> {
        StatFlags::from_bits(self.flags)
            .ok_or_else(|| build_internal_error!(format!("Can't construct flags bitfield from {:?}", self.flags)))
    }

    /// Get the starttime of the process as a `DateTime` object.
    ///
    /// See also the [`starttime`](struct.Stat.html#structfield.starttime) field.
    ///
    /// This function requires the "chrono" features to be enabled (which it is by default).
    ///
    /// Since computing the absolute start time requires knowing the current boot time, this function returns
    /// a type that needs info about the current machine.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use procfs::WithCurrentSystemInfo;
    ///
    /// let me = procfs::process::Process::myself().unwrap();
    /// let stat = me.stat().unwrap();
    /// let start = stat.starttime().get().unwrap();
    /// ```
    #[cfg(feature = "chrono")]
    pub fn starttime(&self) -> impl crate::WithSystemInfo<Output = ProcResult<chrono::DateTime<chrono::Local>>> {
        move |si: &crate::SystemInfo| {
            let seconds_since_boot = self.starttime as f32 / si.ticks_per_second() as f32;

            Ok(si.boot_time()? + chrono::Duration::milliseconds((seconds_since_boot * 1000.0) as i64))
        }
    }

    /// Gets the Resident Set Size (in bytes)
    ///
    /// The `rss` field will return the same value in pages
    ///
    /// # Example
    ///
    /// Calculating the rss value in bytes requires knowing the page size, so a `SystemInfo` is needed.
    /// ```rust,ignore
    /// use procfs::WithCurrentSystemInfo;
    ///
    /// let me = procfs::process::Process::myself().unwrap();
    /// let stat = me.stat().unwrap();
    /// let bytes = stat.rss_bytes().get();
    /// ```
    pub fn rss_bytes(&self) -> impl crate::WithSystemInfo<Output = u64> {
        move |si: &crate::SystemInfo| self.rss * si.page_size()
    }
}
