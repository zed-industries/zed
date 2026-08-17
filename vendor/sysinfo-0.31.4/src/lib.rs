// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg_attr(
    all(feature = "system", feature = "disk", feature = "component", feature = "system"),
    doc = include_str!("../README.md")
)]
#![cfg_attr(
    not(all(
        feature = "system",
        feature = "disk",
        feature = "component",
        feature = "system"
    )),
    doc = "For crate-level documentation, all features need to be enabled."
)]
#![cfg_attr(feature = "serde", doc = include_str!("../md_doc/serde.md"))]
#![allow(unknown_lints)]
#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::non_send_fields_in_send_ty)]
#![allow(renamed_and_removed_lints)]
#![allow(clippy::assertions_on_constants)]

#[macro_use]
mod macros;

cfg_if! {
    if #[cfg(feature = "unknown-ci")] {
        // This is used in CI to check that the build for unknown targets is compiling fine.
        mod unknown;
        use crate::unknown as sys;

        #[cfg(test)]
        pub(crate) const MIN_USERS: usize = 0;
    } else if #[cfg(any(
        target_os = "macos", target_os = "ios",
        target_os = "linux", target_os = "android",
        target_os = "freebsd"))]
    {
        mod unix;
        use crate::unix::sys as sys;

        #[cfg(feature = "network")]
        mod network;
        #[cfg(feature = "network")]
        use crate::unix::network_helper;

        #[cfg(test)]
        pub(crate) const MIN_USERS: usize = 1;
    } else if #[cfg(windows)] {
        mod windows;
        use crate::windows as sys;

        #[cfg(feature = "network")]
        mod network;
        #[cfg(feature = "network")]
        use crate::windows::network_helper;

        #[cfg(test)]
        pub(crate) const MIN_USERS: usize = 1;
    } else {
        mod unknown;
        use crate::unknown as sys;

        #[cfg(test)]
        pub(crate) const MIN_USERS: usize = 0;
    }
}

#[cfg(feature = "component")]
pub use crate::common::component::{Component, Components};
#[cfg(feature = "disk")]
pub use crate::common::disk::{Disk, DiskKind, Disks};
#[cfg(feature = "network")]
pub use crate::common::network::{IpNetwork, MacAddr, NetworkData, Networks};
#[cfg(feature = "system")]
pub use crate::common::system::{
    get_current_pid, CGroupLimits, Cpu, CpuRefreshKind, DiskUsage, LoadAvg, MemoryRefreshKind, Pid,
    Process, ProcessRefreshKind, ProcessStatus, ProcessesToUpdate, RefreshKind, Signal, System,
    ThreadKind, UpdateKind,
};
#[cfg(feature = "user")]
pub use crate::common::user::{Group, Groups, User, Users};
#[cfg(any(feature = "user", feature = "system"))]
pub use crate::common::{Gid, Uid};
#[cfg(feature = "system")]
pub use crate::sys::{MINIMUM_CPU_UPDATE_INTERVAL, SUPPORTED_SIGNALS};

#[cfg(feature = "user")]
pub(crate) use crate::common::user::GroupInner;
#[cfg(feature = "user")]
pub(crate) use crate::sys::UserInner;
#[cfg(feature = "component")]
pub(crate) use crate::sys::{ComponentInner, ComponentsInner};
#[cfg(feature = "system")]
pub(crate) use crate::sys::{CpuInner, ProcessInner, SystemInner};
#[cfg(feature = "disk")]
pub(crate) use crate::sys::{DiskInner, DisksInner};
#[cfg(feature = "network")]
pub(crate) use crate::sys::{NetworkDataInner, NetworksInner};

pub use crate::sys::IS_SUPPORTED_SYSTEM;

#[cfg(feature = "c-interface")]
pub use crate::c_interface::*;

#[cfg(feature = "c-interface")]
mod c_interface;
mod common;
mod debug;
#[cfg(feature = "serde")]
mod serde;
pub(crate) mod utils;

/// This function is only used on Linux targets, when the `system` feature is enabled. In other
/// cases, it does nothing and returns `false`.
///
/// On Linux, to improve performance, we keep a `/proc` file open for each process we index with
/// a maximum number of files open equivalent to half of the system limit.
///
/// The problem is that some users might need all the available file descriptors so we need to
/// allow them to change this limit.
///
/// Note that if you set a limit bigger than the system limit, the system limit will be set.
///
/// Returns `true` if the new value has been set.
///
#[cfg_attr(feature = "system", doc = "```no_run")]
#[cfg_attr(not(feature = "system"), doc = "```ignore")]
/// use sysinfo::{System, set_open_files_limit};
///
/// // We call the function before any call to the processes update.
/// if !set_open_files_limit(10) {
///     // It'll always return false on non-linux targets.
///     eprintln!("failed to update the open files limit...");
/// }
/// let s = System::new_all();
/// ```
pub fn set_open_files_limit(mut _new_limit: isize) -> bool {
    cfg_if! {
        if #[cfg(all(feature = "system", not(feature = "unknown-ci"), any(target_os = "linux", target_os = "android")))]
        {
            use crate::sys::system::remaining_files;
            use std::sync::atomic::Ordering;

            if _new_limit < 0 {
                _new_limit = 0;
            }
            let max = sys::system::get_max_nb_fds();
            if _new_limit > max {
                _new_limit = max;
            }

            // If files are already open, to be sure that the number won't be bigger when those
            // files are closed, we subtract the current number of opened files to the new
            // limit.
            remaining_files().fetch_update(Ordering::SeqCst, Ordering::SeqCst, |remaining| {
                let diff = max.saturating_sub(remaining);
                Some(_new_limit.saturating_sub(diff))
            }).unwrap();

            true
        } else {
            false
        }
    }
}

#[cfg(doctest)]
mod doctest {
    macro_rules! compile_fail_import {
        ($mod_name:ident => $($imports:ident),+ $(,)?) => {
            $(#[doc = concat!(r"```compile_fail
use sysinfo::", stringify!($imports), r";
```
")])+
            mod $mod_name {}
        };
    }

    #[cfg(not(feature = "system"))]
    compile_fail_import!(
        no_system_feature =>
        get_current_pid,
        CGroupLimits,
        Cpu,
        CpuRefreshKind,
        DiskUsage,
        LoadAvg,
        MemoryRefreshKind,
        Pid,
        Process,
        ProcessesToUpdate,
        ProcessRefreshKind,
        ProcessStatus,
        RefreshKind,
        Signal,
        System,
        ThreadKind,
        UpdateKind,
    );

    #[cfg(not(feature = "disk"))]
    compile_fail_import!(
        no_disk_feature =>
        Disk,
        Disks,
        DiskKind,
    );

    #[cfg(not(feature = "component"))]
    compile_fail_import!(
        no_component_feature =>
        Component,
        Components,
    );

    #[cfg(not(feature = "network"))]
    compile_fail_import!(
        no_network_feature =>
        IpNetwork,
        MacAddr,
        NetworkData,
        Networks,
    );

    #[cfg(not(feature = "user"))]
    compile_fail_import!(
        no_user_feature =>
        Group,
        Groups,
        User,
        Users,
    );
}

#[cfg(test)]
mod test {
    use crate::*;

    #[cfg(feature = "unknown-ci")]
    #[test]
    fn check_unknown_ci_feature() {
        assert!(!IS_SUPPORTED_SYSTEM);
    }

    // If this test doesn't compile, it means the current OS doesn't implement them correctly.
    #[test]
    fn check_macro_types() {
        fn check_is_supported(_: bool) {}

        check_is_supported(IS_SUPPORTED_SYSTEM);
    }

    // If this test doesn't compile, it means the current OS doesn't implement them correctly.
    #[cfg(feature = "system")]
    #[test]
    fn check_macro_types2() {
        fn check_supported_signals(_: &'static [Signal]) {}
        fn check_minimum_cpu_update_interval(_: std::time::Duration) {}

        check_supported_signals(SUPPORTED_SIGNALS);
        check_minimum_cpu_update_interval(MINIMUM_CPU_UPDATE_INTERVAL);
    }

    #[cfg(feature = "user")]
    #[test]
    fn check_uid_gid() {
        let mut users = Users::new();
        assert!(users.list().is_empty());
        users.refresh_list();
        let user_list = users.list();
        assert!(user_list.len() >= MIN_USERS);

        if IS_SUPPORTED_SYSTEM {
            #[cfg(not(target_os = "windows"))]
            {
                let user = user_list
                    .iter()
                    .find(|u| u.name() == "root")
                    .expect("no root user");
                assert_eq!(**user.id(), 0);
                assert_eq!(*user.group_id(), 0);
                if let Some(user) = users.iter().find(|u| *u.group_id() > 0) {
                    assert!(**user.id() > 0);
                    assert!(*user.group_id() > 0);
                }
                assert!(user_list.iter().filter(|u| **u.id() > 0).count() > 0);
            }

            #[cfg(feature = "system")]
            {
                // And now check that our `get_user_by_id` method works.
                let s = System::new_with_specifics(
                    RefreshKind::new()
                        .with_processes(ProcessRefreshKind::new().with_user(UpdateKind::Always)),
                );
                assert!(s
                    .processes()
                    .iter()
                    .filter_map(|(_, p)| p.user_id())
                    .any(|uid| users.get_user_by_id(uid).is_some()));
            }
        }
    }

    #[cfg(feature = "system")]
    #[test]
    fn check_all_process_uids_resolvable() {
        // On linux, some user IDs don't have an associated user (no idea why though).
        // If `getent` doesn't find them, we can assume it's a dark secret from the linux land.
        if IS_SUPPORTED_SYSTEM && cfg!(not(target_os = "linux")) {
            let s = System::new_with_specifics(
                RefreshKind::new()
                    .with_processes(ProcessRefreshKind::new().with_user(UpdateKind::Always)),
            );
            let users = Users::new_with_refreshed_list();

            // For every process where we can get a user ID, we should also be able
            // to find that user ID in the global user list
            for process in s.processes().values() {
                if let Some(uid) = process.user_id() {
                    assert!(
                        users.get_user_by_id(uid).is_some(),
                        "No UID {:?} found",
                        uid
                    );
                }
            }
        }
    }

    #[test]
    fn ensure_is_supported_is_set_correctly() {
        if MIN_USERS > 0 {
            assert!(IS_SUPPORTED_SYSTEM);
        } else {
            assert!(!IS_SUPPORTED_SYSTEM);
        }
    }
}
