use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(target_arch = "x86_64")] {
        #[allow(non_camel_case_types)]
        pub type boolean_t = libc::c_uint;
    } else {
        #[allow(non_camel_case_types)]
        pub type boolean_t = libc::c_int;
    }
}

cfg_if! {
    if #[cfg(target_pointer_width = "64")] {
        pub type CGFloat = libc::c_double;
    } else {
        pub type CGFloat = libc::c_float;
    }
}
