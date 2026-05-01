#[cfg(not(target_os = "freebsd"))]
mod crashes_full;

#[cfg(not(target_os = "freebsd"))]
pub use crashes_full::*;

#[cfg(target_os = "freebsd")]
mod crashes_freebsd;

#[cfg(target_os = "freebsd")]
pub use crashes_freebsd::*;
