#![warn(
    clippy::alloc_instead_of_core,
    clippy::use_self,
    clippy::std_instead_of_alloc,
    clippy::std_instead_of_core,
    deprecated_in_future,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unused_qualifications
)]
#![allow(
    clippy::too_many_arguments,
    clippy::missing_safety_doc,
    clippy::upper_case_acronyms
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]

//! # Vulkan API
//!
//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/html/index.html>
//!
//! ## Examples
//!
//! ```no_run
//! use ash::{vk, Entry};
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let entry = Entry::linked();
//! let app_info = vk::ApplicationInfo {
//!     api_version: vk::make_api_version(0, 1, 0, 0),
//!     ..Default::default()
//! };
//! let create_info = vk::InstanceCreateInfo {
//!     p_application_info: &app_info,
//!     ..Default::default()
//! };
//! let instance = unsafe { entry.create_instance(&create_info, None)? };
//! # Ok(()) }
//! ```
//!
//! ## Getting started
//!
//! Load the Vulkan library linked at compile time using [`Entry::linked()`], or load it at runtime
//! using [`Entry::load()`], which uses `libloading`. If you want to perform entry point loading
//! yourself, call [`Entry::from_static_fn()`].
//!
//! ## Crate features
//!
//! * **debug** (default): Whether Vulkan structs should implement `Debug`.
//! * **loaded** (default): Support searching for the Vulkan loader manually at runtime.
//! * **linked**: Link the Vulkan loader at compile time.
//! * **std** (default): Whether ash depends on the standard library (otherwise `alloc` is required)

extern crate alloc;

pub use crate::device::Device;
pub use crate::entry::Entry;
#[cfg(feature = "loaded")]
pub use crate::entry::LoadingError;
pub use crate::extensions_generated::*;
pub use crate::instance::Instance;
pub use crate::tables::*;

mod device;
mod entry;
mod extensions_generated;
mod instance;
pub mod prelude;
mod tables;
pub mod util;
/// Raw Vulkan bindings and types, generated from `vk.xml`
#[macro_use]
pub mod vk;

// macros of vk need to be defined beforehand
/// Hand-written ergonomic wrappers for extension functions
mod extensions;

pub trait RawPtr<T> {
    fn as_raw_ptr(&self) -> *const T;
}

impl<'r, T> RawPtr<T> for Option<&'r T> {
    fn as_raw_ptr(&self) -> *const T {
        match *self {
            Some(inner) => inner,
            _ => ::core::ptr::null(),
        }
    }
}

/// Given a mutable raw pointer to a type with an `s_type` member such as [`vk::BaseOutStructure`],
/// match on a set of Vulkan structures. The struct will be rebound to the given variable of the
/// type of the given Vulkan structure.
///
/// Note that all match bodies have to be enclosed by curly braces due to macro parsing limitations.
/// It is unfortunately not possible to write `x @ ash::vk::SomeStruct => one_line_expression(),`.
///
/// ```
/// let mut info = ash::vk::DeviceCreateInfo::default();
/// let info: *mut ash::vk::BaseOutStructure = <*mut _>::cast(&mut info);
/// unsafe {
///     ash::match_out_struct!(match info {
///         info @ ash::vk::DeviceQueueCreateInfo => {
///             dbg!(&info); // Unreachable
///         }
///         info @ ash::vk::DeviceCreateInfo => {
///             dbg!(&info);
///         }
///     })
/// }
/// ```
///
/// In addition this macro propagates implicit return values just like normal `match` blocks, as
/// long as a default value or expression is provided in the "any" match arm
/// (`_ => { some_value() }`). For the time being said arm must be wrapped in curly braces; an
/// expression like `_ => None` is not yet supported.
///
/// ```
/// # let mut info = ash::vk::DeviceCreateInfo::default();
/// # let info: *mut ash::vk::BaseOutStructure = <*mut _>::cast(&mut info);
/// let device_create_flags: Option<ash::vk::DeviceCreateFlags> = unsafe {
///     ash::match_out_struct!(match info {
///         info @ ash::vk::DeviceQueueCreateInfo => {
///             dbg!(&info); // Unreachable
///             Some(ash::vk::DeviceCreateFlags::empty())
///         }
///         info @ ash::vk::DeviceCreateInfo => {
///             dbg!(&info);
///             Some(info.flags)
///         }
///         _ => {
///             None
///         }
///     })
/// };
/// ```
#[macro_export]
macro_rules! match_out_struct {
    (match $p:ident { $($bind:ident @ $ty:path => $body:block $(,)?)+ $(_ => $any:block $(,)?)? }) => {
        match core::ptr::addr_of!((*$p).s_type).read() {
            $(<$ty as $crate::vk::TaggedStructure>::STRUCTURE_TYPE => {
                let $bind = $p
                    .cast::<$ty>()
                    .as_mut()
                    .unwrap();
                $body
            }),+
            _ => { $($any)? }
        }
    };
}

/// Given an immutable raw pointer to a type with an `s_type` member such as [`vk::BaseInStructure`],
/// match on a set of Vulkan structures. The struct will be rebound to the given variable of the
/// type of the given Vulkan structure.
///
/// Note that all match bodies have to be enclosed by curly braces due to macro parsing limitations.
/// It is unfortunately not possible to write `x @ ash::vk::SomeStruct => one_line_expression(),`.
///
/// ```
/// let info = ash::vk::DeviceCreateInfo::default();
/// let info: *const ash::vk::BaseInStructure = <*const _>::cast(&info);
/// unsafe {
///     ash::match_in_struct!(match info {
///         info @ ash::vk::DeviceQueueCreateInfo => {
///             dbg!(&info); // Unreachable
///         }
///         info @ ash::vk::DeviceCreateInfo => {
///             dbg!(&info);
///         }
///     })
/// }
/// ```
///
/// See the [`match_out_struct!`] documentation for an example with implicit return values.
#[macro_export]
macro_rules! match_in_struct {
    (match $p:ident { $($bind:ident @ $ty:path => $body:block $(,)?)+ $(_ => $any:block $(,)?)? }) => {
        match core::ptr::addr_of!((*$p).s_type).read() {
            $(<$ty as $crate::vk::TaggedStructure>::STRUCTURE_TYPE => {
                let $bind = $p
                    .cast::<$ty>()
                    .as_ref()
                    .unwrap();
                $body
            }),+
            _ => { $($any)? }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::vk;
    use alloc::vec::Vec;
    #[test]
    fn test_ptr_chains() {
        let mut variable_pointers = vk::PhysicalDeviceVariablePointerFeatures::default();
        let mut corner = vk::PhysicalDeviceCornerSampledImageFeaturesNV::default();
        let chain = alloc::vec![
            <*mut _>::cast(&mut variable_pointers),
            <*mut _>::cast(&mut corner),
        ];
        let mut device_create_info = vk::DeviceCreateInfo::default()
            .push_next(&mut corner)
            .push_next(&mut variable_pointers);
        let chain2: Vec<*mut vk::BaseOutStructure<'_>> = unsafe {
            vk::ptr_chain_iter(&mut device_create_info)
                .skip(1)
                .collect()
        };
        assert_eq!(chain, chain2);
    }
}
