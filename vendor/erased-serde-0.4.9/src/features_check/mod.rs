//! Shows a user-friendly compiler error on incompatible selected features.

#[allow(unused_macros)]
macro_rules! hide_from_rustfmt {
    ($mod:item) => {
        $mod
    };
}

#[cfg(not(feature = "alloc"))]
hide_from_rustfmt! {
    mod error;
}
