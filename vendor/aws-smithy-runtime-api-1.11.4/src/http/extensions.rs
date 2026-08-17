/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::http::HttpError;

#[derive(Default, Debug)]
pub(crate) struct Extensions {
    extensions_02x: http_02x::Extensions,
    extensions_1x: http_1x::Extensions,
}

impl Extensions {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Adds an extension to the request extensions
    pub(crate) fn insert<T: Send + Sync + Clone + 'static>(&mut self, extension: T) {
        self.extensions_1x.insert(extension.clone());
        self.extensions_02x.insert(extension);
    }
}

impl From<http_02x::Extensions> for Extensions {
    fn from(value: http_02x::Extensions) -> Self {
        Self {
            extensions_02x: value,
            extensions_1x: Default::default(),
        }
    }
}

impl From<http_1x::Extensions> for Extensions {
    fn from(value: http_1x::Extensions) -> Self {
        Self {
            extensions_02x: Default::default(),
            extensions_1x: value,
        }
    }
}

impl TryFrom<Extensions> for http_02x::Extensions {
    type Error = HttpError;

    fn try_from(value: Extensions) -> Result<Self, Self::Error> {
        if value.extensions_1x.len() > value.extensions_02x.len() {
            Err(HttpError::invalid_extensions())
        } else {
            Ok(value.extensions_02x)
        }
    }
}

impl TryFrom<Extensions> for http_1x::Extensions {
    type Error = HttpError;

    fn try_from(value: Extensions) -> Result<Self, Self::Error> {
        if value.extensions_02x.len() > value.extensions_1x.len() {
            Err(HttpError::invalid_extensions())
        } else {
            Ok(value.extensions_1x)
        }
    }
}
