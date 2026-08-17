// Take a look at the license at the top of the repository in the LICENSE file.

#[cfg(any(
    feature = "component",
    feature = "disk",
    feature = "network",
    feature = "system",
    feature = "user"
))]
use serde::{ser::SerializeStruct, Serialize, Serializer};

#[cfg(feature = "disk")]
impl Serialize for crate::Disk {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // `7` corresponds to the (maximum) number of fields.
        let mut state = serializer.serialize_struct("Disk", 7)?;

        state.serialize_field("DiskKind", &self.kind())?;
        if let Some(s) = self.name().to_str() {
            state.serialize_field("name", s)?;
        }
        state.serialize_field("file_system", &self.file_system())?;
        state.serialize_field("mount_point", &self.mount_point())?;
        state.serialize_field("total_space", &self.total_space())?;
        state.serialize_field("available_space", &self.available_space())?;
        state.serialize_field("is_removable", &self.is_removable())?;

        state.end()
    }
}

#[cfg(feature = "disk")]
impl Serialize for crate::Disks {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_seq(self.iter())
    }
}

#[cfg(feature = "disk")]
impl Serialize for crate::DiskKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let (index, variant, maybe_value) = match *self {
            Self::HDD => (0, "HDD", None),
            Self::SSD => (1, "SSD", None),
            Self::Unknown(ref s) => (2, "Unknown", Some(s)),
        };

        if let Some(ref value) = maybe_value {
            serializer.serialize_newtype_variant("DiskKind", index, variant, value)
        } else {
            serializer.serialize_unit_variant("DiskKind", index, variant)
        }
    }
}

#[cfg(feature = "system")]
impl Serialize for crate::Pid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_newtype_struct("Pid", &self.to_string())
    }
}

#[cfg(feature = "system")]
impl Serialize for crate::Process {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // `19` corresponds to the (maximum) number of fields.
        let mut state = serializer.serialize_struct("Process", 19)?;

        state.serialize_field("name", &self.name())?;
        state.serialize_field("cmd", &self.cmd())?;
        state.serialize_field("exe", &self.exe())?;
        state.serialize_field("pid", &self.pid().as_u32())?;
        state.serialize_field("environ", &self.environ())?;
        state.serialize_field("cwd", &self.cwd())?;
        state.serialize_field("root", &self.root())?;
        state.serialize_field("memory", &self.memory())?;
        state.serialize_field("virtual_memory", &self.virtual_memory())?;
        state.serialize_field("parent", &self.parent())?;
        state.serialize_field("status", &self.status())?;
        state.serialize_field("start_time", &self.start_time())?;
        state.serialize_field("run_time", &self.run_time())?;
        state.serialize_field("cpu_usage", &self.cpu_usage())?;
        state.serialize_field("disk_usage", &self.disk_usage())?;
        state.serialize_field("user_id", &self.user_id())?;
        state.serialize_field("group_id", &self.group_id())?;
        state.serialize_field("session_id", &self.session_id())?;

        state.end()
    }
}

#[cfg(feature = "system")]
impl Serialize for crate::Cpu {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // `5` corresponds to the number of fields.
        let mut state = serializer.serialize_struct("Cpu", 5)?;

        state.serialize_field("cpu_usage", &self.cpu_usage())?;
        state.serialize_field("name", &self.name())?;
        state.serialize_field("vendor_id", &self.vendor_id())?;
        state.serialize_field("brand", &self.brand())?;
        state.serialize_field("frequency", &self.frequency())?;

        state.end()
    }
}

#[cfg(feature = "system")]
impl serde::Serialize for crate::System {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // `19` corresponds to the number of fields.
        let mut state = serializer.serialize_struct("System", 19)?;

        state.serialize_field("global_cpu_usage", &self.global_cpu_usage())?;
        state.serialize_field("cpus", &self.cpus())?;

        state.serialize_field("physical_core_count", &self.physical_core_count())?;
        state.serialize_field("total_memory", &self.total_memory())?;
        state.serialize_field("free_memory", &self.free_memory())?;
        state.serialize_field("available_memory", &self.available_memory())?;
        state.serialize_field("used_memory", &self.used_memory())?;
        state.serialize_field("total_swap", &self.total_swap())?;
        state.serialize_field("free_swap", &self.free_swap())?;
        state.serialize_field("used_swap", &self.used_swap())?;

        state.serialize_field("uptime", &Self::uptime())?;
        state.serialize_field("boot_time", &Self::boot_time())?;
        state.serialize_field("load_average", &Self::load_average())?;
        state.serialize_field("name", &Self::name())?;
        state.serialize_field("kernel_version", &Self::kernel_version())?;
        state.serialize_field("os_version", &Self::os_version())?;
        state.serialize_field("long_os_version", &Self::long_os_version())?;
        state.serialize_field("distribution_id", &Self::distribution_id())?;
        state.serialize_field("host_name", &Self::host_name())?;

        state.end()
    }
}

#[cfg(feature = "system")]
impl Serialize for crate::CGroupLimits {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // `3` corresponds to the number of fields.
        let mut state = serializer.serialize_struct("CGroupLimits", 3)?;

        state.serialize_field("total_memory", &self.total_memory)?;
        state.serialize_field("free_memory", &self.free_memory)?;
        state.serialize_field("free_swap", &self.free_swap)?;

        state.end()
    }
}

#[cfg(feature = "system")]
impl Serialize for crate::ThreadKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let (index, variant) = match *self {
            Self::Kernel => (0, "Kernel"),
            Self::Userland => (1, "Userland"),
        };

        serializer.serialize_unit_variant("ThreadKind", index, variant)
    }
}

#[cfg(feature = "system")]
impl Serialize for crate::Signal {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let (index, variant) = match *self {
            Self::Hangup => (0, "Hangup"),
            Self::Interrupt => (1, "Interrupt"),
            Self::Quit => (2, "Quit"),
            Self::Illegal => (3, "Illegal"),
            Self::Trap => (4, "Trap"),
            Self::Abort => (5, "Abort"),
            Self::IOT => (6, "IOT"),
            Self::Bus => (7, "Bus"),
            Self::FloatingPointException => (8, "FloatingPointException"),
            Self::Kill => (9, "Kill"),
            Self::User1 => (10, "User1"),
            Self::Segv => (11, "Segv"),
            Self::User2 => (12, "User2"),
            Self::Pipe => (13, "Pipe"),
            Self::Alarm => (14, "Alarm"),
            Self::Term => (15, "Term"),
            Self::Child => (16, "Child"),
            Self::Continue => (17, "Continue"),
            Self::Stop => (18, "Stop"),
            Self::TSTP => (19, "TSTP"),
            Self::TTIN => (20, "TTIN"),
            Self::TTOU => (21, "TTOU"),
            Self::Urgent => (22, "Urgent"),
            Self::XCPU => (23, "XCPU"),
            Self::XFSZ => (24, "XFSZ"),
            Self::VirtualAlarm => (25, "VirtualAlarm"),
            Self::Profiling => (26, "Profiling"),
            Self::Winch => (27, "Winch"),
            Self::IO => (28, "IO"),
            Self::Poll => (29, "Poll"),
            Self::Power => (30, "Power"),
            Self::Sys => (31, "Sys"),
        };

        serializer.serialize_unit_variant("Signal", index, variant)
    }
}

#[cfg(feature = "system")]
impl Serialize for crate::LoadAvg {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // `3` corresponds to the number of fields.
        let mut state = serializer.serialize_struct("LoadAvg", 3)?;

        state.serialize_field("one", &self.one)?;
        state.serialize_field("five", &self.five)?;
        state.serialize_field("fifteen", &self.fifteen)?;
        state.end()
    }
}

#[cfg(feature = "system")]
impl Serialize for crate::ProcessStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let (index, variant, maybe_value) = match *self {
            Self::Idle => (0, "Idle", None),
            Self::Run => (1, "Run", None),
            Self::Sleep => (2, "Sleep", None),
            Self::Stop => (3, "Stop", None),
            Self::Zombie => (4, "Zombie", None),
            Self::Tracing => (5, "Tracing", None),
            Self::Dead => (6, "Dead", None),
            Self::Wakekill => (7, "Wakekill", None),
            Self::Waking => (8, "Waking", None),
            Self::Parked => (9, "Parked", None),
            Self::LockBlocked => (10, "LockBlocked", None),
            Self::UninterruptibleDiskSleep => (11, "UninterruptibleDiskSleep", None),
            Self::Unknown(n) => (12, "Unknown", Some(n)),
        };

        if let Some(ref value) = maybe_value {
            serializer.serialize_newtype_variant("ProcessStatus", index, variant, value)
        } else {
            serializer.serialize_unit_variant("ProcessStatus", index, variant)
        }
    }
}

#[cfg(feature = "system")]
impl Serialize for crate::DiskUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // `4` corresponds to the number of fields.
        let mut state = serializer.serialize_struct("DiskUsage", 4)?;

        state.serialize_field("total_written_bytes", &self.total_written_bytes)?;
        state.serialize_field("written_bytes", &self.written_bytes)?;
        state.serialize_field("total_read_bytes", &self.total_read_bytes)?;
        state.serialize_field("read_bytes", &self.read_bytes)?;

        state.end()
    }
}

#[cfg(feature = "component")]
impl Serialize for crate::Components {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_seq(self.iter())
    }
}

#[cfg(feature = "component")]
impl Serialize for crate::Component {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // `4` corresponds to the number of fields.
        let mut state = serializer.serialize_struct("Component", 4)?;

        state.serialize_field("temperature", &self.temperature())?;
        state.serialize_field("max", &self.max())?;
        state.serialize_field("critical", &self.critical())?;
        state.serialize_field("label", &self.label())?;

        state.end()
    }
}

#[cfg(feature = "network")]
impl Serialize for crate::Networks {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_seq(self.iter())
    }
}

#[cfg(feature = "network")]
impl Serialize for crate::NetworkData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // `13` corresponds to the number of fields.
        let mut state = serializer.serialize_struct("NetworkData", 13)?;

        state.serialize_field("received", &self.received())?;
        state.serialize_field("total_received", &self.total_received())?;
        state.serialize_field("transmitted", &self.transmitted())?;
        state.serialize_field("total_transmitted", &self.total_transmitted())?;
        state.serialize_field("packets_received", &self.packets_received())?;
        state.serialize_field("total_packets_received", &self.total_packets_received())?;
        state.serialize_field("packets_transmitted", &self.packets_transmitted())?;
        state.serialize_field(
            "total_packets_transmitted",
            &self.total_packets_transmitted(),
        )?;
        state.serialize_field("errors_on_received", &self.errors_on_received())?;
        state.serialize_field("total_errors_on_received", &self.total_errors_on_received())?;
        state.serialize_field("errors_on_transmitted", &self.errors_on_transmitted())?;
        state.serialize_field(
            "total_errors_on_transmitted",
            &self.total_errors_on_transmitted(),
        )?;
        state.serialize_field("mac_address", &self.mac_address())?;
        state.serialize_field("ip_networks", &self.ip_networks())?;

        state.end()
    }
}

#[cfg(feature = "network")]
impl Serialize for crate::MacAddr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_newtype_struct("MacAddr", &self.0)
    }
}

#[cfg(feature = "network")]
impl Serialize for crate::IpNetwork {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("IpNetwork", 2)?;

        state.serialize_field("addr", &self.addr)?;
        state.serialize_field("prefix", &self.prefix)?;

        state.end()
    }
}

#[cfg(feature = "user")]
impl Serialize for crate::Users {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_seq(self.iter())
    }
}

#[cfg(feature = "user")]
impl Serialize for crate::User {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // `4` corresponds to the number of fields.
        let mut state = serializer.serialize_struct("User", 4)?;

        state.serialize_field("id", &self.id())?;
        state.serialize_field("group_id", &self.group_id())?;
        state.serialize_field("name", &self.name())?;
        state.serialize_field("groups", &self.groups())?;

        state.end()
    }
}

#[cfg(feature = "user")]
impl Serialize for crate::Group {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // `2` corresponds to the number of fields.
        let mut state = serializer.serialize_struct("Group", 2)?;

        state.serialize_field("id", &self.id())?;
        state.serialize_field("name", &self.name())?;

        state.end()
    }
}

#[cfg(any(feature = "user", feature = "system"))]
impl Serialize for crate::Gid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_newtype_struct("Gid", &self.to_string())
    }
}

#[cfg(any(feature = "user", feature = "system"))]
impl Serialize for crate::Uid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_newtype_struct("Uid", &self.to_string())
    }
}
