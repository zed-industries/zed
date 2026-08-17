use std::borrow::Cow;
use std::fmt::{self, Display};
use std::str::FromStr;

use anyhow::{ensure, Error, Result};
use serde::Serialize;
use wasm_encoder::{ComponentSection, CustomSection, Encode, Section};
use wasmparser::CustomSectionReader;

/// License(s) under which contained software is distributed as an SPDX License Expression.
#[derive(Debug, Clone, PartialEq)]
pub struct Licenses(CustomSection<'static>);

impl Licenses {
    /// Create a new instance of `Licenses`.
    pub fn new(s: &str) -> Result<Self> {
        Ok(spdx::Expression::parse(s)?.into())
    }

    /// Parse a `licenses` custom section from a wasm binary.
    pub(crate) fn parse_custom_section(reader: &CustomSectionReader<'_>) -> Result<Self> {
        ensure!(
            reader.name() == "licenses",
            "The `licenses` custom section should have a name of 'license'"
        );
        let data = String::from_utf8(reader.data().to_owned())?;
        Self::new(&data)
    }
}

impl FromStr for Licenses {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl From<spdx::Expression> for Licenses {
    fn from(expression: spdx::Expression) -> Self {
        Self(CustomSection {
            name: "licenses".into(),
            data: Cow::Owned(expression.to_string().into_bytes()),
        })
    }
}

impl Serialize for Licenses {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl Display for Licenses {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // NOTE: this will never panic since we always guarantee the data is
        // encoded as utf8, even if we internally store it as [u8].
        let data = String::from_utf8(self.0.data.to_vec()).unwrap();
        write!(f, "{data}")
    }
}

impl ComponentSection for Licenses {
    fn id(&self) -> u8 {
        ComponentSection::id(&self.0)
    }
}

impl Section for Licenses {
    fn id(&self) -> u8 {
        Section::id(&self.0)
    }
}

impl Encode for Licenses {
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
        component.section(
            &Licenses::new("Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT").unwrap(),
        );
        let component = component.finish();

        let mut parsed = false;
        for section in wasmparser::Parser::new(0).parse_all(&component) {
            if let Payload::CustomSection(reader) = section.unwrap() {
                let description = Licenses::parse_custom_section(&reader).unwrap();
                assert_eq!(
                    description.to_string(),
                    "Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT"
                );
                parsed = true;
            }
        }
        assert!(parsed);
    }

    #[test]
    fn serialize() {
        let description =
            Licenses::new("Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT").unwrap();
        let json = serde_json::to_string(&description).unwrap();
        assert_eq!(
            r#""Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT""#,
            json
        );
    }
}
