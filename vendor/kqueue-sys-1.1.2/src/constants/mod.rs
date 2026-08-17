#[cfg(any(target_os = "macos", target_os = "ios"))]
mod darwin;
#[cfg(any(target_os = "macos", target_os = "ios"))]
pub use self::darwin::*;

#[cfg(target_os = "dragonfly")]
mod dragonfly;
#[cfg(target_os = "dragonfly")]
pub use self::dragonfly::*;

#[cfg(target_os = "freebsd")]
mod freebsd;
#[cfg(target_os = "freebsd")]
pub use self::freebsd::*;

#[cfg(target_os = "netbsd")]
mod netbsd;
#[cfg(target_os = "netbsd")]
pub use self::netbsd::*;

#[cfg(target_os = "openbsd")]
mod openbsd;
#[cfg(target_os = "openbsd")]
pub use self::openbsd::*;
