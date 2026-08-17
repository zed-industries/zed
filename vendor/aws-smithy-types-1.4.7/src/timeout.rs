/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! This module defines types that describe timeouts that can be applied to various stages of the
//! Smithy networking stack.

use crate::config_bag::value::Value;
use crate::config_bag::{ItemIter, Storable, Store, StoreReplace};
use std::time::Duration;

#[derive(Clone, Debug, Default, PartialEq, Copy)]
enum CanDisable<T> {
    Disabled,
    #[default]
    Unset,
    Set(T),
}

impl<T> CanDisable<T> {
    fn none_implies_disabled(value: Option<T>) -> Self {
        match value {
            Some(t) => CanDisable::Set(t),
            None => CanDisable::Disabled,
        }
    }

    fn is_some(&self) -> bool {
        matches!(self, CanDisable::Set(_))
    }

    fn value(self) -> Option<T> {
        match self {
            CanDisable::Set(v) => Some(v),
            _ => None,
        }
    }

    fn merge_from_lower_priority(self, other: Self) -> Self {
        match (self, other) {
            // if we are unset. take the value from the other
            (CanDisable::Unset, value) => value,
            (us, _) => us,
        }
    }
}

impl<T> From<T> for CanDisable<T> {
    fn from(value: T) -> Self {
        Self::Set(value)
    }
}

/// Builder for [`TimeoutConfig`].
#[non_exhaustive]
#[derive(Clone, Debug, Default)]
pub struct TimeoutConfigBuilder {
    connect_timeout: CanDisable<Duration>,
    read_timeout: CanDisable<Duration>,
    operation_timeout: CanDisable<Duration>,
    operation_attempt_timeout: CanDisable<Duration>,
}

impl TimeoutConfigBuilder {
    /// Creates a new builder with no timeouts set.
    pub fn new() -> Self {
        Default::default()
    }

    /// Sets the connect timeout.
    ///
    /// The connect timeout is a limit on the amount of time it takes to initiate a socket connection.
    pub fn connect_timeout(mut self, connect_timeout: Duration) -> Self {
        self.connect_timeout = connect_timeout.into();
        self
    }

    /// Sets the connect timeout.
    ///
    /// If `None` is passed, this will explicitly disable the connection timeout.
    ///
    /// The connect timeout is a limit on the amount of time it takes to initiate a socket connection.
    pub fn set_connect_timeout(&mut self, connect_timeout: Option<Duration>) -> &mut Self {
        self.connect_timeout = CanDisable::none_implies_disabled(connect_timeout);
        self
    }

    /// Disables the connect timeout
    pub fn disable_connect_timeout(mut self) -> Self {
        self.connect_timeout = CanDisable::Disabled;
        self
    }

    /// Sets the read timeout.
    ///
    /// The read timeout is the limit on the amount of time it takes to read the first byte of a response
    /// from the time the request is initiated.
    pub fn read_timeout(mut self, read_timeout: Duration) -> Self {
        self.read_timeout = read_timeout.into();
        self
    }

    /// Sets the read timeout.
    ///
    /// If `None` is passed, this will explicitly disable the read timeout. To disable all timeouts use [`TimeoutConfig::disabled`].
    ///
    /// The read timeout is the limit on the amount of time it takes to read the first byte of a response
    /// from the time the request is initiated.
    pub fn set_read_timeout(&mut self, read_timeout: Option<Duration>) -> &mut Self {
        self.read_timeout = CanDisable::none_implies_disabled(read_timeout);
        self
    }

    /// Disables the read timeout
    pub fn disable_read_timeout(mut self) -> Self {
        self.read_timeout = CanDisable::Disabled;
        self
    }

    /// Sets the operation timeout.
    ///
    /// An operation represents the full request/response lifecycle of a call to a service.
    /// The operation timeout is a limit on the total amount of time it takes for an operation to be
    /// fully serviced, including the time for all retries that may have been attempted for it.
    ///
    /// If you want to set a timeout on individual retry attempts, then see [`Self::operation_attempt_timeout`]
    /// or [`Self::set_operation_attempt_timeout`].
    pub fn operation_timeout(mut self, operation_timeout: Duration) -> Self {
        self.operation_timeout = operation_timeout.into();
        self
    }

    /// Sets the operation timeout.
    ///
    /// If `None` is passed, this will explicitly disable the read timeout. To disable all timeouts use [`TimeoutConfig::disabled`].
    ///
    /// An operation represents the full request/response lifecycle of a call to a service.
    /// The operation timeout is a limit on the total amount of time it takes for an operation to be
    /// fully serviced, including the time for all retries that may have been attempted for it.
    ///
    /// If you want to set a timeout on individual retry attempts, then see [`Self::operation_attempt_timeout`]
    /// or [`Self::set_operation_attempt_timeout`].
    pub fn set_operation_timeout(&mut self, operation_timeout: Option<Duration>) -> &mut Self {
        self.operation_timeout = CanDisable::none_implies_disabled(operation_timeout);
        self
    }

    /// Disables the operation timeout
    pub fn disable_operation_timeout(mut self) -> Self {
        self.operation_timeout = CanDisable::Disabled;
        self
    }

    /// Sets the operation attempt timeout.
    ///
    /// An operation represents the full request/response lifecycle of a call to a service.
    /// When retries are enabled, then this setting makes it possible to set a timeout for individual
    /// retry attempts (including the initial attempt) for an operation.
    ///
    /// If you want to set a timeout on the total time for an entire request including all of its retries,
    /// then see [`Self::operation_timeout`] /// or [`Self::set_operation_timeout`].
    pub fn operation_attempt_timeout(mut self, operation_attempt_timeout: Duration) -> Self {
        self.operation_attempt_timeout = operation_attempt_timeout.into();
        self
    }

    /// Sets the operation attempt timeout.
    ///
    /// If `None` is passed, this will explicitly disable the operation timeout. To disable all timeouts use [`TimeoutConfig::disabled`].
    ///
    /// An operation represents the full request/response lifecycle of a call to a service.
    /// When retries are enabled, then this setting makes it possible to set a timeout for individual
    /// retry attempts (including the initial attempt) for an operation.
    ///
    /// If you want to set a timeout on individual retry attempts, then see [`Self::operation_attempt_timeout`]
    /// or [`Self::set_operation_attempt_timeout`].
    pub fn set_operation_attempt_timeout(
        &mut self,
        operation_attempt_timeout: Option<Duration>,
    ) -> &mut Self {
        self.operation_attempt_timeout =
            CanDisable::none_implies_disabled(operation_attempt_timeout);
        self
    }

    /// Disables the operation_attempt timeout
    pub fn disable_operation_attempt_timeout(mut self) -> Self {
        self.operation_attempt_timeout = CanDisable::Disabled;
        self
    }

    /// Merges two timeout config builders together.
    ///
    /// Values from `other` will only be used as a fallback for values
    /// from `self`. Useful for merging configs from different sources together when you want to
    /// handle "precedence" per value instead of at the config level
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::time::Duration;
    /// # use aws_smithy_types::timeout::TimeoutConfig;
    /// let a = TimeoutConfig::builder()
    ///     .connect_timeout(Duration::from_secs(3));
    /// let b = TimeoutConfig::builder()
    ///     .connect_timeout(Duration::from_secs(5))
    ///     .operation_timeout(Duration::from_secs(3));
    /// let timeout_config = a.take_unset_from(b).build();
    ///
    /// // A's value take precedence over B's value
    /// assert_eq!(timeout_config.connect_timeout(), Some(Duration::from_secs(3)));
    /// // A never set an operation timeout so B's value is used
    /// assert_eq!(timeout_config.operation_timeout(), Some(Duration::from_secs(3)));
    /// ```
    pub fn take_unset_from(self, other: Self) -> Self {
        Self {
            connect_timeout: self
                .connect_timeout
                .merge_from_lower_priority(other.connect_timeout),
            read_timeout: self
                .read_timeout
                .merge_from_lower_priority(other.read_timeout),
            operation_timeout: self
                .operation_timeout
                .merge_from_lower_priority(other.operation_timeout),
            operation_attempt_timeout: self
                .operation_attempt_timeout
                .merge_from_lower_priority(other.operation_attempt_timeout),
        }
    }

    /// Builds a `TimeoutConfig`.
    pub fn build(self) -> TimeoutConfig {
        TimeoutConfig {
            connect_timeout: self.connect_timeout,
            read_timeout: self.read_timeout,
            operation_timeout: self.operation_timeout,
            operation_attempt_timeout: self.operation_attempt_timeout,
        }
    }
}

impl From<TimeoutConfig> for TimeoutConfigBuilder {
    fn from(timeout_config: TimeoutConfig) -> Self {
        TimeoutConfigBuilder {
            connect_timeout: timeout_config.connect_timeout,
            read_timeout: timeout_config.read_timeout,
            operation_timeout: timeout_config.operation_timeout,
            operation_attempt_timeout: timeout_config.operation_attempt_timeout,
        }
    }
}

/// Top-level configuration for timeouts
///
/// # Example
///
/// ```rust
/// # use std::time::Duration;
///
/// # fn main() {
/// use aws_smithy_types::timeout::TimeoutConfig;
///
/// let timeout_config = TimeoutConfig::builder()
///     .operation_timeout(Duration::from_secs(30))
///     .operation_attempt_timeout(Duration::from_secs(10))
///     .connect_timeout(Duration::from_secs(3))
///     .build();
///
/// assert_eq!(
///     timeout_config.operation_timeout(),
///     Some(Duration::from_secs(30))
/// );
/// assert_eq!(
///     timeout_config.operation_attempt_timeout(),
///     Some(Duration::from_secs(10))
/// );
/// assert_eq!(
///     timeout_config.connect_timeout(),
///     Some(Duration::from_secs(3))
/// );
/// # }
/// ```
#[non_exhaustive]
#[derive(Clone, PartialEq, Debug)]
pub struct TimeoutConfig {
    connect_timeout: CanDisable<Duration>,
    read_timeout: CanDisable<Duration>,
    operation_timeout: CanDisable<Duration>,
    operation_attempt_timeout: CanDisable<Duration>,
}

impl Storable for TimeoutConfig {
    type Storer = StoreReplace<TimeoutConfig>;
}

/// Merger which merges timeout config settings when loading.
///
/// If no timeouts are set, `TimeoutConfig::disabled()` will be returned.
///
/// This API is not meant to be used externally.
#[derive(Debug)]
pub struct MergeTimeoutConfig;

impl Storable for MergeTimeoutConfig {
    type Storer = MergeTimeoutConfig;
}
impl Store for MergeTimeoutConfig {
    type ReturnedType<'a> = TimeoutConfig;
    type StoredType = <StoreReplace<TimeoutConfig> as Store>::StoredType;

    fn merge_iter(iter: ItemIter<'_, Self>) -> Self::ReturnedType<'_> {
        let mut result: Option<TimeoutConfig> = None;
        // The item iterator iterates "backwards" over the config bags, starting at the highest
        // priority layers and works backwards
        for tc in iter {
            match (result.as_mut(), tc) {
                (Some(result), Value::Set(tc)) => {
                    // This maintains backwards compatible behavior where setting an EMPTY timeout config is equivalent to `TimeoutConfig::disabled()`
                    if result.has_timeouts() {
                        result.take_defaults_from(tc);
                    }
                }
                (None, Value::Set(tc)) => {
                    result = Some(tc.clone());
                }
                (_, Value::ExplicitlyUnset(_)) => result = Some(TimeoutConfig::disabled()),
            }
        }
        result.unwrap_or(TimeoutConfig::disabled())
    }
}

impl TimeoutConfig {
    /// Returns a builder to create a `TimeoutConfig`.
    pub fn builder() -> TimeoutConfigBuilder {
        TimeoutConfigBuilder::new()
    }

    /// Returns a builder equivalent of this `TimeoutConfig`.
    pub fn to_builder(&self) -> TimeoutConfigBuilder {
        TimeoutConfigBuilder::from(self.clone())
    }

    /// Converts this `TimeoutConfig` into a builder.
    pub fn into_builder(self) -> TimeoutConfigBuilder {
        TimeoutConfigBuilder::from(self)
    }

    /// Fill any unfilled values in `self` from `other`.
    pub fn take_defaults_from(&mut self, other: &TimeoutConfig) -> &mut Self {
        self.connect_timeout = self
            .connect_timeout
            .merge_from_lower_priority(other.connect_timeout);
        self.read_timeout = self
            .read_timeout
            .merge_from_lower_priority(other.read_timeout);
        self.operation_timeout = self
            .operation_timeout
            .merge_from_lower_priority(other.operation_timeout);
        self.operation_attempt_timeout = self
            .operation_attempt_timeout
            .merge_from_lower_priority(other.operation_attempt_timeout);
        self
    }

    /// Returns a timeout config with all timeouts disabled.
    pub fn disabled() -> TimeoutConfig {
        TimeoutConfig {
            connect_timeout: CanDisable::Disabled,
            read_timeout: CanDisable::Disabled,
            operation_timeout: CanDisable::Disabled,
            operation_attempt_timeout: CanDisable::Disabled,
        }
    }

    /// Returns this config's connect timeout.
    ///
    /// The connect timeout is a limit on the amount of time it takes to initiate a socket connection.
    pub fn connect_timeout(&self) -> Option<Duration> {
        self.connect_timeout.value()
    }

    /// Returns this config's read timeout.
    ///
    /// The read timeout is the limit on the amount of time it takes to read the first byte of a response
    /// from the time the request is initiated.
    pub fn read_timeout(&self) -> Option<Duration> {
        self.read_timeout.value()
    }

    /// Returns this config's operation timeout.
    ///
    /// An operation represents the full request/response lifecycle of a call to a service.
    /// The operation timeout is a limit on the total amount of time it takes for an operation to be
    /// fully serviced, including the time for all retries that may have been attempted for it.
    pub fn operation_timeout(&self) -> Option<Duration> {
        self.operation_timeout.value()
    }

    /// Returns this config's operation attempt timeout.
    ///
    /// An operation represents the full request/response lifecycle of a call to a service.
    /// When retries are enabled, then this setting makes it possible to set a timeout for individual
    /// retry attempts (including the initial attempt) for an operation.
    pub fn operation_attempt_timeout(&self) -> Option<Duration> {
        self.operation_attempt_timeout.value()
    }

    /// Returns true if any of the possible timeouts are set.
    pub fn has_timeouts(&self) -> bool {
        self.connect_timeout.is_some()
            || self.read_timeout.is_some()
            || self.operation_timeout.is_some()
            || self.operation_attempt_timeout.is_some()
    }
}

/// Configuration subset of [`TimeoutConfig`] for operation timeouts
#[non_exhaustive]
#[derive(Clone, PartialEq, Debug)]
pub struct OperationTimeoutConfig {
    operation_timeout: Option<Duration>,
    operation_attempt_timeout: Option<Duration>,
}

impl OperationTimeoutConfig {
    /// Returns this config's operation timeout.
    ///
    /// An operation represents the full request/response lifecycle of a call to a service.
    /// The operation timeout is a limit on the total amount of time it takes for an operation to be
    /// fully serviced, including the time for all retries that may have been attempted for it.
    pub fn operation_timeout(&self) -> Option<Duration> {
        self.operation_timeout
    }

    /// Returns this config's operation attempt timeout.
    ///
    /// An operation represents the full request/response lifecycle of a call to a service.
    /// When retries are enabled, then this setting makes it possible to set a timeout for individual
    /// retry attempts (including the initial attempt) for an operation.
    pub fn operation_attempt_timeout(&self) -> Option<Duration> {
        self.operation_attempt_timeout
    }

    /// Returns true if any of the possible timeouts are set.
    pub fn has_timeouts(&self) -> bool {
        self.operation_timeout.is_some() || self.operation_attempt_timeout.is_some()
    }
}

impl From<&TimeoutConfig> for OperationTimeoutConfig {
    fn from(cfg: &TimeoutConfig) -> Self {
        OperationTimeoutConfig {
            operation_timeout: cfg.operation_timeout.value(),
            operation_attempt_timeout: cfg.operation_attempt_timeout.value(),
        }
    }
}

impl From<TimeoutConfig> for OperationTimeoutConfig {
    fn from(cfg: TimeoutConfig) -> Self {
        OperationTimeoutConfig::from(&cfg)
    }
}

#[cfg(test)]
mod test {
    use crate::config_bag::{CloneableLayer, ConfigBag};
    use crate::timeout::{MergeTimeoutConfig, TimeoutConfig};
    use std::time::Duration;

    #[test]
    fn timeout_configs_merged_in_config_bag() {
        let mut read_timeout = CloneableLayer::new("timeout");
        read_timeout.store_put(
            TimeoutConfig::builder()
                .read_timeout(Duration::from_secs(3))
                .connect_timeout(Duration::from_secs(1))
                .build(),
        );
        let mut operation_timeout = CloneableLayer::new("timeout");
        operation_timeout.store_put(
            TimeoutConfig::builder()
                .operation_timeout(Duration::from_secs(5))
                .connect_timeout(Duration::from_secs(10))
                .build(),
        );
        let cfg = ConfigBag::of_layers(vec![read_timeout.into(), operation_timeout.into()]);
        let loaded = cfg.load::<MergeTimeoutConfig>();
        // set by base layer
        assert_eq!(loaded.read_timeout(), Some(Duration::from_secs(3)));

        // set by higher layer
        assert_eq!(loaded.operation_timeout(), Some(Duration::from_secs(5)));

        // overridden by higher layer
        assert_eq!(loaded.connect_timeout(), Some(Duration::from_secs(10)));
        let mut next = cfg.add_layer("disabled");
        next.interceptor_state()
            .store_put(TimeoutConfig::disabled());

        assert_eq!(next.load::<MergeTimeoutConfig>().read_timeout(), None);

        // builder().build() acts equivalently to disabled
        next.interceptor_state()
            .store_put(TimeoutConfig::builder().build());
        assert_eq!(next.load::<MergeTimeoutConfig>().read_timeout(), None);

        // But if instead, you set a field of the timeout config, it will merge as expected.
        next.interceptor_state().store_put(
            TimeoutConfig::builder()
                .operation_attempt_timeout(Duration::from_secs(1))
                .build(),
        );
        assert_eq!(
            next.load::<MergeTimeoutConfig>().read_timeout(),
            Some(Duration::from_secs(3))
        );
    }
}
