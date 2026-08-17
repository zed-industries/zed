//! OS-specific extensions.

cfg_unix! {
    pub mod unix;
}

cfg_windows! {
    pub mod windows;
}
