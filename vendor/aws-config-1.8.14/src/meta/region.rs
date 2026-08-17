/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Region providers that augment existing providers with new functionality

use aws_types::region::Region;
use std::borrow::Cow;
use std::fmt::Debug;
use tracing::Instrument;

/// Load a region by selecting the first from a series of region providers.
///
/// # Examples
///
/// ```no_run
/// # fn example() {
/// use aws_types::region::Region;
/// use std::env;
/// use aws_config::meta::region::RegionProviderChain;
///
/// // region provider that first checks the `CUSTOM_REGION` environment variable,
/// // then checks the default provider chain, then falls back to us-east-2
/// let provider = RegionProviderChain::first_try(env::var("CUSTOM_REGION").ok().map(Region::new))
///     .or_default_provider()
///     .or_else(Region::new("us-east-2"));
/// # }
/// ```
#[derive(Debug)]
pub struct RegionProviderChain {
    providers: Vec<Box<dyn ProvideRegion>>,
}

impl RegionProviderChain {
    /// Load a region from the provider chain
    ///
    /// The first provider to return a non-optional region will be selected
    pub async fn region(&self) -> Option<Region> {
        for provider in &self.providers {
            if let Some(region) = provider
                .region()
                .instrument(tracing::debug_span!("region_provider_chain", provider = ?provider))
                .await
            {
                return Some(region);
            }
        }
        None
    }

    /// Create a default provider chain that starts by checking this provider.
    pub fn first_try(provider: impl ProvideRegion + 'static) -> Self {
        RegionProviderChain {
            providers: vec![Box::new(provider)],
        }
    }

    /// Add a fallback provider to the region provider chain.
    pub fn or_else(mut self, fallback: impl ProvideRegion + 'static) -> Self {
        self.providers.push(Box::new(fallback));
        self
    }

    /// Create a region provider chain that starts by checking the default provider.
    pub fn default_provider() -> Self {
        Self::first_try(crate::default_provider::region::default_provider())
    }

    /// Fallback to the default provider
    pub fn or_default_provider(mut self) -> Self {
        self.providers
            .push(Box::new(crate::default_provider::region::default_provider()));
        self
    }
}

impl ProvideRegion for Option<Region> {
    fn region(&self) -> future::ProvideRegion<'_> {
        future::ProvideRegion::ready(self.clone())
    }
}

impl ProvideRegion for RegionProviderChain {
    fn region(&self) -> future::ProvideRegion<'_> {
        future::ProvideRegion::new(RegionProviderChain::region(self))
    }
}

/// Future wrapper returned by [`ProvideRegion`]
///
/// Note: this module should only be used when implementing your own region providers.
pub mod future {
    use std::future::Future;
    use std::pin::Pin;
    use std::task::{Context, Poll};

    use aws_smithy_async::future::now_or_later::NowOrLater;

    use aws_types::region::Region;

    type BoxFuture<'a> = Pin<Box<dyn Future<Output = Option<Region>> + Send + 'a>>;
    /// Future returned by [`ProvideRegion`](super::ProvideRegion)
    ///
    /// - When wrapping an already loaded region, use [`ready`](ProvideRegion::ready).
    /// - When wrapping an asynchronously loaded region, use [`new`](ProvideRegion::new).
    #[derive(Debug)]
    pub struct ProvideRegion<'a>(NowOrLater<Option<Region>, BoxFuture<'a>>);
    impl<'a> ProvideRegion<'a> {
        /// A future that wraps the given future
        pub fn new(future: impl Future<Output = Option<Region>> + Send + 'a) -> Self {
            Self(NowOrLater::new(Box::pin(future)))
        }

        /// A future that resolves to a given region
        pub fn ready(region: Option<Region>) -> Self {
            Self(NowOrLater::ready(region))
        }
    }

    impl Future for ProvideRegion<'_> {
        type Output = Option<Region>;

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            Pin::new(&mut self.0).poll(cx)
        }
    }
}

/// Provide a [`Region`] to use with AWS requests
///
/// For most cases [`default_provider`](crate::default_provider::region::default_provider) will be the best option, implementing
/// a standard provider chain.
pub trait ProvideRegion: Send + Sync + Debug {
    /// Load a region from this provider
    fn region(&self) -> future::ProvideRegion<'_>;
}

impl ProvideRegion for Region {
    fn region(&self) -> future::ProvideRegion<'_> {
        future::ProvideRegion::ready(Some(self.clone()))
    }
}

impl ProvideRegion for &Region {
    fn region(&self) -> future::ProvideRegion<'_> {
        future::ProvideRegion::ready(Some((*self).clone()))
    }
}

impl ProvideRegion for Box<dyn ProvideRegion> {
    fn region(&self) -> future::ProvideRegion<'_> {
        self.as_ref().region()
    }
}

impl ProvideRegion for &'static str {
    fn region(&self) -> future::ProvideRegion<'_> {
        future::ProvideRegion::ready(Some(Region::new(Cow::Borrowed(*self))))
    }
}

#[cfg(test)]
mod test {
    use crate::meta::region::RegionProviderChain;
    use aws_types::region::Region;
    use futures_util::FutureExt;

    #[test]
    fn provider_chain() {
        let a = None;
        let b = Some(Region::new("us-east-1"));
        let chain = RegionProviderChain::first_try(a).or_else(b);
        assert_eq!(
            chain.region().now_or_never().expect("ready"),
            Some(Region::new("us-east-1"))
        );
    }

    #[test]
    fn empty_chain() {
        let chain = RegionProviderChain::first_try(None).or_else(None);
        assert_eq!(chain.region().now_or_never().expect("ready"), None);
    }
}
