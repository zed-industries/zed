//! Annotations following the [OCI Annotations Spec].
//!
//! The fields of these annotations are encoded into custom sections of
//! component binaries, and are explicitly compatible with the OCI Annotations
//! Spec. That enables Compontents to be encoded to OCI and back without needing
//! to perform any additional parsing. This greatly simplifies adding metadata to
//! component registries, since language-native component toolchains can encode them
//! directly into components. Which in turn can be picked up by Component-to-OCI
//! tooling to take those annotations and display them in a way that registries can
//! understand.
//!
//! For the files in this submodule that means we want to be explicitly
//! compatible with the OCI Annotations specification. Any deviation in our
//! parsing rules from the spec should be considered a bug we have to fix.
//!
//! [OCI Annotations Spec]: https://specs.opencontainers.org/image-spec/annotations/

pub use authors::Authors;
pub use description::Description;
pub use homepage::Homepage;
pub use licenses::Licenses;
pub use revision::Revision;
pub use source::Source;
pub use version::Version;

mod authors;
mod description;
mod homepage;
mod licenses;
mod revision;
mod source;
mod version;
