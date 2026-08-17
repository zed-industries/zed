// Take a look at the license at the top of the repository in the LICENSE file.

pub(crate) mod utils;

cfg_if! {
    if #[cfg(feature = "system")] {
        pub mod cpu;
        pub mod process;
        pub mod system;

        pub(crate) use self::cpu::CpuInner;
        pub(crate) use self::process::ProcessInner;
        pub(crate) use self::system::SystemInner;
        pub use self::system::{MINIMUM_CPU_UPDATE_INTERVAL, SUPPORTED_SIGNALS};
    }
    if #[cfg(feature = "disk")] {
        pub mod disk;

        pub(crate) use self::disk::DiskInner;
        pub(crate) use crate::unix::DisksInner;
    }

    if #[cfg(feature = "component")] {
        pub mod component;

        pub(crate) use self::component::{ComponentInner, ComponentsInner};
    }

    if #[cfg(feature = "network")] {
        pub mod network;

        pub(crate) use self::network::{NetworkDataInner, NetworksInner};
    }

    if #[cfg(feature = "user")] {
        pub(crate) use crate::unix::groups::get_groups;
        pub(crate) use crate::unix::users::{get_users, UserInner};
    }
}

#[doc = include_str!("../../../md_doc/is_supported.md")]
pub const IS_SUPPORTED_SYSTEM: bool = true;
