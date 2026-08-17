use core::fmt;

use fluent_uri::Uri;
use serde_json::Value;

#[cfg(target_family = "wasm")]
pub trait RetrieverBounds {}

#[cfg(target_family = "wasm")]
impl<T: ?Sized> RetrieverBounds for T {}

#[cfg(not(target_family = "wasm"))]
pub trait RetrieverBounds: Send + Sync {}

#[cfg(not(target_family = "wasm"))]
impl<T: ?Sized + Send + Sync> RetrieverBounds for T {}

/// Trait for retrieving resources from external sources.
///
/// Implementors of this trait can be used to fetch resources that are not
/// initially present in a [`crate::Registry`].
pub trait Retrieve: RetrieverBounds {
    /// Attempt to retrieve a resource from the given URI.
    ///
    /// # Arguments
    ///
    /// * `uri` - The URI of the resource to retrieve.
    ///
    /// # Errors
    ///
    /// This method can fail for various reasons:
    /// - Resource not found
    /// - Network errors (for remote resources)
    /// - Permission errors
    fn retrieve(
        &self,
        uri: &Uri<String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync>>;
}

#[derive(Debug, Clone)]
struct DefaultRetrieverError;

impl fmt::Display for DefaultRetrieverError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Default retriever does not fetch resources")
    }
}

impl std::error::Error for DefaultRetrieverError {}

/// A retriever that always fails, used as a default when external resource fetching is not needed.
#[derive(Debug, PartialEq, Eq)]
pub struct DefaultRetriever;

impl Retrieve for DefaultRetriever {
    fn retrieve(&self, _: &Uri<String>) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
        Err(Box::new(DefaultRetrieverError))
    }
}

#[cfg(feature = "retrieve-async")]
#[cfg_attr(target_family = "wasm", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_family = "wasm"), async_trait::async_trait)]
pub trait AsyncRetrieve: RetrieverBounds {
    /// Asynchronously retrieve a resource from the given URI.
    ///
    /// This is the non-blocking equivalent of [`Retrieve::retrieve`].
    ///
    /// # Arguments
    ///
    /// * `uri` - The URI of the resource to retrieve.
    ///
    /// # Errors
    ///
    /// This method can fail for various reasons:
    /// - Resource not found
    /// - Network errors (for remote resources)
    /// - Permission errors
    async fn retrieve(
        &self,
        uri: &Uri<String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync>>;
}

#[cfg(feature = "retrieve-async")]
#[cfg_attr(target_family = "wasm", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_family = "wasm"), async_trait::async_trait)]
impl AsyncRetrieve for DefaultRetriever {
    async fn retrieve(
        &self,
        _: &Uri<String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
        Err(Box::new(DefaultRetrieverError))
    }
}
