/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Attributes (also referred to as tags or annotations in other telemetry systems) are structured
//! key-value pairs that annotate a span or event. Structured data allows observability backends
//! to index and process telemetry data in ways that simple log messages lack.

use std::collections::HashMap;

/// The valid types of values accepted by [Attributes].
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq)]
pub enum AttributeValue {
    /// Holds an [i64]
    I64(i64),
    /// Holds an [f64]
    F64(f64),
    /// Holds a [String]
    String(String),
    /// Holds a [bool]
    Bool(bool),
}

/// Structured telemetry metadata.
#[non_exhaustive]
#[derive(Clone, Debug, Default)]
pub struct Attributes {
    attrs: HashMap<String, AttributeValue>,
}

impl Attributes {
    /// Create a new empty instance of [Attributes].
    pub fn new() -> Self {
        Self::default()
    }

    /// Set an attribute.
    pub fn set(&mut self, key: impl Into<String>, value: impl Into<AttributeValue>) {
        self.attrs.insert(key.into(), value.into());
    }

    /// Get an attribute.
    pub fn get(&self, key: impl Into<String>) -> Option<&AttributeValue> {
        self.attrs.get(&key.into())
    }

    /// Get all of the attribute key value pairs.
    pub fn attributes(&self) -> &HashMap<String, AttributeValue> {
        &self.attrs
    }

    /// Get an owned [Iterator] of ([String], [AttributeValue]).
    pub fn into_attributes(self) -> impl Iterator<Item = (String, AttributeValue)> {
        self.attrs.into_iter()
    }
}
