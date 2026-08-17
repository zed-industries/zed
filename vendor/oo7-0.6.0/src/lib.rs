#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(rustdoc::broken_intra_doc_links)]
#![doc = include_str!("../README.md")]
#[cfg(all(all(feature = "tokio", feature = "async-std"), not(doc)))]
compile_error!("You can't enable both async-std & tokio features at once");
#[cfg(all(not(feature = "tokio"), not(feature = "async-std"), not(doc)))]
compile_error!("You have to enable either tokio or async-std feature");
#[cfg(all(all(feature = "native_crypto", feature = "openssl_crypto"), not(doc)))]
compile_error!("You can't enable both openssl_crypto & native_crypto features at once");
#[cfg(all(
    not(feature = "native_crypto"),
    not(feature = "openssl_crypto"),
    not(doc)
))]
compile_error!("You have to enable either openssl_crypto or native_crypto feature");

use std::collections::HashMap;

mod error;
mod key;
mod mac;
mod migration;

#[cfg(feature = "unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
pub use key::Key;
#[cfg(not(feature = "unstable"))]
pub(crate) use key::Key;
pub use mac::Mac;

#[cfg(not(feature = "unstable"))]
mod crypto;
#[cfg(feature = "unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
pub mod crypto;
pub mod dbus;
pub mod file;

mod keyring;
mod secret;

pub use ashpd;
#[cfg(feature = "schema")]
#[cfg_attr(docsrs, doc(cfg(feature = "schema")))]
pub use error::SchemaError;
pub use error::{Error, Result};
pub use keyring::{Item, Keyring};
pub use migration::migrate;
#[cfg(feature = "schema")]
#[cfg_attr(docsrs, doc(cfg(feature = "schema")))]
pub use oo7_macros::SecretSchema;
pub use secret::{ContentType, Secret};
pub use zbus;

/// A schema attribute.
///
/// Currently the key, is not really used but would allow
/// to map a Rust struct of simple types to an item attributes with type check.
pub const XDG_SCHEMA_ATTRIBUTE: &str = "xdg:schema";

/// A content type attribute.
///
/// Defines the type of the secret stored in the item.
pub const CONTENT_TYPE_ATTRIBUTE: &str = "xdg:content-type";

/// An item/collection attributes.
pub trait AsAttributes {
    fn as_attributes(&self) -> HashMap<String, String>;

    fn hash(&self, key: &Key) -> Vec<(String, std::result::Result<Mac, crate::crypto::Error>)> {
        self.as_attributes()
            .into_iter()
            .map(|(k, v)| (k, crypto::compute_mac(v.as_bytes(), key)))
            .collect()
    }
}

macro_rules! impl_as_attributes {
    ($rust_type:ty) => {
        impl<K, V> AsAttributes for $rust_type
        where
            K: AsRef<str>,
            V: AsRef<str>,
        {
            fn as_attributes(&self) -> std::collections::HashMap<String, String> {
                self.iter()
                    .map(|(k, v)| (k.as_ref().to_string(), v.as_ref().to_string()))
                    .collect()
            }
        }

        impl<K, V> AsAttributes for &$rust_type
        where
            K: AsRef<str>,
            V: AsRef<str>,
        {
            fn as_attributes(&self) -> std::collections::HashMap<String, String> {
                self.iter()
                    .map(|(k, v)| (k.as_ref().to_string(), v.as_ref().to_string()))
                    .collect()
            }
        }
    };
}

impl_as_attributes!([(K, V)]);
impl_as_attributes!(HashMap<K, V>);
impl_as_attributes!(std::collections::BTreeMap<K, V>);
impl_as_attributes!(Vec<(K, V)>);

impl<K, V, const N: usize> AsAttributes for [(K, V); N]
where
    K: AsRef<str>,
    V: AsRef<str>,
{
    fn as_attributes(&self) -> HashMap<String, String> {
        self.iter()
            .map(|(k, v)| (k.as_ref().to_string(), v.as_ref().to_string()))
            .collect()
    }
}

// Implementation for references to arrays
impl<K, V, const N: usize> AsAttributes for &[(K, V); N]
where
    K: AsRef<str>,
    V: AsRef<str>,
{
    fn as_attributes(&self) -> HashMap<String, String> {
        self.iter()
            .map(|(k, v)| (k.as_ref().to_string(), v.as_ref().to_string()))
            .collect()
    }
}
