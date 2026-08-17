//! Contains types and helpers for dealing with `EXC_RESOURCE` exceptions.
//!
//! `EXC_RESOURCE` exceptions embed details about the resource and the limits
//! it exceeded within the `code` and, in some cases `subcode`, fields of the exception
//!
//! See <https://github.com/apple-oss-distributions/xnu/blob/e6231be02a03711ca404e5121a151b24afbff733/osfmk/kern/exc_resource.h>
//! for the various constants and decoding of exception information wrapped in
//! this module.

use mach2::exception_types::EXC_RESOURCE;
use std::time::Duration;

/// The details for an `EXC_RESOURCE` exception as retrieved from the exception's
/// code and subcode
pub enum ResourceException {
    /// This is sent by the kernel when the CPU usage monitor is tripped. Possibly fatal.
    Cpu(CpuResourceException),
    /// This is sent by the kernel when the platform idle wakeups monitor is tripped. Possibly fatal.
    Wakeups(WakeupsResourceException),
    /// This is sent by the kernel when a task crosses its high watermark memory limit. Never fatal at least on current `MacOS` versions.
    Memory(MemoryResourceException),
    /// This is sent by the kernel when a task crosses its I/O limits. Never fatal.
    Io(IoResourceException),
    /// This is sent by the kernel when a task crosses its thread limit. Always fatal.
    Threads(ThreadsResourceException),
    /// This is sent by the kernel when the process is leaking ipc ports and has
    /// filled its port space. Always fatal.
    Ports(PortsResourceException),
    /// An unknown resource kind due to an addition to the set of possible
    /// resource exception kinds in `exc_resource.h`
    Unknown { kind: u8, flavor: u8 },
}

/// Each different resource exception type has 1 or more flavors that it can be,
/// and while these most likely don't change often, we try to be forward
/// compatible by not failing if a particular flavor is unknown
#[derive(Copy, Clone, Debug)]
pub enum Flavor<T: Copy + Clone + std::fmt::Debug> {
    Known(T),
    Unknown(u8),
}

impl<T: TryFrom<u8> + Copy + Clone + std::fmt::Debug> From<u64> for Flavor<T> {
    #[inline]
    fn from(code: u64) -> Self {
        let flavor = resource_exc_flavor(code);
        if let Ok(known) = T::try_from(flavor) {
            Self::Known(known)
        } else {
            Self::Unknown(flavor)
        }
    }
}

impl<T: PartialEq + Copy + Clone + std::fmt::Debug> PartialEq<T> for Flavor<T> {
    fn eq(&self, o: &T) -> bool {
        match self {
            Self::Known(flavor) => flavor == o,
            Self::Unknown(_) => false,
        }
    }
}

/// Retrieves the resource exception kind from an exception code
#[inline]
pub fn resource_exc_kind(code: u64) -> u8 {
    ((code >> 61) & 0x7) as u8
}

/// Retrieves the resource exception flavor from an exception code
#[inline]
pub fn resource_exc_flavor(code: u64) -> u8 {
    ((code >> 58) & 0x7) as u8
}

impl super::ExceptionInfo {
    /// If this is an `EXC_RESOURCE` exception, retrieves the exception metadata
    /// from the code, otherwise returns `None`
    pub fn resource_exception(&self) -> Option<ResourceException> {
        if self.kind != EXC_RESOURCE {
            return None;
        }

        let kind = resource_exc_kind(self.code);

        let res_exc = if kind == ResourceKind::Cpu as u8 {
            ResourceException::Cpu(CpuResourceException::from_exc_info(self.code, self.subcode))
        } else if kind == ResourceKind::Wakeups as u8 {
            ResourceException::Wakeups(WakeupsResourceException::from_exc_info(
                self.code,
                self.subcode,
            ))
        } else if kind == ResourceKind::Memory as u8 {
            ResourceException::Memory(MemoryResourceException::from_exc_info(self.code))
        } else if kind == ResourceKind::Io as u8 {
            ResourceException::Io(IoResourceException::from_exc_info(self.code, self.subcode))
        } else if kind == ResourceKind::Threads as u8 {
            ResourceException::Threads(ThreadsResourceException::from_exc_info(self.code))
        } else if kind == ResourceKind::Ports as u8 {
            ResourceException::Ports(PortsResourceException::from_exc_info(self.code))
        } else {
            ResourceException::Unknown {
                kind,
                flavor: resource_exc_flavor(self.code),
            }
        };

        Some(res_exc)
    }
}

/// The types of resources that an `EXC_RESOURCE` exception can pertain to
#[repr(u8)]
pub enum ResourceKind {
    Cpu = 1,
    Wakeups = 2,
    Memory = 3,
    Io = 4,
    Threads = 5,
    Ports = 6,
}

/// The flavors for a [`CpuResourceException`]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CpuFlavor {
    /// The process has surpassed its CPU limit
    Monitor = 1,
    /// The process has surpassed its CPU limit, and the process has been configured
    /// to make this exception fatal
    MonitorFatal = 2,
}

impl TryFrom<u8> for CpuFlavor {
    type Error = ();

    fn try_from(flavor: u8) -> Result<Self, Self::Error> {
        match flavor {
            1 => Ok(Self::Monitor),
            2 => Ok(Self::MonitorFatal),
            _ => Err(()),
        }
    }
}

/// These exceptions _may_ be fatal. They are not fatal by default at task
/// creation but can be made fatal by calling `proc_rlimit_control` with
/// `RLIMIT_CPU_USAGE_MONITOR` as the second argument and `CPUMON_MAKE_FATAL`
/// set in the flags. The flavor extracted from the exception code determines if
/// the exception is fatal.
///
/// [Kernel code](https://github.com/apple-oss-distributions/xnu/blob/e7776783b89a353188416a9a346c6cdb4928faad/osfmk/kern/thread.c#L2475-L2616)
#[derive(Copy, Clone, Debug)]
pub struct CpuResourceException {
    pub flavor: Flavor<CpuFlavor>,
    /// If the exception is fatal. Currently only true if the flavor is [`CpuFlavor::MonitorFatal`]
    pub is_fatal: bool,
    /// The time period in which the CPU limit was surpassed
    pub observation_interval: Duration,
    /// The CPU % limit
    pub limit: u8,
    /// The CPU % consumed by the task
    pub consumed: u8,
}

impl CpuResourceException {
    /*
     * code:
     * +-----------------------------------------------+
     * |[63:61] RESOURCE |[60:58] FLAVOR_CPU_ |[57:32] |
     * |_TYPE_CPU        |MONITOR[_FATAL]     |Unused  |
     * +-----------------------------------------------+
     * |[31:7]  Interval (sec)    | [6:0] CPU limit (%)|
     * +-----------------------------------------------+
     *
     * subcode:
     * +-----------------------------------------------+
     * |                          | [6:0] % of CPU     |
     * |                          | actually consumed  |
     * +-----------------------------------------------+
     *
     */
    #[inline]
    pub fn from_exc_info(code: u64, subcode: Option<u64>) -> Self {
        debug_assert_eq!(resource_exc_kind(code), ResourceKind::Cpu as u8);

        let flavor = Flavor::from(code);
        let interval_seconds = (code >> 7) & 0x1ffffff;
        let limit = (code & 0x7f) as u8;
        let consumed = subcode.map_or(0, |sc| sc & 0x7f) as u8;

        // The default is that cpu resource exceptions are not fatal, so
        // we only check the flavor against the (currently) one known value
        // that indicates the exception is fatal
        Self {
            flavor,
            is_fatal: flavor == CpuFlavor::MonitorFatal,
            observation_interval: Duration::from_secs(interval_seconds),
            limit,
            consumed,
        }
    }
}

/// The flavors for a [`WakeupsResourceException`]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WakeupsFlavor {
    Monitor = 1,
}

impl TryFrom<u8> for WakeupsFlavor {
    type Error = ();

    fn try_from(flavor: u8) -> Result<Self, Self::Error> {
        match flavor {
            1 => Ok(Self::Monitor),
            _ => Err(()),
        }
    }
}

/// These exceptions may be fatal. They are not fatal by default at task
/// creation, but can be made fatal by calling `proc_rlimit_control` with
/// `RLIMIT_WAKEUPS_MONITOR` as the second argument and `WAKEMON_MAKE_FATAL`
/// set in the flags. Calling [`proc_get_wakemon_params`](https://github.com/apple-oss-distributions/xnu/blob/e6231be02a03711ca404e5121a151b24afbff733/libsyscall/wrappers/libproc/libproc.c#L592-L608)
/// determines whether these exceptions are fatal.
///
/// [Kernel source](https://github.com/apple-oss-distributions/xnu/blob/e6231be02a03711ca404e5121a151b24afbff733/osfmk/kern/task.c#L7501-L7580)
pub struct WakeupsResourceException {
    pub flavor: Flavor<WakeupsFlavor>,
    /// The time period in which the number of wakeups was surpassed
    pub observation_interval: Duration,
    /// The number of wakeups permitted per second
    pub permitted: u32,
    /// The number of wakeups observed per second
    pub observed: u32,
}

impl WakeupsResourceException {
    /*
     * code:
     * +-----------------------------------------------+
     * |[63:61] RESOURCE |[60:58] FLAVOR_     |[57:32] |
     * |_TYPE_WAKEUPS    |WAKEUPS_MONITOR     |Unused  |
     * +-----------------------------------------------+
     * | [31:20] Observation     | [19:0] # of wakeups |
     * |         interval (sec)  | permitted (per sec) |
     * +-----------------------------------------------+
     *
     * subcode:
     * +-----------------------------------------------+
     * |                         | [19:0] # of wakeups |
     * |                         | observed (per sec)  |
     * +-----------------------------------------------+
     *
     */
    #[inline]
    pub fn from_exc_info(code: u64, subcode: Option<u64>) -> Self {
        debug_assert_eq!(resource_exc_kind(code), ResourceKind::Wakeups as u8);

        let flavor = Flavor::from(code);
        // Note that Apple has a bug in exc_resource.h where the masks in the
        // decode macros for the interval and the permitted wakeups have been swapped
        let interval_seconds = (code >> 20) & 0xfff;
        let permitted = (code & 0xfffff) as u32;
        let observed = subcode.map_or(0, |sc| sc & 0xfffff) as u32;

        Self {
            flavor,
            observation_interval: Duration::from_secs(interval_seconds),
            permitted,
            observed,
        }
    }
}

/// The flavors for a [`MemoryResourceException`]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MemoryFlavor {
    HighWatermark = 1,
}

impl TryFrom<u8> for MemoryFlavor {
    type Error = ();

    fn try_from(flavor: u8) -> Result<Self, Self::Error> {
        match flavor {
            1 => Ok(Self::HighWatermark),
            _ => Err(()),
        }
    }
}

/// These exceptions, as of this writing, are never fatal.
///
/// While memory exceptions _can_ be fatal, this appears to only be possible if
/// the kernel is built with `CONFIG_JETSAM` or in `DEVELOPMENT` or `DEBUG` modes,
/// so as of now, they should never be considered fatal, at least on `MacOS`
///
/// [Kernel source](https://github.com/apple-oss-distributions/xnu/blob/e6231be02a03711ca404e5121a151b24afbff733/osfmk/kern/task.c#L6767-L6874)
pub struct MemoryResourceException {
    pub flavor: Flavor<MemoryFlavor>,
    /// The limit in MiB of the high watermark
    pub limit_mib: u16,
}

impl MemoryResourceException {
    /*
     * code:
     * +------------------------------------------------+
     * |[63:61] RESOURCE |[60:58] FLAVOR_HIGH_ |[57:32] |
     * |_TYPE_MEMORY     |WATERMARK            |Unused  |
     * +------------------------------------------------+
     * |                         | [12:0] HWM limit (MB)|
     * +------------------------------------------------+
     *
     * subcode:
     * +------------------------------------------------+
     * |                                         unused |
     * +------------------------------------------------+
     *
     */
    #[inline]
    pub fn from_exc_info(code: u64) -> Self {
        debug_assert_eq!(resource_exc_kind(code), ResourceKind::Memory as u8);

        let flavor = Flavor::from(code);
        let limit_mib = (code & 0x1fff) as u16;

        Self { flavor, limit_mib }
    }
}

/// The flavors for an [`IoResourceException`]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IoFlavor {
    PhysicalWrites = 1,
    LogicalWrites = 2,
}

impl TryFrom<u8> for IoFlavor {
    type Error = ();

    fn try_from(flavor: u8) -> Result<Self, Self::Error> {
        match flavor {
            1 => Ok(Self::PhysicalWrites),
            2 => Ok(Self::LogicalWrites),
            _ => Err(()),
        }
    }
}

/// These exceptions are never fatal.
///
/// [Kernel source](https://github.com/apple-oss-distributions/xnu/blob/e6231be02a03711ca404e5121a151b24afbff733/osfmk/kern/task.c#L7739-L7792)
pub struct IoResourceException {
    pub flavor: Flavor<MemoryFlavor>,
    /// The time period in which the I/O limit was surpassed
    pub observation_interval: Duration,
    /// The I/O limit in MiB of the high watermark
    pub limit_mib: u16,
    /// The observed I/O in MiB
    pub observed_mib: u16,
}

impl IoResourceException {
    /*
     * code:
     * +-----------------------------------------------+
     * |[63:61] RESOURCE |[60:58] FLAVOR_IO_  |[57:32] |
     * |_TYPE_IO         |PHYSICAL/LOGICAL    |Unused  |
     * +-----------------------------------------------+
     * |[31:15]  Interval (sec)    | [14:0] Limit (MB) |
     * +-----------------------------------------------+
     *
     * subcode:
     * +-----------------------------------------------+
     * |                           | [14:0] I/O Count  |
     * |                           | (in MB)           |
     * +-----------------------------------------------+
     *
     */
    #[inline]
    pub fn from_exc_info(code: u64, subcode: Option<u64>) -> Self {
        debug_assert_eq!(resource_exc_kind(code), ResourceKind::Io as u8);

        let flavor = Flavor::from(code);
        let interval_seconds = (code >> 15) & 0x1ffff;
        let limit_mib = (code & 0x7fff) as u16;
        let observed_mib = subcode.map_or(0, |sc| sc & 0x7fff) as u16;

        Self {
            flavor,
            observation_interval: Duration::from_secs(interval_seconds),
            limit_mib,
            observed_mib,
        }
    }
}

/// The flavors for a [`ThreadsResourceException`]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ThreadsFlavor {
    HighWatermark = 1,
}

impl TryFrom<u8> for ThreadsFlavor {
    type Error = ();

    fn try_from(flavor: u8) -> Result<Self, Self::Error> {
        match flavor {
            1 => Ok(Self::HighWatermark),
            _ => Err(()),
        }
    }
}

/// This exception is provided for completeness sake, but is only possible if
/// the kernel is built in `DEVELOPMENT` or `DEBUG` modes.
///
/// [Kernel source](https://github.com/apple-oss-distributions/xnu/blob/e6231be02a03711ca404e5121a151b24afbff733/osfmk/kern/thread.c#L2575-L2620)
pub struct ThreadsResourceException {
    pub flavor: Flavor<ThreadsFlavor>,
    /// The thread limit
    pub limit: u16,
}

impl ThreadsResourceException {
    /*
     * code:
     * +--------------------------------------------------+
     * |[63:61] RESOURCE |[60:58] FLAVOR_        |[57:32] |
     * |_TYPE_THREADS    |THREADS_HIGH_WATERMARK |Unused  |
     * +--------------------------------------------------+
     * |[31:15]  Unused  | [14:0] Limit                   |
     * +--------------------------------------------------+
     *
     * subcode:
     * +-----------------------------------------------+
     * |                         | Unused              |
     * |                         |                     |
     * +-----------------------------------------------+
     *
     */
    #[inline]
    pub fn from_exc_info(code: u64) -> Self {
        debug_assert_eq!(resource_exc_kind(code), ResourceKind::Threads as u8);

        let flavor = Flavor::from(code);
        let limit = (code & 0x7fff) as u16;

        Self { flavor, limit }
    }
}

/// The flavors for a [`PortsResourceException`]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PortsFlavor {
    SpaceFull = 1,
}

impl TryFrom<u8> for PortsFlavor {
    type Error = ();

    fn try_from(flavor: u8) -> Result<Self, Self::Error> {
        match flavor {
            1 => Ok(Self::SpaceFull),
            _ => Err(()),
        }
    }
}

/// This exception is always fatal, and in fact I'm unsure if this exception
/// is even observable, as the kernel will kill the offending process if
/// the port space is full
///
/// [Kernel source](https://github.com/apple-oss-distributions/xnu/blob/e7776783b89a353188416a9a346c6cdb4928faad/osfmk/kern/task.c#L7907-L7969)
pub struct PortsResourceException {
    pub flavor: Flavor<ThreadsFlavor>,
    /// The number of allocated ports
    pub allocated: u32,
}

impl PortsResourceException {
    /*
     * code:
     * +-----------------------------------------------+
     * |[63:61] RESOURCE |[60:58] FLAVOR_     |[57:32] |
     * |_TYPE_PORTS      |PORT_SPACE_FULL     |Unused  |
     * +-----------------------------------------------+
     * | [31:24] Unused          | [23:0] # of ports   |
     * |                         | allocated           |
     * +-----------------------------------------------+
     *
     * subcode:
     * +-----------------------------------------------+
     * |                         | Unused              |
     * |                         |                     |
     * +-----------------------------------------------+
     *
     */
    #[inline]
    pub fn from_exc_info(code: u64) -> Self {
        debug_assert_eq!(resource_exc_kind(code), ResourceKind::Ports as u8);

        let flavor = Flavor::from(code);
        let allocated = (code & 0xffffff) as u32;

        Self { flavor, allocated }
    }
}
