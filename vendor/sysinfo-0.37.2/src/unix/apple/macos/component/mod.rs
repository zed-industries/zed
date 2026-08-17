// Take a look at the license at the top of the repository in the LICENSE file.

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub(crate) mod x86;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub(crate) use self::x86::*;

#[cfg(target_arch = "aarch64")]
pub(crate) mod arm;

#[cfg(target_arch = "aarch64")]
pub(crate) use self::arm::*;
