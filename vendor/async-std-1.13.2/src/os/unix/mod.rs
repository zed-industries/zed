//! Platform-specific extensions for Unix platforms.

cfg_std! {
    pub mod io;
}

cfg_default! {
    pub mod fs;
    pub mod net;
}

#[cfg(all(feature = "unstable", feature = "std"))]
#[cfg_attr(feature = "docs", doc(cfg(unstable)))]
#[doc(inline)]
pub use async_process::unix as process;
