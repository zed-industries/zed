use crate::AddMetadata;
use std::fmt::Debug;

/// Add metadata (module name, producers) to a WebAssembly file.
///
/// Supports both core WebAssembly modules and components. In components,
/// metadata will be added to the outermost component.
#[derive(clap::Parser, Debug, Clone, Default)]
#[non_exhaustive]
pub struct AddMetadataOpts {
    /// Add a module or component name to the names section
    #[clap(long, value_name = "NAME", conflicts_with = "clear_name")]
    pub name: Option<String>,

    /// Remove a module or component name from the names section
    #[clap(long, conflicts_with = "name")]
    pub clear_name: bool,

    /// Add a programming language to the producers section
    #[clap(long, value_parser = parse_key_value, value_name = "NAME=VERSION")]
    pub language: Vec<(String, String)>,

    /// Add a tool and its version to the producers section
    #[clap(long = "processed-by", value_parser = parse_key_value, value_name="NAME=VERSION")]
    pub processed_by: Vec<(String, String)>,

    /// Add an SDK and its version to the producers section
    #[clap(long, value_parser = parse_key_value, value_name="NAME=VERSION")]
    pub sdk: Vec<(String, String)>,

    /// Contact details of the people or organization responsible,
    /// encoded as a freeform string.
    #[clap(long, value_name = "NAME", conflicts_with = "clear_authors")]
    #[cfg(feature = "oci")]
    pub authors: Option<crate::Authors>,

    /// Remove authors from the metadata
    #[clap(long, conflicts_with = "authors")]
    #[cfg(feature = "oci")]
    pub clear_authors: bool,

    /// A human-readable description of the binary
    #[clap(long, value_name = "NAME", conflicts_with = "clear_description")]
    #[cfg(feature = "oci")]
    pub description: Option<crate::Description>,

    /// Remove description from the metadata
    #[clap(long, conflicts_with = "description")]
    #[cfg(feature = "oci")]
    pub clear_description: bool,

    /// License(s) under which contained software is distributed as an SPDX License Expression.
    #[clap(long, value_name = "NAME", conflicts_with = "clear_licenses")]
    #[cfg(feature = "oci")]
    pub licenses: Option<crate::Licenses>,

    /// Remove licenses from the metadata
    #[clap(long, conflicts_with = "licenses")]
    #[cfg(feature = "oci")]
    pub clear_licenses: bool,

    /// URL to get source code for building the image
    #[clap(long, value_name = "NAME", conflicts_with = "clear_source")]
    #[cfg(feature = "oci")]
    pub source: Option<crate::Source>,

    /// Remove source from the metadata
    #[clap(long, conflicts_with = "source")]
    #[cfg(feature = "oci")]
    pub clear_source: bool,

    /// URL to find more information on the binary
    #[clap(long, value_name = "NAME", conflicts_with = "clear_homepage")]
    #[cfg(feature = "oci")]
    pub homepage: Option<crate::Homepage>,

    /// Remove homepage from the metadata
    #[clap(long, conflicts_with = "homepage")]
    #[cfg(feature = "oci")]
    pub clear_homepage: bool,

    /// Source control revision identifier for the packaged software.
    #[clap(long, value_name = "NAME", conflicts_with = "clear_revision")]
    #[cfg(feature = "oci")]
    pub revision: Option<crate::Revision>,

    /// Remove revision from the metadata
    #[clap(long, conflicts_with = "revision")]
    #[cfg(feature = "oci")]
    pub clear_revision: bool,

    /// Version of the packaged software
    #[clap(long, value_name = "NAME", conflicts_with = "clear_version")]
    #[cfg(feature = "oci")]
    pub version: Option<crate::Version>,

    /// Remove version from the metadata
    #[clap(long, conflicts_with = "version")]
    #[cfg(feature = "oci")]
    pub clear_version: bool,
}

pub(crate) fn parse_key_value(s: &str) -> anyhow::Result<(String, String)> {
    s.split_once('=')
        .map(|(k, v)| (k.to_owned(), v.to_owned()))
        .ok_or_else(|| anyhow::anyhow!("expected KEY=VALUE"))
}

impl From<AddMetadataOpts> for AddMetadata {
    fn from(value: AddMetadataOpts) -> Self {
        let mut add = AddMetadata::default();
        if let Some(name) = value.name {
            add.name = crate::AddMetadataField::Set(name);
        } else if value.clear_name {
            add.name = crate::AddMetadataField::Clear;
        }

        add.language = value.language;
        add.processed_by = value.processed_by;
        add.sdk = value.sdk;

        #[cfg(feature = "oci")]
        {
            if let Some(authors) = value.authors {
                add.authors = crate::AddMetadataField::Set(authors);
            } else if value.clear_authors {
                add.authors = crate::AddMetadataField::Clear;
            }

            if let Some(description) = value.description {
                add.description = crate::AddMetadataField::Set(description);
            } else if value.clear_description {
                add.description = crate::AddMetadataField::Clear;
            }

            if let Some(licenses) = value.licenses {
                add.licenses = crate::AddMetadataField::Set(licenses);
            } else if value.clear_licenses {
                add.licenses = crate::AddMetadataField::Clear;
            }

            if let Some(source) = value.source {
                add.source = crate::AddMetadataField::Set(source);
            } else if value.clear_source {
                add.source = crate::AddMetadataField::Clear;
            }

            if let Some(homepage) = value.homepage {
                add.homepage = crate::AddMetadataField::Set(homepage);
            } else if value.clear_homepage {
                add.homepage = crate::AddMetadataField::Clear;
            }

            if let Some(revision) = value.revision {
                add.revision = crate::AddMetadataField::Set(revision);
            } else if value.clear_revision {
                add.revision = crate::AddMetadataField::Clear;
            }

            if let Some(version) = value.version {
                add.version = crate::AddMetadataField::Set(version);
            } else if value.clear_version {
                add.version = crate::AddMetadataField::Clear;
            }
        }

        add
    }
}
