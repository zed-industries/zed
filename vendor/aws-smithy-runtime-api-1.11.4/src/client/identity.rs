/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::box_error::BoxError;
use crate::client::runtime_components::sealed::ValidateConfig;
use crate::client::runtime_components::{RuntimeComponents, RuntimeComponentsBuilder};
use crate::impl_shared_conversions;
use aws_smithy_types::config_bag::ConfigBag;
use aws_smithy_types::type_erasure::TypeErasedBox;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt;
use std::fmt::Debug;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::SystemTime;

#[cfg(feature = "http-auth")]
pub mod http;

new_type_future! {
    #[doc = "Future for [`IdentityResolver::resolve_identity`]."]
    pub struct IdentityFuture<'a, Identity, BoxError>;
}

static NEXT_CACHE_PARTITION: AtomicUsize = AtomicUsize::new(0);

/// Cache partition key for identity caching.
///
/// Identities need cache partitioning because a single identity cache is used across
/// multiple identity providers across multiple auth schemes. In addition, a single auth scheme
/// may have many different identity providers due to operation-level config overrides.
///
/// This partition _must_ be respected when retrieving from the identity cache and _should_
/// be part of the cache key.
///
/// Calling [`IdentityCachePartition::new`] will create a new globally unique cache partition key,
/// and the [`SharedIdentityResolver`] will automatically create and store a partion on construction.
/// Thus, every configured identity resolver will be assigned a unique partition.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct IdentityCachePartition(usize);

impl IdentityCachePartition {
    /// Create a new globally unique cache partition key.
    pub fn new() -> Self {
        Self(NEXT_CACHE_PARTITION.fetch_add(1, Ordering::Relaxed))
    }

    /// Helper for unit tests to create an identity cache partition with a known value.
    #[cfg(feature = "test-util")]
    pub fn new_for_tests(value: usize) -> IdentityCachePartition {
        Self(value)
    }
}

/// Caching resolver for identities.
pub trait ResolveCachedIdentity: fmt::Debug + Send + Sync {
    /// Returns a cached identity, or resolves an identity and caches it if its not already cached.
    fn resolve_cached_identity<'a>(
        &'a self,
        resolver: SharedIdentityResolver,
        runtime_components: &'a RuntimeComponents,
        config_bag: &'a ConfigBag,
    ) -> IdentityFuture<'a>;

    #[doc = include_str!("../../rustdoc/validate_base_client_config.md")]
    fn validate_base_client_config(
        &self,
        runtime_components: &RuntimeComponentsBuilder,
        cfg: &ConfigBag,
    ) -> Result<(), BoxError> {
        let _ = (runtime_components, cfg);
        Ok(())
    }

    #[doc = include_str!("../../rustdoc/validate_final_config.md")]
    fn validate_final_config(
        &self,
        runtime_components: &RuntimeComponents,
        cfg: &ConfigBag,
    ) -> Result<(), BoxError> {
        let _ = (runtime_components, cfg);
        Ok(())
    }
}

/// Shared identity cache.
#[derive(Clone, Debug)]
pub struct SharedIdentityCache(Arc<dyn ResolveCachedIdentity>);

impl SharedIdentityCache {
    /// Creates a new [`SharedIdentityCache`] from the given cache implementation.
    pub fn new(cache: impl ResolveCachedIdentity + 'static) -> Self {
        Self(Arc::new(cache))
    }
}

impl ResolveCachedIdentity for SharedIdentityCache {
    fn resolve_cached_identity<'a>(
        &'a self,
        resolver: SharedIdentityResolver,
        runtime_components: &'a RuntimeComponents,
        config_bag: &'a ConfigBag,
    ) -> IdentityFuture<'a> {
        self.0
            .resolve_cached_identity(resolver, runtime_components, config_bag)
    }
}

impl ValidateConfig for SharedIdentityResolver {}

impl ValidateConfig for SharedIdentityCache {
    fn validate_base_client_config(
        &self,
        runtime_components: &RuntimeComponentsBuilder,
        cfg: &ConfigBag,
    ) -> Result<(), BoxError> {
        self.0.validate_base_client_config(runtime_components, cfg)
    }

    fn validate_final_config(
        &self,
        runtime_components: &RuntimeComponents,
        cfg: &ConfigBag,
    ) -> Result<(), BoxError> {
        self.0.validate_final_config(runtime_components, cfg)
    }
}

impl_shared_conversions!(convert SharedIdentityCache from ResolveCachedIdentity using SharedIdentityCache::new);

/// Resolver for identities.
///
/// Every [`AuthScheme`](crate::client::auth::AuthScheme) has one or more compatible
/// identity resolvers, which are selected from runtime components by the auth scheme
/// implementation itself.
///
/// The identity resolver must return an [`IdentityFuture`] with the resolved identity, or an error
/// if resolution failed. There is no optionality for identity resolvers. The identity either
/// resolves successfully, or it fails. The orchestrator will choose exactly one auth scheme
/// to use, and thus, its chosen identity resolver is the only identity resolver that runs.
/// There is no fallback to other auth schemes in the absence of an identity.
pub trait ResolveIdentity: Send + Sync + Debug {
    /// Asynchronously resolves an identity for a request using the given config.
    fn resolve_identity<'a>(
        &'a self,
        runtime_components: &'a RuntimeComponents,
        config_bag: &'a ConfigBag,
    ) -> IdentityFuture<'a>;

    /// Returns a fallback identity.
    ///
    /// This method should be used as a fallback plan, i.e., when a call to `resolve_identity`
    /// is interrupted by a timeout and its future fails to complete.
    ///
    /// The fallback identity should be set aside and ready to be returned
    /// immediately. Therefore, a new identity should NOT be fetched
    /// within this method, which might cause a long-running operation.
    fn fallback_on_interrupt(&self) -> Option<Identity> {
        None
    }

    /// Returns the location of an identity cache associated with this identity resolver.
    ///
    /// By default, identity resolvers will use the identity cache stored in runtime components.
    /// Implementing types can change the cache location if they want to. Refer to [`IdentityCacheLocation`]
    /// explaining why a concrete identity resolver might want to change the cache location.
    fn cache_location(&self) -> IdentityCacheLocation {
        IdentityCacheLocation::RuntimeComponents
    }

    /// Returns the identity cache partition associated with this identity resolver.
    ///
    /// By default this returns `None` and cache partitioning is left up to `SharedIdentityResolver`.
    fn cache_partition(&self) -> Option<IdentityCachePartition> {
        None
    }
}

/// Cache location for identity caching.
///
/// Identities are usually cached in the identity cache owned by [`RuntimeComponents`]. However,
/// we do have identities whose caching mechanism is internally managed by their identity resolver,
/// in which case we want to avoid the `RuntimeComponents`-owned identity cache interfering with
/// the internal caching policy.
#[non_exhaustive]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum IdentityCacheLocation {
    /// Indicates the identity cache is owned by [`RuntimeComponents`].
    RuntimeComponents,
    /// Indicates the identity cache is internally managed by the identity resolver.
    IdentityResolver,
}

/// Container for a shared identity resolver.
#[derive(Clone, Debug)]
pub struct SharedIdentityResolver {
    inner: Arc<dyn ResolveIdentity>,
    cache_partition: IdentityCachePartition,
}

impl SharedIdentityResolver {
    /// Creates a new [`SharedIdentityResolver`] from the given resolver.
    pub fn new(resolver: impl ResolveIdentity + 'static) -> Self {
        // NOTE: `IdentityCachePartition` is globally unique by construction so even
        // custom implementations of `ResolveIdentity::cache_partition()` are unique.
        let partition = match resolver.cache_partition() {
            Some(p) => p,
            None => IdentityCachePartition::new(),
        };

        Self {
            inner: Arc::new(resolver),
            cache_partition: partition,
        }
    }

    /// Returns the globally unique cache partition key for this identity resolver.
    ///
    /// See the [`IdentityCachePartition`] docs for more information on what this is used for
    /// and why.
    pub fn cache_partition(&self) -> IdentityCachePartition {
        self.cache_partition
    }
}

impl ResolveIdentity for SharedIdentityResolver {
    fn resolve_identity<'a>(
        &'a self,
        runtime_components: &'a RuntimeComponents,
        config_bag: &'a ConfigBag,
    ) -> IdentityFuture<'a> {
        self.inner.resolve_identity(runtime_components, config_bag)
    }

    fn cache_location(&self) -> IdentityCacheLocation {
        self.inner.cache_location()
    }

    fn cache_partition(&self) -> Option<IdentityCachePartition> {
        Some(self.cache_partition())
    }
}

impl_shared_conversions!(convert SharedIdentityResolver from ResolveIdentity using SharedIdentityResolver::new);

type DataDebug = Arc<dyn (Fn(&Arc<dyn Any + Send + Sync>) -> &dyn Debug) + Send + Sync>;

/// An identity that can be used for authentication.
///
/// The [`Identity`] is a container for any arbitrary identity data that may be used
/// by a [`Sign`](crate::client::auth::Sign) implementation. Under the hood, it
/// has an `Arc<dyn Any>`, and it is the responsibility of the signer to downcast
/// to the appropriate data type using the `data()` function.
///
/// The `Identity` also holds an optional expiration time, which may duplicate
/// an expiration time on the identity data. This is because an `Arc<dyn Any>`
/// can't be downcast to any arbitrary trait, and expiring identities are
/// common enough to be built-in.
#[derive(Clone)]
pub struct Identity {
    data: Arc<dyn Any + Send + Sync>,
    data_debug: DataDebug,
    expiration: Option<SystemTime>,
    properties: HashMap<TypeId, Arc<TypeErasedBox>>,
}

impl Identity {
    /// Creates a new identity with the given data and expiration time.
    pub fn new<T>(data: T, expiration: Option<SystemTime>) -> Self
    where
        T: Any + Debug + Send + Sync,
    {
        Self {
            data: Arc::new(data),
            data_debug: Arc::new(|d| d.downcast_ref::<T>().expect("type-checked") as _),
            expiration,
            properties: HashMap::default(),
        }
    }

    /// Returns [`Builder`] for [`Identity`].
    pub fn builder() -> Builder {
        Builder::default()
    }

    /// Returns the raw identity data.
    pub fn data<T: Any + Debug + Send + Sync + 'static>(&self) -> Option<&T> {
        self.data.downcast_ref()
    }

    /// Returns the expiration time for this identity, if any.
    pub fn expiration(&self) -> Option<SystemTime> {
        self.expiration
    }

    /// Returns arbitrary property associated with this `Identity`.
    pub fn property<T: Any + Debug + Send + Sync + 'static>(&self) -> Option<&T> {
        self.properties
            .get(&TypeId::of::<T>())
            .and_then(|b| b.downcast_ref())
    }
}

impl Debug for Identity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_struct = f.debug_struct("Identity");
        debug_struct
            .field("data", (self.data_debug)(&self.data))
            .field("expiration", &self.expiration);
        for (i, prop) in self.properties.values().enumerate() {
            debug_struct.field(&format!("property_{i}"), prop);
        }
        debug_struct.finish()
    }
}

impl ResolveIdentity for Identity {
    fn resolve_identity<'a>(
        &'a self,
        _runtime_components: &'a RuntimeComponents,
        _config_bag: &'a ConfigBag,
    ) -> IdentityFuture<'a> {
        IdentityFuture::ready(Ok(self.clone()))
    }
}

#[derive(Debug)]
enum ErrorKind {
    /// Field required to build the target type is missing.
    MissingRequiredField(&'static str),
}

/// Error constructing [`Identity`].
#[derive(Debug)]
pub struct BuildError {
    kind: ErrorKind,
}

impl BuildError {
    fn missing_required_field(field_name: &'static str) -> Self {
        BuildError {
            kind: ErrorKind::MissingRequiredField(field_name),
        }
    }
}

impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        use ErrorKind::*;
        match self.kind {
            MissingRequiredField(field_name) => write!(f, "missing required field: `{field_name}`"),
        }
    }
}

impl std::error::Error for BuildError {}

/// Builder for [`Identity`]
#[derive(Default)]
pub struct Builder {
    data: Option<Arc<dyn Any + Send + Sync>>,
    data_debug: Option<DataDebug>,
    expiration: Option<SystemTime>,
    properties: HashMap<TypeId, Arc<TypeErasedBox>>,
}

impl Builder {
    /// Set raw identity data for the builder.
    pub fn data<T: Any + Debug + Send + Sync + 'static>(mut self, data: T) -> Self {
        self.set_data(data);
        self
    }

    /// Set raw identity data for the builder.
    pub fn set_data<T: Any + Debug + Send + Sync + 'static>(&mut self, data: T) {
        self.data = Some(Arc::new(data));
        self.data_debug = Some(Arc::new(|d| {
            d.downcast_ref::<T>().expect("type-checked") as _
        }));
    }

    /// Set expiration for the builder.
    pub fn expiration(mut self, expiration: SystemTime) -> Self {
        self.set_expiration(Some(expiration));
        self
    }

    /// Set expiration for the builder.
    pub fn set_expiration(&mut self, expiration: Option<SystemTime>) {
        self.expiration = expiration;
    }

    /// Set arbitrary property for the builder.
    pub fn property<T: Any + Debug + Send + Sync + 'static>(mut self, prop: T) -> Self {
        self.set_property(prop);
        self
    }

    /// Set arbitrary property for the builder.
    pub fn set_property<T: Any + Debug + Send + Sync + 'static>(&mut self, prop: T) {
        self.properties
            .insert(TypeId::of::<T>(), Arc::new(TypeErasedBox::new(prop)));
    }

    /// Build [`Identity`].
    pub fn build(self) -> Result<Identity, BuildError> {
        Ok(Identity {
            data: self
                .data
                .ok_or_else(|| BuildError::missing_required_field("data"))?,
            data_debug: self
                .data_debug
                .expect("should always be set when `data` is set"),
            expiration: self.expiration,
            properties: self.properties,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aws_smithy_async::time::{SystemTimeSource, TimeSource};

    #[test]
    fn check_send_sync() {
        fn is_send_sync<T: Send + Sync>(_: T) {}
        is_send_sync(Identity::new("foo", None));
    }

    #[test]
    fn create_retrieve_identity() {
        #[derive(Debug)]
        struct MyIdentityData {
            first: String,
            last: String,
        }

        let ts = SystemTimeSource::new();
        let expiration = ts.now();
        let identity = Identity::new(
            MyIdentityData {
                first: "foo".into(),
                last: "bar".into(),
            },
            Some(expiration),
        );

        assert_eq!("foo", identity.data::<MyIdentityData>().unwrap().first);
        assert_eq!("bar", identity.data::<MyIdentityData>().unwrap().last);
        assert_eq!(Some(expiration), identity.expiration());
    }

    #[test]
    fn insert_get_identity_properties() {
        #[derive(Debug)]
        struct MyIdentityData {
            first: String,
            last: String,
        }
        #[derive(Debug)]
        struct PropertyAlpha;
        #[derive(Debug)]
        struct PropertyBeta;

        let ts = SystemTimeSource::new();
        let expiration = ts.now();
        let identity = Identity::builder()
            .data(MyIdentityData {
                first: "foo".into(),
                last: "bar".into(),
            })
            .expiration(expiration)
            .property(PropertyAlpha)
            .property(PropertyBeta)
            .build()
            .unwrap();

        assert_eq!("foo", identity.data::<MyIdentityData>().unwrap().first);
        assert_eq!("bar", identity.data::<MyIdentityData>().unwrap().last);
        assert_eq!(Some(expiration), identity.expiration());
        assert!(identity.property::<PropertyAlpha>().is_some());
        assert!(identity.property::<PropertyBeta>().is_some());
    }
}
