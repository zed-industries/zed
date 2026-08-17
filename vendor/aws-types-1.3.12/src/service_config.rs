/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Code for extracting service config from the user's environment.

use std::fmt;

/// A struct used with the [`LoadServiceConfig`] trait to extract service config from the user's environment.
// [profile active-profile]
// services = dev
//
// [services dev]
// service-id =
//   config-key = config-value
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ServiceConfigKey<'a> {
    service_id: &'a str,
    profile: &'a str,
    env: &'a str,
}

impl<'a> ServiceConfigKey<'a> {
    /// Create a new [`ServiceConfigKey`] builder struct.
    pub fn builder() -> builder::Builder<'a> {
        Default::default()
    }
    /// Get the service ID.
    pub fn service_id(&self) -> &'a str {
        self.service_id
    }
    /// Get the profile key.
    pub fn profile(&self) -> &'a str {
        self.profile
    }
    /// Get the environment key.
    pub fn env(&self) -> &'a str {
        self.env
    }
}

pub mod builder {
    //! Builder for [`ServiceConfigKey`].

    use super::ServiceConfigKey;
    use std::fmt;

    /// Builder for [`ServiceConfigKey`].
    #[derive(Default, Debug)]
    pub struct Builder<'a> {
        service_id: Option<&'a str>,
        profile: Option<&'a str>,
        env: Option<&'a str>,
    }

    impl<'a> Builder<'a> {
        /// Set the service ID.
        pub fn service_id(mut self, service_id: &'a str) -> Self {
            self.service_id = Some(service_id);
            self
        }

        /// Set the profile key.
        pub fn profile(mut self, profile: &'a str) -> Self {
            self.profile = Some(profile);
            self
        }

        /// Set the environment key.
        pub fn env(mut self, env: &'a str) -> Self {
            self.env = Some(env);
            self
        }

        /// Build the [`ServiceConfigKey`].
        ///
        /// Returns an error if any of the required fields are missing.
        pub fn build(self) -> Result<ServiceConfigKey<'a>, Error> {
            Ok(ServiceConfigKey {
                service_id: self.service_id.ok_or_else(Error::missing_service_id)?,
                profile: self.profile.ok_or_else(Error::missing_profile)?,
                env: self.env.ok_or_else(Error::missing_env)?,
            })
        }
    }

    #[allow(clippy::enum_variant_names)]
    #[derive(Debug)]
    enum ErrorKind {
        MissingServiceId,
        MissingProfile,
        MissingEnv,
    }

    impl fmt::Display for ErrorKind {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                ErrorKind::MissingServiceId => write!(f, "missing required service-id"),
                ErrorKind::MissingProfile => write!(f, "missing required active profile name"),
                ErrorKind::MissingEnv => write!(f, "missing required environment variable name"),
            }
        }
    }

    /// Error type for [`ServiceConfigKey::builder`]
    #[derive(Debug)]
    pub struct Error {
        kind: ErrorKind,
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "couldn't build a ServiceEnvConfigKey: {}", self.kind)
        }
    }

    impl std::error::Error for Error {}

    impl Error {
        /// Create a new "missing service ID" error
        pub fn missing_service_id() -> Self {
            Self {
                kind: ErrorKind::MissingServiceId,
            }
        }
        /// Create a new "missing profile key" error
        pub fn missing_profile() -> Self {
            Self {
                kind: ErrorKind::MissingProfile,
            }
        }
        /// Create a new "missing env key" error
        pub fn missing_env() -> Self {
            Self {
                kind: ErrorKind::MissingEnv,
            }
        }
    }
}

/// Implementers of this trait can provide service config defined in a user's environment.
pub trait LoadServiceConfig: fmt::Debug + Send + Sync {
    /// Given a [`ServiceConfigKey`], return the value associated with it.
    fn load_config(&self, key: ServiceConfigKey<'_>) -> Option<String>;
}
