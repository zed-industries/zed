#[cfg(not(target_os = "freebsd"))]
mod crashes;

#[cfg(not(target_os = "freebsd"))]
pub use crashes::*;

#[cfg(target_os = "freebsd")]
mod crashes_freebsd;

#[cfg(target_os = "freebsd")]
pub use crashes_freebsd::*;
