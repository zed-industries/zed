//! A [Secret Service](https://specifications.freedesktop.org/secret-service-spec/latest/index.html) implementation.
//!
//! That is usually done with
//! ```no_run
//! use oo7::dbus::Service;
//!
//! # async fn run() -> oo7::Result<()> {
//! let service = Service::new().await?;
//!
//! let mut attributes = std::collections::HashMap::new();
//! attributes.insert("type", "password");
//! attributes.insert("user_id", "some_other_identifier");
//!
//! let collection = service.default_collection().await?;
//! // Store a secret
//! collection
//!     .create_item("My App's secret", &attributes, "password", true, None)
//!     .await?;
//!
//! // Retrieve it later thanks to it attributes
//! let items = collection.search_items(&attributes).await?;
//! let item = items.first().unwrap();
//! assert_eq!(item.secret().await?, oo7::Secret::text("password"));
//!
//! #   Ok(())
//! # }
//! ```
//!
//! ## Timeout
//!
//! If a DBus method call takes longer than 30 seconds (for example, waiting for
//! user input on a prompt), the call will fail with a
//! `zbus::Error::InputOutput(std::io::Error(kind: ErrorKind::TimedOut))`.

/// Barebone DBus API of the Secret Service specifications.
///
/// The API is not supposed to be used by the applications in general unless
/// the wrapper API doesn't provide functionality you need.
#[cfg(feature = "unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
pub mod api;

#[cfg(not(feature = "unstable"))]
#[allow(unused)]
mod api;

mod algorithm;
#[cfg(not(feature = "unstable"))]
pub(crate) use algorithm::Algorithm;
#[cfg(feature = "unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
pub use algorithm::Algorithm;
mod item;
pub use item::Item;
mod error;
mod service;
pub use error::{Error, ServiceError};
pub use service::Service;
mod collection;
pub use collection::Collection;
