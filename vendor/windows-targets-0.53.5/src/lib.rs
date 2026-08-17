#![doc = include_str!("../readme.md")]
#![no_std]

#[cfg(windows_raw_dylib)]
pub use windows_link::link;

/// Defines an external function to import.
#[cfg(all(windows, not(windows_raw_dylib)))]
#[macro_export]
macro_rules! link {
    ($library:literal $abi:literal $($link_name:literal)? fn $($function:tt)*) => (
        #[link(name = "windows.0.53.0")]
        extern $abi {
            $(#[link_name=$link_name])?
            pub fn $($function)*;
        }
    )
}

/// Defines an external function to import.
#[cfg(all(not(windows), not(windows_raw_dylib)))]
#[macro_export]
macro_rules! link {
    ($library:literal $abi:literal $($link_name:literal)? fn $($function:tt)*) => (
        extern $abi {
            pub fn $($function)*;
        }
    )
}
