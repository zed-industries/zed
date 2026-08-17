use std::env;
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

/// Probe for SSL certificates on the system, then configure the SSL certificate `SSL_CERT_FILE`
/// and `SSL_CERT_DIR` environment variables in this process for OpenSSL to use.
///
/// Preconfigured values in the environment variables will not be overwritten if the paths they
/// point to exist and are accessible.
///
/// Returns `true` if any certificate file or directory was found while probing.
/// Combine this with `has_ssl_cert_env_vars()` to check whether previously configured environment
/// variables are valid.
///
/// # Safety
///
/// This function is not safe because it mutates the process's environment
/// variables which is generally not safe. See the [documentation in libstd][doc]
/// for information about why setting environment variables is not safe.
///
/// If possible use the [`probe`] function and directly configure OpenSSL
/// methods instead of relying on environment variables.
///
/// [doc]: https://doc.rust-lang.org/stable/std/env/fn.set_var.html#safety
pub unsafe fn try_init_openssl_env_vars() -> bool {
    let ProbeResult {
        cert_file,
        cert_dir,
    } = probe();
    // we won't be overwriting existing env variables because if they're valid probe() will have
    // returned them unchanged
    if let Some(path) = &cert_file {
        unsafe {
            put(ENV_CERT_FILE, path.as_os_str());
        }
    }

    if !cert_dir.is_empty() {
        let mut joined = OsString::new();
        for (i, path) in cert_dir.iter().enumerate() {
            if i != 0 {
                joined.push(":");
            }
            joined.push(path.as_os_str());
        }

        unsafe {
            put(ENV_CERT_DIR, &joined);
        }
    }

    unsafe fn put(var: &str, path: &OsStr) {
        // Avoid calling `setenv` if the variable already has the same contents. This avoids a
        // crash when called from out of perl <5.38 (Debian Bookworm is at 5.36), as old versions
        // of perl tend to manipulate the `environ` pointer directly.
        if env::var_os(var).as_deref() != Some(path) {
            unsafe {
                env::set_var(var, path);
            }
        }
    }

    cert_file.is_some() || !cert_dir.is_empty()
}

/// Probe the current system for the "cert file" and "cert dir" variables that
/// OpenSSL typically requires.
///
/// The probe result is returned as a [`ProbeResult`] structure here.
pub fn probe() -> ProbeResult {
    let mut result = ProbeResult::from_env();
    if result.cert_file.is_none() {
        result.cert_file =
            CERTIFICATE_FILE_NAMES
                .iter()
                .find_map(|p| match Path::new(p).exists() {
                    true => Some(PathBuf::from(p)),
                    false => None,
                });
    }

    for certs_dir in candidate_cert_dirs() {
        let cert_dir = PathBuf::from(certs_dir);
        if cert_dir.exists() {
            result.cert_dir.push(cert_dir);
        }
    }

    result
}

/// Probe the system for the directory in which CA certificates should likely be
/// found.
///
/// This will only search known system locations.
pub fn candidate_cert_dirs() -> impl Iterator<Item = &'static Path> {
    CERTIFICATE_DIRS
        .iter()
        .map(Path::new)
        .filter(|p| p.exists())
}

/// Check whether the OpenSSL `SSL_CERT_FILE` and/or `SSL_CERT_DIR` environment variable is
/// configured in this process with an existing file or directory.
///
/// That being the case would indicate that certificates will be found successfully by OpenSSL.
///
/// Returns `true` if either variable is set to an existing file or directory.
pub fn has_ssl_cert_env_vars() -> bool {
    let probe = ProbeResult::from_env();
    probe.cert_file.is_some() || !probe.cert_dir.is_empty()
}

pub struct ProbeResult {
    pub cert_file: Option<PathBuf>,
    pub cert_dir: Vec<PathBuf>,
}

impl ProbeResult {
    fn from_env() -> ProbeResult {
        let var = |name| env::var_os(name).map(PathBuf::from).filter(|p| p.exists());
        ProbeResult {
            cert_file: var(ENV_CERT_FILE),
            cert_dir: match var(ENV_CERT_DIR) {
                Some(p) => vec![p],
                None => vec![],
            },
        }
    }
}

// see http://gagravarr.org/writing/openssl-certs/others.shtml
// Go's related definitions can be found here:
// https://github.com/golang/go/tree/master/src/crypto/x509
// Look at `root_*.go` files for platform-specific files and directories.

#[cfg(target_os = "linux")]
const CERTIFICATE_DIRS: &[&str] = &[
    "/etc/ssl/certs",             // SLES 10, SLES 11
    "/etc/pki/tls/certs",         // Fedora, RHEL
    "/etc/security/certificates", // OpenHarmony, https://developer.huawei.com/consumer/en/doc/best-practices/bpta-network-ca-security#section121091116142117
];

#[cfg(target_os = "freebsd")]
const CERTIFICATE_DIRS: &[&str] = &[
    "/etc/ssl/certs",         // FreeBSD 12.2+,
    "/usr/local/share/certs", // FreeBSD
];

#[cfg(any(target_os = "illumos", target_os = "solaris"))]
const CERTIFICATE_DIRS: &[&str] = &["/etc/certs/CA"];

#[cfg(target_os = "netbsd")]
const CERTIFICATE_DIRS: &[&str] = &["/etc/openssl/certs"];

#[cfg(target_os = "aix")]
const CERTIFICATE_DIRS: &[&str] = &["/var/ssl/certs"];

#[cfg(not(any(
    target_os = "linux",
    target_os = "freebsd",
    target_os = "illumos",
    target_os = "solaris",
    target_os = "netbsd",
    target_os = "aix"
)))]
const CERTIFICATE_DIRS: &[&str] = &["/etc/ssl/certs"];

#[cfg(target_os = "linux")]
const CERTIFICATE_FILE_NAMES: &[&str] = &[
    "/etc/ssl/certs/ca-certificates.crt", // Debian, Ubuntu, Gentoo, Joyent SmartOS, etc.
    "/etc/pki/ca-trust/extracted/pem/tls-ca-bundle.pem", // CentOS, RHEL 7 (should come before RHEL 6)
    "/etc/pki/tls/certs/ca-bundle.crt",                  // Fedora, RHEL 6
    "/etc/ssl/ca-bundle.pem",                            // OpenSUSE
    "/etc/pki/tls/cacert.pem",                           // OpenELEC (a media center Linux distro)
    "/etc/ssl/cert.pem",                                 // Alpine Linux
    "/opt/etc/ssl/certs/ca-certificates.crt", // Entware, https://github.com/rustls/openssl-probe/pull/21
    "/etc/ssl/certs/cacert.pem", // OpenHarmony, https://developer.huawei.com/consumer/en/doc/harmonyos-faqs/faqs-network-41
];

#[cfg(target_os = "freebsd")]
const CERTIFICATE_FILE_NAMES: &[&str] = &["/usr/local/etc/ssl/cert.pem"];

#[cfg(target_os = "dragonfly")]
const CERTIFICATE_FILE_NAMES: &[&str] = &["/usr/local/share/certs/ca-root-nss.crt"];

#[cfg(target_os = "netbsd")]
const CERTIFICATE_FILE_NAMES: &[&str] = &["/etc/openssl/certs/ca-certificates.crt"];

#[cfg(target_os = "openbsd")]
const CERTIFICATE_FILE_NAMES: &[&str] = &["/etc/ssl/cert.pem"];

#[cfg(target_os = "solaris")] // Solaris 11.2+
const CERTIFICATE_FILE_NAMES: &[&str] = &["/etc/certs/ca-certificates.crt"];

#[cfg(target_os = "illumos")]
const CERTIFICATE_FILE_NAMES: &[&str] = &[
    "/etc/ssl/cacert.pem", // OmniOS, https://github.com/rustls/openssl-probe/pull/22
    "/etc/certs/ca-certificates.crt", // OpenIndiana, https://github.com/rustls/openssl-probe/pull/22
];

#[cfg(target_os = "android")] // Termux on Android, https://github.com/rustls/openssl-probe/pull/2
const CERTIFICATE_FILE_NAMES: &[&str] = &["/data/data/com.termux/files/usr/etc/tls/cert.pem"];

#[cfg(target_os = "haiku")] // https://github.com/rustls/openssl-probe/pull/4
const CERTIFICATE_FILE_NAMES: &[&str] = &["/boot/system/data/ssl/CARootCertificates.pem"];

#[cfg(not(any(
    target_os = "linux",
    target_os = "freebsd",
    target_os = "dragonfly",
    target_os = "netbsd",
    target_os = "openbsd",
    target_os = "solaris",
    target_os = "illumos",
    target_os = "android",
    target_os = "haiku",
)))]
const CERTIFICATE_FILE_NAMES: &[&str] = &["/etc/ssl/certs/ca-certificates.crt"];

/// The OpenSSL environment variable to configure what certificate file to use.
pub const ENV_CERT_FILE: &str = "SSL_CERT_FILE";

/// The OpenSSL environment variable to configure what certificates directory to use.
pub const ENV_CERT_DIR: &str = "SSL_CERT_DIR";
