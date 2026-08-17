#![allow(unknown_lints)]
// The suggested fix with `str::parse` removes support for Rust 1.48
#![allow(clippy::from_str_radix_10)]
#![deny(broken_intra_doc_links, invalid_html_tags)]
//! This crate provides to an interface into the linux `procfs` filesystem, usually mounted at
//! `/proc`.
//!
//! This is a pseudo-filesystem which is available on most every linux system and provides an
//! interface to kernel data structures.
//!
//! # `procfs-core`
//!
//! The `procfs-core` crate is a fully platform-independent crate that contains most of the data-structures and
//! parsing code.  Most people should first look at the `procfs` crate instead.

use bitflags::bitflags;

use std::fmt;
use std::io::{BufRead, BufReader, Read};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::{collections::HashMap, time::Duration};

#[cfg(feature = "serde1")]
use serde::{Deserialize, Serialize};

/// Types which can be parsed from a Read implementation.
pub trait FromRead: Sized {
    /// Read the type from a Read.
    fn from_read<R: Read>(r: R) -> ProcResult<Self>;

    /// Read the type from a file.
    fn from_file<P: AsRef<Path>>(path: P) -> ProcResult<Self> {
        std::fs::File::open(path.as_ref())
            .map_err(|e| e.into())
            .and_then(|f| Self::from_read(f))
            .map_err(|e| e.error_path(path.as_ref()))
    }
}

/// Types which can be parsed from a BufRead implementation.
pub trait FromBufRead: Sized {
    fn from_buf_read<R: BufRead>(r: R) -> ProcResult<Self>;
}

impl<T: FromBufRead> FromRead for T {
    fn from_read<R: Read>(r: R) -> ProcResult<Self> {
        Self::from_buf_read(BufReader::new(r))
    }
}

/// Types which can be parsed from a Read implementation and system info.
pub trait FromReadSI: Sized {
    /// Parse the type from a Read and system info.
    fn from_read<R: Read>(r: R, system_info: &SystemInfo) -> ProcResult<Self>;

    /// Parse the type from a file.
    fn from_file<P: AsRef<Path>>(path: P, system_info: &SystemInfo) -> ProcResult<Self> {
        std::fs::File::open(path.as_ref())
            .map_err(|e| e.into())
            .and_then(|f| Self::from_read(f, system_info))
            .map_err(|e| e.error_path(path.as_ref()))
    }
}

/// Types which can be parsed from a BufRead implementation and system info.
pub trait FromBufReadSI: Sized {
    fn from_buf_read<R: BufRead>(r: R, system_info: &SystemInfo) -> ProcResult<Self>;
}

impl<T: FromBufReadSI> FromReadSI for T {
    fn from_read<R: Read>(r: R, system_info: &SystemInfo) -> ProcResult<Self> {
        Self::from_buf_read(BufReader::new(r), system_info)
    }
}

/// Extension traits useful for importing wholesale.
pub mod prelude {
    pub use super::{FromBufRead, FromBufReadSI, FromRead, FromReadSI};
}

#[doc(hidden)]
pub trait IntoOption<T> {
    fn into_option(t: Self) -> Option<T>;
}

impl<T> IntoOption<T> for Option<T> {
    fn into_option(t: Option<T>) -> Option<T> {
        t
    }
}

impl<T, R> IntoOption<T> for Result<T, R> {
    fn into_option(t: Result<T, R>) -> Option<T> {
        t.ok()
    }
}

#[doc(hidden)]
pub trait IntoResult<T, E> {
    fn into(t: Self) -> Result<T, E>;
}

#[macro_export]
#[doc(hidden)]
macro_rules! build_internal_error {
    ($err: expr) => {
        crate::ProcError::InternalError(crate::InternalError {
            msg: format!("Internal Unwrap Error: {}", $err),
            file: file!(),
            line: line!(),
            #[cfg(feature = "backtrace")]
            backtrace: backtrace::Backtrace::new(),
        })
    };
    ($err: expr, $msg: expr) => {
        crate::ProcError::InternalError(crate::InternalError {
            msg: format!("Internal Unwrap Error: {}: {}", $msg, $err),
            file: file!(),
            line: line!(),
            #[cfg(feature = "backtrace")]
            backtrace: backtrace::Backtrace::new(),
        })
    };
}

// custom NoneError, since std::option::NoneError is nightly-only
// See https://github.com/rust-lang/rust/issues/42327
#[doc(hidden)]
pub struct NoneError;

impl std::fmt::Display for NoneError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NoneError")
    }
}

impl<T> IntoResult<T, NoneError> for Option<T> {
    fn into(t: Option<T>) -> Result<T, NoneError> {
        t.ok_or(NoneError)
    }
}

impl<T, E> IntoResult<T, E> for Result<T, E> {
    fn into(t: Result<T, E>) -> Result<T, E> {
        t
    }
}

#[allow(unused_macros)]
#[macro_export]
#[doc(hidden)]
macro_rules! proc_panic {
    ($e:expr) => {
        crate::IntoOption::into_option($e).unwrap_or_else(|| {
            panic!(
                "Failed to unwrap {}. Please report this as a procfs bug.",
                stringify!($e)
            )
        })
    };
    ($e:expr, $msg:expr) => {
        crate::IntoOption::into_option($e).unwrap_or_else(|| {
            panic!(
                "Failed to unwrap {} ({}). Please report this as a procfs bug.",
                stringify!($e),
                $msg
            )
        })
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! expect {
    ($e:expr) => {
        match crate::IntoResult::into($e) {
            Ok(v) => v,
            Err(e) => return Err(crate::build_internal_error!(e)),
        }
    };
    ($e:expr, $msg:expr) => {
        match crate::IntoResult::into($e) {
            Ok(v) => v,
            Err(e) => return Err(crate::build_internal_error!(e, $msg)),
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! from_str {
    ($t:tt, $e:expr) => {{
        let e = $e;
        crate::expect!(
            $t::from_str_radix(e, 10),
            format!("Failed to parse {} ({:?}) as a {}", stringify!($e), e, stringify!($t),)
        )
    }};
    ($t:tt, $e:expr, $radix:expr) => {{
        let e = $e;
        crate::expect!(
            $t::from_str_radix(e, $radix),
            format!("Failed to parse {} ({:?}) as a {}", stringify!($e), e, stringify!($t))
        )
    }};
    ($t:tt, $e:expr, $radix:expr, pid:$pid:expr) => {{
        let e = $e;
        crate::expect!(
            $t::from_str_radix(e, $radix),
            format!(
                "Failed to parse {} ({:?}) as a {} (pid {})",
                stringify!($e),
                e,
                stringify!($t),
                $pid
            )
        )
    }};
}

/// Auxiliary system information interface.
///
/// A few function in this crate require some extra system info to compute their results.  For example,
/// the [crate::process::Stat::rss_bytes()] function needs to know the page size.  Since `procfs-core` only parses
/// data and never interacts with a real system, this `SystemInfoInterface` is what allows real system info to be used.
///
/// If you are a user of the `procfs` crate, you'll normally use the `[procfs::WithCurrentSystemInfo]` trait.
/// For example:
///
/// ```rust,ignore
/// use procfs::WithCurrentSystemInfo;
///
/// let me = procfs::process::Process::myself().unwrap();
/// let stat = me.stat().unwrap();
/// let bytes = stat.rss_bytes().get();
/// ```
///
/// However, imagine that you captured a process's stat info, along with page size:
/// ```rust
/// # use procfs_core::{FromRead, WithSystemInfo};
/// # let stat_data = std::io::Cursor::new(b"475071 (cat) R 323893 475071 323893 34826 475071 4194304 94 0 0 0 0 0 0 0 20 0 1 0 201288208 5738496 225 18446744073709551615 94881179934720 94881179954601 140722831478832 0 0 0 0 0 0 0 0 0 17 4 0 0 0 0 0 94881179970608 94881179972224 94881184485376 140722831483757 140722831483777 140722831483777 140722831486955 0");
/// let stat = procfs_core::process::Stat::from_read(stat_data).unwrap();
///
/// let system_info = procfs_core::ExplicitSystemInfo {
///     boot_time_secs: 1692972606,
///     ticks_per_second: 100,
///     page_size: 4096,
///     is_little_endian: true,
/// };
///
/// let rss_bytes = stat.rss_bytes().with_system_info(&system_info);
/// ```
pub trait SystemInfoInterface {
    fn boot_time_secs(&self) -> ProcResult<u64>;
    fn ticks_per_second(&self) -> u64;
    fn page_size(&self) -> u64;
    /// Whether the system is little endian (true) or big endian (false).
    fn is_little_endian(&self) -> bool;

    #[cfg(feature = "chrono")]
    fn boot_time(&self) -> ProcResult<chrono::DateTime<chrono::Local>> {
        use chrono::TimeZone;
        let date_time = expect!(chrono::Local.timestamp_opt(self.boot_time_secs()? as i64, 0).single());
        Ok(date_time)
    }
}

/// Auxiliary system information.
pub type SystemInfo = dyn SystemInfoInterface;

/// A convenience stuct implementing [SystemInfoInterface] with explicitly-specified values.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct ExplicitSystemInfo {
    pub boot_time_secs: u64,
    pub ticks_per_second: u64,
    pub page_size: u64,
    pub is_little_endian: bool,
}

impl SystemInfoInterface for ExplicitSystemInfo {
    fn boot_time_secs(&self) -> ProcResult<u64> {
        Ok(self.boot_time_secs)
    }

    fn ticks_per_second(&self) -> u64 {
        self.ticks_per_second
    }

    fn page_size(&self) -> u64 {
        self.page_size
    }

    fn is_little_endian(&self) -> bool {
        self.is_little_endian
    }
}

/// Values which can provide an output given the [SystemInfo].
pub trait WithSystemInfo<'a>: 'a {
    type Output: 'a;

    /// Get the output derived from the given [SystemInfo].
    fn with_system_info(self, info: &SystemInfo) -> Self::Output;
}

impl<'a, F: 'a, R: 'a> WithSystemInfo<'a> for F
where
    F: FnOnce(&SystemInfo) -> R,
{
    type Output = R;

    fn with_system_info(self, info: &SystemInfo) -> Self::Output {
        self(info)
    }
}

#[doc(hidden)]
pub fn from_iter<'a, I, U>(i: I) -> ProcResult<U>
where
    I: IntoIterator<Item = &'a str>,
    U: FromStr,
{
    let mut iter = i.into_iter();
    let val = expect!(iter.next());
    match FromStr::from_str(val) {
        Ok(u) => Ok(u),
        Err(..) => Err(build_internal_error!("Failed to convert")),
    }
}

fn from_iter_optional<'a, I, U>(i: I) -> ProcResult<Option<U>>
where
    I: IntoIterator<Item = &'a str>,
    U: FromStr,
{
    let mut iter = i.into_iter();
    let Some(val) = iter.next() else {
        return Ok(None);
    };
    match FromStr::from_str(val) {
        Ok(u) => Ok(Some(u)),
        Err(..) => Err(build_internal_error!("Failed to convert")),
    }
}

mod cgroups;
pub use cgroups::*;

mod cpuinfo;
pub use cpuinfo::*;

mod crypto;
pub use crypto::*;

mod devices;
pub use devices::*;

mod diskstats;
pub use diskstats::*;

mod iomem;
pub use iomem::*;

pub mod keyring;

mod locks;
pub use locks::*;

mod mounts;
pub use mounts::*;

mod partitions;
pub use partitions::*;

mod meminfo;
pub use meminfo::*;

pub mod net;

mod pressure;
pub use pressure::*;

pub mod process;

mod kpageflags;
pub use kpageflags::*;

pub mod sys;
pub use sys::kernel::Version as KernelVersion;

mod sysvipc_shm;
pub use sysvipc_shm::*;

mod uptime;
pub use uptime::*;

// TODO temporary, only for procfs
pub trait FromStrRadix: Sized {
    fn from_str_radix(t: &str, radix: u32) -> Result<Self, std::num::ParseIntError>;
}

impl FromStrRadix for u64 {
    fn from_str_radix(s: &str, radix: u32) -> Result<u64, std::num::ParseIntError> {
        u64::from_str_radix(s, radix)
    }
}
impl FromStrRadix for i32 {
    fn from_str_radix(s: &str, radix: u32) -> Result<i32, std::num::ParseIntError> {
        i32::from_str_radix(s, radix)
    }
}

fn split_into_num<T: FromStrRadix>(s: &str, sep: char, radix: u32) -> ProcResult<(T, T)> {
    let mut s = s.split(sep);
    let a = expect!(FromStrRadix::from_str_radix(expect!(s.next()), radix));
    let b = expect!(FromStrRadix::from_str_radix(expect!(s.next()), radix));
    Ok((a, b))
}

/// This is used to hold both an IO error as well as the path of the file that originated the error
#[derive(Debug)]
#[doc(hidden)]
pub struct IoErrorWrapper {
    pub path: PathBuf,
    pub inner: std::io::Error,
}

impl std::error::Error for IoErrorWrapper {}
impl fmt::Display for IoErrorWrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "IoErrorWrapper({}): {}", self.path.display(), self.inner)
    }
}

/// The main error type for the procfs crate.
///
/// For more info, see the [ProcError] type.
pub type ProcResult<T> = Result<T, ProcError>;

/// The various error conditions in the procfs crate.
///
/// Most of the variants have an `Option<PathBuf>` component.  If the error root cause was related
/// to some operation on a file, the path of this file will be stored in this component.
#[derive(Debug)]
pub enum ProcError {
    /// A standard permission denied error.
    ///
    /// This will be a common error, since some files in the procfs filesystem are only readable by
    /// the root user.
    PermissionDenied(Option<PathBuf>),
    /// This might mean that the process no longer exists, or that your kernel doesn't support the
    /// feature you are trying to use.
    NotFound(Option<PathBuf>),
    /// This might mean that a procfs file has incomplete contents.
    ///
    /// If you encounter this error, consider retrying the operation.
    Incomplete(Option<PathBuf>),
    /// Any other IO error (rare).
    Io(std::io::Error, Option<PathBuf>),
    /// Any other non-IO error (very rare).
    Other(String),
    /// This error indicates that some unexpected error occurred.  This is a bug.  The inner
    /// [InternalError] struct will contain some more info.
    ///
    /// If you ever encounter this error, consider it a bug in the procfs crate and please report
    /// it on github.
    InternalError(InternalError),
}

/// Extensions for dealing with ProcErrors.
pub trait ProcErrorExt {
    /// Add path information to the error.
    fn error_path(self, path: &Path) -> Self;
}

impl ProcErrorExt for ProcError {
    fn error_path(mut self, path: &Path) -> Self {
        use ProcError::*;
        match &mut self {
            PermissionDenied(p) | NotFound(p) | Incomplete(p) | Io(_, p) if p.is_none() => {
                *p = Some(path.to_owned());
            }
            _ => (),
        }
        self
    }
}

impl<T> ProcErrorExt for ProcResult<T> {
    fn error_path(self, path: &Path) -> Self {
        self.map_err(|e| e.error_path(path))
    }
}

/// An internal error in the procfs crate
///
/// If you encounter this error, consider it a bug and please report it on
/// [github](https://github.com/eminence/procfs).
///
/// If you compile with the optional `backtrace` feature (disabled by default),
/// you can gain access to a stack trace of where the error happened.
#[cfg_attr(feature = "serde1", derive(Serialize))]
pub struct InternalError {
    pub msg: String,
    pub file: &'static str,
    pub line: u32,
    #[cfg(feature = "backtrace")]
    #[cfg_attr(feature = "serde1", serde(skip))]
    pub backtrace: backtrace::Backtrace,
}

impl std::fmt::Debug for InternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "bug at {}:{} (please report this procfs bug)\n{}",
            self.file, self.line, self.msg
        )
    }
}

impl std::fmt::Display for InternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "bug at {}:{} (please report this procfs bug)\n{}",
            self.file, self.line, self.msg
        )
    }
}

impl From<std::io::Error> for ProcError {
    fn from(io: std::io::Error) -> Self {
        use std::io::ErrorKind;
        let kind = io.kind();
        // the only way we'll have a path for the IO error is if this IO error
        // has a inner type
        if io.get_ref().is_some() {
            let inner = io.into_inner().unwrap();

            // is this inner type a IoErrorWrapper?
            match inner.downcast::<IoErrorWrapper>() {
                Ok(wrapper) => {
                    let path = wrapper.path;
                    match kind {
                        ErrorKind::PermissionDenied => ProcError::PermissionDenied(Some(path)),
                        ErrorKind::NotFound => ProcError::NotFound(Some(path)),
                        _other => {
                            // All platforms happen to have ESRCH=3, and windows actually
                            // translates it to a `NotFound` anyway.
                            const ESRCH: i32 = 3;
                            if matches!(wrapper.inner.raw_os_error(), Some(raw) if raw == ESRCH) {
                                // This "No such process" error gets mapped into a NotFound error
                                return ProcError::NotFound(Some(path));
                            } else {
                                ProcError::Io(wrapper.inner, Some(path))
                            }
                        }
                    }
                }
                Err(io) => {
                    // reconstruct the original error
                    ProcError::Io(std::io::Error::new(kind, io), None)
                }
            }
        } else {
            match kind {
                ErrorKind::PermissionDenied => ProcError::PermissionDenied(None),
                ErrorKind::NotFound => ProcError::NotFound(None),
                _other => ProcError::Io(io, None),
            }
        }
    }
}

impl From<&'static str> for ProcError {
    fn from(val: &'static str) -> Self {
        ProcError::Other(val.to_owned())
    }
}

impl From<std::num::ParseIntError> for ProcError {
    fn from(val: std::num::ParseIntError) -> Self {
        ProcError::Other(format!("ParseIntError: {}", val))
    }
}

impl From<std::string::ParseError> for ProcError {
    fn from(e: std::string::ParseError) -> Self {
        match e {}
    }
}

impl std::fmt::Display for ProcError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            // Variants with paths:
            ProcError::PermissionDenied(Some(p)) => write!(f, "Permission Denied: {}", p.display()),
            ProcError::NotFound(Some(p)) => write!(f, "File not found: {}", p.display()),
            ProcError::Incomplete(Some(p)) => write!(f, "Data incomplete: {}", p.display()),
            ProcError::Io(inner, Some(p)) => {
                write!(f, "Unexpected IO error({}): {}", p.display(), inner)
            }
            // Variants without paths:
            ProcError::PermissionDenied(None) => write!(f, "Permission Denied"),
            ProcError::NotFound(None) => write!(f, "File not found"),
            ProcError::Incomplete(None) => write!(f, "Data incomplete"),
            ProcError::Io(inner, None) => write!(f, "Unexpected IO error: {}", inner),

            ProcError::Other(s) => write!(f, "Unknown error {}", s),
            ProcError::InternalError(e) => write!(f, "Internal error: {}", e),
        }
    }
}

impl std::error::Error for ProcError {}

/// Load average figures.
///
/// Load averages are calculated as the number of jobs in the run queue (state R) or waiting for
/// disk I/O (state D) averaged over 1, 5, and 15 minutes.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct LoadAverage {
    /// The one-minute load average
    pub one: f32,
    /// The five-minute load average
    pub five: f32,
    /// The fifteen-minute load average
    pub fifteen: f32,
    /// The number of currently runnable kernel scheduling  entities  (processes,  threads).
    pub cur: u32,
    /// The number of kernel scheduling entities that currently exist on the system.
    pub max: u32,
    /// The fifth field is the PID of the process that was most recently created on the system.
    pub latest_pid: u32,
}

impl FromRead for LoadAverage {
    fn from_read<R: Read>(mut reader: R) -> ProcResult<Self> {
        let mut line = String::new();

        reader.read_to_string(&mut line)?;
        let mut s = line.split_whitespace();

        let one = expect!(f32::from_str(expect!(s.next())));
        let five = expect!(f32::from_str(expect!(s.next())));
        let fifteen = expect!(f32::from_str(expect!(s.next())));
        let curmax = expect!(s.next());
        let latest_pid = expect!(u32::from_str(expect!(s.next())));

        let mut s = curmax.split('/');
        let cur = expect!(u32::from_str(expect!(s.next())));
        let max = expect!(u32::from_str(expect!(s.next())));

        Ok(LoadAverage {
            one,
            five,
            fifteen,
            cur,
            max,
            latest_pid,
        })
    }
}

/// Possible values for a kernel config option
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub enum ConfigSetting {
    Yes,
    Module,
    Value(String),
}

/// The kernel configuration.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct KernelConfig(pub HashMap<String, ConfigSetting>);

impl FromBufRead for KernelConfig {
    fn from_buf_read<R: BufRead>(r: R) -> ProcResult<Self> {
        let mut map = HashMap::new();

        for line in r.lines() {
            let line = line?;
            if line.starts_with('#') {
                continue;
            }
            if line.contains('=') {
                let mut s = line.splitn(2, '=');
                let name = expect!(s.next()).to_owned();
                let value = match expect!(s.next()) {
                    "y" => ConfigSetting::Yes,
                    "m" => ConfigSetting::Module,
                    s => ConfigSetting::Value(s.to_owned()),
                };
                map.insert(name, value);
            }
        }

        Ok(KernelConfig(map))
    }
}

/// The amount of time, measured in ticks, the CPU has been in specific states
///
/// These fields are measured in ticks because the underlying data from the kernel is measured in ticks.
/// The number of ticks per second is generally 100 on most systems.
///
/// To convert this value to seconds, you can divide by the tps.  There are also convenience methods
/// that you can use too.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct CpuTime {
    /// Ticks spent in user mode
    pub user: u64,
    /// Ticks spent in user mode with low priority (nice)
    pub nice: u64,
    /// Ticks spent in system mode
    pub system: u64,
    /// Ticks spent in the idle state
    pub idle: u64,
    /// Ticks waiting for I/O to complete
    ///
    /// This value is not reliable, for the following reasons:
    ///
    /// 1. The CPU will not wait for I/O to complete; iowait is the time that a
    ///    task is waiting for I/O to complete.  When a CPU goes into idle state
    ///    for outstanding task I/O, another task will be scheduled on this CPU.
    ///
    /// 2. On a multi-core CPU, this task waiting for I/O to complete is not running
    ///    on any CPU, so the iowait for each CPU is difficult to calculate.
    ///
    /// 3. The value in this field may *decrease* in certain conditions.
    ///
    /// (Since Linux 2.5.41)
    pub iowait: Option<u64>,
    /// Ticks servicing interrupts
    ///
    /// (Since Linux 2.6.0)
    pub irq: Option<u64>,
    /// Ticks servicing softirqs
    ///
    /// (Since Linux 2.6.0)
    pub softirq: Option<u64>,
    /// Ticks of stolen time.
    ///
    /// Stolen time is the time spent in other operating systems when running in
    /// a virtualized environment.
    ///
    /// (Since Linux 2.6.11)
    pub steal: Option<u64>,
    /// Ticks spent running a virtual CPU for guest operating systems under control
    /// of the linux kernel
    ///
    /// (Since Linux 2.6.24)
    pub guest: Option<u64>,
    /// Ticks spent running a niced guest
    ///
    /// (Since Linux 2.6.33)
    pub guest_nice: Option<u64>,

    tps: u64,
}

impl CpuTime {
    fn from_str(s: &str, ticks_per_second: u64) -> ProcResult<CpuTime> {
        let mut s = s.split_whitespace();

        // Store this field in the struct so we don't have to attempt to unwrap ticks_per_second() when we convert
        // from ticks into other time units
        let tps = ticks_per_second;

        s.next();
        let user = from_str!(u64, expect!(s.next()));
        let nice = from_str!(u64, expect!(s.next()));
        let system = from_str!(u64, expect!(s.next()));
        let idle = from_str!(u64, expect!(s.next()));

        let iowait = s.next().map(|s| Ok(from_str!(u64, s))).transpose()?;
        let irq = s.next().map(|s| Ok(from_str!(u64, s))).transpose()?;
        let softirq = s.next().map(|s| Ok(from_str!(u64, s))).transpose()?;
        let steal = s.next().map(|s| Ok(from_str!(u64, s))).transpose()?;
        let guest = s.next().map(|s| Ok(from_str!(u64, s))).transpose()?;
        let guest_nice = s.next().map(|s| Ok(from_str!(u64, s))).transpose()?;

        Ok(CpuTime {
            user,
            nice,
            system,
            idle,
            iowait,
            irq,
            softirq,
            steal,
            guest,
            guest_nice,
            tps,
        })
    }

    /// Milliseconds spent in user mode
    pub fn user_ms(&self) -> u64 {
        let ms_per_tick = 1000 / self.tps;
        self.user * ms_per_tick
    }

    /// Time spent in user mode
    pub fn user_duration(&self) -> Duration {
        Duration::from_millis(self.user_ms())
    }

    /// Milliseconds spent in user mode with low priority (nice)
    pub fn nice_ms(&self) -> u64 {
        let ms_per_tick = 1000 / self.tps;
        self.nice * ms_per_tick
    }

    /// Time spent in user mode with low priority (nice)
    pub fn nice_duration(&self) -> Duration {
        Duration::from_millis(self.nice_ms())
    }

    /// Milliseconds spent in system mode
    pub fn system_ms(&self) -> u64 {
        let ms_per_tick = 1000 / self.tps;
        self.system * ms_per_tick
    }

    /// Time spent in system mode
    pub fn system_duration(&self) -> Duration {
        Duration::from_millis(self.system_ms())
    }

    /// Milliseconds spent in the idle state
    pub fn idle_ms(&self) -> u64 {
        let ms_per_tick = 1000 / self.tps;
        self.idle * ms_per_tick
    }

    /// Time spent in the idle state
    pub fn idle_duration(&self) -> Duration {
        Duration::from_millis(self.idle_ms())
    }

    /// Milliseconds spent waiting for I/O to complete
    pub fn iowait_ms(&self) -> Option<u64> {
        let ms_per_tick = 1000 / self.tps;
        self.iowait.map(|io| io * ms_per_tick)
    }

    /// Time spent waiting for I/O to complete
    pub fn iowait_duration(&self) -> Option<Duration> {
        self.iowait_ms().map(Duration::from_millis)
    }

    /// Milliseconds spent servicing interrupts
    pub fn irq_ms(&self) -> Option<u64> {
        let ms_per_tick = 1000 / self.tps;
        self.irq.map(|ms| ms * ms_per_tick)
    }

    /// Time spent servicing interrupts
    pub fn irq_duration(&self) -> Option<Duration> {
        self.irq_ms().map(Duration::from_millis)
    }

    /// Milliseconds spent servicing softirqs
    pub fn softirq_ms(&self) -> Option<u64> {
        let ms_per_tick = 1000 / self.tps;
        self.softirq.map(|ms| ms * ms_per_tick)
    }

    /// Time spent servicing softirqs
    pub fn softirq_duration(&self) -> Option<Duration> {
        self.softirq_ms().map(Duration::from_millis)
    }

    /// Milliseconds of stolen time
    pub fn steal_ms(&self) -> Option<u64> {
        let ms_per_tick = 1000 / self.tps;
        self.steal.map(|ms| ms * ms_per_tick)
    }

    /// Amount of stolen time
    pub fn steal_duration(&self) -> Option<Duration> {
        self.steal_ms().map(Duration::from_millis)
    }

    /// Milliseconds spent running a virtual CPU for guest operating systems under control of the linux kernel
    pub fn guest_ms(&self) -> Option<u64> {
        let ms_per_tick = 1000 / self.tps;
        self.guest.map(|ms| ms * ms_per_tick)
    }

    /// Time spent running a virtual CPU for guest operating systems under control of the linux kernel
    pub fn guest_duration(&self) -> Option<Duration> {
        self.guest_ms().map(Duration::from_millis)
    }

    /// Milliseconds spent running a niced guest
    pub fn guest_nice_ms(&self) -> Option<u64> {
        let ms_per_tick = 1000 / self.tps;
        self.guest_nice.map(|ms| ms * ms_per_tick)
    }

    /// Time spent running a niced guest
    pub fn guest_nice_duration(&self) -> Option<Duration> {
        self.guest_nice_ms().map(Duration::from_millis)
    }
}

/// Kernel/system statistics, from `/proc/stat`
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct KernelStats {
    /// The amount of time the system spent in various states
    pub total: CpuTime,
    /// The amount of time that specific CPUs spent in various states
    pub cpu_time: Vec<CpuTime>,

    /// The number of context switches that the system underwent
    pub ctxt: u64,

    /// Boot time, in number of seconds since the Epoch
    pub btime: u64,

    /// Number of forks since boot
    pub processes: u64,

    /// Number of processes in runnable state
    ///
    /// (Since Linux 2.5.45)
    pub procs_running: Option<u32>,

    /// Number of processes blocked waiting for I/O
    ///
    /// (Since Linux 2.5.45)
    pub procs_blocked: Option<u32>,
}

impl FromBufReadSI for KernelStats {
    fn from_buf_read<R: BufRead>(r: R, system_info: &SystemInfo) -> ProcResult<Self> {
        let lines = r.lines();

        let mut total_cpu = None;
        let mut cpus = Vec::new();
        let mut ctxt = None;
        let mut btime = None;
        let mut processes = None;
        let mut procs_running = None;
        let mut procs_blocked = None;

        for line in lines {
            let line = line?;
            if line.starts_with("cpu ") {
                total_cpu = Some(CpuTime::from_str(&line, system_info.ticks_per_second())?);
            } else if line.starts_with("cpu") {
                cpus.push(CpuTime::from_str(&line, system_info.ticks_per_second())?);
            } else if let Some(stripped) = line.strip_prefix("ctxt ") {
                ctxt = Some(from_str!(u64, stripped));
            } else if let Some(stripped) = line.strip_prefix("btime ") {
                btime = Some(from_str!(u64, stripped));
            } else if let Some(stripped) = line.strip_prefix("processes ") {
                processes = Some(from_str!(u64, stripped));
            } else if let Some(stripped) = line.strip_prefix("procs_running ") {
                procs_running = Some(from_str!(u32, stripped));
            } else if let Some(stripped) = line.strip_prefix("procs_blocked ") {
                procs_blocked = Some(from_str!(u32, stripped));
            }
        }

        Ok(KernelStats {
            total: expect!(total_cpu),
            cpu_time: cpus,
            ctxt: expect!(ctxt),
            btime: expect!(btime),
            processes: expect!(processes),
            procs_running,
            procs_blocked,
        })
    }
}

/// Various virtual memory statistics
///
/// Since the exact set of statistics will vary from kernel to kernel, and because most of them are
/// not well documented, this struct contains a HashMap instead of specific members. Consult the
/// kernel source code for more details of this data.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct VmStat(pub HashMap<String, i64>);

impl FromBufRead for VmStat {
    fn from_buf_read<R: BufRead>(r: R) -> ProcResult<Self> {
        let mut map = HashMap::new();
        for line in r.lines() {
            let line = line?;
            let mut split = line.split_whitespace();
            let name = expect!(split.next());
            let val = from_str!(i64, expect!(split.next()));
            map.insert(name.to_owned(), val);
        }

        Ok(VmStat(map))
    }
}

/// Details about a loaded kernel module
///
/// For an example, see the [lsmod.rs](https://github.com/eminence/procfs/tree/master/examples)
/// example in the source repo.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct KernelModule {
    /// The name of the module
    pub name: String,

    /// The size of the module
    pub size: u32,

    /// The number of references in the kernel to this module.  This can be -1 if the module is unloading
    pub refcount: i32,

    /// A list of modules that depend on this module.
    pub used_by: Vec<String>,

    /// The module state
    ///
    /// This will probably always be "Live", but it could also be either "Unloading" or "Loading"
    pub state: String,
}

/// A set of loaded kernel modules
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct KernelModules(pub HashMap<String, KernelModule>);

impl FromBufRead for KernelModules {
    /// This should correspond to the data in `/proc/modules`.
    fn from_buf_read<R: BufRead>(r: R) -> ProcResult<Self> {
        // kernel reference: kernel/module.c m_show()
        let mut map = HashMap::new();
        for line in r.lines() {
            let line: String = line?;
            let mut s = line.split_whitespace();
            let name = expect!(s.next());
            let size = from_str!(u32, expect!(s.next()));
            let refcount = from_str!(i32, expect!(s.next()));
            let used_by: &str = expect!(s.next());
            let state = expect!(s.next());

            map.insert(
                name.to_string(),
                KernelModule {
                    name: name.to_string(),
                    size,
                    refcount,
                    used_by: if used_by == "-" {
                        Vec::new()
                    } else {
                        used_by
                            .split(',')
                            .filter(|s| !s.is_empty())
                            .map(|s| s.to_string())
                            .collect()
                    },
                    state: state.to_string(),
                },
            );
        }

        Ok(KernelModules(map))
    }
}

/// A list of the arguments passed to the Linux kernel at boot time.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct KernelCmdline(pub Vec<String>);

impl FromRead for KernelCmdline {
    /// This should correspond to the data in `/proc/cmdline`.
    fn from_read<R: Read>(mut r: R) -> ProcResult<Self> {
        let mut buf = String::new();
        r.read_to_string(&mut buf)?;
        Ok(KernelCmdline(
            buf.split(' ')
                .filter_map(|s| if !s.is_empty() { Some(s.to_string()) } else { None })
                .collect(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kernel_from_str() {
        let k = KernelVersion::from_str("1.2.3").unwrap();
        assert_eq!(k.major, 1);
        assert_eq!(k.minor, 2);
        assert_eq!(k.patch, 3);

        let k = KernelVersion::from_str("4.9.16-gentoo").unwrap();
        assert_eq!(k.major, 4);
        assert_eq!(k.minor, 9);
        assert_eq!(k.patch, 16);

        let k = KernelVersion::from_str("4.9.266-0.1.ac.225.84.332.metal1.x86_64").unwrap();
        assert_eq!(k.major, 4);
        assert_eq!(k.minor, 9);
        assert_eq!(k.patch, 266);
    }

    #[test]
    fn test_kernel_cmp() {
        let a = KernelVersion::from_str("1.2.3").unwrap();
        let b = KernelVersion::from_str("1.2.3").unwrap();
        let c = KernelVersion::from_str("1.2.4").unwrap();
        let d = KernelVersion::from_str("1.5.4").unwrap();
        let e = KernelVersion::from_str("2.5.4").unwrap();

        assert_eq!(a, b);
        assert!(a < c);
        assert!(a < d);
        assert!(a < e);
        assert!(e > d);
        assert!(e > c);
        assert!(e > b);
    }

    #[test]
    fn test_loadavg_from_reader() -> ProcResult<()> {
        let load_average = LoadAverage::from_read("2.63 1.00 1.42 3/4280 2496732".as_bytes())?;

        assert_eq!(load_average.one, 2.63);
        assert_eq!(load_average.five, 1.00);
        assert_eq!(load_average.fifteen, 1.42);
        assert_eq!(load_average.max, 4280);
        assert_eq!(load_average.cur, 3);
        assert_eq!(load_average.latest_pid, 2496732);
        Ok(())
    }

    #[test]
    fn test_from_str() -> ProcResult<()> {
        assert_eq!(from_str!(u8, "12"), 12);
        assert_eq!(from_str!(u8, "A", 16), 10);
        Ok(())
    }

    #[test]
    fn test_from_str_fail() {
        fn inner() -> ProcResult<()> {
            let s = "four";
            from_str!(u8, s);
            unreachable!()
        }

        assert!(inner().is_err())
    }

    #[test]
    fn test_nopanic() {
        fn _inner() -> ProcResult<bool> {
            let x: Option<bool> = None;
            let y: bool = expect!(x);
            Ok(y)
        }

        let r = _inner();
        println!("{:?}", r);
        assert!(r.is_err());

        fn _inner2() -> ProcResult<bool> {
            let _f: std::fs::File = expect!(std::fs::File::open("/doesnotexist"));
            Ok(true)
        }

        let r = _inner2();
        println!("{:?}", r);
        assert!(r.is_err());
    }

    #[cfg(feature = "backtrace")]
    #[test]
    fn test_backtrace() {
        fn _inner() -> ProcResult<bool> {
            let _f: std::fs::File = expect!(std::fs::File::open("/doesnotexist"));
            Ok(true)
        }

        let r = _inner();
        println!("{:?}", r);
    }
}
