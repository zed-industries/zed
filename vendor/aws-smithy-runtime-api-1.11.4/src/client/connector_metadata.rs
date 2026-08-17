/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! New-type for a configurable app name.

use std::borrow::Cow;
use std::fmt;

/// The name of the crate that provides the HTTP connectors and its version.
///
/// This should be set by the connector's runtime plugin. Note that this is for
/// the **connector** returned by an HTTP client, not the HTTP client itself.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConnectorMetadata {
    name: Cow<'static, str>,
    version: Option<Cow<'static, str>>,
}

impl ConnectorMetadata {
    /// Create a new [`ConnectorMetadata`].
    pub fn new(name: impl Into<Cow<'static, str>>, version: Option<Cow<'static, str>>) -> Self {
        Self {
            name: name.into(),
            version,
        }
    }

    /// Return the name of the crate backing a connector.
    pub fn name(&self) -> Cow<'static, str> {
        self.name.clone()
    }

    /// Return the version of the crate backing a connector.
    pub fn version(&self) -> Option<Cow<'static, str>> {
        self.version.clone()
    }
}

impl fmt::Display for ConnectorMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "http#{}", self.name)?;
        if let Some(version) = self.version.as_deref() {
            write!(f, "-{version}")?;
        }

        Ok(())
    }
}
