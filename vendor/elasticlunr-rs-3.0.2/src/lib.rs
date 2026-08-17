//!# elasticlunr-rs
//!
//! [![Build Status](https://travis-ci.org/mattico/elasticlunr-rs.svg?branch=master)](https://travis-ci.org/mattico/elasticlunr-rs)
//! [![Documentation](https://docs.rs/elasticlunr-rs/badge.svg)](https://docs.rs/elasticlunr-rs)
//! [![Crates.io](https://img.shields.io/crates/v/elasticlunr-rs.svg)](https://crates.io/crates/elasticlunr-rs)
//!
//! A partial port of [elasticlunr](https://github.com/weixsong/elasticlunr.js) to Rust. Intended to
//! be used for generating compatible search indices.
//!
//! Access to all index-generating functionality is provided. Most users will only need to use the
//! [`Index`](struct.Index.html) or [`IndexBuilder`](struct.IndexBuilder.html) types.
//!
//! The [`Language`] trait can be used to implement a custom language.
//!
//! ## Example
//!
//! ```
//! use std::fs::File;
//! use std::io::Write;
//! use elasticlunr::Index;
//!
//! let mut index = Index::new(&["title", "body"]);
//! index.add_doc("1", &["This is a title", "This is body text!"]);
//! // Add more docs...
//! let mut file = File::create("out.json").unwrap();
//! file.write_all(index.to_json_pretty().as_bytes());
//! ```

#[macro_use]
extern crate serde_derive;

#[cfg(test)]
#[macro_use]
extern crate maplit;

/// The version of elasticlunr.js this library was designed for.
pub const ELASTICLUNR_VERSION: &str = "0.9.5";

pub mod config;
pub mod document_store;
pub mod inverted_index;
pub mod lang;
pub mod pipeline;

use std::collections::BTreeMap;

use document_store::DocumentStore;
use inverted_index::InvertedIndex;
use lang::English;
pub use lang::Language;
pub use pipeline::Pipeline;

type Tokenizer = Option<Box<dyn Fn(&str) -> Vec<String>>>;

/// A builder for an `Index` with custom parameters.
///
/// # Example
/// ```
/// # use elasticlunr::{Index, IndexBuilder};
/// let mut index = IndexBuilder::new()
///     .save_docs(false)
///     .add_fields(&["title", "subtitle", "body"])
///     .set_ref("doc_id")
///     .build();
/// index.add_doc("doc_a", &["Chapter 1", "Welcome to Copenhagen", "..."]);
/// ```
pub struct IndexBuilder {
    save: bool,
    fields: Vec<String>,
    field_tokenizers: Vec<Tokenizer>,
    ref_field: String,
    pipeline: Option<Pipeline>,
    language: Box<dyn Language>,
}

impl Default for IndexBuilder {
    fn default() -> Self {
        IndexBuilder {
            save: true,
            fields: Vec::new(),
            field_tokenizers: Vec::new(),
            ref_field: "id".into(),
            pipeline: None,
            language: Box::new(English::new()),
        }
    }
}

impl IndexBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_language(language: Box<dyn Language>) -> Self {
        Self {
            language,
            ..Default::default()
        }
    }

    /// Set whether or not documents should be saved in the `Index`'s document store.
    pub fn save_docs(mut self, save: bool) -> Self {
        self.save = save;
        self
    }

    /// Add a document field to the `Index`.
    ///
    /// # Panics
    ///
    /// Panics if a field with the name already exists.
    pub fn add_field(mut self, field: &str) -> Self {
        let field = field.into();
        if self.fields.contains(&field) {
            panic!("Duplicate fields in index: {}", field);
        }
        self.fields.push(field);
        self.field_tokenizers.push(None);
        self
    }

    /// Add a document field to the `Index`, with a custom tokenizer for that field.
    ///
    /// # Panics
    ///
    /// Panics if a field with the name already exists.
    pub fn add_field_with_tokenizer(
        mut self,
        field: &str,
        tokenizer: Box<dyn Fn(&str) -> Vec<String>>,
    ) -> Self {
        let field = field.into();
        if self.fields.contains(&field) {
            panic!("Duplicate fields in index: {}", field);
        }
        self.fields.push(field);
        self.field_tokenizers.push(Some(tokenizer));
        self
    }

    /// Add the document fields to the `Index`.
    ///
    /// # Panics
    ///
    /// Panics if two fields have the same name.
    pub fn add_fields<I>(mut self, fields: I) -> Self
    where
        I: IntoIterator,
        I::Item: AsRef<str>,
    {
        for field in fields {
            self = self.add_field(field.as_ref())
        }
        self
    }

    /// Set the key used to store the document reference field.
    pub fn set_ref(mut self, ref_field: &str) -> Self {
        self.ref_field = ref_field.into();
        self
    }

    /// Build an `Index` from this builder.
    pub fn build(self) -> Index {
        let IndexBuilder {
            save,
            fields,
            field_tokenizers,
            ref_field,
            pipeline,
            language,
        } = self;

        let index = fields
            .iter()
            .map(|f| (f.clone(), InvertedIndex::new()))
            .collect();

        let pipeline = pipeline.unwrap_or_else(|| language.make_pipeline());

        Index {
            index,
            fields,
            field_tokenizers,
            ref_field,
            document_store: DocumentStore::new(save),
            pipeline,
            version: crate::ELASTICLUNR_VERSION,
            lang: language,
        }
    }
}

/// An elasticlunr search index.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Index {
    fields: Vec<String>,
    #[serde(skip)]
    field_tokenizers: Vec<Tokenizer>,
    pipeline: Pipeline,
    #[serde(rename = "ref")]
    ref_field: String,
    version: &'static str,
    index: BTreeMap<String, InvertedIndex>,
    document_store: DocumentStore,
    #[serde(with = "ser_lang")]
    lang: Box<dyn Language>,
}

mod ser_lang {
    use crate::Language;
    use serde::de;
    use serde::{Deserializer, Serializer};
    use std::fmt;

    pub fn serialize<S>(lang: &Box<dyn Language>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&lang.name())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Box<dyn Language>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(LanguageVisitor)
    }

    struct LanguageVisitor;

    impl<'de> de::Visitor<'de> for LanguageVisitor {
        type Value = Box<dyn Language>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a capitalized language name")
        }

        fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            match crate::lang::from_name(v) {
                Some(l) => Ok(l),
                None => Err(E::custom(format!("Unknown language name: {}", v))),
            }
        }
    }
}

impl Index {
    /// Create a new index with the provided fields.
    ///
    /// # Example
    ///
    /// ```
    /// # use elasticlunr::{Index};
    /// let mut index = Index::new(&["title", "body"]);
    /// index.add_doc("1", &["this is a title", "this is body text"]);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if a field with the name already exists.
    pub fn new<I>(fields: I) -> Self
    where
        I: IntoIterator,
        I::Item: AsRef<str>,
    {
        IndexBuilder::new().add_fields(fields).build()
    }

    /// Create a new index with the provided fields for the given
    /// [`Language`](lang/enum.Language.html).
    ///
    /// # Example
    ///
    /// ```
    /// use elasticlunr::{Index, lang::English};
    /// let mut index = Index::with_language(Box::new(English::new()), &["title", "body"]);
    /// index.add_doc("1", &["this is a title", "this is body text"]);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if a field with the name already exists.
    pub fn with_language<I>(lang: Box<dyn Language>, fields: I) -> Self
    where
        I: IntoIterator,
        I::Item: AsRef<str>,
    {
        IndexBuilder::with_language(lang).add_fields(fields).build()
    }

    /// Add the data from a document to the index.
    ///
    /// *NOTE: The elements of `data` should be provided in the same order as
    /// the fields used to create the index.*
    ///
    /// # Example
    /// ```
    /// # use elasticlunr::Index;
    /// let mut index = Index::new(&["title", "body"]);
    /// index.add_doc("1", &["this is a title", "this is body text"]);
    /// ```
    pub fn add_doc<I>(&mut self, doc_ref: &str, data: I)
    where
        I: IntoIterator,
        I::Item: AsRef<str>,
    {
        let mut doc = BTreeMap::new();
        doc.insert(self.ref_field.clone(), doc_ref.into());
        let mut token_freq = BTreeMap::new();

        for (i, value) in data.into_iter().enumerate() {
            let field = &self.fields[i];
            let tokenizer = self.field_tokenizers[i].as_ref();
            doc.insert(field.clone(), value.as_ref().to_string());

            if field == &self.ref_field {
                continue;
            }

            let raw_tokens = if let Some(tokenizer) = tokenizer {
                tokenizer(value.as_ref())
            } else {
                self.lang.tokenize(value.as_ref())
            };

            let tokens = self.pipeline.run(raw_tokens);

            self.document_store
                .add_field_length(doc_ref, field, tokens.len());

            for token in tokens {
                *token_freq.entry(token).or_insert(0u64) += 1;
            }

            for (token, count) in &token_freq {
                let freq = (*count as f64).sqrt();

                self.index
                    .get_mut(field)
                    .unwrap_or_else(|| panic!("InvertedIndex does not exist for field {}", field))
                    .add_token(doc_ref, token, freq);
            }
        }

        self.document_store.add_doc(doc_ref, doc);
    }

    pub fn get_fields(&self) -> &[String] {
        &self.fields
    }

    /// Returns the index, serialized to pretty-printed JSON.
    pub fn to_json_pretty(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }

    /// Returns the index, serialized to JSON.
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_field_to_builder() {
        let idx = IndexBuilder::new()
            .add_fields(&["foo", "bar", "baz"])
            .build();

        let idx_fields = idx.get_fields();
        for f in &["foo", "bar", "baz"] {
            assert_eq!(idx_fields.iter().filter(|x| x == f).count(), 1);
        }
    }

    #[test]
    fn adding_document_to_index() {
        let mut idx = Index::new(&["body"]);
        idx.add_doc("1", &["this is a test"]);

        assert_eq!(idx.document_store.len(), 1);
        assert_eq!(
            idx.document_store.get_doc("1").unwrap(),
            btreemap! {
                "id".into() => "1".into(),
                "body".into() => "this is a test".into(),
            }
        );
    }

    #[test]
    fn adding_document_with_empty_field() {
        let mut idx = Index::new(&["title", "body"]);

        idx.add_doc("1", &["", "test"]);
        assert_eq!(idx.index["body"].get_doc_frequency("test"), 1);
        assert_eq!(idx.index["body"].get_docs("test").unwrap()["1"], 1.);
    }

    #[test]
    #[should_panic]
    fn creating_index_with_identical_fields_panics() {
        let _idx = Index::new(&["title", "body", "title"]);
    }
}
