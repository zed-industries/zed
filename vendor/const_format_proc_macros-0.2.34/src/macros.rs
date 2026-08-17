#![allow(unused_macros)]

#[doc(hidden)]
macro_rules! to_stream {
    ( $stream:ident ; $($expr:expr),* $(,)* ) => {{
        // use quote::TokenStreamExt;

        $( $expr.to_tokens($stream); )*
    }}
}

#[doc(hidden)]
macro_rules! spanned_err {
    ( $e:expr, $($fmt:tt)* ) => ({
        $crate::utils::spanned_err(
            &$e,
            &format!($($fmt)*),
        )
    })
}

#[doc(hidden)]
macro_rules! return_spanned_err {
    ( $e:expr, $($fmt:tt)* ) => ({
        return Err($crate::utils::spanned_err(
            &$e,
            &format!($($fmt)*),
        ))
    })
}

#[doc(hidden)]
macro_rules! syn_err {
    ( $span:expr, $($fmt:tt)* ) => ({
        $crate::utils::syn_err(
            $span,
            &format!($($fmt)*),
        )
    })
}

#[doc(hidden)]
macro_rules! return_syn_err {
    ( $span:expr, $($fmt:tt)* ) => ({
        return Err($crate::utils::syn_err(
            $span,
            &format!($($fmt)*),
        ))
    })
}
