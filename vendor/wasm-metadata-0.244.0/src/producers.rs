use anyhow::Result;
use indexmap::{IndexMap, map::Entry};
use wasm_encoder::Encode;
use wasmparser::{BinaryReader, KnownCustom, Parser, ProducersSectionReader};

use crate::{AddMetadata, rewrite_wasm};
/// A representation of a WebAssembly producers section.
///
/// Spec: <https://github.com/WebAssembly/tool-conventions/blob/main/ProducersSection.md>
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde_derive::Serialize))]
pub struct Producers(
    #[cfg_attr(
        feature = "serde",
        serde(serialize_with = "indexmap::map::serde_seq::serialize")
    )]
    IndexMap<String, IndexMap<String, String>>,
);

impl Default for Producers {
    fn default() -> Self {
        Self::empty()
    }
}

impl Producers {
    /// Creates an empty producers section
    pub fn empty() -> Self {
        Producers(IndexMap::new())
    }

    /// Indicates if section is empty
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Read the producers section from a Wasm binary. Supports both core
    /// Modules and Components. In the component case, only returns the
    /// producers section in the outer component, ignoring all interior
    /// components and modules.
    pub fn from_wasm(bytes: &[u8]) -> Result<Option<Self>> {
        let mut depth = 0;
        for payload in Parser::new(0).parse_all(bytes) {
            let payload = payload?;
            use wasmparser::Payload::*;
            match payload {
                ModuleSection { .. } | ComponentSection { .. } => depth += 1,
                End { .. } => depth -= 1,
                CustomSection(c) if depth == 0 => {
                    if let KnownCustom::Producers(_) = c.as_known() {
                        let producers = Self::from_bytes(c.data(), c.data_offset())?;
                        return Ok(Some(producers));
                    }
                }
                _ => {}
            }
        }
        Ok(None)
    }
    /// Read the producers section from a Wasm binary.
    pub fn from_bytes(bytes: &[u8], offset: usize) -> Result<Self> {
        let reader = BinaryReader::new(bytes, offset);
        let section = ProducersSectionReader::new(reader)?;
        let mut fields = IndexMap::new();
        for field in section.into_iter() {
            let field = field?;
            let mut values = IndexMap::new();
            for value in field.values.into_iter() {
                let value = value?;
                values.insert(value.name.to_owned(), value.version.to_owned());
            }
            fields.insert(field.name.to_owned(), values);
        }
        Ok(Producers(fields))
    }
    /// Add a name & version value to a field.
    ///
    /// The spec says expected field names are "language", "processed-by", and "sdk".
    /// The version value should be left blank for languages.
    pub fn add(&mut self, field: &str, name: &str, version: &str) {
        match self.0.entry(field.to_string()) {
            Entry::Occupied(e) => {
                e.into_mut().insert(name.to_owned(), version.to_owned());
            }
            Entry::Vacant(e) => {
                let mut m = IndexMap::new();
                m.insert(name.to_owned(), version.to_owned());
                e.insert(m);
            }
        }
    }

    /// Add all values found in another `Producers` section. Values in `other` take
    /// precedence.
    pub fn merge(&mut self, other: &Self) {
        for (field, values) in other.iter() {
            for (name, version) in values.iter() {
                self.add(field, name, version);
            }
        }
    }

    /// Get the contents of a field
    pub fn get<'a>(&'a self, field: &str) -> Option<ProducersField<'a>> {
        self.0.get(&field.to_owned()).map(ProducersField)
    }

    /// Iterate through all fields
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = (&'a String, ProducersField<'a>)> + 'a {
        self.0
            .iter()
            .map(|(name, field)| (name, ProducersField(field)))
    }

    /// Construct the fields specified by [`AddMetadata`]
    pub(crate) fn from_meta(add: &AddMetadata) -> Self {
        let mut s = Self::empty();
        for (lang, version) in add.language.iter() {
            s.add("language", &lang, &version);
        }
        for (name, version) in add.processed_by.iter() {
            s.add("processed-by", &name, &version);
        }
        for (name, version) in add.sdk.iter() {
            s.add("sdk", &name, &version);
        }
        s
    }

    /// Serialize into [`wasm_encoder::ProducersSection`].
    pub(crate) fn section(&self) -> wasm_encoder::ProducersSection {
        let mut section = wasm_encoder::ProducersSection::new();
        for (fieldname, fieldvalues) in self.0.iter() {
            let mut field = wasm_encoder::ProducersField::new();
            for (name, version) in fieldvalues {
                field.value(&name, &version);
            }
            section.field(&fieldname, &field);
        }
        section
    }

    /// Serialize into the raw bytes of a wasm custom section.
    pub fn raw_custom_section(&self) -> Vec<u8> {
        let mut ret = Vec::new();
        self.section().encode(&mut ret);
        ret
    }

    /// Merge into an existing wasm module. Rewrites the module with this producers section
    /// merged into its existing one, or adds this producers section if none is present.
    pub fn add_to_wasm(&self, input: &[u8]) -> Result<Vec<u8>> {
        rewrite_wasm(&Default::default(), self, input)
    }
}

/// Contents of a producers field
#[derive(Debug)]
pub struct ProducersField<'a>(&'a IndexMap<String, String>);

impl<'a> ProducersField<'a> {
    /// Get the version associated with a name in the field
    pub fn get(&self, name: &str) -> Option<&'a String> {
        self.0.get(&name.to_owned())
    }
    /// Iterate through all name-version pairs in the field
    pub fn iter(&self) -> impl Iterator<Item = (&'a String, &'a String)> + 'a {
        self.0.iter()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{Metadata, Payload};
    use wasm_encoder::Module;

    #[test]
    fn producers_empty_module() {
        let module = Module::new().finish();
        let mut producers = Producers::empty();
        producers.add("language", "bar", "");
        producers.add("processed-by", "baz", "1.0");

        let module = producers.add_to_wasm(&module).unwrap();

        match Payload::from_binary(&module).unwrap() {
            Payload::Module(Metadata {
                name, producers, ..
            }) => {
                assert_eq!(name, None);
                let producers = producers.expect("some producers");
                assert_eq!(producers.get("language").unwrap().get("bar").unwrap(), "");
                assert_eq!(
                    producers.get("processed-by").unwrap().get("baz").unwrap(),
                    "1.0"
                );
            }
            _ => panic!("metadata should be module"),
        }
    }

    #[test]
    fn producers_add_another_field() {
        let module = Module::new().finish();
        let mut producers = Producers::empty();
        producers.add("language", "bar", "");
        producers.add("processed-by", "baz", "1.0");
        let module = producers.add_to_wasm(&module).unwrap();

        let mut producers = Producers::empty();
        producers.add("language", "waaat", "");
        let module = producers.add_to_wasm(&module).unwrap();

        match Payload::from_binary(&module).unwrap() {
            Payload::Module(Metadata {
                name, producers, ..
            }) => {
                assert_eq!(name, None);
                let producers = producers.expect("some producers");
                assert_eq!(producers.get("language").unwrap().get("bar").unwrap(), "");
                assert_eq!(producers.get("language").unwrap().get("waaat").unwrap(), "");
                assert_eq!(
                    producers.get("processed-by").unwrap().get("baz").unwrap(),
                    "1.0"
                );
            }
            _ => panic!("metadata should be module"),
        }
    }

    #[test]
    fn producers_overwrite_field() {
        let module = Module::new().finish();
        let mut producers = Producers::empty();
        producers.add("processed-by", "baz", "1.0");
        let module = producers.add_to_wasm(&module).unwrap();

        let mut producers = Producers::empty();
        producers.add("processed-by", "baz", "420");
        let module = producers.add_to_wasm(&module).unwrap();

        match Payload::from_binary(&module).unwrap() {
            Payload::Module(Metadata { producers, .. }) => {
                let producers = producers.expect("some producers");
                assert_eq!(
                    producers.get("processed-by").unwrap().get("baz").unwrap(),
                    "420"
                );
            }
            _ => panic!("metadata should be module"),
        }
    }
}
