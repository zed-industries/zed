/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

/// Smithy retry classifiers.
pub mod classifiers;

/// Smithy retry strategies.
pub mod strategy;

mod client_rate_limiter;
mod token_bucket;

use aws_smithy_types::config_bag::{Storable, StoreReplace};
use std::fmt;

pub use client_rate_limiter::{
    ClientRateLimiter, ClientRateLimiterBuilder, ClientRateLimiterPartition,
};
pub use token_bucket::{TokenBucket, TokenBucketBuilder, MAXIMUM_CAPACITY};

use std::borrow::Cow;

/// Represents the retry partition, e.g. an endpoint, a region
///
/// A retry partition created with [`RetryPartition::new`] uses built-in
/// token bucket and rate limiter settings, with no option for customization.
/// Default partitions with the same name share the same token bucket
/// and client rate limiter.
///
/// To customize these components, use a custom retry partition via [`RetryPartition::custom`].
/// A custom partition owns its token bucket and rate limiter, which:
/// - Are independent from those in any default partition.
/// - Are not shared with other custom partitions, even if they have the same name.
///
/// To share a token bucket and rate limiter among custom partitions,
/// either clone the custom partition itself or clone these components
/// beforehand and pass them to each custom partition.
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct RetryPartition {
    pub(crate) inner: RetryPartitionInner,
}

#[derive(Clone, Debug)]
pub(crate) enum RetryPartitionInner {
    Default(Cow<'static, str>),
    Custom {
        name: Cow<'static, str>,
        token_bucket: TokenBucket,
        client_rate_limiter: ClientRateLimiter,
    },
}

impl RetryPartition {
    /// Creates a new `RetryPartition` from the given `name`.
    pub fn new(name: impl Into<Cow<'static, str>>) -> Self {
        Self {
            inner: RetryPartitionInner::Default(name.into()),
        }
    }

    /// Creates a builder for a custom `RetryPartition`.
    pub fn custom(name: impl Into<Cow<'static, str>>) -> RetryPartitionBuilder {
        RetryPartitionBuilder {
            name: name.into(),
            token_bucket: None,
            client_rate_limiter: None,
        }
    }

    fn name(&self) -> &str {
        match &self.inner {
            RetryPartitionInner::Default(name) => name,
            RetryPartitionInner::Custom { name, .. } => name,
        }
    }
}

impl PartialEq for RetryPartition {
    fn eq(&self, other: &Self) -> bool {
        match (&self.inner, &other.inner) {
            (RetryPartitionInner::Default(name1), RetryPartitionInner::Default(name2)) => {
                name1 == name2
            }
            (
                RetryPartitionInner::Custom { name: name1, .. },
                RetryPartitionInner::Custom { name: name2, .. },
            ) => name1 == name2,
            // Different variants: not equal
            _ => false,
        }
    }
}

impl Eq for RetryPartition {}

impl std::hash::Hash for RetryPartition {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match &self.inner {
            RetryPartitionInner::Default(name) => {
                // Hash discriminant for Default variant
                0u8.hash(state);
                name.hash(state);
            }
            RetryPartitionInner::Custom { name, .. } => {
                // Hash discriminant for Configured variant
                1u8.hash(state);
                name.hash(state);
            }
        }
    }
}

impl fmt::Display for RetryPartition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl Storable for RetryPartition {
    type Storer = StoreReplace<RetryPartition>;
}

/// Builder for creating custom retry partitions.
pub struct RetryPartitionBuilder {
    name: Cow<'static, str>,
    token_bucket: Option<TokenBucket>,
    client_rate_limiter: Option<ClientRateLimiter>,
}

impl RetryPartitionBuilder {
    /// Sets the token bucket.
    pub fn token_bucket(mut self, token_bucket: TokenBucket) -> Self {
        self.token_bucket = Some(token_bucket);
        self
    }

    /// Sets the client rate limiter.
    pub fn client_rate_limiter(mut self, client_rate_limiter: ClientRateLimiter) -> Self {
        self.client_rate_limiter = Some(client_rate_limiter);
        self
    }

    /// Builds the custom retry partition.
    pub fn build(self) -> RetryPartition {
        RetryPartition {
            inner: RetryPartitionInner::Custom {
                name: self.name,
                token_bucket: self.token_bucket.unwrap_or_default(),
                client_rate_limiter: self.client_rate_limiter.unwrap_or_default(),
            },
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    fn hash_value<T: Hash>(t: &T) -> u64 {
        let mut hasher = DefaultHasher::new();
        t.hash(&mut hasher);
        hasher.finish()
    }

    #[test]
    fn test_retry_partition_equality() {
        let default1 = RetryPartition::new("test");
        let default2 = RetryPartition::new("test");
        let default3 = RetryPartition::new("other");

        let configured1 = RetryPartition::custom("test").build();
        let configured2 = RetryPartition::custom("test").build();
        let configured3 = RetryPartition::custom("other").build();

        // Same variant, same name
        assert_eq!(default1, default2);
        assert_eq!(configured1, configured2);

        // Same variant, different name
        assert_ne!(default1, default3);
        assert_ne!(configured1, configured3);

        // Different variant, same name
        assert_ne!(default1, configured1);
    }

    #[test]
    fn test_retry_partition_hash() {
        let default = RetryPartition::new("test");
        let configured = RetryPartition::custom("test").build();

        // Different variants with same name should have different hashes
        assert_ne!(hash_value(&default), hash_value(&configured));

        // Same variants with same name should have same hashes
        let default2 = RetryPartition::new("test");
        assert_eq!(hash_value(&default), hash_value(&default2));
    }
}
