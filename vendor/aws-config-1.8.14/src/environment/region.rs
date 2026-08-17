/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::meta::region::{future, ProvideRegion};
use aws_types::os_shim_internal::Env;
use aws_types::region::Region;

/// Load a region from environment variables
///
/// This provider will first check the value of `AWS_REGION`, falling back to `AWS_DEFAULT_REGION`
/// when `AWS_REGION` is unset.
#[derive(Debug, Default)]
pub struct EnvironmentVariableRegionProvider {
    env: Env,
}

impl EnvironmentVariableRegionProvider {
    /// Create a new `EnvironmentVariableRegionProvider`
    pub fn new() -> Self {
        EnvironmentVariableRegionProvider { env: Env::real() }
    }

    /// Create an region provider from a given `Env`
    ///
    /// This method is used for tests that need to override environment variables.
    pub(crate) fn new_with_env(env: Env) -> Self {
        EnvironmentVariableRegionProvider { env }
    }
}

impl ProvideRegion for EnvironmentVariableRegionProvider {
    fn region(&self) -> future::ProvideRegion<'_> {
        let region = self
            .env
            .get("AWS_REGION")
            .or_else(|_| self.env.get("AWS_DEFAULT_REGION"))
            .map(Region::new)
            .ok();
        future::ProvideRegion::ready(region)
    }
}
#[cfg(test)]
mod test {
    use crate::environment::region::EnvironmentVariableRegionProvider;
    use crate::meta::region::ProvideRegion;
    use aws_types::os_shim_internal::Env;
    use aws_types::region::Region;
    use futures_util::FutureExt;

    fn test_provider(vars: &[(&str, &str)]) -> EnvironmentVariableRegionProvider {
        EnvironmentVariableRegionProvider::new_with_env(Env::from_slice(vars))
    }

    #[test]
    fn no_region() {
        assert_eq!(
            test_provider(&[])
                .region()
                .now_or_never()
                .expect("no polling"),
            None
        );
    }

    #[test]
    fn prioritize_aws_region() {
        let provider = test_provider(&[
            ("AWS_REGION", "us-east-1"),
            ("AWS_DEFAULT_REGION", "us-east-2"),
        ]);
        assert_eq!(
            provider.region().now_or_never().expect("no polling"),
            Some(Region::new("us-east-1"))
        );
    }

    #[test]
    fn fallback_to_default_region() {
        assert_eq!(
            test_provider(&[("AWS_DEFAULT_REGION", "us-east-2")])
                .region()
                .now_or_never()
                .expect("no polling"),
            Some(Region::new("us-east-2"))
        );
    }
}
