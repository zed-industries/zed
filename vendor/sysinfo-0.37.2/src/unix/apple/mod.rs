// Take a look at the license at the top of the repository in the LICENSE file.

#[cfg(any(target_os = "ios", feature = "apple-sandbox"))]
pub(crate) mod app_store;

mod ffi;
mod utils;

cfg_if! {
    if #[cfg(all(target_os = "macos", any(feature = "disk", feature = "system", feature = "component")))] {
        pub(crate) mod macos;
        pub(crate) use self::macos as inner;
    } else if #[cfg(all(target_os = "ios", any(feature = "system", feature = "component")))] {
        pub(crate) mod ios;
        pub(crate) use self::ios as inner;
    }
}

cfg_if! {
    if #[cfg(feature = "system")] {
        pub mod cpu;
        pub mod motherboard;
        pub mod process;
        pub mod product;
        pub mod system;

        pub(crate) use self::cpu::CpuInner;
        pub(crate) use self::motherboard::MotherboardInner;
        pub(crate) use self::process::ProcessInner;
        pub(crate) use self::product::ProductInner;
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
        pub mod users;

        pub(crate) use crate::unix::groups::get_groups;
        pub(crate) use crate::unix::users::{get_users, UserInner};
    }
}

#[doc = include_str!("../../../md_doc/is_supported.md")]
pub const IS_SUPPORTED_SYSTEM: bool = true;

// Make formattable by rustfmt.
#[cfg(any())]
mod component;
#[cfg(any())]
mod cpu;
#[cfg(any())]
mod disk;
#[cfg(any())]
mod ios;
#[cfg(any())]
mod macos;
#[cfg(any())]
mod motherboard;
#[cfg(any())]
mod network;
#[cfg(any())]
mod process;
#[cfg(any())]
mod product;
#[cfg(any())]
mod system;
#[cfg(any())]
mod users;
