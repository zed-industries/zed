use {
    crate::{minidump_format::PlatformId, serializers::*},
    nix::sys::utsname::uname,
};

cfg_if::cfg_if! {
    if #[cfg(any(
        target_arch = "x86_64",
        target_arch = "x86",
        target_arch = "mips",
        target_arch = "mips64"
    ))]
    {
        pub mod x86_mips;
        pub use x86_mips as imp;
    } else if #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
    ))]
    {
        pub mod arm;
        pub use arm as imp;
    }
}

pub use imp::write_cpu_information;

#[derive(Debug, thiserror::Error, serde::Serialize)]
pub enum CpuInfoError {
    #[error("IO error for file /proc/cpuinfo")]
    IOError(
        #[from]
        #[serde(serialize_with = "serialize_io_error")]
        std::io::Error,
    ),
    #[error("Not all entries of /proc/cpuinfo found!")]
    NotAllProcEntriesFound,
    #[error("Couldn't parse core from file")]
    UnparsableInteger(
        #[from]
        #[serde(skip)]
        std::num::ParseIntError,
    ),
    #[error("Couldn't parse cores: {0}")]
    UnparsableCores(String),
}

/// Retrieves the [`MDOSPlatform`] and synthesized version information
pub fn os_information() -> (PlatformId, String) {
    let platform_id = if cfg!(target_os = "android") {
        PlatformId::Android
    } else {
        PlatformId::Linux
    };

    // This is quite unfortunate, but the primary reason that uname could fail
    // would be if it failed to fill out the nodename (hostname) field, even
    // though we don't care about that particular field at all
    let info = uname().map_or_else(
        |_e| {
            let os = if platform_id == PlatformId::Linux {
                "Linux"
            } else {
                "Android"
            };

            let machine = if cfg!(target_arch = "x86_64") {
                "x86_64"
            } else if cfg!(target_arch = "x86") {
                "x86"
            } else if cfg!(target_arch = "aarch64") {
                "aarch64"
            } else if cfg!(target_arch = "arm") {
                "arm"
            } else {
                "<unknown>"
            };

            // TODO: Fallback to other sources of information, eg /etc/os-release
            format!("{os} <unknown> <unknown> {machine}")
        },
        |info| {
            format!(
                "{} {} {} {}",
                info.sysname().to_str().unwrap_or("<unknown>"),
                info.release().to_str().unwrap_or("<unknown>"),
                info.version().to_str().unwrap_or("<unknown>"),
                info.machine().to_str().unwrap_or("<unknown>"),
            )
        },
    );

    (platform_id, info)
}
