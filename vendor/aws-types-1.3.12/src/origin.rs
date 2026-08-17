/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Types for tracking the origin of config values.

use std::cmp::Ordering;
use std::fmt;

/// A type for tracking the origin of config values.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[non_exhaustive]
pub struct Origin {
    inner: Inner,
}

impl fmt::Display for Origin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Inner::*;

        match self.inner {
            Imds => write!(f, "IMDS"),
            ProfileFile(Kind::Shared) => write!(f, "shared profile file"),
            ProfileFile(Kind::Service) => write!(f, "service profile file"),
            EnvironmentVariable(Kind::Shared) => write!(f, "shared environment variable"),
            EnvironmentVariable(Kind::Service) => write!(f, "service environment variable"),
            Programmatic(Kind::Shared) => write!(f, "shared client"),
            Programmatic(Kind::Service) => write!(f, "service client"),
            Unknown => write!(f, "unknown"),
        }
    }
}

impl Origin {
    /// The origin is unknown.
    pub fn unknown() -> Self {
        Self {
            inner: Inner::Unknown,
        }
    }

    /// Set with IMDS.
    pub fn imds() -> Self {
        Self { inner: Inner::Imds }
    }

    /// Set on a shared config struct.
    pub fn shared_config() -> Self {
        Self {
            inner: Inner::Programmatic(Kind::Shared),
        }
    }

    /// Set on a service config struct.
    pub fn service_config() -> Self {
        Self {
            inner: Inner::Programmatic(Kind::Service),
        }
    }

    /// Set by an environment variable.
    pub fn shared_environment_variable() -> Self {
        Self {
            inner: Inner::EnvironmentVariable(Kind::Shared),
        }
    }

    /// Set by a service-specific environment variable.
    pub fn service_environment_variable() -> Self {
        Self {
            inner: Inner::EnvironmentVariable(Kind::Service),
        }
    }

    /// Set in a profile file.
    pub fn shared_profile_file() -> Self {
        Self {
            inner: Inner::ProfileFile(Kind::Shared),
        }
    }

    /// Service-specific, set in a profile file.
    pub fn service_profile_file() -> Self {
        Self {
            inner: Inner::ProfileFile(Kind::Service),
        }
    }

    /// Return true if the origin was set programmatically i.e. on an `SdkConfig` or service `Config`.
    pub fn is_client_config(&self) -> bool {
        matches!(
            self,
            Origin {
                inner: Inner::Programmatic(..),
                ..
            }
        )
    }
}

impl Default for Origin {
    fn default() -> Self {
        Self::unknown()
    }
}

#[derive(Debug, Clone, Copy)]
enum Inner {
    Imds,
    ProfileFile(Kind),
    EnvironmentVariable(Kind),
    Programmatic(Kind),
    Unknown,
}

impl Inner {
    pub(self) fn is_unknown(&self) -> bool {
        matches!(self, Inner::Unknown)
    }
}

// Unknown is like NaN. It's not equal to anything, not even itself.
impl PartialEq for Inner {
    fn eq(&self, other: &Self) -> bool {
        use Inner::*;

        match (self, other) {
            (Imds, Imds) => true,
            (Programmatic(a), Programmatic(b)) => a == b,
            (EnvironmentVariable(a), EnvironmentVariable(b)) => a == b,
            (ProfileFile(a), ProfileFile(b)) => a == b,
            _ => false,
        }
    }
}

impl PartialOrd for Inner {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use Inner::*;

        if self.is_unknown() || other.is_unknown() {
            return None;
        }

        match self {
            // IMDS is the lowest priority
            Imds => Some(Ordering::Less),
            // ProfileFile is the second-lowest priority
            ProfileFile(kind) => match other {
                Imds => Some(Ordering::Greater),
                ProfileFile(other_kind) => kind.partial_cmp(other_kind),
                _ => Some(Ordering::Less),
            },
            // EnvironmentVariable is the second-highest priority
            EnvironmentVariable(kind) => match other {
                Imds | ProfileFile(_) => Some(Ordering::Greater),
                EnvironmentVariable(other_kind) => kind.partial_cmp(other_kind),
                _ => Some(Ordering::Less),
            },
            // Programmatic is the highest priority
            Programmatic(kind) => match other {
                Imds | EnvironmentVariable(_) | ProfileFile(_) => Some(Ordering::Greater),
                Programmatic(other_kind) => kind.partial_cmp(other_kind),
                _ => unreachable!(
                    "When we have something higher than programmatic we can update this case."
                ),
            },
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
enum Kind {
    Shared,
    Service,
}

#[cfg(test)]
mod test {
    use super::Origin;

    #[test]
    fn test_precedence_low_to_high() {
        // Lowest to highest precedence
        let list = [
            Origin::imds(),
            Origin::shared_profile_file(),
            Origin::service_profile_file(),
            Origin::shared_environment_variable(),
            Origin::service_environment_variable(),
            Origin::shared_config(),
            Origin::service_config(),
        ];

        for window in list.windows(2) {
            let &[a, b] = window else { unreachable!() };
            assert!(a < b);
        }
    }

    #[test]
    fn test_precedence_high_to_low() {
        // Highest to lowest precedence
        let list = [
            Origin::service_config(),
            Origin::shared_config(),
            Origin::service_environment_variable(),
            Origin::shared_environment_variable(),
            Origin::service_profile_file(),
            Origin::shared_profile_file(),
            Origin::imds(),
        ];

        for window in list.windows(2) {
            let &[a, b] = window else { unreachable!() };
            assert!(a > b);
        }
    }

    #[test]
    fn test_unknown_is_not_equal() {
        assert_ne!(Origin::unknown(), Origin::imds());
        assert_ne!(Origin::unknown(), Origin::shared_config());
        assert_ne!(Origin::unknown(), Origin::service_config());
        assert_ne!(Origin::unknown(), Origin::shared_environment_variable());
        assert_ne!(Origin::unknown(), Origin::service_environment_variable());
        assert_ne!(Origin::unknown(), Origin::shared_profile_file());
        assert_ne!(Origin::unknown(), Origin::service_profile_file());
        assert_ne!(Origin::unknown(), Origin::unknown());
    }

    #[test]
    fn test_self_equality() {
        assert_eq!(Origin::imds(), Origin::imds());
        assert_eq!(Origin::shared_config(), Origin::shared_config());
        assert_eq!(Origin::service_config(), Origin::service_config());
        assert_eq!(
            Origin::shared_environment_variable(),
            Origin::shared_environment_variable()
        );
        assert_eq!(
            Origin::service_environment_variable(),
            Origin::service_environment_variable()
        );
        assert_eq!(Origin::shared_profile_file(), Origin::shared_profile_file());
        assert_eq!(
            Origin::service_profile_file(),
            Origin::service_profile_file()
        );
    }
}
