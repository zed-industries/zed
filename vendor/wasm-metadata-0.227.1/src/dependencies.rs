use std::fmt::{self, Display};
use std::io::{read_to_string, Read};
use std::str::FromStr;

use anyhow::{ensure, Result};
use auditable_serde::VersionInfo;
use flate2::read::{ZlibDecoder, ZlibEncoder};
use flate2::Compression;
use serde::Serialize;
use wasm_encoder::{ComponentSection, CustomSection, Encode, Section};
use wasmparser::CustomSectionReader;

/// Human-readable description of the binary
#[derive(Debug, Clone, PartialEq)]
pub struct Dependencies {
    version_info: VersionInfo,
    custom_section: CustomSection<'static>,
}

impl Dependencies {
    /// Parse an `description` custom section from a wasm binary.
    pub(crate) fn parse_custom_section(reader: &CustomSectionReader<'_>) -> Result<Self> {
        ensure!(
            reader.name() == ".dep-v0",
            "The `dependencies` custom section should have a name of '.dep-v0'"
        );
        let decompressed_data = read_to_string(ZlibDecoder::new(reader.data()))?;
        let dependency_tree = auditable_serde::VersionInfo::from_str(&decompressed_data)?;

        Ok(Self {
            version_info: dependency_tree,
            custom_section: CustomSection {
                name: ".dep-v0".into(),
                data: reader.data().to_owned().into(),
            },
        })
    }

    /// Create a new instance of `Dependencies`.
    pub fn new(dependency_tree: auditable_serde::VersionInfo) -> Self {
        let data = serde_json::to_string(&dependency_tree).unwrap();

        let mut ret_vec = Vec::new();
        let mut encoder = ZlibEncoder::new(data.as_bytes(), Compression::fast());
        encoder.read_to_end(&mut ret_vec).unwrap();

        Self {
            version_info: dependency_tree,
            custom_section: CustomSection {
                name: ".dep-v0".into(),
                data: ret_vec.into(),
            },
        }
    }

    /// Provides access to the version information stored in the object
    pub fn version_info(&self) -> &VersionInfo {
        &self.version_info
    }
}

impl Serialize for Dependencies {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl Display for Dependencies {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // NOTE: this will never panic since we always guarantee the data is
        // encoded as utf8, even if we internally store it as [u8].
        // let data = String::from_utf8(self.0.data.to_vec()).unwrap();
        let data = serde_json::to_string(&self.version_info).unwrap();
        write!(f, "{}", data)
    }
}

impl ComponentSection for Dependencies {
    fn id(&self) -> u8 {
        ComponentSection::id(&self.custom_section)
    }
}

impl Section for Dependencies {
    fn id(&self) -> u8 {
        Section::id(&self.custom_section)
    }
}

impl Encode for Dependencies {
    fn encode(&self, sink: &mut Vec<u8>) {
        self.custom_section.encode(sink);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use auditable_serde::{Source, VersionInfo};
    use std::str::FromStr;
    use wasm_encoder::Component;
    use wasmparser::Payload;

    #[test]
    fn roundtrip() {
        let json_str = r#"{"packages":[{"name":"adler","version":"0.2.3","source":"registry"}]}"#;
        let info = VersionInfo::from_str(json_str).unwrap();
        assert_eq!(&info.packages[0].name, "adler");
        let mut component = Component::new();
        component.section(&Dependencies::new(info));
        let component = component.finish();

        let mut parsed = false;
        for section in wasmparser::Parser::new(0).parse_all(&component) {
            if let Payload::CustomSection(reader) = section.unwrap() {
                let dependencies = Dependencies::parse_custom_section(&reader).unwrap();
                assert_eq!(dependencies.to_string(), json_str);
                parsed = true;
            }
        }
        assert!(parsed);
    }

    #[test]
    fn serialize() {
        let json_str = r#"{"packages":[{"name":"adler","version":"0.2.3","source":"registry"}]}"#;
        let info = VersionInfo::from_str(json_str).unwrap();
        let dependencies = Dependencies::new(info);
        assert_eq!(dependencies.version_info().packages[0].name, "adler");
        assert_eq!(
            dependencies.version_info().packages[0].version.to_string(),
            "0.2.3"
        );
        assert_eq!(
            dependencies.version_info().packages[0].source,
            Source::Registry,
        );
    }
}
