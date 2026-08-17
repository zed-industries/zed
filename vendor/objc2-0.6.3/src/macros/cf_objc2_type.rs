/// Helper macro for implementing [`objc2`][crate] traits for
/// CoreFoundation-like types.
#[doc(hidden)] // For now, though still a breaking change to modify
#[macro_export]
macro_rules! cf_objc2_type {
    (unsafe impl $(<$($generic:ident : ?$sized:ident),* $(,)?>)? RefEncode<$encoding_name:literal> for $ty:ty {}) => {
        // SAFETY: Caller upholds that the struct is a ZST type, and
        // represents a C struct with the given encoding.
        unsafe impl $(<$($generic : ?$sized),*>)? $crate::encode::RefEncode for $ty {
            const ENCODING_REF: $crate::encode::Encoding = $crate::encode::Encoding::Pointer(
                &$crate::encode::Encoding::Struct($encoding_name, &[]),
            );
        }

        // SAFETY: CF types are message-able in the Objective-C runtime.
        //
        // (Yes, even e.g. `CFArray<u32>`, though the return type from methods
        // might not be what's expected).
        unsafe impl $(<$($generic : ?$sized),*>)? $crate::Message for $ty {}

        // Allow converting to AnyObject.
        // Similar to __extern_class_impl_as_ref_borrow!
        impl $(<$($generic : ?$sized),*>)? $crate::__macro_helpers::AsRef<$crate::runtime::AnyObject> for $ty {
            #[inline]
            fn as_ref(&self) -> &$crate::runtime::AnyObject {
                // SAFETY: CF types are valid to re-interpret as AnyObject.
                unsafe { $crate::__macro_helpers::transmute(self) }
            }
        }

        impl $(<$($generic : ?$sized),*>)? $crate::__macro_helpers::Borrow<$crate::runtime::AnyObject> for $ty {
            #[inline]
            fn borrow(&self) -> &$crate::runtime::AnyObject {
                <Self as $crate::__macro_helpers::AsRef<$crate::runtime::AnyObject>>::as_ref(self)
            }
        }

        // Do not implement `ClassType`, CoreFoundation objects are root
        // objects, and all inherit from the same (hidden) __NSCFType class.
        //
        // This also means that casting etc. must be implemented differently
        // for CoreFoundation objects (compare).

        // NOTE: Make sure to keep objc2-core-foundation/src/base.rs up to
        // date with changes in here.
    };
}
