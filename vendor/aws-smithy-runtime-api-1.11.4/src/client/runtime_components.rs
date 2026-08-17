/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Runtime components used to make a request and handle a response.
//!
//! Runtime components are trait implementations that are _always_ used by the orchestrator.
//! There are other trait implementations that can be configured for a client, but if they
//! aren't directly and always used by the orchestrator, then they are placed in the
//! [`ConfigBag`] instead of in [`RuntimeComponents`].

use crate::box_error::BoxError;
use crate::client::auth::{
    AuthScheme, AuthSchemeId, ResolveAuthSchemeOptions, SharedAuthScheme,
    SharedAuthSchemeOptionResolver,
};
use crate::client::endpoint::{ResolveEndpoint, SharedEndpointResolver};
use crate::client::http::{HttpClient, SharedHttpClient};
use crate::client::identity::{
    ResolveCachedIdentity, ResolveIdentity, SharedIdentityCache, SharedIdentityResolver,
};
use crate::client::interceptors::{Intercept, SharedInterceptor};
use crate::client::retries::classifiers::{ClassifyRetry, SharedRetryClassifier};
use crate::client::retries::{RetryStrategy, SharedRetryStrategy};
use crate::impl_shared_conversions;
use crate::shared::IntoShared;
use aws_smithy_async::rt::sleep::{AsyncSleep, SharedAsyncSleep};
use aws_smithy_async::time::{SharedTimeSource, TimeSource};
use aws_smithy_types::config_bag::ConfigBag;
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

pub(crate) static EMPTY_RUNTIME_COMPONENTS_BUILDER: RuntimeComponentsBuilder =
    RuntimeComponentsBuilder::new("empty");

pub(crate) mod sealed {
    use super::*;

    /// Validates client configuration.
    ///
    /// This trait can be used to validate that certain required components or config values
    /// are available, and provide an error with helpful instructions if they are not.
    pub trait ValidateConfig: fmt::Debug + Send + Sync {
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
}
use sealed::ValidateConfig;

#[derive(Clone)]
enum ValidatorInner {
    BaseConfigStaticFn(fn(&RuntimeComponentsBuilder, &ConfigBag) -> Result<(), BoxError>),
    Shared(Arc<dyn ValidateConfig>),
}

impl fmt::Debug for ValidatorInner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BaseConfigStaticFn(_) => f.debug_tuple("StaticFn").finish(),
            Self::Shared(_) => f.debug_tuple("Shared").finish(),
        }
    }
}

/// A client config validator.
#[derive(Clone, Debug)]
pub struct SharedConfigValidator {
    inner: ValidatorInner,
}

impl SharedConfigValidator {
    /// Creates a new shared config validator.
    pub(crate) fn new(validator: impl ValidateConfig + 'static) -> Self {
        Self {
            inner: ValidatorInner::Shared(Arc::new(validator) as _),
        }
    }

    /// Creates a base client validator from a function.
    ///
    /// A base client validator gets called upon client construction. The full
    /// config may not be available at this time (hence why it has
    /// [`RuntimeComponentsBuilder`] as an argument rather than [`RuntimeComponents`]).
    /// Any error returned from the validator function will become a panic in the
    /// client constructor.
    ///
    /// # Examples
    ///
    /// Creating a validator function:
    /// ```no_run
    /// use aws_smithy_runtime_api::box_error::BoxError;
    /// use aws_smithy_runtime_api::client::runtime_components::{
    ///     RuntimeComponentsBuilder,
    ///     SharedConfigValidator
    /// };
    /// use aws_smithy_types::config_bag::ConfigBag;
    ///
    /// fn my_validation(
    ///     components: &RuntimeComponentsBuilder,
    ///     config: &ConfigBag
    /// ) -> Result<(), BoxError> {
    ///     if components.sleep_impl().is_none() {
    ///         return Err("I need a sleep_impl!".into());
    ///     }
    ///     Ok(())
    /// }
    ///
    /// let validator = SharedConfigValidator::base_client_config_fn(my_validation);
    /// ```
    pub fn base_client_config_fn(
        validator: fn(&RuntimeComponentsBuilder, &ConfigBag) -> Result<(), BoxError>,
    ) -> Self {
        Self {
            inner: ValidatorInner::BaseConfigStaticFn(validator),
        }
    }
}

impl ValidateConfig for SharedConfigValidator {
    fn validate_base_client_config(
        &self,
        runtime_components: &RuntimeComponentsBuilder,
        cfg: &ConfigBag,
    ) -> Result<(), BoxError> {
        match &self.inner {
            ValidatorInner::BaseConfigStaticFn(validator) => validator(runtime_components, cfg),
            ValidatorInner::Shared(validator) => {
                validator.validate_base_client_config(runtime_components, cfg)
            }
        }
    }

    fn validate_final_config(
        &self,
        runtime_components: &RuntimeComponents,
        cfg: &ConfigBag,
    ) -> Result<(), BoxError> {
        match &self.inner {
            ValidatorInner::Shared(validator) => {
                validator.validate_final_config(runtime_components, cfg)
            }
            _ => Ok(()),
        }
    }
}

impl_shared_conversions!(convert SharedConfigValidator from ValidateConfig using SharedConfigValidator::new);

/// Internal to `declare_runtime_components!`.
///
/// Merges a field from one builder into another.
macro_rules! merge {
    (Option $other:ident . $name:ident => $self:ident) => {
        $self.$name = $other.$name.clone().or($self.$name.take());
    };
    (Vec $other:ident . $name:ident => $self:ident) => {
        if !$other.$name.is_empty() {
            $self.$name.extend($other.$name.iter().cloned());
        }
    };
    (OptionalAuthSchemeMap $other:ident . $name:ident => $self:ident ) => {
        if let Some(m) = &$other.$name {
            let mut us = $self.$name.unwrap_or_default();
            us.extend(m.iter().map(|(k, v)| (k.clone(), v.clone())));
            $self.$name = Some(us);
        }
    };
}
/// Internal to `declare_runtime_components!`.
///
/// This is used when creating the builder's `build` method
/// to populate each individual field value. The `required`/`atLeastOneRequired`
/// validations are performed here.
macro_rules! builder_field_value {
    (Option $self:ident . $name:ident) => {
        $self.$name
    };
    (Option $self:ident . $name:ident required) => {
        $self.$name.ok_or(BuildError(concat!(
            "the `",
            stringify!($name),
            "` runtime component is required"
        )))?
    };
    (Vec $self:ident . $name:ident) => {
        $self.$name
    };
    (OptionalAuthSchemeMap $self:ident . $name:ident atLeastOneRequired) => {{
        match $self.$name {
            Some(map) => map,
            None => {
                return Err(BuildError(concat!(
                    "at least one `",
                    stringify!($name),
                    "` runtime component is required"
                )));
            }
        }
    }};
    (Vec $self:ident . $name:ident atLeastOneRequired) => {{
        if $self.$name.is_empty() {
            return Err(BuildError(concat!(
                "at least one `",
                stringify!($name),
                "` runtime component is required"
            )));
        }
        $self.$name
    }};
}
/// Internal to `declare_runtime_components!`.
///
/// Converts the field type from `Option<T>` or `Vec<T>` into `Option<Tracked<T>>` or `Vec<Tracked<T>>` respectively.
/// Also removes the `Option` wrapper for required fields in the non-builder struct.
macro_rules! runtime_component_field_type {
    (Option $inner_type:ident) => {
        Option<Tracked<$inner_type>>
    };
    (Option $inner_type:ident required) => {
        Tracked<$inner_type>
    };
    (Vec $inner_type:ident) => {
        Vec<Tracked<$inner_type>>
    };
    (Vec $inner_type:ident atLeastOneRequired) => {
        Vec<Tracked<$inner_type>>
    };
    (OptionalAuthSchemeMap $inner_type: ident atLeastOneRequired) => { AuthSchemeMap<Tracked<$inner_type>> };
}
/// Internal to `declare_runtime_components!`.
///
/// Converts an `$outer_type` into an empty instantiation for that type.
/// This is needed since `Default::default()` can't be used in a `const` function,
/// and `RuntimeComponentsBuilder::new()` is `const`.
macro_rules! empty_builder_value {
    (Option) => {
        None
    };
    (Vec) => {
        Vec::new()
    };
    (OptionalAuthSchemeMap) => {
        None
    };
}

type OptionalAuthSchemeMap<V> = Option<AuthSchemeMap<V>>;
type AuthSchemeMap<V> = HashMap<AuthSchemeId, V>;

/// Macro to define the structs for both `RuntimeComponents` and `RuntimeComponentsBuilder`.
///
/// This is a macro in order to keep the fields consistent between the two, and to automatically
/// update the `merge_from` and `build` methods when new components are added.
///
/// It also facilitates unit testing since the overall mechanism can be unit tested with different
/// fields that are easy to check in tests (testing with real components makes it hard
/// to tell that the correct component was selected when merging builders).
///
/// # Example usage
///
/// The two identifiers after "fields for" become the names of the struct and builder respectively.
/// Following that, all the fields are specified. Fields MUST be wrapped in `Option` or `Vec`.
/// To make a field required in the non-builder struct, add `#[required]` for `Option` fields, or
/// `#[atLeastOneRequired]` for `Vec` fields.
///
/// ```no_compile
/// declare_runtime_components! {
///     fields for TestRc and TestRcBuilder {
///         some_optional_string: Option<String>,
///
///         some_optional_vec: Vec<String>,
///
///         #[required]
///         some_required_string: Option<String>,
///
///         #[atLeastOneRequired]
///         some_required_vec: Vec<String>,
///     }
/// }
/// ```
macro_rules! declare_runtime_components {
    (fields for $rc_name:ident and $builder_name:ident {
        $($(#[$option:ident])? $field_name:ident : $outer_type:ident<$inner_type:ident> ,)+
    }) => {
        /// Components that can only be set in runtime plugins that the orchestrator uses directly to call an operation.
        #[derive(Clone, Debug)]
        pub struct $rc_name {
            $($field_name: runtime_component_field_type!($outer_type $inner_type $($option)?),)+
        }

        /// Builder for [`RuntimeComponents`].
        #[derive(Clone, Debug)]
        pub struct $builder_name {
            builder_name: &'static str,
            $($field_name: $outer_type<Tracked<$inner_type>>,)+
        }
        impl $builder_name {
            /// Creates a new builder.
            ///
            /// Since multiple builders are merged together to make the final [`RuntimeComponents`],
            /// all components added by this builder are associated with the given `name` so that
            /// the origin of a component can be easily found when debugging.
            pub const fn new(name: &'static str) -> Self {
                Self {
                    builder_name: name,
                    $($field_name: empty_builder_value!($outer_type),)+
                }
            }

            /// Merge in components from another builder.
            pub fn merge_from(mut self, other: &Self) -> Self {
                $(merge!($outer_type other.$field_name => self);)+
                self
            }

            /// Builds [`RuntimeComponents`] from this builder.
            pub fn build(self) -> Result<$rc_name, BuildError> {
                let mut rcs = $rc_name {
                    $($field_name: builder_field_value!($outer_type self.$field_name $($option)?),)+
                };
                rcs.sort();

                Ok(rcs)
            }
        }
    };
}

declare_runtime_components! {
    fields for RuntimeComponents and RuntimeComponentsBuilder {
        #[required]
        auth_scheme_option_resolver: Option<SharedAuthSchemeOptionResolver>,

        // A connector is not required since a client could technically only be used for presigning
        http_client: Option<SharedHttpClient>,

        #[required]
        endpoint_resolver: Option<SharedEndpointResolver>,

        #[atLeastOneRequired]
        auth_schemes: OptionalAuthSchemeMap<SharedAuthScheme>,

        #[required]
        identity_cache: Option<SharedIdentityCache>,

        #[atLeastOneRequired]
        identity_resolvers: OptionalAuthSchemeMap<SharedIdentityResolver>,

        interceptors: Vec<SharedInterceptor>,

        retry_classifiers: Vec<SharedRetryClassifier>,

        #[required]
        retry_strategy: Option<SharedRetryStrategy>,

        time_source: Option<SharedTimeSource>,

        sleep_impl: Option<SharedAsyncSleep>,

        config_validators: Vec<SharedConfigValidator>,
    }
}

impl RuntimeComponents {
    /// Returns a builder for runtime components.
    pub fn builder(name: &'static str) -> RuntimeComponentsBuilder {
        RuntimeComponentsBuilder::new(name)
    }

    /// Clones and converts this [`RuntimeComponents`] into a [`RuntimeComponentsBuilder`].
    pub fn to_builder(&self) -> RuntimeComponentsBuilder {
        RuntimeComponentsBuilder::from_runtime_components(
            self.clone(),
            "RuntimeComponentsBuilder::from_runtime_components",
        )
    }

    /// Returns the auth scheme option resolver.
    pub fn auth_scheme_option_resolver(&self) -> SharedAuthSchemeOptionResolver {
        self.auth_scheme_option_resolver.value.clone()
    }

    /// Returns the HTTP client.
    pub fn http_client(&self) -> Option<SharedHttpClient> {
        self.http_client.as_ref().map(|s| s.value.clone())
    }

    /// Returns the endpoint resolver.
    pub fn endpoint_resolver(&self) -> SharedEndpointResolver {
        self.endpoint_resolver.value.clone()
    }

    /// Returns the requested auth scheme if it is set.
    pub fn auth_scheme(&self, scheme_id: impl AsRef<AuthSchemeId>) -> Option<SharedAuthScheme> {
        self.auth_schemes
            .get(scheme_id.as_ref())
            .map(|s| s.value.clone())
    }

    /// Returns the identity cache.
    pub fn identity_cache(&self) -> SharedIdentityCache {
        self.identity_cache.value.clone()
    }

    /// Returns an iterator over the interceptors.
    pub fn interceptors(&self) -> impl Iterator<Item = SharedInterceptor> + '_ {
        self.interceptors.iter().map(|s| s.value.clone())
    }

    /// Returns an iterator over the retry classifiers.
    pub fn retry_classifiers(&self) -> impl Iterator<Item = SharedRetryClassifier> + '_ {
        self.retry_classifiers.iter().map(|s| s.value.clone())
    }

    // Needed for `impl ValidateConfig for SharedRetryClassifier {`
    #[cfg(debug_assertions)]
    pub(crate) fn retry_classifiers_slice(&self) -> &[Tracked<SharedRetryClassifier>] {
        self.retry_classifiers.as_slice()
    }

    /// Returns the retry strategy.
    pub fn retry_strategy(&self) -> SharedRetryStrategy {
        self.retry_strategy.value.clone()
    }

    /// Returns the async sleep implementation.
    pub fn sleep_impl(&self) -> Option<SharedAsyncSleep> {
        self.sleep_impl.as_ref().map(|s| s.value.clone())
    }

    /// Returns the time source.
    pub fn time_source(&self) -> Option<SharedTimeSource> {
        self.time_source.as_ref().map(|s| s.value.clone())
    }

    /// Returns the config validators.
    pub fn config_validators(&self) -> impl Iterator<Item = SharedConfigValidator> + '_ {
        self.config_validators.iter().map(|s| s.value.clone())
    }

    /// Validate the final client configuration.
    ///
    /// This is intended to be called internally by the client.
    pub fn validate_final_config(&self, cfg: &ConfigBag) -> Result<(), BoxError> {
        macro_rules! validate {
            (Required: $field:expr) => {
                ValidateConfig::validate_final_config(&$field.value, self, cfg)?;
            };
            (Option: $field:expr) => {
                if let Some(field) = $field.as_ref() {
                    ValidateConfig::validate_final_config(&field.value, self, cfg)?;
                }
            };
            (Vec: $field:expr) => {
                for entry in $field {
                    ValidateConfig::validate_final_config(&entry.value, self, cfg)?;
                }
            };
            (Map: $field:expr) => {
                for entry in $field.values() {
                    ValidateConfig::validate_final_config(&entry.value, self, cfg)?;
                }
            };
        }

        for validator in self.config_validators() {
            validator.validate_final_config(self, cfg)?;
        }

        validate!(Option: self.http_client);
        validate!(Required: self.endpoint_resolver);
        validate!(Map: &self.auth_schemes);
        validate!(Required: self.identity_cache);
        validate!(Map: self.identity_resolvers);
        validate!(Vec: &self.interceptors);
        validate!(Required: self.retry_strategy);
        validate!(Vec: &self.retry_classifiers);

        Ok(())
    }

    fn sort(&mut self) {
        self.retry_classifiers.sort_by_key(|rc| rc.value.priority());
    }
}

impl RuntimeComponentsBuilder {
    /// Creates a new [`RuntimeComponentsBuilder`], inheriting all fields from the given
    /// [`RuntimeComponents`].
    pub fn from_runtime_components(rc: RuntimeComponents, builder_name: &'static str) -> Self {
        Self {
            builder_name,
            auth_scheme_option_resolver: Some(rc.auth_scheme_option_resolver),
            http_client: rc.http_client,
            endpoint_resolver: Some(rc.endpoint_resolver),
            auth_schemes: Some(rc.auth_schemes),
            identity_cache: Some(rc.identity_cache),
            identity_resolvers: Some(rc.identity_resolvers),
            interceptors: rc.interceptors,
            retry_classifiers: rc.retry_classifiers,
            retry_strategy: Some(rc.retry_strategy),
            time_source: rc.time_source,
            sleep_impl: rc.sleep_impl,
            config_validators: rc.config_validators,
        }
    }

    /// Returns the auth scheme option resolver.
    pub fn auth_scheme_option_resolver(&self) -> Option<SharedAuthSchemeOptionResolver> {
        self.auth_scheme_option_resolver
            .as_ref()
            .map(|s| s.value.clone())
    }

    /// Sets the auth scheme option resolver.
    pub fn set_auth_scheme_option_resolver(
        &mut self,
        auth_scheme_option_resolver: Option<impl ResolveAuthSchemeOptions + 'static>,
    ) -> &mut Self {
        self.auth_scheme_option_resolver =
            self.tracked(auth_scheme_option_resolver.map(IntoShared::into_shared));
        self
    }

    /// Sets the auth scheme option resolver.
    pub fn with_auth_scheme_option_resolver(
        mut self,
        auth_scheme_option_resolver: Option<impl ResolveAuthSchemeOptions + 'static>,
    ) -> Self {
        self.set_auth_scheme_option_resolver(auth_scheme_option_resolver);
        self
    }

    /// Returns the HTTP client.
    pub fn http_client(&self) -> Option<SharedHttpClient> {
        self.http_client.as_ref().map(|s| s.value.clone())
    }

    /// Sets the HTTP client.
    pub fn set_http_client(&mut self, connector: Option<impl HttpClient + 'static>) -> &mut Self {
        self.http_client = self.tracked(connector.map(IntoShared::into_shared));
        self
    }

    /// Sets the HTTP client.
    pub fn with_http_client(mut self, connector: Option<impl HttpClient + 'static>) -> Self {
        self.set_http_client(connector);
        self
    }

    /// Returns the endpoint resolver.
    pub fn endpoint_resolver(&self) -> Option<SharedEndpointResolver> {
        self.endpoint_resolver.as_ref().map(|s| s.value.clone())
    }

    /// Sets the endpoint resolver.
    pub fn set_endpoint_resolver(
        &mut self,
        endpoint_resolver: Option<impl ResolveEndpoint + 'static>,
    ) -> &mut Self {
        self.endpoint_resolver =
            endpoint_resolver.map(|s| Tracked::new(self.builder_name, s.into_shared()));
        self
    }

    /// Sets the endpoint resolver.
    pub fn with_endpoint_resolver(
        mut self,
        endpoint_resolver: Option<impl ResolveEndpoint + 'static>,
    ) -> Self {
        self.set_endpoint_resolver(endpoint_resolver);
        self
    }

    /// Returns the auth schemes.
    pub fn auth_schemes(&self) -> impl Iterator<Item = SharedAuthScheme> + '_ {
        self.auth_schemes
            .iter()
            .flat_map(|s| s.values().map(|t| t.value.clone()))
    }

    /// Adds an auth scheme, replacing the existing one.
    pub fn push_auth_scheme(&mut self, auth_scheme: impl AuthScheme + 'static) -> &mut Self {
        let mut auth_schemes = self.auth_schemes.take().unwrap_or_default();
        auth_schemes.insert(
            auth_scheme.scheme_id(),
            Tracked::new(self.builder_name, auth_scheme.into_shared()),
        );
        self.auth_schemes = Some(auth_schemes);
        self
    }

    /// Adds an auth scheme.
    pub fn with_auth_scheme(mut self, auth_scheme: impl AuthScheme + 'static) -> Self {
        self.push_auth_scheme(auth_scheme);
        self
    }

    /// Returns the identity cache.
    pub fn identity_cache(&self) -> Option<SharedIdentityCache> {
        self.identity_cache.as_ref().map(|s| s.value.clone())
    }

    /// Sets the identity cache.
    pub fn set_identity_cache(
        &mut self,
        identity_cache: Option<impl ResolveCachedIdentity + 'static>,
    ) -> &mut Self {
        self.identity_cache =
            identity_cache.map(|c| Tracked::new(self.builder_name, c.into_shared()));
        self
    }

    /// Sets the identity cache.
    pub fn with_identity_cache(
        mut self,
        identity_cache: Option<impl ResolveCachedIdentity + 'static>,
    ) -> Self {
        self.set_identity_cache(identity_cache);
        self
    }

    /// Returns [`SharedIdentityResolver`] configured in the builder for a given `scheme_id`.
    pub fn identity_resolver(&self, scheme_id: &AuthSchemeId) -> Option<SharedIdentityResolver> {
        self.identity_resolvers
            .as_ref()
            .and_then(|resolvers| resolvers.get(scheme_id))
            .map(|tracked| tracked.value.clone())
    }

    /// This method is broken since it does not replace an existing identity resolver of the given auth scheme ID.
    /// Use `set_identity_resolver` instead.
    #[deprecated(
        note = "This method is broken since it does not replace an existing identity resolver of the given auth scheme ID. Use `set_identity_resolver` instead."
    )]
    pub fn push_identity_resolver(
        &mut self,
        scheme_id: AuthSchemeId,
        identity_resolver: impl ResolveIdentity + 'static,
    ) -> &mut Self {
        self.set_identity_resolver(scheme_id, identity_resolver)
    }

    /// Sets the identity resolver for a given `scheme_id`.
    ///
    /// If there is already an identity resolver for that `scheme_id`, this method will replace
    /// the existing one with the passed-in `identity_resolver`.
    pub fn set_identity_resolver(
        &mut self,
        scheme_id: AuthSchemeId,
        identity_resolver: impl ResolveIdentity + 'static,
    ) -> &mut Self {
        let mut resolvers = self.identity_resolvers.take().unwrap_or_default();
        resolvers.insert(
            scheme_id,
            Tracked::new(self.builder_name, identity_resolver.into_shared()),
        );
        self.identity_resolvers = Some(resolvers);
        self
    }

    /// Adds an identity resolver.
    pub fn with_identity_resolver(
        mut self,
        scheme_id: AuthSchemeId,
        identity_resolver: impl ResolveIdentity + 'static,
    ) -> Self {
        self.set_identity_resolver(scheme_id, identity_resolver);
        self
    }

    /// Returns the interceptors.
    pub fn interceptors(&self) -> impl Iterator<Item = SharedInterceptor> + '_ {
        self.interceptors.iter().map(|s| s.value.clone())
    }

    /// Adds all the given interceptors.
    pub fn extend_interceptors(
        &mut self,
        interceptors: impl Iterator<Item = SharedInterceptor>,
    ) -> &mut Self {
        self.interceptors
            .extend(interceptors.map(|s| Tracked::new(self.builder_name, s)));
        self
    }

    /// Adds an interceptor.
    pub fn push_interceptor(&mut self, interceptor: impl Intercept + 'static) -> &mut Self {
        self.interceptors
            .push(Tracked::new(self.builder_name, interceptor.into_shared()));
        self
    }

    /// Adds an interceptor.
    pub fn with_interceptor(mut self, interceptor: impl Intercept + 'static) -> Self {
        self.push_interceptor(interceptor);
        self
    }

    /// Directly sets the interceptors and clears out any that were previously pushed.
    pub fn set_interceptors(
        &mut self,
        interceptors: impl Iterator<Item = SharedInterceptor>,
    ) -> &mut Self {
        self.interceptors.clear();
        self.interceptors
            .extend(interceptors.map(|s| Tracked::new(self.builder_name, s)));
        self
    }

    /// Directly sets the interceptors and clears out any that were previously pushed.
    pub fn with_interceptors(
        mut self,
        interceptors: impl Iterator<Item = SharedInterceptor>,
    ) -> Self {
        self.set_interceptors(interceptors);
        self
    }

    /// Returns the retry classifiers.
    pub fn retry_classifiers(&self) -> impl Iterator<Item = SharedRetryClassifier> + '_ {
        self.retry_classifiers.iter().map(|s| s.value.clone())
    }

    /// Adds all the given retry classifiers.
    pub fn extend_retry_classifiers(
        &mut self,
        retry_classifiers: impl Iterator<Item = SharedRetryClassifier>,
    ) -> &mut Self {
        self.retry_classifiers
            .extend(retry_classifiers.map(|s| Tracked::new(self.builder_name, s)));
        self
    }

    /// Adds a retry_classifier.
    pub fn push_retry_classifier(
        &mut self,
        retry_classifier: impl ClassifyRetry + 'static,
    ) -> &mut Self {
        self.retry_classifiers.push(Tracked::new(
            self.builder_name,
            retry_classifier.into_shared(),
        ));
        self
    }

    /// Adds a retry_classifier.
    pub fn with_retry_classifier(mut self, retry_classifier: impl ClassifyRetry + 'static) -> Self {
        self.push_retry_classifier(retry_classifier);
        self
    }

    /// Directly sets the retry_classifiers and clears out any that were previously pushed.
    pub fn set_retry_classifiers(
        &mut self,
        retry_classifiers: impl Iterator<Item = SharedRetryClassifier>,
    ) -> &mut Self {
        self.retry_classifiers.clear();
        self.retry_classifiers
            .extend(retry_classifiers.map(|s| Tracked::new(self.builder_name, s)));
        self
    }

    /// Returns the retry strategy.
    pub fn retry_strategy(&self) -> Option<SharedRetryStrategy> {
        self.retry_strategy.as_ref().map(|s| s.value.clone())
    }

    /// Sets the retry strategy.
    pub fn set_retry_strategy(
        &mut self,
        retry_strategy: Option<impl RetryStrategy + 'static>,
    ) -> &mut Self {
        self.retry_strategy =
            retry_strategy.map(|s| Tracked::new(self.builder_name, s.into_shared()));
        self
    }

    /// Sets the retry strategy.
    pub fn with_retry_strategy(
        mut self,
        retry_strategy: Option<impl RetryStrategy + 'static>,
    ) -> Self {
        self.retry_strategy =
            retry_strategy.map(|s| Tracked::new(self.builder_name, s.into_shared()));
        self
    }

    /// Returns the async sleep implementation.
    pub fn sleep_impl(&self) -> Option<SharedAsyncSleep> {
        self.sleep_impl.as_ref().map(|s| s.value.clone())
    }

    /// Sets the async sleep implementation.
    pub fn set_sleep_impl(&mut self, sleep_impl: Option<SharedAsyncSleep>) -> &mut Self {
        self.sleep_impl = self.tracked(sleep_impl);
        self
    }

    /// Sets the async sleep implementation.
    pub fn with_sleep_impl(mut self, sleep_impl: Option<impl AsyncSleep + 'static>) -> Self {
        self.set_sleep_impl(sleep_impl.map(IntoShared::into_shared));
        self
    }

    /// Returns the time source.
    pub fn time_source(&self) -> Option<SharedTimeSource> {
        self.time_source.as_ref().map(|s| s.value.clone())
    }

    /// Sets the time source.
    pub fn set_time_source(&mut self, time_source: Option<SharedTimeSource>) -> &mut Self {
        self.time_source = self.tracked(time_source);
        self
    }

    /// Sets the time source.
    pub fn with_time_source(mut self, time_source: Option<impl TimeSource + 'static>) -> Self {
        self.set_time_source(time_source.map(IntoShared::into_shared));
        self
    }

    /// Returns the config validators.
    pub fn config_validators(&self) -> impl Iterator<Item = SharedConfigValidator> + '_ {
        self.config_validators.iter().map(|s| s.value.clone())
    }

    /// Adds all the given config validators.
    pub fn extend_config_validators(
        &mut self,
        config_validators: impl Iterator<Item = SharedConfigValidator>,
    ) -> &mut Self {
        self.config_validators
            .extend(config_validators.map(|s| Tracked::new(self.builder_name, s)));
        self
    }

    /// Adds a config validator.
    pub fn push_config_validator(
        &mut self,
        config_validator: impl ValidateConfig + 'static,
    ) -> &mut Self {
        self.config_validators.push(Tracked::new(
            self.builder_name,
            config_validator.into_shared(),
        ));
        self
    }

    /// Adds a config validator.
    pub fn with_config_validator(
        mut self,
        config_validator: impl ValidateConfig + 'static,
    ) -> Self {
        self.push_config_validator(config_validator);
        self
    }

    /// Validate the base client configuration.
    ///
    /// This is intended to be called internally by the client.
    pub fn validate_base_client_config(&self, cfg: &ConfigBag) -> Result<(), BoxError> {
        macro_rules! validate {
            ($field:expr) => {
                #[allow(for_loops_over_fallibles)]
                for entry in $field {
                    ValidateConfig::validate_base_client_config(&entry.value, self, cfg)?;
                }
            };
        }

        for validator in self.config_validators() {
            validator.validate_base_client_config(self, cfg)?;
        }
        validate!(&self.http_client);
        validate!(&self.endpoint_resolver);
        if let Some(auth_schemes) = &self.auth_schemes {
            validate!(auth_schemes.values())
        }
        validate!(&self.identity_cache);
        if let Some(resolvers) = &self.identity_resolvers {
            validate!(resolvers.values())
        }
        validate!(&self.interceptors);
        validate!(&self.retry_strategy);
        Ok(())
    }

    /// Converts this builder into [`TimeComponents`].
    pub fn into_time_components(mut self) -> TimeComponents {
        TimeComponents {
            sleep_impl: self.sleep_impl.take().map(|s| s.value),
            time_source: self.time_source.take().map(|s| s.value),
        }
    }

    /// Wraps `v` in tracking associated with this builder
    fn tracked<T>(&self, v: Option<T>) -> Option<Tracked<T>> {
        v.map(|v| Tracked::new(self.builder_name, v))
    }
}

/// Time-related subset of components that can be extracted directly from [`RuntimeComponentsBuilder`] prior to validation.
#[derive(Debug)]
pub struct TimeComponents {
    sleep_impl: Option<SharedAsyncSleep>,
    time_source: Option<SharedTimeSource>,
}

impl TimeComponents {
    /// Returns the async sleep implementation if one is available.
    pub fn sleep_impl(&self) -> Option<SharedAsyncSleep> {
        self.sleep_impl.clone()
    }

    /// Returns the time source if one is available.
    pub fn time_source(&self) -> Option<SharedTimeSource> {
        self.time_source.clone()
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(test, derive(Eq, PartialEq))]
pub(crate) struct Tracked<T> {
    _origin: &'static str,
    value: T,
}

impl<T> Tracked<T> {
    fn new(origin: &'static str, value: T) -> Self {
        Self {
            _origin: origin,
            value,
        }
    }

    #[cfg(debug_assertions)]
    pub(crate) fn value(&self) -> &T {
        &self.value
    }
}

impl RuntimeComponentsBuilder {
    /// Creates a runtime components builder with all the required components filled in with fake (panicking) implementations.
    #[cfg(feature = "test-util")]
    pub fn for_tests() -> Self {
        use crate::client::endpoint::{EndpointFuture, EndpointResolverParams};
        use crate::client::identity::IdentityFuture;

        #[derive(Debug)]
        struct FakeAuthSchemeOptionResolver;
        impl ResolveAuthSchemeOptions for FakeAuthSchemeOptionResolver {
            fn resolve_auth_scheme_options(
                &self,
                _: &crate::client::auth::AuthSchemeOptionResolverParams,
            ) -> Result<std::borrow::Cow<'_, [AuthSchemeId]>, BoxError> {
                unreachable!("fake auth scheme option resolver must be overridden for this test")
            }
        }

        #[derive(Debug)]
        struct FakeClient;
        impl HttpClient for FakeClient {
            fn http_connector(
                &self,
                _: &crate::client::http::HttpConnectorSettings,
                _: &RuntimeComponents,
            ) -> crate::client::http::SharedHttpConnector {
                unreachable!("fake client must be overridden for this test")
            }
        }

        #[derive(Debug)]
        struct FakeEndpointResolver;
        impl ResolveEndpoint for FakeEndpointResolver {
            fn resolve_endpoint<'a>(&'a self, _: &'a EndpointResolverParams) -> EndpointFuture<'a> {
                unreachable!("fake endpoint resolver must be overridden for this test")
            }
        }

        #[derive(Debug)]
        struct FakeAuthScheme;
        impl AuthScheme for FakeAuthScheme {
            fn scheme_id(&self) -> AuthSchemeId {
                AuthSchemeId::new("fake")
            }

            fn identity_resolver(
                &self,
                _: &dyn GetIdentityResolver,
            ) -> Option<SharedIdentityResolver> {
                None
            }

            fn signer(&self) -> &dyn crate::client::auth::Sign {
                unreachable!("fake http auth scheme must be overridden for this test")
            }
        }

        #[derive(Debug)]
        struct FakeIdentityResolver;
        impl ResolveIdentity for FakeIdentityResolver {
            fn resolve_identity<'a>(
                &'a self,
                _: &'a RuntimeComponents,
                _: &'a ConfigBag,
            ) -> IdentityFuture<'a> {
                unreachable!("fake identity resolver must be overridden for this test")
            }
        }

        #[derive(Debug)]
        struct FakeRetryStrategy;
        impl RetryStrategy for FakeRetryStrategy {
            fn should_attempt_initial_request(
                &self,
                _: &RuntimeComponents,
                _: &ConfigBag,
            ) -> Result<crate::client::retries::ShouldAttempt, BoxError> {
                unreachable!("fake retry strategy must be overridden for this test")
            }

            fn should_attempt_retry(
                &self,
                _: &crate::client::interceptors::context::InterceptorContext,
                _: &RuntimeComponents,
                _: &ConfigBag,
            ) -> Result<crate::client::retries::ShouldAttempt, BoxError> {
                unreachable!("fake retry strategy must be overridden for this test")
            }
        }

        #[derive(Debug)]
        struct FakeTimeSource;
        impl TimeSource for FakeTimeSource {
            fn now(&self) -> std::time::SystemTime {
                unreachable!("fake time source must be overridden for this test")
            }
        }

        #[derive(Debug)]
        struct FakeSleep;
        impl AsyncSleep for FakeSleep {
            fn sleep(&self, _: std::time::Duration) -> aws_smithy_async::rt::sleep::Sleep {
                unreachable!("fake sleep must be overridden for this test")
            }
        }

        #[derive(Debug)]
        struct FakeIdentityCache;
        impl ResolveCachedIdentity for FakeIdentityCache {
            fn resolve_cached_identity<'a>(
                &'a self,
                resolver: SharedIdentityResolver,
                components: &'a RuntimeComponents,
                config_bag: &'a ConfigBag,
            ) -> IdentityFuture<'a> {
                IdentityFuture::new(async move {
                    resolver.resolve_identity(components, config_bag).await
                })
            }
        }

        Self::new("aws_smithy_runtime_api::client::runtime_components::RuntimeComponentBuilder::for_tests")
            .with_auth_scheme(FakeAuthScheme)
            .with_auth_scheme_option_resolver(Some(FakeAuthSchemeOptionResolver))
            .with_endpoint_resolver(Some(FakeEndpointResolver))
            .with_http_client(Some(FakeClient))
            .with_identity_cache(Some(FakeIdentityCache))
            .with_identity_resolver(AuthSchemeId::new("fake"), FakeIdentityResolver)
            .with_retry_strategy(Some(FakeRetryStrategy))
            .with_sleep_impl(Some(SharedAsyncSleep::new(FakeSleep)))
            .with_time_source(Some(SharedTimeSource::new(FakeTimeSource)))
    }
}

/// An error that occurs when building runtime components.
#[derive(Debug)]
pub struct BuildError(&'static str);

impl std::error::Error for BuildError {}

impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A trait for retrieving a shared identity resolver.
///
/// This trait exists so that [`AuthScheme::identity_resolver`]
/// can have access to configured identity resolvers without having access to all the runtime components.
pub trait GetIdentityResolver: Send + Sync {
    /// Returns the requested identity resolver if it is set.
    fn identity_resolver(&self, scheme_id: AuthSchemeId) -> Option<SharedIdentityResolver>;
}

impl GetIdentityResolver for RuntimeComponents {
    fn identity_resolver(&self, scheme_id: AuthSchemeId) -> Option<SharedIdentityResolver> {
        self.identity_resolvers
            .get(&scheme_id)
            .map(|s| s.value.clone())
    }
}

#[cfg(all(test, feature = "test-util"))]
mod tests {
    use super::{BuildError, RuntimeComponentsBuilder, Tracked};
    use crate::client::runtime_components::ValidateConfig;

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct TestComponent(String);
    impl ValidateConfig for TestComponent {}
    impl From<&'static str> for TestComponent {
        fn from(value: &'static str) -> Self {
            TestComponent(value.into())
        }
    }

    #[test]
    #[allow(unreachable_pub)]
    #[allow(dead_code)]
    fn the_builders_should_merge() {
        declare_runtime_components! {
            fields for TestRc and TestRcBuilder {
                #[required]
                some_required_component: Option<TestComponent>,

                some_optional_component: Option<TestComponent>,

                #[atLeastOneRequired]
                some_required_vec: Vec<TestComponent>,

                some_optional_vec: Vec<TestComponent>,
            }
        }

        impl TestRc {
            fn sort(&mut self) {}
        }

        let builder1 = TestRcBuilder {
            builder_name: "builder1",
            some_required_component: Some(Tracked::new("builder1", "override_me".into())),
            some_optional_component: Some(Tracked::new("builder1", "override_me optional".into())),
            some_required_vec: vec![Tracked::new("builder1", "first".into())],
            some_optional_vec: vec![Tracked::new("builder1", "first optional".into())],
        };
        let builder2 = TestRcBuilder {
            builder_name: "builder2",
            some_required_component: Some(Tracked::new("builder2", "override_me_too".into())),
            some_optional_component: Some(Tracked::new(
                "builder2",
                "override_me_too optional".into(),
            )),
            some_required_vec: vec![Tracked::new("builder2", "second".into())],
            some_optional_vec: vec![Tracked::new("builder2", "second optional".into())],
        };
        let builder3 = TestRcBuilder {
            builder_name: "builder3",
            some_required_component: Some(Tracked::new("builder3", "correct".into())),
            some_optional_component: Some(Tracked::new("builder3", "correct optional".into())),
            some_required_vec: vec![Tracked::new("builder3", "third".into())],
            some_optional_vec: vec![Tracked::new("builder3", "third optional".into())],
        };
        let rc = TestRcBuilder::new("root")
            .merge_from(&builder1)
            .merge_from(&builder2)
            .merge_from(&builder3)
            .build()
            .expect("success");
        assert_eq!(
            Tracked::new("builder3", TestComponent::from("correct")),
            rc.some_required_component
        );
        assert_eq!(
            Some(Tracked::new(
                "builder3",
                TestComponent::from("correct optional")
            )),
            rc.some_optional_component
        );
        assert_eq!(
            vec![
                Tracked::new("builder1", TestComponent::from("first")),
                Tracked::new("builder2", TestComponent::from("second")),
                Tracked::new("builder3", TestComponent::from("third"))
            ],
            rc.some_required_vec
        );
        assert_eq!(
            vec![
                Tracked::new("builder1", TestComponent::from("first optional")),
                Tracked::new("builder2", TestComponent::from("second optional")),
                Tracked::new("builder3", TestComponent::from("third optional"))
            ],
            rc.some_optional_vec
        );
    }

    #[test]
    #[allow(unreachable_pub)]
    #[allow(dead_code)]
    #[should_panic(expected = "the `_some_component` runtime component is required")]
    fn require_field_singular() {
        declare_runtime_components! {
            fields for TestRc and TestRcBuilder {
                #[required]
                _some_component: Option<TestComponent>,
            }
        }

        impl TestRc {
            fn sort(&mut self) {}
        }

        let rc = TestRcBuilder::new("test").build().unwrap();

        // Ensure the correct types were used
        let _: Tracked<TestComponent> = rc._some_component;
    }

    #[test]
    #[allow(unreachable_pub)]
    #[allow(dead_code)]
    #[should_panic(expected = "at least one `_some_vec` runtime component is required")]
    fn require_field_plural() {
        declare_runtime_components! {
            fields for TestRc and TestRcBuilder {
                #[atLeastOneRequired]
                _some_vec: Vec<TestComponent>,
            }
        }

        impl TestRc {
            fn sort(&mut self) {}
        }

        let rc = TestRcBuilder::new("test").build().unwrap();

        // Ensure the correct types were used
        let _: Vec<Tracked<TestComponent>> = rc._some_vec;
    }

    #[test]
    #[allow(unreachable_pub)]
    #[allow(dead_code)]
    fn optional_fields_dont_panic() {
        declare_runtime_components! {
            fields for TestRc and TestRcBuilder {
                _some_optional_component: Option<TestComponent>,
                _some_optional_vec: Vec<TestComponent>,
            }
        }

        impl TestRc {
            fn sort(&mut self) {}
        }

        let rc = TestRcBuilder::new("test").build().unwrap();

        // Ensure the correct types were used
        let _: Option<Tracked<TestComponent>> = rc._some_optional_component;
        let _: Vec<Tracked<TestComponent>> = rc._some_optional_vec;
    }

    #[test]
    fn building_test_builder_should_not_panic() {
        let _ = RuntimeComponentsBuilder::for_tests().build(); // should not panic
    }

    #[test]
    fn set_identity_resolver_should_replace_existing_resolver_for_given_auth_scheme() {
        use crate::client::auth::AuthSchemeId;
        use crate::client::identity::{Identity, IdentityFuture, ResolveIdentity};
        use crate::client::runtime_components::{GetIdentityResolver, RuntimeComponents};
        use aws_smithy_types::config_bag::ConfigBag;
        use tokio::runtime::Runtime;

        #[derive(Debug)]
        struct AnotherFakeIdentityResolver;
        impl ResolveIdentity for AnotherFakeIdentityResolver {
            fn resolve_identity<'a>(
                &'a self,
                _: &'a RuntimeComponents,
                _: &'a ConfigBag,
            ) -> IdentityFuture<'a> {
                IdentityFuture::ready(Ok(Identity::new("doesn't matter", None)))
            }
        }

        // Set a different `IdentityResolver` for the `fake` auth scheme already configured in
        // a test runtime components builder
        let rc = RuntimeComponentsBuilder::for_tests()
            .with_identity_resolver(AuthSchemeId::new("fake"), AnotherFakeIdentityResolver)
            .build()
            .expect("should build RuntimeComponents");

        let resolver = rc
            .identity_resolver(AuthSchemeId::new("fake"))
            .expect("identity resolver should be found");

        let identity = Runtime::new().unwrap().block_on(async {
            resolver
                .resolve_identity(&rc, &ConfigBag::base())
                .await
                .expect("identity should be resolved")
        });

        assert_eq!(Some(&"doesn't matter"), identity.data::<&str>());
    }
}
