#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/meilisearch/heed/main/assets/heed-pigeon.ico?raw=true"
)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/meilisearch/heed/main/assets/heed-pigeon-logo.png?raw=true"
)]

//! Types that can be used to serialize and deserialize types inside databases.

#![warn(missing_docs)]

mod bytes;
mod decode_ignore;
mod integer;
mod lazy_decode;
mod str;
mod unit;

#[cfg(feature = "serde-bincode")]
mod serde_bincode;

#[cfg(feature = "serde-json")]
mod serde_json;

#[cfg(feature = "serde-rmp")]
mod serde_rmp;

pub use self::bytes::Bytes;
pub use self::decode_ignore::DecodeIgnore;
pub use self::integer::*;
pub use self::lazy_decode::{Lazy, LazyDecode};
#[cfg(feature = "serde-bincode")]
pub use self::serde_bincode::SerdeBincode;
#[cfg(feature = "serde-json")]
pub use self::serde_json::SerdeJson;
#[cfg(feature = "serde-rmp")]
pub use self::serde_rmp::SerdeRmp;
pub use self::str::Str;
pub use self::unit::Unit;
