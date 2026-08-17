//! Generate a macro to add the various `used` and other attributes to a static.
#![allow(unknown_lints, edition_2024_expr_fragment_specifier)]

use crate::{__chain, __perform};

__perform!((all=() export=()), __chain[
    __add_used,
    __add_asan,
    __add_export_name[$],
    __generate_macro[$],
]);

#[cfg(linktime_used_linker)]
macro_rules! __add_used {
    (@entry next=$next:path[$next_args:tt], input=(all=($($input:tt)*) export=$export:tt)) => {
        $next!($next_args, (all=($($input)* #[used(linker)]) export=$export));
    };
}

#[cfg(not(linktime_used_linker))]
macro_rules! __add_used {
    (@entry next=$next:path[$next_args:tt], input=(all=($($input:tt)*) export=$export:tt)) => {
        $next!($next_args, (all=($($input)* #[used]) export=$export));
    };
}

#[cfg(linktime_asan)]
macro_rules! __add_asan {
    (@entry next=$next:path[$next_args:tt], input=(all=($($input:tt)*) export=$export:tt)) => {
        $next!($next_args, (all=($($input)* #[sanitize(address = "off")]) export=$export));
    };
}

#[cfg(not(linktime_asan))]
macro_rules! __add_asan {
    (@entry next=$next:path[$next_args:tt], input=$input:tt) => {
        $next!($next_args, $input);
    };
}

#[cfg(target_os = "aix")]
macro_rules! __add_export_name {
    (@entry next=$next:path[$next_args:tt], input=(all=$all:tt export=($($input:tt)*)), args=[$dollar:tt]) => {
        $next!($next_args, (all=$all export=((
            "_P", env!("CARGO_PKG_NAME"),
            "_M", ::core::module_path!(),
            "_L", line!(),
            "_C", column!()
        ))));
    };
}

#[cfg(not(target_os = "aix"))]
macro_rules! __add_export_name {
    (@entry next=$next:path[$next_args:tt], input=$input:tt, args=[$dollar:tt]) => {
        $next!($next_args, $input);
    };
}

macro_rules! __generate_macro {
    (@entry next=$next:path[$next_args:tt], input=(all=($($all:tt)*) export=($(($($export:tt)*))?)), args=[$dollar:tt]) => {
        #[doc(hidden)]
        #[macro_export]
        macro_rules! __add_linktime_attributes_to_static {
            (
                #[link_section = $link_section:expr]
                $dollar (#[$meta:meta])* $vis:vis static $ident:ident : $dollar ($static:tt)*
            ) => {
                $($all)*
                $(#[export_name = concat! (stringify!($ident), $($export)* )])?
                $dollar (#[$meta])*
                #[link_section = $link_section]
                $vis static $ident : $dollar ($static)*
            };

            (
                #[export_name = $export_name:expr]
                $dollar (#[$meta:meta])* $vis:vis static $ident:ident : $dollar ($static:tt)*
            ) => {
                #[export_name = $export_name]
                $($all)*
                $dollar (#[$meta])*
                $vis static $ident : $dollar ($static)*
            };

            (
                extern "C" {
                    #[link_name = $link_name:expr]
                    $dollar (#[$meta:meta])* $vis:vis static $ident:ident : $ty:ty;
                }
            ) => {
                #[allow(missing_unsafe_on_extern)] // MSRV
                extern "C" {
                    #[link_name = $link_name]
                    $dollar (#[$meta])* $vis static $ident : $ty;
                }
            };

            (
                $dollar ($item:tt)*
            ) => {
                $dollar ($item)*
            };
        }
    };
}

pub(crate) use __add_asan;
pub(crate) use __add_export_name;
pub(crate) use __add_used;
pub(crate) use __generate_macro;
