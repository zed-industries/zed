/*
 *  Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 *  SPDX-License-Identifier: Apache-2.0
 */
//! Smithy Endpoint Types

use crate::config_bag::{Storable, StoreReplace};
use crate::Document;
use std::borrow::Cow;
use std::collections::HashMap;

type MaybeStatic = Cow<'static, str>;

/* ANCHOR: endpoint */
/// Smithy Endpoint Type
///
/// Generally, this type should not be used from user code
#[derive(Debug, Clone, PartialEq)]
pub struct Endpoint {
    url: MaybeStatic,
    headers: HashMap<MaybeStatic, Vec<MaybeStatic>>,
    properties: HashMap<MaybeStatic, Document>,
}

/* ANCHOR_END: endpoint */

#[allow(unused)]
impl Endpoint {
    /// Returns the URL of this endpoint
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Returns the headers associated with this endpoint
    pub fn headers(&self) -> impl Iterator<Item = (&str, impl Iterator<Item = &str>)> {
        self.headers
            .iter()
            .map(|(k, v)| (k.as_ref(), v.iter().map(|v| v.as_ref())))
    }

    /// Returns the properties associated with this endpoint
    pub fn properties(&self) -> &HashMap<Cow<'static, str>, Document> {
        &self.properties
    }

    /// Converts this endpoint back into a [`Builder`]
    pub fn into_builder(self) -> Builder {
        Builder { endpoint: self }
    }

    /// A builder for [`Endpoint`]
    pub fn builder() -> Builder {
        Builder::new()
    }
}

impl Storable for Endpoint {
    type Storer = StoreReplace<Self>;
}

#[derive(Debug, Clone)]
/// Builder for [`Endpoint`]
pub struct Builder {
    endpoint: Endpoint,
}

#[allow(unused)]
impl Builder {
    pub(crate) fn new() -> Self {
        Self {
            endpoint: Endpoint {
                url: Default::default(),
                headers: HashMap::new(),
                properties: HashMap::new(),
            },
        }
    }

    /// Set the URL of the Endpoint
    ///
    /// # Examples
    /// ```rust
    /// use aws_smithy_types::endpoint::Endpoint;
    /// let endpoint = Endpoint::builder().url("https://www.example.com").build();
    /// ```
    pub fn url(mut self, url: impl Into<MaybeStatic>) -> Self {
        self.endpoint.url = url.into();
        self
    }

    /// Adds a header to the endpoint
    ///
    /// If there is already a header for this key, this header will be appended to that key
    ///
    /// # Examples
    /// ```rust
    /// use aws_smithy_types::endpoint::Endpoint;
    /// let endpoint = Endpoint::builder().url("https://www.example.com").header("x-my-header", "hello").build();
    /// ```
    pub fn header(mut self, name: impl Into<MaybeStatic>, value: impl Into<MaybeStatic>) -> Self {
        self.endpoint
            .headers
            .entry(name.into())
            .or_default()
            .push(value.into());
        self
    }

    /// Adds a property to the endpoint
    ///
    /// If there is already a property for this key, the existing property will be overwritten
    ///
    /// # Examples
    /// ```rust
    /// use aws_smithy_types::endpoint::Endpoint;
    /// let endpoint = Endpoint::builder()
    ///   .url("https://www.example.com")
    ///   .property("x-my-header", true)
    ///   .build();
    /// ```
    pub fn property(mut self, key: impl Into<MaybeStatic>, value: impl Into<Document>) -> Self {
        self.endpoint.properties.insert(key.into(), value.into());
        self
    }

    /// Constructs an [`Endpoint`] from this builder
    ///
    /// # Panics
    /// Panics if URL is unset or empty
    pub fn build(self) -> Endpoint {
        assert_ne!(self.endpoint.url(), "", "URL was unset");
        self.endpoint
    }
}

#[cfg(test)]
mod test {
    use crate::endpoint::Endpoint;
    use crate::Document;
    use std::borrow::Cow;
    use std::collections::HashMap;

    #[test]
    fn endpoint_builder() {
        let endpoint = Endpoint::builder()
            .url("https://www.amazon.com")
            .header("x-amz-test", "header-value")
            .property("custom", Document::Bool(true))
            .build();
        assert_eq!(endpoint.url, Cow::Borrowed("https://www.amazon.com"));
        assert_eq!(
            endpoint.headers,
            HashMap::from([(
                Cow::Borrowed("x-amz-test"),
                vec![Cow::Borrowed("header-value")]
            )])
        );
        assert_eq!(
            endpoint.properties,
            HashMap::from([(Cow::Borrowed("custom"), Document::Bool(true))])
        );

        assert_eq!(endpoint.url(), "https://www.amazon.com");
        assert_eq!(
            endpoint
                .headers()
                .map(|(k, v)| (k, v.collect::<Vec<_>>()))
                .collect::<Vec<_>>(),
            vec![("x-amz-test", vec!["header-value"])]
        );
    }

    #[test]
    fn borrowed_values() {
        fn foo(a: &str) {
            // borrowed values without a static lifetime need to be converted into owned values
            let endpoint = Endpoint::builder().url(a.to_string()).build();
            assert_eq!(endpoint.url(), a);
        }

        foo("asdf");
    }
}
