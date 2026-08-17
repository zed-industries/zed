/// Not yet public API.
//
// Note: While this is not public, it is still a breaking change to change
// the API for this, since framework crates rely on it.
#[doc(hidden)]
#[macro_export]
macro_rules! extern_category {
    (
        $(#[$m:meta])*
        $v:vis unsafe trait $name:ident {
            $($methods:tt)*
        }

        $(#[$impl_m:meta])*
        unsafe impl $category:ident for $ty:ty {}
    ) => {
        $(#[$m])*
        $v unsafe trait $name: ClassType {
            // TODO: Do this better
            $crate::__extern_protocol_rewrite_methods! {
                $($methods)*
            }

            #[doc(hidden)]
            const __UNSAFE_INNER: ();
        }

        $(#[$impl_m])*
        unsafe impl $category for $ty {
            const __UNSAFE_INNER: () = ();
        }
    };
}
