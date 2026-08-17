//! Wrappers around the OSX Security Framework.
#![warn(missing_docs)]
#![allow(non_upper_case_globals)]
#![allow(clippy::manual_non_exhaustive)] // MSRV
#![allow(clippy::bad_bit_mask)] // bitflags

#[macro_use]
extern crate core_foundation;

use core_foundation_sys::base::OSStatus;
use security_framework_sys::base::errSecSuccess;

use crate::base::{Error, Result};
#[cfg(target_os = "macos")]
use crate::os::macos::access::SecAccess;
#[cfg(target_os = "macos")]
use crate::os::macos::keychain::SecKeychain;

#[cfg(test)]
macro_rules! p {
    ($e:expr) => {
        match $e {
            Ok(s) => s,
            Err(e) => panic!("{:?}", e),
        }
    };
}

#[cfg(all(not(feature = "OSX_10_13"), any(feature = "alpn", feature = "session-tickets")))]
#[macro_use]
mod dlsym;

pub mod access_control;
#[cfg(target_os = "macos")]
pub mod authorization;
pub mod base;
#[cfg(any(target_os = "macos", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
pub mod certificate;
pub mod cipher_suite;
#[cfg(any(target_os = "macos", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
pub mod identity;
#[cfg(any(target_os = "macos", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
pub mod import_export;
#[cfg(any(target_os = "macos", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
pub mod item;
#[cfg(any(target_os = "macos", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
pub mod key;
pub mod os;
pub mod passwords;
pub mod passwords_options;
#[cfg(any(target_os = "macos", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
pub mod policy;
pub mod random;
#[cfg(any(target_os = "macos", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
pub mod secure_transport;
#[cfg(any(target_os = "macos", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
pub mod trust;
#[cfg(target_os = "macos")]
pub mod trust_settings;

#[cfg(target_os = "macos")]
trait Pkcs12ImportOptionsInternals {
    fn keychain(&mut self, keychain: SecKeychain) -> &mut Self;
    fn access(&mut self, access: SecAccess) -> &mut Self;
}

#[cfg(target_os = "macos")]
trait ItemSearchOptionsInternals {
    fn keychains(&mut self, keychains: &[SecKeychain]) -> &mut Self;
}

trait AsInner {
    type Inner;
    fn as_inner(&self) -> Self::Inner;
}

#[inline(always)]
fn cvt(err: OSStatus) -> Result<()> {
    match err {
        errSecSuccess => Ok(()),
        err => Err(Error::from_code(err)),
    }
}

#[cfg(test)]
mod test {
    use crate::certificate::SecCertificate;

    pub fn certificate() -> SecCertificate {
        let certificate = include_bytes!("../test/server.der");
        p!(SecCertificate::from_der(certificate))
    }
}
