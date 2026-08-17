#[cfg(windows)]
include!("windows.rs");

mod bindings;
mod can_into;
mod com_bindings;
mod ref_count;
mod sha1;
mod weak_ref_count;

pub(crate) use bindings::*;
pub use can_into::*;
pub use com_bindings::*;
pub use ref_count::*;
pub use sha1::*;
pub use weak_ref_count::*;

#[doc(hidden)]
#[macro_export]
macro_rules! interface_hierarchy {
    ($child:ident, $parent:ty) => {
        impl ::windows_core::imp::CanInto<$parent> for $child {}
        impl ::core::convert::From<&$child> for &$parent {
            fn from(value: &$child) -> Self {
                unsafe { ::core::mem::transmute(value) }
            }
        }
        impl ::core::convert::From<$child> for $parent {
            fn from(value: $child) -> Self {
                unsafe { ::core::mem::transmute(value) }
            }
        }
    };
    ($child:ident, $first:ty, $($rest:ty),+) => {
        $crate::imp::interface_hierarchy!($child, $first);
        $crate::imp::interface_hierarchy!($child, $($rest),+);
    };
}

#[doc(hidden)]
pub use interface_hierarchy;

#[doc(hidden)]
#[macro_export]
macro_rules! required_hierarchy {
    ($child:ident, $parent:ty) => {
        impl ::windows_core::imp::CanInto<$parent> for $child { const QUERY: bool = true; }
    };
    ($child:ident, $first:ty, $($rest:ty),+) => {
        $crate::imp::required_hierarchy!($child, $first);
        $crate::imp::required_hierarchy!($child, $($rest),+);
    };
}

#[doc(hidden)]
pub use required_hierarchy;

#[doc(hidden)]
#[macro_export]
macro_rules! define_interface {
    ($name:ident, $vtbl:ident, $iid:literal) => {
        #[repr(transparent)]
        #[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::clone::Clone)]
        pub struct $name(::windows_core::IUnknown);
        unsafe impl ::windows_core::Interface for $name {
            type Vtable = $vtbl;
            const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128($iid);
        }
        impl ::core::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> core::fmt::Result {
                f.debug_tuple(stringify!($name))
                    .field(&::windows_core::Interface::as_raw(self))
                    .finish()
            }
        }
    };
    ($name:ident, $vtbl:ident) => {
        #[repr(transparent)]
        #[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::clone::Clone)]
        pub struct $name(::core::ptr::NonNull<::core::ffi::c_void>);
        unsafe impl ::windows_core::Interface for $name {
            type Vtable = $vtbl;
            const IID: ::windows_core::GUID = ::windows_core::GUID::zeroed();
            const UNKNOWN: bool = false;
        }
        impl ::core::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> core::fmt::Result {
                f.debug_tuple(stringify!($name)).field(&self.0).finish()
            }
        }
    };
}

#[doc(hidden)]
pub use define_interface;

#[doc(hidden)]
pub use alloc::boxed::Box;
