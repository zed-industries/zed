/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Sections within an AWS config profile.

use std::collections::HashMap;
use std::fmt;

/// Key-Value property pair
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Property {
    key: String,
    value: String,
}

impl Property {
    /// Value of this property
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Name of this property
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Creates a new property
    pub fn new(key: String, value: String) -> Self {
        Property { key, value }
    }
}

type SectionKey = String;
type SectionName = String;
type PropertyName = String;
type SubPropertyName = String;
type PropertyValue = String;

/// A key for to a property value.
///
/// ```txt
/// # An example AWS profile config section with properties and sub-properties
/// [section-key section-name]
/// property-name = property-value
/// property-name =
///   sub-property-name = property-value
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PropertiesKey {
    section_key: SectionKey,
    section_name: SectionName,
    property_name: PropertyName,
    sub_property_name: Option<SubPropertyName>,
}

impl PropertiesKey {
    /// Create a new [`PropertiesKeyBuilder`].
    pub fn builder() -> PropertiesKeyBuilder {
        Default::default()
    }
}

impl fmt::Display for PropertiesKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let PropertiesKey {
            section_key,
            section_name,
            property_name,
            sub_property_name,
        } = self;
        match sub_property_name {
            Some(sub_property_name) => {
                write!(
                    f,
                    "[{section_key} {section_name}].{property_name}.{sub_property_name}"
                )
            }
            None => {
                write!(f, "[{section_key} {section_name}].{property_name}")
            }
        }
    }
}

/// Builder for [`PropertiesKey`]s.
#[derive(Debug, Default)]
pub struct PropertiesKeyBuilder {
    section_key: Option<SectionKey>,
    section_name: Option<SectionName>,
    property_name: Option<PropertyName>,
    sub_property_name: Option<SubPropertyName>,
}

impl PropertiesKeyBuilder {
    /// Set the section key for this builder.
    pub fn section_key(mut self, section_key: impl Into<String>) -> Self {
        self.section_key = Some(section_key.into());
        self
    }

    /// Set the section name for this builder.
    pub fn section_name(mut self, section_name: impl Into<String>) -> Self {
        self.section_name = Some(section_name.into());
        self
    }

    /// Set the property name for this builder.
    pub fn property_name(mut self, property_name: impl Into<String>) -> Self {
        self.property_name = Some(property_name.into());
        self
    }

    /// Set the sub-property name for this builder.
    pub fn sub_property_name(mut self, sub_property_name: impl Into<String>) -> Self {
        self.sub_property_name = Some(sub_property_name.into());
        self
    }

    /// Build this builder. If all required fields are set,
    /// `Ok(PropertiesKey)` is returned. Otherwise, an error is returned.
    pub fn build(self) -> Result<PropertiesKey, String> {
        Ok(PropertiesKey {
            section_key: self
                .section_key
                .ok_or("A section_key is required".to_owned())?,
            section_name: self
                .section_name
                .ok_or("A section_name is required".to_owned())?,
            property_name: self
                .property_name
                .ok_or("A property_name is required".to_owned())?,
            sub_property_name: self.sub_property_name,
        })
    }
}

/// A map of [`PropertiesKey`]s to property values.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Properties {
    inner: HashMap<PropertiesKey, PropertyValue>,
}

#[allow(dead_code)]
impl Properties {
    /// Create a new empty [`Properties`].
    pub fn new() -> Self {
        Default::default()
    }

    #[cfg(test)]
    pub(crate) fn new_from_slice(slice: &[(PropertiesKey, PropertyValue)]) -> Self {
        let mut properties = Self::new();
        for (key, value) in slice {
            properties.insert(key.clone(), value.clone());
        }
        properties
    }

    /// Insert a new key/value pair into this map.
    pub fn insert(&mut self, properties_key: PropertiesKey, value: PropertyValue) {
        let _ = self
            .inner
            // If we don't clone then we don't get to log a useful warning for a value getting overwritten.
            .entry(properties_key.clone())
            .and_modify(|v| {
                tracing::trace!("overwriting {properties_key}: was {v}, now {value}");
                v.clone_from(&value);
            })
            .or_insert(value);
    }

    /// Given a [`PropertiesKey`], return the corresponding value, if any.
    pub fn get(&self, properties_key: &PropertiesKey) -> Option<&PropertyValue> {
        self.inner.get(properties_key)
    }
}
