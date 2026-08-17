// Take a look at the license at the top of the repository in the LICENSE file.

cfg_if! {
    if #[cfg(any(target_os = "macos", target_os = "ios"))] {
        pub(crate) mod apple;
        pub(crate) use apple as sys;

        #[allow(unused_imports)]
        pub(crate) use libc::__error as libc_errno;
    } else if #[cfg(any(target_os = "linux", target_os = "android"))] {
        pub(crate) mod linux;
        pub(crate) use linux as sys;

        #[cfg(target_os = "linux")]
        #[allow(unused_imports)]
        pub(crate) use libc::__errno_location as libc_errno;
        #[cfg(target_os = "android")]
        #[allow(unused_imports)]
        pub(crate) use libc::__errno as libc_errno;
    } else if #[cfg(target_os = "freebsd")] {
        pub(crate) mod freebsd;
        pub(crate) use freebsd as sys;

        #[allow(unused_imports)]
        pub(crate) use libc::__error as libc_errno;
    } else {
        compile_error!("Invalid cfg!");
    }

    if #[cfg(feature = "disk")] {
        pub(crate) struct DisksInner {
            pub(crate) disks: Vec<crate::Disk>,
        }

        impl DisksInner {
            pub(crate) fn from_vec(disks: Vec<crate::Disk>) -> Self {
                Self { disks }
            }

            pub(crate) fn into_vec(self) -> Vec<crate::Disk> {
                self.disks
            }
        }
    }

    if #[cfg(feature = "network")] {
        pub(crate) mod network_helper;
    }

    if #[cfg(feature = "user")] {
        pub(crate) mod users;
        pub(crate) mod groups;
    }
}

pub(crate) mod utils;
