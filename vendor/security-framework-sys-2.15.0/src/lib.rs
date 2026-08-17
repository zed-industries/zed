#![allow(bad_style)]

#[cfg_attr(
    any(target_os = "macos", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"),
    link(name = "Security", kind = "framework")
)]
extern "C" {}

#[cfg(target_os = "macos")]
pub mod access;
pub mod access_control;
#[cfg(target_os = "macos")]
pub mod authorization;
pub mod base;
#[cfg(any(target_os = "macos", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
pub mod certificate;
#[cfg(target_os = "macos")]
pub mod certificate_oids;
pub mod cipher_suite;
#[cfg(target_os = "macos")]
pub mod cms;
#[cfg(target_os = "macos")]
pub mod code_signing;
#[cfg(target_os = "macos")]
pub mod digest_transform;
#[cfg(target_os = "macos")]
pub mod encrypt_transform;
pub mod identity;
pub mod import_export;
pub mod item;
pub mod key;
pub mod keychain;
pub mod keychain_item;
pub mod policy;
pub mod random;
pub mod secure_transport;
#[cfg(target_os = "macos")]
pub mod transform;
pub mod trust;
#[cfg(target_os = "macos")]
pub mod trust_settings;
