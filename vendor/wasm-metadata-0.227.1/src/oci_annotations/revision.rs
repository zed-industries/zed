use std::borrow::Cow;
use std::fmt::{self, Display};
use std::str::FromStr;

use anyhow::{ensure, Error, Result};
use serde::Serialize;
use wasm_encoder::{ComponentSection, CustomSection, Encode, Section};
use wasmparser::CustomSectionReader;

/// Source control revision identifier for the packaged software.
#[derive(Debug, Clone, PartialEq)]
pub struct Revision(CustomSection<'static>);

impl Revision {
    /// Create a new instance of `Desrciption`.
    pub fn new<S: Into<Cow<'static, str>>>(s: S) -> Self {
        Self(CustomSection {
            name: "revision".into(),
            data: match s.into() {
                Cow::Borrowed(s) => Cow::Borrowed(s.as_bytes()),
                Cow::Owned(s) => Cow::Owned(s.into()),
            },
        })
    }

    /// Parse an `revision` custom section from a wasm binary.
    pub(crate) fn parse_custom_section(reader: &CustomSectionReader<'_>) -> Result<Self> {
        ensure!(
            reader.name() == "revision",
            "The `revision` custom section should have a name of 'revision'"
        );
        let data = String::from_utf8(reader.data().to_owned())?;
        Ok(Self::new(data))
    }
}

impl FromStr for Revision {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s.to_owned()))
    }
}

impl Serialize for Revision {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl Display for Revision {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // NOTE: this will never panic since we always guarantee the data is
        // encoded as utf8, even if we internally store it as [u8].
        let data = String::from_utf8(self.0.data.to_vec()).unwrap();
        write!(f, "{data}")
    }
}

impl ComponentSection for Revision {
    fn id(&self) -> u8 {
        ComponentSection::id(&self.0)
    }
}

impl Section for Revision {
    fn id(&self) -> u8 {
        Section::id(&self.0)
    }
}

impl Encode for Revision {
    fn encode(&self, sink: &mut Vec<u8>) {
        self.0.encode(sink);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use wasm_encoder::Component;
    use wasmparser::Payload;

    #[test]
    fn roundtrip() {
        let mut component = Component::new();
        component.section(&Revision::new("de978e17a80c1118f606fce919ba9b7d5a04a5ad"));
        let component = component.finish();

        let mut parsed = false;
        for section in wasmparser::Parser::new(0).parse_all(&component) {
            if let Payload::CustomSection(reader) = section.unwrap() {
                let revision = Revision::parse_custom_section(&reader).unwrap();
                assert_eq!(
                    revision.to_string(),
                    "de978e17a80c1118f606fce919ba9b7d5a04a5ad"
                );
                parsed = true;
            }
        }
        assert!(parsed);
    }

    #[test]
    fn serialize() {
        let revision = Revision::new("de978e17a80c1118f606fce919ba9b7d5a04a5ad");
        let json = serde_json::to_string(&revision).unwrap();
        assert_eq!(r#""de978e17a80c1118f606fce919ba9b7d5a04a5ad""#, json);
    }
}
