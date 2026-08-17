// Take a look at the license at the top of the repository in the LICENSE file.

#[cfg(feature = "component")]
pub(crate) mod component;
#[cfg(feature = "disk")]
pub(crate) mod disk;
#[cfg(feature = "network")]
pub(crate) mod network;
#[cfg(feature = "system")]
pub(crate) mod system;
#[cfg(feature = "user")]
pub(crate) mod user;

macro_rules! xid {
    ($(#[$outer:meta])+ $name:ident, $type:ty $(, $trait:ty)?) => {
        #[cfg(any(feature = "system", feature = "user"))]
        $(#[$outer])+
        #[repr(transparent)]
        #[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
        pub struct $name(pub(crate) $type);

        #[cfg(any(feature = "system", feature = "user"))]
        impl std::ops::Deref for $name {
            type Target = $type;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        $(
        #[cfg(any(feature = "system", feature = "user"))]
        impl TryFrom<usize> for $name {
            type Error = <$type as TryFrom<usize>>::Error;

            fn try_from(t: usize) -> Result<Self, <$type as TryFrom<usize>>::Error> {
                Ok(Self(<$type>::try_from(t)?))
            }
        }

        #[cfg(any(feature = "system", feature = "user"))]
        impl $trait for $name {
            type Err = <$type as $trait>::Err;

            fn from_str(t: &str) -> Result<Self, <$type as $trait>::Err> {
                Ok(Self(<$type>::from_str(t)?))
            }
        }
        )?
    };
}

macro_rules! uid {
    ($type:ty$(, $trait:ty)?) => {
        xid!(
            /// A user id wrapping a platform specific type.
            Uid,
            $type
            $(, $trait)?
        );
    };
}

macro_rules! gid {
    ($type:ty) => {
        xid!(
            /// A group id wrapping a platform specific type.
            #[derive(Copy)]
            Gid,
            $type,
            std::str::FromStr
        );
    };
}

cfg_if! {
    if #[cfg(all(
        not(feature = "unknown-ci"),
        any(
            target_os = "freebsd",
            target_os = "linux",
            target_os = "android",
            target_os = "macos",
            target_os = "ios",
        )
    ))] {
        uid!(libc::uid_t, std::str::FromStr);
        gid!(libc::gid_t);
    } else if #[cfg(windows)] {
        uid!(crate::windows::Sid);
        gid!(u32);
        // Manual implementation outside of the macro...
        #[cfg(any(feature = "system", feature = "user"))]
        impl std::str::FromStr for Uid {
            type Err = <crate::windows::Sid as std::str::FromStr>::Err;

            fn from_str(t: &str) -> Result<Self, Self::Err> {
                Ok(Self(t.parse()?))
            }
        }
    } else {
        uid!(u32, std::str::FromStr);
        gid!(u32);
    }
}
