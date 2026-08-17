use std::borrow::Cow;
use std::fmt::{self, Display};
use std::str::FromStr;

use anyhow::{ensure, Error, Result};
use serde::Serialize;
use url::Url;
use wasm_encoder::{ComponentSection, CustomSection, Encode, Section};
use wasmparser::CustomSectionReader;

/// URL to get source code for building the image
#[derive(Debug, Clone, PartialEq)]
pub struct Source(CustomSection<'static>);

impl Source {
    /// Create a new instance of `Source`.
    pub fn new(s: &str) -> Result<Self> {
        Ok(Url::parse(s)?.into())
    }

    /// Parse a `source` custom section from a wasm binary.
    pub(crate) fn parse_custom_section(reader: &CustomSectionReader<'_>) -> Result<Self> {
        ensure!(
            reader.name() == "source",
            "The `source` custom section should have a name of 'source'"
        );
        let data = String::from_utf8(reader.data().to_owned())?;
        Self::new(&data)
    }
}

impl FromStr for Source {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl From<Url> for Source {
    fn from(expression: Url) -> Self {
        Self(CustomSection {
            name: "source".into(),
            data: Cow::Owned(expression.to_string().into_bytes()),
        })
    }
}

impl Serialize for Source {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // NOTE: this will never panic since we always guarantee the data is
        // encoded as utf8, even if we internally store it as [u8].
        let data = String::from_utf8(self.0.data.to_vec()).unwrap();
        write!(f, "{data}")
    }
}

impl ComponentSection for Source {
    fn id(&self) -> u8 {
        ComponentSection::id(&self.0)
    }
}

impl Section for Source {
    fn id(&self) -> u8 {
        Section::id(&self.0)
    }
}

impl Encode for Source {
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
        component.section(&Source::new("https://github.com/bytecodealliance/wasm-tools").unwrap());
        let component = component.finish();

        let mut parsed = false;
        for section in wasmparser::Parser::new(0).parse_all(&component) {
            if let Payload::CustomSection(reader) = section.unwrap() {
                let description = Source::parse_custom_section(&reader).unwrap();
                assert_eq!(
                    description.to_string(),
                    "https://github.com/bytecodealliance/wasm-tools"
                );
                parsed = true;
            }
        }
        assert!(parsed);
    }

    #[test]
    fn serialize() {
        let description = Source::new("https://github.com/bytecodealliance/wasm-tools").unwrap();
        let json = serde_json::to_string(&description).unwrap();
        assert_eq!(r#""https://github.com/bytecodealliance/wasm-tools""#, json);
    }
}
