/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::Number;
use std::borrow::Cow;
use std::collections::HashMap;

#[cfg(any(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    all(aws_sdk_unstable, feature = "serde-serialize")
))]
use serde;

/* ANCHOR: document */

/// Document Type
///
/// Document types represents protocol-agnostic open content that is accessed like JSON data.
/// Open content is useful for modeling unstructured data that has no schema, data that can't be
/// modeled using rigid types, or data that has a schema that evolves outside of the purview of a model.
/// The serialization format of a document is an implementation detail of a protocol.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
#[cfg_attr(
    any(
        all(aws_sdk_unstable, feature = "serde-deserialize"),
        all(aws_sdk_unstable, feature = "serde-serialize")
    ),
    serde(untagged)
)]
pub enum Document {
    /// JSON object
    Object(HashMap<String, Document>),
    /// JSON array
    Array(Vec<Document>),
    /// JSON number
    Number(Number),
    /// JSON string
    String(String),
    /// JSON boolean
    Bool(bool),
    /// JSON null
    Null,
}

impl Document {
    /// Returns the inner map value if this `Document` is an object.
    pub fn as_object(&self) -> Option<&HashMap<String, Document>> {
        if let Self::Object(object) = self {
            Some(object)
        } else {
            None
        }
    }

    /// Returns the mutable inner map value if this `Document` is an object.
    pub fn as_object_mut(&mut self) -> Option<&mut HashMap<String, Document>> {
        if let Self::Object(object) = self {
            Some(object)
        } else {
            None
        }
    }

    /// Returns the inner array value if this `Document` is an array.
    pub fn as_array(&self) -> Option<&Vec<Document>> {
        if let Self::Array(array) = self {
            Some(array)
        } else {
            None
        }
    }

    /// Returns the mutable inner array value if this `Document` is an array.
    pub fn as_array_mut(&mut self) -> Option<&mut Vec<Document>> {
        if let Self::Array(array) = self {
            Some(array)
        } else {
            None
        }
    }

    /// Returns the inner number value if this `Document` is a number.
    pub fn as_number(&self) -> Option<&Number> {
        if let Self::Number(number) = self {
            Some(number)
        } else {
            None
        }
    }

    /// Returns the inner string value if this `Document` is a string.
    pub fn as_string(&self) -> Option<&str> {
        if let Self::String(string) = self {
            Some(string)
        } else {
            None
        }
    }

    /// Returns the inner boolean value if this `Document` is a boolean.
    pub fn as_bool(&self) -> Option<bool> {
        if let Self::Bool(boolean) = self {
            Some(*boolean)
        } else {
            None
        }
    }

    /// Returns `Some(())` if this `Document` is a null.
    pub fn as_null(&self) -> Option<()> {
        if let Self::Null = self {
            Some(())
        } else {
            None
        }
    }

    /// Returns `true` if this `Document` is an object.
    pub fn is_object(&self) -> bool {
        matches!(self, Self::Object(_))
    }

    /// Returns `true` if this `Document` is an array.
    pub fn is_array(&self) -> bool {
        matches!(self, Self::Array(_))
    }

    /// Returns `true` if this `Document` is a number.
    pub fn is_number(&self) -> bool {
        matches!(self, Self::Number(_))
    }

    /// Returns `true` if this `Document` is a string.
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    /// Returns `true` if this `Document` is a bool.
    pub fn is_bool(&self) -> bool {
        matches!(self, Self::Bool(_))
    }

    /// Returns `true` if this `Document` is a boolean.
    pub fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }
}

/// The default value is `Document::Null`.
impl Default for Document {
    fn default() -> Self {
        Self::Null
    }
}

impl From<bool> for Document {
    fn from(value: bool) -> Self {
        Document::Bool(value)
    }
}

impl<'a> From<&'a str> for Document {
    fn from(value: &'a str) -> Self {
        Document::String(value.to_string())
    }
}

impl<'a> From<Cow<'a, str>> for Document {
    fn from(value: Cow<'a, str>) -> Self {
        Document::String(value.into_owned())
    }
}

impl From<String> for Document {
    fn from(value: String) -> Self {
        Document::String(value)
    }
}

impl From<Vec<Document>> for Document {
    fn from(values: Vec<Document>) -> Self {
        Document::Array(values)
    }
}

impl From<HashMap<String, Document>> for Document {
    fn from(values: HashMap<String, Document>) -> Self {
        Document::Object(values)
    }
}

impl From<u64> for Document {
    fn from(value: u64) -> Self {
        Document::Number(Number::PosInt(value))
    }
}

impl From<i64> for Document {
    fn from(value: i64) -> Self {
        Document::Number(Number::NegInt(value))
    }
}

impl From<i32> for Document {
    fn from(value: i32) -> Self {
        Document::Number(Number::NegInt(value as i64))
    }
}

impl From<f64> for Document {
    fn from(value: f64) -> Self {
        Document::Number(Number::Float(value))
    }
}

impl From<Number> for Document {
    fn from(value: Number) -> Self {
        Document::Number(value)
    }
}

impl<T> From<Option<T>> for Document
where
    Document: From<T>,
{
    fn from(value: Option<T>) -> Self {
        match value {
            Some(inner) => inner.into(),
            None => Document::Null,
        }
    }
}

/* ANCHOR END: document */

#[cfg(test)]
mod test {
    /// checks if a) serialization of json suceeds and b) it is compatible with serde_json
    #[test]
    #[cfg(all(
        aws_sdk_unstable,
        feature = "serde-serialize",
        feature = "serde-deserialize"
    ))]
    fn serialize_json() {
        use crate::Document;
        use crate::Number;
        use std::collections::HashMap;
        let mut map: HashMap<String, Document> = HashMap::new();
        // string
        map.insert("hello".into(), "world".to_string().into());
        // numbers
        map.insert("pos_int".into(), Document::Number(Number::PosInt(1).into()));
        map.insert(
            "neg_int".into(),
            Document::Number(Number::NegInt(-1).into()),
        );
        map.insert(
            "float".into(),
            Document::Number(Number::Float(0.1 + 0.2).into()),
        );
        // booleans
        map.insert("true".into(), true.into());
        map.insert("false".into(), false.into());
        // check if array with different datatypes would succeed
        map.insert(
            "array".into(),
            vec![
                map.clone().into(),
                "hello-world".to_string().into(),
                true.into(),
                false.into(),
            ]
            .into(),
        );
        // map
        map.insert("map".into(), map.clone().into());
        // null
        map.insert("null".into(), Document::Null);
        let obj = Document::Object(map);
        // comparing string isnt going to work since there is no gurantee for the ordering of the keys
        let target_file = include_str!("../test_data/serialize_document.json");
        let json: Result<serde_json::Value, _> = serde_json::from_str(target_file);
        // serializer
        assert_eq!(serde_json::to_value(&obj).unwrap(), json.unwrap());
        let doc: Result<Document, _> = serde_json::from_str(target_file);
        assert_eq!(obj, doc.unwrap());
    }
}
