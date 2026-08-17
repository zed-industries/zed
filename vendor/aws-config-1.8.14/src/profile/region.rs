/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Load a region from an AWS profile

use crate::meta::region::{future, ProvideRegion};
#[allow(deprecated)]
use crate::profile::profile_file::ProfileFiles;
use crate::profile::ProfileSet;
use crate::provider_config::ProviderConfig;
use aws_types::region::Region;

/// Load a region from a profile file
///
/// This provider will attempt to load AWS shared configuration, then read the `region` property
/// from the active profile.
///
#[doc = include_str!("location_of_profile_files.md")]
///
/// # Examples
///
/// **Loads "us-west-2" as the region**
/// ```ini
/// [default]
/// region = us-west-2
/// ```
///
/// **Loads `us-east-1` as the region _if and only if_ the `AWS_PROFILE` environment variable is set
/// to `other`.**
///
/// ```ini
/// [profile other]
/// region = us-east-1
/// ```
///
/// This provider is part of the [default region provider chain](crate::default_provider::region).
#[derive(Debug, Default)]
pub struct ProfileFileRegionProvider {
    provider_config: ProviderConfig,
}

/// Builder for [ProfileFileRegionProvider]
#[derive(Debug, Default)]
pub struct Builder {
    config: Option<ProviderConfig>,
    profile_override: Option<String>,
    #[allow(deprecated)]
    profile_files: Option<ProfileFiles>,
}

impl Builder {
    /// Override the configuration for this provider
    pub fn configure(mut self, config: &ProviderConfig) -> Self {
        self.config = Some(config.clone());
        self
    }

    /// Override the profile name used by the [`ProfileFileRegionProvider`]
    pub fn profile_name(mut self, profile_name: impl Into<String>) -> Self {
        self.profile_override = Some(profile_name.into());
        self
    }

    /// Set the profile file that should be used by the [`ProfileFileRegionProvider`]
    #[allow(deprecated)]
    pub fn profile_files(mut self, profile_files: ProfileFiles) -> Self {
        self.profile_files = Some(profile_files);
        self
    }

    /// Build a [ProfileFileRegionProvider] from this builder
    pub fn build(self) -> ProfileFileRegionProvider {
        let conf = self
            .config
            .unwrap_or_default()
            .with_profile_config(self.profile_files, self.profile_override);
        ProfileFileRegionProvider {
            provider_config: conf,
        }
    }
}

impl ProfileFileRegionProvider {
    /// Create a new [ProfileFileRegionProvider]
    ///
    /// To override the selected profile, set the `AWS_PROFILE` environment variable or use the [`Builder`].
    pub fn new() -> Self {
        Self {
            provider_config: ProviderConfig::default(),
        }
    }

    /// [`Builder`] to construct a [`ProfileFileRegionProvider`]
    pub fn builder() -> Builder {
        Builder::default()
    }

    async fn region(&self) -> Option<Region> {
        let profile_set = self.provider_config.profile().await?;

        resolve_profile_chain_for_region(profile_set)
    }
}

fn resolve_profile_chain_for_region(profile_set: &'_ ProfileSet) -> Option<Region> {
    if profile_set.is_empty() {
        return None;
    }

    let mut selected_profile = profile_set.selected_profile();
    let mut visited_profiles = vec![];

    loop {
        let profile = profile_set.get_profile(selected_profile)?;
        // Check to see if we're in a loop and return if that's true.
        // Else, add the profile we're currently checking to our list of visited profiles.
        if visited_profiles.contains(&selected_profile) {
            return None;
        } else {
            visited_profiles.push(selected_profile);
        }

        // Attempt to get region and source_profile for current profile
        let selected_profile_region = profile
            .get("region")
            .map(|region| Region::new(region.to_owned()));
        let source_profile = profile.get("source_profile");

        // Check to see what we got
        match (selected_profile_region, source_profile) {
            // Profile had a region specified, return it :D
            (Some(region), _) => {
                return Some(region);
            }
            // No region specified, source_profile is self-referential so we return to avoid infinite loop
            (None, Some(source_profile)) if source_profile == selected_profile => {
                return None;
            }
            // No region specified, no source_profile specified so we return empty-handed
            (None, None) => {
                return None;
            }
            // No region specified, check source profile for a region in next loop iteration
            (None, Some(source_profile)) => {
                selected_profile = source_profile;
            }
        }
    }
}

impl ProvideRegion for ProfileFileRegionProvider {
    fn region(&self) -> future::ProvideRegion<'_> {
        future::ProvideRegion::new(self.region())
    }
}

#[cfg(test)]
mod test {
    use crate::profile::ProfileFileRegionProvider;
    use crate::provider_config::ProviderConfig;
    use crate::test_case::no_traffic_client;
    use aws_types::os_shim_internal::{Env, Fs};
    use aws_types::region::Region;
    use futures_util::FutureExt;
    use tracing_test::traced_test;

    fn provider_config(dir_name: &str) -> ProviderConfig {
        let fs = Fs::from_test_dir(format!("test-data/profile-provider/{}/fs", dir_name), "/");
        let env = Env::from_slice(&[("HOME", "/home")]);
        ProviderConfig::empty()
            .with_fs(fs)
            .with_env(env)
            .with_http_client(no_traffic_client())
    }

    #[traced_test]
    #[test]
    fn load_region() {
        let provider = ProfileFileRegionProvider::builder()
            .configure(&provider_config("region_override"))
            .build();
        assert_eq!(
            provider.region().now_or_never().unwrap(),
            Some(Region::from_static("us-east-1"))
        );
    }

    #[test]
    fn load_region_env_profile_override() {
        let conf = provider_config("region_override").with_env(Env::from_slice(&[
            ("HOME", "/home"),
            ("AWS_PROFILE", "base"),
        ]));
        let provider = ProfileFileRegionProvider::builder()
            .configure(&conf)
            .build();
        assert_eq!(
            provider.region().now_or_never().unwrap(),
            Some(Region::from_static("us-east-1"))
        );
    }

    #[test]
    fn load_region_nonexistent_profile() {
        let conf = provider_config("region_override").with_env(Env::from_slice(&[
            ("HOME", "/home"),
            ("AWS_PROFILE", "doesnotexist"),
        ]));
        let provider = ProfileFileRegionProvider::builder()
            .configure(&conf)
            .build();
        assert_eq!(provider.region().now_or_never().unwrap(), None);
    }

    #[test]
    fn load_region_explicit_override() {
        let conf = provider_config("region_override");
        let provider = ProfileFileRegionProvider::builder()
            .configure(&conf)
            .profile_name("base")
            .build();
        assert_eq!(
            provider.region().now_or_never().unwrap(),
            Some(Region::from_static("us-east-1"))
        );
    }

    #[tokio::test]
    async fn load_region_from_source_profile() {
        let config = r#"
[profile credentials]
aws_access_key_id = test-access-key-id
aws_secret_access_key = test-secret-access-key
aws_session_token = test-session-token
region = us-east-1

[profile needs-source]
source_profile = credentials
role_arn = arn:aws:iam::123456789012:role/test
"#
        .trim();

        let fs = Fs::from_slice(&[("test_config", config)]);
        let env = Env::from_slice(&[("AWS_CONFIG_FILE", "test_config")]);
        let provider_config = ProviderConfig::empty()
            .with_fs(fs)
            .with_env(env)
            .with_http_client(no_traffic_client());

        assert_eq!(
            Some(Region::new("us-east-1")),
            ProfileFileRegionProvider::builder()
                .profile_name("needs-source")
                .configure(&provider_config)
                .build()
                .region()
                .await
        );
    }
}
