//! # referencing
//!
//! An implementation-agnostic JSON reference resolution library for Rust.
mod anchors;
mod cache;
mod error;
mod list;
pub mod meta;
mod registry;
mod resolver;
mod resource;
mod retriever;
mod segments;
mod specification;
pub mod uri;
mod vocabularies;

pub(crate) use anchors::Anchor;
pub use error::{Error, UriError};
pub use fluent_uri::{Iri, IriRef, Uri, UriRef};
pub use list::List;
pub use registry::{parse_index, pointer, Registry, RegistryOptions, SPECIFICATIONS};
pub use resolver::{Resolved, Resolver};
pub use resource::{unescape_segment, Resource, ResourceRef};
pub use retriever::{DefaultRetriever, Retrieve};
pub(crate) use segments::Segments;
pub use specification::Draft;
pub use vocabularies::{Vocabulary, VocabularySet};

#[cfg(feature = "retrieve-async")]
pub use retriever::AsyncRetrieve;
