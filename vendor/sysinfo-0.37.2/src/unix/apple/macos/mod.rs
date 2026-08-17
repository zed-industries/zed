// Take a look at the license at the top of the repository in the LICENSE file.

pub mod ffi;

cfg_if! {
    if #[cfg(all(feature = "system", not(feature = "apple-sandbox")))] {
        pub(crate) mod cpu;
        pub mod system;
        pub mod process;
    }
    if #[cfg(all(feature = "system", feature = "apple-sandbox"))] {
        pub use crate::sys::app_store::process;
    }


    if #[cfg(any(
            feature = "disk",
            all(
                not(feature = "apple-sandbox"),
                any(
                    feature = "system",
                    all(
                        feature = "component",
                        any(target_arch = "x86", target_arch = "x86_64")
                    )
                )
            ),
        ))]
    {
        pub(crate) mod utils;
    }

    if #[cfg(feature = "disk")] {
        pub mod disk;
    }

    if #[cfg(feature = "apple-sandbox")] {
        #[cfg(feature = "component")]
        pub use crate::sys::app_store::component;
    } else if #[cfg(feature = "component")] {
        pub mod component;
    }
}

// Make formattable by rustfmt.
#[cfg(any())]
mod component;
#[cfg(any())]
mod cpu;
#[cfg(any())]
mod disk;
#[cfg(any())]
mod process;
#[cfg(any())]
mod system;
#[cfg(any())]
mod utils;
